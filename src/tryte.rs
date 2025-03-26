use crate::{
    Digit,
    Digit::{Neg, Pos, Zero},
    Ternary,
};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg as StdNeg, Not, Sub};
use crate::concepts::DigitOperate;

/// The `Tryte<S>` struct represents a Copy type balanced ternary number with exactly S digits (6 by default).
/// Each digit in a balanced ternary system can have one of three values: -1, 0, or 1.
///
/// A [Tryte<6>] can holds value between `-364` and `+364`.
///
/// The underlying representation of the number is an array of SIZE `Digit` values.
/// This struct provides conversion methods to and from other formats.
///
/// # Default SIZE
///
/// `SIZE` is 6 by default (the size of a tryte in a Setun computer).
///
/// > **6 trits ~= 9.505 bits**
///
/// > `-364` to `364`
///
/// # Warning
///
/// Because arithmetic operations are performed in with 64 bits integers, `SIZE` cannot be > 40.
///
/// > **40 trits ~= 63,398 bits**
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
        if index > SIZE - 1 {
            None
        } else {
            Some(*self.raw.iter().rev().nth(index).unwrap())
        }
    }

    /// See [Ternary::each].
    fn each(&self, f: impl Fn(Digit) -> Digit) -> Self {
        Self::from_ternary(&self.to_ternary().each(f))
    }

    /// See [Ternary::each_with].
    fn each_with(&self, f: impl Fn(Digit, Digit) -> Digit, with: Digit) -> Self {
        Self::from_ternary(&self.to_ternary().each_with(f, with))
    }

    /// See [Ternary::each_zip].
    fn each_zip(&self, f: impl Fn(Digit, Digit) -> Digit, other: Self) -> Self {
        Self::from_ternary(&self.to_ternary().each_zip(f, other.to_ternary()))
    }

    /// See [Ternary::each_zip_carry].
    fn each_zip_carry(
        &self,
        f: impl Fn(Digit, Digit, Digit) -> (Digit, Digit),
        other: Self,
    ) -> Self {
        Self::from_ternary(&self.to_ternary().each_zip_carry(f, other.to_ternary()))
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
        Self::from_ternary(&(&self.to_ternary() + &rhs.to_ternary()))
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

#[cfg(test)]
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
