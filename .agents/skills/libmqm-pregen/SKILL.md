---
name: libmqm-pregen
description: Use when working on the generated IBM MQ bindings in this repository - regenerating the committed files under src/pregen/, bumping the MQ client version, adding or changing MQ constants, editing the build scripts under libmqm-*/build/, or reviewing a huge generated diff. Covers the pregen write-back, per-platform file naming, the constant registry, and the CI checks that gate all of it.
---

# Regenerating the IBM MQ bindings

Nearly every public item in this workspace is generated from IBM's C headers and
**committed**. There is no hand-written API layer to fall back on, so a mistake here
either silently disappears on the next regeneration or fails a CI check that compares
the committed files against a fresh generation.

## The two build paths

| Path | Selected by | Bindings come from | Needs MQ headers |
|---|---|---|---|
| Committed (normal builds, tests, downstream) | default features | `src/pregen/**` via `include!` | no |
| Generated | `bindgen` enabled | `bindgen` + syn visitors parse `$MQ_HOME/inc` at build time into `OUT_DIR` | yes |

`pregen` implies `bindgen`, and additionally **copies the freshly generated files back
over `src/pregen/**`** - a write to the working tree.

## Regenerate

```sh
MQ_HOME=/path/to/mq/client cargo build --no-default-features --features pregen
```

- Set `MQ_HOME` to a client whose version matches `.github/mq_version_pregen.json`,
  otherwise CI's diff check fails later. Defaults are `/opt/mqm` and
  `c:/Program Files/IBM/MQ`.
- Needs `libclang` (bindgen) and a C compiler (`cc` builds `libmqm-sys/src/c/*.c`).
- **Run once per platform/arch, on that platform.** The output encodes pointer width,
  endianness and platform defines. `libmqm-default`'s generator actively refuses to run
  when the host endianness or pointer width differs from the target, so cross-compiling
  it will not work.
- CI does this in the `pregen` job (ubuntu x64/arm, macOS, Windows) and `pregen_arch`
  (s390x, ppc64le under qemu), then asserts
  `git diff --stat --exit-code libmqm-sys/src libmqm-default/src libmqm-constants/src`.
- On a PR, comment `/pregen` to have CI regenerate every platform and commit the result
  to a `pregen_<run-id>` branch that is merged into the PR.

## What gets written, and where

Directory naming differs per crate. `<arch>` is the Rust target arch, so `powerpc64le`
appears as `powerpc64`, and macOS is always `any-macos` (one client covers both arches).

| Crate | Files written |
|---|---|
| `libmqm-sys` | `src/pregen/<arch>-<os>/{mqi,mqai,exits,pcf,version}.rs`, plus shared `src/pregen/{function,mock,dlopen2,link}.rs` |
| `libmqm-default` | `src/pregen/<arch>-<os>-defaults.rs` |
| `libmqm-constants` | `src/pregen/<arch>-<os>-mapping.rs`, `src/pregen/<arch>-<os>-new_types.rs` |

Every generated file starts with `/* Generated with MQ client version <x> */`; the
client that produced the committed bindings is also in
`libmqm-sys/src/pregen/<arch>-<os>/version.rs` (`CLIENT_BUILD_VERSION`).

## Bumping the MQ client version

1. Edit only `.github/mq_version_pregen.json` - keys are `runner.os` + `runner.arch`
   concatenated (`LinuxX64`, `LinuxPPC64LE`, `macOSARM64`, `WindowsX64`, ...). CI reads
   this table to choose which client to install for pregen, so all platforms move
   together.
2. Run `/pregen` on the PR (or regenerate locally and let CI verify).
3. Bumping past the newest `mqc_*` feature means adding the next one first -
   the `mqc_*` features are a chain (each implies the previous) and `mqc_latest`
   currently resolves to the newest of them.

## Adding or changing a constant

The registry is hand-maintained in `libmqm-constants/build/constants/list.rs`:

- `CONSTANTS` - entries whose names the client library knows, via a `_STR` helper from
  `libmqm-sys/src/c/strings.c` (`mqstr::MQ<NAME>_STR`). Fields: `prefix`, `new_type`,
  `orig_type`, `usage`, optional `feature` gate, optional doc string.
- `PREFIX_CONSTANTS` - prefixes with no `_STR` function. Anything starting with the
  prefix that is not claimed by a longer prefix is swept in as an "extra".
- `CONSTANTS_DOC` - extra documentation keyed by exact constant name.
- New constant groups must be gated (`#[cfg(feature = "mqc_x_y_z_w")]`) so older clients
  keep working.

A constant whose name is not in the client library and does not match a prefix lands in
the **unmapped** comment block at the top of the generated
`src/pregen/<arch>-<os>-new_types.rs` - check there first when a constant appears to be
missing. `const_ignore_regex()` in `build/constants/generate.rs` deliberately excludes
lengths, versions, ranges and `_MASK` names.

Lookup tables are chosen by the number of constants for a prefix: linear `<=63`, binary
search `64..=255`, otherwise a phf perfect hash.

## Feature gates worth knowing

- `libmqm-sys/build/main.rs` reads `README.Redist` / `README.Client` / `README.Advanced`
  from `MQ_HOME`, extracts the client version, and asserts the highest enabled `mqc_*`
  feature does not exceed it. Enabling `mqc_latest` against an older client fails the
  build with that message.
- The three crates' `pregen` sets deliberately pin *different* versions
  (`libmqm-default` is the oldest); do not assume they match.
- `constant_lookup` / `struct_defaults` / `versiongen` are internal switches turned on by
  the generators in the other two crates. They imply header access, so any build that
  enables them needs `MQ_HOME`.

## Reviewing a generated diff

The diff is normally enormous and mostly meaningless:

- a phf-backed prefix lays its constants out by hash, so adding or removing a single
  constant reshuffles that whole table and can rewrite thousands of lines for a one-line
  change. Expect it and do not try to hand-merge it; re-running the same generator against
  the same client reproduces the file byte for byte.
- What actually matters: the `Generated with MQ client version` banners, `version.rs`,
  newly added or removed constants, new/changed MQ structure fields (which also change
  the `MQCD_DEFAULT`-style byte arrays), and the unmapped-constant comment block.

```sh
rg -m1 'Generated with MQ client version' libmqm-sys/src/pregen/x86_64-linux/mqi.rs
git diff --stat libmqm-sys/src libmqm-default/src libmqm-constants/src
```

## Verifying

```sh
cargo fmt -- --check
cargo check --locked --all-features          # needs MQ_HOME for headers/link
cargo nextest run --profile ci --features mqc_9_3_5_0,pcf,exits,mqai,dlopen2
```

CI compiles with `RUSTFLAGS=-Dwarnings` and runs `cargo hack` clippy over feature
powersets, so generated modules carry blanket `#![allow(...)]` prologues - leave them
alone rather than fixing lints item by item.

## Do not

- Hand-edit anything under `src/pregen/` - the next generation overwrites it and CI
  fails until it matches.
- Add a safe wrapper here; that belongs in the separate `mqi` crate.
- "Fix" MQ naming (`non_snake_case`, `non_camel_case_types`, `unreadable_literal` are
  allowed on purpose).
- Patch generated output to change an argument type or name - extend the rewrite tables in
  `libmqm-sys/build/main.rs` (`FnArgType::replace_all` / `replace_fns`) or the visitors in
  `build/rustify.rs`, `build/mq_trait.rs` and `build/doc_comments.rs` instead.
- Regenerate with a client version that differs from `.github/mq_version_pregen.json`.

## Common failures

| Symptom | Cause |
|---|---|
| `could not extract version from MQ README file` | `MQ_HOME` wrong or pointed at something that is not an MQ install |
| `MQC version <x> does not meet the minimum requirement for feature <f>` | enabled `mqc_*` is newer than the installed client |
| `Failed to compile c files` | MQ headers not reachable from `MQ_HOME` (`inc/`, or `tools/c/include` on Windows) |
| bindgen panics / cannot find `libclang` | install `libclang` |
| CI `Detect pregenerated changes` fails | committed `src/pregen/**` does not match a fresh generation for that platform |
