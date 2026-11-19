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

/// A fixed-width word containing exactly `SIZE` balanced trits (six by default).
/// For positive widths its signed range is `−(3^SIZE − 1)/2 ..= (3^SIZE − 1)/2`.
/// `Tryte<0>` is the empty word and represents only zero.
///
/// # Arithmetic
///
/// Addition, subtraction, and multiplication wrap modulo `3^SIZE`. Use the
/// `checked_*`, `overflowing_*`, or `saturating_*` methods for explicit overflow
/// handling. The `/` operator truncates toward zero and panics on division by zero.
/// [`Self::checked_modular_div`] divides by an invertible word modulo `3^SIZE`.
///
/// # Representation and conversions
///
/// `Display` writes the full width. Numeric constructors accept redundant
/// leading zeros and check that the value fits. [`Self::checked_to_i64`]
/// checks the numeric value even for words wider than 40 trits.
///
/// Arithmetic uses digit arrays with scratch space proportional to `SIZE`.
/// The word is `Copy`. Its size grows linearly with its width.
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

    /// Copies all `SIZE` trits into a [`Ternary`], including leading zeros.
    pub fn to_ternary(&self) -> Ternary {
        Ternary::new(self.raw.to_vec())
    }

    /// Borrows the complete word in most-significant-first order.
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

    fn each(&self, f: impl Fn(Digit) -> Digit) -> Self {
        Self::new(array::from_fn(|i| f(self.raw[i])))
    }

    fn each_with(&self, f: impl Fn(Digit, Digit) -> Digit, with: Digit) -> Self {
        Self::new(array::from_fn(|i| f(self.raw[i], with)))
    }

    fn each_zip(&self, f: impl Fn(Digit, Digit) -> Digit, other: Self) -> Self {
        Self::new(array::from_fn(|i| f(self.raw[i], other.raw[i])))
    }

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
