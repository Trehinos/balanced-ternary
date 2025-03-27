//! This module provides implementations for arithmetic operations on the `Ternary` type
//! such as addition, subtraction, multiplication, and division.
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
//! The following arithmetic operations are implemented for the `Ternary` :
//!
//! ## `Ternary` type
//!
//! - `Neg` and `Not` for `&Ternary`: Negates the `Ternary` by negating each digit in its balanced ternary representation.
//! - `Add<&Ternary>` for `&Ternary`: Adds two `Ternary` values and returns a new `Ternary`. Panics on overflow.
//! - `Sub<&Ternary>` for `&Ternary`: Subtracts one `Ternary` from another and returns a new `Ternary`. Panics on overflow.
//! - `Mul<&Ternary>` for `&Ternary`: Multiplies two `Ternary` values and returns a new `Ternary`. Panics on overflow.
//! - `Div<&Ternary>` for `&Ternary`: Divides one `Ternary` by another and returns a new `Ternary`. Panics on overflow or division by zero.
//! - `BitAnd<&Ternary>` for `&Ternary`: Computes the bitwise AND operation on two `Ternary` operands.
//! - `BitOr<&Ternary>` for `&Ternary`: Computes the bitwise OR operation on two `Ternary` operands.
//! - `BitXor<&Ternary>` for `&Ternary`: Computes the bitwise XOR operation on two `Ternary` operands.

use crate::concepts::DigitOperate;
use crate::{Digit, Ternary};
use alloc::vec;
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Sub};

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

impl Add<Digit> for &Ternary {
    type Output = Ternary;

    fn add(self, rhs: Digit) -> Self::Output {
        Ternary::from_dec(
            self.to_dec()
                .checked_add(rhs.to_i8() as i64)
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

impl Sub<Digit> for &Ternary {
    type Output = Ternary;
    fn sub(self, rhs: Digit) -> Self::Output {
        Ternary::from_dec(
            self.to_dec()
                .checked_sub(rhs.to_i8() as i64)
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
        self.each_zip(Digit::bitand, rhs.clone())
    }
}

impl BitOr<&Ternary> for &Ternary {
    type Output = Ternary;

    fn bitor(self, rhs: &Ternary) -> Self::Output {
        self.each_zip(Digit::bitor, rhs.clone())
    }
}

impl BitXor<&Ternary> for &Ternary {
    type Output = Ternary;

    fn bitxor(self, rhs: &Ternary) -> Self::Output {
        self.each_zip(Digit::bitxor, rhs.clone())
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
