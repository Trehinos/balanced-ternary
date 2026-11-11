# balanced-ternary — Remaining Work Only

## Priority overview

| ID  | Priority                 | Outstanding task                                                                                   | Primary files                                                      |
|-----|--------------------------|----------------------------------------------------------------------------------------------------|--------------------------------------------------------------------|
| R01 | done                     | Repair `Digit::sub`: the current source still produces incorrect results                           | `src/digit.rs`                                                     |
| R02 | **P0 — correctness**     | Repair `checked_to_dec()` for `i64::MIN`: intermediate `i64` overflow                              | `src/lib.rs`                                                       |
| R03 | **P0 — correctness**     | Remove `i64` conversion from `Ternary::trim()`                                                     | `src/lib.rs`                                                       |
| R04 | **P0 — invariants**      | Enforce `Ter40` range and define fixed-width overflow behavior                                     | `src/store.rs`                                                     |
| R05 | **P1 — semantics**       | Standardize overflow, carry, and checked constructors for `Tryte<N>` / `Ter40`                     | `src/tryte.rs`, `src/store.rs`                                     |
| R06 | **P1 — architecture**    | Implement arbitrary-precision arithmetic directly on `Ternary` digits                              | `src/operations.rs`, `src/lib.rs`                                  |
| R07 | **P1 — performance**     | Eliminate temporary `Ternary` and `String` objects in remaining fixed-width methods and formatting | `src/tryte.rs`, `src/store.rs`, `src/lib.rs`                       |
| R08 | **P1 — conversion API**  | Normalize conversion and parsing error behavior, including padding/empty input                     | `src/lib.rs`, `src/tryte.rs`, `src/store.rs`, `src/conversions.rs` |
| R09 | **P2 — architecture**    | Introduce arithmetic directly on `TritsChunk` / `DataTernary` in balanced base 243                 | `src/store.rs`                                                     |
| R10 | **P2 — semantic design** | Decide numeric-value versus representation equality for `DataTernary`                              | `src/store.rs`                                                     |
| R11 | **P1 — verification**    | Expand boundary/property tests and CI feature matrix                                               | tests, `.github/workflows/rust.yml`                                |
| R12 | **P2 — release quality** | Benchmark, documentation consistency, release metadata, optional API extensions                    | docs, benchmarks, `Cargo.toml`, `.changelog/`                      |

---

## R01 — Fix the still-incorrect `Digit::sub` [P0]

**Source:** [
`src/digit.rs`](https://github.com/Trehinos/balanced-ternary/blob/28614306f09289ddfaa19e6d7732ff452d97faf5/src/digit.rs#L746-L761).

**Verified from source:** The current subtraction implementation still dispatches to `other.post()`
for `self == Neg` and `other.pre()` for `self == Pos`. For example, `Pos - Zero` returns `Neg`, but
it must return `Pos`. The `v2.1.1` changelog says this was fixed, but the implementation on the
reviewed branch does **not** reflect that correction. Treat this as an outstanding defect,
irrespective of the changelog.

**Minimal fix:**

```rust
impl Sub<Digit> for Digit {
    type Output = Digit;

    fn sub(self, other: Digit) -> Self::Output {
        self + (-other)
    }
}
```

This is subtraction in the same cyclic, modulo-3 digit algebra as the existing `Digit::add`; it is **not** 
an unbounded integer subtraction of two trits.

**Acceptance:** Test all nine pairs of `Neg`, `Zero`, and `Pos`; compare the result with the unique
representative in `{-1, 0, +1}` modulo 3. Include `Pos - Zero == Pos`, `Neg - Zero == Neg`,
`Zero - Pos == Neg`, and `Pos - Neg == Neg` (because `2 ≡ -1 (mod 3)`).

## R02 — Fix a regression in `Ternary::checked_to_dec()` [P0]

**Source:** [
`src/lib.rs`](https://github.com/Trehinos/balanced-ternary/blob/28614306f09289ddfaa19e6d7732ff452d97faf5/src/lib.rs#L315-L353).

**Verified from source and arithmetic:** The implementation currently performs:

```rust
acc.checked_mul(3) ?.checked_add(digit.to_i8() as i64)
```

This is incorrect for at least `Ternary::from_dec(i64::MIN)`, even though its *final* value fits in
`i64`. At the last step of Horner's algorithm, the previous accumulator is
`-3_074_457_345_618_258_603`; multiplying by 3 temporarily gives `-9_223_372_036_854_775_809` (below
`i64::MIN`) before adding the final `+1`. Therefore `checked_to_dec()` returns `None` and `to_dec()`
panics for an otherwise valid value.

**Recommended replacement:** Use a wider intermediate and check the final result:

```rust
pub fn checked_to_dec(&self) -> Option<i64> {
    let value = self.digits.iter().try_fold(0i128, |acc, digit| {
        acc.checked_mul(3)?.checked_add(digit.to_i8() as i128)
    })?;
    i64::try_from(value).ok()
}
```

`i128` works in `no_std`. Checked `i128` arithmetic safely returns `None` for unrepresentably large
inputs. Preserve the public `to_dec()` signature and its documented panic behavior.

**Acceptance tests:**

```rust
for n in [i64::MIN, i64::MIN + 1, - 243, - 1, 0, 1, 243, i64::MAX - 1, i64::MAX] {
    let ternary = Ternary::from_dec(n);
    assert_eq ! (ternary.checked_to_dec(), Some(n)); assert_eq ! (ternary.to_dec(), n);
}
assert_eq!(Ternary::parse(&"0".repeat(150)).checked_to_dec(), Some(0));
assert_eq!(Ternary::parse(&"+".repeat(150)).checked_to_dec(), None);
```

Add randomized tests over the full `i64` domain, plus an exhaustive small interval. **Do not assume
the existing overflow test covers the `i64::MIN` boundary**: at the reviewed revision, it only
checks 41 identical positive/negative digits and a long zero vector.

## R03 — Make `Ternary::trim()` independent of `i64` [P0]

**Source:** [
`src/lib.rs`](https://github.com/Trehinos/balanced-ternary/blob/28614306f09289ddfaa19e6d7732ff452d97faf5/src/lib.rs#L432-L451).

Currently, `trim()` starts by calling `self.to_dec() == 0`. This means trimming a valid 100-trit
nonzero value can panic, even though removing leading zeros is purely structural.

The crate already defines `significant_digits() -> &[Digit]` for numeric equality and comparison.
Reuse it:

```rust
pub fn trim(&self) -> Self {
    let significant = self.significant_digits();
    if significant.is_empty() {
        Self::new(vec![Digit::Zero])
    } else {
        Self::new(significant.to_vec())
    }
}
```

**Acceptance:** Arbitrary 256/1024-trit positive and negative values trim successfully; `"000+"`
becomes `"+"`; empty and all-zero inputs return the canonical single `Zero`; `trim()` is idempotent
and does not change the numeric value.

## R04 — Enforce the actual 40-trit invariant of `Ter40` [P0]

**Source:** [
`src/store.rs`](https://github.com/Trehinos/balanced-ternary/blob/28614306f09289ddfaa19e6d7732ff452d97faf5/src/store.rs#L360-L380)
and its arithmetic operator implementations.

The associated bounds exist, but `Ter40::from_dec(i64)` still stores any `i64` unchanged.
`Ter40::from_dec(i64::MAX)` is therefore constructible even though `i64::MAX` needs 41 balanced
trits. `Ter40::to_ternary()` then calls `with_length(40)`, which **does not truncate** oversized
representations, and `map_trits()` panics when this invalid object is used.

**Required work:**

1. Add `Ter40::checked_from_dec(value: i64) -> Option<Self>` that validates
   `Self::MIN.0..=Self::MAX.0`.
2. Change the existing `from_dec()` to enforce that range with an explicit panic, or adopt a
   documented fallible constructor strategy in a breaking release.
3. Make `from_ternary()` validate the **numeric** value, not merely the raw length; do not silently
   accept out-of-range values.
4. Audit `From<i64>` for `Ter40`: it must uphold the same invariant or be intentionally replaced by
   `TryFrom<i64>` in a breaking release. Avoid implementing `TryFrom<i64>` while `From<i64>` is
   present, because Rust already supplies a blanket `TryFrom` through `Into`.
5. Audit `Add`, `Sub`, `Mul`, `Div`, `Neg`: they currently use primitive `i64` arithmetic and can
   produce values outside the 40-trit interval. The current behavior may differ between debug and
   release when `i64` itself overflows.

**Acceptance:** Every publicly constructible `Ter40` value lies within its declared bounds; every
successful arithmetic result also lies within the bounds; all invalid cases have deterministic
behavior in debug and release. Test `MAX`, `MIN`, `MAX+1`, `MIN-1`, `MAX+MAX`, `MIN+MIN`, division
by zero, and `MIN / -1` under the chosen overflow policy.

## R05 — Define fixed-width overflow and carry APIs [P1]

**Sources:** `src/tryte.rs`, `src/store.rs`, `src/concepts.rs`.

The repaired `each_zip_carry()` methods operate on a fixed number of trits for `Tryte<N>` and
`Ter40`, but their public return type drops the final carry. This is acceptable as a *wrapping
digit-array operation*, but it is not adequate for checked arithmetic.

**Add explicit, non-ambiguous methods:**

```rust
// Illustrative API targets; choose consistent naming/signatures:
Tryte::<N>::checked_add( self , rhs) -> Option< Self > Tryte::<N>::wrapping_add( self , rhs) -> Self Tryte::<N>::overflowing_add( self , rhs) -> ( Self, bool)
Tryte::<N>::saturating_add( self , rhs) -> Self

Ter40::checked_add( self , rhs) -> Option< Self > Ter40::wrapping_add( self , rhs) -> Self Ter40::overflowing_add( self , rhs) -> ( Self, bool)
Ter40::saturating_add( self , rhs) -> Self
```

Repeat as appropriate for subtraction, multiplication, and negation. For division, explicitly define
division-by-zero behavior and the rounding convention. Use the type's **ternary** bounds, not the
wider `i64` bounds. Consider `each_zip_carry_with_overflow()` or `each_zip_carry_full()` returning
`(Self, Digit)`; the `Ter40::map_trits()` helper already calculates the outgoing carry.

**Open architectural decision:** What should the `Add`, `Sub`, etc. operator overloads do by
default: panic on overflow, wrap modulo `3^N`, or use another policy? Pick one and document it
consistently. At present `Tryte<N>` arithmetic delegates through `Ternary`/`i64` and can panic on a
representable 40-trit wraparound; `Ter40` uses ordinary `i64` arithmetic that can violate the
40-trit range.

**Width-0 edge case:** Decide whether `Tryte<0>` is a supported, valid zero-width data word. The
current constructor accepts it, and its only numeric value would be zero; document or reject it
consistently.

## R06 — Replace `i64`-based arithmetic in `Ternary` [P1]

**Source:** [
`src/operations.rs`](https://github.com/Trehinos/balanced-ternary/blob/28614306f09289ddfaa19e6d7732ff452d97faf5/src/operations.rs).

At the reviewed commit, `Add`, `Sub`, `Mul`, and `Div` for `&Ternary` still call `to_dec()`, do
arithmetic on `i64`, and convert back. This artificially restricts a `Vec<Digit>`-backed type to
64-bit numerical operations and triggers avoidable allocations.

**Recommended sequence:**

1. **Addition:** Implement native right-to-left balanced-trit addition with a carry. The existing
   `each_zip_carry()` mechanism can help. For a sum `s = a + b + carry`, use
   `new_carry = (s + 1).div_euclid(3)` and `digit = s - 3 * new_carry` to obtain both results in
   `{-1,0,1}`. Normalize the final result only if numeric arithmetic is intended to produce
   canonical representations.
2. **Subtraction:** Reuse addition and digitwise negation, paying attention to `Digit` subtraction
   semantics versus multi-trit subtraction.
3. **Multiplication:** Initially use schoolbook digit multiplication and carry normalization (
   `O(n*m)`), then benchmark before considering more advanced algorithms.
4. **Division/remainder:** Design quotient/remainder semantics (truncate toward zero, as Rust signed
   integers do, is the least surprising choice); implement long division without converting the full
   value to `i64`.
5. **Other traits:** Add `Rem`, `AddAssign`, `SubAssign`, and borrowed/owned operator combinations
   only after the core arithmetic is correct and tested.

**Acceptance:** Arithmetic operates on 100+ trits without `i64` conversion; signs and carries are
correct; `a+b-b == a`, `a*0 == 0`, `a*1 == a`, and `a == (a/b)*b + (a%b)` for nonzero `b` under the
selected quotient/remainder convention. Cross-check small values against `i128` where representable
and independently generate wide test vectors.

## R07 — Eliminate remaining allocations/conversions in fixed-width methods [P1]

**Sources:** [
`src/tryte.rs`](https://github.com/Trehinos/balanced-ternary/blob/28614306f09289ddfaa19e6d7732ff452d97faf5/src/tryte.rs), [
`src/store.rs`](https://github.com/Trehinos/balanced-ternary/blob/28614306f09289ddfaa19e6d7732ff452d97faf5/src/store.rs), [
`src/lib.rs`](https://github.com/Trehinos/balanced-ternary/blob/28614306f09289ddfaa19e6d7732ff452d97faf5/src/lib.rs).

The `DigitOperate::each*` methods now work directly over the `Tryte<N>` array or `Ter40` scalar. *
*Do not redo that work.** Remaining hotspots are elsewhere:

- `Tryte<N>`: `to_i64()` converts through `to_ternary()`; `from_i64()` converts through `Ternary`;
  operator implementations (`Neg`, `Add`, `Sub`, `Mul`, `Div`, `BitAnd`, `BitOr`, `BitXor`, `Not`)
  still construct temporary `Ternary` instances. Use the direct array for digitwise logic and
  fixed-width carry arithmetic. For bounded `N<=40`, a direct Horner fold over the array is
  sufficient for `to_i64()`.
- `Tryte<N>::fmt`: formats through an allocated `Ternary` and `String`; write characters directly to
  the formatter.
- `Ter40::digit()` and `to_digits()` repeatedly materialize `Ternary`. Extract trits using a scalar
  path; `to_digits()` must necessarily allocate its returned vector, but it should not allocate a
  temporary `Ternary` first.
- `Ter40::fmt`: consider writing 40 trits directly, without creating a `Ternary`.
- `Ternary::fmt`: avoid `to_string_repr()` allocation inside `Display`; write each digit with the
  formatter.
- `Ternary::get_digit()`, `with_length()`, `concat()`: simplify indexing/preallocation where
  supported; benchmark rather than assuming meaningful speedups.
- `DataTernary::from_ternary()`: presently constructs a fresh `Vec<Digit>` and `Ternary` for every
  5-trit chunk. Encode each five-trit slice directly into `i8`, with correct range handling.
  `to_digits()` redundantly calls `.trim()` on a normalized value.

**Acceptance:** No heap allocation in fixed-width digitwise operators; clear count of unavoidable
allocations; equivalent output and width semantics; benchmark before/after.

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

1. **Correctness hotfix:** R01, R02, R03; add regression tests in the same commit(s). Explicitly
   test `i64::MIN` and long `trim()`.
2. **Fixed-width safety:** R04, then decide R05 before changing operator overloads. Preserve or
   intentionally break compatibility with a documented release plan.
3. **Arithmetic core:** R06. Start with add/sub and property tests; implement mul and div/rem
   separately.
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

## Prompt to resume in a later session

> Inspect the latest `Trehinos/balanced-ternary` repository and compare it against
`balanced_ternary_remaining_work.md` (review baseline: `28614306f09289ddfaa19e6d7732ff452d97faf5`).
> Only work on tasks that are **still outstanding** in the newest source. Begin with R01 (
`Digit::sub`) and R02 (`checked_to_dec` on `i64::MIN`), then R03 (`trim`) unless those have been
> fixed already. Keep `no_std + alloc`, show precise code changes and tests, and never assume a
> changelog entry proves the source was corrected. Do not modify the repository without explicit
> permission.
