# Security policy

## Supported versions

These crates are consumed from crates.io or by git ref, so what you pin decides what you run:

- The latest published release of each crate and the `develop` branch get fixes.
- Older releases are not patched. Move to a newer version to pick up a fix.
- The `mqc_*` features select the oldest IBM MQ client you build against, so a release is only
  tested against the newest client it supports.

## Reporting a vulnerability

Please report vulnerabilities privately rather than in a public issue: open the **Security** tab of
this repository and choose **Report a vulnerability**, which creates a private advisory that only
you and the maintainers can see.

Useful details to include:

- the crate and its version, or the git ref you pinned,
- your target triple and the IBM MQ client you built against (the version reported by
  `README.Redist` in your install),
- the cargo features you enabled, and whether the bindings came from the committed `src/pregen`
  files or from a `bindgen` build,
- what an attacker would gain, and the steps to reproduce it - a minimal reproduction, ideally a
  single `unsafe` call and the structures it passes, is worth more than a description.

We will investigate, ask for anything we are missing and keep you updated on the outcome. Tell us
if you would like to be credited in the advisory or release notes.

## Scope

In scope:

- the hand-written sources, the build scripts and the generators - `libmqm-sys/build`,
  `libmqm-constants/build`, the C shims in `libmqm-sys/src/c`, and the runtime loading in
  `libmqm-sys/src/dlopen2.rs` and `libmqm-sys/src/link.rs`,
- the committed bindings under `src/pregen`: a structure layout, field offset, function signature
  or constant that would make the `unsafe` API unsound, or that does not match the IBM MQ client
  version it claims to have been generated from,
- the workflows in `.github/workflows`, for example command injection, an unpinned third-party
  action, or generated files a pull request can change without CI noticing.

Out of scope:

- vulnerabilities in IBM MQ itself, in the MQ client installs this crate is built against, or in the
  MQ C libraries it binds to - report those to IBM,
- the correctness of *your* `unsafe` code: this crate deliberately exposes the raw MQI, MQAI and
  PCF functions, so calling them correctly is the caller's responsibility,
- the safe API wrapper in [`mqi`](https://github.com/advantic-au/mqi), which has its own policy,
- vulnerabilities in `bindgen`, `syn`, `dlopen2` and the other dependencies - report those
  upstream,
- known issues in an old MQ client version that is still published and installable.
