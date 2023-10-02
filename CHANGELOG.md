# Extel Changelog
All changes to Extel should be logged here after the release is generated and extel is published to cargo.

## v0.2.1
### Documentation
  - Fixed extel README using `rs` instead of `rust` for code block.

## v0.2.0
### Features
  - cmd! macro now supports Path/PathBuf/OsStr using the cmd!(CMD => [arg1, arg2, ...]) syntax.
  - err! macro maps strings to a TestFailed error.
  - ExtelResult is now a Result<(), extel::errors::Error> type to support error propagation.

### Fixes
  - extel_parameterized no longer throws errors when the function the macro is affecting as a doc comment.

### Documentation
  - Update some docs to include features introduced in v0.1.3 and new features introduced in v0.2.0.

### CI
  - Remove clippy action and instead run clippy manually in a shell through an action.

## v0.1.3
### Fixes
  - Fixed extel_assert missing from extel::prelude

## v0.1.2
### Features
  - Implemented extel_assert to be used in place of pass/fail macros.

## v0.1.1
### Fixes
  - Fixed `Cargo.toml` misconfigured so feature documentation was not being generated correctly for extel_parameterized.

## v0.1.0
Initial release of Extel. Contains a prelude module to export the required base features for using Extel.
