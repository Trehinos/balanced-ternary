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
//! - `Mul<&Ternary>` for `&Ternary`: Multiplies two `Ternary` values and returns a new `Ternary`. Panics on overflow.
//! - `Div<&Ternary>` for `&Ternary`: Divides one `Ternary` by another and returns a new `Ternary`. Panics on overflow or division by zero.
//! - `BitAnd<&Ternary>` for `&Ternary`: Computes the bitwise AND operation on two `Ternary` operands.
//! - `BitOr<&Ternary>` for `&Ternary`: Computes the bitwise OR operation on two `Ternary` operands.
//! - `BitXor<&Ternary>` for `&Ternary`: Computes the bitwise XOR operation on two `Ternary` operands.

use crate::concepts::DigitOperate;
use crate::{Digit, Ternary};
use alloc::vec;
use alloc::vec::Vec;
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Shl, Shr, Sub};

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
        Ternary::from_dec(
            self.to_dec()
                .checked_mul(rhs.to_dec())
                .expect("Overflow in multiplication."),
        )
    }
}

impl Div<&Ternary> for &Ternary {
    type Output = Ternary;

    fn div(self, rhs: &Ternary) -> Self::Output {
        Ternary::from_dec(
            self.to_dec()
                .checked_div(rhs.to_dec())
                .expect("Overflow in division or division by zero."),
        )
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
                assert_eq!(sum, sum.trim());
                assert_eq!(difference, difference.trim());
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

}
