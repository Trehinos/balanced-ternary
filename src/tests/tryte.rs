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
