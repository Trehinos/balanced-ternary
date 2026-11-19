use crate::concepts::DigitOperate;
use crate::{Digit, Ternary, TernaryConversionError};
use alloc::vec::Vec;
use core::fmt::{Display, Write};
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Sub};

/// A compact representation of five balanced ternary digits.
///
/// The numeric value is stored in an `i8` and must
/// be within the inclusive range -121..=121.
///
/// Five balanced trits represent 243 distinct values.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct TritsChunk(i8);

impl TritsChunk {
    pub const ZERO: Self = Self(0);
    pub const MIN: Self = Self(-121);
    pub const MAX: Self = Self(121);

    /// Attempts to construct a chunk from an i8.
    ///
    /// Returns None outside the inclusive range -121..=121.
    pub const fn checked_from_dec(value: i8) -> Option<Self> {
        if value < Self::MIN.0 || value > Self::MAX.0 {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Creates a `TritsChunk` from a given decimal value.
    ///
    /// # Arguments
    ///
    /// * `from` - An `i8` value representing the decimal value to be converted into a `TritsChunk`.
    ///
    /// # Panics
    ///
    /// This function panics if the input value is out of the valid range `-121..=121`.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::TritsChunk;
    ///
    /// let chunk = TritsChunk::from_dec(42);
    /// assert_eq!(chunk.to_dec(), 42);
    /// ```
    pub fn from_dec(from: i8) -> Self {
        Self::checked_from_dec(from).expect("TritsChunk value out of range")
    }

    /// Returns the signed value of the five-trit chunk.
    pub fn to_dec(&self) -> i8 {
        self.0
    }

    /// Converts the `TritsChunk` into its ternary representation.
    ///
    /// # Returns
    ///
    /// A `Ternary` type representing the ternary form of the `TritsChunk`.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::{TritsChunk, Ternary};
    ///
    /// let chunk = TritsChunk::from_dec(42);
    /// let ternary = chunk.to_ternary();
    /// assert_eq!(ternary.to_dec(), 42);
    /// ```
    pub fn to_ternary(&self) -> Ternary {
        Ternary::from_dec(self.0 as i64)
    }

    /// Converts the chunk into a fixed-length five-trit balanced ternary representation.
    pub fn to_fixed_ternary(&self) -> Ternary {
        Ternary::new(self.to_digits())
    }

    /// Returns exactly five digits, including leading zeros.
    pub fn to_digits(&self) -> Vec<Digit> {
        self.to_digit_array().to_vec()
    }

    /// Decodes the chunk into exactly five balanced
    /// ternary digits, including leading zeros.
    ///
    /// The most significant digit comes first.
    /// No heap allocation is performed.
    fn to_digit_array(self) -> [Digit; 5] {
        let mut value = self.0;
        let mut digits = [Digit::Zero; 5];
        for digit in digits.iter_mut().rev() {
            let remainder = value.rem_euclid(3);
            value = value.div_euclid(3);

            *digit = match remainder {
                0 => Digit::Zero,
                1 => Digit::Pos,
                2 => {
                    value += 1;
                    Digit::Neg
                }
                _ => unreachable!(),
            };
        }
        debug_assert_eq!(value, 0);
        digits
    }

    /// Converts a numeric value into a five-trit chunk.
    ///
    /// Leading zeros are ignored. Empty input represents zero.
    ///
    /// # Panics
    ///
    /// Panics if the numeric value is outside -121..=121.
    pub fn from_ternary(ternary: Ternary) -> Self {
        Self::checked_from_ternary(&ternary).expect("TritsChunk value out of range")
    }

    /// Attempts to convert a numeric value into a five-trit chunk.
    ///
    /// Leading zeros are ignored. Empty input represents zero.
    /// Returns None if more than five significant trits are required.
    ///
    /// No heap allocation is performed.
    pub fn checked_from_ternary(ternary: &Ternary) -> Option<Self> {
        let digits = ternary.significant_digits();
        if digits.len() > 5 {
            return None;
        }

        let value = digits
            .iter()
            .fold(0i8, |acc, digit| acc * 3 + digit.to_i8());
        Some(Self(value))
    }
}

impl TryFrom<i8> for TritsChunk {
    type Error = TernaryConversionError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Self::checked_from_dec(value).ok_or(TernaryConversionError::OutOfRange)
    }
}

impl TryFrom<&Ternary> for TritsChunk {
    type Error = TernaryConversionError;
    fn try_from(value: &Ternary) -> Result<Self, Self::Error> {
        Self::checked_from_ternary(value).ok_or(TernaryConversionError::OutOfRange)
    }
}

impl TryFrom<Ternary> for TritsChunk {
    type Error = TernaryConversionError;

    fn try_from(value: Ternary) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

/// Compact variable-length storage for balanced ternary digits,
/// organized into five-trit chunks.
///
/// This type provides storage and conversions, not arithmetic.
///
/// # Structural equality and hashing
///
/// `PartialEq`, `Eq`, and `Hash` operate on the complete stored
/// chunk sequence, including its length and leading zero chunks.
/// They do not normalize or compare numeric values.
///
/// Input digits are left-padded to a multiple of five during
/// encoding. Different inputs can therefore produce identical
/// storage when they differ only by padding within the first chunk.
///
/// ```
/// use balanced_ternary::{DataTernary, Ternary};
///
/// let short = DataTernary::from_ternary(Ternary::parse("+"));
/// let same = DataTernary::from_ternary(Ternary::parse("0000+"));
/// let wider = DataTernary::from_ternary(Ternary::parse("00000+"));
///
/// assert_eq!(short, same);
/// assert_ne!(short, wider);
/// assert_eq!(short.to_ternary(), wider.to_ternary());
///
/// assert_ne!(DataTernary::default(), DataTernary::from_dec(0));
/// ```
///
/// `HashMap` and `HashSet` distinguish different stored chunk
/// sequences even when their numeric values are equal.
///
/// # No ordering
///
/// `PartialOrd` and `Ord` are intentionally not implemented.
/// Direct comparisons with `<` or `>`, sorting with `sort()`,
/// and insertion into `BTreeMap` or `BTreeSet` are unsupported.
///
/// Convert to `Ternary` when numeric comparison is required.
///
/// ```compile_fail
/// use balanced_ternary::DataTernary;
///
/// let _ = DataTernary::default() < DataTernary::from_dec(0);
/// ```
///
/// ```compile_fail
/// use balanced_ternary::DataTernary;
///
/// fn requires_ord<T: Ord>() {}
/// requires_ord::<DataTernary>();
/// ```
///
/// # Empty storage and zero
///
/// `Default` creates empty storage. Its numeric value is zero,
/// but its stored width is zero:
///
/// - `Display` produces an empty string.
/// - `to_fixed_ternary()` produces an empty representation.
/// - `to_ternary()` and `to_digits()` produce one zero trit.
/// - `checked_to_dec()` returns `Some(0)`.
///
/// In contrast, `from_dec(0)` stores one zero chunk.
///
/// Numeric conversions normalize representations. To preserve the
/// stored chunk sequence through a Ternary round trip, use
/// `to_fixed_ternary()` followed by `from_ternary()`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct DataTernary {
    pub(crate) chunks: Vec<TritsChunk>,
}

impl DataTernary {
    /// Encodes a balanced ternary representation into
    /// a sequence of five-trit chunks.
    ///
    /// Leading zeros are added when necessary to make
    /// the total representation length a multiple of five.
    ///
    /// The original digit sequence is otherwise preserved.
    ///
    /// Chunks are encoded directly in balanced base 243,
    /// without creating intermediate Ternary objects.
    ///
    /// Empty input produces empty storage.
    /// Nonempty all-zero input retains its padded storage width.
    ///
    /// Time complexity: O(n), where n is the number of trits.
    pub fn from_ternary(ternary: Ternary) -> Self {
        let digits = ternary.to_digit_slice();
        let padding = (5 - digits.len() % 5) % 5;
        let chunk_count = digits.len() / 5 + usize::from(digits.len() % 5 != 0);
        let mut chunks = Vec::with_capacity(chunk_count);
        let mut accumulator = 0i16;
        let mut count = padding;
        for digit in digits {
            accumulator = accumulator * 3 + digit.to_i8() as i16;
            count += 1;

            if count == 5 {
                debug_assert!((-121..=121).contains(&accumulator));
                chunks.push(TritsChunk::from_dec(accumulator as i8));
                accumulator = 0;
                count = 0;
            }
        }

        debug_assert_eq!(count, 0);
        Self { chunks }
    }

    /// Converts the stored value into a normalized balanced ternary representation.
    ///
    /// Empty and all-zero storage produce a single zero trit.
    /// Use [`Self::to_fixed_ternary`] to preserve the stored width.
    pub fn to_ternary(&self) -> Ternary {
        Ternary::new(self.decode_digits(true))
    }

    /// Decodes the stored chunks into a fixed-length
    /// balanced ternary representation.
    ///
    /// Each chunk contributes exactly five digits,
    /// including leading zeros.
    ///
    /// Empty storage produces an empty representation.
    pub fn to_fixed_ternary(&self) -> Ternary {
        Ternary::new(self.decode_digits(false))
    }

    /// Returns the normalized balanced ternary digit sequence.
    ///
    /// Chunks are decoded directly, without constructing
    /// an intermediate Ternary.
    ///
    /// Leading zeros are removed. Empty storage and
    /// all-zero representations produce a single Zero digit.
    pub fn to_digits(&self) -> Vec<Digit> {
        self.decode_digits(true)
    }

    /// Creates a DataTernary from a signed 64-bit integer.
    ///
    /// The value is converted directly into balanced
    /// base-243 chunks, without constructing a Ternary.
    ///
    /// The resulting representation uses the minimum
    /// number of chunks required to store the value.
    ///
    /// Zero is represented by a single zero chunk.
    ///
    /// ```
    /// use balanced_ternary::{DataTernary};
    ///
    /// let data_ternary = DataTernary::from_dec(42);
    /// assert_eq!(data_ternary.to_dec(), 42);
    /// ```
    pub fn from_dec(mut value: i64) -> Self {
        let mut chunks = Vec::new();
        loop {
            let remainder = value.rem_euclid(243);
            value = value.div_euclid(243);

            let chunk = if remainder <= 121 {
                remainder as i8
            } else {
                value += 1;
                (remainder - 243) as i8
            };
            chunks.push(TritsChunk::from_dec(chunk));
            if value == 0 {
                break;
            }
        }

        chunks.reverse();
        Self { chunks }
    }

    /// Attempts to convert the stored numeric value into an i64.
    ///
    /// Evaluates the chunks directly in balanced base 243.
    /// Leading zero chunks do not affect representability.
    /// Empty storage converts to zero.
    ///
    /// Returns None if the value is outside the i64 range.
    /// Intermediate arithmetic is checked to handle arbitrarily long input.
    ///
    /// No heap allocation is performed.
    pub fn checked_to_dec(&self) -> Option<i64> {
        let value = self.chunks.iter().try_fold(0i128, |acc, chunk| {
            acc.checked_mul(243)?.checked_add(chunk.to_dec() as i128)
        })?;

        i64::try_from(value).ok()
    }

    /// Converts the stored chunks into a signed 64-bit integer.
    ///
    /// Uses direct balanced base-243 evaluation without
    /// constructing an intermediate Ternary.
    ///
    /// Empty storage converts to zero.
    /// Use [`Self::checked_to_dec`] for a fallible conversion.
    ///
    /// # Panics
    ///
    /// Panics if the numeric value cannot be represented
    /// by an i64.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::{DataTernary};
    ///
    /// let data_ternary = DataTernary::from_dec(42);
    /// let decimal = data_ternary.to_dec();
    /// assert_eq!(decimal, 42);
    /// ```
    pub fn to_dec(&self) -> i64 {
        self.checked_to_dec()
            .expect("Ternary value does not fit in i64")
    }

    /// Decodes the stored chunks into balanced ternary digits.
    ///
    /// When `trim` is true, leading zeros are removed,
    /// and an empty or all-zero value becomes a single zero trit.
    ///
    /// When `trim` is false, the result contains
    /// exactly five digits per chunk.
    fn decode_digits(&self, trim: bool) -> Vec<Digit> {
        let mut digits = Vec::with_capacity(self.chunks.len() * 5);
        for chunk in &self.chunks {
            for digit in chunk.to_digit_array() {
                if trim && digits.is_empty() && digit == Digit::Zero {
                    continue;
                }
                digits.push(digit);
            }
        }

        if trim && digits.is_empty() {
            digits.push(Digit::Zero);
        }
        digits
    }
}

impl Display for DataTernary {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for chunk in &self.chunks {
            for digit in chunk.to_digit_array() {
                f.write_char(digit.to_char())?;
            }
        }
        Ok(())
    }
}

impl From<Ternary> for DataTernary {
    fn from(value: Ternary) -> Self {
        Self::from_ternary(value)
    }
}

impl From<DataTernary> for Ternary {
    fn from(value: DataTernary) -> Self {
        value.to_ternary()
    }
}

impl TryFrom<&DataTernary> for i64 {
    type Error = TernaryConversionError;
    fn try_from(value: &DataTernary) -> Result<Self, Self::Error> {
        value
            .checked_to_dec()
            .ok_or(TernaryConversionError::OutOfRange)
    }
}

impl TryFrom<DataTernary> for i64 {
    type Error = TernaryConversionError;
    fn try_from(value: DataTernary) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

/// A signed, bounded 40-trit balanced ternary integer.
///
/// Values are stored as an `i64`, with the representable range:
///
/// - MIN: -6_078_832_729_528_464_400
/// - MAX:  6_078_832_729_528_464_400
///
/// Unlike `Tryte<40>`, `Ter40` uses checked numeric arithmetic.
/// Arithmetic operators panic when their result exceeds the
/// representable 40-trit range.
///
/// The `checked_*` arithmetic methods return `None` instead.
///
/// Construction from `i64` or `Ternary` validates the numeric value.
/// Leading-zero padding in a `Ternary` representation is accepted.
///
/// `Display` writes all 40 trits. For the outgoing digit of a carry
/// operation, see [`Self::each_zip_carry_full`].
///
/// # Examples
///
/// ```
/// use balanced_ternary::Ter40;
///
/// let a = Ter40::from_dec(42);
/// let b = Ter40::from_dec(10);
///
/// assert_eq!((a + b).to_dec(), 52);
/// assert_eq!(
///     Ter40::MAX.checked_add(Ter40::from_dec(1)),
///     None
/// );
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Ter40(i64);

impl Ter40 {
    pub const MIN: Self = Self(-6_078_832_729_528_464_400);
    pub const MAX: Self = Self(6_078_832_729_528_464_400);
    pub const ZERO: Self = Self(0);

    /// Attempts to construct a Ter40 from an i64.
    ///
    /// Returns None if the value exceeds the
    /// representable 40-trit signed range.
    pub const fn checked_from_dec(value: i64) -> Option<Self> {
        if value < Self::MIN.0 || value > Self::MAX.0 {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Constructs a Ter40 from an i64.
    ///
    /// Panics if the value is out of range.
    pub fn from_dec(value: i64) -> Self {
        Self::checked_from_dec(value).expect("Ter40 value out of range")
    }

    pub const fn to_dec(&self) -> i64 {
        self.0
    }

    /// Attempts to convert a Ternary into a Ter40.
    ///
    /// Leading zeros are accepted, as only the
    /// numeric value determines representability.
    pub fn checked_from_ternary(ternary: &Ternary) -> Option<Self> {
        Self::checked_from_dec(ternary.checked_to_dec()?)
    }

    /// Constructs a Ter40 from a Ternary.
    ///
    /// Panics if the numeric value is out of range.
    pub fn from_ternary(ternary: Ternary) -> Self {
        Self::checked_from_ternary(&ternary).expect("Ter40 value out of range")
    }

    /// Converts an i128 intermediate into Ter40.
    fn checked_from_i128(value: i128) -> Option<Self> {
        if value < Self::MIN.0 as i128 || value > Self::MAX.0 as i128 {
            None
        } else {
            Some(Self(value as i64))
        }
    }

    /// Converts the value into a balanced ternary
    /// representation containing exactly 40 trits,
    /// including leading zeros.
    pub fn to_ternary(&self) -> Ternary {
        Ternary::new(self.to_digit_array().to_vec())
    }

    fn pop_trit(value: &mut i64) -> Digit {
        let remainder = value.rem_euclid(3);
        *value = value.div_euclid(3);

        match remainder {
            0 => Digit::Zero,
            1 => Digit::Pos,
            2 => {
                *value += 1;
                Digit::Neg
            }
            _ => unreachable!(),
        }
    }

    /// Decodes the value into exactly 40 balanced trits.
    ///
    /// Digits are ordered from most significant to
    /// least significant, including leading zeros.
    ///
    /// This method performs no heap allocation.
    fn to_digit_array(self) -> [Digit; 40] {
        let mut value = self.0;
        let mut digits = [Digit::Zero; 40];

        for digit in digits.iter_mut().rev() {
            *digit = Self::pop_trit(&mut value);
        }

        debug_assert_eq!(value, 0);

        digits
    }

    fn map_trits(
        &self,
        other: Self,
        f: impl Fn(Digit, Digit, Digit) -> (Digit, Digit),
    ) -> (Self, Digit) {
        assert!(
            self.0 >= Self::MIN.0
                && self.0 <= Self::MAX.0
                && other.0 >= Self::MIN.0
                && other.0 <= Self::MAX.0,
            "Ter40 value out of range"
        );

        let mut lhs = self.0;
        let mut rhs = other.0;
        let mut carry = Digit::Zero;
        let mut result = 0i64;
        let mut power = 1i64;
        for index in 0..40 {
            let a = Self::pop_trit(&mut lhs);
            let b = Self::pop_trit(&mut rhs);
            let (next_carry, digit) = f(a, b, carry);

            result += digit.to_i8() as i64 * power;
            carry = next_carry;
            if index < 39 {
                power *= 3;
            }
        }

        (Self(result), carry)
    }
    /// Adds two values without exceeding the 40-trit range.
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        Self::checked_from_i128(self.0 as i128 + rhs.0 as i128)
    }

    /// Subtracts two values without exceeding the 40-trit range.
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        Self::checked_from_i128(self.0 as i128 - rhs.0 as i128)
    }

    /// Multiplies two values without exceeding the 40-trit range.
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        Self::checked_from_i128(self.0 as i128 * rhs.0 as i128)
    }

    /// Divides two values, truncating toward zero.
    ///
    /// Returns None for division by zero.
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        if rhs.0 == 0 {
            return None;
        }

        Self::checked_from_i128(self.0 as i128 / rhs.0 as i128)
    }

    /// Applies a digit-wise operation to exactly 40 trits.
    ///
    /// Returns the resulting value and the outgoing carry.
    pub fn each_zip_carry_full(
        &self,
        f: impl Fn(Digit, Digit, Digit) -> (Digit, Digit),
        other: Self,
    ) -> (Self, Digit) {
        self.map_trits(other, f)
    }
}

impl DigitOperate for Ter40 {
    fn to_digits(&self) -> Vec<Digit> {
        self.to_digit_array().to_vec()
    }

    fn digit(&self, index: usize) -> Option<Digit> {
        if index >= 40 {
            return None;
        }

        let mut value = self.0;
        for _ in 0..index {
            Self::pop_trit(&mut value);
        }
        Some(Self::pop_trit(&mut value))
    }

    fn each(&self, f: impl Fn(Digit) -> Digit) -> Self
    where
        Self: Sized,
    {
        self.map_trits(Self(0), |a, _, _| (Digit::Zero, f(a))).0
    }

    fn each_with(&self, f: impl Fn(Digit, Digit) -> Digit, other: Digit) -> Self
    where
        Self: Sized,
    {
        self.map_trits(Self(0), |a, _, _| (Digit::Zero, f(a, other)))
            .0
    }

    fn each_zip(&self, f: impl Fn(Digit, Digit) -> Digit, other: Self) -> Self
    where
        Self: Sized,
    {
        self.map_trits(other, |a, b, _| (Digit::Zero, f(a, b))).0
    }

    fn each_zip_carry(&self, f: impl Fn(Digit, Digit, Digit) -> (Digit, Digit), other: Self) -> Self
    where
        Self: Sized,
    {
        self.each_zip_carry_full(f, other).0
    }
}

impl Display for Ter40 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for digit in self.to_digit_array() {
            f.write_char(digit.to_char())?;
        }
        Ok(())
    }
}

impl Add for Ter40 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs).expect("Ter40 addition overflow")
    }
}

impl Sub for Ter40 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs).expect("Ter40 subtraction overflow")
    }
}

impl Mul for Ter40 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.checked_mul(rhs)
            .expect("Ter40 multiplication overflow")
    }
}

impl Div for Ter40 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.checked_div(rhs).expect("Ter40 division by zero")
    }
}

impl Neg for Ter40 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        // The signed ternary range is symmetric.
        Self(-self.0)
    }
}

impl BitAnd for Ter40 {
    type Output = Self;
    fn bitand(self, other: Self) -> Self::Output {
        self.each_zip(Digit::bitand, other)
    }
}

impl BitOr for Ter40 {
    type Output = Self;
    fn bitor(self, other: Self) -> Self::Output {
        self.each_zip(Digit::bitor, other)
    }
}

impl BitXor for Ter40 {
    type Output = Self;
    fn bitxor(self, other: Self) -> Self::Output {
        self.each_zip(Digit::bitxor, other)
    }
}

impl TryFrom<i64> for Ter40 {
    type Error = TernaryConversionError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Self::checked_from_dec(value).ok_or(TernaryConversionError::OutOfRange)
    }
}

impl TryFrom<&Ternary> for Ter40 {
    type Error = TernaryConversionError;
    fn try_from(value: &Ternary) -> Result<Self, Self::Error> {
        Self::checked_from_ternary(value).ok_or(TernaryConversionError::OutOfRange)
    }
}

impl TryFrom<Ternary> for Ter40 {
    type Error = TernaryConversionError;
    fn try_from(value: Ternary) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl From<Ter40> for i64 {
    fn from(value: Ter40) -> Self {
        value.0
    }
}

impl From<Ter40> for Ternary {
    fn from(value: Ter40) -> Self {
        value.to_ternary()
    }
}
