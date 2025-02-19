//! A module defining the `BalancedTryte` structure and its associated functionality.
//!
//! The `BalancedTryte` struct represents a Copy type balanced ternary number with exactly 6 digits.
//! Each digit in a balanced ternary system can have one of three values: -1, 0, or 1.
//!
//! This module provides utilities to convert between `BalancedTryte` and various
//! representations such as `Ternary`, `u8`, and `u16`. It ensures that the `BalancedTryte`
//! always consists of exactly 6 ternary digits.
//!
//! A [Tryte] can holds value between `-364` and `+364`.

use crate::{
    Digit,
    Digit::{Neg, Pos, Zero},
    Ternary,
};
use alloc::string::ToString;
use core::fmt::{Display, Formatter};
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg as StdNeg, Not, Sub};

/// A struct representing a balanced ternary number with a fixed length of 6 digits.
///
/// The underlying representation of the number is an array of six `Digit` values.
/// This struct provides conversion methods to and from other formats.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub struct Tryte {
    /// The raw representation of the `BalancedTryte` as 6 ternary digits.
    raw: [Digit; 6],
}

impl Tryte {
    /// `364` or `++++++`
    pub const MAX: Self = Self { raw: [Pos; 6] };
    /// `-364` or `------`
    pub const MIN: Self = Self { raw: [Neg; 6] };
    /// `0` or `000000`
    pub const ZERO: Self = Self { raw: [Zero; 6] };

    /// Converts the `BalancedTryte` into its `Ternary` representation.
    ///
    /// # Returns
    ///
    /// A `Ternary` object representing the same balanced ternary number.
    pub fn to_ternary(&self) -> Ternary {
        Ternary::new(self.raw.to_vec())
    }

    /// Retrieves a slice containing the digits of the `BalancedTryte`.
    ///
    /// # Returns
    ///
    /// A slice referencing the six-digit array of the `BalancedTryte`.
    ///
    /// This function allows access to the raw representation of the
    /// balanced ternary number as a slice of `Digit` values.
    pub fn to_digit_slice(&self) -> &[Digit] {
        &self.raw
    }

    /// Creates a `BalancedTryte` from the given `Ternary`.
    ///
    /// # Arguments
    ///
    /// * `v` - A reference to a `Ternary` object.
    ///
    /// # Panics
    ///
    /// This function panics if the `Ternary` contains more than 6 digits.
    pub fn from_ternary(v: &Ternary) -> Self {
        if v.log() > 6 {
            panic!("Cannot convert a Ternary with more than 6 digits to a Tryte.");
        }
        let mut digits = [Zero; 6];
        for (i, d) in v.digits.iter().rev().enumerate() {
            digits[5 - i] = *d;
        }
        Self { raw: digits }
    }

    /// Converts the `BalancedTryte` into a signed 16-bit integer.
    ///
    /// # Returns
    ///
    /// A `i16` representing the decimal value of the `BalancedTryte`.
    pub fn to_i16(&self) -> i16 {
        self.to_ternary().to_dec() as i16
    }

    /// Creates a `BalancedTryte` from a signed 8-bit integer.
    ///
    /// # Arguments
    ///
    /// * `v` - An unsigned 8-bit integer.
    ///
    /// # Returns
    ///
    /// A `BalancedTryte` representing the equivalent ternary number.
    pub fn from_i8(v: i8) -> Self {
        Self::from_ternary(&Ternary::from_dec(v as i64))
    }

    /// Creates a `BalancedTryte` from a signed 16-bit integer.
    ///
    /// # Arguments
    ///
    /// * `v` - A signed 16-bit integer.
    ///
    /// # Returns
    ///
    /// A `BalancedTryte` representing the equivalent ternary number.
    pub fn from_i16(v: i16) -> Self {
        Self::from_ternary(&Ternary::from_dec(v as i64))
    }

    /// Retrieves the digit at the specified index in the `BalancedTryte`.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the digit to retrieve (0-based, right-to-left).
    ///
    /// # Returns
    ///
    /// The `Digit` at the specified index.
    ///
    /// # Panics
    ///
    /// This function panics if the index is greater than 5.
    pub fn digit(&self, index: usize) -> Digit {
        if index > 5 {
            panic!(
                "Cannot access a digit at index {}. Tryte has only 6 digits.",
                index
            );
        }
        *self.raw.iter().rev().nth(index).unwrap()
    }
}

impl Display for Tryte {
    /// Formats the `BalancedTryte` for display.
    ///
    /// The `BalancedTryte` is displayed in its balanced ternary representation
    /// as a 6-character string.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:06}", self.to_ternary().to_string())
    }
}

impl StdNeg for Tryte {
    type Output = Tryte;
    fn neg(self) -> Self::Output {
        Self::from_ternary(&-&self.to_ternary())
    }
}

impl Add for Tryte {
    type Output = Tryte;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_ternary(&(&self.to_ternary() + &rhs.to_ternary()))
    }
}

impl Sub for Tryte {
    type Output = Tryte;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_ternary(&(&self.to_ternary() - &rhs.to_ternary()))
    }
}

impl Mul for Tryte {
    type Output = Tryte;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_ternary(&(&self.to_ternary() * &rhs.to_ternary()))
    }
}

impl Div for Tryte {
    type Output = Tryte;

    fn div(self, rhs: Self) -> Self::Output {
        Self::from_ternary(&(&self.to_ternary() / &rhs.to_ternary()))
    }
}

impl BitAnd for Tryte {
    type Output = Tryte;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self::from_ternary(&(&self.to_ternary() & &rhs.to_ternary()))
    }
}

impl BitOr for Tryte {
    type Output = Tryte;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::from_ternary(&(&self.to_ternary() | &rhs.to_ternary()))
    }
}

impl BitXor for Tryte {
    type Output = Tryte;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::from_ternary(&(&self.to_ternary() ^ &rhs.to_ternary()))
    }
}

impl Not for Tryte {
    type Output = Tryte;
    fn not(self) -> Self::Output {
        -self
    }
}

impl From<Ternary> for Tryte {
    fn from(value: Ternary) -> Self {
        Tryte::from_ternary(&value)
    }
}

impl From<Tryte> for Ternary {
    fn from(value: Tryte) -> Self {
        value.to_ternary()
    }
}

#[cfg(test)]
#[test]
pub fn test_tryte() {
    let tryte = Tryte::from_i16(255);
    assert_eq!(tryte.to_i16(), 255);
    assert_eq!(tryte.to_string(), "+00++0");

    let tryte = Tryte::from_i8(16);
    assert_eq!(tryte.to_i16(), 16);
    assert_eq!(tryte.to_string(), "00+--+");

    assert_eq!(Tryte::MAX.to_string(), "++++++");
    assert_eq!(Tryte::MAX.to_i16(), 364);
    assert_eq!(Tryte::MIN.to_string(), "------");
    assert_eq!(Tryte::MIN.to_i16(), -364);
    assert_eq!(Tryte::ZERO.to_string(), "000000");
    assert_eq!(Tryte::ZERO.to_i16(), 0);
}
