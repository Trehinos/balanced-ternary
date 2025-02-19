//! A [balanced ternary](https://en.wikipedia.org/wiki/Balanced_ternary) data structure.
//!
//! A `Ternary` object in this module represents a number in the balanced ternary numeral system.
//! Balanced ternary is a non-standard positional numeral system that uses three digits: {-1, 0, +1}
//! represented here as `Neg` for -1, `Zero` for 0, and `Pos` for +1. It is useful in some domains
//! of computer science and mathematics due to its arithmetic properties and representation
//! symmetry.
//!
//! # Data Structures
//!
//! - **`Digit` Enum**:
//!     Represents a single digit for balanced ternary values, with possible values:
//!     - `Neg` for -1
//!     - `Zero` for 0
//!     - `Pos` for +1
//!
//! - **`Ternary` Struct**:
//!     Represents a balanced ternary number as a collection of `Digit`s.
//!     Provides utility functions for conversion, parsing, and manipulation.
//!
//! # Examples
//!
//! ## Converting between representations:
//! ```rust
//! use balanced_ternary::*;
//!
//! let ternary = Ternary::from_dec(5);
//! assert_eq!(ternary.to_string(), "+--");
//! assert_eq!(ternary.to_dec(), 5);
//!
//! let parsed = Ternary::parse("+--");
//! assert_eq!(parsed.to_string(), "+--");
//! assert_eq!(parsed.to_dec(), 5);
//! ```
//!
//! ## Negative numbers:
//! ```rust
//! use balanced_ternary::*;
//!
//! let neg_five = Ternary::from_dec(-5);
//! assert_eq!(neg_five.to_string(), "-++");
//! assert_eq!(neg_five.to_dec(), -5);
//!
//! let negated = -&neg_five;
//! assert_eq!(negated.to_string(), "+--");
//! assert_eq!(negated.to_dec(), 5);
//! ```
//!
//! ## Larger numbers:
//! ```rust
//! use balanced_ternary::*;
//!
//! let big = Ternary::from_dec(121);
//! assert_eq!(big.to_string(), "+++++");
//! assert_eq!(big.to_dec(), 121);
//!
//! let neg_big = Ternary::from_dec(-121);
//! assert_eq!(neg_big.to_string(), "-----");
//! assert_eq!(neg_big.to_dec(), -121);
//! ```
//!
//! ## Operations
//! ```
//! use balanced_ternary::Ternary;
//!
//! let repr9 = Ternary::parse("+00");
//! let repr4 = Ternary::parse("++");
//! let repr13 = &repr9 + &repr4;
//! let repr17 = &repr13 + &repr4;
//! let repr34 = &repr17 + &repr17;
//!
//! assert_eq!(repr13.to_string(), "+++");
//! assert_eq!(repr17.to_string(), "+-0-");
//! assert_eq!(repr34.to_string(), "++-+");
//!
//! let repr30 = &repr34 - &repr4;
//! assert_eq!(repr30.to_dec(), 30);
//! assert_eq!(repr30.to_string(), "+0+0");
//! ```
//!
#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};
use core::fmt::{Display, Formatter};
use core::str::FromStr;

/// Provides helper functions for formatting integers in a given radix.
///
/// Used internally to convert decimal numbers into their ternary representation.
/// - `x`: The number to be formatted.
/// - `radix`: The base of the numeral system.
///
/// Returns a string representation of the number in the specified base.
fn format_radix(x: i64, radix: u32) -> String {
    let mut result = vec![];
    let sign = x.signum();
    let mut x = x.abs() as u64;
    loop {
        let m = (x % radix as u64) as u32;
        x /= radix as u64;
        result.push(core::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    format!(
        "{}{}",
        if sign == -1 { "-" } else { "" },
        result.into_iter().rev().collect::<String>()
    )
}

pub mod digit;

pub use crate::digit::{
    Digit,
    Digit::{Neg, Pos, Zero},
};

/// Represents a balanced ternary number using a sequence of `Digit`s.
///
/// Provides functions for creating, parsing, converting, and manipulating balanced ternary numbers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ternary {
    digits: Vec<Digit>,
}

impl Ternary {
    /// Creates a new balanced ternary number from a vector of `Digit`s.
    pub fn new(digits: Vec<Digit>) -> Ternary {
        Ternary { digits }
    }

    /// Returns the number of digits (length) of the balanced ternary number.
    pub fn log(&self) -> usize {
        self.digits.len()
    }

    /// Returns a reference to the [Digit] indexed by `index` if it exists.
    ///
    /// Digits are indexed **from the right**:
    /// ```
    /// use balanced_ternary::Ternary;
    ///
    /// // Indexes :
    /// //                              32
    /// //                             4||1
    /// //                            5||||0
    /// //                            ||||||
    /// //                            vvvvvv
    /// let ternary = Ternary::parse("+++--+");
    /// assert_eq!(ternary.get_digit(1).unwrap().to_char(), '-')
    /// ```
    pub fn get_digit(&self, index: usize) -> Option<&Digit> {
        self.digits.iter().rev().nth(index)
    }

    /// Parses a string representation of a balanced ternary number into a `Ternary` object.
    ///
    /// Each character in the string must be one of `+`, `0`, or `-`.
    pub fn parse(str: &str) -> Self {
        let mut repr = Ternary::new(vec![]);
        for c in str.chars() {
            repr.digits.push(Digit::from_char(c));
        }
        repr
    }

    /// Converts the `Ternary` object to its integer (decimal) representation.
    ///
    /// Calculates the sum of each digit's value multiplied by the appropriate power of 3.
    pub fn to_dec(&self) -> i64 {
        let mut dec = 0;
        for (rank, digit) in self.digits.iter().rev().enumerate() {
            dec += digit.to_i8() as i64 * 3_i64.pow(rank as u32);
        }
        dec
    }

    /// Creates a balanced ternary number from a decimal integer.
    ///
    /// The input number is converted into its balanced ternary representation,
    /// with digits represented as `Digit`s.
    pub fn from_dec(dec: i64) -> Self {
        let sign = dec.signum();
        let str = format_radix(dec.abs(), 3);
        let mut carry = 0u8;
        let mut repr = Ternary::new(vec![]);
        for digit in str.chars().rev() {
            let digit = u8::from_str(&digit.to_string()).unwrap() + carry;
            if digit < 2 {
                repr.digits.push(Digit::from_i8(digit as i8));
                carry = 0;
            } else if digit == 2 {
                repr.digits.push(Digit::from_i8(-1));
                carry = 1;
            } else if digit == 3 {
                repr.digits.push(Digit::from_i8(0));
                carry = 1;
            } else {
                panic!("Ternary::from_dec(): Invalid digit: {}", digit);
            }
        }
        if carry == 1 {
            repr.digits.push(Digit::from_i8(1));
        }
        repr.digits.reverse();
        if sign == -1 {
            -&repr
        } else {
            repr
        }
    }

    /// Converts the balanced ternary number to its unbalanced representation as a string.
    ///
    /// The unbalanced representation treats the digits as standard ternary (0, 1, 2),
    /// instead of balanced ternary (-1, 0, +1). Negative digits are handled by
    /// calculating the decimal value of the balanced ternary number and converting
    /// it back to an unbalanced ternary string.
    ///
    /// Returns:
    /// * `String` - The unbalanced ternary representation of the number, where each
    /// digit is one of `0`, `1`, or `2`.
    ///
    /// Example:
    /// ```
    /// use balanced_ternary::Ternary;
    ///
    /// let repr = Ternary::parse("+--");
    /// assert_eq!(repr.to_unbalanced(), "12");
    /// assert_eq!(repr.to_dec(), 5);
    /// let repr = Ternary::parse("-++");
    /// assert_eq!(repr.to_unbalanced(), "-12");
    /// assert_eq!(repr.to_dec(), -5);
    /// ```
    pub fn to_unbalanced(&self) -> String {
        format_radix(self.to_dec(), 3)
    }

    /// Parses a string representation of an unbalanced ternary number into a `Ternary` object.
    ///
    /// The string must only contain characters valid in the unbalanced ternary numeral system (`0`, `1`, or `2`).
    /// Each character is directly converted into its decimal value and then interpreted as a balanced ternary number.
    ///
    /// # Arguments
    ///
    /// * `unbalanced` - A string slice representing the unbalanced ternary number.
    ///
    /// # Returns
    ///
    /// A `Ternary` object representing the same value as the input string in balanced ternary form.
    ///
    /// # Panics
    ///
    /// This function will panic if the string is not a valid unbalanced ternary number.
    /// For instance, if it contains characters other than `0`, `1`, or `2`.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_ternary::Ternary;
    ///
    /// let ternary = Ternary::from_unbalanced("-12");
    /// assert_eq!(ternary.to_string(), "-++");
    /// assert_eq!(ternary.to_dec(), -5);
    /// ```
    pub fn from_unbalanced(unbalanced: &str) -> Self {
        Self::from_dec(i64::from_str_radix(unbalanced, 3).unwrap())
    }
}

impl Display for Ternary {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut str = String::new();
        for digit in self.digits.iter() {
            str.push(digit.to_char());
        }
        write!(f, "{}", str)
    }
}

pub mod operations;

#[cfg(feature = "tryte")]
pub mod tryte;

#[cfg(feature = "tryte")]
pub use crate::tryte::Tryte;

#[cfg(test)]
#[test]
fn test_ternary() {
    use crate::*;

    let repr5 = Ternary::new(vec![Pos, Neg, Neg]);
    assert_eq!(repr5.to_dec(), 5);
    let repr5 = Ternary::from_dec(5);
    assert_eq!(repr5.to_dec(), 5);

    let repr13 = Ternary::new(vec![Pos, Pos, Pos]);
    assert_eq!(repr13.to_dec(), 13);

    let repr14 = Ternary::parse("+---");
    let repr15 = Ternary::parse("+--0");
    assert_eq!(repr14.to_dec(), 14);
    assert_eq!(repr15.to_dec(), 15);
    assert_eq!(repr14.to_string(), "+---");
    assert_eq!(repr15.to_string(), "+--0");

    let repr120 = Ternary::from_dec(120);
    assert_eq!(repr120.to_dec(), 120);
    assert_eq!(repr120.to_string(), "++++0");
    let repr121 = Ternary::from_dec(121);
    assert_eq!(repr121.to_dec(), 121);
    assert_eq!(repr121.to_string(), "+++++");

    let repr_neg_5 = Ternary::parse("-++");
    assert_eq!(repr_neg_5.to_dec(), -5);
    assert_eq!(repr_neg_5.to_string(), "-++");

    let repr_neg_5 = Ternary::from_dec(-5);
    assert_eq!(repr_neg_5.to_dec(), -5);
    assert_eq!(repr_neg_5.to_string(), "-++");

    let repr_neg_121 = Ternary::from_dec(-121);
    assert_eq!(repr_neg_121.to_dec(), -121);
    assert_eq!(repr_neg_121.to_string(), "-----");

    let test = Ternary::from_dec(18887455);
    assert_eq!(test.to_dec(), 18887455);
    assert_eq!(test.to_string(), "++00--0--+-0++0+");

    let unbalanced = Ternary::from_unbalanced("12");
    assert_eq!(unbalanced.to_dec(), 5);
    assert_eq!(unbalanced.to_string(), "+--");

    let unbalanced = Ternary::from_unbalanced("-12");
    assert_eq!(unbalanced.to_dec(), -5);
    assert_eq!(unbalanced.to_string(), "-++");

    let unbalanced = Ternary::from_dec(121);
    assert_eq!(unbalanced.to_unbalanced(), "11111");
    assert_eq!(unbalanced.to_string(), "+++++");
}
