[![Rust](https://github.com/Trehinos/balanced-ternary/actions/workflows/rust.yml/badge.svg)](https://github.com/Trehinos/balanced-ternary/actions/workflows/rust.yml)

# Balanced Ternary

**Balanced Ternary** is a Rust library for manipulating **[balanced ternary](https://en.wikipedia.org/wiki/Balanced_ternary)** numbers, a numeral system with digits `-1`,
`0`, and `+1`. 

This system is useful in areas like computer science and mathematics due to its symmetry and unique arithmetic properties.

## Features

- **Number Conversions:** Convert between decimal and balanced ternary representations.
- **Arithmetic Operations:** Support for addition, subtraction, multiplication, and division.
- **Custom Representation:** Parse and display numbers using `+`, `0`, and `-` symbols.
- **No Standard Library:** Suitable for `#![no_std]` environments.

## Examples

### Convert between decimal and balanced ternary

```rust
use balanced_ternary::*;

fn test() {
    let ternary = Ternary::from_dec(5);
    assert_eq!(ternary.to_string(), "+--");
    assert_eq!(ternary.to_dec(), 5);
}
```

### Perform arithmetic operations

```rust
use balanced_ternary::*;

fn test() {
    let a = Ternary::from_dec(9);
    let b = Ternary::from_dec(4);
    let sum = &a + &b;
    assert_eq!(ternary.to_string(), "+++");
    assert_eq!(sum.to_dec(), 13);
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
balanced-ternary = "0.1.*"
```

## License
Copyright (c) 2025 Sébastien GELDREICH  
`Balanced Ternary` is licensed under the [MIT License](LICENSE).
