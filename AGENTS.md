# AGENTS.md

Guidance for agents working in this repository.

## What this repository is

A Rust workspace of **raw FFI bindings** to the IBM MQ C libraries (MQI, PCF, MQAI, exits).
It deliberately exposes only `unsafe` functions, structures, constants and type aliases; a
safe wrapper lives in a separate repository (`mqi`). Do not add a "safe API" layer here.

Almost all of the public API surface is **generated code that is committed to the
repository**. That single fact drives most of the workflows below.

## Crates

| Crate | Purpose | Publishes |
|---|---|---|
| `libmqm-sys` | Bindgen output: MQI/MQAI/PCF/exits types, functions, `unsafe` traits (`Mqi`, `Mqai`, `Exits`), `LinkedMq` (compile-time link), `dlopen2::MqWrapper` (runtime link), `mock::Mq` | crates.io |
| `libmqm-default` | `const` initialisers for MQ structures, emitted as `transmute`d byte arrays (they must be `const`) | crates.io |
| `libmqm-constants` | Typed constants ("new types"), bitflag impls, and name-to-value lookup tables | crates.io (independent version line) |

`libmqm-constants` and `libmqm-default` both depend on `libmqm-sys`.

## Repository layout

```
libmqm-sys/
  build/main.rs            build script entry (every crate uses build/main.rs, declared via `build =` in Cargo.toml)
  build/mqi_bindgen.rs     bindgen invocation for the MQ headers
  build/doc_comments.rs    extracts IBM header comments into /// docs (drives boilerplate in MQI structs)
  build/rustify.rs         syn visitors: MQLONG literal wrapping, `p` prefix stripping, path prefixing
  build/mq_trait.rs        generates the traits, LinkedMq impl, dlopen2 impl and mockall impls from the extern fns
  build/ibm_references.csv name,IBM-doc-URL table used to build "References" doc sections
  doc_link_scrape/         python (requests+bs4) script that regenerates ibm_references.csv
  src/c/                   small C shims compiled with `cc` (string lookup tables, struct defaults)
  src/pregen/<arch>-<os>/  committed generated bindings (directories)
libmqm-default/src/pregen/<arch>-<os>-defaults.rs   flat files
libmqm-constants/build/constants/list.rs            hand-maintained constant registry (CONSTANTS, PREFIX_CONSTANTS, CONSTANTS_DOC)
libmqm-constants/src/pregen/<arch>-<os>-{mapping,new_types}.rs   flat files
```

Pregen naming differs per crate: `libmqm-sys` uses directories, the other two use
flat files with a `-mapping` / `-new_types` / `-defaults` suffix. macOS is normalised to
`any-macos` because a single client covers both architectures. The target arch for
`powerpc64le` is spelled `powerpc64`.

## Prerequisites

An IBM MQ client or server install is **required** to build or test anything:

- `MQ_HOME` must point at the install (default `/opt/mqm`, or `c:/Program Files/IBM/MQ`).
  It is read by the build script and used to locate headers (`inc/` or `tools/c/include`).
- The MQ shared library must be discoverable at runtime for the `dlopen2` tests
  (`libmqm_r.so` / `mqm.dll`); on Linux set `LD_LIBRARY_PATH` to `$MQ_HOME/lib64`.
- Header parsing needs `libclang` (`bindgen`).
- Rust 1.95 minimum (see `rust-version` in the workspace `Cargo.toml`).

A quick local queue manager for experiments: `docker run -d --publish 1414:1414 icr.io/ibm-messaging/mq:latest`.

## Commands

There is no Makefile; everything is cargo. CI is the authoritative source for the exact
invocations (`.github/workflows/ci.yml`).

```sh
# build / check
cargo check --locked --all-features
cargo build --no-default-features --features pregen        # regenerates and REWRITES src/pregen (see below)

# test (needs a client install; MQ_HOME + LD_LIBRARY_PATH)
cargo nextest run --profile ci --features mqc_9_3_5_0,pcf,exits,mqai,dlopen2
cargo nextest run --locked --profile ci --all-features

# format / lint (CI runs these with RUSTFLAGS=-Dwarnings)
cargo fmt -- --check
cargo hack -p libmqm-sys --each-feature --include-features mqai,pcf,exits,dlopen2,link,mqc_latest,mock,bindgen --features bindgen clippy --tests
cargo hack -p libmqm-sys --feature-powerset --depth 2 --include-features mqai,pcf,exits,dlopen2,link,mqc_latest,mock,default clippy --tests
cargo hack -p libmqm-constants --feature-powerset --depth 2 --include-features mqai,pcf,exits,mqc_9_4_5_0,default,generate clippy --tests
cargo hack -p libmqm-default --feature-powerset --depth 2 --include-features pcf,exits,mqc_9_3_1_0,default,generate clippy --tests

# docs (matches docs.rs build)
cargo docs-rs -p libmqm-sys --target x86_64-unknown-linux-gnu
```

Feature combinations are combinatorial and matter: every `clippy` variant above must pass,
which is why generated modules carry blanket `#![allow(...)]` prologues rather than
per-item lint fixes.

## Generated code: the most important workflow

At build time the bindings come from one of two sources, selected by the `bindgen` feature:

- `bindgen` **enabled** (`pregen` implies it): `bindgen` + syn visitors parse the MQ headers
  from `MQ_HOME` and generate the bindings into `OUT_DIR`. With `pregen` also enabled, the
  same generated files are copied back over `src/pregen/**`. This is a **local write to the
  working tree**.
- `bindgen` disabled (the default): the committed `src/pregen/**` files are `include!`d
  instead. This is the path used by normal builds, tests and downstream users.

Consequences an agent must respect:

1. **Never hand-edit anything under `src/pregen/`.** Any manual change is silently
   overwritten the next time someone regenerates, and CI fails if the committed files do
   not match what a fresh generation produces.
2. Regenerating requires the MQ client version pinned in `.github/mq_version_pregen.json`,
   and each platform/arch must be regenerated *on that platform* (the generated code
   encodes pointer widths, endianness and platform `#define`s). CI does this in the
   `pregen` (ubuntu x64/arm, macOS, Windows) and `pregen_arch` (s390x, ppc64le under
   emulation) jobs and then asserts `git diff --stat --exit-code libmqm-sys/src libmqm-default/src libmqm-constants/src`.
   On a PR you can trigger this via the `/pregen` slash command, which pushes commits
   containing the refreshed files.
3. After changing the *generator* (anything in `build/` or `libmqm-constants/build/`), the
   pregen files in the repo are stale until regenerated for every platform. Small generator
   changes therefore produce large diffs.
4. Generated files start with a `/* Generated with MQ client version <x> */` banner. If the
   header shows an older client version than `.github/mq_version_pregen.json`, the files
   need a pregen run.

To bump the MQ client version used for everything (including pregen), edit only
`.github/mq_version_pregen.json` (keys are `runner.os` + `runner.arch` concatenated, e.g.
`LinuxPPC64LE`) and then run `/pregen`.

The build script asserts that the highest enabled `mqc_*` feature does not exceed the
version of the installed client (it reads `README.Redist` / `README.Client` /
`README.Advanced`). Enabling `mqc_latest` against an older client fails the build with a
clear message.

## Adding or changing constants (`libmqm-constants`)

The constant registry is **hand-maintained** in `libmqm-constants/build/constants/list.rs`
(3400+ lines) and drives the generated typed constants:

- `CONSTANTS`: entries with a `_STR` helper from the C shim (`src/c/strings.c` exposes
  `MQI_BY_NAME_STR` / `MQI_BY_VALUE_STR` through `libmqm_sys::str`), i.e. constants whose
  names are known to the MQ client library. Each entry declares `prefix`, `new_type`,
  `orig_type`, `usage`, an optional `feature` gate and an optional doc string.
- `PREFIX_CONSTANTS`: fallback for prefixes with no `_STR` function; any constant matching
  the prefix (that is not claimed by a longer prefix) is swept in as an "extra".
- `CONSTANTS_DOC`: extra documentation keyed by exact constant name.
- `const_ignore_regex()` in `build/constants/generate.rs` filters constants that should not
  be mapped (lengths, versions, ranges, `_MASK` names etc.).
- Any constant that ends up unmapped is emitted as a comment block at the top of the
  generated `new_types` pregen file - check there first if a constant is "missing".

Lookup tables are chosen by size: linear search for <=63 entries, binary search <=255,
otherwise a phf perfect hash. `usage` selects one of the `impl_value!` / `impl_bitflags!`
macros from `src/value.rs` / `src/bitflags.rs`.

## Feature flags

```
libmqm-sys:   default = ["link", "mqc_9_3_0_0"]
              link      -> link_mqm + link_api (link libmqm_r / mqm at compile time)
              dlopen2   -> runtime loading via dlopen2 (MqWrapper)
              mqai, pcf, exits  -> expose those binding modules
              bindgen   -> generate instead of using pregen
              constant_lookup / struct_defaults / versiongen  -> internal build switches
              pregen    -> pcf + exits + mqai + bindgen + mqc_latest (writes into src/)
              mock      -> mockall-generated Mq
              docsrs    -> everything, for docs.rs
```

`mqc_*` features form a chain: each implies the previous (`mqc_9_3_1_0` -> `mqc_9_3_0_0` ...),
and `mqc_latest` currently resolves to `mqc_9_4_5_0`. `libmqm-default` and
`libmqm-constants` have their own `mqc_*` features that forward to `libmqm-sys`.

Note that `libmqm-default` caps at `mqc_9_3_1_0` / `libmqm-constants` at `mqc_9_4_5_0` for
their *pregen* sets - the versions are intentionally different per crate (the defaults
changed least recently).

## Code style and conventions

- Formatting is enforced: `rustfmt.toml` sets `max_width = 130`, Unix newlines, edition 2021,
  `use_field_init_shorthand = true`. Run `cargo fmt`.
- Workspace lints: `clippy::all` is **deny**, `nursery` and `pedantic` are warn,
  `similar_names` allowed, `unsafe_op_in_unsafe_fn` warn, `rust_2024_compatibility` warn.
  CI compiles with `RUSTFLAGS=-Dwarnings`, so warnings are effectively errors.
- MQ identifiers are preserved verbatim (`non_snake_case`, `non_camel_case_types`,
  `unreadable_literal` are allowed in generated modules). Do not "fix" them.
- Generated modules are wrapped in `#[rustfmt::skip]` plus an `#![allow(...)]` prologue and
  are pulled in via `#[path = ...]`/`include!`. Follow the same pattern when re-exporting
  (`src/lib.rs`, `src/pregen/mod.rs` are the templates).
- In generated traits, C `p` argument prefixes are stripped (`pHconn` -> `hconn`) and
  `CompCode`/`Reason` are rewritten to `&mut MQLONG`. The rewrite tables live in
  `libmqm-sys/build/main.rs` (`FnArgType::replace_all` / `replace_fns`); extend them there
  rather than patching generated output.
- `MQPMR` is special-cased in doc-link rewriting because it is not a real struct.
- Public items need docs; `cargo docs-rs` runs with `RUSTDOCFLAGS=-Dwarnings` on nightly.
- When bumping `rust-version` in the workspace `Cargo.toml`, also update the `minimum` job in
  `.github/workflows/ci.yml` (the comment on the `rust-version` line says so).

## Testing

- There is no `tests/` directory. Tests are inline `#[cfg(test)] mod tests` blocks, usually
  tagged `#[cfg_attr(coverage_nightly, coverage(off))]`. `cfg(coverage)` /
  `cfg(coverage_nightly)` are declared in the workspace lint `check-cfg` list - use those
  names for any new coverage attributes.
- Runner is `cargo-nextest`; the `ci` profile is defined in `.config/nextest.toml` and only
  sets `fail-fast = false` (note the `.config/`, not `.cargo/`).
- Several tests genuinely need the MQ client library at runtime
  (`libmqm-sys/src/dlopen2.rs` loads `libmqm_r` and issues an `MQCONN`; the connection result
  codes are intentionally not asserted, so no queue manager is needed, just the library).
- Unit tests around the lookup/bitflag machinery (`libmqm-constants/src/value.rs`,
  `lookup.rs`, `bitflags.rs`, `impl_types.rs`) are self-contained and encode the expected
  `Display`/`Debug` formats - update them deliberately if you change the formats.

## CI, releases and repo workflow

- PRs target **`develop`** (that is also where the release workflow runs). `Cargo.lock` is
  gitignored and not committed; the `minimum` job regenerates it with `-Z minimal-versions`
  and checks on Rust 1.95.
- Workflow files are checked by **zizmor**; third-party actions must be pinned (SHAs, with a
  policy allow-list in `.github/zizmor.yml`). Keep new actions pinned.
- Slash commands on a PR: `/pregen` (regenerate platform bindings and commit them to the PR)
  and `/semver` (cargo-semver-checks).
- **release-plz** drives versioning and `CHANGELOG.md`, triggered on push to `develop`.
  `release-plz.toml` puts `libmqm-sys` and `libmqm-default` in the same `version_group`
  ("main", so they move together) and keeps `libmqm-constants` on its own version line
  (currently 0.8.x vs 0.10.x). `CHANGELOG.md` is only written for `libmqm-sys` but includes
  entries from the other two. Do not hand-edit versions or the changelog; write conventional
  commits and let the release PR do it.
- Dependabot runs weekly for cargo and GitHub Actions (4-day cooldown).

## Local environment notes

- This checkout is a **colocated Git + Jujutsu repo** (both `.git/` and `.jj/` exist, with
  `jj`'s git backend pointing at `.git`). `jj` commands work here; pick one tool for
  history edits and stay consistent so the two views do not diverge.
- The `C` shims under `libmqm-sys/src/c/` are only compiled when `constant_lookup` or
  `struct_defaults` is enabled. They exist to expose the MQ macro defaults and the
  `_STR` name tables to Rust; they are not part of the public API and should not be
  referenced from `src/`.
