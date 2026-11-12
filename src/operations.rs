//! This module provides implementations for arithmetic operations on the `Ternary` type
//! such as addition, subtraction, multiplication, and division.
//! Using `Ternary` arithmetic:
//!
//! ```rust
//! use balanced_ternary::Ternary;
//!
//! let repr9 = Ternary::parse("+00"); // Represents decimal 9 in balanced ternary
//! let repr4 = Ternary::parse("++");  // Represents decimal 4 in balanced ternary
//! let sum = &repr9 + &repr4;         // Results in Ternary::parse("+++"), decimal 13
//! assert_eq!(sum.to_dec(), 13);
//! let difference = &sum - &repr4;   // Results in Ternary::parse("+00"), decimal 9
//! assert_eq!(difference.to_dec(), 9);
//! ```
//!
//! # Implementations
//!
//! The following arithmetic operations are implemented for the `Ternary` :
//!
//! ## `Ternary` type
//!
//! - `Neg` and `Not` for `&Ternary`: Negates the `Ternary` by negating each digit in its balanced ternary representation.
//! - `Add<&Ternary>` for `&Ternary`: Performs arbitrary-precision balanced ternary addition directly on digits.
//! - `Sub<&Ternary>` for `&Ternary`: Performs arbitrary-precision balanced ternary subtraction directly on digits.
//! - `Mul<&Ternary>` for `&Ternary`: Performs arbitrary-precision multiplication directly on balanced ternary digits.
//! - `Div<&Ternary>` for `&Ternary`: Performs arbitrary-precision signed integer division, truncating toward zero.
//!   Panics on division by zero.
//! - `Rem<&Ternary>` for `&Ternary`: Computes the remainder of the signed integer division. The remainder has the sign of the dividend.
//! - `Ternary::div_rem()` computes quotient and remainder together, without converting the operands to machine integers.
//! - `BitAnd<&Ternary>` for `&Ternary`: Computes the bitwise AND operation on two `Ternary` operands.
//! - `BitOr<&Ternary>` for `&Ternary`: Computes the bitwise OR operation on two `Ternary` operands.
//! - `BitXor<&Ternary>` for `&Ternary`: Computes the bitwise XOR operation on two `Ternary` operands.

use crate::concepts::DigitOperate;
use crate::{Digit, Ternary};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Shl, Shr, Sub};

impl Ternary {
    fn add_sub_digits(lhs: &[Digit], rhs: &[Digit], subtract: bool) -> Ternary {
        let width = lhs.len().max(rhs.len());

        let mut result = Vec::with_capacity(width.saturating_add(1));
        let mut lhs = lhs.iter().rev();
        let mut rhs = rhs.iter().rev();
        let mut carry = 0i8;

        for _ in 0..width {
            let a = lhs.next().copied().unwrap_or(Digit::Zero).to_i8();
            let b = rhs.next().copied().unwrap_or(Digit::Zero).to_i8();
            let b = if subtract { -b } else { b };
            let sum = a + b + carry;
            carry = (sum + 1).div_euclid(3);

            let digit = sum - 3 * carry;
            result.push(Digit::from_i8(digit));
        }

        if carry != 0 {
            result.push(Digit::from_i8(carry));
        }
        while result.len() > 1 && result.last() == Some(&Digit::Zero) {
            result.pop();
        }
        if result.is_empty() {
            result.push(Digit::Zero);
        }

        result.reverse();
        Ternary::new(result)
    }
    /// Removes most-significant zeros from a little-endian
    /// unsigned base-3 representation.
    fn trim_base3(digits: &mut Vec<u8>) {
        while digits.len() > 1 && digits.last() == Some(&0) {
            digits.pop();
        }
        if digits.is_empty() {
            digits.push(0);
        }
    }

    /// Converts a balanced ternary number into its absolute
    /// magnitude, stored as little-endian base-3 digits.
    pub(super) fn magnitude_base3(value: &Ternary) -> Vec<u8> {
        let digits = value.significant_digits();
        let negative = digits.first() == Some(&Digit::Neg);
        let mut result = Vec::with_capacity(digits.len().max(1));
        let mut carry = 0i8;

        for digit in digits.iter().rev() {
            let value = if negative {
                -digit.to_i8()
            } else {
                digit.to_i8()
            } + carry;
            result.push(value.rem_euclid(3) as u8);
            carry = value.div_euclid(3);
        }
        debug_assert_eq!(carry, 0);
        Self::trim_base3(&mut result);
        result
    }

    /// Compares two normalized, little-endian base-3 numbers.
    fn compare_base3(lhs: &[u8], rhs: &[u8]) -> Ordering {
        lhs.len()
            .cmp(&rhs.len())
            .then_with(|| lhs.iter().rev().cmp(rhs.iter().rev()))
    }

    /// Subtracts rhs from lhs in place.
    ///
    /// Preconditions:
    /// - lhs >= rhs
    /// - Both operands are normalized base-3 magnitudes.
    fn subtract_base3(lhs: &mut Vec<u8>, rhs: &[u8]) {
        debug_assert!(Self::compare_base3(lhs, rhs) != Ordering::Less);

        let mut borrow = 0i8;
        for (index, digit) in lhs.iter_mut().enumerate() {
            let other = rhs.get(index).copied().unwrap_or(0) as i8;
            let mut difference = *digit as i8 - other - borrow;
            if difference < 0 {
                difference += 3;
                borrow = 1;
            } else {
                borrow = 0;
            }
            *digit = difference as u8;
        }
        debug_assert_eq!(borrow, 0);
        Self::trim_base3(lhs);
    }

    /// Performs unsigned long division in base 3.
    ///
    /// Both inputs use little-endian digits.
    /// The divisor must be nonzero.
    ///
    /// Returns (quotient, remainder).
    fn div_rem_base3(dividend: &[u8], divisor: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let mut quotient = Vec::with_capacity(dividend.len());
        let mut remainder = vec![0u8];
        for &digit in dividend.iter().rev() {
            remainder.insert(0, digit);
            Self::trim_base3(&mut remainder);

            let mut quotient_digit = 0u8;
            while Self::compare_base3(&remainder, divisor) != Ordering::Less {
                Self::subtract_base3(&mut remainder, divisor);
                quotient_digit += 1;
            }
            debug_assert!(quotient_digit <= 2);
            quotient.push(quotient_digit);
        }
        quotient.reverse();
        Self::trim_base3(&mut quotient);
        (quotient, remainder)
    }

    /// Converts a little-endian unsigned base-3 magnitude into
    /// a normalized balanced ternary number with the requested sign.
    ///
    /// Every input digit must be in `0..=2`.
    /// An empty magnitude represents zero.
    pub(super) fn ternary_from_base3(
        magnitude: impl ExactSizeIterator<Item = u8>,
        negative: bool,
    ) -> Ternary {
        let mut digits = Vec::with_capacity(magnitude.len() + 1);
        let mut carry = 0u8;
        for digit in magnitude {
            let value = digit + carry;
            if value <= 1 {
                digits.push(Digit::from_i8(value as i8));
                carry = 0;
            } else {
                digits.push(Digit::from_i8(value as i8 - 3));
                carry = 1;
            }
        }
        if carry != 0 {
            digits.push(Digit::Pos);
        }
        while digits.len() > 1 && digits.last() == Some(&Digit::Zero) {
            digits.pop();
        }
        if digits.is_empty() {
            digits.push(Digit::Zero);
        }
        if negative {
            for digit in &mut digits {
                *digit = -*digit;
            }
        }
        digits.reverse();
        Ternary::new(digits)
    }

    pub fn div_rem(&self, rhs: &Self) -> (Self, Self) {
        assert!(
            !rhs.significant_digits().is_empty(),
            "Ternary division by zero"
        );
        let lhs_negative = self.significant_digits().first() == Some(&Digit::Neg);
        let rhs_negative = rhs.significant_digits().first() == Some(&Digit::Neg);
        let dividend = Self::magnitude_base3(self);
        let divisor = Self::magnitude_base3(rhs);
        let (quotient, remainder) = Self::div_rem_base3(&dividend, &divisor);

        (
            Self::ternary_from_base3(quotient.iter().copied(), lhs_negative != rhs_negative),
            Self::ternary_from_base3(remainder.iter().copied(), lhs_negative),
        )
    }
}

impl Neg for &Ternary {
    type Output = Ternary;

    /// Returns the negation of the current `Ternary` object.
    ///
    /// Negates each digit in the number.
    fn neg(self) -> Self::Output {
        let mut repr = Ternary::new(vec![]);
        for digit in self.digits.iter() {
            repr.digits.push(-*digit);
        }
        repr
    }
}

impl Add<&Ternary> for &Ternary {
    type Output = Ternary;
    fn add(self, rhs: &Ternary) -> Self::Output {
        Ternary::add_sub_digits(&self.digits, &rhs.digits, false)
    }
}

impl Add<Digit> for &Ternary {
    type Output = Ternary;
    fn add(self, rhs: Digit) -> Self::Output {
        Ternary::add_sub_digits(&self.digits, &[rhs], false)
    }
}

impl Sub<&Ternary> for &Ternary {
    type Output = Ternary;
    fn sub(self, rhs: &Ternary) -> Self::Output {
        Ternary::add_sub_digits(&self.digits, &rhs.digits, true)
    }
}

impl Sub<Digit> for &Ternary {
    type Output = Ternary;
    fn sub(self, rhs: Digit) -> Self::Output {
        Ternary::add_sub_digits(&self.digits, &[rhs], true)
    }
}

impl Mul<&Ternary> for &Ternary {
    type Output = Ternary;

    fn mul(self, rhs: &Ternary) -> Self::Output {
        let lhs = self.significant_digits();
        let rhs = rhs.significant_digits();
        if lhs.is_empty() || rhs.is_empty() {
            return Ternary::new(vec![Digit::Zero]);
        }

        let mut result = vec![Digit::Zero; lhs.len() + rhs.len() + 1];
        for (j, b) in rhs.iter().rev().enumerate() {
            let mut carry = 0i8;
            for (i, a) in lhs.iter().rev().enumerate() {
                let index = i + j;
                let sum = result[index].to_i8() + a.to_i8() * b.to_i8() + carry;
                carry = (sum + 1).div_euclid(3);
                result[index] = Digit::from_i8(sum - 3 * carry);
            }

            let mut index = lhs.len() + j;
            while carry != 0 {
                let sum = result[index].to_i8() + carry;
                carry = (sum + 1).div_euclid(3);
                result[index] = Digit::from_i8(sum - 3 * carry);
                index += 1;
            }
        }
        while result.len() > 1 && result.last() == Some(&Digit::Zero) {
            result.pop();
        }

        result.reverse();
        Ternary::new(result)
    }
}

impl Div<&Ternary> for &Ternary {
    type Output = Ternary;
    fn div(self, rhs: &Ternary) -> Self::Output {
        self.div_rem(rhs).0
    }
}

impl Rem<&Ternary> for &Ternary {
    type Output = Ternary;
    fn rem(self, rhs: &Ternary) -> Self::Output {
        self.div_rem(rhs).1
    }
}

impl BitAnd<&Ternary> for &Ternary {
    type Output = Ternary;

    fn bitand(self, rhs: &Ternary) -> Self::Output {
        self.each_zip(Digit::bitand, rhs.clone())
    }
}

impl BitOr<&Ternary> for &Ternary {
    type Output = Ternary;

    fn bitor(self, rhs: &Ternary) -> Self::Output {
        self.each_zip(Digit::bitor, rhs.clone())
    }
}

impl BitXor<&Ternary> for &Ternary {
    type Output = Ternary;

    fn bitxor(self, rhs: &Ternary) -> Self::Output {
        self.each_zip(Digit::bitxor, rhs.clone())
    }
}

impl Shl<usize> for &Ternary {
    type Output = Ternary;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut repr = Ternary::new(vec![]);
        repr.digits.extend(self.digits.iter().cloned());
        repr.digits
            .extend(core::iter::repeat(Digit::Zero).take(rhs));
        repr
    }
}

impl Shr<usize> for &Ternary {
    type Output = Ternary;

    fn shr(self, rhs: usize) -> Self::Output {
        if rhs >= self.digits.len() {
            return Ternary::parse("0");
        }
        let len = self.digits.len() - rhs;
        let mut repr = Ternary::new(self.digits[..len].to_vec());
        if repr.digits.is_empty() {
            repr.digits.push(Digit::Zero);
        }
        repr
    }
}

impl Not for &Ternary {
    type Output = Ternary;
    fn not(self) -> Self::Output {
        -self
    }
}

#[cfg(test)]
mod tests {
    use crate::Ternary;
    use alloc::string::ToString;
    use alloc::vec;

    #[test]
    fn test_ternary_ops() {
        use alloc::string::ToString;

        let repr9 = Ternary::parse("+00");
        let repr4 = Ternary::parse("++");
        let repr13 = &repr9 + &repr4;
        let repr17 = &repr13 + &repr4;
        let repr34 = &repr17 + &repr17;

        assert_eq!(repr13.to_string(), "+++");
        assert_eq!(repr17.to_string(), "+-0-");
        assert_eq!(repr34.to_string(), "++-+");

        let repr30 = &repr34 - &repr4;
        assert_eq!(repr30.to_dec(), 30);
        assert_eq!(repr30.to_string(), "+0+0");

        let repr120 = &repr30 * &repr4;
        assert_eq!(repr120.to_dec(), 120);
        assert_eq!(repr120.to_string(), "++++0");

        let repr_neg120 = -&repr120;
        assert_eq!(repr_neg120.to_dec(), -120);
        assert_eq!(repr_neg120.to_string(), "----0");

        let bitwise = &Ternary::parse("++00") & &Ternary::parse("0000");
        assert_eq!(bitwise.to_string(), "0000");

        let bitwise = &Ternary::parse("++00") & &Ternary::parse("0+00");
        assert_eq!(bitwise.to_string(), "0+00");

        let bitwise = &Ternary::parse("+000") | &Ternary::parse("000-");
        assert_eq!(bitwise.to_string(), "+000");

        let bitwise = &Ternary::parse("+000") & &Ternary::parse("000-");
        assert_eq!(bitwise.to_string(), "000-");

        let bitwise = &Ternary::parse("+000") | &Ternary::parse("000+");
        assert_eq!(bitwise.to_string(), "+00+");
    }

    #[test]
    fn test_shift_ops() {
        use alloc::string::ToString;
        let t = Ternary::parse("+0-");
        assert_eq!((&t << 2).to_string(), "+0-00");
        let back = &(&t << 2) >> 2;
        assert_eq!(back.to_string(), "+0-");
        let zero = &t >> 5;
        assert_eq!(zero.to_string(), "0");
    }

    #[test]
    fn test_native_add_sub_exhaustive() {
        use crate::{Digit, Ternary};

        for a in -100i64..=100 {
            for b in -100i64..=100 {
                let lhs = Ternary::from_dec(a);
                let rhs = Ternary::from_dec(b);
                let sum = &lhs + &rhs;
                let difference = &lhs - &rhs;

                assert_eq!(sum.to_dec(), a + b);
                assert_eq!(difference.to_dec(), a - b);
                assert_eq!(&sum - &rhs, lhs);
                assert_eq!(&difference + &rhs, lhs);
                assert_eq!(sum.to_digit_slice(), sum.trim().to_digit_slice());
                assert_eq!(difference.to_digit_slice(), difference.trim().to_digit_slice());
            }
        }

        for value in -100i64..=100 {
            let lhs = Ternary::from_dec(value);
            for digit in [Digit::Neg, Digit::Zero, Digit::Pos] {
                let d = digit.to_i8() as i64;
                assert_eq!((&lhs + digit).to_dec(), value + d);
                assert_eq!((&lhs - digit).to_dec(), value - d);
            }
        }
    }

    #[test]
    fn test_native_add_sub_i64_boundaries() {
        use crate::Ternary;

        let max = Ternary::from_dec(i64::MAX);
        let min = Ternary::from_dec(i64::MIN);
        let one = Ternary::from_dec(1);
        let above_max = &max + &one;
        let below_min = &min - &one;

        assert_eq!(above_max.checked_to_dec(), None);
        assert_eq!(below_min.checked_to_dec(), None);
        assert_eq!(&above_max - &one, max);
        assert_eq!(&below_min + &one, min);
        assert_eq!((&max + &min).to_dec(), -1);
    }

    #[test]
    fn test_native_add_sub_large_values() {
        use crate::Ternary;

        for width in [256, 1024] {
            let positive = Ternary::parse(&"+".repeat(width));
            let negative = Ternary::parse(&"-".repeat(width));
            let one = Ternary::parse("+");
            assert_eq!((&positive + &negative).to_string(), "0");

            let incremented = &positive + &one;
            let decremented = &negative - &one;
            assert_eq!(incremented.log(), width + 1);
            assert_eq!(decremented.log(), width + 1);
            assert_eq!(&incremented - &one, positive);
            assert_eq!(&decremented + &one, negative);
            assert_eq!((&positive - &positive).to_string(), "0");
        }
    }

    #[test]
    fn test_native_add_sub_normalization() {
        use crate::Ternary;

        let empty = Ternary::new(vec![]);
        let zero = Ternary::parse("0000");
        let positive = Ternary::parse("000+");
        let negative = Ternary::parse("000-");
        assert_eq!((&empty + &empty).to_string(), "0");
        assert_eq!((&zero - &zero).to_string(), "0");
        assert_eq!((&positive + &negative).to_string(), "0");
        assert_eq!((&positive - &negative).to_string(), "+-");
        assert_eq!((&empty + &positive).to_string(), "+");
    }

    #[test]
    fn test_native_multiplication_exhaustive() {
        use crate::Ternary;

        for a in -100i64..=100 {
            for b in -100i64..=100 {
                let lhs = Ternary::from_dec(a);
                let rhs = Ternary::from_dec(b);
                let result = &lhs * &rhs;
                assert_eq!(result.to_dec(), a * b);
                assert_eq!(result.to_digit_slice(), result.trim().to_digit_slice());
                assert_eq!(&lhs * &rhs, &rhs * &lhs);
            }
        }
    }

    #[test]
    fn test_native_division_exhaustive() {
        use crate::Ternary;

        for a in -100i64..=100 {
            for b in -100i64..=100 {
                if b == 0 {
                    continue;
                }

                let lhs = Ternary::from_dec(a);
                let rhs = Ternary::from_dec(b);
                let (quotient, remainder) = lhs.div_rem(&rhs);

                assert_eq!(quotient.to_dec(), a / b);
                assert_eq!(remainder.to_dec(), a % b);
                assert_eq!(&lhs / &rhs, quotient);
                assert_eq!(&lhs % &rhs, remainder);
                assert_eq!(&(&quotient * &rhs) + &remainder, lhs);
                assert!(remainder.to_dec().abs() < b.abs());
                if remainder.to_dec() != 0 {
                    assert_eq!(remainder.to_dec().signum(), a.signum());
                }
                assert_eq!(quotient.to_digit_slice(), quotient.trim().to_digit_slice());
                assert_eq!(remainder.to_digit_slice(), remainder.trim().to_digit_slice());
            }
        }
    }

    #[test]
    fn test_native_arithmetic_large_values() {
        use crate::Ternary;

        let a = Ternary::parse(&"+".repeat(512));
        let b = Ternary::parse(&"+".repeat(128));
        let product = &a * &b;

        assert_eq!(product.checked_to_dec(), None);
        assert_eq!(&product / &b, a);
        assert_eq!(&product % &b, Ternary::from_dec(0));

        let (quotient, remainder) = a.div_rem(&b);
        assert_eq!(&(&quotient * &b) + &remainder, a);
        assert!(remainder < b);
        assert_eq!(quotient.checked_to_dec(), None);

        let negative = -&a;
        let (neg_quotient, neg_remainder) = negative.div_rem(&b);
        assert_eq!(neg_quotient, -&quotient);
        assert_eq!(neg_remainder, -&remainder);
    }

    #[test]
    fn test_native_arithmetic_i64_boundaries() {
        use crate::Ternary;

        let min = Ternary::from_dec(i64::MIN);
        let max = Ternary::from_dec(i64::MAX);
        let negative_one = Ternary::from_dec(-1);
        let quotient = &min / &negative_one;
        assert_eq!(quotient, -&min);
        assert_eq!(quotient.checked_to_dec(), None);

        let product = &max * &max;
        assert_eq!(product.checked_to_dec(), None);
        assert_eq!(&product / &max, max);
    }

    #[test]
    #[should_panic(expected = "Ternary division by zero")]
    fn test_native_division_by_zero() {
        use crate::Ternary;

        let a = Ternary::from_dec(42);
        let zero = Ternary::parse("000");
        let _ = &a / &zero;
    }

    #[test]
    fn test_native_mul_div_noncanonical() {
        use crate::Ternary;

        let a = Ternary::parse("000+--");
        let b = Ternary::parse("000+");
        assert_eq!((&a * &b).to_string(), "+--");

        let (quotient, remainder) = a.div_rem(&b);
        assert_eq!(quotient.to_string(), "+--");
        assert_eq!(remainder.to_string(), "0");

        let empty = Ternary::parse("");
        assert_eq!((&empty * &a).to_string(), "0");
        assert_eq!((&empty / &b).to_string(), "0");
        assert_eq!((&empty % &b).to_string(), "0");
    }
}
