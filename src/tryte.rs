use crate::concepts::DigitOperate;
use crate::{
    Digit,
    Digit::{Neg, Pos, Zero},
    ParseTernaryError, Ternary, TernaryConversionError,
};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::array;
use core::error::Error;
use core::fmt::{Display, Formatter, Write};
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg as StdNeg, Not, Sub};
use core::str::FromStr;

/// Error returned when parsing a fixed-width balanced ternary word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseTryteError {
    /// The input contains an invalid character.
    Parse(ParseTernaryError),
    /// The numeric value does not fit in the destination word.
    Conversion(TernaryConversionError),
}

impl Display for ParseTryteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Parse(error) => Display::fmt(error, f),
            Self::Conversion(error) => Display::fmt(error, f),
        }
    }
}

impl Error for ParseTryteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parse(error) => Some(error),
            Self::Conversion(error) => Some(error),
        }
    }
}

/// A fixed-width balanced ternary word containing exactly
/// `SIZE` trits.
///
/// Each trit is represented by a `Digit` with one of three
/// possible values: -1, 0, or +1.
///
/// Unlike `Ter40`, which is a bounded signed integer,
/// `Tryte<SIZE>` represents a fixed-width ternary word.
///
/// # Arithmetic
///
/// - Addition, subtraction, and multiplication wrap modulo 3^SIZE.
/// - Division (`/`) performs a signed integer division,
///   truncating toward zero.
/// - `checked_*` methods return `None` on overflow or
///   division by zero.
/// - `overflowing_*` methods return the wrapped result
///   and an overflow flag.
/// - `saturating_*` methods clamp arithmetic results
///   to the representable range.
/// - `checked_modular_inverse()` and
///   `checked_modular_div()` provide modular arithmetic.
///
/// Division is not modular by default. Modular division
/// requires an invertible divisor.
///
/// # Width
///
/// SIZE is fixed at compile time and has no numeric upper limit.
/// Practical widths are constrained by memory and Rust type-layout limits.
///
/// Arithmetic operates directly on digit arrays without heap allocation.
/// Temporary storage grows linearly with SIZE.
///
/// The type is Copy, but copying a word has a cost proportional to SIZE.
///
/// The default width is 6 trits, as in the Setun computer.
///
/// The signed numeric range for SIZE > 0 is:
///
/// -(3^SIZE - 1) / 2 ..= (3^SIZE - 1) / 2
///
/// Examples:
///
/// - 6 trits: -364 ..= 364
/// - 40 trits: -6_078_832_729_528_464_400
///           ..= 6_078_832_729_528_464_400
///
/// `Tryte<0>` is an empty word whose only numeric value is zero.
/// Numeric conversions accept zero, including empty and all-zero
/// ternary representations, and reject every nonzero value.
///
/// # Conversions
///
/// Numeric conversions check significant trits rather than raw length.
/// Leading zeros are accepted, and successful conversions preserve
/// exactly SIZE trits.
///
/// Constructors do not wrap out-of-range values, even though
/// addition, subtraction, and multiplication use wrapping arithmetic.
///
/// Use `TryFrom` for detailed conversion errors or `checked_from_*`
/// for an Option result. Parsing through `FromStr` does not allocate.
///
/// Conversion into i64 is fallible and depends on the numeric value,
/// not the word width. Use `checked_to_i64()` or `i64::try_from()`.
///
/// Every i64 value can be represented using 41 or more balanced trits.
///
/// # Examples
///
/// ```
/// use balanced_ternary::Tryte;
///
/// type Word = Tryte<2>;
///
/// let one = Word::from_i64(1);
/// let two = Word::from_i64(2);
///
/// // Wrapping addition.
/// assert_eq!(Word::MAX + one, Word::MIN);
///
/// // Checked addition.
/// assert_eq!(Word::MAX.checked_add(one), None);
///
/// // Ordinary integer division.
/// assert_eq!(one / two, Word::ZERO);
///
/// // Modular division modulo 9.
/// assert_eq!(
///     one.checked_modular_div(two),
///     Some(Word::from_i64(-4))
/// );
/// ```
#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub struct Tryte<const SIZE: usize = 6> {
    /// The raw representation of the `Tryte` as SIZE ternary digits.
    raw: [Digit; SIZE],
}

impl<const SIZE: usize> Tryte<SIZE> {
    /// `++...++`
    pub const MAX: Self = Self::new([Pos; SIZE]);
    /// `--...--`
    pub const MIN: Self = Self::new([Neg; SIZE]);
    /// `00...00`
    pub const ZERO: Self = Self::new([Zero; SIZE]);

    /// Creates a word containing exactly SIZE balanced ternary digits.
    ///
    /// Storage requirements grow linearly with SIZE.
    pub const fn new(digits: [Digit; SIZE]) -> Self {
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
    /// A slice referencing the SIZE-digit array of the `Tryte`.
    ///
    /// This function allows access to the raw representation of the
    /// balanced ternary number as a slice of `Digit` values.
    pub const fn to_digit_slice(&self) -> &[Digit] {
        &self.raw
    }

    /// Converts a numeric value into a fixed-width word.
    ///
    /// Leading zeros are ignored when checking representability.
    /// Empty and all-zero representations are accepted, including for SIZE = 0.
    ///
    /// # Panics
    ///
    /// Panics if the numeric value does not fit.
    pub fn from_ternary(value: &Ternary) -> Self {
        Self::try_from(value).expect("Value does not fit in Tryte")
    }

    /// Attempts to convert a numeric value into a fixed-width word.
    ///
    /// Leading zeros do not affect representability.
    /// Empty and all-zero representations are accepted, including for SIZE = 0.
    ///
    /// Returns None if the value does not fit.
    /// No heap allocation is performed.
    pub fn checked_from_ternary(value: &Ternary) -> Option<Self> {
        Self::try_from(value).ok()
    }

    /// Attempts to convert the numeric value into an i64.
    ///
    /// Leading zeros do not affect representability.
    /// An empty word converts to zero.
    ///
    /// No heap allocation is performed.
    pub fn checked_to_i64(&self) -> Option<i64> {
        let value = self.raw.iter().try_fold(0i128, |acc, digit| {
            acc.checked_mul(3)?.checked_add(digit.to_i8() as i128)
        })?;

        i64::try_from(value).ok()
    }

    /// Converts the numeric value into an i64.
    ///
    /// # Panics
    ///
    /// Panics if the value is outside the i64 range.
    /// Use [`Self::checked_to_i64`] for a fallible conversion.
    pub fn to_i64(&self) -> i64 {
        self.checked_to_i64()
            .expect("Tryte value does not fit in i64")
    }

    /// Converts an i64 into a fixed-width word without wrapping.
    ///
    /// SIZE = 0 accepts only zero.
    ///
    /// # Panics
    ///
    /// Panics if the numeric value does not fit.
    pub fn from_i64(value: i64) -> Self {
        Self::try_from(value).expect("Value does not fit in Tryte")
    }

    /// Attempts to convert an i64 into a fixed-width word.
    ///
    /// SIZE = 0 accepts only zero.
    ///
    /// No heap allocation is performed.
    pub fn checked_from_i64(value: i64) -> Option<Self> {
        Self::try_from(value).ok()
    }

    /// Adds two fixed-width ternary words and returns
    /// the wrapped result and the outgoing carry.
    ///
    /// The carry is Neg, Zero or Pos.
    /// The result always contains exactly SIZE trits.
    pub fn add_with_carry(self, rhs: Self) -> (Self, Digit) {
        self.each_zip_carry_full(
            |a, b, carry| {
                let sum = a.to_i8() + b.to_i8() + carry.to_i8();
                let next_carry = (sum + 1).div_euclid(3);
                (
                    Digit::from_i8(next_carry),
                    Digit::from_i8(sum - next_carry * 3),
                )
            },
            rhs,
        )
    }

    /// Adds two words modulo 3^SIZE.
    pub fn wrapping_add(self, rhs: Self) -> Self {
        self.add_with_carry(rhs).0
    }

    /// Adds two words and reports a numeric overflow.
    ///
    /// The boolean indicates whether the exact sum
    /// exceeds the signed SIZE-trit range.
    pub fn overflowing_add(self, rhs: Self) -> (Self, bool) {
        let (result, carry) = self.add_with_carry(rhs);
        (result, carry != Zero)
    }

    /// Returns None if the exact sum exceeds
    /// the representable SIZE-trit range.
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        let (result, carry) = self.add_with_carry(rhs);
        if carry == Zero { Some(result) } else { None }
    }

    /// Adds two words, clamping the result
    /// to MIN or MAX on overflow.
    pub fn saturating_add(self, rhs: Self) -> Self {
        let (result, carry) = self.add_with_carry(rhs);
        match carry {
            Neg => Self::MIN,
            Zero => result,
            Pos => Self::MAX,
        }
    }

    /// Subtracts two fixed-width ternary words.
    ///
    /// Returns the wrapped result and the outgoing
    /// carry, which can be Neg, Zero or Pos.
    pub fn sub_with_carry(self, rhs: Self) -> (Self, Digit) {
        self.add_with_carry(-rhs)
    }

    /// Subtracts modulo 3^SIZE.
    pub fn wrapping_sub(self, rhs: Self) -> Self {
        self.sub_with_carry(rhs).0
    }

    /// Subtracts and reports signed ternary overflow.
    pub fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
        let (result, carry) = self.sub_with_carry(rhs);
        (result, carry != Zero)
    }

    /// Returns None if the exact difference is
    /// outside the signed SIZE-trit range.
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        let (result, carry) = self.sub_with_carry(rhs);
        if carry == Zero { Some(result) } else { None }
    }

    /// Subtracts, clamping to MIN or MAX on overflow.
    pub fn saturating_sub(self, rhs: Self) -> Self {
        let (result, carry) = self.sub_with_carry(rhs);
        match carry {
            Neg => Self::MIN,
            Zero => result,
            Pos => Self::MAX,
        }
    }

    /// Returns -1, 0, or 1 according to the numeric value.
    fn numeric_sign(&self) -> i8 {
        self.raw
            .iter()
            .find(|digit| **digit != Zero)
            .map_or(0, Digit::to_i8)
    }

    /// Multiplies into one or two SIZE-trit parts.
    ///
    /// Both parts and their digits are stored least-significant first.
    /// PARTS = 1 retains only the wrapped product.
    /// PARTS = 2 retains the complete exact product.
    fn multiply_digits<const PARTS: usize>(&self, rhs: &Self) -> [[Digit; SIZE]; PARTS] {
        debug_assert!(PARTS == 1 || PARTS == 2);

        let mut product = [[Zero; SIZE]; PARTS];
        if SIZE == 0 {
            return product;
        }

        for (j, b) in rhs.raw.iter().rev().enumerate() {
            if *b == Zero {
                continue;
            }

            let mut carry = 0i8;
            for (i, a) in self.raw.iter().rev().enumerate() {
                let index = i + j;
                let part = index / SIZE;
                if part >= PARTS {
                    break;
                }

                let digit = &mut product[part][index % SIZE];
                let sum = digit.to_i8() + a.to_i8() * b.to_i8() + carry;
                carry = (sum + 1).div_euclid(3);
                *digit = Digit::from_i8(sum - 3 * carry);
            }

            let mut index = SIZE + j;
            while carry != 0 && index / SIZE < PARTS {
                let digit = &mut product[index / SIZE][index % SIZE];
                let sum = digit.to_i8() + carry;
                carry = (sum + 1).div_euclid(3);
                *digit = Digit::from_i8(sum - 3 * carry);
                index += 1;
            }
            if PARTS == 2 {
                debug_assert_eq!(carry, 0);
            }
        }

        product
    }

    /// Converts the absolute value into ordinary base-3 digits.
    ///
    /// Digits are stored most-significant first.
    /// The magnitude always fits in SIZE ordinary ternary digits.
    fn magnitude_base3(&self) -> [u8; SIZE] {
        let sign = self.numeric_sign();
        let mut digits = [0u8; SIZE];
        let mut carry = 0i8;
        for i in (0..SIZE).rev() {
            let value = self.raw[i].to_i8() * sign + carry;
            digits[i] = value.rem_euclid(3) as u8;
            carry = value.div_euclid(3);
        }
        debug_assert_eq!(carry, 0);
        digits
    }

    /// Converts an ordinary base-3 magnitude into a signed word.
    ///
    /// The magnitude must fit in the signed SIZE-trit range.
    fn from_magnitude_base3(magnitude: &[u8; SIZE], negative: bool) -> Self {
        let mut digits = [Zero; SIZE];
        let mut carry = 0u8;
        for i in (0..SIZE).rev() {
            let value = magnitude[i] + carry;
            let digit = if value <= 1 {
                carry = 0;
                Digit::from_i8(value as i8)
            } else {
                carry = 1;
                Digit::from_i8(value as i8 - 3)
            };
            digits[i] = if negative { -digit } else { digit };
        }

        debug_assert_eq!(carry, 0);
        Self::new(digits)
    }

    /// Multiplies two words modulo 3^SIZE.
    ///
    /// Only the low SIZE trits are computed.
    /// No heap allocation is performed.
    pub fn wrapping_mul(self, rhs: Self) -> Self {
        let [mut digits] = self.multiply_digits::<1>(&rhs);
        digits.reverse();
        Self::new(digits)
    }

    /// Multiplies and reports signed numeric overflow.
    ///
    /// Overflow is detected from the high trits of the complete product,
    /// after all carries and cancellations have been resolved.
    pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        let [mut low, high] = self.multiply_digits::<2>(&rhs);

        let overflow = high.iter().any(|digit| *digit != Zero);
        low.reverse();

        (Self::new(low), overflow)
    }

    /// Returns None if the exact product is outside the signed word range.
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        let (result, overflow) = self.overflowing_mul(rhs);

        if overflow { None } else { Some(result) }
    }

    /// Multiplies, clamping the exact result to MIN or MAX on overflow.
    pub fn saturating_mul(self, rhs: Self) -> Self {
        let (result, overflow) = self.overflowing_mul(rhs);

        if !overflow {
            result
        } else if self.numeric_sign() != rhs.numeric_sign() {
            Self::MIN
        } else {
            Self::MAX
        }
    }

    /// Divides two signed ternary values, truncating toward zero.
    ///
    /// Returns None if the divisor is zero.
    /// A nonzero divisor cannot cause numeric overflow because the
    /// signed balanced ternary range is symmetric.
    ///
    /// Uses long division in ordinary base three.
    /// No heap allocation is performed.
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        let divisor_sign = rhs.numeric_sign();
        if divisor_sign == 0 {
            return None;
        }

        let dividend_sign = self.numeric_sign();
        if dividend_sign == 0 {
            return Some(Self::ZERO);
        }

        let dividend = self.magnitude_base3();
        let divisor = rhs.magnitude_base3();
        if dividend < divisor {
            return Some(Self::ZERO);
        }

        let mut remainder = [0u8; SIZE];
        let mut quotient = [0u8; SIZE];
        for position in 0..SIZE {
            debug_assert_eq!(remainder[0], 0);
            remainder.copy_within(1.., 0);
            remainder[SIZE - 1] = dividend[position];
            while remainder >= divisor {
                let mut borrow = 0i8;
                for i in (0..SIZE).rev() {
                    let difference = remainder[i] as i8 - divisor[i] as i8 - borrow;
                    if difference < 0 {
                        remainder[i] = (difference + 3) as u8;
                        borrow = 1;
                    } else {
                        remainder[i] = difference as u8;
                        borrow = 0;
                    }
                }

                debug_assert_eq!(borrow, 0);
                quotient[position] += 1;
            }

            debug_assert!(quotient[position] <= 2);
        }

        Some(Self::from_magnitude_base3(
            &quotient,
            dividend_sign != divisor_sign,
        ))
    }

    /// Performs signed integer division.
    ///
    /// Panics on division by zero.
    /// A nonzero divisor cannot cause overflow
    /// because the balanced range is symmetric.
    ///
    /// This is not multiplicative inversion modulo 3^SIZE.
    pub fn wrapping_div(self, rhs: Self) -> Self {
        self.checked_div(rhs).expect("Tryte division by zero")
    }

    /// Divides and reports overflow.
    ///
    /// The overflow flag is always false for
    /// a nonzero divisor.
    ///
    /// Panics on division by zero.
    pub fn overflowing_div(self, rhs: Self) -> (Self, bool) {
        (self.wrapping_div(rhs), false)
    }

    /// Divides with saturating semantics.
    ///
    /// Saturation is never necessary for a nonzero
    /// divisor, as the quotient always fits.
    ///
    /// Panics on division by zero.
    pub fn saturating_div(self, rhs: Self) -> Self {
        self.wrapping_div(rhs)
    }

    /// Returns the multiplicative inverse modulo 3^SIZE.
    ///
    /// Returns None for SIZE = 0 or when the least-significant
    /// trit is zero.
    ///
    /// The inverse is constructed one trit at a time.
    /// No heap allocation is performed.
    pub fn checked_modular_inverse(self) -> Option<Self> {
        if SIZE == 0 {
            return None;
        }

        let unit = self.raw[SIZE - 1].to_i8();
        if unit == 0 {
            return None;
        }

        let mut inverse = Self::ZERO;
        let mut product = Self::ZERO;
        for shift in 0..SIZE {
            let position = SIZE - 1 - shift;
            let target = if shift == 0 { 1i8 } else { 0i8 };
            let digit = (target - product.raw[position].to_i8()) * unit;
            inverse.raw[position] = Digit::from_i8(digit);
            if digit == 0 {
                continue;
            }

            let mut carry = 0i8;
            for offset in 0..SIZE - shift {
                let index = SIZE - 1 - shift - offset;
                let sum = product.raw[index].to_i8()
                    + digit * self.raw[SIZE - 1 - offset].to_i8()
                    + carry;
                carry = (sum + 1).div_euclid(3);
                product.raw[index] = Digit::from_i8(sum - 3 * carry);
            }
        }
        Some(inverse)
    }

    /// Computes division modulo 3^SIZE by multiplying
    /// by the modular inverse of the divisor.
    ///
    /// Returns `None` if SIZE is zero or the divisor
    /// is not invertible modulo 3^SIZE.
    ///
    /// For SIZE > 0, the divisor is invertible if and
    /// only if its numeric value is not divisible by 3.
    ///
    /// This operation differs from ordinary integer division.
    /// It does not attempt to solve modular equations with
    /// non-invertible divisors.
    pub fn checked_modular_div(self, rhs: Self) -> Option<Self> {
        let inverse = rhs.checked_modular_inverse()?;
        Some(self.wrapping_mul(inverse))
    }

    /// Applies a digit-wise operation with carry.
    ///
    /// Returns both the fixed-width result and
    /// the outgoing carry from the most significant trit.
    ///
    /// The callback returns (next_carry, result_digit).
    pub fn each_zip_carry_full(
        &self,
        f: impl Fn(Digit, Digit, Digit) -> (Digit, Digit),
        other: Self,
    ) -> (Self, Digit) {
        let mut digits = [Zero; SIZE];
        let mut carry = Zero;

        for i in (0..SIZE).rev() {
            let (next_carry, digit) = f(self.raw[i], other.raw[i], carry);

            digits[i] = digit;
            carry = next_carry;
        }

        (Self::new(digits), carry)
    }
}

impl<const SIZE: usize> DigitOperate for Tryte<SIZE> {
    fn to_digits(&self) -> Vec<Digit> {
        self.to_digit_slice().to_vec()
    }

    /// Retrieves the digit at the specified index in the `Tryte`.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the digit to retrieve (0-based, right-to-left).
    ///
    /// # Returns
    ///
    /// The `Digit` at the specified index or None.
    fn digit(&self, index: usize) -> Option<Digit> {
        if index < SIZE {
            Some(self.raw[SIZE - 1 - index])
        } else {
            None
        }
    }

    /// See [Ternary::each].
    fn each(&self, f: impl Fn(Digit) -> Digit) -> Self {
        Self::new(array::from_fn(|i| f(self.raw[i])))
    }

    /// See [Ternary::each_with].
    fn each_with(&self, f: impl Fn(Digit, Digit) -> Digit, with: Digit) -> Self {
        Self::new(array::from_fn(|i| f(self.raw[i], with)))
    }

    /// See [Ternary::each_zip].
    fn each_zip(&self, f: impl Fn(Digit, Digit) -> Digit, other: Self) -> Self {
        Self::new(array::from_fn(|i| f(self.raw[i], other.raw[i])))
    }

    /// See [Ternary::each_zip_carry].
    fn each_zip_carry(
        &self,
        f: impl Fn(Digit, Digit, Digit) -> (Digit, Digit),
        other: Self,
    ) -> Self {
        self.each_zip_carry_full(f, other).0
    }
}

impl<const SIZE: usize> Display for Tryte<SIZE> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        for digit in &self.raw {
            f.write_char(digit.to_char())?;
        }

        Ok(())
    }
}

impl<const SIZE: usize> StdNeg for Tryte<SIZE> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(array::from_fn(|i| -self.raw[i]))
    }
}

impl<const SIZE: usize> Add for Tryte<SIZE> {
    type Output = Tryte<SIZE>;

    fn add(self, rhs: Self) -> Self::Output {
        self.wrapping_add(rhs)
    }
}

impl<const SIZE: usize> Sub for Tryte<SIZE> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.wrapping_sub(rhs)
    }
}

impl<const SIZE: usize> Mul for Tryte<SIZE> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.wrapping_mul(rhs)
    }
}

impl<const SIZE: usize> Div for Tryte<SIZE> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.wrapping_div(rhs)
    }
}

impl<const SIZE: usize> BitAnd for Tryte<SIZE> {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        self.each_zip(Digit::bitand, rhs)
    }
}

impl<const SIZE: usize> BitOr for Tryte<SIZE> {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        self.each_zip(Digit::bitor, rhs)
    }
}

impl<const SIZE: usize> BitXor for Tryte<SIZE> {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        self.each_zip(Digit::bitxor, rhs)
    }
}

impl<const SIZE: usize> Not for Tryte<SIZE> {
    type Output = Self;
    fn not(self) -> Self::Output {
        -self
    }
}

impl<const SIZE: usize> TryFrom<&Ternary> for Tryte<SIZE> {
    type Error = TernaryConversionError;

    fn try_from(value: &Ternary) -> Result<Self, Self::Error> {
        let significant = value.significant_digits();
        if significant.len() > SIZE {
            return Err(TernaryConversionError::OutOfRange);
        }
        let mut digits = [Zero; SIZE];
        digits[SIZE - significant.len()..].copy_from_slice(significant);
        Ok(Self { raw: digits })
    }
}

impl<const SIZE: usize> TryFrom<Ternary> for Tryte<SIZE> {
    type Error = TernaryConversionError;

    fn try_from(value: Ternary) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl<const SIZE: usize> TryFrom<i64> for Tryte<SIZE> {
    type Error = TernaryConversionError;

    fn try_from(mut value: i64) -> Result<Self, Self::Error> {
        let mut digits = [Zero; SIZE];
        for digit in digits.iter_mut().rev() {
            let remainder = value.rem_euclid(3);
            value = value.div_euclid(3);

            *digit = match remainder {
                0 => Zero,
                1 => Pos,
                2 => {
                    value += 1;
                    Neg
                }
                _ => unreachable!(),
            };
        }

        if value != 0 {
            return Err(TernaryConversionError::OutOfRange);
        }
        Ok(Self { raw: digits })
    }
}

impl<const SIZE: usize> From<Tryte<SIZE>> for Ternary {
    fn from(value: Tryte<SIZE>) -> Self {
        value.to_ternary()
    }
}

impl<const SIZE: usize> TryFrom<&str> for Tryte<SIZE> {
    type Error = ParseTryteError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<const SIZE: usize> TryFrom<String> for Tryte<SIZE> {
    type Error = ParseTryteError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

impl<const SIZE: usize> From<Tryte<SIZE>> for String {
    fn from(value: Tryte<SIZE>) -> Self {
        value.to_string()
    }
}

impl<const SIZE: usize> TryFrom<&Tryte<SIZE>> for i64 {
    type Error = TernaryConversionError;
    fn try_from(value: &Tryte<SIZE>) -> Result<Self, Self::Error> {
        value
            .checked_to_i64()
            .ok_or(TernaryConversionError::OutOfRange)
    }
}

impl<const SIZE: usize> TryFrom<Tryte<SIZE>> for i64 {
    type Error = TernaryConversionError;
    fn try_from(value: Tryte<SIZE>) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl<const SIZE: usize> FromStr for Tryte<SIZE> {
    type Err = ParseTryteError;

    /// Parses a numeric value into a fixed-width word.
    ///
    /// Leading zeros are ignored when checking representability.
    /// Empty input represents zero.
    ///
    /// Invalid characters take precedence over numeric overflow.
    ///
    /// No heap allocation is performed.
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut digits = [Zero; SIZE];
        let mut significant_len = 0usize;
        for (index, character) in input.char_indices() {
            let digit = Digit::try_from_char(character).map_err(|_| {
                ParseTryteError::Parse(ParseTernaryError::InvalidCharacter { character, index })
            })?;
            if significant_len != 0 || digit != Zero {
                if significant_len < SIZE {
                    digits[significant_len] = digit;
                }
                significant_len += 1;
            }
        }

        if significant_len > SIZE {
            return Err(ParseTryteError::Conversion(
                TernaryConversionError::OutOfRange,
            ));
        }

        let padding = SIZE - significant_len;
        digits.copy_within(..significant_len, padding);
        digits[..padding].fill(Zero);

        Ok(Self { raw: digits })
    }
}

#[cfg(test)]
mod tests {
    use crate::Tryte;
    use alloc::string::ToString;

    #[test]
    pub fn test_tryte() {
        let tryte = Tryte::<6>::from_i64(255);
        assert_eq!(tryte.to_i64(), 255);
        assert_eq!(tryte.to_string(), "+00++0");

        let tryte = Tryte::<6>::from_i64(16);
        assert_eq!(tryte.to_i64(), 16);
        assert_eq!(tryte.to_string(), "00+--+");

        assert_eq!(Tryte::<6>::MAX.to_string(), "++++++");
        assert_eq!(Tryte::<6>::MAX.to_i64(), 364);
        assert_eq!(Tryte::<6>::MIN.to_string(), "------");
        assert_eq!(Tryte::<6>::MIN.to_i64(), -364);
        assert_eq!(Tryte::<6>::ZERO.to_string(), "000000");
        assert_eq!(Tryte::<6>::ZERO.to_i64(), 0);
    }

    #[test]
    pub fn test_tryte_from_str() {
        use core::str::FromStr;

        let tryte = Tryte::<6>::from_str("+-0").unwrap();
        assert_eq!(tryte.to_string(), "000+-0");

        assert!(Tryte::<6>::from_str("+-x").is_err());
    }

    #[test]
    fn tryte_addition_exhaustive() {
        use crate::{Digit, Tryte};

        fn check<const N: usize>() {
            let modulus = 3_i64.pow(N as u32);
            let max = (modulus - 1) / 2;
            let min = -max;

            for a in min..=max {
                for b in min..=max {
                    let lhs = Tryte::<N>::from_i64(a);
                    let rhs = Tryte::<N>::from_i64(b);
                    let exact = a + b;
                    let expected = (exact + max).rem_euclid(modulus) - max;
                    let carry = if exact > max {
                        Digit::Pos
                    } else if exact < min {
                        Digit::Neg
                    } else {
                        Digit::Zero
                    };
                    assert_eq!((lhs + rhs).to_i64(), expected);
                    assert_eq!(lhs.wrapping_add(rhs).to_i64(), expected);

                    let (result, actual_carry) = lhs.add_with_carry(rhs);
                    assert_eq!(result.to_i64(), expected);
                    assert_eq!(actual_carry, carry);

                    let (overflowing_result, overflowed) = lhs.overflowing_add(rhs);
                    assert_eq!(overflowing_result.to_i64(), expected);
                    assert_eq!(overflowed, carry != Digit::Zero);
                    assert_eq!(
                        lhs.checked_add(rhs),
                        if carry == Digit::Zero {
                            Some(result)
                        } else {
                            None
                        }
                    );
                    assert_eq!(lhs.saturating_add(rhs).to_i64(), exact.clamp(min, max));
                }
            }
        }

        check::<1>();
        check::<2>();
        check::<3>();
    }

    #[test]
    fn tryte_40_addition_boundaries() {
        use crate::Tryte;
        type Tryte40 = Tryte<40>;

        let one = Tryte40::from_i64(1);
        let neg_one = Tryte40::from_i64(-1);

        assert_eq!(Tryte40::MAX + one, Tryte40::MIN);
        assert_eq!(Tryte40::MIN + neg_one, Tryte40::MAX);
        assert_eq!(Tryte40::MAX + Tryte40::MAX, neg_one);
        assert_eq!(Tryte40::MAX + Tryte40::MIN, Tryte40::ZERO);
        assert_eq!(Tryte40::MAX.checked_add(one), None);
        assert_eq!(Tryte40::MIN.checked_add(neg_one), None);
        assert_eq!(Tryte40::MAX.saturating_add(one), Tryte40::MAX);
        assert_eq!(Tryte40::MIN.saturating_add(neg_one), Tryte40::MIN);
    }

    #[test]
    fn tryte_zero_width_addition() {
        use crate::Tryte;
        let zero = Tryte::<0>::ZERO;
        assert_eq!(zero + zero, zero);
    }

    #[test]
    fn tryte_subtraction_exhaustive() {
        use crate::{Digit, Tryte};

        fn check<const N: usize>() {
            let modulus = 3_i64.pow(N as u32);
            let max = (modulus - 1) / 2;

            for a in -max..=max {
                for b in -max..=max {
                    let lhs = Tryte::<N>::from_i64(a);
                    let rhs = Tryte::<N>::from_i64(b);

                    let exact = a - b;
                    let wrapped = (exact + max).rem_euclid(modulus) - max;
                    let expected_carry = Digit::from_i8(((exact - wrapped) / modulus) as i8);
                    let (result, carry) = lhs.sub_with_carry(rhs);
                    assert_eq!(result.to_i64(), wrapped);
                    assert_eq!(carry, expected_carry);
                    assert_eq!((lhs - rhs).to_i64(), wrapped);
                    assert_eq!(lhs.wrapping_sub(rhs), result);

                    let (overflowing_result, overflowed) = lhs.overflowing_sub(rhs);
                    assert_eq!(overflowing_result.to_digit_slice(), result.to_digit_slice());
                    assert_eq!(overflowed, expected_carry != Digit::Zero);
                    assert_eq!(
                        lhs.checked_sub(rhs),
                        if expected_carry == Digit::Zero {
                            Some(result)
                        } else {
                            None
                        }
                    );
                    assert_eq!(lhs.saturating_sub(rhs).to_i64(), exact.clamp(-max, max));
                }
            }
        }

        check::<1>();
        check::<2>();
        check::<3>();
    }

    #[test]
    fn tryte_multiplication_exhaustive() {
        use crate::Tryte;

        fn check<const N: usize>() {
            let modulus = 3_i64.pow(N as u32);
            let max = (modulus - 1) / 2;

            for a in -max..=max {
                for b in -max..=max {
                    let lhs = Tryte::<N>::from_i64(a);
                    let rhs = Tryte::<N>::from_i64(b);
                    let exact = a * b;
                    let wrapped = (exact + max).rem_euclid(modulus) - max;
                    let overflow = exact < -max || exact > max;
                    assert_eq!((lhs * rhs).to_i64(), wrapped);
                    assert_eq!(lhs.wrapping_mul(rhs).to_i64(), wrapped);

                    let (result, actual_overflow) = lhs.overflowing_mul(rhs);
                    assert_eq!(result.to_i64(), wrapped);
                    assert_eq!(actual_overflow, overflow);
                    assert_eq!(
                        lhs.checked_mul(rhs),
                        if overflow { None } else { Some(result) }
                    );
                    assert_eq!(lhs.saturating_mul(rhs).to_i64(), exact.clamp(-max, max));
                }
            }
        }

        check::<1>();
        check::<2>();
        check::<3>();
        check::<4>();
    }

    #[test]
    fn tryte_division_exhaustive() {
        use crate::Tryte;

        fn check<const N: usize>() {
            let modulus = 3_i64.pow(N as u32);
            let max = (modulus - 1) / 2;

            for a in -max..=max {
                let lhs = Tryte::<N>::from_i64(a);

                assert_eq!(lhs.checked_div(Tryte::<N>::ZERO), None);

                for b in -max..=max {
                    if b == 0 {
                        continue;
                    }

                    let rhs = Tryte::<N>::from_i64(b);
                    let expected = a / b;

                    assert_eq!((lhs / rhs).to_i64(), expected);
                    assert_eq!(lhs.checked_div(rhs).unwrap().to_i64(), expected);
                    assert_eq!(lhs.wrapping_div(rhs).to_i64(), expected);
                    let (overflowing_result, overflowed) = lhs.overflowing_div(rhs);
                    assert_eq!(overflowing_result.to_i64(), expected);
                    assert!(!overflowed);
                    assert_eq!(lhs.saturating_div(rhs).to_i64(), expected);
                }
            }
        }

        check::<1>();
        check::<2>();
        check::<3>();
        check::<4>();
    }

    #[test]
    fn tryte_40_arithmetic_boundaries() {
        use crate::Tryte;

        type Word = Tryte<40>;

        let one = Word::from_i64(1);
        let two = Word::from_i64(2);
        let neg_one = Word::from_i64(-1);

        // Subtraction wraps at the 40-trit boundary.
        assert_eq!(Word::MIN - one, Word::MAX);
        assert_eq!(Word::MAX - neg_one, Word::MIN);
        assert_eq!(Word::MIN.checked_sub(one), None);
        assert_eq!(Word::MAX.checked_sub(neg_one), None);

        // Multiplication wraps modulo 3^40.
        assert_eq!(Word::MAX * two, neg_one);
        assert_eq!(Word::MIN * two, one);
        assert_eq!(Word::MAX.checked_mul(two), None);
        assert_eq!(Word::MIN.checked_mul(two), None);

        // Division uses signed integer semantics.
        assert_eq!(Word::MIN / neg_one, Word::MAX);
        assert_eq!(Word::MAX / neg_one, Word::MIN);
        assert_eq!(Word::MIN.checked_div(Word::ZERO), None);

        // Negation is always representable.
        assert_eq!(-Word::MIN, Word::MAX);
        assert_eq!(-Word::MAX, Word::MIN);
    }

    #[test]
    fn tryte_zero_width_operations() {
        use crate::Tryte;
        let zero = Tryte::<0>::ZERO;
        assert_eq!(zero - zero, zero);
        assert_eq!(zero * zero, zero);
        assert_eq!(zero.checked_div(zero), None);
    }

    #[test]
    #[should_panic(expected = "Tryte division by zero")]
    fn tryte_division_by_zero_panics() {
        use crate::Tryte;
        let _ = Tryte::<6>::MAX / Tryte::<6>::ZERO;
    }

    #[test]
    fn tryte_modular_division_exhaustive() {
        use crate::Tryte;

        fn check<const N: usize>() {
            let modulus = 3_i64.pow(N as u32);
            let max = (modulus - 1) / 2;
            let one = Tryte::<N>::from_i64(1);

            for b in -max..=max {
                let divisor = Tryte::<N>::from_i64(b);
                let inverse = divisor.checked_modular_inverse();

                // A divisor is invertible iff it is not divisible by 3.
                let invertible = b.rem_euclid(3) != 0;
                assert_eq!(inverse.is_some(), invertible);

                if let Some(inverse) = inverse {
                    assert_eq!(divisor * inverse, one);
                    for a in -max..=max {
                        let numerator = Tryte::<N>::from_i64(a);
                        let quotient = numerator
                            .checked_modular_div(divisor)
                            .expect("Divisor must be invertible");

                        // (a / b) * b == a (mod 3^N).
                        assert_eq!(quotient * divisor, numerator);
                        assert_eq!(quotient, numerator.wrapping_mul(inverse));
                    }
                } else {
                    assert_eq!(one.checked_modular_div(divisor), None);
                }
            }
        }

        check::<1>();
        check::<2>();
        check::<3>();
        check::<4>();
    }

    #[test]
    fn tryte_zero_width_modular_division() {
        use crate::Tryte;
        let zero = Tryte::<0>::ZERO;

        assert_eq!(zero.checked_modular_inverse(), None);
        assert_eq!(zero.checked_modular_div(zero), None);
        assert_eq!(zero.checked_div(zero), None);
        assert_eq!(zero + zero, zero);
        assert_eq!(zero - zero, zero);
        assert_eq!(zero * zero, zero);
    }

    #[test]
    fn tryte_direct_i64_conversion() {
        use crate::Tryte;

        fn check<const N: usize>(min: i64, max: i64) {
            for value in min..=max {
                let word = Tryte::<N>::from_i64(value);
                assert_eq!(word.to_i64(), value);
                let reference = Tryte::<N>::from_ternary(&crate::Ternary::from_dec(value));
                assert_eq!(word, reference);
            }
        }

        check::<1>(-1, 1);
        check::<3>(-13, 13);
        check::<6>(-364, 364);

        type Word = Tryte<40>;
        for value in [
            0,
            1,
            -1,
            42,
            -42,
            6_078_832_729_528_464_400,
            -6_078_832_729_528_464_400,
        ] {
            assert_eq!(Word::from_i64(value).to_i64(), value);
        }
        assert_eq!(Tryte::<0>::ZERO.to_i64(), 0);
    }

    #[test]
    #[should_panic(expected = "Value does not fit")]
    fn tryte_direct_conversion_overflow() {
        use crate::Tryte;
        let _ = Tryte::<6>::from_i64(365);
    }

    #[test]
    fn tryte_direct_bitwise_operations() {
        use crate::Tryte;

        for a in -13..=13 {
            for b in -13..=13 {
                let lhs = Tryte::<3>::from_i64(a);
                let rhs = Tryte::<3>::from_i64(b);
                let ta = lhs.to_ternary();
                let tb = rhs.to_ternary();
                let expected_and = Tryte::<3>::from_ternary(&(&ta & &tb));
                let expected_or = Tryte::<3>::from_ternary(&(&ta | &tb));
                let expected_xor = Tryte::<3>::from_ternary(&(&ta ^ &tb));
                assert_eq!(lhs & rhs, expected_and);
                assert_eq!(lhs | rhs, expected_or);
                assert_eq!(lhs ^ rhs, expected_xor);
            }
        }
    }

    #[test]
    fn tryte_direct_display() {
        use crate::Tryte;
        use alloc::string::ToString;

        assert_eq!(Tryte::<6>::ZERO.to_string(), "000000");
        assert_eq!(Tryte::<6>::MAX.to_string(), "++++++");
        assert_eq!(Tryte::<6>::MIN.to_string(), "------");
        assert_eq!(Tryte::<6>::from_i64(255).to_string(), "+00++0");
        assert_eq!(Tryte::<6>::from_i64(-1).to_string(), "00000-");
        assert_eq!(Tryte::<0>::ZERO.to_string(), "");
    }

    fn check_numeric_conversion<const N: usize>(value: i64) {
        use crate::{ParseTryteError, Ternary, TernaryConversionError};

        let expected = if N >= 41 {
            Ok(value)
        } else {
            let max = ((3i128.pow(N as u32) - 1) / 2) as i64;
            if (-max..=max).contains(&value) {
                Ok(value)
            } else {
                Err(TernaryConversionError::OutOfRange)
            }
        };

        let ternary = Ternary::from_dec(value).with_length(80);
        let text = ternary.to_string();

        assert_eq!(
            Tryte::<N>::try_from(value).map(|word| word.to_i64()),
            expected
        );
        assert_eq!(
            Tryte::<N>::try_from(&ternary).map(|word| word.to_i64()),
            expected
        );
        assert_eq!(
            Tryte::<N>::try_from(ternary.clone()).map(|word| word.to_i64()),
            expected
        );

        assert_eq!(
            Tryte::<N>::checked_from_i64(value).map(|word| word.to_i64()),
            expected.ok()
        );
        assert_eq!(
            Tryte::<N>::checked_from_ternary(&ternary).map(|word| word.to_i64()),
            expected.ok()
        );

        let parsed_expected = expected.map_err(ParseTryteError::Conversion);

        for result in [
            text.parse::<Tryte<N>>(),
            Tryte::<N>::try_from(text.as_str()),
            Tryte::<N>::try_from(text.clone()),
        ] {
            assert_eq!(
                result.as_ref().map(|word| word.to_i64()).map_err(|e| *e),
                parsed_expected
            );

            if let Ok(word) = result {
                assert_eq!(word.to_digit_slice().len(), N);
                assert_eq!(word.to_string().len(), N);
            }
        }
    }

    #[test]
    fn checked_conversions_small_widths_exhaustive() {
        for value in -365..=365 {
            check_numeric_conversion::<0>(value);
            check_numeric_conversion::<1>(value);
            check_numeric_conversion::<2>(value);
            check_numeric_conversion::<6>(value);
        }
    }

    #[test]
    fn checked_conversions_40_trit_boundaries() {
        let max = 6_078_832_729_528_464_400i64;

        for value in [i64::MIN, -max - 1, -max, -1, 0, 1, max, max + 1, i64::MAX] {
            check_numeric_conversion::<40>(value);
        }
    }

    #[test]
    fn empty_and_zero_width_conversions() {
        use crate::Ternary;

        for input in ["", "0", "000", "0000000000"] {
            let ternary = Ternary::parse(input);

            assert_eq!(Tryte::<0>::from_ternary(&ternary), Tryte::<0>::ZERO);
            assert_eq!(input.parse::<Tryte<0>>().unwrap().to_string(), "");
            assert_eq!(input.parse::<Tryte<6>>().unwrap().to_string(), "000000");
        }

        assert_eq!(Tryte::<0>::from_i64(0), Tryte::<0>::ZERO);
        assert_eq!(Tryte::<0>::ZERO.to_ternary().to_digit_slice(), &[]);
    }

    #[test]
    fn parsing_validates_characters_even_after_overflow() {
        use crate::{ParseTernaryError, ParseTryteError};

        for (input, character, index) in [
            ("x", 'x', 0),
            ("+-x", 'x', 2),
            ("000+🙂", '🙂', 4),
            ("+ ", ' ', 1),
            ("−", '−', 0),
        ] {
            assert_eq!(
                input.parse::<Tryte<1>>(),
                Err(ParseTryteError::Parse(
                    ParseTernaryError::InvalidCharacter { character, index }
                ))
            );
        }
    }

    #[test]
    fn long_padding_and_large_values() {
        use crate::{ParseTryteError, Ternary, TernaryConversionError};

        for suffix in ["+", "-"] {
            let input = alloc::format!("{}{suffix}", "0".repeat(4096));
            let value = Ternary::parse(&input);

            assert_eq!(Tryte::<1>::from_ternary(&value).to_string(), suffix);
            assert_eq!(input.parse::<Tryte<1>>().unwrap().to_string(), suffix);
        }

        for symbol in ["+", "-"] {
            let input = symbol.repeat(4096);
            let value = Ternary::parse(&input);

            assert_eq!(
                Tryte::<40>::try_from(&value),
                Err(TernaryConversionError::OutOfRange)
            );
            assert_eq!(
                input.parse::<Tryte<40>>(),
                Err(ParseTryteError::Conversion(
                    TernaryConversionError::OutOfRange
                ))
            );
        }
    }

    #[test]
    fn large_width_numeric_conversions() {
        for value in [i64::MIN, i64::MIN + 1, -1, 0, 1, i64::MAX - 1, i64::MAX] {
            check_numeric_conversion::<41>(value);
            check_numeric_conversion::<64>(value);
            check_numeric_conversion::<128>(value);

            let word = Tryte::<128>::from_i64(value);

            assert_eq!(word.checked_to_i64(), Some(value));
            assert_eq!(i64::try_from(&word), Ok(value));
            assert_eq!(i64::try_from(word), Ok(value));
        }

        assert_eq!(Tryte::<0>::ZERO.checked_to_i64(), Some(0));
        assert_eq!(Tryte::<1024>::ZERO.checked_to_i64(), Some(0));
    }

    #[test]
    fn large_width_i64_overflow() {
        use crate::{Ternary, TernaryConversionError};

        let one = Ternary::from_dec(1);
        let above_max = &Ternary::from_dec(i64::MAX) + &one;
        let below_min = &Ternary::from_dec(i64::MIN) - &one;

        for value in [above_max, below_min] {
            let word = Tryte::<128>::try_from(&value).unwrap();

            assert_eq!(word.checked_to_i64(), None);
            assert_eq!(
                i64::try_from(&word),
                Err(TernaryConversionError::OutOfRange)
            );
            assert_eq!(i64::try_from(word), Err(TernaryConversionError::OutOfRange));
        }

        assert_eq!(Tryte::<1024>::MAX.checked_to_i64(), None);
        assert_eq!(Tryte::<1024>::MIN.checked_to_i64(), None);
    }

    #[test]
    #[should_panic(expected = "Tryte value does not fit in i64")]
    fn large_word_to_i64_panics_on_overflow() {
        let _ = Tryte::<128>::MAX.to_i64();
    }

    #[test]
    fn large_width_parsing() {
        use crate::{Digit, ParseTernaryError, ParseTryteError};

        let input = "+0-".repeat(128);
        let word = input.parse::<Tryte<384>>().unwrap();

        assert_eq!(word.to_string(), input);
        assert_eq!(word.to_digit_slice().len(), 384);

        let zero = "".parse::<Tryte<128>>().unwrap();
        assert_eq!(zero.to_digit_slice(), &[Digit::Zero; 128]);

        assert_eq!(
            "x".parse::<Tryte<128>>(),
            Err(ParseTryteError::Parse(
                ParseTernaryError::InvalidCharacter {
                    character: 'x',
                    index: 0,
                }
            ))
        );
    }

    fn check_wide_arithmetic<const N: usize>() {
        use crate::{Digit, Ternary};
        use Digit::{Neg, Pos, Zero};

        fn next_digit(seed: &mut u64) -> Digit {
            *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);

            match (*seed >> 32) % 3 {
                0 => Neg,
                1 => Zero,
                _ => Pos,
            }
        }

        let zero = Ternary::from_dec(0);
        let one = Ternary::from_dec(1);
        let modulus = Ternary::parse(&alloc::format!("+{}", "0".repeat(N)));

        let mut seed = 0x9E3779B97F4A7C15u64;

        for case in 0..24 {
            let lhs = Tryte::<N>::new(core::array::from_fn(|_| next_digit(&mut seed)));

            let rhs = Tryte::<N>::new(core::array::from_fn(|index| {
                let digit = next_digit(&mut seed);

                // Alternate between full-width and shorter divisors.
                if case % 2 == 0 && index < N / 2 {
                    Zero
                } else {
                    digit
                }
            }));

            let a = lhs.to_ternary();
            let b = rhs.to_ternary();
            let exact_product = &a * &b;

            let mut low_digits = [Zero; N];
            for (slot, digit) in low_digits
                .iter_mut()
                .rev()
                .zip(exact_product.to_digit_slice().iter().rev())
            {
                *slot = *digit;
            }

            let wrapped = Tryte::<N>::new(low_digits);
            let checked = Tryte::<N>::checked_from_ternary(&exact_product);

            assert_eq!(lhs * rhs, wrapped);
            assert_eq!(lhs.wrapping_mul(rhs), wrapped);
            let (overflowing_result, overflowed) = lhs.overflowing_mul(rhs);
            assert_eq!(overflowing_result, wrapped);
            assert_eq!(overflowed, checked.is_none());
            assert_eq!(lhs.checked_mul(rhs), checked);

            let saturated = match checked {
                Some(value) => value,
                None if exact_product < zero => Tryte::<N>::MIN,
                None => Tryte::<N>::MAX,
            };
            assert_eq!(lhs.saturating_mul(rhs), saturated);

            if b == zero {
                assert_eq!(lhs.checked_div(rhs), None);
            } else {
                let expected = &a / &b;
                let quotient = lhs.checked_div(rhs).unwrap();

                assert_eq!(quotient.to_ternary(), expected);
                assert_eq!(lhs / rhs, quotient);
                assert_eq!(lhs.wrapping_div(rhs), quotient);
                let (overflowing_result, overflowed) = lhs.overflowing_div(rhs);
                assert_eq!(overflowing_result, quotient);
                assert!(!overflowed);
                assert_eq!(lhs.saturating_div(rhs), quotient);
            }

            let inverse = lhs.checked_modular_inverse();
            let invertible = lhs.to_digit_slice()[N - 1] != Zero;

            assert_eq!(inverse.is_some(), invertible);

            if let Some(inverse) = inverse {
                let product = &a * &inverse.to_ternary();
                let difference = &product - &one;

                // Verify the congruence using independent arithmetic.
                assert_eq!(&difference % &modulus, zero);

                let quotient = rhs.checked_modular_div(lhs).unwrap();
                let product = &quotient.to_ternary() * &a;
                let difference = &product - &b;

                assert_eq!(&difference % &modulus, zero);
            } else {
                assert_eq!(rhs.checked_modular_div(lhs), None);
            }
        }
    }

    #[test]
    fn wide_arithmetic_matches_arbitrary_precision() {
        check_wide_arithmetic::<41>();
        check_wide_arithmetic::<64>();
        check_wide_arithmetic::<128>();
        check_wide_arithmetic::<256>();
    }

    #[test]
    fn wide_arithmetic_boundaries() {
        type Word = Tryte<128>;

        let one = Word::from_i64(1);
        let two = Word::from_i64(2);
        let negative_one = Word::from_i64(-1);

        assert_eq!(Word::MAX + one, Word::MIN);
        assert_eq!(Word::MIN - one, Word::MAX);
        assert_eq!(Word::MAX.checked_add(one), None);
        assert_eq!(Word::MIN.checked_sub(one), None);

        assert_eq!(Word::MAX.wrapping_mul(two), negative_one);
        assert_eq!(Word::MIN.wrapping_mul(two), one);
        let (max_mul_result, max_mul_overflowed) = Word::MAX.overflowing_mul(two);
        assert_eq!(max_mul_result, negative_one);
        assert!(max_mul_overflowed);
        let (min_mul_result, min_mul_overflowed) = Word::MIN.overflowing_mul(two);
        assert_eq!(min_mul_result, one);
        assert!(min_mul_overflowed);

        assert_eq!(Word::MAX.saturating_mul(two), Word::MAX);
        assert_eq!(Word::MIN.saturating_mul(two), Word::MIN);
        assert_eq!(Word::MAX.checked_mul(one), Some(Word::MAX));
        assert_eq!(Word::MIN.checked_mul(negative_one), Some(Word::MAX));
        assert_eq!(Word::MAX * Word::ZERO, Word::ZERO);
        assert_eq!(Word::MIN / negative_one, Word::MAX);
        assert_eq!(Word::MAX / negative_one, Word::MIN);
        assert_eq!(one / two, Word::ZERO);
        assert_eq!(negative_one / two, Word::ZERO);
        assert_eq!(Word::MAX.checked_div(Word::ZERO), None);
        assert_eq!(one.checked_modular_inverse(), Some(one));
        assert_eq!(negative_one.checked_modular_inverse(), Some(negative_one));
        assert_eq!(Word::from_i64(3).checked_modular_inverse(), None);
        assert_eq!(two.checked_modular_inverse(), Some(-Word::MAX));
    }

    #[test]
    fn multiplication_can_overflow_to_zero() {
        use crate::Digit::{Pos, Zero};

        let mut digits = [Zero; 128];
        digits[63] = Pos; // 3^64.

        let power = Tryte::<128>::new(digits);

        let (result, overflowed) = power.overflowing_mul(power);
        assert_eq!(result.to_digit_slice(), Tryte::<128>::ZERO.to_digit_slice());
        assert!(overflowed);
        assert_eq!(power.checked_mul(power), None);
        assert_eq!(power.saturating_mul(power), Tryte::<128>::MAX);
        assert_eq!(power.saturating_mul(-power), Tryte::<128>::MIN);
    }
}
