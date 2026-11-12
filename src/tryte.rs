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
        let mut digits = [Zero; SIZE];
        let mut carry = Zero;

        for i in (0..SIZE).rev() {
            let sum = self.raw[i].to_i8() + rhs.raw[i].to_i8() + carry.to_i8();

            let next_carry = (sum + 1).div_euclid(3);

            digits[i] = Digit::from_i8(sum - next_carry * 3);

            carry = Digit::from_i8(next_carry);
        }

        (Self::new(digits), carry)
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
        let mut result = [Zero; SIZE];
        let mut carry = Zero;
        for i in (0..SIZE).rev() {
            let (next_carry, digit) = f(self.raw[i], other.raw[i], carry);

            result[i] = digit;
            carry = next_carry;
        }
        Self::new(result)
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
    type Output = Tryte<SIZE>;
    fn neg(self) -> Self::Output {
        Self::from_ternary(&-&self.to_ternary())
    }
}

impl<const SIZE: usize> Add for Tryte<SIZE> {
    type Output = Tryte<SIZE>;

    fn add(self, rhs: Self) -> Self::Output {
        self.wrapping_add(rhs)
    }
}

impl<const SIZE: usize> Sub for Tryte<SIZE> {
    type Output = Tryte<SIZE>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_ternary(&(&self.to_ternary() - &rhs.to_ternary()))
    }
}

impl<const SIZE: usize> Mul for Tryte<SIZE> {
    type Output = Tryte<SIZE>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_ternary(&(&self.to_ternary() * &rhs.to_ternary()))
    }
}

impl<const SIZE: usize> Div for Tryte<SIZE> {
    type Output = Tryte<SIZE>;

    fn div(self, rhs: Self) -> Self::Output {
        Self::from_ternary(&(&self.to_ternary() / &rhs.to_ternary()))
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

                    // Wrapped arithmetic.
                    assert_eq!((lhs + rhs).to_i64(), expected);

                    assert_eq!(lhs.wrapping_add(rhs).to_i64(), expected);

                    // Outgoing carry.
                    let (result, actual_carry) = lhs.add_with_carry(rhs);

                    assert_eq!(result.to_i64(), expected);
                    assert_eq!(actual_carry, carry);

                    // Overflow detection.
                    let (overflowing_result, overflowed) = lhs.overflowing_add(rhs);
                    assert_eq!(overflowing_result.to_i64(), expected);
                    assert_eq!(overflowed, carry != Digit::Zero);

                    // Checked arithmetic.
                    assert_eq!(
                        lhs.checked_add(rhs),
                        if carry == Digit::Zero {
                            Some(result)
                        } else {
                            None
                        }
                    );

                    // Saturating arithmetic.
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
}
