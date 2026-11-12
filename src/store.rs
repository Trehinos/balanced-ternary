use crate::concepts::DigitOperate;
use crate::{Digit, Ternary};
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
        if !(-121..=121).contains(&from) {
            panic!("TritsChunk::from_dec(): Invalid value: {}", from);
        }
        Self(from)
    }

    /// Converts the `TritsChunk` into its decimal representation.
    ///
    /// # Returns
    ///
    /// An `i8` value representing the decimal form of the `TritsChunk`.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::TritsChunk;
    ///
    /// let chunk = TritsChunk::from_dec(42);
    /// assert_eq!(chunk.to_dec(), 42);
    /// ```
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

    /// Creates a `TritsChunk` from a given `Ternary` value.
    ///
    /// # Arguments
    ///
    /// * `ternary` - A `Ternary` value to be converted into a `TritsChunk`.
    ///
    /// # Panics
    ///
    /// This function panics if the provided `ternary` value has a logarithmic length greater than 5,
    /// indicating that it cannot be represented by a single `TritsChunk`.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::{TritsChunk, Ternary};
    ///
    /// let ternary = Ternary::from_dec(42);
    /// let chunk = TritsChunk::from_ternary(ternary);
    /// assert_eq!(chunk.to_dec(), 42);
    /// ```
    pub fn from_ternary(ternary: Ternary) -> Self {
        if ternary.log() > 5 {
            panic!(
                "TritsChunk::from_ternary(): Ternary is too long: {}",
                ternary
            );
        }
        Self(ternary.to_dec() as i8)
    }
}

/// A variable-length balanced ternary representation
/// stored as five-trit chunks.
///
/// Equality, hashing, and ordering currently operate
/// on the stored chunk sequence, not on the normalized
/// numeric value.
///
/// Consequently, representations with different leading
/// zero chunks can compare as unequal even when they
/// represent the same number.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DataTernary {
    chunks: Vec<TritsChunk>,
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


    /// Converts the stored chunks into a signed 64-bit integer.
    ///
    /// Uses direct balanced base-243 evaluation without
    /// constructing an intermediate Ternary.
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
        self.chunks
            .iter()
            .try_fold(0i128, |acc, chunk| {
                acc.checked_mul(243)?.checked_add(chunk.to_dec() as i128)
            })
            .and_then(|value| i64::try_from(value).ok())
            .expect("Ternary value does not fit in i64")
    }

    /// Decodes the stored chunks into balanced ternary digits.
    ///
    /// When `trim` is true, leading zeros are removed,
    /// and an empty or all-zero value becomes [Zero].
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
/// Digit-wise operations provided by `DigitOperate` operate on
/// exactly 40 trits and may discard the outgoing carry.
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

impl From<i64> for Ter40 {
    fn from(value: i64) -> Self {
        Self::from_dec(value)
    }
}

impl From<Ter40> for i64 {
    fn from(value: Ter40) -> Self {
        value.0
    }
}

impl From<Ternary> for Ter40 {
    fn from(value: Ternary) -> Self {
        Self::from_ternary(value)
    }
}

impl From<Ter40> for Ternary {
    fn from(value: Ter40) -> Self {
        value.to_ternary()
    }
}

#[cfg(test)]
mod tests {
    use crate::DataTernary;
    use alloc::string::ToString;
    use alloc::vec;

    #[test]
    fn single_chunk_creation() {
        use crate::Ternary;

        let ternary = Ternary::parse("+-0-+");
        let data = DataTernary::from_ternary(ternary.clone());

        assert_eq!(data.chunks.len(), 1);
        assert_eq!(data.to_ternary(), ternary);
    }

    #[test]
    fn round_trip() {
        use crate::Ternary;

        let ternary = Ternary::parse("+0-0++-");
        let data = DataTernary::from_ternary(ternary.clone());

        assert_eq!(data.to_ternary(), ternary);
    }

    #[test]
    fn data_ternary_round_trip_zero_chunks() {
        use crate::Ternary;

        let cases = [
            ("+", "0000+"),
            ("+00000", "0000+00000"),
            ("-00000", "0000-00000"),
            ("+0000000000", "0000+0000000000"),
            ("0000000000", "0000000000"),
        ];

        for (original, expected_fixed) in cases {
            let data = DataTernary::from_ternary(Ternary::parse(original));

            assert_eq!(data.to_fixed_ternary().to_string(), expected_fixed);

            assert_eq!(
                data.to_ternary().to_string(),
                Ternary::parse(original).trim().to_string()
            );
        }
    }

    #[test]
    fn ter40_checked_construction() {
        use crate::Ter40;

        for value in [Ter40::MIN.to_dec(), -1, 0, 1, Ter40::MAX.to_dec()] {
            assert_eq!(Ter40::checked_from_dec(value), Some(Ter40::from_dec(value)));

            assert_eq!(Ter40::from(value).to_dec(), value);
        }

        for value in [
            i64::MIN,
            Ter40::MIN.to_dec() - 1,
            Ter40::MAX.to_dec() + 1,
            i64::MAX,
        ] {
            assert_eq!(Ter40::checked_from_dec(value), None);
        }
    }

    #[test]
    fn ter40_checked_ternary_conversion() {
        use crate::{Digit, Ter40, Ternary};

        // More than 40 digits, but a representable value.
        let mut digits = vec![Digit::Zero; 100];
        digits.push(Digit::Pos);

        let padded = Ternary::new(digits);

        assert_eq!(
            Ter40::checked_from_ternary(&padded),
            Some(Ter40::from_dec(1))
        );

        // Values outside the signed 40-trit range.
        let positive = Ternary::parse(&"+".repeat(41));
        let negative = Ternary::parse(&"-".repeat(41));

        assert_eq!(Ter40::checked_from_ternary(&positive), None);
        assert_eq!(Ter40::checked_from_ternary(&negative), None);

        // Every valid boundary fits in exactly 40 trits.
        for value in [Ter40::MIN, Ter40::MAX] {
            let ternary = value.to_ternary();

            assert_eq!(ternary.log(), 40);
            assert_eq!(Ter40::checked_from_ternary(&ternary), Some(value));
        }
    }

    #[test]
    #[should_panic(expected = "Ter40 value out of range")]
    fn ter40_constructor_overflow() {
        use crate::Ter40;
        let _ = Ter40::from_dec(i64::MAX);
    }

    #[test]
    #[should_panic(expected = "Ter40 value out of range")]
    fn ter40_from_i64_overflow() {
        use crate::Ter40;
        let _: Ter40 = i64::MIN.into();
    }

    #[test]
    #[should_panic(expected = "Ter40 value out of range")]
    fn ter40_from_ternary_overflow() {
        use crate::{Ter40, Ternary};
        let _: Ter40 = Ternary::from_dec(i64::MAX).into();
    }

    #[test]
    #[should_panic(expected = "Ter40 addition overflow")]
    fn ter40_add_overflow() {
        use crate::Ter40;
        let _ = Ter40::MAX + Ter40::from_dec(1);
    }

    #[test]
    #[should_panic(expected = "Ter40 subtraction overflow")]
    fn ter40_sub_overflow() {
        use crate::Ter40;
        let _ = Ter40::MIN - Ter40::from_dec(1);
    }

    #[test]
    #[should_panic(expected = "Ter40 multiplication overflow")]
    fn ter40_mul_overflow() {
        use crate::Ter40;
        let _ = Ter40::MAX * Ter40::from_dec(2);
    }

    #[test]
    #[should_panic(expected = "Ter40 division by zero")]
    fn ter40_div_by_zero() {
        use crate::Ter40;
        let _ = Ter40::from_dec(1) / Ter40::ZERO;
    }

    #[test]
    fn ter40_direct_digit_access() {
        use crate::concepts::DigitOperate;
        use crate::{Digit, Ter40, Ternary};

        let values = [
            Ter40::MIN.to_dec(),
            -1_000_000,
            -1,
            0,
            1,
            1_000_000,
            Ter40::MAX.to_dec(),
        ];

        for value in values {
            let word = Ter40::from_dec(value);
            let reference = Ternary::from_dec(value).with_length(40);
            assert_eq!(word.to_digits(), reference.to_digit_slice());
            assert_eq!(
                word.to_ternary().to_digit_slice(),
                reference.to_digit_slice()
            );

            for index in 0..40 {
                assert_eq!(
                    word.digit(index),
                    reference.get_digit(index).copied(),
                    "Failed for value {value} at index {index}"
                );
            }
            assert_eq!(word.digit(40), None);
            assert_eq!(word.digit(100), None);
        }
        assert_eq!(Ter40::ZERO.digit(39), Some(Digit::Zero));
        assert_eq!(Ter40::MAX.digit(39), Some(Digit::Pos));
        assert_eq!(Ter40::MIN.digit(39), Some(Digit::Neg));
    }

    #[test]
    fn ter40_direct_display() {
        use crate::{Ter40, Ternary};
        use alloc::string::ToString;

        for value in [Ter40::MIN.to_dec(), -42, -1, 0, 1, 42, Ter40::MAX.to_dec()] {
            let word = Ter40::from_dec(value);
            let expected = Ternary::from_dec(value).with_length(40).to_string();
            assert_eq!(word.to_string(), expected);
            assert_eq!(word.to_string().len(), 40);
        }
        assert_eq!(Ter40::ZERO.to_string(), "0".repeat(40));
        assert_eq!(Ter40::MAX.to_string(), "+".repeat(40));
        assert_eq!(Ter40::MIN.to_string(), "-".repeat(40));
    }

    #[test]
    fn ter40_direct_conversion_exhaustive() {
        use crate::concepts::DigitOperate;
        use crate::{Ter40, Ternary};

        for value in -1000i64..=1000 {
            let word = Ter40::from_dec(value);
            let reference = Ternary::from_dec(value).with_length(40);
            assert_eq!(
                word.to_ternary().to_digit_slice(),
                reference.to_digit_slice()
            );
            assert_eq!(word.to_digits(), reference.to_digit_slice());
            assert_eq!(word.to_ternary().to_dec(), value);
        }
    }

    #[test]
    fn ternary_r07_regression() {
        use crate::{Digit, Ternary};
        use alloc::string::ToString;

        let value = Ternary::parse("000+-0");
        assert_eq!(value.to_string(), "000+-0");
        assert_eq!(value.get_digit(0), Some(&Digit::Zero));
        assert_eq!(value.get_digit(1), Some(&Digit::Neg));
        assert_eq!(value.get_digit(2), Some(&Digit::Pos));
        assert_eq!(value.get_digit(6), None);
        assert_eq!(value.get_digit(usize::MAX), None);
        assert_eq!(value.with_length(8).to_string(), "00000+-0");
        assert_eq!(value.with_length(2).to_string(), "000+-0");
        assert_eq!(value.concat(&Ternary::parse("-+")).to_string(), "000+-0-+");
        assert_eq!(value.to_string_repr(Digit::to_char_theta), "0001Θ0");

        let empty = Ternary::parse("");
        assert_eq!(empty.to_string(), "");
        assert_eq!(empty.with_length(0).to_string(), "");
        assert_eq!(empty.with_length(3).to_string(), "000");
    }

    #[test]
    fn trits_chunk_direct_decoding() {
        use crate::{Ternary, TritsChunk};

        for value in -121i8..=121 {
            let chunk = TritsChunk::from_dec(value);
            let expected = Ternary::from_dec(value as i64).with_length(5);
            assert_eq!(chunk.to_digits(), expected.to_digit_slice());
            assert_eq!(chunk.to_fixed_ternary(), expected);
        }
    }

    #[test]
    fn data_ternary_direct_encoding() {
        use crate::{DataTernary, Ternary};
        use alloc::string::ToString;

        let cases = [
            "",
            "0",
            "000",
            "+",
            "-",
            "+-0-+",
            "+00000",
            "-00000",
            "0000000000",
            "000+--",
            "+0-0++-",
        ];

        for input in cases {
            let original = Ternary::parse(input);
            let padding = (5 - input.len() % 5) % 5;
            let expected_fixed = original.with_length(input.len() + padding);
            let storage = DataTernary::from_ternary(original.clone());

            assert_eq!(
                storage.to_fixed_ternary(),
                expected_fixed,
                "Fixed representation failed for {input}"
            );
            assert_eq!(
                storage.to_ternary(),
                original.trim(),
                "Numeric representation failed for {input}"
            );
            assert_eq!(storage.to_digits(), original.trim().to_digit_slice());
            assert_eq!(storage.to_string(), expected_fixed.to_string());
        }
    }

    #[test]
    fn data_ternary_direct_numeric_conversion() {
        use crate::{DataTernary, Ternary};

        let boundaries = [
            i64::MIN,
            -1_000_000,
            -243,
            -122,
            -121,
            -1,
            0,
            1,
            121,
            122,
            243,
            1_000_000,
            i64::MAX,
        ];

        for value in boundaries {
            let optimized = DataTernary::from_dec(value);
            let reference = DataTernary::from_ternary(Ternary::from_dec(value));
            assert_eq!(optimized, reference);
            assert_eq!(optimized.to_dec(), value);
        }
        for value in -10_000i64..=10_000 {
            assert_eq!(DataTernary::from_dec(value).to_dec(), value);
        }
    }

    #[test]
    fn data_ternary_large_and_empty_values() {
        use crate::{DataTernary, Digit, Ternary};
        use alloc::string::ToString;

        let input = "+-0".repeat(100);
        let original = Ternary::parse(&input);
        let storage = DataTernary::from_ternary(original.clone());
        assert_eq!(storage.to_ternary(), original);
        assert_eq!(storage.to_string(), input);
        assert_eq!(storage.to_digits().len(), input.len());

        let empty = DataTernary::default();
        assert_eq!(empty.to_fixed_ternary().to_string(), "");
        assert_eq!(empty.to_ternary().to_string(), "0");
        assert_eq!(empty.to_digits(), vec![Digit::Zero]);
        assert_eq!(empty.to_string(), "");
        assert_eq!(empty.to_dec(), 0);
    }
}
