[![Rust](https://github.com/Trehinos/balanced-ternary/actions/workflows/rust.yml/badge.svg)](https://github.com/Trehinos/balanced-ternary/actions/workflows/rust.yml)

# Balanced Ternary

**Balanced Ternary** is a Rust library for manipulating
**[balanced ternary](https://en.wikipedia.org/wiki/Balanced_ternary)**
numbers, a numeral system with digits `-1`, `0`, and `+1`.

This system is particularly useful in specialized computing applications such as reversible computing, digital signal
processing, and three-valued logic modeling.

## Features

- **No Standard Library:** Suitable for `#![no_std]` environments.
- **Number Conversions:** Convert between decimal and balanced ternary representations.
- **Arithmetic Operations:** Support for addition, subtraction, multiplication, and division.
- **[Three-value Logic Operations](https://en.wikipedia.org/wiki/Three-valued_logic):**
    - Support for bitwise and, or, xor, and not (in Kleene algebra (K3)).
    - **Advanced logic**: Implementation of
      [K3](https://en.wikipedia.org/wiki/De_Morgan_algebra#Kleene_algebra),
      [BI3](https://en.wikipedia.org/wiki/Many-valued_logic#Bochvar's_internal_three-valued_logic),
      [L3](https://en.wikipedia.org/wiki/%C5%81ukasiewicz_logic),
      [RM3](https://en.wikipedia.org/wiki/Paraconsistent_logic#An_ideal_three-valued_paraconsistent_logic),
      [paraconsistent-logic](https://en.wikipedia.org/wiki/Paraconsistent_logic#An_ideal_three-valued_paraconsistent_logic)
      and [HT](https://en.wikipedia.org/wiki/Intermediate_logic) imply operation,
      and some more HT, BI3, L3 and post-logic operations.
- **Custom Representation:** Parse and display numbers using `+`, `0`, and `-` symbols by default, or custom ones.
- Provides the types:
    - `Digit` (`Neg`, `Zero` or `Pos`),
    - `Ternary` (heap allocated variable-length balanced-ternary number),
    - `Tryte<S>` (S characters long copy-type ternary number).

## Three-valued logic

The library supports numerous three-valued logic operations, each of them having its own specificities:

- **K3** (Kleene logic)  
  A three-valued logic that introduces an "unknown" (0) state, useful for dealing with partial information.
- **BI3** (Bochvar logic)  
  A logic designed to handle "nonsense" or meaningless statements, where 0 represents an invalid or undefined value.
- **L3** (Łukasiewicz logic)  
  A non-classical logic allowing for degrees of truth, often used in fuzzy logic and multi-valued reasoning.
- **RM3** (Routley-Meyer paraconsistent logic)  
  A logic that tolerates contradictions without collapsing into triviality, useful in paraconsistent reasoning.
- **HT** (Heyting logic-inspired ternary system)  
  A variant of intermediate logic, often related to intuitionistic logic and constructive reasoning.
- **Paraconsistent logic**  
  A logic framework that avoids the principle of explosion, allowing systems to work with contradictory information.
- **Post logic**  
  A logical system that extends classical logic with additional operators to handle uncertainty in a structured way.

### Digits operations

### Digits operations

The library provides a variety of operations that can be performed on individual balanced ternary digits. These
operations include logical operations, arithmetic operations, and utility functions that are useful for manipulating
ternary numbers at the digit level. Below are some examples of how these operations can be used:

```rust
fn test_ternary_eq(a: Ternary, b: &str) {
    let repr = Ternary::parse(b);
    assert_eq!(a.to_string(), repr.to_string());
}
fn test_binary_op(a: &Ternary, f: impl Fn(Digit, Digit) -> Digit, b: &Ternary, c: &str) {
    test_ternary_eq(a.each_zip(f, b.clone()), c);
}
fn test_operations() {
    use core::ops::{BitAnd, BitOr, BitXor, Mul, Not};

    let short = Ternary::parse("-0+");
    let long = Ternary::parse("---000+++");
    let other = Ternary::parse("-0+-0+-0+");

    // K3
    test_ternary_eq(short.each(Digit::not), "+0-");
    test_binary_op(&long, Digit::bitand, &other, "----00-0+");
    test_binary_op(&long, Digit::bitor, &other, "-0+00++++");
    test_binary_op(&long, Digit::bitxor, &other, "-0+000+0-");
    test_binary_op(&long, Digit::k3_equiv, &other, "+0-000-0+");
    test_binary_op(&long, Digit::k3_imply, &other, "+++00+-0+");

    // HT
    test_ternary_eq(short.each(Digit::ht_not), "+--");
    test_binary_op(&long, Digit::ht_imply, &other, "+++-++-0+");

    // BI3
    test_binary_op(&long, Digit::bi3_and, &other, "-0-000-0+");
    test_binary_op(&long, Digit::bi3_or, &other, "-0+000+0+");
    test_binary_op(&long, Digit::bi3_imply, &other, "+0+000-0+");

    // L3
    test_ternary_eq(short.each(Digit::possibly), "-++");
    test_ternary_eq(short.each(Digit::necessary), "--+");
    test_ternary_eq(short.each(Digit::contingently), "-+-");
    test_binary_op(&long, Digit::l3_imply, &other, "+++0++-0+");

    // PARA / RM3
    test_binary_op(&long, Digit::rm3_imply, &other, "+++-0+--+");
    test_binary_op(&long, Digit::para_imply, &other, "+++-0+-0+");

    // Other operations
    test_ternary_eq(short.each(Digit::post), "0+-");
    test_ternary_eq(short.each(Digit::pre), "+-0");
    test_ternary_eq(short.each(Digit::absolute_positive), "+0+");
    test_ternary_eq(short.each(Digit::positive), "00+");
    test_ternary_eq(short.each(Digit::not_negative), "0++");
    test_ternary_eq(short.each(Digit::not_positive), "--0");
    test_ternary_eq(short.each(Digit::negative), "-00");
    test_ternary_eq(short.each(Digit::absolute_negative), "-0-");

    test_binary_op(&long, Digit::mul, &other, "+0-000-0+");
}

```

## Examples

### Convert between decimal and balanced ternary

```rust
use balanced_ternary::*;

fn test() {
    let ternary = Ternary::from_dec(5);
    assert_eq!(ternary.to_string(), "+--");

    let ternary = Ternary::parse("+--");
    assert_eq!(ternary.to_dec(), 5);
}
```

### Perform arithmetic or logic operations

```rust
use balanced_ternary::*;

fn test() {
    let a = Ternary::from_dec(9);
    let b = Ternary::from_dec(4);
    let sum = &a + &b;
    assert_eq!(sum.to_string(), "+++");
    assert_eq!(sum.to_dec(), 13);

    let bitwise = &Ternary::parse("++00") & &Ternary::parse("0000");
    assert_eq!(bitwise.to_string(), "0000");

    let bitwise = &Ternary::parse("++00") & &Ternary::parse("0+00");
    assert_eq!(bitwise.to_string(), "0+00");

    let bitwise = &Ternary::parse("+000") | &Ternary::parse("000-");
    assert_eq!(bitwise.to_string(), "+000");


    let bitwise = &Ternary::parse("+000") & &Ternary::parse("000-");
    assert_eq!(bitwise.to_string(), "000-");

    let bitwise = &Ternary::parse("+000") | &Ternary::parse("000+");
    assert_eq!(bitwise.to_string(), "+00+");
}
```

### Handle negative numbers

```rust
use balanced_ternary::*;

fn test() {
    let negative = Ternary::from_dec(-5);
    assert_eq!(negative.to_string(), "-++");
}
```

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
balanced-ternary = "^1"
```

## Documentation

The complete API documentation can be found [on docs.rs](https://docs.rs/balanced-ternary).
There you can find descriptions and examples of available types and methods.

## License

Copyright (c) 2025 Sébastien GELDREICH  
`Balanced Ternary` is licensed under the [MIT License](LICENSE).
