use crate::concepts::DigitOperate;
use crate::{Digit, Ternary};
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Display;
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

    /// Converts the `TritsChunk` into its fixed-length ternary representation.
    ///
    /// # Returns
    ///
    /// A `Ternary` type representing the 5-digit fixed-length ternary form of the `TritsChunk`.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::{TritsChunk, Ternary};
    ///
    /// let chunk = TritsChunk::from_dec(42);
    /// let fixed_ternary = chunk.to_fixed_ternary();
    /// assert_eq!(fixed_ternary.to_dec(), 42);
    /// assert_eq!(fixed_ternary.to_digit_slice().len(), 5);
    /// ```
    pub fn to_fixed_ternary(&self) -> Ternary {
        Ternary::from_dec(self.0 as i64).with_length(5)
    }

    /// Converts the `TritsChunk` into a vector of its individual ternary digits.
    ///
    /// # Returns
    ///
    /// A `Vec<Digit>` representing the individual ternary digits of the `TritsChunk`.
    ///
    /// The resulting vector will always contain 5 digits since the `TritsChunk` is
    /// represented in a fixed-length ternary form.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::{TritsChunk, Digit};
    ///
    /// let chunk = TritsChunk::from_dec(42);
    /// let digits: Vec<Digit> = chunk.to_digits();
    /// assert_eq!(digits.len(), 5);
    /// ```
    pub fn to_digits(&self) -> Vec<Digit> {
        self.to_fixed_ternary().to_digit_slice().to_vec()
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
    /// Creates a new instance of `DataTernary` from a given `Ternary` value.
    ///
    /// This method ensures that the total number of ternary digits is a multiple of 5
    /// by padding as necessary. It then divides the ternary number into chunks of
    /// 5 digits each, which are stored in the `DataTernary` structure.
    ///
    /// # Arguments
    ///
    /// * `ternary` - A `Ternary` value to be converted into a `DataTernary`.
    ///
    /// # Returns
    ///
    /// A new `DataTernary` instance containing the converted chunks.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::{DataTernary, Ternary};
    ///
    /// let ternary = Ternary::from_dec(42);
    /// let data_ternary = DataTernary::from_ternary(ternary);
    /// assert_eq!(data_ternary.to_dec(), 42);
    /// ```
    pub fn from_ternary(ternary: Ternary) -> Self {
        let len = ternary.log();
        let diff = (5 - len % 5) % 5;
        let ternary = ternary.with_length(len + diff);
        let mut chunks = Vec::new();
        for i in 0..(ternary.log() / 5) {
            let digits = ternary.to_digit_slice()[i * 5..(i + 1) * 5].to_vec();
            chunks.push(TritsChunk::from_ternary(Ternary::new(digits)));
        }
        Self { chunks }
    }

    /// Converts a `DataTernary` into its equivalent `Ternary` representation.
    ///
    /// This function iterates over all the `TritsChunk` instances in the `DataTernary`,
    /// extracts their ternary representations, and reconstructs them into the full
    /// `Ternary` value. The resulting `Ternary` value may be trimmed to remove
    /// any leading zeroes in its ternary digit representation.
    ///
    /// # Returns
    ///
    /// A `Ternary` value that represents the combined ternary digits of the
    /// `DataTernary`.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::{DataTernary, Ternary};
    ///
    /// let ternary = Ternary::from_dec(42);
    /// let data_ternary = DataTernary::from_ternary(ternary.clone());
    /// assert_eq!(data_ternary.to_ternary(), ternary);
    /// ```
    pub fn to_ternary(&self) -> Ternary {
        let fixed = self.to_fixed_ternary();
        let digits = fixed.to_digit_slice();

        match digits.iter().position(|d| *d != Digit::Zero) {
            Some(index) => Ternary::new(digits[index..].to_vec()),
            None => Ternary::new(vec![Digit::Zero]),
        }
    }

    /// Converts the `DataTernary` into its fixed-length `Ternary` representation.
    ///
    /// This method iterates over all the `TritsChunk` instances in the `DataTernary` and
    /// extracts and combines their ternary digits into a single `Ternary` value.
    /// The resulting `Ternary` value will contain a fixed number of digits without trimming
    /// or removing leading zeroes.
    ///
    /// # Returns
    ///
    /// A `Ternary` value representing the combined fixed-length ternary digits of the `DataTernary`.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::{DataTernary, Ternary};
    ///
    /// let ternary = Ternary::from_dec(42);
    /// let data_ternary = DataTernary::from_ternary(ternary);
    /// let fixed_ternary = data_ternary.to_fixed_ternary();
    /// assert_eq!(fixed_ternary.to_dec(), 42); // When properly encoded
    /// ```
    pub fn to_fixed_ternary(&self) -> Ternary {
        let mut digits = Vec::with_capacity(self.chunks.len() * 5);
        for chunk in &self.chunks {
            digits.extend(chunk.to_digits());
        }
        Ternary::new(digits)
    }

    /// Converts the `DataTernary` into a vector of ternary digits.
    ///
    /// This method first converts the `DataTernary` structure into its `Ternary` representation,
    /// trims any leading zeroes, and then returns the sequence of ternary digits as a `Vec<Digit>`.
    ///
    /// # Returns
    ///
    /// A `Vec<Digit>` containing the ternary digits that represent the `DataTernary` value.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::{DataTernary, Digit, Ternary};
    ///
    /// let ternary = Ternary::from_dec(42);
    /// let data_ternary = DataTernary::from_ternary(ternary);
    /// let digits = data_ternary.to_digits();
    /// assert_eq!(digits, vec![Digit::Pos, Digit::Neg, Digit::Neg, Digit::Neg, Digit::Zero]);
    /// ```
    pub fn to_digits(&self) -> Vec<Digit> {
        self.to_ternary().trim().to_digit_slice().to_vec()
    }

    /// Converts a decimal number into a `DataTernary` structure.
    ///
    /// This method takes a signed 64-bit integer as input and converts it into a
    /// `Ternary` representation, which is then stored in the compact `DataTernary`
    /// structure. The conversion ensures that the ternary representation uses
    /// fixed-length chunks for efficient storage.
    ///
    /// # Arguments
    ///
    /// * `from` - A signed 64-bit integer value to be converted into `DataTernary`.
    ///
    /// # Returns
    ///
    /// A `DataTernary` instance that represents the given decimal number.
    ///
    /// # Example
    ///
    /// ```
    /// use balanced_ternary::{DataTernary};
    ///
    /// let data_ternary = DataTernary::from_dec(42);
    /// assert_eq!(data_ternary.to_dec(), 42);
    /// ```
    pub fn from_dec(from: i64) -> Self {
        Self::from_ternary(Ternary::from_dec(from))
    }

    /// Converts a `DataTernary` into its decimal representation.
    ///
    /// This method reconstructs the ternary value represented by the `DataTernary`
    /// structure and converts it into the corresponding signed 64-bit decimal integer.
    ///
    /// # Returns
    ///
    /// A signed 64-bit integer (`i64`) representing the decimal equivalent of the
    /// `DataTernary` structure.
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
        self.to_ternary().to_dec()
    }
}

impl Display for DataTernary {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for chunk in &self.chunks {
            write!(f, "{}", chunk.to_fixed_ternary())?;
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

    /// Converts this value into a balanced ternary representation
    /// containing exactly 40 digits, including leading zeros.
    pub fn to_ternary(&self) -> Ternary {
        Ternary::from_dec(self.0).with_length(40)
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
}

impl DigitOperate for Ter40 {
    fn to_digits(&self) -> Vec<Digit> {
        self.to_ternary().to_digits()
    }

    fn digit(&self, index: usize) -> Option<Digit> {
        self.to_ternary().digit(index)
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
        self.map_trits(other, f).0
    }
}

impl Display for Ter40 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_ternary())
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
}
