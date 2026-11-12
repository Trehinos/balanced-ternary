//! A [balanced ternary](https://en.wikipedia.org/wiki/Balanced_ternary) data structure.
//!
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
//! `Ternary` represents arbitrary-precision signed integers
//! using balanced ternary digits. Arithmetic is performed
//! directly on digit sequences, while conversions to native
//! integer types remain limited by their respective ranges.
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
//! - [DataTernary]: variable-length storage of five-trit [TritsChunk]s, with structural equality and hashing, without ordering or arithmetic,
//! - [TritsChunk]: a fixed size copy-type 5 digits stored into one byte,
//! - [Ter40]: a fixed size copy-type 40 digits stored into one 64 bits integer. Implements [DigitOperate].
//!

#![no_std]
extern crate alloc;

pub mod concepts;

#[cfg(feature = "ternary-string")]
use crate::concepts::DigitOperate;
#[cfg(feature = "ternary-string")]
use alloc::{string::String, vec, vec::Vec};
#[cfg(feature = "ternary-string")]
use core::{
    cmp::Ordering,
    error::Error,
    fmt::{Display, Formatter, Write},
    hash::{Hash, Hasher},
    str::FromStr,
};

#[cfg(feature = "ternary-string")]
/// Error returned when parsing a balanced ternary representation fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseTernaryError {
    /// The input contains a character other than ones expected in a balanced ternary representation.
    InvalidCharacter {
        /// The invalid character.
        character: char,
        /// The zero-based UTF-8 byte offset in the input.
        index: usize,
    },
}

#[cfg(feature = "ternary-string")]
impl Display for ParseTernaryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidCharacter { character, index } => write!(
                f,
                "invalid balanced ternary character {character:?} at byte index {index}"
            ),
        }
    }
}

#[cfg(feature = "ternary-string")]
/// Error returned when a numeric conversion cannot be performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TernaryConversionError {
    /// The numeric value is outside the destination type's range.
    OutOfRange,
}

#[cfg(feature = "ternary-string")]
impl Display for TernaryConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OutOfRange => f.write_str("value is outside the destination type's range"),
        }
    }
}

#[cfg(feature = "ternary-string")]
impl Error for TernaryConversionError {}

#[cfg(feature = "ternary-string")]
impl Error for ParseTernaryError {}

/// Error returned when parsing an unbalanced ternary number fails.
#[cfg(feature = "ternary-string")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseUnbalancedTernaryError {
    /// The input is empty or contains only a sign.
    MissingDigits,

    /// The input contains a character other than an expected base-3 digit.
    InvalidCharacter {
        /// The invalid character.
        character: char,
        /// The zero-based UTF-8 byte offset in the original input.
        index: usize,
    },
}

#[cfg(feature = "ternary-string")]
impl Display for ParseUnbalancedTernaryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingDigits => {
                f.write_str("unbalanced ternary input must contain at least one digit")
            }
            Self::InvalidCharacter { character, index } => write!(
                f,
                "invalid unbalanced ternary character {character:?} at byte index {index}"
            ),
        }
    }
}

#[cfg(feature = "ternary-string")]
impl Error for ParseUnbalancedTernaryError {}

mod digit;

pub use crate::digit::{
    Digit,
    Digit::{Neg, Pos, Zero},
    DigitConversionError,
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
/// Parses a balanced ternary value into a six-trit word.
///
/// Leading zeros are accepted. Empty input represents zero.
///
/// # Panics
///
/// Panics if the input contains an invalid character or the numeric
/// value does not fit in six trits.
///
/// # Examples
///
/// ```
/// use balanced_ternary::tryte;
///
/// assert_eq!(tryte("000000+").to_string(), "00000+");
/// assert_eq!(tryte("").to_string(), "000000");
/// ```
pub fn tryte(from: &str) -> Tryte {
    from.parse().expect("invalid six-trit word")
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

/// An arbitrary-precision balanced ternary integer.
///
/// Represents a signed integer as a variable-length sequence
/// of balanced ternary digits (`Neg`, `Zero`, and `Pos`).
///
/// Unlike `Tryte<N>` and `Ter40`, this type has no fixed
/// numeric range.
///
/// # Arithmetic
///
/// Addition, subtraction, multiplication, division, and the
/// remainder are implemented directly on digit sequences,
/// without converting operands to machine integers.
///
/// - `+`, `-`, `*`: arbitrary-precision arithmetic.
/// - `/`: signed integer division, truncating toward zero.
/// - `%`: remainder with the sign of the dividend.
/// - `div_rem()`: computes quotient and remainder together.
///
/// Arithmetic results are normalized by removing leading
/// zeros. Zero is represented by a single `Zero` digit.
///
/// # Representation
///
/// Leading zeros are permitted in stored representations.
/// Equality, hashing, and ordering use the numeric value,
/// independently of leading-zero padding.
///
/// The original digit sequence can be accessed through
/// `to_digit_slice()`.
///
/// Empty representations are valid and have the numeric value zero.
/// Balanced parsing through `parse()`, `try_parse()`, and `FromStr`,
/// and formatting through `Display`, preserve the exact digit sequence,
/// including leading zeros and empty input.
///
/// Use `trim()` to get a canonical numeric representation.
///
/// # Conversions
///
/// Although arithmetic supports arbitrary precision,
/// `to_dec()` returns an `i64` and panics if the value
/// cannot be represented.
///
/// Use `checked_to_dec()` to get an Option, or `i64::try_from()`
/// to get a Result. Both borrowed and owned conversions are supported.
///
/// Conversions to and from unbalanced ternary support arbitrary precision
/// and produce normalized numeric representations.
///
/// Unbalanced input requires at least one digit, optionally preceded
/// by an ASCII `+` or `-`. Use `try_from_unbalanced()` for fallible parsing.
///
/// # Examples
///
/// ```
/// use balanced_ternary::Ternary;
///
/// let a = Ternary::parse(&"+".repeat(100));
/// let b = Ternary::from_dec(3);
///
/// let product = &a * &b;
///
/// assert_eq!(&product / &b, a);
/// assert_eq!(&product % &b, Ternary::from_dec(0));
///
/// // Arithmetic is not limited to i64.
/// assert_eq!(product.checked_to_dec(), None);
/// ```
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

    /// Returns the digit at the specified position,
    /// counting from the least significant digit.
    ///
    /// Returns None if the index is out of bounds.
    ///
    /// Time complexity: O(1).
    pub fn get_digit(&self, index: usize) -> Option<&Digit> {
        let position = self.digits.len().checked_sub(index)?.checked_sub(1)?;
        self.digits.get(position)
    }

    /// Parses a balanced ternary representation.
    ///
    /// Leading zeros and empty input are preserved.
    /// An empty representation has numeric value zero and displays as "".
    ///
    /// # Errors
    ///
    /// Returns the first invalid character and its UTF-8 byte offset.
    /// Only '+', '0', and '-' are accepted; whitespace is not ignored.
    pub fn try_parse(input: &str) -> Result<Self, ParseTernaryError> {
        let mut digits = Vec::with_capacity(input.len());
        for (index, character) in input.char_indices() {
            let digit = Digit::try_from_char(character)
                .map_err(|_| ParseTernaryError::InvalidCharacter { character, index })?;
            digits.push(digit);
        }
        Ok(Self::new(digits))
    }

    /// Parses a balanced ternary representation.
    ///
    /// Leading zeros and empty input are preserved.
    ///
    /// # Panics
    ///
    /// Panics if the input contains a character other than '+', '0', or '-'.
    /// Use [`Self::try_parse`] or [`FromStr`] for fallible parsing.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_ternary::Ternary;
    ///
    /// let value = Ternary::parse("00+-0");
    /// assert_eq!(value.to_string(), "00+-0");
    ///
    /// let empty = Ternary::parse("");
    /// assert!(empty.to_digit_slice().is_empty());
    /// assert_eq!(empty.to_string(), "");
    /// assert_eq!(empty.to_dec(), 0);
    /// ```
    pub fn parse(input: &str) -> Self {
        Self::try_parse(input).expect("invalid balanced ternary string")
    }

    /// Converts the `Ternary` object to its integer (decimal) representation.
    ///
    /// Calculates the sum of each digit's value multiplied by the appropriate power of 3.
    ///
    /// # Panics
    ///
    /// Panics if the numeric value cannot be represented
    /// by a signed 64-bit integer.
    ///
    /// Use `checked_to_dec()` for a non-panicking conversion.
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

    /// Attempts to convert this balanced ternary value
    /// into a signed 64-bit integer.
    ///
    /// Returns `Some(value)` if the complete numeric value
    /// fits within the `i64` range, or `None` otherwise.
    ///
    /// Intermediate calculations use checked `i128`
    /// arithmetic to avoid premature `i64` overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_ternary::Ternary;
    ///
    /// assert_eq!(
    ///     Ternary::from_dec(i64::MIN).checked_to_dec(),
    ///     Some(i64::MIN)
    /// );
    ///
    /// assert_eq!(
    ///     Ternary::parse(&"+".repeat(100)).checked_to_dec(),
    ///     None
    /// );
    /// ```
    pub fn checked_to_dec(&self) -> Option<i64> {
        let value = self.digits.iter().try_fold(0i128, |acc, digit| {
            acc.checked_mul(3)?.checked_add(digit.to_i8() as i128)
        });
        i64::try_from(value?).ok()
    }

    /// Converts this number to an arbitrary-precision unbalanced
    /// ternary string using the digits `0`, `1`, and `2`.
    ///
    /// Negative values have a leading `-`. Positive values have no sign.
    /// The output has no leading zeros, except that zero is written as `"0"`.
    /// Empty balanced representations also produce `"0"`.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_ternary::Ternary;
    ///
    /// assert_eq!(Ternary::from_dec(5).to_unbalanced(), "12");
    /// assert_eq!(Ternary::from_dec(-5).to_unbalanced(), "-12");
    /// assert_eq!(Ternary::parse("").to_unbalanced(), "0");
    /// ```
    pub fn to_unbalanced(&self) -> String {
        let negative = self.significant_digits().first() == Some(&Neg);
        let mut digits = Self::magnitude_base3(self);
        for digit in &mut digits {
            *digit += b'0';
        }
        if negative {
            digits.push(b'-');
        }

        digits.reverse();
        String::from_utf8(digits).expect("unbalanced ternary output contains only ASCII")
    }

    /// Parses an arbitrary-precision unbalanced ternary number.
    ///
    /// The input must contain an optional leading ASCII `+` or `-`,
    /// followed by at least one digit from `0`, `1`, and `2`.
    /// Whitespace and separators are not accepted.
    ///
    /// Leading zeros are accepted. The result is normalized:
    /// zero has one zero trit, and nonzero values have no leading zeros.
    ///
    /// # Errors
    ///
    /// Returns [`ParseUnbalancedTernaryError::MissingDigits`] for empty
    /// input or a sign without digits.
    ///
    /// Otherwise, reports the first invalid character and its UTF-8 byte
    /// offset in the original input.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_ternary::Ternary;
    ///
    /// let value = Ternary::try_from_unbalanced("+0012").unwrap();
    /// assert_eq!(value.to_string(), "+--");
    ///
    /// let zero = Ternary::try_from_unbalanced("-000").unwrap();
    /// assert_eq!(zero.to_string(), "0");
    /// ```
    pub fn try_from_unbalanced(unbalanced: &str) -> Result<Self, ParseUnbalancedTernaryError> {
        let (negative, digits, offset) = match unbalanced.as_bytes().first() {
            Some(b'-') => (true, &unbalanced[1..], 1),
            Some(b'+') => (false, &unbalanced[1..], 1),
            _ => (false, unbalanced, 0),
        };
        if digits.is_empty() {
            return Err(ParseUnbalancedTernaryError::MissingDigits);
        }
        for (index, character) in digits.char_indices() {
            if !matches!(character, '0' | '1' | '2') {
                return Err(ParseUnbalancedTernaryError::InvalidCharacter {
                    character,
                    index: offset + index,
                });
            }
        }

        let digits = digits.trim_start_matches('0');
        Ok(Self::ternary_from_base3(
            digits.bytes().rev().map(|digit| digit - b'0'),
            negative,
        ))
    }

    /// Parses an arbitrary-precision unbalanced ternary number.
    ///
    /// Uses the same syntax and normalization rules as
    /// [`Self::try_from_unbalanced`].
    ///
    /// # Panics
    ///
    /// Panics if the input is not a valid unbalanced ternary number.
    ///
    /// # Examples
    ///
    /// ```
    /// use balanced_ternary::Ternary;
    ///
    /// assert_eq!(Ternary::from_unbalanced("-12").to_dec(), -5);
    /// ```
    pub fn from_unbalanced(unbalanced: &str) -> Self {
        Self::try_from_unbalanced(unbalanced).expect("invalid unbalanced ternary string")
    }

    /// Removes leading zero digits.
    ///
    /// Returns a canonical representation containing
    /// at least one digit.
    ///
    /// An empty or all-zero representation becomes
    /// a single zero digit.
    ///
    /// This operation does not depend on integer
    /// conversion and supports arbitrarily large values.
    pub fn trim(&self) -> Self {
        let significant = self.significant_digits();

        if significant.is_empty() {
            Self::new(vec![Zero])
        } else {
            Self::new(significant.to_vec())
        }
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
        let current_length = self.digits.len();
        let mut digits = Vec::with_capacity(length.max(current_length));
        digits.resize(length.saturating_sub(current_length), Zero);
        digits.extend_from_slice(&self.digits);

        Self::new(digits)
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
        let mut result = String::with_capacity(self.digits.len());
        for digit in &self.digits {
            result.push(transform(digit));
        }
        result
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
        let mut digits = Vec::with_capacity(self.digits.len() + other.digits.len());
        digits.extend_from_slice(&self.digits);
        digits.extend_from_slice(&other.digits);

        Ternary::new(digits)
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
        for digit in &self.digits {
            f.write_char(digit.to_char())?;
        }
        Ok(())
    }
}

#[cfg(feature = "ternary-string")]
impl FromStr for Ternary {
    type Err = ParseTernaryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_parse(s)
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
pub use crate::tryte::{ParseTryteError, Tryte};

#[cfg(test)]
#[cfg(feature = "ternary-string")]
mod tests {
    use crate::concepts::DigitOperate;
    use crate::{Digit, Ternary};
    use Digit::{Pos, Zero};
    use alloc::string::ToString;
    use alloc::vec;
    use core::cmp::Ordering;

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

    #[test]
    fn test_each() {
        use crate::*;
        use alloc::string::ToString;
        let ternary = Ternary::parse("+0-");
        assert_eq!(ternary.each(Digit::possibly).to_string(), "++-");
    }

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

    #[test]
    fn test_ordering() {
        use crate::ter;

        assert!(ter("-+") < ter("0"));
        assert!(ter("0") < ter("++"));
    }

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

    #[test]
    fn test_iterators() {
        use crate::*;

        let ternary = Ternary::parse("+0-");
        let expected = vec![Pos, Zero, Neg];
        assert_eq!(ternary.iter().cloned().collect::<Vec<_>>(), expected);
        let collected: Vec<Digit> = Ternary::parse("+0-").into_iter().collect();
        assert_eq!(collected, expected);
    }

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

        // Long representations containing only zeros.
        assert_eq!(Ternary::parse(&"0".repeat(150)).checked_to_dec(), Some(0));

        // Values exceeding i128 must not panic.
        assert_eq!(Ternary::parse(&"+".repeat(150)).checked_to_dec(), None);
        assert_eq!(Ternary::parse(&"-".repeat(150)).checked_to_dec(), None);

        // Leading zeros must not change a valid value.
        let min = Ternary::from_dec(i64::MIN);
        let mut padded = vec![Zero; 150];
        padded.extend_from_slice(min.to_digit_slice());

        assert_eq!(Ternary::new(padded).checked_to_dec(), Some(i64::MIN));
    }

    #[test]
    fn numeric_equality() {
        assert_eq!(Ternary::parse("+"), Ternary::parse("000+"));
        assert_eq!(Ternary::parse("-"), Ternary::parse("00-"));
        assert_eq!(Ternary::parse("0"), Ternary::parse("000"));
        assert_eq!(Ternary::parse("0"), Ternary::new(vec![]));

        assert_ne!(Ternary::parse("+"), Ternary::parse("-"));
    }

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

    #[test]
    fn arbitrary_precision_comparison() {
        let positive = Ternary::parse(&"+".repeat(256));
        let larger = Ternary::parse(&"+".repeat(257));

        assert!(positive < larger);

        let negative = Ternary::parse(&"-".repeat(256));
        let smaller = Ternary::parse(&"-".repeat(257));

        assert!(smaller < negative);
    }

    fn add_digits(a: Digit, b: Digit, carry: Digit) -> (Digit, Digit) {
        let sum = a.to_i8() + b.to_i8() + carry.to_i8();
        let next = (sum + 1).div_euclid(3);

        (Digit::from_i8(next), Digit::from_i8(sum - next * 3))
    }

    #[test]
    fn ternary_each_preserves_length() {
        let a = Ternary::parse("00+-");

        assert_eq!(a.each(|d| d).to_string(), "00+-");

        assert_eq!(a.each_with(|d, _| d, Pos).to_string(), "00+-");
    }

    #[test]
    fn ternary_zip_preserves_argument_order() {
        let a = Ternary::parse("+");
        let b = Ternary::parse("--");

        assert_eq!(a.each_zip(|left, _| left, b.clone()).to_string(), "0+");

        assert_eq!(a.each_zip(|_, right| right, b.clone()).to_string(), "--");

        assert_eq!(
            a.each_zip_carry(|left, _, _| (Zero, left), b,).to_string(),
            "0+"
        );
    }

    #[test]
    fn ternary_carry_is_preserved() {
        let a = Ternary::parse("++");
        let b = Ternary::parse("+");

        assert_eq!(a.each_zip_carry(add_digits, b).to_string(), "+--");

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

        assert_eq!(a.each_zip_carry(add_digits, b), Tryte::<2>::MIN);

        assert_eq!(Tryte::<0>::ZERO.digit(0), None);
    }

    #[cfg(feature = "ternary-store")]
    #[test]
    fn ter40_carry_wraps_at_fixed_width() {
        use crate::Ter40;

        let result = Ter40::MAX.each_zip_carry(add_digits, Ter40::from_dec(1));

        assert_eq!(result, Ter40::MIN);
    }

    #[test]
    fn test_i64_conversion_boundaries() {
        let values = [
            i64::MIN,
            i64::MIN + 1,
            -243,
            -1,
            0,
            1,
            243,
            i64::MAX - 1,
            i64::MAX,
        ];

        for value in values {
            let ternary = Ternary::from_dec(value);

            assert_eq!(
                ternary.checked_to_dec(),
                Some(value),
                "Conversion failed for {value}"
            );

            assert_eq!(ternary.to_dec(), value);
        }
    }

    #[test]
    fn test_i64_conversion_roundtrip() {
        for value in -10_000i64..=10_000 {
            let ternary = Ternary::from_dec(value);
            assert_eq!(ternary.checked_to_dec(), Some(value));
        }

        let mut seed = 0x9E3779B97F4A7C15u64;

        for _ in 0..10_000 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);

            let value = seed as i64;
            let ternary = Ternary::from_dec(value);

            assert_eq!(ternary.checked_to_dec(), Some(value));
        }
    }

    #[test]
    fn test_ternary_trim() {
        let cases = [
            ("", "0"),
            ("0", "0"),
            ("000000", "0"),
            ("+", "+"),
            ("-", "-"),
            ("000+", "+"),
            ("000-", "-"),
            ("000+-0", "+-0"),
            ("000-0+", "-0+"),
        ];

        for (input, expected) in cases {
            let value = Ternary::parse(input);
            let trimmed = value.trim();

            assert_eq!(
                trimmed.to_string(),
                expected,
                "Trimming failed for {input:?}"
            );

            // Check structural idempotence, not only
            // numeric equality.
            assert_eq!(trimmed.to_digit_slice(), trimmed.trim().to_digit_slice());
        }
    }

    #[test]
    fn test_ternary_trim_arbitrary_precision() {
        use crate::Digit::Neg;

        for width in [256, 1024] {
            for digit in [Pos, Neg] {
                // Prepend zeros to a large nonzero value.
                let mut digits = vec![Zero; 32];
                digits.extend(vec![digit; width]);

                let original = Ternary::new(digits.clone());
                let trimmed = original.trim();

                // The significant digits are preserved.
                assert_eq!(trimmed.to_digit_slice(), &digits[32..]);

                assert_eq!(trimmed.log(), width);

                // The original representation is unchanged.
                assert_eq!(original.to_digit_slice(), digits.as_slice());

                // Trimming twice produces the same digits.
                assert_eq!(trimmed.trim().to_digit_slice(), trimmed.to_digit_slice());
            }
        }
    }
    #[test]
    fn parsing_preserves_exact_representation() {
        use crate::Digit::{Neg, Pos, Zero};
        use alloc::string::String;

        let cases: &[(&str, &[Digit])] = &[
            ("", &[]),
            ("0", &[Zero]),
            ("000", &[Zero, Zero, Zero]),
            ("+", &[Pos]),
            ("-", &[Neg]),
            ("00+-0", &[Zero, Zero, Pos, Neg, Zero]),
        ];

        for &(input, expected_digits) in cases {
            let results = [
                Ternary::try_parse(input),
                input.parse::<Ternary>(),
                Ternary::try_from(input),
                Ternary::try_from(String::from(input)),
            ];

            for result in results {
                let value = result.unwrap();

                assert_eq!(value.to_digit_slice(), expected_digits);
                assert_eq!(value.to_string(), input);
            }

            assert_eq!(Ternary::parse(input).to_digit_slice(), expected_digits);
        }
    }

    #[test]
    fn parsing_reports_the_first_invalid_character() {
        use crate::ParseTernaryError;
        use alloc::string::String;

        let cases = [
            ("x", 'x', 0),
            ("+0x-", 'x', 2),
            ("+x?", 'x', 1),
            (" 0", ' ', 0),
            ("0 ", ' ', 1),
            ("0\n", '\n', 1),
            ("0\0", '\0', 1),
            ("0é", 'é', 1),
            ("0−", '−', 1),
            ("+🙂", '🙂', 1),
            ("12", '1', 0),
        ];

        for (input, character, index) in cases {
            let expected = ParseTernaryError::InvalidCharacter { character, index };

            let results = [
                Ternary::try_parse(input),
                input.parse::<Ternary>(),
                Ternary::try_from(input),
                Ternary::try_from(String::from(input)),
            ];

            for result in results {
                assert_eq!(result, Err(expected), "Input: {input:?}");
            }
        }
    }

    #[test]
    fn empty_parsing_preserves_zero_width() {
        let empty = Ternary::try_parse("").unwrap();
        let zero = Ternary::from_dec(0);

        // Numeric equality does not imply identical representations.
        assert_eq!(empty, zero);
        assert_ne!(empty.to_digit_slice(), zero.to_digit_slice());

        assert_eq!(empty.checked_to_dec(), Some(0));
        assert_eq!(empty.log(), 0);
        assert_eq!(empty.to_string(), "");
        assert_eq!(empty.trim().to_digit_slice(), &[Digit::Zero]);
    }

    #[test]
    fn parsing_supports_large_representations() {
        let input = alloc::format!("000{}", "+0-".repeat(1024));
        let value = Ternary::try_parse(&input).unwrap();

        assert_eq!(value.to_string(), input);
        assert_eq!(value.log(), input.len());
        assert_eq!(value.checked_to_dec(), None);
    }

    #[test]
    #[should_panic(expected = "invalid balanced ternary string")]
    fn infallible_parse_rejects_invalid_input() {
        let _ = Ternary::parse("+x");
    }

    mod unbalanced_tests {
        use crate::{ParseUnbalancedTernaryError, Ternary};
        use alloc::{
            format,
            string::{String, ToString},
            vec::Vec,
        };

        fn reference_unbalanced(value: i64) -> String {
            let mut magnitude = value.unsigned_abs();
            let mut bytes = Vec::new();

            loop {
                bytes.push(b'0' + (magnitude % 3) as u8);
                magnitude /= 3;

                if magnitude == 0 {
                    break;
                }
            }

            if value < 0 {
                bytes.push(b'-');
            }

            bytes.reverse();
            String::from_utf8(bytes).unwrap()
        }

        #[test]
        fn unbalanced_parsing_normalizes_numeric_input() {
            for (input, balanced, unbalanced) in [
                ("0", "0", "0"),
                ("000", "0", "0"),
                ("+000", "0", "0"),
                ("-000", "0", "0"),
                ("1", "+", "1"),
                ("2", "+-", "2"),
                ("-2", "-+", "-2"),
                ("12", "+--", "12"),
                ("+0012", "+--", "12"),
                ("-0012", "-++", "-12"),
                ("22", "+0-", "22"),
            ] {
                let value = Ternary::try_from_unbalanced(input).unwrap();

                assert_eq!(value.to_string(), balanced, "input: {input}");
                assert_eq!(value.to_unbalanced(), unbalanced, "input: {input}");
                assert_eq!(
                    Ternary::from_unbalanced(input).to_digit_slice(),
                    value.to_digit_slice(),
                    "input: {input}",
                );
            }
        }

        #[test]
        fn unbalanced_parsing_requires_digits() {
            for input in ["", "+", "-"] {
                assert_eq!(
                    Ternary::try_from_unbalanced(input),
                    Err(ParseUnbalancedTernaryError::MissingDigits),
                    "input: {input:?}",
                );
            }
        }

        #[test]
        fn unbalanced_parsing_reports_the_first_invalid_character() {
            for (input, character, index) in [
                ("3", '3', 0),
                (" 1", ' ', 0),
                ("1 ", ' ', 1),
                ("1\n", '\n', 1),
                ("1_0", '_', 1),
                ("++1", '+', 1),
                ("--1", '-', 1),
                ("+-1", '-', 1),
                ("1-0", '-', 1),
                ("0b10", 'b', 1),
                ("é12", 'é', 0),
                ("+12é", 'é', 3),
                ("-0🙂", '🙂', 2),
                ("１２", '１', 0),
            ] {
                assert_eq!(
                    Ternary::try_from_unbalanced(input),
                    Err(ParseUnbalancedTernaryError::InvalidCharacter { character, index }),
                    "input: {input:?}",
                );
            }
        }

        #[test]
        fn unbalanced_conversions_match_native_reference() {
            for value in (-10_000i64..=10_000).chain([i64::MIN, i64::MAX]) {
                let expected = reference_unbalanced(value);
                let ternary = Ternary::from_dec(value);

                assert_eq!(ternary.to_unbalanced(), expected, "value: {value}",);

                let parsed = Ternary::try_from_unbalanced(&expected).unwrap();

                assert_eq!(parsed.checked_to_dec(), Some(value));
                assert_eq!(parsed.to_digit_slice(), ternary.to_digit_slice());
            }
        }

        #[test]
        fn unbalanced_conversions_support_large_known_values() {
            for width in [1usize, 40, 41, 128, 4096] {
                let zeros = "0".repeat(width);
                let inner_zeros = "0".repeat(width - 1);
                let twos = "2".repeat(width);

                // Check 3^width and 3^width - 1, with both signs.
                for (unbalanced, balanced) in [
                    (format!("1{zeros}"), format!("+{zeros}")),
                    (format!("-1{zeros}"), format!("-{zeros}")),
                    (twos.clone(), format!("+{inner_zeros}-")),
                    (format!("-{twos}"), format!("-{inner_zeros}+")),
                ] {
                    let value = Ternary::try_from_unbalanced(&unbalanced).unwrap();

                    assert_eq!(value.to_string(), balanced);
                    assert_eq!(value.to_unbalanced(), unbalanced);

                    if width >= 40 {
                        assert_eq!(value.checked_to_dec(), None);
                    }
                }
            }
        }

        #[test]
        fn unbalanced_round_trips_normalize_balanced_representations() {
            for input in ["", "0", "000", "000+-0", "-++", "+0-+-0"] {
                let value = Ternary::parse(input);
                let restored = Ternary::try_from_unbalanced(&value.to_unbalanced()).unwrap();

                assert_eq!(
                    restored.to_digit_slice(),
                    value.trim().to_digit_slice(),
                    "input: {input:?}",
                );
            }

            let input = format!("000{}", "+0-".repeat(1024));
            let value = Ternary::parse(&input);
            let restored = Ternary::try_from_unbalanced(&value.to_unbalanced()).unwrap();

            assert_eq!(restored.to_digit_slice(), value.trim().to_digit_slice(),);
        }

        #[test]
        fn unbalanced_parsing_accepts_long_zero_padding() {
            let padding = "0".repeat(4096);

            let zero = Ternary::try_from_unbalanced(&format!("-{padding}")).unwrap();
            assert_eq!(zero.to_string(), "0");

            let value = Ternary::try_from_unbalanced(&format!("+{padding}12")).unwrap();
            assert_eq!(value.to_string(), "+--");
            assert_eq!(value.to_unbalanced(), "12");
        }

        #[test]
        #[should_panic(expected = "invalid unbalanced ternary string")]
        fn unbalanced_panicking_constructor_rejects_invalid_input() {
            let _ = Ternary::from_unbalanced("1023");
        }
    }

    mod r11_properties;
}
