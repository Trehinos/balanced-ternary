//! A module defining the `Tryte` structure and its associated functionality.
//!
//! The `Tryte` struct represents a Copy type balanced ternary number with exactly 6 digits.
//! Each digit in a balanced ternary system can have one of three values: -1, 0, or 1.
//!
//! This module provides utilities to convert between `Tryte` and various
//! representations such as `Ternary`, `u8`, and `u16`. It ensures that the `Tryte`
//! always consists of exactly 6 ternary digits.
//!
//! A [Tryte] can holds value between `-364` and `+364`.
//!
//! The strucutre [Tryte36] is also defined to holds big integer values.

use crate::{
    Digit,
    Digit::{Neg, Pos, Zero},
    Ternary,
};
use alloc::string::{String, ToString};
use core::fmt::{Display, Formatter};
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg as StdNeg, Not, Sub};

/// A struct representing a balanced ternary number with a fixed length of 6 digits.
///
/// The underlying representation of the number is an array of six `Digit` values.
/// This struct provides conversion methods to and from other formats.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub struct Tryte {
    /// The raw representation of the `Tryte` as 6 ternary digits.
    raw: [Digit; 6],
}

impl Tryte {
    /// `364` or `++++++`
    pub const MAX: Self = Self::new([Pos; 6]);
    /// `-364` or `------`
    pub const MIN: Self = Self::new([Neg; 6]);
    /// `0` or `000000`
    pub const ZERO: Self = Self::new([Zero; 6]);

    /// Creates a new `Tryte` instance from a given array of `Digit`s.
    ///
    /// # Arguments
    ///
    /// * `raw` - An array of exactly 6 `Digit` values representing the balanced ternary digits.
    ///
    /// # Returns
    ///
    /// A new `Tryte` instance with the specified balanced ternary digits.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_ternary::{Tryte, Digit::{Pos, Zero, Neg}};
    ///
    /// let digits = [Pos, Zero, Neg, Zero, Pos, Neg];
    /// let tryte = Tryte::new(digits);
    /// assert_eq!(tryte.to_digit_slice(), &digits);
    /// ```
    pub const fn new(digits: [Digit; 6]) -> Self {
        Self { raw: digits }
    }

    /// Converts the `Tryte` into its `Ternary` representation.
    ///
    /// # Returns
    ///
    /// A `Ternary` object representing the same balanced ternary number.
    pub fn to_ternary(&self) -> Ternary {
        Ternary::new(self.raw.to_vec())
    }

    /// Retrieves a slice containing the digits of the `Tryte`.
    ///
    /// # Returns
    ///
    /// A slice referencing the six-digit array of the `Tryte`.
    ///
    /// This function allows access to the raw representation of the
    /// balanced ternary number as a slice of `Digit` values.
    pub const fn to_digit_slice(&self) -> &[Digit] {
        &self.raw
    }

    /// Creates a `Tryte` from the given `Ternary`.
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

    /// Converts the `Tryte` into a signed 16-bit integer.
    ///
    /// # Returns
    ///
    /// A `i16` representing the decimal value of the `Tryte`.
    pub fn to_i16(&self) -> i16 {
        self.to_ternary().to_dec() as i16
    }

    /// Creates a `Tryte` from a signed 8-bit integer.
    ///
    /// # Arguments
    ///
    /// * `v` - An unsigned 8-bit integer.
    ///
    /// # Returns
    ///
    /// A `Tryte` representing the equivalent ternary number.
    pub fn from_i8(v: i8) -> Self {
        Self::from_ternary(&Ternary::from_dec(v as i64))
    }

    /// Creates a `Tryte` from a signed 16-bit integer.
    ///
    /// # Arguments
    ///
    /// * `v` - A signed 16-bit integer.
    ///
    /// # Returns
    ///
    /// A `Tryte` representing the equivalent ternary number.
    pub fn from_i16(v: i16) -> Self {
        Self::from_ternary(&Ternary::from_dec(v as i64))
    }

    /// Retrieves the digit at the specified index in the `Tryte`.
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

    /// See [Ternary::each].
    pub fn each(&self, f: impl Fn(Digit) -> Digit) -> Self {
        Self::from_ternary(&self.to_ternary().each(f))
    }

    /// See [Ternary::each_with].
    pub fn each_with(&self, f: impl Fn(Digit, Digit) -> Digit, with: Digit) -> Self {
        Self::from_ternary(&self.to_ternary().each_with(f, with))
    }

    /// See [Ternary::each_zip].
    pub fn each_zip(&self, f: impl Fn(Digit, Digit) -> Digit, other: Self) -> Self {
        Self::from_ternary(&self.to_ternary().each_zip(f, other.to_ternary()))
    }

    /// See [Ternary::each_zip_carry].
    pub fn each_zip_carry(
        &self,
        f: impl Fn(Digit, Digit, Digit) -> (Digit, Digit),
        other: Self,
    ) -> Self {
        Self::from_ternary(&self.to_ternary().each_zip_carry(f, other.to_ternary()))
    }
}

impl Display for Tryte {
    /// Formats the `Tryte` for display.
    ///
    /// The `Tryte` is displayed in its balanced ternary representation
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

impl From<&str> for Tryte {
    fn from(value: &str) -> Self {
        Self::from_ternary(&Ternary::parse(value))
    }
}

impl From<String> for Tryte {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<Tryte> for String {
    fn from(value: Tryte) -> Self {
        value.to_string()
    }
}

impl From<i16> for Tryte {
    fn from(value: i16) -> Self {
        Self::from_i16(value)
    }
}

impl From<Tryte> for i16 {
    fn from(value: Tryte) -> Self {
        value.to_i16()
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

/// A struct representing a balanced ternary number with a fixed length of 36 digits.
///
/// The underlying representation of the number is an array of 36 `Digit` values.
/// This struct provides conversion methods to and from other formats.
///
/// 6 trits x6 = 36 trits ~= 9.5 bits x6 ~= 57 bits
#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub struct Tryte36 {
    raw: [Digit; 36],
}

impl Tryte36 {
    pub const fn new(digits: [Digit; 36]) -> Self {
        Self { raw: digits }
    }

    /// `3.9883 x 10^22` or `++++++++++++++++++++++++`
    pub const MAX: Self = Self::new([Pos; 36]);
    /// `-3.9883 x 10^22` or `------------------------`
    pub const MIN: Self = Self::new([Neg; 36]);
    /// `0` or `000000000000000000000000`
    pub const ZERO: Self = Self::new([Zero; 36]);

    /// Converts the `Tryte36` into its `Ternary` representation.
    ///
    /// # Returns
    ///
    /// A `Ternary` object representing the same balanced ternary number.
    pub fn to_ternary(&self) -> Ternary {
        Ternary::new(self.raw.to_vec())
    }

    /// Retrieves a slice containing the digits of the `Tryte36`.
    ///
    /// # Returns
    ///
    /// A slice referencing the 36-digit array of the `Tryte36`.
    ///
    /// This function allows access to the raw representation of the
    /// balanced ternary number as a slice of `Digit` values.
    pub const fn to_digit_slice(&self) -> &[Digit] {
        &self.raw
    }

    /// Creates a `Tryte36` from the given `Ternary`.
    ///
    /// # Arguments
    ///
    /// * `v` - A reference to a `Ternary` object.
    ///
    /// # Panics
    ///
    /// This function panics if the `Ternary` contains more than 36 digits.
    pub fn from_ternary(v: &Ternary) -> Self {
        if v.log() > 36 {
            panic!("Cannot convert a Ternary with more than 36 digits to a Tryte36.");
        }
        let mut digits = [Zero; 36];
        for (i, d) in v.digits.iter().rev().enumerate() {
            digits[35 - i] = *d;
        }
        Self { raw: digits }
    }

    /// Converts the `Tryte36` into a signed 64-bit integer.
    ///
    /// # Returns
    ///
    /// A `i64` representing the decimal value of the `Tryte36`.
    pub fn to_i64(&self) -> i64 {
        self.to_ternary().to_dec()
    }

    /// Creates a `Tryte36` from a signed 64-bit integer.
    ///
    /// # Arguments
    ///
    /// * `v` - An signed 64-bit integer.
    ///
    /// # Returns
    ///
    /// A `Tryte36` representing the equivalent ternary number.
    pub fn from_i64(v: i64) -> Self {
        Self::from_ternary(&Ternary::from_dec(v))
    }

    /// Retrieves the digit at the specified index in the `Tryte36`.
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
    /// This function panics if the index is greater than 47.
    pub fn digit(&self, index: usize) -> Digit {
        if index > 35 {
            panic!(
                "Cannot access a digit at index {}. Tryte36 has only 36 digits.",
                index
            );
        }
        *self.raw.iter().rev().nth(index).unwrap()
    }

    /// See [Ternary::each].
    pub fn each(&self, f: impl Fn(Digit) -> Digit) -> Self {
        Self::from_ternary(&self.to_ternary().each(f))
    }

    /// See [Ternary::each_with].
    pub fn each_with(&self, f: impl Fn(Digit, Digit) -> Digit, with: Digit) -> Self {
        Self::from_ternary(&self.to_ternary().each_with(f, with))
    }

    /// See [Ternary::each_zip].
    pub fn each_zip(&self, f: impl Fn(Digit, Digit) -> Digit, other: Self) -> Self {
        Self::from_ternary(&self.to_ternary().each_zip(f, other.to_ternary()))
    }

    /// See [Ternary::each_zip_carry].
    pub fn each_zip_carry(
        &self,
        f: impl Fn(Digit, Digit, Digit) -> (Digit, Digit),
        other: Self,
    ) -> Self {
        Self::from_ternary(&self.to_ternary().each_zip_carry(f, other.to_ternary()))
    }
}

impl Display for Tryte36 {
    /// Formats the `Tryte36` for display.
    ///
    /// The `Tryte36` is displayed in its balanced ternary representation
    /// as a 36-character string.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:036}", self.to_ternary().to_string())
    }
}

impl StdNeg for Tryte36 {
    type Output = Tryte36;
    fn neg(self) -> Self::Output {
        Self::from_ternary(&-&self.to_ternary())
    }
}

impl Add for Tryte36 {
    type Output = Tryte36;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_ternary(&(&self.to_ternary() + &rhs.to_ternary()))
    }
}

impl Sub for Tryte36 {
    type Output = Tryte36;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_ternary(&(&self.to_ternary() - &rhs.to_ternary()))
    }
}

impl Mul for Tryte36 {
    type Output = Tryte36;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_ternary(&(&self.to_ternary() * &rhs.to_ternary()))
    }
}

impl Div for Tryte36 {
    type Output = Tryte36;

    fn div(self, rhs: Self) -> Self::Output {
        Self::from_ternary(&(&self.to_ternary() / &rhs.to_ternary()))
    }
}

impl BitAnd for Tryte36 {
    type Output = Tryte36;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self::from_ternary(&(&self.to_ternary() & &rhs.to_ternary()))
    }
}

impl BitOr for Tryte36 {
    type Output = Tryte36;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::from_ternary(&(&self.to_ternary() | &rhs.to_ternary()))
    }
}

impl BitXor for Tryte36 {
    type Output = Tryte36;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::from_ternary(&(&self.to_ternary() ^ &rhs.to_ternary()))
    }
}

impl Not for Tryte36 {
    type Output = Tryte36;
    fn not(self) -> Self::Output {
        -self
    }
}

impl From<Ternary> for Tryte36 {
    fn from(value: Ternary) -> Self {
        Tryte36::from_ternary(&value)
    }
}

impl From<Tryte36> for Ternary {
    fn from(value: Tryte36) -> Self {
        value.to_ternary()
    }
}

impl From<&str> for Tryte36 {
    fn from(value: &str) -> Self {
        Self::from_ternary(&Ternary::parse(value))
    }
}

impl From<String> for Tryte36 {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<Tryte36> for String {
    fn from(value: Tryte36) -> Self {
        value.to_string()
    }
}

impl From<i64> for Tryte36 {
    fn from(value: i64) -> Self {
        Self::from_i64(value)
    }
}

impl From<Tryte36> for i64 {
    fn from(value: Tryte36) -> Self {
        value.to_i64()
    }
}
