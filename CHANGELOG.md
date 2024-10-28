# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.1](https://github.com/advantic-au/libmqm-sys/compare/v0.4.0...v0.4.1) - 2024-10-28

### Other

- github token
- Github Action: release-plz ([#17](https://github.com/advantic-au/libmqm-sys/pull/17))
- minor documentation updates ([#16](https://github.com/advantic-au/libmqm-sys/pull/16))
- MQC version feature flags ([#15](https://github.com/advantic-au/libmqm-sys/pull/15))
- MQ client 9.4.1.0 ([#14](https://github.com/advantic-au/libmqm-sys/pull/14))
- Github Actions: MQ client 9.4.1.0
- Dependabot
- github action: mq-client matrix
- MQ client 9.2 ([#12](https://github.com/advantic-au/libmqm-sys/pull/12))
- MQ client 9.4.0.0 macOS
- MQ client 9.4.0.5 Windows
- Semantic sorting of bindgen and extern merging.
- github actions: pregen: no fail on no change, and windows support
- github action: pregen attempt 2
- github action: pregen fixes
- github action: pregen update pull request
- Github Actions: Add pull-request permission to slash_command_dispatch job
- Github actions: scd react permissions
- github actions: use secrets
- github actions: temporarily remove windows from pregen command
- github actions: First draft of slash commands
- fixed mac detection
- Bump MQ version to 9.4.0.5 for pregen
- treat crlf correctly for cross platform
- mq version detection fixes on windows
- Embed MQ version in generated bindings
- upgrade to 9.4.0.5 for linux pregen
- github actions: ubuntu pregen check MQ_HOME set
- github action: pregen check
- Github Action: Fix clippy
- rust clippy action
- github action: rust-cache
- minimum github action fix
- minimum and clippy github action fixes
- Corrected mq client version parameter
- Simplified workflow
- Remove docsrs build warnings
- Automated pregen builds
- inputs.path -> inputs.download-path
- Further iteration of mqredist download
- Revert "Try using published mq-client-setup"
- Revert "parameter typo"
- Revert "mqc-redist parameter correction"
- Revert "setup-mqclient action typo"
- Revert "3rd attempt at reuse of setup-mqclient"
- Revert "Fixed parameter to mq-data-path"
- Fixed parameter to mq-data-path
- 3rd attempt at reuse of setup-mqclient
- setup-mqclient action typo
- mqc-redist parameter correction
- parameter typo
- Try using published mq-client-setup
- Moved MQC redist download into separate action
- Minor readme fixes
- Minor readme updates and expect reasons.
- Remove warnings when docs-rs is built
- CI pipeline improvements (documents, warnings, efficiency.
- Minimum version of libc to avoid c_void error
- Fixed cache retrieval of MQC redist
- Minor shift of checkout
- MSRV attempt 2
- Check MSRV in github workflow
- Remove mqc tar after download.
- imports_granularity = "Crate" not in stable
- Workflow fixes. 2nd attempt.
- MQC redist download
- Github Actions. Initial attempt.
- Switch to regex-lite
- Explicitly add rust target version for bindgen generated code
- MSRV 1.77 (bindgenn)
- Add cmqcfc.h to mqai feature
- Cleaned up some unused code warnings
- Naming convention change for MqWrapper
- Identifier naming cleanup
