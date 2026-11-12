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
| R08 | **P1 — conversion API**  | Normalize conversion and parsing error behavior, including padding/empty input                         | `src/lib.rs`, `src/tryte.rs`, `src/store.rs`, `src/conversions.rs` |
| R09 | **P2 — architecture**    | Introduce arithmetic directly on `TritsChunk` / `DataTernary` in balanced base 243                     | `src/store.rs`                                                     |
| R10 | **P2 — semantic design** | Decide numeric-value versus representation equality for `DataTernary`                                  | `src/store.rs`                                                     |
| R11 | **P1 — verification**    | Expand boundary/property tests and CI feature matrix                                                   | tests, `.github/workflows/rust.yml`                                |
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
- Review empty representation and conversion semantics (R08).
- Implement arithmetic directly on compact chunks (R09).

## R08 — Clarify conversions, parsing, and zero-width representation [P1]

**Sources:** `src/lib.rs`, `src/tryte.rs`, `src/store.rs`, `src/conversions.rs`.

**Remaining ambiguities / defects:**

- `Ternary::from_str("")` currently succeeds because `.chars().all(...)` returns true for the empty
  string. `Ternary::parse("")` creates an empty vector. This behaves numerically like zero under
  equality but formats as an empty string. Decide whether empty input should be rejected,
  canonicalized to `"0"`, or explicitly accepted as an empty representation; apply consistently to
  `FromStr`, `parse`, and `Display`.
- `Tryte<N>::from_ternary()` rejects values based on the *raw trit count*, not on significant
  length: `"00000+"` cannot become `Tryte<1>` even though its value fits. Decide whether conversion
  should accept leading-zero padding; if yes, check significant digits before enforcing width.
  Similarly review `TritsChunk::from_ternary()`.
- `Tryte<N>::from_str()` validates characters but can still panic on width overflow, despite
  returning `Result`. It needs a meaningful error variant for width/range overflow. The same issue
  arises with other `FromStr`/`From` conversions.
- `Ternary::to_unbalanced()` and `from_unbalanced()` still rely on `i64`; long values fail even if
  they are valid `Ternary` numbers. Either implement arbitrary-length base-3 conversion or
  explicitly expose a checked, bounded variant.
- `Digit::from_char` / `From<char>` panic for invalid characters; consider `try_from_char()` or an
  error-oriented constructor. Replacing the existing `From<char>` with `TryFrom<char>` is a breaking
  change because of the blanket `TryFrom` implementation.
- `DataTernary::default()` contains no chunks. Its fixed-width conversion and `Display` currently
  have zero length, whereas its numeric conversion yields a single `0`. Decide whether empty storage
  is a valid zero-width sequence or should canonicalize to a 5-trit zero chunk.
- Document that `Ternary` equality is numeric but `to_digit_slice()` retains the exact
  representation; consider an explicitly named `same_digits()` helper if users need structural
  comparison.

**Acceptance:** All fallible parsing/conversion APIs return errors instead of panicking on
valid-but-too-wide input; empty and padding cases have tests; arbitrary precision versus bounded
conversion is clearly indicated by names/docs.

## R09 — Add arithmetic to compact `DataTernary` storage [P2]

**Source:** `src/store.rs`.

Five balanced trits have `3^5 = 243` possible values, represented by one signed byte in
`[-121,121]`; the current storage density is already near optimal. The next gain is **performing
operations on chunks without decoding all individual trits**.

Treat chunks as digits in balanced base `243`:

```rust
fn add_chunks(a: i8, b: i8, carry: i16) -> (i8, i16) {
    let sum = a as i16 + b as i16 + carry;
    let next_carry = (sum + 121).div_euclid(243);
    let digit = sum - 243 * next_carry;
    (digit as i8, next_carry)
}
```

**Work plan:** Define chunk order and sign/zero extension; add `checked`/`wrapping` semantics where
appropriate; implement a right-to-left chunk adder (`O(number_of_chunks)`), subtraction by negation,
and normalized output. Test partial leading blocks, internal zero blocks, negative numbers, carries
at chunk boundaries, and 100+ trit inputs. Later consider multiplication directly on chunks. Do not
spend time further compressing five-trit storage before benchmarking.

## R10 — Choose `DataTernary` equality/order semantics [P2]

**Source:** `src/store.rs`.

`DataTernary` still derives `PartialEq`, `Eq`, `Hash`, `PartialOrd`, and `Ord` on its vector of
`TritsChunk`. Unlike the numeric `Ternary` type, this gives **structural** equality and
lexicographic chunk ordering. For example, values created from `"+"` and `"00000+"` can have
different chunk vectors even though they represent the same number. Lexicographic order of signed
chunks is not necessarily numerical order across different lengths or signs.

**Decision required:**

- If `DataTernary` is a *storage sequence*, retain structural `Eq` and `Hash`, but avoid presenting
  its derived `Ord` as numerical ordering; provide an explicitly numeric comparator if needed.
- If it is also a *number*, implement numeric `Eq`/`Hash`/`Ord` consistently by ignoring only
  leading zero chunks; compare signs, significant lengths, and chunks in balanced base 243.

Do not silently switch semantics without documenting compatibility with `HashMap`, `HashSet`, and
`BTreeSet`. Add collection tests for any chosen policy.

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
