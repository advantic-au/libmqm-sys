# Contributing

Thanks for taking the time to contribute. All feedback, suggestions, enhancements and fixes are
welcome.

## Prerequisites

An IBM MQ client or server install is required to build or test anything:

- Set `MQ_HOME` to the install (defaults are `/opt/mqm` and `c:/Program Files/IBM/MQ`). The build
  script reads `README.Redist`, `README.Client` or `README.Advanced` from it, and fails if the
  highest enabled `mqc_*` feature is newer than the installed client.
- Header parsing needs `libclang` (`bindgen`).
- The `dlopen2` tests load the MQ shared library at runtime; on Linux set
  `LD_LIBRARY_PATH=$MQ_HOME/lib64`.
- Rust 1.95 or newer (`rust-version` in the root `Cargo.toml`). `cargo-nextest` and `cargo-hack`
  are needed to reproduce the CI invocations below.

For a queue manager to experiment against:

```sh
docker run -d --publish 1414:1414 icr.io/ibm-messaging/mq:latest
```

## Build, test and lint

```sh
cargo check --locked --all-features

# tests (replace nextest with `cargo test` if you do not have it)
cargo nextest run --profile ci --features mqc_9_3_5_0,pcf,exits,mqai,dlopen2

cargo fmt -- --check

# the clippy matrix CI runs (RUSTFLAGS=-Dwarnings, so warnings fail the build)
cargo hack -p libmqm-sys --each-feature --include-features mqai,pcf,exits,dlopen2,link,mqc_latest,mock,bindgen --features bindgen clippy --tests
cargo hack -p libmqm-sys --feature-powerset --depth 2 --include-features mqai,pcf,exits,dlopen2,link,mqc_latest,mock,default clippy --tests
cargo hack -p libmqm-constants --feature-powerset --depth 2 --include-features mqai,pcf,exits,mqc_10_0_1_0,default,generate clippy --tests
cargo hack -p libmqm-default --feature-powerset --depth 2 --include-features pcf,exits,mqc_9_3_1_0,default,generate clippy --tests

# docs, matching docs.rs
cargo docs-rs -p libmqm-sys --target x86_64-unknown-linux-gnu
```

Feature combinations matter: the clippy lines above are the ones CI gates on, and `.github/workflows/ci.yml`
is the authoritative list.

## Generated code

Almost the whole public API is generated from IBM's C headers and **committed** under `src/pregen`.
The build scripts under `libmqm-*/build` are the source of truth - if generated output is wrong,
fix the generator, not the output.

- **Never hand-edit `src/pregen`.** Regeneration overwrites it, and CI fails when the committed
  files do not match a fresh generation for that platform.
- Regenerate with:

  ```sh
  MQ_HOME=/path/to/mq/client cargo build --no-default-features --features pregen
  ```

  The output encodes pointer width, endianness and platform defines, so each platform must be
  regenerated on that platform. Comment `/pregen` on a pull request to have CI regenerate all of
  them and commit the result to the PR.
- To move to a new MQ client, edit `.github/mq_version_pregen.json` (keys are `runner.os` and
  `runner.arch` concatenated, for example `LinuxX64`) and run `/pregen`. Then add the matching
  `mqc_*` feature if the release needs one.
- The `mqc_*` features form a chain, each implying the previous, and `mqc_latest` resolves to the
  newest. A version feature gates what the constants generator emits, not the emitted items.
- New constants are registered by hand in `libmqm-constants/build/constants/list.rs`
  (`CONSTANTS`, `PREFIX_CONSTANTS`, `CONSTANTS_DOC`). Anything that ends up unmapped is listed in a
  comment at the top of the generated `new_types` file - check there first when a constant looks
  missing.

Regenerating with a client version that differs from `.github/mq_version_pregen.json` will fail CI
later, so keep them in step.

## Commits, releases and pull requests

- Pull requests target **`develop`**.
- Write conventional commits: `release-plz` derives versions and `CHANGELOG.md` from them. `feat:`
  and `fix:` bump, `!` marks a breaking change (raising the MSRV is one), and other types such as
  `chore:` do not. Do not hand-edit versions or the changelog.
- When you change `rust-version`, update the `minimum` job in `.github/workflows/ci.yml` as well -
  it installs that toolchain and re-resolves dependencies with `-Z minimal-versions`.
- Workflow changes are checked by `zizmor`; keep third-party actions SHA-pinned (the policy
  allow-list is `.github/zizmor.yml`) and prefer GitHub's `$/...` self-repository syntax over
  `./...` for in-repo actions.
- `/semver` on a pull request runs `cargo-semver-checks`.

## Style

- `cargo fmt` with the settings in `rustfmt.toml` (max width 130, Unix newlines, edition 2024).
- clippy: `all` is denied, `nursery` and `pedantic` are warnings, and CI compiles with
  `RUSTFLAGS=-Dwarnings`, so warnings have to be fixed rather than tolerated.
- MQ identifiers are kept exactly as IBM spells them (`non_snake_case`, `non_camel_case_types` and
  `unreadable_literal` are allowed in generated modules). Do not rename them.
- Generated modules carry blanket `#![allow(...)]` prologues; leave them alone instead of fixing
  lints item by item.
- This crate exposes the raw `unsafe` API on purpose. A safe API layer belongs in
  [`mqi`](https://github.com/advantic-au/mqi), not here.

## Reporting problems

- Bugs and feature requests: use the issue templates.
- Security issues: report them privately, see [SECURITY.md](SECURITY.md).
