[![Rust](https://github.com/Trehinos/balanced-ternary/actions/workflows/rust.yml/badge.svg)](https://github.com/Trehinos/balanced-ternary/actions/workflows/rust.yml)

# Balanced Ternary

**Balanced Ternary** is a Rust library for manipulating
**[balanced ternary](https://en.wikipedia.org/wiki/Balanced_ternary)**
numbers, a numeral system with digits `-1`, `0`, and `+1`.

This system is particularly useful in specialized computing applications such as reversible computing, digital signal processing, and three-valued logic modeling.

## Three-valued logic
The library supports numerous three-valued logic operations, each of them having its own specificities:
- K3 (Kleene logic) – A three-valued logic that introduces an "unknown" (0) state,  
  useful for dealing with partial information.
- BI3 (Bochvar logic) – A logic designed to handle "nonsense" or meaningless statements, 
  where 0 represents an invalid or undefined value.
- L3 (Łukasiewicz logic) – A non-classical logic allowing for degrees of truth, 
  often used in fuzzy logic and multi-valued reasoning.
- RM3 (Routley-Meyer paraconsistent logic) – A logic that tolerates contradictions without collapsing into triviality,
  useful in paraconsistent reasoning.
- HT (Heyting logic-inspired ternary system) – A variant of intermediate logic, 
  often related to intuitionistic logic and constructive reasoning.
- Paraconsistent logic – A logic framework that avoids the principle of explosion,
  allowing systems to work with contradictory information.
- Post logic – A logical system that extends classical logic with additional operators to handle uncertainty in a structured way.

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

### Digits operations

| Unary operations                                   | -    | 0 | +    |
|----------------------------------------------------|------|---|------|
| possibly (**L3**)                                  | -    | + | +    |
| necessary (**L3**)                                 | -    | - | +    |
| contingently (**L3**)                              | -    | + | -    |
| ht_not (**HT**)                                    | +    | - | -    |
| post (**post logic**)<br>eqv. `self.pre().pre()`   | 0    | + | -    |
| pre (inverse of post)<br>eqv. `self.post().post()` | +    | - | 0    |
| `!` (not) / `-` (neg) (**K3**)                     | +    | 0 | -    |
| absolute_positive                                  | +    | 0 | +    |
| positive                                           | 0    | 0 | +    |
| not_negative                                       | 0    | + | +    |
| not_positive                                       | -    | - | 0    |
| negative                                           | -    | 0 | 0    |
| absolute_negative                                  | -    | 0 | -    |
| inc                                                | 0    | + | `+-` |
| dec                                                | `-+` | - | 0    |

| Binary operations               | -<br>- | -<br>0 | -<br>+ | 0<br>- | 0<br>0 | 0<br>+ | +<br>- | +<br>0 | +<br>+ |
|---------------------------------|--------|--------|--------|--------|--------|--------|--------|--------|--------|
| `+` (add)                       | `-+`   | -      | 0      | -      | 0      | +      | 0      | +      | `+-`   |
| `-` (sub)                       | 0      | -      | `-+`   | +      | 0      | -      | `+-`   | +      | 0      |
| `/` (div)                       | +      |        | -      | 0      |        | 0      | -      |        | +      |
| `*` (mul)                       | +      | 0      | -      | 0      | 0      | 0      | -      | 0      | +      |
| `&` (bitand) (**K3**)           | -      | -      | -      | -      | 0      | 0      | -      | 0      | +      |
| bi3_and (**BI3**)               | -      | 0      | -      | 0      | 0      | 0      | -      | 0      | +      |
| `\|` (bitor) (**K3**)           | -      | 0      | +      | 0      | 0      | +      | +      | +      | +      |
| bi3_or (**BI3**)                | -      | 0      | +      | 0      | 0      | 0      | +      | 0      | +      |
| `^` (bitxor) (**K3**)           | -      | 0      | +      | 0      | 0      | 0      | +      | 0      | -      |
| k3_equiv (**K3**)               | +      | 0      | -      | 0      | 0      | 0      | -      | 0      | +      |
| k3_imply (**K3**)               | +      | +      | +      | 0      | 0      | +      | -      | 0      | +      |
| bi3_imply (**BI3**)             | +      | 0      | +      | 0      | 0      | 0      | -      | 0      | +      |
| l3_imply (**L3**)               | +      | +      | +      | 0      | +      | +      | -      | 0      | +      |
| rm3_imply (**RM3**)             | +      | +      | +      | -      | 0      | +      | -      | -      | +      |
| para_imply (**paraconsistent**) | +      | +      | +      | -      | 0      | +      | -      | 0      | +      |
| ht_imply (**HT**)               | +      | +      | +      | -      | +      | +      | -      | 0      | +      |

> `add`, `sub`, `inc` and `dec` return `Ternary` while other operations return `Digit`.

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
balanced-ternary = "1.0.2"
```

## Documentation
The complete API documentation can be found [on crate.io](https://docs.rs/balanced-ternary).
There you can find descriptions and examples of available types and methods.

## License

Copyright (c) 2025 Sébastien GELDREICH  
`Balanced Ternary` is licensed under the [MIT License](LICENSE).
