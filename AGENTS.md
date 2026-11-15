# Contribution guidelines

## Library contracts

- Keep the library and its runtime dependencies compatible with `no_std` and
  `alloc`. Host-side tests and benchmarks may use `std` and dev-dependencies.
- Preserve the numeric and storage semantics described in the
  [type overview](README.md#getting-started) and each type's rustdoc.
  Test exact digits or chunks whenever an operation promises to preserve width.
- Preserve the public signatures of `concepts::DigitOperate`. Expose additional
  results such as outgoing carry through additive methods.
- Avoid unnecessary heap allocation in fixed-width arithmetic.

## Before merging

1. Cover each public function with a test.
2. Check formatting and review Clippy diagnostics.
3. Build and run unit tests, integration tests, and doctests for every effective
   feature combination in debug and release.
4. Build the library for `thumbv7em-none-eabi` across those feature combinations.
5. Build the API reference with warnings treated as errors.

The [CI workflow](.github/workflows/rust.yml) contains the exact commands.
