# Contribution Guidelines

This crate provides utilities for working with balanced ternary numbers and aims to be fully usable in `no_std` environments. The project exposes several modules:

- **digit** – core `Digit` type and operations.
- **concepts** – `DigitOperate` trait for acting on collections of digits.
- **operations** – arithmetic and logic implementations for the `Ternary` type.
- **tryte** – fixed size balanced ternary numbers.
- **store** – compact storage types (`TritsChunk`, `DataTernary`, `Ter40`).
- **conversions** – `From`/`Into` implementations for common types.

All of these reside under `src/` and are organised as individual modules imported by `lib.rs`.

The crate and all dependencies must remain usable with `#![no_std]`; avoid adding anything that would pull in `std` by default.

Before merging new code, ensure that:

1. Every public function has an accompanying unit test.
2. `cargo build` and `cargo test` run successfully. These steps mirror the CI defined in `.github/workflows/rust.yml`.

