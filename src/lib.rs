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
//! ## Features
//!
//! All features are enabled by default.
//!
//! To enable only some features, use the `default-features` option
//! in your [dependency declaration](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features):
//!
//! ```toml
//! [dependencies.balanced-ternary]
//! version = "*.*"
//! default-features = false
//!
//! # Choose which one to enable
//! features = ["ternary-string", "tryte", "ternary-store"]
//! ```
//!
//! ### Featureless
//!
//! Without any feature, this library provide the type `Digit` and all its operations and the trait `DigitOperate`.
//!
//! ### `ternary-string`
//!
//! Add the structure [Ternary] which is a vector of [Digit]s and a lot of utilities
//! to manipulate digits into the ternary number. Implements [DigitOperate].
//!
//! ### `tryte`
//!
//! > Needs the feature `ternary-string`.
//!
//! Add the type [Tryte]`<N>` which is a fixed size copy-type ternary number. Implements [DigitOperate].
//!
//! ### `ternary-store`
//!
//! > Needs the feature `ternary-string`.
//!
//! Add structures to store ternaries efficiently. These types are provided:
//! - [DataTernary]: a variable length ternary number stored into [TritsChunk]s,
//! - [TritsChunk]: a fixed size copy-type 5 digits stored into one byte,
//! - [Ter40]: a fixed size copy-type 40 digits stored into one 64 bits integer. Implements [DigitOperate].
//!

#![no_std]
extern crate alloc;

pub mod concepts;

#[cfg(feature = "ternary-string")]
use alloc::{format, string::String, vec, vec::Vec};

use crate::concepts::DigitOperate;
use core::hash::{Hash, Hasher};
#[cfg(feature = "ternary-string")]
use core::{
    cmp::Ordering,
    error::Error,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[cfg(feature = "ternary-string")]
/// Error returned when parsing a string into a [`Ternary`] fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseTernaryError;

#[cfg(feature = "ternary-string")]
impl Display for ParseTernaryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid character in balanced ternary string")
    }
}

#[cfg(feature = "ternary-string")]
impl Error for ParseTernaryError {}

/// Provides helper functions for formatting integers in a given radix.
///
/// Used internally to convert decimal numbers into their ternary representation.
/// - `x`: The number to be formatted.
/// - `radix`: The base of the numeral system.
///
/// Returns a string representation of the number in the specified base.
#[cfg(feature = "ternary-string")]
fn format_radix(x: i64, radix: u32) -> String {
    let mut result = vec![];
    let sign = x.signum();
    let mut x = x.unsigned_abs();
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

mod digit;

pub use crate::digit::{
    Digit,
    Digit::{Neg, Pos, Zero},
};

/// Converts a character into a `Digit`.
///
/// # Arguments
/// * `from` - A single character (`+`, `0`, or `-`).
/// * **Panics** if the input character is invalid.
///
/// # Returns
/// * A `Digit` enum corresponding to the character.
///
/// # Example
/// ```
/// use balanced_ternary::{trit, Digit};
///
/// let digit = trit('+');
/// assert_eq!(digit, Digit::Pos);
/// ```
pub const fn trit(from: char) -> Digit {
    Digit::from_char(from)
}

/// Converts a string representation of a balanced ternary number into a `Ternary` object.
///
/// This function is a convenient shorthand for creating `Ternary` numbers
/// from string representations. The input string must consist of balanced
/// ternary characters: `+`, `0`, and `-`.
///
/// # Arguments
///
/// * `from` - A string slice representing the balanced ternary number.
/// * **Panics** if an input character is invalid.
///
/// # Returns
///
/// A `Ternary` object created from the provided string representation.
///
/// # Example
/// ```
/// use balanced_ternary::{ter, Ternary};
///
/// let ternary = ter("+-0+");
/// assert_eq!(ternary.to_string(), "+-0+");
/// let ternary = "+-0+".parse::<Ternary>().unwrap();
/// assert_eq!(ternary.to_string(), "+-0+");
/// ```
#[cfg(feature = "ternary-string")]
pub fn ter(from: &str) -> Ternary {
    Ternary::parse(from)
}

#[cfg(feature = "tryte")]
/// Creates a `Tryte` object from a string representation of a balanced ternary number.
/// It contains approximately 9.5 bits of information.
///
/// This function first converts the input string representation into a `Ternary` object
/// using the `ter` function, and then constructs a `Tryte` from that `Ternary`.
///
/// # Panics
///
/// This function panics if the `Ternary` contains more than 6 digits or if an input character is invalid.
///
/// # Arguments
///
/// * `from` - A string slice representing the balanced ternary number. It must contain
///   valid balanced ternary characters (`+`, `0`, or `-`) only.
/// * Panics if an input character is invalid.
///
/// # Returns
///
/// A `Tryte` object constructed from the provided balanced ternary string.
///
/// # Example
/// ```
/// use balanced_ternary::{tryte, Tryte};
///
/// let tryte_value = tryte("+0+0");
/// assert_eq!(tryte_value.to_string(), "00+0+0");
/// ```
pub fn tryte(from: &str) -> Tryte {
    Tryte::from_ternary(&ter(from))
}

/// Creates a `DataTernary` object from a string representation of a balanced ternary number.
///
/// This function converts the provided string representation of a balanced ternary number
/// into a `DataTernary` object. It first parses the input string into a `Ternary`
/// using the `ter` function, and then constructs the `DataTernary`.
///
/// # Arguments
///
/// * `from` - A string slice that contains a valid balanced ternary number.
///   Valid characters are `+`, `0`, and `-`.
///
/// # Panics
///
/// * Panics if the input contains invalid balanced ternary characters.
///
/// # Returns
///
/// A `DataTernary` object constructed from the input string.
///
/// # Example
/// ```
/// use balanced_ternary::{dter, DataTernary};
///
/// let data_ternary = dter("+-0-");
/// assert_eq!(data_ternary.to_string(), "0+-0-");
/// ```
#[cfg(feature = "ternary-store")]
pub fn dter(from: &str) -> DataTernary {
    DataTernary::from_ternary(ter(from))
}

/// Represents a balanced ternary number using a sequence of `Digit`s.
///
/// Provides functions for creating, parsing, converting, and manipulating balanced ternary numbers.
#[derive(Debug, Clone)]
#[cfg(feature = "ternary-string")]
pub struct Ternary {
    digits: Vec<Digit>,
}

#[cfg(feature = "ternary-string")]
impl Ternary {
    /// Creates a new balanced ternary number from a vector of `Digit`s.
    pub fn new(digits: Vec<Digit>) -> Ternary {
        Ternary { digits }
    }

    /// Returns the number of digits (length) of the balanced ternary number.
    pub fn log(&self) -> usize {
        self.digits.len()
    }

    /// Retrieves a slice containing the digits of the `Ternary`.
    ///
    /// # Returns
    ///
    /// A slice referencing the digits vec of the `Ternary`.
    ///
    /// This function allows access to the raw representation of the
    /// balanced ternary number as a slice of `Digit` values.
    pub fn to_digit_slice(&self) -> &[Digit] {
        self.digits.as_slice()
    }

    /// Returns an iterator over the digits from most significant to least significant.
    pub fn iter(&self) -> core::slice::Iter<'_, Digit> {
        self.digits.iter()
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
    ///
    /// # Example
    /// ```
    /// use balanced_ternary::Ternary;
    ///
    /// let ternary = "+-0".parse::<Ternary>().unwrap();
    /// assert_eq!(ternary.to_string(), "+-0");
    /// ```
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
        self.checked_to_dec()
            .expect("Ternary value does not fit in i64")
    }

    /// Creates a balanced ternary number from a decimal integer.
    ///
    /// The input number is converted into its balanced ternary representation,
    /// with digits represented as `Digit`s.
    pub fn from_dec(mut dec: i64) -> Self {
        let mut digits = Vec::new();
        loop {
            let remainder = dec.rem_euclid(3);
            dec = dec.div_euclid(3);
            let digit = match remainder {
                0 => Zero,
                1 => Pos,
                2 => {
                    dec += 1;
                    Neg
                }
                _ => unreachable!(),
            };
            digits.push(digit);
            if dec == 0 {
                break;
            }
        }
        digits.reverse();
        Self::new(digits)
    }

    pub fn checked_to_dec(&self) -> Option<i64> {
        self.digits.iter().try_fold(0i64, |acc, digit| {
            acc.checked_mul(3)?.checked_add(digit.to_i8() as i64)
        })
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

    /// Removes leading `Zero` digits from the `Ternary` number, effectively trimming
    /// it down to its simplest representation. The resulting `Ternary` number
    /// will still represent the same value.
    ///
    /// # Returns
    ///
    /// * `Self` - A new `Ternary` object, trimmed of leading zeros.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_ternary::{ Neg, Pos, Ternary, Zero};
    ///
    /// let ternary = Ternary::new(vec![Zero, Zero, Pos, Neg]);
    /// let trimmed = ternary.trim();
    /// assert_eq!(trimmed.to_string(), "+-");
    /// ```
    ///
    /// # Notes
    ///
    /// This method does not mutate the original `Ternary` object but returns a new representation.
    pub fn trim(&self) -> Self {
        if self.to_dec() == 0 {
            return Ternary::parse("0");
        }
        let mut repr = Ternary::new(vec![]);
        let mut first_digit = false;
        for digit in self.digits.iter() {
            if !first_digit && digit != &Zero {
                first_digit = true;
            }
            if first_digit {
                repr.digits.push(*digit);
            }
        }
        repr
    }

    /// Adjusts the representation of the `Ternary` number to have a fixed number of digits.
    ///
    /// If the current `Ternary` has fewer digits than the specified `length`, leading zero digits
    /// will be added to the `Ternary` to match the desired length. If the current `Ternary` has
    /// more digits than the specified `length`, it will be returned unmodified.
    ///
    /// # Arguments
    ///
    /// * `length` - The desired length of the `Ternary` number.
    ///
    /// # Returns
    ///
    /// * `Self` - A new `Ternary` object with the specified fixed length.
    ///
    /// # Notes
    ///
    /// If `length` is smaller than the existing number of digits, the function does not truncate
    /// the number but instead returns the original `Ternary` unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_ternary::{Ternary, Zero, Pos};
    ///
    /// let ternary = Ternary::new(vec![Pos]);
    /// let fixed = ternary.with_length(5);
    /// assert_eq!(fixed.to_string(), "0000+");
    ///
    /// let fixed = ternary.with_length(1);
    /// assert_eq!(fixed.to_string(), "+");
    /// ```
    pub fn with_length(&self, length: usize) -> Self {
        if length < self.log() {
            return self.clone();
        }
        let zeroes = vec![Zero; length - self.log()];
        let mut repr = Ternary::new(vec![]);
        repr.digits.extend(zeroes);
        repr.digits.extend(self.digits.iter().cloned());
        repr
    }

    /// Converts the `Ternary` number into a string representation by applying a given
    /// transformation function to each digit of the ternary number.
    ///
    /// # Arguments
    ///
    /// * `transform` - A function or closure that takes a `Digit` and returns a `char`, representing the digit.
    ///
    /// # Returns
    ///
    /// A `String`-based representation of the `Ternary` number resulting from
    /// applying the transformation to its digits.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_ternary::{Digit, Pos, Neg, Zero, Ternary};
    ///
    /// let ternary = Ternary::new(vec![Pos, Zero, Neg]);
    ///
    /// let custom_repr = ternary.to_string_repr(Digit::to_char_t);
    /// assert_eq!(custom_repr, "10T");
    /// let custom_repr = ternary.to_string_repr(Digit::to_char_theta);
    /// assert_eq!(custom_repr, "10Θ");
    /// let custom_repr = ternary.to_string_repr(Digit::to_char);
    /// assert_eq!(custom_repr, "+0-");
    /// ```
    ///
    /// # Notes
    ///
    /// * The function provides flexibility to define custom string representations
    ///   for the ternary number digits.
    /// * Call to `Ternary::to_string()` is equivalent to `Ternary::to_string_repr(Digit::to_char)`.
    pub fn to_string_repr<F: Fn(&Digit) -> char>(&self, transform: F) -> String {
        let mut str = String::new();
        for digit in self.digits.iter() {
            str.push(transform(digit));
        }
        str
    }

    /// Concatenates the current `Ternary` number with another `Ternary` number.
    ///
    /// This function appends the digits of the provided `Ternary` object to the digits
    /// of the current `Ternary` object, creating a new `Ternary` number as the result.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to the `Ternary` number to be concatenated to the current one.
    ///
    /// # Returns
    ///
    /// * `Ternary` - A new `Ternary` object formed by concatenating the digits.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_ternary::{Ternary, Pos, Zero, Neg};
    ///
    /// let ternary1 = Ternary::new(vec![Pos, Zero]);
    /// let ternary2 = Ternary::new(vec![Neg, Pos]);
    ///
    /// let concatenated = ternary1.concat(&ternary2);
    /// assert_eq!(concatenated.to_string(), "+0-+");
    /// ```
    pub fn concat(&self, other: &Ternary) -> Ternary {
        let mut t = Ternary::new(vec![]);
        t.digits.extend(self.digits.iter().cloned());
        t.digits.extend(other.digits.iter().cloned());
        t
    }

    fn significant_digits(&self) -> &[Digit] {
        let start = self
            .digits
            .iter()
            .position(|digit| *digit != Zero)
            .unwrap_or(self.digits.len());

        &self.digits[start..]
    }
}

#[cfg(feature = "ternary-string")]
impl PartialEq for Ternary {
    fn eq(&self, other: &Self) -> bool {
        self.significant_digits() == other.significant_digits()
    }
}

#[cfg(feature = "ternary-string")]
impl Eq for Ternary {}

#[cfg(feature = "ternary-string")]
impl Hash for Ternary {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.significant_digits().hash(state);
    }
}

#[cfg(feature = "ternary-string")]
impl DigitOperate for Ternary {
    fn to_digits(&self) -> Vec<Digit> {
        self.to_digit_slice().to_vec()
    }

    fn digit(&self, index: usize) -> Option<Digit> {
        self.get_digit(index).cloned()
    }

    fn each(&self, f: impl Fn(Digit) -> Digit) -> Self {
        Self::new(self.digits.iter().copied().map(f).collect())
    }

    fn each_with(&self, f: impl Fn(Digit, Digit) -> Digit, other: Digit) -> Self {
        Self::new(
            self.digits
                .iter()
                .copied()
                .map(|digit| f(digit, other))
                .collect(),
        )
    }

    fn each_zip(&self, f: impl Fn(Digit, Digit) -> Digit, other: Self) -> Self {
        let len = self.digits.len().max(other.digits.len());
        let mut digits = Vec::with_capacity(len);
        let lhs = self
            .digits
            .iter()
            .rev()
            .copied()
            .chain(core::iter::repeat(Zero));
        let rhs = other
            .digits
            .iter()
            .rev()
            .copied()
            .chain(core::iter::repeat(Zero));
        for (a, b) in lhs.zip(rhs).take(len) {
            digits.push(f(a, b));
        }
        digits.reverse();
        Self::new(digits)
    }

    fn each_zip_carry(
        &self,
        f: impl Fn(Digit, Digit, Digit) -> (Digit, Digit),
        other: Self,
    ) -> Self {
        let len = self.digits.len().max(other.digits.len());
        let mut digits = Vec::with_capacity(len.saturating_add(1));
        let lhs = self
            .digits
            .iter()
            .rev()
            .copied()
            .chain(core::iter::repeat(Zero));
        let rhs = other
            .digits
            .iter()
            .rev()
            .copied()
            .chain(core::iter::repeat(Zero));
        let mut carry = Zero;
        for (a, b) in lhs.zip(rhs).take(len) {
            let (next_carry, result) = f(a, b, carry);

            digits.push(result);
            carry = next_carry;
        }
        if carry != Zero {
            digits.push(carry);
        }
        digits.reverse();
        Self::new(digits)
    }
}

#[cfg(feature = "ternary-string")]
impl Display for Ternary {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_string_repr(Digit::to_char))
    }
}

#[cfg(feature = "ternary-string")]
impl FromStr for Ternary {
    type Err = ParseTernaryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|c| matches!(c, '+' | '0' | '-')) {
            Ok(Ternary::parse(s))
        } else {
            Err(ParseTernaryError)
        }
    }
}

#[cfg(feature = "ternary-string")]
impl Ord for Ternary {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.significant_digits();
        let b = other.significant_digits();

        let sign_a = a.first().map_or(0, Digit::to_i8);
        let sign_b = b.first().map_or(0, Digit::to_i8);
        match sign_a.cmp(&sign_b) {
            Ordering::Equal => {}
            order => return order,
        }

        if sign_a == 0 {
            return Ordering::Equal;
        }

        let length_order = if sign_a > 0 {
            a.len().cmp(&b.len())
        } else {
            b.len().cmp(&a.len())
        };
        if length_order != Ordering::Equal {
            return length_order;
        }

        a.iter().map(Digit::to_i8).cmp(b.iter().map(Digit::to_i8))
    }
}

#[cfg(feature = "ternary-string")]
impl PartialOrd for Ternary {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "ternary-string")]
impl IntoIterator for Ternary {
    type Item = Digit;
    type IntoIter = alloc::vec::IntoIter<Digit>;

    fn into_iter(self) -> Self::IntoIter {
        self.digits.into_iter()
    }
}

#[cfg(feature = "ternary-string")]
mod operations;

mod conversions;

#[cfg(feature = "ternary-store")]
mod store;

#[cfg(feature = "ternary-store")]
pub use crate::store::{DataTernary, Ter40, TritsChunk};

#[cfg(feature = "tryte")]
mod tryte;

#[cfg(feature = "tryte")]
pub use crate::tryte::Tryte;

#[cfg(test)]
#[cfg(feature = "ternary-string")]
#[test]
fn test_ternary() {
    use crate::*;
    use alloc::string::ToString;

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

#[cfg(test)]
#[cfg(feature = "ternary-string")]
#[test]
fn test_each() {
    use crate::*;
    use alloc::string::ToString;
    let ternary = Ternary::parse("+0-");
    assert_eq!(ternary.each(Digit::possibly).to_string(), "++-");
}

#[cfg(test)]
#[cfg(feature = "ternary-string")]
#[test]
fn test_operations() {
    use alloc::string::ToString;
    fn test_ternary_eq(a: Ternary, b: &str) {
        let repr = Ternary::parse(b);
        assert_eq!(a.to_string(), repr.to_string());
    }
    fn test_binary_op(a: &Ternary, f: impl Fn(Digit, Digit) -> Digit, b: &Ternary, c: &str) {
        test_ternary_eq(a.each_zip(f, b.clone()), c);
    }

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

#[cfg(test)]
#[cfg(feature = "ternary-string")]
#[test]
fn test_from_str() {
    use alloc::string::ToString;
    use core::str::FromStr;

    let ternary = Ternary::from_str("+-0").unwrap();
    assert_eq!(ternary.to_string(), "+-0");

    assert!(Ternary::from_str("+-x").is_err());

    #[cfg(feature = "tryte")]
    {
        let tryte = <crate::Tryte>::from_str("+-0").unwrap();
        assert_eq!(tryte.to_string(), "000+-0");
        assert!(<crate::Tryte>::from_str("+-x").is_err());
    }
}

#[cfg(test)]
#[cfg(feature = "ternary-string")]
#[test]
fn test_ordering() {
    use crate::ter;

    assert!(ter("-+") < ter("0"));
    assert!(ter("0") < ter("++"));
}

#[cfg(test)]
#[cfg(feature = "ternary-string")]
#[test]
fn test_ordering_additional() {
    use crate::ter;

    // Validate comparisons across a range of values
    assert!(ter("--") < ter("-0"));
    assert!(ter("-0") < ter("-"));
    assert!(ter("+") < ter("+-"));
    assert!(ter("+-") < ter("++"));

    // Sorting should arrange values by their decimal value
    let mut values = vec![
        ter("+"),
        ter("--"),
        ter("+-"),
        ter("-"),
        ter("0"),
        ter("-0"),
        ter("++"),
    ];
    values.sort();
    let expected = vec![
        ter("--"),
        ter("-0"),
        ter("-"),
        ter("0"),
        ter("+"),
        ter("+-"),
        ter("++"),
    ];
    assert_eq!(values, expected);
}

#[cfg(test)]
#[cfg(feature = "ternary-string")]
#[test]
fn test_iterators() {
    use crate::*;

    let ternary = Ternary::parse("+0-");
    let expected = vec![Pos, Zero, Neg];
    assert_eq!(ternary.iter().cloned().collect::<Vec<_>>(), expected);
    let collected: Vec<Digit> = Ternary::parse("+0-").into_iter().collect();
    assert_eq!(collected, expected);
}

#[cfg(test)]
#[cfg(feature = "ternary-string")]
#[test]
fn test_i64_conversion_overflow() {
    use crate::{Neg, Pos, Ternary, Zero};

    let positive = Ternary::new(vec![Pos; 41]);
    let negative = Ternary::new(vec![Neg; 41]);

    assert_eq!(positive.checked_to_dec(), None);
    assert_eq!(negative.checked_to_dec(), None);

    // Leading zeros must not cause an overflow.
    let zero = Ternary::new(vec![Zero; 100]);
    assert_eq!(zero.checked_to_dec(), Some(0));
}

#[cfg(test)]
#[cfg(feature = "ternary-string")]
#[test]
fn numeric_equality() {
    assert_eq!(Ternary::parse("+"), Ternary::parse("000+"));
    assert_eq!(Ternary::parse("-"), Ternary::parse("00-"));
    assert_eq!(Ternary::parse("0"), Ternary::parse("000"));
    assert_eq!(Ternary::parse("0"), Ternary::new(vec![]));

    assert_ne!(Ternary::parse("+"), Ternary::parse("-"));
}

#[cfg(test)]
#[cfg(feature = "ternary-string")]
#[test]
fn numeric_ordering() {
    assert!(Ternary::parse("--") < Ternary::parse("-"));
    assert!(Ternary::parse("-") < Ternary::parse("0"));
    assert!(Ternary::parse("0") < Ternary::parse("+"));
    assert!(Ternary::parse("+") < Ternary::parse("++"));

    assert!(Ternary::parse("-+") < Ternary::parse("-"));
    assert!(Ternary::parse("+") < Ternary::parse("+-"));

    assert_eq!(
        Ternary::parse("000+").cmp(&Ternary::parse("+")),
        Ordering::Equal
    );
}

#[cfg(test)]
#[cfg(feature = "ternary-string")]
#[test]
fn arbitrary_precision_comparison() {
    let positive = Ternary::parse(&"+".repeat(256));
    let larger = Ternary::parse(&"+".repeat(257));

    assert!(positive < larger);

    let negative = Ternary::parse(&"-".repeat(256));
    let smaller = Ternary::parse(&"-".repeat(257));

    assert!(smaller < negative);
}

#[cfg(test)]
#[cfg(feature = "ternary-string")]
mod test_each {
    use alloc::string::ToString;
    use crate::{
        Digit, Ternary,
    };

    use Digit::{Pos, Zero};
    use crate::concepts::DigitOperate;

    fn add_digits(a: Digit, b: Digit, carry: Digit) -> (Digit, Digit) {
        let sum = a.to_i8() + b.to_i8() + carry.to_i8();
        let next = (sum + 1).div_euclid(3);

        (
            Digit::from_i8(next),
            Digit::from_i8(sum - next * 3),
        )
    }

    #[test]
    fn ternary_each_preserves_length() {
        let a = Ternary::parse("00+-");

        assert_eq!(a.each(|d| d).to_string(), "00+-");

        assert_eq!(
            a.each_with(|d, _| d, Pos).to_string(),
            "00+-"
        );
    }

    #[test]
    fn ternary_zip_preserves_argument_order() {
        let a = Ternary::parse("+");
        let b = Ternary::parse("--");

        assert_eq!(
            a.each_zip(|left, _| left, b.clone()).to_string(),
            "0+"
        );

        assert_eq!(
            a.each_zip(|_, right| right, b.clone()).to_string(),
            "--"
        );

        assert_eq!(
            a.each_zip_carry(
                |left, _, _| (Zero, left),
                b,
            ).to_string(),
            "0+"
        );
    }

    #[test]
    fn ternary_carry_is_preserved() {
        let a = Ternary::parse("++");
        let b = Ternary::parse("+");

        assert_eq!(
            a.each_zip_carry(add_digits, b).to_string(),
            "+--"
        );

        assert_eq!(
            Ternary::parse("-")
                .each_zip_carry(add_digits, Ternary::parse("-"))
                .to_string(),
            "-+"
        );
    }

    #[cfg(feature = "tryte")]
    #[test]
    fn tryte_carry_wraps_at_fixed_width() {
        use crate::Tryte;

        let a = Tryte::<2>::new([Pos, Pos]);
        let b = Tryte::<2>::new([Zero, Pos]);

        assert_eq!(
            a.each_zip_carry(add_digits, b),
            Tryte::<2>::MIN
        );

        assert_eq!(Tryte::<0>::ZERO.digit(0), None);
    }

    #[cfg(feature = "ternary-store")]
    #[test]
    fn ter40_carry_wraps_at_fixed_width() {
        use crate::Ter40;

        let result = Ter40::MAX.each_zip_carry(
            add_digits,
            Ter40::from_dec(1),
        );

        assert_eq!(result, Ter40::MIN);
    }

}