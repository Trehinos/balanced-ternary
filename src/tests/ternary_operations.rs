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
            assert_eq!(
                difference.to_digit_slice(),
                difference.trim().to_digit_slice()
            );
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
            assert_eq!(
                remainder.to_digit_slice(),
                remainder.trim().to_digit_slice()
            );
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
