//! This module provides implementations for arithmetic operations on `Digit` and `Ternary` types
//! such as addition, subtraction, multiplication, and division.
//!
//! These operations adhere to the rules of balanced ternary arithmetic.
//!
//! # Examples
//!
//! Using `Digit` arithmetic:
//!
//! ```rust
//! use balanced_ternary::Digit;
//!
//! let a = Digit::Neg;
//! let b = Digit::Zero;
//! let sum = a + b;
//! assert_eq!(sum.to_string(), "-");
//! let product = a * b; // Results in Digit::Neg
//! assert_eq!(product.to_char(), '0')
//! ```
//!
//! Using `Ternary` arithmetic:
//!
//! ```rust
//! use balanced_ternary::Ternary;
//!
//! let repr9 = Ternary::parse("+00"); // Represents decimal 9 in balanced ternary
//! let repr4 = Ternary::parse("++");  // Represents decimal 4 in balanced ternary
//! let sum = &repr9 + &repr4;         // Results in Ternary::parse("+++"), decimal 13
//! assert_eq!(sum.to_dec(), 13);
//! let difference = &sum - &repr4;   // Results in Ternary::parse("+00"), decimal 9
//! assert_eq!(difference.to_dec(), 9);
//! ```
//!
//! # Implementations
//!
//! The following arithmetic operations are implemented for `Digit` and `Ternary` types:
//!
//! ## `Digit` type
//!
//! - `Neg` and `Not` for `Digit`: Negates the digit value, adhering to balanced ternary rules.
//! - `Add<Digit>` for `Digit`: Adds two `Digit` values and returns a `Ternary`.
//! - `Sub<Digit>` for `Digit`: Subtracts one `Digit` from another and returns a `Ternary`.
//! - `Mul<Digit>` for `Digit`: Multiplies two `Digit` values and returns a `Digit`.
//! - `Div<Digit>` for `Digit`: Divides one `Digit` by another and returns a `Digit`. Division by zero panics.
//!
//! ### Logical Operations for `Digit`
//!
//! The `Digit` type supports bitwise logical operations, which are implemented according to logical rules applicable to balanced ternary digits.
//!
//! #### `BitAnd` for `Digit`
//!
//! Performs a bitwise AND operation between two `Digit` values.
//!
//! - `Digit::Neg & other` → `Digit::Neg`
//! - `Digit::Zero & Digit::Neg` → `Digit::Neg`
//! - `Digit::Zero & other` → `Digit::Zero`
//! - `Digit::Pos & other` → `other`
//!
//! #### `BitOr` for `Digit`
//!
//! Performs a bitwise OR operation between two `Digit` values.
//!
//! - `Digit::Neg | other` → `other`
//! - `Digit::Zero | Digit::Pos` → `Digit::Pos`
//! - `Digit::Zero | other` → `Digit::Zero`
//! - `Digit::Pos | other` → `Digit::Pos`
//!
//! #### `BitXor` for `Digit`
//!
//! Performs a bitwise XOR operation between two `Digit` values.
//!
//! - `Digit::Neg ^ other` → `other`
//! - `Digit::Zero ^ other` → `Digit::Zero`
//! - `Digit::Pos ^ other` → `-other`
//!
//! ## `Ternary` type
//!
//! - `Neg` and `Not` for `&Ternary`: Negates the `Ternary` by negating each digit in its balanced ternary representation.
//! - `Add<&Ternary>` for `&Ternary`: Adds two `Ternary` values and returns a new `Ternary`. Panics on overflow.
//! - `Sub<&Ternary>` for `&Ternary`: Subtracts one `Ternary` from another and returns a new `Ternary`. Panics on overflow.
//! - `Mul<&Ternary>` for `&Ternary`: Multiplies two `Ternary` values and returns a new `Ternary`. Panics on overflow.
//! - `Div<&Ternary>` for `&Ternary`: Divides one `Ternary` by another and returns a new `Ternary`. Panics on overflow or division by zero.

use crate::{Digit, Ternary};
use alloc::vec;
use alloc::vec::Vec;
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Sub};

impl Neg for Digit {
    type Output = Self;

    /// Returns the negation of the `Digit`.
    ///
    /// - `Digit::Neg` becomes `Digit::Pos`
    /// - `Digit::Pos` becomes `Digit::Neg`
    /// - `Digit::Zero` remains `Digit::Zero`
    fn neg(self) -> Self::Output {
        match self {
            Digit::Neg => Digit::Pos,
            Digit::Zero => Digit::Zero,
            Digit::Pos => Digit::Neg,
        }
    }
}

impl Not for Digit {
    type Output = Self;
    fn not(self) -> Self::Output {
        -self
    }
}

impl Add<Digit> for Digit {
    type Output = Ternary;
    fn add(self, other: Digit) -> Self::Output {
        match self {
            Digit::Neg => match other {
                Digit::Neg => Ternary::parse("-+"),
                Digit::Zero => Ternary::parse("-"),
                Digit::Pos => Ternary::parse("0"),
            },
            Digit::Zero => Ternary::new(vec![other]),
            Digit::Pos => match other {
                Digit::Neg => Ternary::parse("0"),
                Digit::Zero => Ternary::parse("+"),
                Digit::Pos => Ternary::parse("+-"),
            },
        }
    }
}

impl Sub<Digit> for Digit {
    type Output = Ternary;
    fn sub(self, other: Digit) -> Self::Output {
        match self {
            Digit::Neg => match other {
                Digit::Neg => Ternary::parse("0"),
                Digit::Zero => Ternary::parse("-"),
                Digit::Pos => Ternary::parse("-+"),
            },
            Digit::Zero => Ternary::new(vec![-other]),
            Digit::Pos => match other {
                Digit::Neg => Ternary::parse("+-"),
                Digit::Zero => Ternary::parse("+"),
                Digit::Pos => Ternary::parse("0"),
            },
        }
    }
}

impl Mul<Digit> for Digit {
    type Output = Digit;

    fn mul(self, other: Digit) -> Self::Output {
        match self {
            Digit::Neg => -other,
            Digit::Zero => Digit::Zero,
            Digit::Pos => other,
        }
    }
}

impl Div<Digit> for Digit {
    type Output = Digit;

    fn div(self, other: Digit) -> Self::Output {
        match self {
            Digit::Neg => match other {
                Digit::Neg => Digit::Pos,
                Digit::Zero => panic!("Cannot divide by zero."),
                Digit::Pos => Digit::Neg,
            },
            Digit::Zero => match other {
                Digit::Neg => Digit::Zero,
                Digit::Zero => panic!("Cannot divide by zero."),
                Digit::Pos => Digit::Zero,
            },
            Digit::Pos => match other {
                Digit::Neg => Digit::Neg,
                Digit::Zero => panic!("Cannot divide by zero."),
                Digit::Pos => Digit::Pos,
            },
        }
    }
}

impl BitAnd for Digit {
    type Output = Self;
    fn bitand(self, other: Self) -> Self::Output {
        match self {
            Digit::Neg => Digit::Neg,
            Digit::Zero => match other {
                Digit::Neg => Digit::Neg,
                _ => Digit::Zero,
            },
            Digit::Pos => other,
        }
    }
}

impl BitOr for Digit {
    type Output = Self;
    fn bitor(self, other: Self) -> Self::Output {
        match self {
            Digit::Neg => other,
            Digit::Zero => match other {
                Digit::Pos => Digit::Pos,
                _ => Digit::Zero,
            },
            Digit::Pos => Digit::Pos,
        }
    }
}

impl BitXor for Digit {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match self {
            Digit::Neg => rhs,
            Digit::Zero => Digit::Zero,
            Digit::Pos => -rhs,
        }
    }
}

impl Neg for &Ternary {
    type Output = Ternary;

    /// Returns the negation of the current `Ternary` object.
    ///
    /// Negates each digit in the number.
    fn neg(self) -> Self::Output {
        let mut repr = Ternary::new(vec![]);
        for digit in self.digits.iter() {
            repr.digits.push(-*digit);
        }
        repr
    }
}

impl Add<&Ternary> for &Ternary {
    type Output = Ternary;

    fn add(self, rhs: &Ternary) -> Self::Output {
        Ternary::from_dec(
            self.to_dec()
                .checked_add(rhs.to_dec())
                .expect("Overflow in addition."),
        )
    }
}

impl Sub<&Ternary> for &Ternary {
    type Output = Ternary;

    fn sub(self, rhs: &Ternary) -> Self::Output {
        Ternary::from_dec(
            self.to_dec()
                .checked_sub(rhs.to_dec())
                .expect("Overflow in subtraction."),
        )
    }
}

impl Mul<&Ternary> for &Ternary {
    type Output = Ternary;

    fn mul(self, rhs: &Ternary) -> Self::Output {
        Ternary::from_dec(
            self.to_dec()
                .checked_mul(rhs.to_dec())
                .expect("Overflow in multiplication."),
        )
    }
}

impl Div<&Ternary> for &Ternary {
    type Output = Ternary;

    fn div(self, rhs: &Ternary) -> Self::Output {
        Ternary::from_dec(
            self.to_dec()
                .checked_div(rhs.to_dec())
                .expect("Overflow in division or division by zero."),
        )
    }
}

impl BitAnd<&Ternary> for &Ternary {
    type Output = Ternary;

    fn bitand(self, rhs: &Ternary) -> Self::Output {
        if self.log() < rhs.log() {
            return rhs & self;
        }
        let mut digits = Vec::new();
        for (i, d) in self.digits.iter().rev().enumerate() {
            let other = rhs.get_digit(i).unwrap_or(&Digit::Zero);
            digits.push(*d & *other);
        }
        digits.reverse();
        Ternary::new(digits)
    }
}

impl BitOr<&Ternary> for &Ternary {
    type Output = Ternary;

    fn bitor(self, rhs: &Ternary) -> Self::Output {
        if self.log() < rhs.log() {
            return rhs | self;
        }
        let mut digits = Vec::new();
        for (i, d) in self.digits.iter().rev().enumerate() {
            let other = rhs.get_digit(i).unwrap_or(&Digit::Zero);
            digits.push(*d | *other);
        }
        digits.reverse();
        Ternary::new(digits)
    }
}

impl BitXor<&Ternary> for &Ternary {
    type Output = Ternary;

    fn bitxor(self, rhs: &Ternary) -> Self::Output {
        if self.log() < rhs.log() {
            return rhs ^ self;
        }
        let mut digits = Vec::new();
        for (i, d) in self.digits.iter().rev().enumerate() {
            let other = rhs.get_digit(i).unwrap_or(&Digit::Zero);
            digits.push(*d ^ *other);
        }
        digits.reverse();
        Ternary::new(digits)
    }
}

impl Not for &Ternary {
    type Output = Ternary;
    fn not(self) -> Self::Output {
        -self
    }
}

#[cfg(test)]
#[test]
fn test_ternary_ops() {
    use alloc::string::ToString;

    let repr9 = Ternary::parse("+00");
    let repr4 = Ternary::parse("++");
    let repr13 = &repr9 + &repr4;
    let repr17 = &repr13 + &repr4;
    let repr34 = &repr17 + &repr17;

    assert_eq!(repr13.to_string(), "+++");
    assert_eq!(repr17.to_string(), "+-0-");
    assert_eq!(repr34.to_string(), "++-+");

    let repr30 = &repr34 - &repr4;
    assert_eq!(repr30.to_dec(), 30);
    assert_eq!(repr30.to_string(), "+0+0");

    let repr120 = &repr30 * &repr4;
    assert_eq!(repr120.to_dec(), 120);
    assert_eq!(repr120.to_string(), "++++0");

    let repr_neg120 = -&repr120;
    assert_eq!(repr_neg120.to_dec(), -120);
    assert_eq!(repr_neg120.to_string(), "----0");

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
