//! Balanced ternary arithmetic, compact storage, and three-valued logic.
//! A numeral uses powers of three with trits drawn from −1, 0, and +1.
//!
//! [`Digit`] and [`concepts::DigitOperate`] are always available.
//! The library uses `core` and `alloc`.
//!
//! # Cargo features
//!
//! - `ternary-string` exposes `Ternary`, an arbitrary-precision signed integer.
//! - `tryte` adds the fixed-width word `Tryte<N>`.
//! - `ternary-store` adds `TritsChunk`, `DataTernary`, and `Ter40`.
//!
//! The last two features are enabled by default and each includes `ternary-string`.

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

/// Returns the trit written as `+`, `0`, or `-`.
///
/// # Panics
///
/// Panics on an invalid character. Use [`Digit::try_from_char`] for fallible input.
///
/// ```
/// assert_eq!(balanced_ternary::trit('+'), balanced_ternary::Pos);
/// ```
pub const fn trit(from: char) -> Digit {
    Digit::from_char(from)
}

/// Shorthand for [`Ternary::parse`].
///
/// # Panics
///
/// Panics on an invalid balanced digit. Use [`Ternary::try_parse`] for fallible input.
///
/// ```
/// assert_eq!(balanced_ternary::ter("0-").to_string(), "0-");
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

/// Parses balanced digits into compact five-trit chunks.
///
/// # Panics
///
/// Panics on an invalid balanced digit.
///
/// ```
/// use balanced_ternary::dter;
///
/// assert_eq!(dter("+-0-").to_string(), "0+-0-");
/// ```
#[cfg(feature = "ternary-store")]
pub fn dter(from: &str) -> DataTernary {
    DataTernary::from_ternary(ter(from))
}

/// An arbitrary-precision signed integer stored as a sequence of balanced trits.
///
/// Balanced parsing and `Display` preserve the exact digits, including leading
/// zeros and empty input. An empty sequence has numeric value zero. Equality,
/// ordering, and hashing compare numeric values. Use [`Self::to_digit_slice`]
/// to compare representations and [`Self::trim`] to obtain canonical digits.
///
/// The binary arithmetic operators (`+`, `-`, `*`, `/`, `%`) return canonical
/// results. Unary negation changes each trit's sign and preserves the width.
/// See [`Self::div_rem`] for signed quotient and remainder semantics.
///
/// [`Self::try_from_unbalanced`] and [`Self::to_unbalanced`] convert ordinary
/// base-3 numerals at arbitrary precision. For a bounded `i64` conversion,
/// [`Self::checked_to_dec`] returns an `Option` and `i64::try_from` returns
/// a `Result`. [`Self::to_dec`] panics on overflow.
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

    /// Borrows the stored digits in most-significant-first order, including padding.
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

    /// Returns a copy padded on the left with zeros to at least `length` digits.
    /// If the representation is already long enough, all its digits are retained.
    ///
    /// ```
    /// use balanced_ternary::Ternary;
    ///
    /// let value = Ternary::parse("0+");
    /// assert_eq!(value.with_length(4).to_string(), "000+");
    /// assert_eq!(value.with_length(1).to_string(), "0+");
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

// Run the README's public examples with the features they use.
#[cfg(all(doctest, feature = "tryte", feature = "ternary-store"))]
#[doc = include_str!("../README.md")]
mod readme_examples {}

#[cfg(test)]
mod tests {
    mod digit_base;
    mod digit_operations;
    mod r11_properties;
    #[cfg(feature = "ternary-string")]
    mod ternary_base;
    #[cfg(feature = "ternary-string")]
    mod ternary_conversions;
    #[cfg(feature = "ternary-string")]
    mod ternary_operations;
    #[cfg(feature = "ternary-store")]
    mod ternary_store;
    #[cfg(feature = "tryte")]
    mod tryte;
}
