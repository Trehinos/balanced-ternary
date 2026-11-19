# Three-Valued Logic and Balanced Ternary in Rust

*A practical introduction with `balanced-ternary` 3.2.0*

A configuration setting has not been loaded yet. A check has not finished. A sensor has not reported a value. In each
case, `false` would say more than you actually know.

Three-valued logic gives you a third state and rules for combining it with true and false. Balanced ternary uses three
digits to represent signed integers. These ideas share a three-element set, but they answer different questions.

This tutorial develops both ideas using the Rust crate `balanced-ternary`. You will start with individual truth values,
build a small rule evaluator, move on to arbitrary-precision and fixed-width arithmetic, and finally choose storage
formats and Cargo features for `no_std` code.

You need basic Rust: enums, functions, references, traits, `Option`, `Result`, and Cargo. No previous knowledge of
three-valued logic is assumed.

**Version scope.** This tutorial targets **balanced-ternary 3.2.0**. Use the pinned dependency below to follow along
reproducibly. All examples use the current `toward` and `change` names.

## Contents

1. [Three values, several meanings](#1-three-values-several-meanings)
2. [Create a Rust project](#2-create-a-rust-project)
3. [Meet `Digit`](#3-meet-digit)
4. [Reason with strong Kleene logic](#4-reason-with-strong-kleene-logic)
5. [Choose how to handle uncertainty](#5-choose-how-to-handle-uncertainty)
6. [Explore the other three-valued logics](#6-explore-the-other-three-valued-logics)
7. [Build a rule evaluator](#7-build-a-rule-evaluator)
8. [Read balanced ternary numbers](#8-read-balanced-ternary-numbers)
9. [Calculate with `Ternary`](#9-calculate-with-ternary)
10. [Separate numeric values from representations](#10-separate-numeric-values-from-representations)
11. [Transform collections of trits](#11-transform-collections-of-trits)
12. [Use fixed-width words with `Tryte<N>`](#12-use-fixed-width-words-with-tryten)
13. [Distinguish integer and modular division](#13-distinguish-integer-and-modular-division)
14. [Use a compact bounded integer with `Ter40`](#14-use-a-compact-bounded-integer-with-ter40)
15. [Store trits compactly](#15-store-trits-compactly)
16. [Configure features and `no_std`](#16-configure-features-and-no_std)
17. [Choose types and handle failures](#17-choose-types-and-handle-failures)
18. [Exercises and solutions](#18-exercises-and-solutions)
19. [Operation guide and references](#19-operation-guide-and-references)

## 1. Three values, several meanings

A **bit** has two possible states. A **trit**, or ternary digit, has three.

In this crate, a trit is a `Digit`:

| Rust value    | Balanced symbol | Numeric value | Our initial logical interpretation |
|---------------|-----------------|--------------:|------------------------------------|
| `Digit::Neg`  | `-`             |            −1 | False                              |
| `Digit::Zero` | `0`             |             0 | Unknown                            |
| `Digit::Pos`  | `+`             |            +1 | True                               |

The logical interpretation is a choice made by your program. The enum itself does not know whether it represents a truth
value, a signed digit, a direction, or a neutral state.

For the first part of this tutorial, **unknown means that the truth of a proposition has not been established**. It does
not mean false. It also does not mean a 50% probability of being true.

An `Option<bool>` can encode these three states:

| `Option<bool>` | `Digit` |
|----------------|---------|
| `Some(false)`  | `Neg`   |
| `None`         | `Zero`  |
| `Some(true)`   | `Pos`   |

The additional benefit of a logic library is that it supplies operations over those states. You still need to decide
which rules fit your application.

For example, consider:

> The item is available **and** delivery is possible.

If availability is false, the whole statement is false, even if delivery is unknown. If availability is true and
delivery is unknown, the whole statement remains unknown. This is the starting point for **strong Kleene logic**.

Keep three layers separate throughout the tutorial:

- **Representation:** which Rust values and symbols store the states?
- **Interpretation:** what does each state mean in this application?
- **Operation:** which table determines the result for each combination?

Choosing three states does not uniquely determine the logic.

## 2. Create a Rust project

The crate's declared minimum Rust version is **1.85**, and it uses edition 2024.

Create a project:

```sh
cargo new ternary-tutorial
cd ternary-tutorial
```

Add this entry under `[dependencies]` in `Cargo.toml`:

```toml
[dependencies]
balanced-ternary = "=3.2.0"
```

The package name uses a hyphen; the Rust import uses an underscore:

```rust
use balanced_ternary::Digit;

fn main() {
    let state = Digit::Zero;
    println!("State: {}", state.to_char());
    assert_eq!(state.to_char(), '0');
}
```

Run it with `cargo run`. The displayed state is `0`.

Unless a block contains its own `fn main()`, put the Rust examples inside `main`. Imports can also be placed there. Each
example is self-contained; you can replace the previous example instead of combining them. The `no_std` examples later
in the tutorial are explicitly intended for `src/lib.rs`.

The default features enable all the types covered here. We will reduce the feature set in section 16.

## 3. Meet `Digit`

### Construct and inspect a digit

Use the enum variants directly, or validate an external representation:

```rust
use balanced_ternary::Digit;
use Digit::{Neg, Pos, Zero};

assert_eq!(Digit::try_from_char('+'), Ok(Pos));
assert_eq!(Digit::try_from_i8(-1), Ok(Neg));
assert!(Digit::try_from_char('?').is_err());
assert!(Digit::try_from_i8(2).is_err());

assert_eq!(Neg.to_i8(), -1);
assert_eq!(Zero.to_char(), '0');
assert_eq!(Pos.to_char(), '+');
```

`from_char` and `from_i8` also exist, but panic on invalid input. The `try_` forms are appropriate at an input boundary.
In 3.2.0, `Digit` itself does not implement `Display`: print `digit.to_char()` for a symbol, or use `{:?}` for its enum
variant.

Use `to_i8()` for the signed numeric value. **Do not cast the enum with `as i8` to obtain −1, 0, or +1.** Its
discriminants are an implementation representation, not those signed values.

### Cross the Boolean boundary explicitly

```rust
use balanced_ternary::Digit::{Neg, Pos, Zero};

assert_eq!(Neg.try_to_bool(), Some(false));
assert_eq!(Zero.try_to_bool(), None);
assert_eq!(Pos.try_to_bool(), Some(true));

let message = match Zero.try_to_bool() {
Some(true) => "confirmed",
Some(false) => "rejected",
None => "waiting for information",
}; assert_eq!(message, "waiting for information");
```

The method named `ht_bool()` returns a plain `bool`, but **panics on `Zero`**. It is not a default policy for resolving
unknown values.

When converting in the other direction, an explicit adapter makes the interpretation visible:

```rust
use balanced_ternary::Digit;

fn from_optional_bool(value: Option<bool>) -> Digit {
    match value {
        Some(false) => Digit::Neg,
        None => Digit::Zero,
        Some(true) => Digit::Pos,
    }
}

assert_eq!(from_optional_bool(None), Digit::Zero);
```

## 4. Reason with strong Kleene logic

For this section, abbreviate false, unknown, and true as **F**, **U**, and **T**.

The crate uses Rust's `!`, `&`, `|`, and `^` operators for digit logic. Despite the Rust trait names `Not`, `BitAnd`,
`BitOr`, and `BitXor`, these operations apply ternary truth tables. They do not operate on the binary encoding of the
enum.

### Negation

Negation exchanges true and false while preserving uncertainty:

| `a` | `!a` |
|-----|------|
| F   | T    |
| U   | U    |
| T   | F    |

```rust
use balanced_ternary::Digit::{Neg, Pos, Zero};

assert_eq!(!Neg, Pos);
assert_eq!(!Zero, Zero);
assert_eq!(!Pos, Neg);
```

If you do not know whether the item is available, you also do not know whether it is unavailable.

### AND, OR, and XOR

The table below lists all nine pairs. The left input is `a`, and the right input is `b`.

| `a` | `b` | `a & b` | `a \| b` | `a ^ b` |
|-----|-----|---------|----------|---------|
| F   | F   | F       | F        | F       |
| F   | U   | F       | U        | U       |
| F   | T   | F       | T        | T       |
| U   | F   | F       | U        | U       |
| U   | U   | U       | U        | U       |
| U   | T   | U       | T        | U       |
| T   | F   | F       | T        | T       |
| T   | U   | U       | T        | U       |
| T   | T   | T       | T        | F       |

For AND, a known false operand settles the result. For OR, a known true operand settles the result. XOR remains unknown
whenever either operand is unknown.

```rust
use balanced_ternary::Digit::{Neg, Pos, Zero};

assert_eq!(Neg & Zero, Neg);
assert_eq!(Pos & Zero, Zero);
assert_eq!(Pos | Zero, Pos);
assert_eq!(Neg | Zero, Zero);
assert_eq!(Pos ^ Zero, Zero);
```

A useful way to calculate AND and OR is to order the states `F < U < T`: AND selects the lower state, and OR selects the
higher state. This is the **truth ordering**, not a measure of how much information you have. F and T are both known
values; U is less informative than either.

Parenthesize mixed expressions to make the grouping obvious. Use `&` and `|`, not Boolean `&&` and `||`. These
overloaded operators **evaluate both operand expressions**; they do not short-circuit.

### Implication and equivalence

`a.k3_imply(b)` expresses the material implication “if `a`, then `b`” using `(!a) | b`.

`a.k3_equiv(b)` combines the implications in both directions.

| `a` | `b` | `a.k3_imply(b)` | `a.k3_equiv(b)` |
|-----|-----|-----------------|-----------------|
| F   | F   | T               | T               |
| F   | U   | T               | U               |
| F   | T   | T               | F               |
| U   | F   | U               | U               |
| U   | U   | U               | U               |
| U   | T   | T               | U               |
| T   | F   | F               | F               |
| T   | U   | U               | U               |
| T   | T   | T               | T               |

A false premise makes this implication true. That follows from its truth table; it does not establish causation or run a
conditional action.

You can verify the definitions over the entire three-element domain:

```rust
use balanced_ternary::Digit::{Neg, Pos, Zero};

for a in [Neg, Zero, Pos] {
for b in [Neg, Zero, Pos] {
assert_eq ! (a.k3_imply(b), ( ! a) | b); assert_eq !(
a.k3_equiv(b),
a.k3_imply(b) & b.k3_imply(a),
);
}
}
```

### Equality is not logical equivalence

Rust can tell that two enum values are both `Zero`. That is a statement about their representations. It does not
establish that the unknown propositions they represent have the same truth value.

```rust
use balanced_ternary::Digit::{Pos, Zero};

assert!(Zero == Zero);                    // Rust bool: same enum value.
assert_eq!(Zero.equal(Zero), Pos);         // Same comparison, encoded as a Digit.
assert_eq!(Zero.k3_equiv(Zero), Zero);     // Logical equivalence remains unknown.
```

The same issue appears in familiar Boolean identities:

```rust
use balanced_ternary::Digit::Zero;

let a = Zero;
assert_eq!(a | !a, Zero);
assert_eq!(a & !a, Zero);
```

These expressions do not become true and false respectively under these tables. A `Digit` stores a state, not the
identity of a proposition or correlations between expressions. A symbolic system that tracks such relationships would be
a separate layer.

## 5. Choose how to handle uncertainty

Preserving uncertainty is useful during computation. At some point, an application may need a definite action. Make that
conversion a policy decision.

The crate supplies three useful unary operations:

| Input | `necessary()` | `possibly()` | `contingently()` |
|-------|---------------|--------------|------------------|
| F     | F             | F            | F                |
| U     | F             | T            | T                |
| T     | T             | T            | F                |

For this interpretation:

- `necessary()` asks whether the value is definitely true.
- `possibly()` asks whether it has not been ruled out.
- `contingently()` detects the middle state.

These methods implement these tables. Calling them does not turn the crate into a modal theorem prover.

```rust
use balanced_ternary::Digit::{Neg, Pos, Zero};

let eligibility = Zero;

let show_as_candidate = eligibility.possibly().ht_bool(); let show_as_confirmed = eligibility.necessary().ht_bool();

assert!(show_as_candidate);
assert!(!show_as_confirmed);
assert_eq!(eligibility, Zero); // Neither policy changes the original value.
assert_eq!(Neg.possibly(), Neg);
assert_eq!(Pos.necessary(), Pos);
```

Use `try_to_bool()` when the caller should retain the third state. Use a policy when the caller must choose a Boolean
result. Avoid converting unknown to false early and then treating the result as observed evidence.

Also keep missing information separate from operational failure when both matter. For example,
`Result<Digit, ReadError>` can distinguish an unknown logical value from a failed read. A single `Digit` cannot retain
every reason for uncertainty.

## 6. Explore the other three-valued logics

Three-valued logic is a family of systems, not one universal set of rules. The crate exposes operations from several of
them. This section gives each family a separate interpretation, its relevant tables, and code you can run.

We will cover:

- [6.1. What defines a logic?](#61-what-defines-a-logic)
- [6.2. Bochvar and weak Kleene](#62-bochvar-and-weak-kleene-infectious-undefinedness)
- [6.3. Łukasiewicz](#63-łukasiewicz-an-intermediate-truth-degree)
- [6.4. Here-and-there / Gödel G3](#64-here-and-there--gödel-g3-two-related-worlds)
- [6.5. LP: the Logic of Paradox](#65-lp-the-logic-of-paradox)
- [6.6. RM3: R-mingle 3](#66-rm3-r-mingle-3)
- [6.7. The crate's `para_imply`](#67-the-crates-para_imply-a-separate-conditional)
- [6.8. McCarthy's sequential logic](#68-mccarthys-sequential-logic)
- [6.9. Post's cyclic operations](#69-posts-cyclic-operations)
- [6.10. Compare and choose](#610-compare-and-choose)
- [6.11. Application-oriented operations](#611-application-oriented-operations)

### 6.1. What defines a logic?

So far, we have interpreted `Zero` as unknown. That convention will change in some of the systems below. Keep the Rust
encoding fixed while making the interpretation explicit:

| Family                         | Meaning assigned to `Zero` in this presentation          | Main operations                                                |
|--------------------------------|----------------------------------------------------------|----------------------------------------------------------------|
| Strong Kleene K3               | Unknown or undefined                                     | `!`, `&`, `\|`, `k3_imply`, `k3_equiv`                         |
| Bochvar internal / weak Kleene | Undefined, propagating through compound expressions      | `!`, `bi3_and`, `bi3_or`, `bi3_imply`                          |
| Łukasiewicz L3                 | Intermediate truth degree                                | `!`, `&`, `\|`, `l3_imply`, `l3_strong_*`                      |
| Here-and-there / Gödel G3      | True “there,” but not true “here”                        | `ht_not`, `&`, `\|`, `ht_imply`                                |
| LP                             | Both true and false                                      | `!`, `&`, `\|`; `k3_imply` has LP's material-conditional table |
| RM3                            | Both true and false                                      | `!`, `&`, `\|`, `rm3_imply`, `rm3_fusion`                      |
| McCarthy                       | An undefined result that interacts with evaluation order | `!`, `mc_and`, `mc_or`                                         |
| Post-style cyclic operations   | A position in a three-state cycle                        | `post`, `pre`; other operations must be chosen explicitly      |

A truth table determines the value of an expression. To discuss **valid inference**, we also need to specify which
values count as **designated**: the values that premises and conclusions must preserve.

For example, K3 designates only T. LP and RM3 designate both T and B, where B means “both true and false.” In ordinary
matrix semantics, an inference is valid when **every assignment that gives all premises designated values also gives the
conclusion a designated value**.

This is why two systems can share truth tables and still differ as logics. The distinction is developed
in [Indrzejczak and Petrukhin, section 2](https://link.springer.com/chapter/10.1007/978-3-031-38499-8_19).

`balanced-ternary` provides value operations, not a formula parser, proof engine, or automatic choice of designated
values. Its `Digit` type does not track the chosen family. Name domain functions or introduce wrapper types when mixing
interpretations would be a bug.

In the tables below, **F = `Neg`** and **T = `Pos`**. We write the middle value as U for unknown/undefined, B for both,
and M when we need a neutral label. All three labels still encode the same Rust variant, `Zero`.

### 6.2. Bochvar and weak Kleene: infectious undefinedness

Strong Kleene logic allows some known inputs to settle the result despite an unknown input. Bochvar's **internal**
operations take a stricter approach: if any operand is undefined, the compound expression is undefined.

For these connectives, the tables coincide with weak Kleene tables. “Internal” matters: this subsection does not claim
to implement every additional connective of Bochvar's broader systems.

| `a` | `b` | `bi3_and` | `bi3_or` | `bi3_imply` |
|-----|-----|-----------|----------|-------------|
| F   | F   | F         | F        | T           |
| F   | U   | U         | U        | U           |
| F   | T   | F         | T        | T           |
| U   | F   | U         | U        | U           |
| U   | U   | U         | U        | U           |
| U   | T   | U         | U        | U           |
| T   | F   | F         | T        | F           |
| T   | U   | U         | U        | U           |
| T   | T   | T         | T        | T           |

Negation remains `!`: F becomes T, T becomes F, and U stays U.

```rust
use balanced_ternary::Digit::{Neg, Pos, Zero};

assert_eq!(Neg & Zero, Neg);                // Strong Kleene.
assert_eq!(Neg.bi3_and(Zero), Zero);        // Bochvar internal.
assert_eq!(Pos | Zero, Pos);                // Strong Kleene.
assert_eq!(Pos.bi3_or(Zero), Zero);         // Bochvar internal.
assert_eq!(Neg.bi3_imply(Zero), Zero);
```

An application-level interpretation is “this expression is not meaningful unless all its parts are defined.” Imagine a
validator whose inputs can be valid, invalid, or not interpretable. You might want the last state to propagate even when
another field is already invalid:

```rust
use balanced_ternary::Digit::{Neg, Zero};

let malformed_date = Zero; let invalid_identifier = Neg;

let validity = invalid_identifier.bi3_and(malformed_date); assert_eq!(validity, Zero);
```

This is a proposed modeling choice, not a requirement that every validator should use Bochvar. A separate error report
is still necessary if you need to retain why the date was not interpretable.

The implication can be constructed using this family's OR:

```rust
use balanced_ternary::Digit::{Neg, Pos, Zero};

for a in [Neg, Zero, Pos] {
for b in [Neg, Zero, Pos] {
assert_eq ! (a.bi3_imply(b), ( ! a).bi3_or(b));
}
}
```

Using `(!a) | b` instead would silently switch to the strong Kleene implication. The choice of connective matters even
when the formula's written shape is unchanged.

### 6.3. Łukasiewicz: an intermediate truth degree

For L3, use the truth degrees F, M, and T, conventionally written `0`, `1/2`, and `1`. The balanced encoding maps them
to −1, 0, and +1. This middle truth degree is not automatically a probability.

The ordinary lattice conjunction and disjunction are still min and max, available as `&` and `|`. Łukasiewicz
implication is different from Kleene implication: **M implies M evaluates to T**.

The crate also supplies **strong conjunction** and **strong disjunction**. These are separate connectives, not
alternative names for `&` and `|`.

| `a` | `b` | `l3_imply` | `l3_strong_and` | `l3_strong_or` |
|-----|-----|------------|-----------------|----------------|
| F   | F   | T          | F               | F              |
| F   | M   | T          | F               | M              |
| F   | T   | T          | F               | T              |
| M   | F   | M          | F               | M              |
| M   | M   | T          | F               | T              |
| M   | T   | T          | M               | T              |
| T   | F   | F          | F               | T              |
| T   | M   | M          | M               | T              |
| T   | T   | T          | T               | T              |

```rust
use balanced_ternary::Digit::{Neg, Pos, Zero};

assert_eq!(Zero.k3_imply(Zero), Zero);
assert_eq!(Zero.l3_imply(Zero), Pos);

assert_eq!(Zero & Zero, Zero);
assert_eq!(Zero.l3_strong_and(Zero), Neg);
assert_eq!(Zero | Zero, Zero);
assert_eq!(Zero.l3_strong_or(Zero), Pos);
```

For balanced numeric encodings `a` and `b`, the formulas are:

| Connective      | Formula for the output encoding |
|-----------------|---------------------------------|
| `l3_imply`      | `min(1, 1 - a + b)`             |
| `l3_strong_and` | `max(-1, a + b - 1)`            |
| `l3_strong_or`  | `min(1, a + b + 1)`             |

These are formulas over small integers. Do not evaluate `a + b` using `Digit` addition inside the formula: that operator
wraps modulo three.

```rust
use balanced_ternary::Digit::{Neg, Pos, Zero};

for a in [Neg, Zero, Pos] {
for b in [Neg, Zero, Pos] {
let x = a.to_i8(); let y = b.to_i8(); assert_eq !(a.l3_imply(b).to_i8(), (1 - x + y).min(1)); assert_eq ! (a.l3_strong_and(b).to_i8(), (x + y - 1).max(- 1)); assert_eq ! (a.l3_strong_or(b).to_i8(), (x + y + 1).min(1));
}
}
```

You can define L3 equivalence as implication in both directions using lattice conjunction:

```rust
use balanced_ternary::Digit;
use Digit::{Pos, Zero};

fn l3_equiv(a: Digit, b: Digit) -> Digit {
    a.l3_imply(b) & b.l3_imply(a)
}

assert_eq!(l3_equiv(Zero, Zero), Pos);
assert_eq!(Zero.k3_equiv(Zero), Zero);
```

`l3_equiv` above is a local helper, not a built-in method in 3.2.0. The changed self-equivalence illustrates why you
should not reuse `k3_equiv` merely because its name contains “equiv.” For the theoretical comparison,
see [Hölldobler and Ramli, section 2](https://iccl.inf.tu-dresden.de/w/images/7/72/Hoelldobler%3A09%3Awv02.pdf).

### 6.4. Here-and-there / Gödel G3: two related worlds

**HT means here-and-there.** Its propositional three-valued presentation corresponds to Gödel's G3. One way to
understand it is through two worlds, “here” and “there,” with truth persistent from here to there.

For an atomic proposition, this leaves three possibilities:

| Rust value | True here? | True there? | Label |
|------------|------------|-------------|-------|
| `Neg`      | No         | No          | F     |
| `Zero`     | No         | Yes         | M     |
| `Pos`      | Yes        | Yes         | T     |

The missing pair, “true here but not there,” violates persistence. This is a semantic relation between worlds; it need
not represent chronological time. See the HT/QHT preliminaries
in [Cabalar et al.](https://www.dc.fi.udc.es/~cabalar/freeht.pdf).

The crate's `&` and `|` give the lattice conjunction and disjunction. Use **`ht_not()`**, not `!`, for HT negation:

| `a` | `ht_not()` | `ht_not().ht_not()` |
|-----|------------|---------------------|
| F   | T          | F                   |
| M   | F          | T                   |
| T   | F          | T                   |

A proposition true there cannot be negated here. Consequently, M's negation is F, and its double negation is T. Double
negation does not simply recover M.

Implication follows the rule “return T if `a <= b`; otherwise return `b`,” using the encoding order F < M < T:

| `a` | `b` | `ht_imply` |
|-----|-----|------------|
| F   | F   | T          |
| F   | M   | T          |
| F   | T   | T          |
| M   | F   | F          |
| M   | M   | T          |
| M   | T   | T          |
| T   | F   | F          |
| T   | M   | M          |
| T   | T   | T          |

```rust
use balanced_ternary::Digit::{Neg, Pos, Zero};

assert_eq!(!Zero, Zero);
assert_eq!(Zero.ht_not(), Neg);
assert_eq!(Zero.ht_not().ht_not(), Pos);
assert_eq!(Zero.ht_imply(Zero), Pos);
assert_eq!(Zero.ht_imply(Neg), Neg);

for a in [Neg, Zero, Pos] {
for b in [Neg, Zero, Pos] {
let expected = if a.to_i8() <= b.to_i8() { Pos } else { b }; assert_eq ! (a.ht_imply(b), expected);
}
}
```

HT implication is not generally definable by replacing `!` with `ht_not()` in material implication:

```rust
use balanced_ternary::Digit::{Pos, Zero};

assert_eq!(Zero.ht_imply(Zero), Pos);
assert_eq!(Zero.ht_not() | Zero, Zero);
```

In this three-valued presentation, only T is designated. The name `ht_bool()` does not make it the appropriate
designated-value test: that method panics on `Zero`. To test whether the result is designated in HT, compare it with
`Digit::Pos`.

These operations are useful building blocks when studying HT formulas or implementing a larger logical evaluator. They
do not by themselves implement answer-set search or equilibrium-model selection.

### 6.5. LP: the Logic of Paradox

LP uses a different middle state: **B means both true and false**, rather than neither known true nor known false. Its
designated values are **T and B**.

The negation, conjunction, disjunction, and material-implication tables can reuse the functions introduced for K3. Read
the U entries in section 4 as B instead. The truth-function calculations match; the interpretation and designated values
differ. This relationship is described
in [Pelletier, Sutcliffe, and Hazen](https://cdn.aaai.org/ocs/15415/15415-68643-1-PB.pdf).

| Rust value | LP interpretation   | Designated? |
|------------|---------------------|-------------|
| `Neg`      | False only          | No          |
| `Zero`     | Both true and false | Yes         |
| `Pos`      | True only           | Yes         |

Here is an explicit designated-value predicate:

```rust
use balanced_ternary::Digit;
use Digit::{Neg, Zero};

fn lp_designated(value: Digit) -> bool {
    value != Neg
}

let p = Zero; // B: both true and false.
let unrelated = Neg;

assert!(lp_designated(p));
assert!(lp_designated(!p));
assert!(lp_designated(p & !p));
assert!(!lp_designated(unrelated));
```

This assignment is a counterexample to **explosion**, the inference “from `p` and `!p`, infer any `q`.” Both premises
are designated, while the unrelated conclusion is not. Allowing a contradiction does not make every statement follow.

As an application interpretation, B can represent positive and negative support for the same claim. However, do not also
use B to mean “no information.” If your data model needs false-only, true-only, both, and neither, those are four
distinct states and cannot all be represented by a single `Digit`.

LP's material conditional uses the same table as `k3_imply`, namely `(!a) | b`. It does **not** validate modus ponens
under LP's designated values:

```rust
use balanced_ternary::Digit::{Neg, Zero};

let a = Zero; // Designated in LP.
let b = Neg;  // Not designated in LP.
let conditional = a.k3_imply(b);

assert_eq!(conditional, Zero); // Also designated in LP.
assert!(a != Neg && conditional != Neg && b == Neg);
```

The two premises, `a` and its material conditional, are designated, while `b` is not. This example demonstrates the
distinction between computing an implication's value and treating that implication as a detachable rule.

There is no `Lp` type or `lp_imply` method in this version. The LP examples deliberately reuse `Digit` operations and
supply their own designated-value test.

### 6.6. RM3: R-mingle 3

RM3 also uses F, B, and T, with B and T designated. It retains the same `!`, `&`, and `|` operations, but changes
implication. The crate exposes this conditional as `rm3_imply`.

It also provides `rm3_fusion`, a multiplicative conjunction distinct from lattice AND:

| `a` | `b` | `rm3_imply` | `rm3_fusion` |
|-----|-----|-------------|--------------|
| F   | F   | T           | F            |
| F   | B   | T           | F            |
| F   | T   | T           | F            |
| B   | F   | F           | F            |
| B   | B   | B           | B            |
| B   | T   | T           | T            |
| T   | F   | F           | F            |
| T   | B   | F           | T            |
| T   | T   | T           | T            |

Compared with LP's material conditional, the RM3 conditional changes B → F and T → B to F. In particular, the B → F
change removes the preceding counterexample to modus ponens.

You can verify that modus ponens preserves designated values for **all nine pairs**:

```rust
use balanced_ternary::Digit;
use Digit::{Neg, Pos, Zero};

fn designated(value: Digit) -> bool {
    value != Neg
}

for a in [Neg, Zero, Pos] {
for b in [Neg, Zero, Pos] {
if designated(a) & & designated(a.rm3_imply(b)) {
assert ! (designated(b));
}
}
}

assert_eq!(Zero.rm3_imply(Neg), Neg);
assert_eq!(Pos.rm3_imply(Zero), Neg);
assert_eq!(Zero.rm3_imply(Zero), Zero); // B is designated, though not T.
```

Notice the final line: “designated” does not mean “equal to `Pos`.” If an implementation accepted only `Pos`, it would
be evaluating a different consequence relation.

For fusion, B acts as an identity; for lattice conjunction, T acts as an identity. They therefore differ when combining
T and B:

```rust
use balanced_ternary::Digit::{Neg, Pos, Zero};

assert_eq!(Pos & Zero, Zero);
assert_eq!(Pos.rm3_fusion(Zero), Pos);

for a in [Neg, Zero, Pos] {
assert_eq ! (a.rm3_fusion(Zero), a); for b in [Neg, Zero, Pos] {
assert_eq ! (a.rm3_fusion(b), ! (a.rm3_imply( ! b)));
}
}
```

The examples above are direct finite checks of the crate's tables. For RM3's logical setting and its distinction from
LP, see [*Automated Reasoning for the Dialetheic Logic RM3*](https://cdn.aaai.org/ocs/15415/15415-68643-1-PB.pdf).

### 6.7. The crate's `para_imply`: a separate conditional

“Paraconsistent” names a property of a logic, not one universally agreed implication table. Do not assume that
`para_imply` is an alias for either LP's material conditional or RM3's conditional.

The implementation is simple: if the antecedent is F, return T; otherwise, return the consequent. Compare the three
functions while interpreting the middle value as B:

| `a` | `b` | `k3_imply` | `para_imply` | `rm3_imply` |
|-----|-----|------------|--------------|-------------|
| F   | F   | T          | T            | T           |
| F   | B   | T          | T            | T           |
| F   | T   | T          | T            | T           |
| B   | F   | B          | F            | F           |
| B   | B   | B          | B            | B           |
| B   | T   | T          | T            | T           |
| T   | F   | F          | F            | F           |
| T   | B   | B          | B            | F           |
| T   | T   | T          | T            | T           |

`para_imply` differs from LP's material conditional at B → F. It differs from RM3 at T → B.

```rust
use balanced_ternary::Digit::{Neg, Pos, Zero};

assert_eq!(Zero.k3_imply(Neg), Zero);
assert_eq!(Zero.para_imply(Neg), Neg);
assert_eq!(Zero.rm3_imply(Neg), Neg);

assert_eq!(Pos.para_imply(Zero), Zero);
assert_eq!(Pos.rm3_imply(Zero), Neg);

for a in [Neg, Zero, Pos] {
for b in [Neg, Zero, Pos] {
let expected = if a == Neg { Pos } else { b }; assert_eq ! (a.para_imply(b), expected); if a != Neg & & a.para_imply(b) != Neg {
assert_ne ! (b, Neg); // Modus ponens preserves {B, T}.
}
}
}
```

With `!`, `&`, `|` and designated set `{B, T}`, adding this conditional still leaves the counterexample to explosion
from the LP subsection intact. That makes a concrete paraconsistent interpretation possible. It does not establish that
every operation elsewhere in the crate belongs to that same system.

This tutorial describes `para_imply` by its actual table rather than assigning it a more specific calculus name that the
API does not document.

### 6.8. McCarthy's sequential logic

McCarthy's conjunction and disjunction make the left operand decisive in a way that models evaluation order:

- For AND, F settles the result as F, T selects the right value, and U stays U.
- For OR, T settles the result as T, F selects the right value, and U stays U.

| `a` | `b` | `mc_and` | `mc_or` |
|-----|-----|----------|---------|
| F   | F   | F        | F       |
| F   | U   | F        | U       |
| F   | T   | F        | T       |
| U   | F   | U        | U       |
| U   | U   | U        | U       |
| U   | T   | U        | U       |
| T   | F   | F        | T       |
| T   | U   | U        | T       |
| T   | T   | T        | T       |

Unlike strong Kleene and Bochvar conjunction/disjunction, these operations are not commutative:

```rust
use balanced_ternary::Digit::{Neg, Pos, Zero};

assert_eq!(Neg.mc_and(Zero), Neg);
assert_eq!(Zero.mc_and(Neg), Zero);
assert_eq!(Pos.mc_or(Zero), Pos);
assert_eq!(Zero.mc_or(Pos), Zero);
```

This makes a difference when a left-hand expression must return a definite result before the right-hand expression is
consulted. The mathematical connection with sequential Kleene logic is described
in [Indrzejczak and Petrukhin, section 2](https://link.springer.com/chapter/10.1007/978-3-031-38499-8_19).

However, Rust method arguments are evaluated before the method runs. **`left.mc_and(expensive_check())` does not
short-circuit.** The method implements a truth table over values that have already been computed.

Use closures when evaluation itself must be conditional:

```rust
use balanced_ternary::Digit;
use Digit::{Neg, Pos, Zero};

fn lazy_mc_and(left: Digit, right: impl FnOnce() -> Digit) -> Digit {
    match left {
        Pos => right(),
        Neg | Zero => left,
    }
}

fn lazy_mc_or(left: Digit, right: impl FnOnce() -> Digit) -> Digit {
    match left {
        Neg => right(),
        Pos | Zero => left,
    }
}

let mut called = false; let result = lazy_mc_and(Neg, | | {
called = true; Pos
}); assert_eq!(result, Neg);
assert!(!called);
assert_eq!(lazy_mc_or(Neg, || Pos), Pos);
assert_eq!(lazy_mc_or(Zero, || Pos), Zero);
```

A returned `Zero` is still an ordinary value. Neither these adapters nor the crate can detect that an arbitrary Rust
computation would fail to terminate and automatically replace it with `Zero`.

### 6.9. Post's cyclic operations

Classical-style negation exchanges F and T and leaves the middle state fixed. Post's cyclic negation instead permutes
all three states. In this crate, `post()` advances through:

**`Neg` → `Zero` → `Pos` → `Neg`.**

`pre()` moves in the opposite direction. Compare them with the other negations:

| `a` | `!a` | `ht_not()` | `post()` | `pre()` |
|-----|------|------------|----------|---------|
| F   | T    | T          | M        | T       |
| M   | M    | F          | T        | F       |
| T   | F    | F          | F        | M       |

```rust
use balanced_ternary::Digit::{Neg, Pos, Zero};

for a in [Neg, Zero, Pos] {
assert_eq ! (a.post().post().post(), a);
assert_eq ! (a.post().pre(), a); assert_eq ! (a.pre().post(), a); assert_eq ! (a.post(), a + Pos);
}

assert_eq!(!Zero, Zero);
assert_eq!(Zero.post(), Pos);
assert_eq!(Zero.post().post(), Neg);
```

Three applications restore the original state. Two applications do not behave like double classical negation. The
equality with `a + Pos` is an equality of single-trit functions under the crate's wrapping arithmetic.

Post's many-valued systems have their own algebraic
setting; [Indrzejczak and Petrukhin](https://link.springer.com/chapter/10.1007/978-3-031-38499-8_19) discuss cyclic and
dual cyclic negation, and [Leyva's thesis](https://digitalcommons.usf.edu/etd/7844/) develops Post systems and algebras.
Here we use the concrete cyclic operations provided by the crate without implying that calling `post()` selects an
entire logical calculus.

A direct programming use is a three-state rotation, provided you deliberately choose this ordering. Do not apply
`post()` to an “unknown” value and interpret the resulting `Pos` as newly discovered evidence: the operation changed the
state by definition.

### 6.10. Compare and choose

The following table places the six built-in implication methods side by side. **M is only a shared label for the middle
encoding**, not a claim that every logic gives it the same meaning.

| `a` | `b` | `k3_imply` | `bi3_imply` | `l3_imply` | `ht_imply` | `rm3_imply` | `para_imply` |
|-----|-----|------------|-------------|------------|------------|-------------|--------------|
| F   | F   | T          | T           | T          | T          | T           | T            |
| F   | M   | T          | M           | T          | T          | T           | T            |
| F   | T   | T          | T           | T          | T          | T           | T            |
| M   | F   | M          | M           | M          | F          | F           | F            |
| M   | M   | M          | M           | T          | T          | M           | M            |
| M   | T   | T          | M           | T          | T          | T           | T            |
| T   | F   | F          | F           | F          | F          | F           | F            |
| T   | M   | M          | M           | M          | M          | F           | M            |
| T   | T   | T          | T           | T          | T          | T           | T            |

The differences concentrate on the middle value. On entirely Boolean inputs F and T, all six agree.

Use these distinctions to select an interpretation:

| Your intended behavior                                                         | Candidate operations            | Decision to make                                       |
|--------------------------------------------------------------------------------|---------------------------------|--------------------------------------------------------|
| A decisive known operand can settle an expression with missing information     | Strong Kleene                   | Keep U until the decision boundary                     |
| Every undefined component invalidates the compound expression                  | Bochvar internal / weak Kleene  | Use `bi3_*` consistently                               |
| The middle value is a truth degree and self-implication should be T            | Łukasiewicz                     | Distinguish lattice from strong conjunction            |
| You are evaluating formulas under the two-world HT interpretation              | `ht_not`, `ht_imply`, `&`, `\|` | Preserve the here/there semantics                      |
| Contradictory support should not entail everything                             | LP or RM3                       | Treat B as designated and choose the conditional       |
| You need the specific conditional “F gives T; otherwise return the consequent” | `para_imply`                    | State the interpretation and designated set explicitly |
| Left-to-right definedness matters                                              | McCarthy                        | Distinguish eager table evaluation from lazy execution |
| You need a cyclic permutation of three states                                  | Post-style `post` / `pre`       | Define the cycle's application meaning                 |

These are starting points, not interchangeable presets. For example, replacing K3's implication with L3's while
retaining all other assumptions changes the system; replacing its interpretation of U with LP's B changes it in another
way.

A small evaluator should bundle its chosen negation, conjunction, disjunction, implication, and designated-value
predicate. Checking those functions over three or nine cases is cheap and makes the intended semantics reviewable.

### 6.11. Application-oriented operations

Other operations in the crate are useful without belonging to one of the preceding logical systems:

| Method       | Rule                                                | Example                       |
|--------------|-----------------------------------------------------|-------------------------------|
| `coalesce`   | Keep the left value unless it is zero               | `Zero.coalesce(Pos) == Pos`   |
| `consensus`  | Keep an agreed value; otherwise return zero         | `Pos.consensus(Neg) == Zero`  |
| `compatible` | Reject opposite nonzero signs                       | `Zero.compatible(Neg) == Pos` |
| `compare`    | Encode the ordering of the two values               | `Neg.compare(Pos) == Neg`     |
| `toward`     | Move one step toward the other value                | `Neg.toward(Pos) == Zero`     |
| `change`     | Return the other value if different; otherwise zero | `Neg.change(Pos) == Pos`      |
| `duel`       | Return the winner under a cyclic rule               | `Neg.duel(Pos) == Neg`        |

`toward` and `change` return new values; neither mutates its receiver. They answer different questions:
`toward` computes the next step toward a target, while `change` returns a different target value or zero when unchanged.

```rust
use balanced_ternary::Digit::{Neg, Pos, Zero};

assert_eq!(Neg.toward(Pos), Zero);
assert_eq!(Zero.toward(Pos), Pos);
assert_eq!(Pos.toward(Pos), Pos);

assert_eq!(Neg.change(Pos), Pos);
assert_eq!(Zero.change(Pos), Pos);
assert_eq!(Pos.change(Pos), Zero);
assert_eq!(Pos.change(Zero), Zero);
```

`change` returns a `Digit`, not a Boolean change indicator. A change to `Zero` and an unchanged value both produce
`Zero`; use `a != b` when you need to detect whether any change occurred.

For `duel`, `Zero` beats `Neg`, `Pos` beats `Zero`, and `Neg` beats `Pos`: a rock-paper-scissors rule, not an ordinary
maximum.

`coalesce` is appropriate only if zero means “use a fallback.” A measured numeric zero may be a perfectly valid value
that should not be replaced. Likewise, `consensus` deliberately loses the distinction between disagreement and absent
information.

The arithmetic, comparison, selection, and movement operations remain useful, but they do not acquire logical semantics
merely because their input and output types are `Digit`.

## 7. Build a rule evaluator

Suppose a product can be shipped when:

> Stock is available AND (delivery is available OR pickup is available).

An unfinished check becomes `None` at the application boundary. We convert it to `Zero`, evaluate with strong Kleene
operators, and retain all three outcomes.

Replace `src/main.rs` with this complete program:

```rust
use balanced_ternary::Digit;
use Digit::{Neg, Pos, Zero};

#[derive(Debug, PartialEq, Eq)]
enum Decision {
    Ready,
    Unavailable,
    Pending,
}

fn observation(value: Option<bool>) -> Digit {
    match value {
        Some(false) => Neg,
        None => Zero,
        Some(true) => Pos,
    }
}

fn fulfillment(
    stock: Option<bool>,
    delivery: Option<bool>,
    pickup: Option<bool>,
) -> Digit {
    observation(stock) & (observation(delivery) | observation(pickup))
}

fn decide(value: Digit) -> Decision {
    match value {
        Pos => Decision::Ready,
        Neg => Decision::Unavailable,
        Zero => Decision::Pending,
    }
}

fn main() {
    let cases = [
        // Known lack of stock settles the result.
        (Some(false), None, None, Decision::Unavailable),
        // Pickup provides a known fulfillment option.
        (Some(true), None, Some(true), Decision::Ready),
        // There might be a delivery option, but it is not confirmed.
        (Some(true), None, Some(false), Decision::Pending),
        // No fulfillment option is available, regardless of stock.
        (None, Some(false), Some(false), Decision::Unavailable),
    ];

    for (stock, delivery, pickup, expected) in cases {
        let result = decide(fulfillment(stock, delivery, pickup));
        assert_eq!(result, expected);
        println!("{result:?}");
    }
}
```

Notice that the decision is not computed by counting positive and negative checks. Each input represents a different
proposition, and the expression states how those propositions combine.

For larger applications, keep these operations behind domain functions like `fulfillment`. That prevents an accidental
arithmetic operation or a different implication family from silently changing your rules.

## 8. Read balanced ternary numbers

Now change interpretation: `Neg`, `Zero`, and `Pos` represent **−1, 0, and +1**. Zero is an ordinary number here, not an
unknown truth value.

Like decimal and binary, balanced ternary is positional. From right to left, the weights are `1, 3, 9, 27, ...`. Each
trit contributes its signed value multiplied by its position's weight.

For example:

```text
+-- = (+1 × 9) + (−1 × 3) + (−1 × 1) = 5
```

A `-` inside a balanced numeral is a digit with value −1. It is not a separator or a unary minus sign attached to the
following digits.

| Decimal | Balanced ternary | Evaluation |
|--------:|------------------|------------|
|      −5 | `-++`            | −9 + 3 + 1 |
|      −2 | `-+`             | −3 + 1     |
|      −1 | `-`              | −1         |
|       0 | `0`              | 0          |
|       1 | `+`              | 1          |
|       2 | `+-`             | 3 − 1      |
|       3 | `+0`             | 3          |
|       4 | `++`             | 3 + 1      |
|       5 | `+--`            | 9 − 3 − 1  |
|       8 | `+0-`            | 9 − 1      |
|       9 | `+00`            | 9          |

Negating a numeral exchanges every `+` and `-`; the zeros stay in place. No separate sign bit is needed.

### Why carries work differently

The sum `1 + 1 = 2` cannot fit into one balanced trit. Write `2` as `3 − 1`: result digit `-`, carry digit `+`. The
two-digit result is `+-`.

The `Digit` addition operator returns only one digit and wraps modulo three:

```rust
use balanced_ternary::{Digit, Ternary};
use Digit::{Neg, Pos, Zero};

assert_eq!(Pos + Pos, Neg); // Single-trit wrapping result.
assert_eq!(Neg + Neg, Pos);
assert_eq!(Pos + Neg, Zero);

let one = Ternary::from_dec(1); let two = & one + & one;
assert_eq!(two.to_string(), "+-"); // The carry is retained.
assert_eq!(two.to_dec(), 2);
```

Addition is therefore not OR. `Pos | Pos` is `Pos`, while `Pos + Pos` is `Neg`.

### Division by zero is not a fourth state

For a nonzero `Digit` divisor, division and multiplication give the same result because the only possible divisors are
−1 and +1, each its own reciprocal. Their domains differ: **division by `Zero` panics**.

```rust
use balanced_ternary::Digit;
use Digit::{Neg, Pos, Zero};

fn checked_digit_div(a: Digit, b: Digit) -> Option<Digit> {
    if b == Zero {
        None
    } else {
        Some(a / b)
    }
}

assert_eq!(checked_digit_div(Pos, Neg), Some(Neg));
assert_eq!(checked_digit_div(Zero, Pos), Some(Zero));
assert_eq!(checked_digit_div(Pos, Zero), None);
```

This helper is application code; `Digit` has no inherent `checked_div` method in 3.2.0. A failure marker such as `!` in
an arithmetic table denotes an invalid operation, not an additional `Digit`. Mapping that failure to logical unknown
would be a separate policy, not the behavior of `/`.

## 9. Calculate with `Ternary`

`Ternary` stores a growable sequence of digits and performs arbitrary-precision signed integer arithmetic. “Arbitrary
precision” means its arithmetic is not restricted to a machine integer's width. Available memory and computation time
still limit practical sizes.

### Parse, display, and convert

```rust
use balanced_ternary::Ternary;

let five = Ternary::try_parse("+--").unwrap(); let eight: Ternary = "+0-".parse().unwrap();

assert_eq!(five.checked_to_dec(), Some(5));
assert_eq!(eight.checked_to_dec(), Some(8));
assert_eq!(Ternary::from_dec(-5).to_string(), "-++");

assert!(Ternary::try_parse("102").is_err());
assert!(Ternary::try_parse(" +-- ").is_err());
```

Balanced parsing accepts only `+`, `0`, and `-`; it does not trim whitespace. Call `input.trim()` yourself if the input
format permits surrounding whitespace.

`Ternary::parse` and the helper `ter` are convenient for trusted literals, but panic on invalid syntax. `try_parse` and
the `FromStr` implementation return errors, including the invalid character and its UTF-8 byte offset.

### Arithmetic borrows its operands

Use references with `Ternary` operators. The following operations produce new values without consuming `a` or `b`:

```rust
use balanced_ternary::Ternary;

let a = Ternary::from_dec(8); let b = Ternary::from_dec(3);

assert_eq!((&a + &b).to_dec(), 11);
assert_eq!((&a - &b).to_dec(), 5);
assert_eq!((&a * &b).to_dec(), 24);
assert_eq!((&a / &b).to_dec(), 2);
assert_eq!((&a % &b).to_dec(), 2);
assert_eq!((-&a).to_dec(), -8);

assert_eq!(a.to_dec(), 8); // Still usable.
```

Division is integer division, not rational arithmetic. The quotient truncates toward zero. A nonzero remainder has the
dividend's sign.

```rust
use balanced_ternary::Ternary;

let a = Ternary::from_dec( - 8); let b = Ternary::from_dec(3); let (q, r) = a.div_rem( & b);

assert_eq!(q.to_dec(), -2);
assert_eq!(r.to_dec(), -2);
assert_eq!(&(&q * &b) + &r, a);
```

Use `div_rem` when you need both results. Division and remainder by any representation of zero panic, including `""` and
`"000"`. In this version, validate a possibly zero `Ternary` divisor before using `/`, `%`, or `div_rem`; there is no
inherent `Ternary::checked_div`.

### Work beyond `i64`

The `from_dec` input and `to_dec` output are `i64`, but they do not define the arithmetic's precision limit. Construct
larger values from digits or ordinary base-3 strings:

```rust
use balanced_ternary::Ternary;

let huge = Ternary::try_parse( & "+".repeat(100)).unwrap(); let three = Ternary::from_dec(3); let product = & huge * & three;

assert_eq!(&product / &three, huge);
assert_eq!(product.checked_to_dec(), None);
assert!(i64::try_from(&product).is_err());
```

`checked_to_dec()` returns `None` when a number does not fit in `i64`; `to_dec()` panics in that situation. Keep a large
value as `Ternary`, or format it in balanced or ordinary base three. Do not round-trip through `i64` to perform
arithmetic.

### Ordinary ternary is a different notation

Ordinary, unbalanced base three uses digits `0`, `1`, and `2`, with a separate minus sign for negative values:

```rust
use balanced_ternary::{Digit, Ternary};

let five = Ternary::try_from_unbalanced("12").unwrap(); let minus_five = Ternary::try_from_unbalanced("-12").unwrap();

assert_eq!(five.to_string(), "+--");
assert_eq!(minus_five.to_string(), "-++");
assert_eq!(five.to_unbalanced(), "12");
assert_eq!(minus_five.to_unbalanced(), "-12");

// The Digit method with the same name is an offset encoding.
assert_eq!(Digit::Neg.to_unbalanced(), 0);
assert_eq!(Digit::Zero.to_unbalanced(), 1);
assert_eq!(Digit::Pos.to_unbalanced(), 2);
```

`Ternary::to_unbalanced()` is numeric base conversion. `Digit::to_unbalanced()` maps the three states to codes
`0, 1, 2`. Applying the latter independently to every digit does **not** convert a balanced numeral into ordinary base
three.

## 10. Separate numeric values from representations

A number can have several padded representations. `Ternary` deliberately keeps some representation details while
comparing by numeric value.

```rust
use balanced_ternary::Ternary;

let padded = Ternary::try_parse("00+--").unwrap(); let plain = Ternary::try_parse("+--").unwrap();

assert_eq!(padded, plain);
assert_eq!(padded.to_string(), "00+--");
assert_ne!(padded.to_digit_slice(), plain.to_digit_slice());
assert_eq!(padded.trim().to_string(), "+--");
assert_eq!(plain.with_length(6).to_string(), "000+--");
assert_eq!(padded.with_length(1).to_string(), "00+--");
```

`with_length(n)` means **at least** `n` digits. It pads on the left and never truncates.

The contracts are:

| Operation                     | Treatment of representation                        |
|-------------------------------|----------------------------------------------------|
| Parsing and `Display`         | Preserve the input digits, including leading zeros |
| `==`, ordering, and hashing   | Use the numeric value                              |
| `trim()`                      | Return canonical digits; zero becomes `"0"`        |
| Binary arithmetic `+ - * / %` | Return canonical results                           |
| Unary negation                | Flip signs and preserve width                      |
| `each` and `each_zip`         | Transform stored positions, including padding      |

An empty balanced string is valid and has numeric value zero:

```rust
use balanced_ternary::Ternary;

let empty = Ternary::try_parse("").unwrap(); assert_eq!(empty, Ternary::from_dec(0));
assert_eq!(empty.to_string(), "");
assert_eq!(empty.trim().to_string(), "0");
assert_eq!(empty.log(), 0);
```

If your application requires at least one input digit, enforce that rule separately. Also, despite its name, `log()`
returns the stored digit count, not a mathematical logarithm.

### Indexing has two directions

Borrowed slices and iterators run from most significant to least significant. Digit lookup counts from the right:

```rust
use balanced_ternary::{Digit, Ternary};
use balanced_ternary::concepts::DigitOperate;

let value = Ternary::parse("+0-");

assert_eq!(value.to_digit_slice()[0], Digit::Pos);
assert_eq!(value.get_digit(0), Some(&Digit::Neg));
assert_eq!(value.digit(0), Some(Digit::Neg));
assert_eq!(value.digit(3), None);
```

Use `to_digit_slice()` or `iter()` to inspect existing digits without allocating a new vector. `to_digits()` produces an
owned vector.

### A right shift is not truncating integer division

Ternary shifts add or remove digit positions. Left shifting by one multiplies the numeric value by three. Right shifting
removes the least significant digit; this is not generally the same as integer division toward zero:

```rust
use balanced_ternary::Ternary;

let two = Ternary::parse("+-"); let three = Ternary::from_dec(3);

assert_eq!((&two << 1).to_dec(), 6);
assert_eq!((&two >> 1).to_dec(), 1); // Removes '-', leaving '+'.
assert_eq!((&two / &three).to_dec(), 0);
```

Use `/` when you want integer division and shifts when you intend to manipulate digit positions.

## 11. Transform collections of trits

Import `DigitOperate` to use the common digit-wise methods on `Ternary`, `Tryte<N>`, and `Ter40`.

| Method                     | Callback input                          | Purpose                                          |
|----------------------------|-----------------------------------------|--------------------------------------------------|
| `each(f)`                  | One digit                               | Apply a unary operation everywhere               |
| `each_with(f, d)`          | Current digit, constant digit           | Combine every position with the same value       |
| `each_zip(f, other)`       | Corresponding left and right digits     | Combine two representations position by position |
| `each_zip_carry(f, other)` | Left digit, right digit, incoming carry | Propagate a carry from right to left             |

### Apply truth functions position by position

```rust
use balanced_ternary::{Digit, Ternary};
use balanced_ternary::concepts::DigitOperate;

let states = Ternary::parse("+0-"); assert_eq!(states.each(Digit::necessary).to_string(), "+--");
assert_eq!(states.each(Digit::possibly).to_string(), "++-");
assert_eq!(states.each_with(Digit::coalesce, Digit::Pos).to_string(), "++-");

let fallback = Ternary::parse("-++"); assert_eq!(states.each_zip(Digit::coalesce, fallback).to_string(), "++-");
```

Here the positions are independent states. Their positional numeric value is irrelevant to the application.

For `Ternary::each_zip`, shorter operands are padded with `Zero` **on the left**, aligning their least significant
digits. The result has the longer input's width:

```rust
use balanced_ternary::{Digit, Ternary};
use balanced_ternary::concepts::DigitOperate;

let left = Ternary::parse("++"); let right = Ternary::parse("+"); // Aligned as "0+".
let result = left.each_zip(Digit::k3_equiv, right);

assert_eq!(result.to_string(), "0+");
```

For logical records, an inserted `Zero` means an inserted unknown state under our interpretation. Check lengths
explicitly when every position is supposed to have a corresponding field.

**Numeric equality does not make digit-wise transformations interchangeable.** `"+"` and `"0+"` are numerically equal,
but mapping `possibly` produces `"+"` and `"++"`. If width is part of your data model, compare slices, use a fixed-width
word, or wrap the representation in a domain type.

### Carry propagation is different from independent digit addition

Adding each pair of trits without a carry is not multi-digit arithmetic:

```rust
use balanced_ternary::Ternary;
use balanced_ternary::concepts::DigitOperate;

let a = Ternary::parse("++"); // 4
let b = Ternary::parse("0+"); // 1

assert_eq!(a.each_zip(|x, y| x + y, b.clone()).to_string(), "+-");
assert_eq!((&a + &b).to_string(), "+--"); // 5
```

For a custom carry operation, the callback returns **`(next_carry, result_digit)`**:

```rust
use balanced_ternary::{Digit, Ternary};
use balanced_ternary::concepts::DigitOperate;

fn add_column(a: Digit, b: Digit, carry: Digit) -> (Digit, Digit) {
    let sum = a.to_i8() + b.to_i8() + carry.to_i8();
    let next_carry = (sum + 1).div_euclid(3);
    let result = sum - 3 * next_carry;
    (Digit::from_i8(next_carry), Digit::from_i8(result))
}

let a = Ternary::parse("++"); let b = Ternary::parse("0+"); let result = a.each_zip_carry(add_column, b); assert_eq!(result.to_string(), "+--");
```

The incoming carry starts at zero. `Ternary` prepends a nonzero final carry. Fixed-width types discard it through the
trait method; their inherent `each_zip_carry_full` returns **`(result_word, outgoing_carry)`** instead. Notice that this
outer tuple has a different order from the callback's tuple.

For normal addition, use `+` or the arithmetic methods. The callback is useful when implementing a different digit
algorithm.

## 12. Use fixed-width words with `Tryte<N>`

`Tryte<N>` contains exactly `N` trits in an array. The default width is six, but the type is not restricted to six or
forty trits.

For positive `N`, its signed range is:

```text
−(3^N − 1) / 2  through  +(3^N − 1) / 2
```

There are `3^N` distinct values, with a symmetric range around zero.

| Type       | Range             |
|------------|-------------------|
| `Tryte<1>` | −1 through +1     |
| `Tryte<2>` | −4 through +4     |
| `Tryte<5>` | −121 through +121 |
| `Tryte<6>` | −364 through +364 |

### Construct a word and preserve its width

```rust
use balanced_ternary::{Digit, Tryte};

let word = Tryte::<6 >::from_i64(5); assert_eq!(word.to_string(), "000+--");
assert_eq!(word.to_i64(), 5);

let states = Tryte::<3 >::new([Digit::Pos, Digit::Zero, Digit::Neg]); assert_eq!(states.to_string(), "+0-");
assert_eq!("00+--".parse::<Tryte<6>>().unwrap(), word);

assert_eq!(Tryte::<2>::checked_from_i64(5), None);
assert!("+--".parse::<Tryte<2>>().is_err());
```

Numeric constructors check whether the value fits. They **do not wrap** just because some arithmetic operators do.
Redundant leading zeros do not cause an otherwise fitting numeric value to be rejected.

`Tryte<N>` is `Copy`, but copying a very large word still has a cost proportional to its size. Its array storage is not
compressed five trits per byte.

### Choose an overflow policy

`+`, `-`, and `*` wrap modulo `3^N`. For `Tryte<2>`, the modulus is nine, and the result is represented in the range −4
through +4. Therefore `4 + 1` wraps to −4.

```rust
use balanced_ternary::{Digit, Tryte};

type Word = Tryte<2>;
let one = Word::from_i64(1);

assert_eq!(Word::MAX + one, Word::MIN);
assert_eq!(Word::MAX.wrapping_add(one), Word::MIN);
assert_eq!(Word::MAX.checked_add(one), None);
assert_eq!(Word::MAX.overflowing_add(one), (Word::MIN, true));
assert_eq!(Word::MAX.saturating_add(one), Word::MAX);

let (result, carry) = Word::MAX.add_with_carry(one); assert_eq!(result, Word::MIN);
assert_eq!(carry, Digit::Pos);
assert_eq!(result.to_i64() + i64::from(carry.to_i8()) * 9, 5);
```

| Family          | On overflow                          |
|-----------------|--------------------------------------|
| `wrapping_*`    | Keep the modular result              |
| `checked_*`     | Return `None`                        |
| `overflowing_*` | Return the wrapped result and `true` |
| `saturating_*`  | Clamp to `MIN` or `MAX`              |

These distinctions apply to addition, subtraction, and multiplication. Division has its own rules, covered next.

### Wider than a machine integer

```rust
use balanced_ternary::{Digit, Tryte};

let wide = Tryte::<80 >::new([Digit::Pos; 80]); assert_eq!(wide.checked_to_i64(), None);

let small_in_a_wide_word = Tryte::<80 >::from_i64(5); assert_eq!(small_in_a_wide_word.checked_to_i64(), Some(5));
```

The conversion limit depends on the numeric value, not just the array's width. Use `checked_to_i64` when a wide word
might exceed the machine range.

`Tryte<0>` is also defined: it is an empty word representing only zero. For ordinary application code, choose a positive
width.

## 13. Distinguish integer and modular division

These operations answer different questions.

**Integer division** asks for a quotient truncated toward zero:

```rust
use balanced_ternary::Tryte;

type Word = Tryte<2>;
let one = Word::from_i64(1); let two = Word::from_i64(2);

assert_eq!(one / two, Word::ZERO);
assert_eq!(one.checked_div(two), Some(Word::ZERO));
assert_eq!(one.checked_div(Word::ZERO), None);
assert_eq!(Word::MIN / Word::from_i64(-1), Word::MAX);
```

Because the range is symmetric, negating `MIN` fits. A nonzero divisor cannot cause integer quotient overflow. `/` still
panics on a zero divisor; `checked_div` handles that case explicitly.

**Modular division** asks for an inverse modulo `3^N`, then multiplies by that inverse. For positive `N`, an integer has
such an inverse exactly when it is not divisible by three.

For a two-trit word, calculations are modulo nine. The inverse of two is represented by −4, since `2 × (−4) = −8`, which
is congruent to one modulo nine.

```rust
use balanced_ternary::Tryte;

type Word = Tryte<2>;
let one = Word::from_i64(1); let two = Word::from_i64(2); let three = Word::from_i64(3);

assert_eq!(two.checked_modular_inverse(), Some(Word::from_i64(-4)));
assert_eq!(one.checked_modular_div(two), Some(Word::from_i64(-4)));
assert_eq!(three.checked_modular_inverse(), None);
assert_eq!(three.checked_modular_div(three), None);
```

The last call returns `None` even though the congruence `3x = 3 (mod 9)` has solutions. This method performs division
through a multiplicative inverse; it is not a general solver for congruences with non-invertible divisors.

Use integer division for ordinary quotients. Use modular division when the mathematical model is explicitly arithmetic
modulo `3^N`. The `wrapping_div` name does not select modular division; use `checked_modular_div` for that operation.

## 14. Use a compact bounded integer with `Ter40`

`Ter40` stores a signed integer in an `i64`, restricted to the range of forty balanced trits:

```text
−6,078,832,729,528,464,400 through +6,078,832,729,528,464,400
```

It occupies eight bytes and supports arithmetic and digit-wise transformations. Unlike `Tryte<40>`, its arithmetic
operators **panic on overflow** instead of wrapping.

```rust
use balanced_ternary::Ter40;

let a = Ter40::from_dec(42); let b = Ter40::from_dec(10);

assert_eq!((a + b).to_dec(), 52);
assert_eq!((a / b).to_dec(), 4);
assert_eq!(Ter40::MAX.checked_add(Ter40::from_dec(1)), None);
assert_eq!(a.checked_div(Ter40::ZERO), None);
assert_eq!(Ter40::checked_from_dec(i64::MAX), None);
assert_eq!(core::mem::size_of::<Ter40>(), 8);
assert_eq!(a.to_ternary().log(), 40);
```

`checked_add`, `checked_sub`, `checked_mul`, and `checked_div` make failure explicit. A valid `Ter40` always fits in
`i64`, so its `to_dec()` does not need an overflow check.

| Property               | `Tryte<40>`             | `Ter40`                            |
|------------------------|-------------------------|------------------------------------|
| Storage                | Array of forty `Digit`s | One `i64`                          |
| Signed numeric range   | Same forty-trit range   | Same forty-trit range              |
| `+`, `-`, `*` overflow | Wrap                    | Panic                              |
| Arbitrary width        | Choose a different `N`  | Always forty trits                 |
| Digit access           | Array-based             | Decodes the integer representation |

Pick the type for its semantics and workload. A smaller representation does not automatically make every operation
faster; repeated digit-wise work and integer arithmetic have different costs.

## 15. Store trits compactly

Numeric computation, fixed-width words, and compact storage serve different purposes. The storage types pack trits more
densely than an array of `Digit`s.

### Five trits in one byte: `TritsChunk`

Five trits have `3^5 = 243` possible combinations, which fit into the 256 bit patterns of a byte. `TritsChunk` stores
the signed numeric value of a five-trit group in an `i8`, with valid values from −121 through +121.

```rust
use balanced_ternary::TritsChunk;

let chunk = TritsChunk::from_dec(5);

assert_eq!(core::mem::size_of::<TritsChunk>(), 1);
assert_eq!(chunk.to_dec(), 5);
assert_eq!(chunk.to_ternary().to_string(), "+--");
assert_eq!(chunk.to_fixed_ternary().to_string(), "00+--");
assert_eq!(TritsChunk::checked_from_dec(122), None);
```

The normalized and fixed-width conversions answer different questions. Use `to_fixed_ternary()` if all five positions
matter.

### A growable sequence of chunks: `DataTernary`

`DataTernary` stores a vector of five-trit chunks. It is a storage representation, not an arithmetic type.

```rust
use balanced_ternary::{DataTernary, Ternary};

let original = Ternary::try_parse("+0-").unwrap(); let stored = DataTernary::from_ternary(original);

assert_eq!(stored.to_string(), "00+0-");
assert_eq!(stored.to_fixed_ternary().to_string(), "00+0-");
assert_eq!(stored.to_ternary().to_string(), "+0-");
assert_eq!(stored.checked_to_dec(), Some(8));
```

Encoding pads on the left to a multiple of five trits. For `n` input trits, the chunk payload needs `ceil(n / 5)` bytes.
The `Vec` itself, its capacity, and allocator overhead are additional costs. The saving concerns the stored payload, not
a universal fivefold reduction in total memory use.

### Storage equality includes the number of chunks

```rust
use balanced_ternary::{DataTernary, Ternary};

let short = DataTernary::from_ternary(Ternary::parse("+")); let long = DataTernary::from_ternary(Ternary::parse("00000+"));

assert_ne!(short, long);
assert_eq!(short.to_ternary(), long.to_ternary());
assert_eq!(short.to_string(), "0000+");
assert_eq!(long.to_string(), "000000000+");

let restored = DataTernary::from_ternary(long.to_fixed_ternary()); assert_eq!(restored, long);
```

`DataTernary` equality and hashing include the stored chunk sequence. It has no numeric ordering or arithmetic operators
in this version. Convert to `Ternary` for those operations.

Also note that `DataTernary::to_digits()` returns **normalized** digits. Unlike `DigitOperate::to_digits()` on a
fixed-width word, it does not preserve the padded storage width. Decode with `to_fixed_ternary()` when width matters.

### Original width must be stored separately

`"+"`, `"0+"`, and `"0000+"` all encode to the same one-chunk representation. The chunk count preserves the padded
width, not the original width.

For a logical record, those leading zeros might be real unknown fields. Record its original length alongside the packed
value:

```rust
use balanced_ternary::{DataTernary, Ternary};

let states = Ternary::parse("0+0-"); let original_width = states.log(); let stored = DataTernary::from_ternary(states.clone());

let padded = stored.to_fixed_ternary(); let digits = padded.to_digit_slice(); assert!(original_width <= digits.len());
let recovered = Ternary::new(digits[digits.len() - original_width..].to_vec());

assert_eq!(recovered.to_digit_slice(), states.to_digit_slice());
assert_eq!(recovered.to_string(), "0+0-");
```

Here the length is trusted because we created it ourselves. Validate externally supplied metadata before slicing.

The empty-storage case has its own contract:

```rust
use balanced_ternary::DataTernary;

let empty = DataTernary::default (); let stored_zero = DataTernary::from_dec(0);

assert_ne!(empty, stored_zero);
assert_eq!(empty.checked_to_dec(), Some(0));
assert_eq!(empty.to_fixed_ternary().to_string(), "");
assert_eq!(empty.to_ternary().to_string(), "0");
```

Finally, compact in-memory storage is not automatically a file or network format. Version 3.2.0 does not expose a
general `DataTernary` byte serialization API. Define a wire format and validation rules explicitly if persistence or
interoperability is your goal; do not reinterpret the private `Vec` layout as serialized data.

## 16. Configure features and `no_std`

The crate is declared `#![no_std]`. That means it does not depend on the Rust standard library; it does **not** mean
that every API avoids dynamic allocation.

Rust's [`alloc` crate](https://doc.rust-lang.org/alloc/) supplies heap-backed collections such as `Vec` and `String`
independently of `std`. Using those collections on a target without `std` requires an allocator supplied by the final
application or its runtime. A library can expose such APIs without providing the final platform integration itself.

### Select the API surface with Cargo features

| Feature selection    | Available main types                                                    |
|----------------------|-------------------------------------------------------------------------|
| No optional features | `Digit` and the `DigitOperate` trait                                    |
| `ternary-string`     | Adds `Ternary`                                                          |
| `tryte`              | Adds `Tryte<N>` and enables `ternary-string`                            |
| `ternary-store`      | Adds `TritsChunk`, `DataTernary`, `Ter40`, and enables `ternary-string` |
| Default              | Enables `tryte` and `ternary-store`, therefore all these types          |

The name `ternary-string` is historical/API naming: it enables the growable numeric `Ternary` type, not merely
formatting functions.

For digit-only application logic, use:

```toml
[dependencies]
balanced-ternary = { version = "=3.2.0", default-features = false }
```

Put this in a library's `src/lib.rs`:

```rust
#![no_std]

use balanced_ternary::Digit;

pub fn all_required(a: Digit, b: Digit) -> Digit {
    a & b
}

pub fn confirmed(value: Digit) -> bool {
    value == Digit::Pos
}
```

Those functions perform no heap allocation.

**Version-specific distinction:** in 3.2.0, the crate declares `extern crate alloc` unconditionally, and
`DigitOperate::to_digits()` returns a `Vec`. Disabling default features does not make the dependency strictly `core`
-only. Non-allocating operations and allocator-free final linking are separate claims; check the latter in your actual
firmware build.

### Fixed-width operations can avoid allocating

Enable fixed-width words with:

```toml
[dependencies]
balanced-ternary = { version = "=3.2.0", default-features = false, features = ["tryte"] }
```

A library can then operate on an array-backed word:

```rust
#![no_std]

use balanced_ternary::{Digit, Tryte};
use balanced_ternary::concepts::DigitOperate;

pub fn confirmed_fields(states: Tryte<6>) -> Tryte<6> {
    states.each(Digit::necessary)
}

pub fn add_checked(a: Tryte<6>, b: Tryte<6>) -> Option<Tryte<6>> {
    a.checked_add(b)
}
```

These operations use fixed-size storage. By contrast, converting a word to `Ternary`, calling `to_digits()`, or creating
a `String` allocates. Very large `Tryte<N>` values and arithmetic scratch arrays can also consume substantial stack
space even when no heap allocation occurs.

### Growable arithmetic needs `alloc`

For growable numbers without the other optional types:

```toml
[dependencies]
balanced-ternary = { version = "=3.2.0", default-features = false, features = ["ternary-string"] }
```

A `no_std` library using an allocated string can be written as follows:

```rust
#![no_std]

extern crate alloc;

use alloc::string::{String, ToString};
use balanced_ternary::{ParseTernaryError, Ternary};

pub fn double(input: &str) -> Result<String, ParseTernaryError> {
    let value = Ternary::try_parse(input)?;
    Ok((&value + &value).to_string())
}
```

Here `ToString` is imported explicitly because the `std` prelude is absent. The application that uses this function must
provide allocation support where required. A complete bare-metal binary also needs target-specific startup, panic
handling, and linking configuration; this example is a reusable library, not that binary.

### Audit operations, not just types

| Operation or representation                          | Heap allocation behavior in 3.2.0                                    |
|------------------------------------------------------|----------------------------------------------------------------------|
| `Digit` logic and arithmetic                         | No allocation                                                        |
| `Tryte<N>` storage, parsing, and arithmetic          | Fixed arrays; no heap allocation                                     |
| `Tryte<N>::to_digit_slice()`                         | Borrows the array                                                    |
| `Ter40` arithmetic and digit-wise transformations    | No heap allocation                                                   |
| `Ternary` owned digit storage and arithmetic results | Use `Vec`                                                            |
| `DataTernary` chunk storage                          | Uses `Vec`                                                           |
| `to_digits()` producing an owned `Vec`               | Can allocate; use a borrowed view where available                    |
| Conversion to `Ternary` from fixed storage           | Allocates a digit vector                                             |
| `.to_string()`                                       | Creates an allocated `String`                                        |
| `Display` writing to an existing formatter           | Does not itself require creating a `String`; the destination matters |

Cargo features are additive across applicable dependency edges: another dependency can enable features you disabled in
your own declaration. Inspect the resolved graph with `cargo tree -e features` when reducing the API surface. See
the [Cargo feature documentation](https://doc.rust-lang.org/cargo/reference/features.html).

A host `cargo check` checks types and compilation on the host. A target build such as
`cargo check --target thumbv7em-none-eabi`, after installing that target, checks the chosen target's library
compatibility. Neither alone proves that a complete firmware binary links without an allocator.

## 17. Choose types and handle failures

### Choose by the contract you need

| Need                                                    | Start with    | Main consideration                                |
|---------------------------------------------------------|---------------|---------------------------------------------------|
| One truth value, direction, or trit                     | `Digit`       | Give the middle state an explicit meaning         |
| Growable signed arithmetic                              | `Ternary`     | Allocation and numeric equality                   |
| Exactly `N` logical positions or a modular numeric word | `Tryte<N>`    | Fixed width; wrapping `+`, `-`, `*`               |
| Compact bounded numeric arithmetic                      | `Ter40`       | Forty-trit range; checked or panicking arithmetic |
| One packed five-trit group                              | `TritsChunk`  | Signed range −121 through +121                    |
| Compact variable-length storage                         | `DataTernary` | Chunk equality and padded width                   |

`Ternary` and `Tryte<N>` can both carry logical records, but arithmetic and logical transformations are still different
operations. Wrapping the chosen type in a domain struct helps prevent accidental misuse.

### Choose fallible APIs at boundaries

| Situation                              | API or approach                                   |
|----------------------------------------|---------------------------------------------------|
| Parse one external digit               | `Digit::try_from_char`, `Digit::try_from_i8`      |
| Parse a balanced numeral               | `Ternary::try_parse` or `str::parse::<Ternary>()` |
| Parse ordinary base three              | `Ternary::try_from_unbalanced`                    |
| Convert a large `Ternary` to `i64`     | `checked_to_dec()` or `i64::try_from(&value)`     |
| Construct a bounded word               | `Tryte::<N>::checked_from_i64` or `TryFrom`       |
| Handle word overflow                   | `checked_add`, `checked_sub`, `checked_mul`       |
| Divide `Tryte<N>` or `Ter40` safely    | `checked_div`                                     |
| Divide `Digit` or `Ternary` safely     | Validate the divisor first in 3.2.0               |
| Convert a truth value to Boolean       | `try_to_bool()` or an explicit resolution policy  |
| Preserve a packed record's exact width | Store original length; use `to_fixed_ternary()`   |

The examples use `unwrap()` for literals and values whose validity they establish. For user input, propagate `Result`
with `?` or handle the error explicitly.

Keep this distinction in mind: a parse error, an arithmetic failure, and a logical unknown are different outcomes.
`Result` and `Option` can carry failures without redefining `Zero` to mean every kind of problem.

## 18. Exercises and solutions

Try the exercises before reading the solutions. All inputs use the conventions established above.

### Exercise 1: decide without inventing information

Evaluate `(stock & delivery) | override` for these states:

| Stock | Delivery | Override |
|-------|----------|----------|
| T     | U        | F        |
| F     | U        | T        |
| U     | F        | F        |

For each result, decide whether `necessary()` and `possibly()` would accept it.

### Exercise 2: test a logical law exhaustively

Write a loop over all nine pairs of digits and verify De Morgan's law:

```text
!(a AND b) = (!a) OR (!b)
```

Then check whether `a OR !a` always returns `Pos`.

### Exercise 3: predict width and overflow

For `Tryte<3>`, find `MIN`, `MAX`, and the result of `MAX + 1`. Predict the output of `checked_add` and `add_with_carry`
for the same operation.

### Exercise 4: preserve a logical record

Pack `"00+0-+"` into `DataTernary`.

1. How many trits does `to_fixed_ternary()` produce?
2. What does `to_ternary()` produce?
3. What extra information is necessary to reconstruct the original six positions exactly?

### Exercise 5: integer division or modular division?

In `Tryte<2>`, compare integer `1 / 2` with `1.checked_modular_div(2)`, using the appropriate constructors. Why can
division by three fail in the modular case while ordinary integer division by three succeeds?

### Exercise 6: the same encoding, a different logic

Let `a = Zero` and `b = Neg`. Evaluate `a.k3_imply(b)`.

1. Under K3, are both premises of modus ponens (`a` and `a.k3_imply(b)`) designated?
2. Under LP, are both designated? Is the conclusion `b` designated?
3. Replace `k3_imply` with `rm3_imply`. Does this assignment still refute modus ponens under designated values
   `{Zero, Pos}`?

### Exercise 7: use the correct HT negation

Let `p = Zero` under the HT interpretation.

1. In which of the two worlds is `p` true?
2. Compare `!p`, `p.ht_not()`, and `p.ht_not().ht_not()`.
3. Compare `p.ht_imply(p)` with `p.ht_not() | p`.
4. Explain why changing the negation function is not enough to express HT implication as material implication.

### Solutions

**1.** The results are U, T, and F. `necessary()` accepts only the T result; `possibly()` accepts U and T. In the last
row, false delivery makes the AND false even though stock is unknown.

**2.** De Morgan's law holds for these tables. Excluded middle is not always `Pos`:

```rust
use balanced_ternary::Digit::{Neg, Pos, Zero};

for a in [Neg, Zero, Pos] {
for b in [Neg, Zero, Pos] {
assert_eq ! ( ! (a & b), ( ! a) | ( ! b));
}
}
assert_eq!(Zero | !Zero, Zero);
```

**3.** The range is −13 through +13, and the modulus is 27:

```rust
use balanced_ternary::{Digit, Tryte};

type Word = Tryte<3>;
let one = Word::from_i64(1);

assert_eq!(Word::MIN.to_i64(), -13);
assert_eq!(Word::MAX.to_i64(), 13);
assert_eq!((Word::MAX + one).to_i64(), -13);
assert_eq!(Word::MAX.checked_add(one), None);
assert_eq!(Word::MAX.add_with_carry(one), (Word::MIN, Digit::Pos));
// -13 + 27 = 14, the unwrapped result.
```

**4.** Six input positions are padded to ten. Normalization yields `"+0-+"`. Retain the original length, six, and take
those six trailing positions from the fixed-width decoding:

```rust
use balanced_ternary::{DataTernary, Ternary};

let original = Ternary::parse("00+0-+"); let stored = DataTernary::from_ternary(original.clone()); let decoded = stored.to_fixed_ternary();

assert_eq!(decoded.log(), 10);
assert_eq!(stored.to_ternary().to_string(), "+0-+");
let start = decoded.log() - original.log(); assert_eq!(&decoded.to_digit_slice()[start..], original.to_digit_slice());
```

**5.** Integer division gives zero. Modular division gives −4 modulo nine. Three is a valid nonzero integer divisor but
is not invertible modulo nine, because it shares a factor with the modulus.

**6.** `a.k3_imply(b)` is `Zero`. In K3, neither premise is designated, so this assignment does not refute modus ponens.
In LP, both premises are designated and the conclusion is not: it is a counterexample. For RM3, `Zero.rm3_imply(Neg)` is
`Neg`, so the conditional premise is no longer designated and this counterexample disappears. Section 6 checks all nine
assignments for RM3.

**7.** `p` is true there but not here. `!p` gives `Zero`, HT negation gives `Neg`, and double HT negation gives `Pos`.
`p.ht_imply(p)` is `Pos`, whereas `p.ht_not() | p` is `Zero`. The conditional has its own table, so the
material-implication construction is insufficient even when using HT negation.

## 19. Operation guide and references

### A compact guide to the digit operations

This table groups the available digit-to-digit operations by practical purpose. It is an index for further exploration,
not a claim that all methods in a row belong to one formal logic. Trait-method spellings such as `bitand` require the
corresponding `core::ops` trait in scope if called by name; operator syntax needs no such import.

| Purpose                         | Operations                                                                                                                                                                                                                                                                                    | What to check                                            |
|---------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------|
| Arithmetic and signed shaping   | Unary `-`; binary `+`, `-`, `*`, `/`; `absolute_positive`, `absolute_negative`, `positive`, `negative`, `not_positive`, `not_negative`, `accept_anything`                                                                                                                                     | Wrapping versus saturation; zero divisors                |
| Truth functions and uncertainty | `!` (`not`), `&` (`bitand`), `\|` (`bitor`), `^` (`bitxor`); `k3_imply`, `k3_equiv`; `bi3_and`, `bi3_or`, `bi3_imply`; `l3_imply`, `l3_strong_and`, `l3_strong_or`; `rm3_imply`, `rm3_fusion`; `ht_imply`, `ht_not`; `para_imply`; `mc_and`, `mc_or`; `necessary`, `possibly`, `contingently` | Interpretation of the middle state and operand order     |
| Comparison and agreement        | `compare`, `equal`, `different`, `greater`, `greater_or_equal`, `lesser`, `lesser_or_equal`, `graded_diff`, `consensus`, `unknown_consensus`, `compatible`                                                                                                                                    | Numeric/state comparison versus logical equivalence      |
| Selection and control           | `coalesce`, `inhibit`, `enable`, `exclusive_merge`, `change`                                                                                                                                                                                                                                  | Which operand is the control; whether zero means absence |
| Cycles and movement             | `post`, `pre`, `is_successor`, `is_predecessor`, `duel`, `third`, `toward`                                                                                                                                                                                                                    | Linear ordering versus cyclic relationships              |

Two particularly easy names to misread are:

- `enable(value, control)` passes the value through for **either** nonzero control digit, including `Neg`. It is not
  “pass only when control is logically true.” In method syntax, use `value.enable(control)`.
- `change` returns the right operand when the two differ, and `Zero` when they agree. It is not the inverse of
  `toward`.

The `inc()` and `dec()` helpers on `Digit` are available with `ternary-string` and return a `Ternary`, because their
mathematical result can need more than one trit. They are distinct from the single-trit cyclic operations.

### Primary implementation references

The source links below are pinned to the 3.2.0 source revision used to validate this tutorial:

- [README and type overview](https://github.com/Trehinos/balanced-ternary/blob/6b50a1ab3b363e4fefa2854876b682f1a6e491f3/README.md)
- [Cargo features and minimum Rust version](https://github.com/Trehinos/balanced-ternary/blob/6b50a1ab3b363e4fefa2854876b682f1a6e491f3/Cargo.toml)
- [
  `Digit` operations](https://github.com/Trehinos/balanced-ternary/blob/6b50a1ab3b363e4fefa2854876b682f1a6e491f3/src/digit.rs)
- [Executable digit truth-table tests](https://github.com/Trehinos/balanced-ternary/blob/6b50a1ab3b363e4fefa2854876b682f1a6e491f3/src/tests/digit_operations.rs)
- [
  `Ternary`, parsing, representation, and conversions](https://github.com/Trehinos/balanced-ternary/blob/6b50a1ab3b363e4fefa2854876b682f1a6e491f3/src/lib.rs)
- [
  `Ternary` arithmetic](https://github.com/Trehinos/balanced-ternary/blob/6b50a1ab3b363e4fefa2854876b682f1a6e491f3/src/operations.rs)
- [
  `DigitOperate` contracts](https://github.com/Trehinos/balanced-ternary/blob/6b50a1ab3b363e4fefa2854876b682f1a6e491f3/src/concepts.rs)
- [`Tryte<N>`](https://github.com/Trehinos/balanced-ternary/blob/6b50a1ab3b363e4fefa2854876b682f1a6e491f3/src/tryte.rs)
- [`Ter40`, `TritsChunk`, and
  `DataTernary`](https://github.com/Trehinos/balanced-ternary/blob/6b50a1ab3b363e4fefa2854876b682f1a6e491f3/src/store.rs)

### Further reading

- [Hölldobler and Ramli, *Logic Programs under Three-Valued Łukasiewicz
  Semantics*](https://iccl.inf.tu-dresden.de/w/images/7/72/Hoelldobler%3A09%3Awv02.pdf), especially section 2, for a
  comparison of Kleene and Łukasiewicz tables. This is advanced reading; its logic-programming development is not
  required to use the crate.
- [Indrzejczak and Petrukhin, *A Uniform Formalisation of Three-Valued Logics in Bisequent
  Calculus*](https://link.springer.com/chapter/10.1007/978-3-031-38499-8_19), section 2, for matrices, designated
  values, weak/strong Kleene, sequential logic, and alternative connectives.
- [Pelletier, Sutcliffe, and Hazen, *Automated Reasoning for the Dialetheic Logic
  RM3*](https://cdn.aaai.org/ocs/15415/15415-68643-1-PB.pdf), for LP, RM3, their conditionals, and designated values.
- [Cabalar, Fariñas, Pearce, and Valverde, *A Free Logic for Stable Models with Partial Intensional
  Functions*](https://www.dc.fi.udc.es/~cabalar/freeht.pdf), section 2's introduction to HT worlds and persistence; its
  extensions to partial functions go beyond this tutorial.
- [Leyva, *The Systems of Post and Post Algebras: A Demonstration of an Obvious
  Fact*](https://digitalcommons.usf.edu/etd/7844/), for the algebraic setting of Post systems.
- [The Embedded Rust Book: a `no_std` environment](https://doc.rust-lang.org/embedded-book/intro/no-std.html).
- [Rust's `alloc` documentation](https://doc.rust-lang.org/alloc/).
- [The Cargo Book: features](https://doc.rust-lang.org/cargo/reference/features.html).

For local API documentation matching your dependency, run:

```sh
cargo doc --open
```

### Example validation

All 60 Rust blocks were checked using Rust 1.100.0 against the 3.2.0 development sources at
[`6b50a1a`](https://github.com/Trehinos/balanced-ternary/tree/6b50a1ab3b363e4fefa2854876b682f1a6e491f3)
(the snapshot's Cargo package version is `3.2.0-dev`). The dependency declarations above target the final **3.2.0**
release.

The 57 executable examples passed their assertions, and the three `no_std` library examples compiled. The truth tables
in section 6 were verified against the same source snapshot; examples also check defining identities and
inference-preservation properties exhaustively over the finite input domain. Each `no_std` example was checked with its
stated Cargo features on both the host and `thumbv7em-none-eabi`. These are library compilation checks, not a claim that
a complete firmware image was linked or run on hardware.
