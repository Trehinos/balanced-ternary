# balanced-ternary — Remaining Work Only

## Priority overview

| ID  | Priority                 | Outstanding task                                                                                       | Primary files                                                      |
|-----|--------------------------|--------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------|
| R01 | done                     | ~~Repair `Digit::sub`: the current source still produces incorrect results~~                           | `src/digit.rs`                                                     |
| R02 | done                     | ~~Repair `checked_to_dec()` for `i64::MIN`: intermediate `i64` overflow~~                              | `src/lib.rs`                                                       |
| R03 | done                     | ~~Remove `i64` conversion from `Ternary::trim()`~~                                                     | `src/lib.rs`                                                       |
| R04 | done                     | ~~Enforce `Ter40` range and define fixed-width overflow behavior~~                                     | `src/store.rs`                                                     |
| R05 | done                     | ~~Standardize overflow, carry, and checked constructors for `Tryte<N>` / `Ter40`~~                     | `src/tryte.rs`, `src/store.rs`                                     |
| R06 | done                     | ~~Implement arbitrary-precision arithmetic directly on `Ternary` digits~~                              | `src/operations.rs`, `src/lib.rs`                                  |
| R07 | done                     | ~~Eliminate temporary `Ternary` and `String` objects in remaining fixed-width methods and formatting~~ | `src/tryte.rs`, `src/store.rs`, `src/lib.rs`                       |
| R08 | done                     | ~~Normalize conversion and parsing error behavior, including padding/empty input~~                     | `src/lib.rs`, `src/tryte.rs`, `src/store.rs`, `src/conversions.rs` |
| R09 | cancelled                | ~~Introduce arithmetic directly on `TritsChunk` / `DataTernary` in balanced base 243~~                 | `src/store.rs`                                                     |
| R10 | done                     | ~~Decide numeric-value versus representation equality for `DataTernary`~~                              | `src/store.rs`                                                     |
| R11 | done                     | ~~Expand boundary/property tests and CI feature matrix~~                                               | tests, `.github/workflows/rust.yml`                                |
| R12 | **P2 — release quality** | Benchmark, documentation consistency, release metadata, optional API extensions                        | docs, benchmarks, `Cargo.toml`, `.changelog/`                      |

---

## R05 — Define fixed-width overflow and carry APIs [P1]

### Semantics

- `Tryte<N>` addition, subtraction and multiplication wrap
  modulo 3^N.
- `/` performs signed integer division, not modular division.
- Modular division requires an invertible divisor.
- A divisor is invertible modulo 3^N iff it is not divisible by 3
  (for N > 0).
- `Tryte<0>` remains supported as an empty word.

### Deferred work

- Normalize conversions and edge cases for `Tryte<0>` (R08).
- Improve performance of remaining conversion-based methods (R07).
- Extend CI coverage and property tests (R11).

## R06 — Replace `i64`-based arithmetic in `Ternary` [P1]

done

## R07 — Eliminate Unnecessary Allocations and Conversions [DONE]

**Sources:** `src/tryte.rs`, `src/store.rs`, `src/lib.rs`.

### Completed

- Replaced intermediate `Ternary` conversions in
  `Tryte<N>` numeric conversions and bitwise operators.
- Implemented allocation-free formatting for `Tryte<N>`.
- Optimized `Ter40` digit access and decoding.
- Removed intermediate allocations from `Ter40` formatting.
- Optimized `Ternary::get_digit()` to O(1).
- Improved allocation efficiency in `Ternary` parsing,
  padding, concatenation, and formatting.
- Implemented direct five-trit decoding for `TritsChunk`.
- Implemented direct balanced base-243 encoding and
  decoding for `DataTernary`.
- Optimized `DataTernary` numeric conversions and formatting.

### Preserved Invariants

- Public API signatures remain unchanged.
- Fixed-width types preserve their exact width.
- Leading zeros and raw digit sequences are preserved
  where required.
- Normalized conversions produce canonical numeric
  representations.
- Empty storage behavior remains unchanged.
- `#![no_std]` compatibility is preserved.

### Remaining Considerations

- Benchmark allocation and formatting improvements (R12).

## R08 — Clarify conversions, parsing, and zero-width representation [DONE]

- Balanced parsing explicitly preserves empty input and leading zeros.
- Fallible parsing reports invalid characters and UTF-8 byte offsets.
- Bounded numeric conversions ignore redundant leading zeros and
  return errors when values do not fit.
- `Tryte<N>` no longer has a 40-trit width limit.
- `Tryte<0>` represents only zero; conversions reject nonzero values.
- Fallible conversions to `i64` report out-of-range values.
- Explicit panicking convenience constructors remain available.
- Unbalanced ternary conversions support arbitrary precision,
  optional ASCII signs, and normalized numeric output.
- Empty unbalanced input and signs without digits are rejected.
- Empty `DataTernary` storage remains valid. Fixed representations
  preserve stored chunks; numeric conversions normalize zero.
- `Ternary` equality, ordering, and hashing use numeric values.
  Exact representations remain accessible through `to_digit_slice()`.

The choice between structural and numeric equality for `DataTernary`
remains tracked separately in R10.

## R09 — Add arithmetic to compact `DataTernary` storage [P2]

cancelled

## R10 — Define DataTernary storage semantics [DONE]

- `DataTernary` provides compact storage and conversions, not arithmetic.
- `PartialEq`, `Eq`, and `Hash` retain structural semantics over the
  complete stored chunk sequence.
- Leading zero chunks and stored width participate in equality.
- Empty storage differs from a stored zero chunk.
- Removed `PartialOrd` and `Ord`; no numeric comparator was added.
- Documented hash collection behavior and the ordering API break.
- Added equality and hash collection regression tests, plus
  compile-fail doctests for unsupported ordering.

## R11 — Test matrix, properties, and CI [P1]

**Sources:** library tests and [
`.github/workflows/rust.yml`](https://github.com/Trehinos/balanced-ternary/blob/28614306f09289ddfaa19e6d7732ff452d97faf5/.github/workflows/rust.yml).

The workflow only runs `cargo build --verbose` and `cargo test --verbose` for default features.
Expand it to exercise feature combinations and verify `no_std` compatibility.

**Suggested local commands:**

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo test --no-default-features
cargo test --no-default-features --features ternary-string
cargo test --no-default-features --features tryte
cargo test --no-default-features --features ternary-store
cargo build --no-default-features
cargo doc --no-deps --all-features
```

**Required test families:**

1. Exhaustive `Digit` arithmetic (nine inputs for each binary operation; check the intended
   three-valued logic truth tables separately).
2. Full signed-64-bit round trips at both boundaries, with many random samples.
3. Property-based `Ternary` arithmetic once direct arithmetic is implemented (identities, signs,
   carries, division/remainder contract).
4. `Ternary` `Eq`/`Ord`/`Hash` consistency, including `HashSet` and `BTreeSet`, leading zeros, and
   very long representations; the current tests cover equality/order but do not exercise
   hash-collection consistency.
5. Fixed-width overflow boundaries for `Tryte<1>`, `<2>`, `<6>`, `<40>` and `Ter40`; explicit tests
   for `Tryte<0>` if supported.
6. `DataTernary` round trips for arbitrary blocks, all-zero chunks, prefix padding, and empty
   storage.
7. Noncommutative `each_zip` callbacks, retained outgoing carry on `Ternary`, and documented
   discarded carry on fixed types; the current suite already contains targeted tests but not broad
   property tests.
8. A test build for targets without `std` where available (the crate's `no_std` declaration alone
   does not prove compatibility with every target/allocator setup).

**Acceptance:** Every enabled feature combination builds; tests pass under debug and release; CI
guards the chosen semantics. Treat clippy output as a review input rather than changing public
behavior to silence warnings.

## R12 — Benchmark, documentation, and release hygiene [P2]

- Add benchmarks for parsing, formatting, conversions, `Tryte` digitwise operations, `Ter40`
  extraction, `DataTernary` encoding/decoding, and increasingly wide arithmetic. Compare median
  runtime and allocation counts; avoid declaring optimization wins without measurements.
- If using `criterion`, keep it a dev dependency so the published library's default dependency/
  `no_std` constraints remain unchanged.
- Review API documentation for semantics of balanced versus ordinary ternary, fixed-width carry
  loss, numerical versus structural equality, width-preserving versus normalized formatting, and
  checked/wrapping operations.
- Update the version metadata consistently. At the reviewed revision, `Cargo.toml` still declares
  `2.1.0`, while `.changelog/v2.1.1.md` exists but its first heading incorrectly says
  `# Version 2.1.0`. Do not claim a release has been published simply because a changelog exists.
- Update `README.md` examples and `AGENTS.md` to match chosen policies; consider MSRV and SemVer
  implications of changing constructors, equality, or operator behavior.
- Avoid introducing a `BoundedTernary` trait until repeated concrete implementations show a stable
  shared API.

---

## Recommended implementation sequence

1. ~~**Correctness hotfix:** R01, R02, R03; add regression tests in the same commit(s). Explicitly
   test `i64::MIN` and long `trim()`.~~
2. ~~**Fixed-width safety:** R04, then decide R05 before changing operator overloads. Preserve or
   intentionally break compatibility with a documented release plan.~~
3. ~~**Arithmetic core:** R06. Start with add/sub and property tests; implement mul and div/rem
   separately.~~
4. **Performance cleanup:** R07 with benchmarks rather than making speculative micro-optimizations.
5. **Conversion semantics:** R08, with edge-case tests and clear error types.
6. **Compact arithmetic and value semantics:** R09–R10, separately from bug fixes.
7. **Continuous verification and release:** R11–R12 should be applied progressively, not deferred
   until the end.

### Working rules

- Keep `no_std + alloc` support and avoid unnecessary dependencies.
- Distinguish **trit sequence** behavior from **numeric value** behavior; preserve width where the
  API promises it.
- Do not conflate `i64` overflow with `N`-trit overflow.
- For any numeric type, keep `Eq`, `Hash`, and `Ord` coherent.
- Write regression tests before changing logic; run a complete feature matrix after changes.
- Do not redo the merged `each*` changes unless a new failing test demonstrates a defect.
- Prefer small, reviewable commits and document observable breaking changes.
