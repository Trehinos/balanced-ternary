use crate::concepts::DigitOperate;
use crate::{
    Digit,
    Digit::{Neg, Pos, Zero},
    Ternary,
};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::array;
use core::fmt::{Display, Formatter};
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg as StdNeg, Not, Sub};
use core::str::FromStr;

/// A fixed-width balanced ternary word containing exactly
/// `SIZE` trits.
///
/// Each trit is represented by a `Digit` and can hold
/// one of three values: -1, 0, or +1.
///
/// Unlike `Ter40`, which is a bounded numeric type,
/// `Tryte<SIZE>` represents a fixed-width sequence of trits.
///
/// Digit-wise operations preserve the word width and
/// discard any outgoing carry.
///
/// The default width is 6 trits, representing values
/// from -364 to +364.
/// # Default SIZE
///
/// `SIZE` is 6 by default (the size of a tryte in a Setun computer).
///
/// > **6 trits ~= 9.505 bits**
///
/// > `-364` to `364`
///
/// > **40 trits ~= 63.398 bits**
/// >
/// > `-6 078 832 729 528 464 400` to `6 078 832 729 528 464 400`
///
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

    /// Creates a new `Tryte` instance from a given array of `Digit`s.
    ///
    /// # Arguments
    ///
    /// * `raw` - An array of exactly SIZE `Digit` values representing the balanced ternary digits.
    ///
    /// # Returns
    ///
    /// A new `Tryte` instance with the specified balanced ternary digits.
    ///
    /// # Panics
    ///
    /// Panic if `SIZE > 40` as 41 trits would be too much information for 64 bits.
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
    pub const fn new(digits: [Digit; SIZE]) -> Self {
        if SIZE > 40 {
            panic!("Cannot construct a Tryte with more than 40 digits (~63.5 bits).")
        }
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
    /// This function panics if the `Ternary` contains more than SIZE digits.
    pub fn from_ternary(v: &Ternary) -> Self {
        if v.log() > SIZE {
            panic!(
                "Cannot convert a Ternary with more than {} digits to a Tryte<{}>.",
                SIZE, SIZE
            );
        }
        let mut digits = [Zero; SIZE];
        for (i, d) in v.digits.iter().rev().enumerate() {
            digits[SIZE - 1 - i] = *d;
        }
        Self::new(digits)
    }

    /// Converts the `Tryte` into a signed 64-bit integer.
    ///
    /// # Returns
    ///
    /// A `i64` representing the decimal value of the `Tryte`.
    pub fn to_i64(&self) -> i64 {
        self.to_ternary().to_dec()
    }

    /// Creates a `Tryte` from a signed 64-bit integer.
    ///
    /// # Arguments
    ///
    /// * `v` - A signed 64-bit integer.
    ///
    /// # Returns
    ///
    /// A `Tryte` representing the equivalent ternary number.
    pub fn from_i64(v: i64) -> Self {
        Self::from_ternary(&Ternary::from_dec(v))
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

    /// Evaluates the numeric value without allocation.
    fn to_i128(self) -> i128 {
        self.raw
            .iter()
            .fold(0i128, |acc, digit| acc * 3 + digit.to_i8() as i128)
    }

    /// Converts an i128 into a wrapped SIZE-trit word.
    ///
    /// Also reports whether the original value was
    /// outside the signed SIZE-trit range.
    fn from_i128_wrapping(value: i128) -> (Self, bool) {
        let modulus = 3_i128.pow(SIZE as u32);
        let max = (modulus - 1) / 2;

        let overflow = value < -max || value > max;
        let mut value = (value + max).rem_euclid(modulus) - max;
        let mut digits = [Zero; SIZE];

        for i in (0..SIZE).rev() {
            let remainder = value.rem_euclid(3);
            value = value.div_euclid(3);
            digits[i] = match remainder {
                0 => Zero,
                1 => Pos,
                2 => {
                    value += 1;
                    Neg
                }
                _ => unreachable!(),
            };
        }
        debug_assert_eq!(value, 0);
        (Self::new(digits), overflow)
    }

    /// Multiplies two words modulo 3^SIZE.
    pub fn wrapping_mul(self, rhs: Self) -> Self {
        self.overflowing_mul(rhs).0
    }

    /// Multiplies and reports signed ternary overflow.
    pub fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
        let product = self.to_i128() * rhs.to_i128();
        Self::from_i128_wrapping(product)
    }

    /// Returns None if the exact product cannot
    /// be represented using SIZE balanced trits.
    pub fn checked_mul(self, rhs: Self) -> Option<Self> {
        let (result, overflow) = self.overflowing_mul(rhs);
        if overflow { None } else { Some(result) }
    }

    /// Multiplies, clamping to MIN or MAX on overflow.
    pub fn saturating_mul(self, rhs: Self) -> Self {
        let product = self.to_i128() * rhs.to_i128();
        let max = (3_i128.pow(SIZE as u32) - 1) / 2;
        if product > max {
            Self::MAX
        } else if product < -max {
            Self::MIN
        } else {
            Self::from_i128_wrapping(product).0
        }
    }

    /// Divides two signed ternary values.
    ///
    /// Division truncates toward zero.
    /// Returns None if the divisor is zero.
    ///
    /// This is an integer division, not a modular division.
    pub fn checked_div(self, rhs: Self) -> Option<Self> {
        let divisor = rhs.to_i128();
        if divisor == 0 {
            return None;
        }
        let quotient = self.to_i128() / divisor;
        Some(Self::from_i128_wrapping(quotient).0)
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
    /// An inverse exists only if the value is not
    /// divisible by 3.
    ///
    /// Returns None for SIZE == 0 or a non-invertible value.
    pub fn checked_modular_inverse(self) -> Option<Self> {
        if SIZE == 0 {
            return None;
        }

        let modulus = 3_i128.pow(SIZE as u32);
        let value = self.to_i128().rem_euclid(modulus);
        if value % 3 == 0 {
            return None;
        }

        let (mut old_r, mut r) = (modulus, value);
        let (mut old_t, mut t) = (0_i128, 1_i128);
        while r != 0 {
            let quotient = old_r / r;
            (old_r, r) = (r, old_r - quotient * r);
            (old_t, t) = (t, old_t - quotient * t);
        }
        debug_assert_eq!(old_r, 1);

        let inverse = old_t.rem_euclid(modulus);
        Some(Self::from_i128_wrapping(inverse).0)
    }

    /// Computes self / rhs modulo 3^SIZE.
    ///
    /// Division is defined as multiplication by the
    /// modular inverse of rhs.
    ///
    /// Returns None when rhs is not invertible.
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
    /// Formats the `Tryte` for display.
    ///
    /// The `Tryte` is displayed in its balanced ternary representation
    /// as a SIZE-character string.
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:01$}", self.to_ternary().to_string(), SIZE)
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
    type Output = Tryte<SIZE>;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self::from_ternary(&(&self.to_ternary() & &rhs.to_ternary()))
    }
}

impl<const SIZE: usize> BitOr for Tryte<SIZE> {
    type Output = Tryte<SIZE>;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::from_ternary(&(&self.to_ternary() | &rhs.to_ternary()))
    }
}

impl<const SIZE: usize> BitXor for Tryte<SIZE> {
    type Output = Tryte<SIZE>;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::from_ternary(&(&self.to_ternary() ^ &rhs.to_ternary()))
    }
}

impl<const SIZE: usize> Not for Tryte<SIZE> {
    type Output = Tryte<SIZE>;
    fn not(self) -> Self::Output {
        -self
    }
}

impl<const SIZE: usize> From<Ternary> for Tryte<SIZE> {
    fn from(value: Ternary) -> Self {
        Tryte::from_ternary(&value)
    }
}

impl<const SIZE: usize> From<Tryte<SIZE>> for Ternary {
    fn from(value: Tryte<SIZE>) -> Self {
        value.to_ternary()
    }
}

impl<const SIZE: usize> From<&str> for Tryte<SIZE> {
    fn from(value: &str) -> Self {
        Self::from_ternary(&Ternary::parse(value))
    }
}

impl<const SIZE: usize> From<String> for Tryte<SIZE> {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl<const SIZE: usize> From<Tryte<SIZE>> for String {
    fn from(value: Tryte<SIZE>) -> Self {
        value.to_string()
    }
}

impl<const SIZE: usize> From<i64> for Tryte<SIZE> {
    fn from(value: i64) -> Self {
        Self::from_i64(value)
    }
}

impl<const SIZE: usize> From<Tryte<SIZE>> for i64 {
    fn from(value: Tryte<SIZE>) -> Self {
        value.to_i64()
    }
}

impl<const SIZE: usize> FromStr for Tryte<SIZE> {
    type Err = crate::ParseTernaryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Tryte::from_ternary(&Ternary::from_str(s)?))
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

}
