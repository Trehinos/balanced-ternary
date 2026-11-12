# Contribution Guidelines

This crate provides utilities for working with balanced ternary numbers and aims to be fully usable in `no_std` environments. The project exposes several modules:

- **digit** – core `Digit` type and operations.
- **concepts** – `DigitOperate` trait for acting on collections of digits.
- **operations** – arithmetic and logic implementations for the `Ternary` type.
- **tryte** – fixed size balanced ternary numbers.
- **store** – compact storage types (`TritsChunk`, `DataTernary`, `Ter40`).
- **conversions** – `From`/`Into` implementations for common types.

All of these reside under `src/` and are organized as individual modules imported by `lib.rs`.

The crate and all dependencies must remain usable with `#![no_std]`; avoid adding anything that would pull in `std` by default.

Before merging new code, ensure that:

1. Every public function has an accompanying test.
2. Formatting checks pass.
3. Builds, unit tests, integration tests, and doctests pass for every
   effective feature combination in debug and release.
4. The library builds for `thumbv7em-none-eabi` without `std`.
5. Documentation builds without warnings.
6. Clippy diagnostics are reviewed.

## Arithmetic semantics

The crate distinguishes numeric values from fixed-width
ternary words:

- `Digit` arithmetic is performed modulo 3.
- `Ternary` supports arbitrary-precision arithmetic and unbalanced
  ternary conversions. Conversions to `i64` remain range-limited.
- `Ter40` is a bounded signed numeric type whose arithmetic
  operators panic on overflow.
- `Tryte<N>` is a fixed-width word. Addition, subtraction
  and multiplication wrap modulo 3^N.
- `Tryte<N>` division uses signed integer division.
- Modular division is available through explicit methods.

Do not silently change these semantics when refactoring.

Preserve the public `DigitOperate` trait signatures.
Use additive APIs to expose additional information such
as outgoing carry.

Maintain `#![no_std]` compatibility and avoid unnecessary
heap allocation in fixed-width arithmetic.
