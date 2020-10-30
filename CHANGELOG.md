# ChangeLog

## [Unreleased]

## [v0.1.6] (2020-10-30)
[Full ChangeLog for v0.1.6]

### Fixed Bugs

- Avoid trigger [a rustc bug](https://github.com/rust-lang/rust/issues/75533) which causes errors when build with target `wasm32-unknown-unknown` ([#54]).

## [v0.1.5] (2020-10-30)
[Full ChangeLog for v0.1.5]

### Major Updates

- Use features to let all structs be optional.
- Should NOT consider hashes as numbers.
  - Add methods to replace numerical methods for hashes.
    - `zero() -> empty()`
    - `is_zero() -> is_empty()`
    - `is_max() -> is_full()`
  - **BREAKING CHANGE** Remove `checked_neg(..)` for hashes.
- Upgrade dependencies.
  - `quote v0.6 -> v1.0`
  - `syn v0.15 -> v1.0`
  - `proc-macro2 v0.4 -> v1.0`
  - Remove dependencies "proc-macro-hack" for crates in the workspace.
    - Update the minimum supported version of rust toolchain to `1.45.0` for the feature: [Stabilizing function-like procedural macros in expressions, patterns, and statements](https://blog.rust-lang.org/2020/07/16/Rust-1.45.0.html#stabilizing-function-like-procedural-macros-in-expressions-patterns-and-statements).
  - Use `thiserror` instead of `failure` since `failure` is officially deprecated.

### Fixed Bugs

- Constructors can not handle short inputs correctly ([#42]).
- `as_fixed_bytes_mut(&mut self)` return an immutable reference ([#42]).

## [v0.1.4] (2019-04-03)
[Full ChangeLog for v0.1.4]

### Major Updates

- Export errors in the main crates.

## [v0.1.3] (2019-04-02)
[Full ChangeLog for v0.1.3]

### Major Updates

- Add a script to run rustfmt for code in `quote!()` ([#22]).
- Add readable constructors via proc-macro ([#33]).

### Fixed Bugs

- Hangs when performing the division and remainder operations ([#34]).
- Overflows when performing the division and remainder operations ([#36]).

### Known Issues

- Errors are not re-exported in the main crates.
  But they still can be imported from the internal crates.

## [v0.1.2] (2019-12-18)
[Full ChangeLog for v0.1.2]

### Major Updates

- Fix dependencies in Cargo.toml.

## [v0.1.1] (2018-12-14) - _Yanked!_
[Full ChangeLog for v0.1.1]

### Major Updates

- Switch to [Rust 2018](https://doc.rust-lang.org/edition-guide/rust-2018/index.html) ([#13]).

## [v0.1.0] (2018-12-07)
[Full ChangeLog for v0.1.0]

### Major Updates

- Release the first version of [Rust-NumExt] -- Libraries to extend the rust built-in numeric types.

[Rust-NumExt]: https://github.com/cryptape/rust-numext
[Unreleased]: https://github.com/cryptape/rust-numext/compare/v0.1.6...HEAD
[v0.1.6]: https://github.com/cryptape/rust-numext/tree/v0.1.6
[v0.1.5]: https://github.com/cryptape/rust-numext/tree/v0.1.5
[v0.1.4]: https://github.com/cryptape/rust-numext/tree/v0.1.4
[v0.1.3]: https://github.com/cryptape/rust-numext/tree/v0.1.3
[v0.1.2]: https://github.com/cryptape/rust-numext/tree/v0.1.2
[v0.1.1]: https://github.com/cryptape/rust-numext/tree/v0.1.1
[v0.1.0]: https://github.com/cryptape/rust-numext/tree/v0.1.0
[Full ChangeLog for v0.1.6]: https://github.com/cryptape/rust-numext/compare/v0.1.5...v0.1.6
[Full ChangeLog for v0.1.5]: https://github.com/cryptape/rust-numext/compare/v0.1.4...v0.1.5
[Full ChangeLog for v0.1.4]: https://github.com/cryptape/rust-numext/compare/v0.1.3...v0.1.4
[Full ChangeLog for v0.1.3]: https://github.com/cryptape/rust-numext/compare/v0.1.2...v0.1.3
[Full ChangeLog for v0.1.2]: https://github.com/cryptape/rust-numext/compare/v0.1.1...v0.1.2
[Full ChangeLog for v0.1.1]: https://github.com/cryptape/rust-numext/compare/v0.1.0...v0.1.1
[Full ChangeLog for v0.1.0]: https://github.com/cryptape/rust-numext/compare/900bf95d2df3e92b4a352a8e01ced355805ea0b6...v0.1.0
[#54]: https://github.com/cryptape/rust-numext/pull/54
[#42]: https://github.com/cryptape/rust-numext/pull/42
[#33]: https://github.com/cryptape/rust-numext/pull/33
[#22]: https://github.com/cryptape/rust-numext/pull/22
[#13]: https://github.com/cryptape/rust-numext/pull/13
[#36]: https://github.com/cryptape/rust-numext/issues/36
[#34]: https://github.com/cryptape/rust-numext/issues/34
