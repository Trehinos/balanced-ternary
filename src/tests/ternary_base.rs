use crate::concepts::DigitOperate;
use crate::{Digit, Ternary};
use Digit::{Pos, Zero};
use alloc::string::ToString;
use alloc::vec;
use core::cmp::Ordering;

#[test]
fn test_ternary() {
    use crate::*;
    use alloc::string::ToString;

    let repr5 = Ternary::new(vec![Pos, Neg, Neg]);
    assert_eq!(repr5.to_dec(), 5);
    let repr5 = Ternary::from_dec(5);
    assert_eq!(repr5.to_dec(), 5);

    let repr13 = Ternary::new(vec![Pos, Pos, Pos]);
    assert_eq!(repr13.to_dec(), 13);

    let repr14 = Ternary::parse("+---");
    let repr15 = Ternary::parse("+--0");
    assert_eq!(repr14.to_dec(), 14);
    assert_eq!(repr15.to_dec(), 15);
    assert_eq!(repr14.to_string(), "+---");
    assert_eq!(repr15.to_string(), "+--0");

    let repr120 = Ternary::from_dec(120);
    assert_eq!(repr120.to_dec(), 120);
    assert_eq!(repr120.to_string(), "++++0");
    let repr121 = Ternary::from_dec(121);
    assert_eq!(repr121.to_dec(), 121);
    assert_eq!(repr121.to_string(), "+++++");

    let repr_neg_5 = Ternary::parse("-++");
    assert_eq!(repr_neg_5.to_dec(), -5);
    assert_eq!(repr_neg_5.to_string(), "-++");

    let repr_neg_5 = Ternary::from_dec(-5);
    assert_eq!(repr_neg_5.to_dec(), -5);
    assert_eq!(repr_neg_5.to_string(), "-++");

    let repr_neg_121 = Ternary::from_dec(-121);
    assert_eq!(repr_neg_121.to_dec(), -121);
    assert_eq!(repr_neg_121.to_string(), "-----");

    let test = Ternary::from_dec(18887455);
    assert_eq!(test.to_dec(), 18887455);
    assert_eq!(test.to_string(), "++00--0--+-0++0+");

    let unbalanced = Ternary::from_unbalanced("12");
    assert_eq!(unbalanced.to_dec(), 5);
    assert_eq!(unbalanced.to_string(), "+--");

    let unbalanced = Ternary::from_unbalanced("-12");
    assert_eq!(unbalanced.to_dec(), -5);
    assert_eq!(unbalanced.to_string(), "-++");

    let unbalanced = Ternary::from_dec(121);
    assert_eq!(unbalanced.to_unbalanced(), "11111");
    assert_eq!(unbalanced.to_string(), "+++++");
}

#[test]
fn test_each() {
    use crate::*;
    use alloc::string::ToString;
    let ternary = Ternary::parse("+0-");
    assert_eq!(ternary.each(Digit::possibly).to_string(), "++-");
}

#[test]
fn test_operations() {
    use alloc::string::ToString;
    fn test_ternary_eq(a: Ternary, b: &str) {
        let repr = Ternary::parse(b);
        assert_eq!(a.to_string(), repr.to_string());
    }
    fn test_binary_op(a: &Ternary, f: impl Fn(Digit, Digit) -> Digit, b: &Ternary, c: &str) {
        test_ternary_eq(a.each_zip(f, b.clone()), c);
    }

    use core::ops::{BitAnd, BitOr, BitXor, Mul, Not};

    let short = Ternary::parse("-0+");
    let long = Ternary::parse("---000+++");
    let other = Ternary::parse("-0+-0+-0+");

    // K3
    test_ternary_eq(short.each(Digit::not), "+0-");
    test_binary_op(&long, Digit::bitand, &other, "----00-0+");
    test_binary_op(&long, Digit::bitor, &other, "-0+00++++");
    test_binary_op(&long, Digit::bitxor, &other, "-0+000+0-");
    test_binary_op(&long, Digit::k3_equiv, &other, "+0-000-0+");
    test_binary_op(&long, Digit::k3_imply, &other, "+++00+-0+");

    // HT
    test_ternary_eq(short.each(Digit::ht_not), "+--");
    test_binary_op(&long, Digit::ht_imply, &other, "+++-++-0+");

    // BI3
    test_binary_op(&long, Digit::bi3_and, &other, "-0-000-0+");
    test_binary_op(&long, Digit::bi3_or, &other, "-0+000+0+");
    test_binary_op(&long, Digit::bi3_imply, &other, "+0+000-0+");

    // L3
    test_ternary_eq(short.each(Digit::possibly), "-++");
    test_ternary_eq(short.each(Digit::necessary), "--+");
    test_ternary_eq(short.each(Digit::contingently), "-+-");
    test_binary_op(&long, Digit::l3_imply, &other, "+++0++-0+");

    // PARA / RM3
    test_binary_op(&long, Digit::rm3_imply, &other, "+++-0+--+");
    test_binary_op(&long, Digit::para_imply, &other, "+++-0+-0+");

    // Other operations
    test_ternary_eq(short.each(Digit::post), "0+-");
    test_ternary_eq(short.each(Digit::pre), "+-0");
    test_ternary_eq(short.each(Digit::absolute_positive), "+0+");
    test_ternary_eq(short.each(Digit::positive), "00+");
    test_ternary_eq(short.each(Digit::not_negative), "0++");
    test_ternary_eq(short.each(Digit::not_positive), "--0");
    test_ternary_eq(short.each(Digit::negative), "-00");
    test_ternary_eq(short.each(Digit::absolute_negative), "-0-");

    test_binary_op(&long, Digit::mul, &other, "+0-000-0+");
}

#[test]
fn test_from_str() {
    use alloc::string::ToString;
    use core::str::FromStr;

    let ternary = Ternary::from_str("+-0").unwrap();
    assert_eq!(ternary.to_string(), "+-0");

    assert!(Ternary::from_str("+-x").is_err());

    #[cfg(feature = "tryte")]
    {
        let tryte = <crate::Tryte>::from_str("+-0").unwrap();
        assert_eq!(tryte.to_string(), "000+-0");
        assert!(<crate::Tryte>::from_str("+-x").is_err());
    }
}

#[test]
fn test_ordering() {
    use crate::ter;

    assert!(ter("-+") < ter("0"));
    assert!(ter("0") < ter("++"));
}

#[test]
fn test_ordering_additional() {
    use crate::ter;

    // Validate comparisons across a range of values
    assert!(ter("--") < ter("-0"));
    assert!(ter("-0") < ter("-"));
    assert!(ter("+") < ter("+-"));
    assert!(ter("+-") < ter("++"));

    // Sorting should arrange values by their decimal value
    let mut values = vec![
        ter("+"),
        ter("--"),
        ter("+-"),
        ter("-"),
        ter("0"),
        ter("-0"),
        ter("++"),
    ];
    values.sort();
    let expected = vec![
        ter("--"),
        ter("-0"),
        ter("-"),
        ter("0"),
        ter("+"),
        ter("+-"),
        ter("++"),
    ];
    assert_eq!(values, expected);
}

#[test]
fn test_iterators() {
    use crate::*;

    let ternary = Ternary::parse("+0-");
    let expected = vec![Pos, Zero, Neg];
    assert_eq!(ternary.iter().cloned().collect::<Vec<_>>(), expected);
    let collected: Vec<Digit> = Ternary::parse("+0-").into_iter().collect();
    assert_eq!(collected, expected);
}

#[test]
fn test_i64_conversion_overflow() {
    use crate::{Neg, Pos, Ternary, Zero};

    let positive = Ternary::new(vec![Pos; 41]);
    let negative = Ternary::new(vec![Neg; 41]);

    assert_eq!(positive.checked_to_dec(), None);
    assert_eq!(negative.checked_to_dec(), None);

    // Leading zeros must not cause an overflow.
    let zero = Ternary::new(vec![Zero; 100]);
    assert_eq!(zero.checked_to_dec(), Some(0));

    // Long representations containing only zeros.
    assert_eq!(Ternary::parse(&"0".repeat(150)).checked_to_dec(), Some(0));

    // Values exceeding i128 must not panic.
    assert_eq!(Ternary::parse(&"+".repeat(150)).checked_to_dec(), None);
    assert_eq!(Ternary::parse(&"-".repeat(150)).checked_to_dec(), None);

    // Leading zeros must not change a valid value.
    let min = Ternary::from_dec(i64::MIN);
    let mut padded = vec![Zero; 150];
    padded.extend_from_slice(min.to_digit_slice());

    assert_eq!(Ternary::new(padded).checked_to_dec(), Some(i64::MIN));
}

#[test]
fn numeric_equality() {
    assert_eq!(Ternary::parse("+"), Ternary::parse("000+"));
    assert_eq!(Ternary::parse("-"), Ternary::parse("00-"));
    assert_eq!(Ternary::parse("0"), Ternary::parse("000"));
    assert_eq!(Ternary::parse("0"), Ternary::new(vec![]));

    assert_ne!(Ternary::parse("+"), Ternary::parse("-"));
}

#[test]
fn numeric_ordering() {
    assert!(Ternary::parse("--") < Ternary::parse("-"));
    assert!(Ternary::parse("-") < Ternary::parse("0"));
    assert!(Ternary::parse("0") < Ternary::parse("+"));
    assert!(Ternary::parse("+") < Ternary::parse("++"));

    assert!(Ternary::parse("-+") < Ternary::parse("-"));
    assert!(Ternary::parse("+") < Ternary::parse("+-"));

    assert_eq!(
        Ternary::parse("000+").cmp(&Ternary::parse("+")),
        Ordering::Equal
    );
}

#[test]
fn arbitrary_precision_comparison() {
    let positive = Ternary::parse(&"+".repeat(256));
    let larger = Ternary::parse(&"+".repeat(257));

    assert!(positive < larger);

    let negative = Ternary::parse(&"-".repeat(256));
    let smaller = Ternary::parse(&"-".repeat(257));

    assert!(smaller < negative);
}

fn add_digits(a: Digit, b: Digit, carry: Digit) -> (Digit, Digit) {
    let sum = a.to_i8() + b.to_i8() + carry.to_i8();
    let next = (sum + 1).div_euclid(3);

    (Digit::from_i8(next), Digit::from_i8(sum - next * 3))
}

#[test]
fn ternary_each_preserves_length() {
    let a = Ternary::parse("00+-");

    assert_eq!(a.each(|d| d).to_string(), "00+-");

    assert_eq!(a.each_with(|d, _| d, Pos).to_string(), "00+-");
}

#[test]
fn ternary_zip_preserves_argument_order() {
    let a = Ternary::parse("+");
    let b = Ternary::parse("--");

    assert_eq!(a.each_zip(|left, _| left, b.clone()).to_string(), "0+");

    assert_eq!(a.each_zip(|_, right| right, b.clone()).to_string(), "--");

    assert_eq!(
        a.each_zip_carry(|left, _, _| (Zero, left), b,).to_string(),
        "0+"
    );
}

#[test]
fn ternary_carry_is_preserved() {
    let a = Ternary::parse("++");
    let b = Ternary::parse("+");

    assert_eq!(a.each_zip_carry(add_digits, b).to_string(), "+--");

    assert_eq!(
        Ternary::parse("-")
            .each_zip_carry(add_digits, Ternary::parse("-"))
            .to_string(),
        "-+"
    );
}

#[cfg(feature = "tryte")]
#[test]
fn tryte_carry_wraps_at_fixed_width() {
    use crate::Tryte;

    let a = Tryte::<2>::new([Pos, Pos]);
    let b = Tryte::<2>::new([Zero, Pos]);

    assert_eq!(a.each_zip_carry(add_digits, b), Tryte::<2>::MIN);

    assert_eq!(Tryte::<0>::ZERO.digit(0), None);
}

#[cfg(feature = "ternary-store")]
#[test]
fn ter40_carry_wraps_at_fixed_width() {
    use crate::Ter40;

    let result = Ter40::MAX.each_zip_carry(add_digits, Ter40::from_dec(1));

    assert_eq!(result, Ter40::MIN);
}

#[test]
fn test_i64_conversion_boundaries() {
    let values = [
        i64::MIN,
        i64::MIN + 1,
        -243,
        -1,
        0,
        1,
        243,
        i64::MAX - 1,
        i64::MAX,
    ];

    for value in values {
        let ternary = Ternary::from_dec(value);

        assert_eq!(
            ternary.checked_to_dec(),
            Some(value),
            "Conversion failed for {value}"
        );

        assert_eq!(ternary.to_dec(), value);
    }
}

#[test]
fn test_i64_conversion_roundtrip() {
    for value in -10_000i64..=10_000 {
        let ternary = Ternary::from_dec(value);
        assert_eq!(ternary.checked_to_dec(), Some(value));
    }

    let mut seed = 0x9E3779B97F4A7C15u64;

    for _ in 0..10_000 {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);

        let value = seed as i64;
        let ternary = Ternary::from_dec(value);

        assert_eq!(ternary.checked_to_dec(), Some(value));
    }
}

#[test]
fn test_ternary_trim() {
    let cases = [
        ("", "0"),
        ("0", "0"),
        ("000000", "0"),
        ("+", "+"),
        ("-", "-"),
        ("000+", "+"),
        ("000-", "-"),
        ("000+-0", "+-0"),
        ("000-0+", "-0+"),
    ];

    for (input, expected) in cases {
        let value = Ternary::parse(input);
        let trimmed = value.trim();

        assert_eq!(
            trimmed.to_string(),
            expected,
            "Trimming failed for {input:?}"
        );

        // Check structural idempotence, not only
        // numeric equality.
        assert_eq!(trimmed.to_digit_slice(), trimmed.trim().to_digit_slice());
    }
}

#[test]
fn test_ternary_trim_arbitrary_precision() {
    use crate::Digit::Neg;

    for width in [256, 1024] {
        for digit in [Pos, Neg] {
            // Prepend zeros to a large nonzero value.
            let mut digits = vec![Zero; 32];
            digits.extend(vec![digit; width]);

            let original = Ternary::new(digits.clone());
            let trimmed = original.trim();

            // The significant digits are preserved.
            assert_eq!(trimmed.to_digit_slice(), &digits[32..]);

            assert_eq!(trimmed.log(), width);

            // The original representation is unchanged.
            assert_eq!(original.to_digit_slice(), digits.as_slice());

            // Trimming twice produces the same digits.
            assert_eq!(trimmed.trim().to_digit_slice(), trimmed.to_digit_slice());
        }
    }
}
#[test]
fn parsing_preserves_exact_representation() {
    use crate::Digit::{Neg, Pos, Zero};
    use alloc::string::String;

    let cases: &[(&str, &[Digit])] = &[
        ("", &[]),
        ("0", &[Zero]),
        ("000", &[Zero, Zero, Zero]),
        ("+", &[Pos]),
        ("-", &[Neg]),
        ("00+-0", &[Zero, Zero, Pos, Neg, Zero]),
    ];

    for &(input, expected_digits) in cases {
        let results = [
            Ternary::try_parse(input),
            input.parse::<Ternary>(),
            Ternary::try_from(input),
            Ternary::try_from(String::from(input)),
        ];

        for result in results {
            let value = result.unwrap();

            assert_eq!(value.to_digit_slice(), expected_digits);
            assert_eq!(value.to_string(), input);
        }

        assert_eq!(Ternary::parse(input).to_digit_slice(), expected_digits);
    }
}

#[test]
fn parsing_reports_the_first_invalid_character() {
    use crate::ParseTernaryError;
    use alloc::string::String;

    let cases = [
        ("x", 'x', 0),
        ("+0x-", 'x', 2),
        ("+x?", 'x', 1),
        (" 0", ' ', 0),
        ("0 ", ' ', 1),
        ("0\n", '\n', 1),
        ("0\0", '\0', 1),
        ("0é", 'é', 1),
        ("0−", '−', 1),
        ("+🙂", '🙂', 1),
        ("12", '1', 0),
    ];

    for (input, character, index) in cases {
        let expected = ParseTernaryError::InvalidCharacter { character, index };

        let results = [
            Ternary::try_parse(input),
            input.parse::<Ternary>(),
            Ternary::try_from(input),
            Ternary::try_from(String::from(input)),
        ];

        for result in results {
            assert_eq!(result, Err(expected), "Input: {input:?}");
        }
    }
}

#[test]
fn empty_parsing_preserves_zero_width() {
    let empty = Ternary::try_parse("").unwrap();
    let zero = Ternary::from_dec(0);

    // Numeric equality does not imply identical representations.
    assert_eq!(empty, zero);
    assert_ne!(empty.to_digit_slice(), zero.to_digit_slice());

    assert_eq!(empty.checked_to_dec(), Some(0));
    assert_eq!(empty.log(), 0);
    assert_eq!(empty.to_string(), "");
    assert_eq!(empty.trim().to_digit_slice(), &[Digit::Zero]);
}

#[test]
fn parsing_supports_large_representations() {
    let input = alloc::format!("000{}", "+0-".repeat(1024));
    let value = Ternary::try_parse(&input).unwrap();

    assert_eq!(value.to_string(), input);
    assert_eq!(value.log(), input.len());
    assert_eq!(value.checked_to_dec(), None);
}

#[test]
#[should_panic(expected = "invalid balanced ternary string")]
fn infallible_parse_rejects_invalid_input() {
    let _ = Ternary::parse("+x");
}

mod unbalanced_tests {
    use crate::{ParseUnbalancedTernaryError, Ternary};
    use alloc::{
        format,
        string::{String, ToString},
        vec::Vec,
    };

    fn reference_unbalanced(value: i64) -> String {
        let mut magnitude = value.unsigned_abs();
        let mut bytes = Vec::new();

        loop {
            bytes.push(b'0' + (magnitude % 3) as u8);
            magnitude /= 3;

            if magnitude == 0 {
                break;
            }
        }

        if value < 0 {
            bytes.push(b'-');
        }

        bytes.reverse();
        String::from_utf8(bytes).unwrap()
    }

    #[test]
    fn unbalanced_parsing_normalizes_numeric_input() {
        for (input, balanced, unbalanced) in [
            ("0", "0", "0"),
            ("000", "0", "0"),
            ("+000", "0", "0"),
            ("-000", "0", "0"),
            ("1", "+", "1"),
            ("2", "+-", "2"),
            ("-2", "-+", "-2"),
            ("12", "+--", "12"),
            ("+0012", "+--", "12"),
            ("-0012", "-++", "-12"),
            ("22", "+0-", "22"),
        ] {
            let value = Ternary::try_from_unbalanced(input).unwrap();

            assert_eq!(value.to_string(), balanced, "input: {input}");
            assert_eq!(value.to_unbalanced(), unbalanced, "input: {input}");
            assert_eq!(
                Ternary::from_unbalanced(input).to_digit_slice(),
                value.to_digit_slice(),
                "input: {input}",
            );
        }
    }

    #[test]
    fn unbalanced_parsing_requires_digits() {
        for input in ["", "+", "-"] {
            assert_eq!(
                Ternary::try_from_unbalanced(input),
                Err(ParseUnbalancedTernaryError::MissingDigits),
                "input: {input:?}",
            );
        }
    }

    #[test]
    fn unbalanced_parsing_reports_the_first_invalid_character() {
        for (input, character, index) in [
            ("3", '3', 0),
            (" 1", ' ', 0),
            ("1 ", ' ', 1),
            ("1\n", '\n', 1),
            ("1_0", '_', 1),
            ("++1", '+', 1),
            ("--1", '-', 1),
            ("+-1", '-', 1),
            ("1-0", '-', 1),
            ("0b10", 'b', 1),
            ("é12", 'é', 0),
            ("+12é", 'é', 3),
            ("-0🙂", '🙂', 2),
            ("１２", '１', 0),
        ] {
            assert_eq!(
                Ternary::try_from_unbalanced(input),
                Err(ParseUnbalancedTernaryError::InvalidCharacter { character, index }),
                "input: {input:?}",
            );
        }
    }

    #[test]
    fn unbalanced_conversions_match_native_reference() {
        for value in (-10_000i64..=10_000).chain([i64::MIN, i64::MAX]) {
            let expected = reference_unbalanced(value);
            let ternary = Ternary::from_dec(value);

            assert_eq!(ternary.to_unbalanced(), expected, "value: {value}",);

            let parsed = Ternary::try_from_unbalanced(&expected).unwrap();

            assert_eq!(parsed.checked_to_dec(), Some(value));
            assert_eq!(parsed.to_digit_slice(), ternary.to_digit_slice());
        }
    }

    #[test]
    fn unbalanced_conversions_support_large_known_values() {
        for width in [1usize, 40, 41, 128, 4096] {
            let zeros = "0".repeat(width);
            let inner_zeros = "0".repeat(width - 1);
            let twos = "2".repeat(width);

            // Check 3^width and 3^width - 1, with both signs.
            for (unbalanced, balanced) in [
                (format!("1{zeros}"), format!("+{zeros}")),
                (format!("-1{zeros}"), format!("-{zeros}")),
                (twos.clone(), format!("+{inner_zeros}-")),
                (format!("-{twos}"), format!("-{inner_zeros}+")),
            ] {
                let value = Ternary::try_from_unbalanced(&unbalanced).unwrap();

                assert_eq!(value.to_string(), balanced);
                assert_eq!(value.to_unbalanced(), unbalanced);

                if width >= 40 {
                    assert_eq!(value.checked_to_dec(), None);
                }
            }
        }
    }

    #[test]
    fn unbalanced_round_trips_normalize_balanced_representations() {
        for input in ["", "0", "000", "000+-0", "-++", "+0-+-0"] {
            let value = Ternary::parse(input);
            let restored = Ternary::try_from_unbalanced(&value.to_unbalanced()).unwrap();

            assert_eq!(
                restored.to_digit_slice(),
                value.trim().to_digit_slice(),
                "input: {input:?}",
            );
        }

        let input = format!("000{}", "+0-".repeat(1024));
        let value = Ternary::parse(&input);
        let restored = Ternary::try_from_unbalanced(&value.to_unbalanced()).unwrap();

        assert_eq!(restored.to_digit_slice(), value.trim().to_digit_slice(),);
    }

    #[test]
    fn unbalanced_parsing_accepts_long_zero_padding() {
        let padding = "0".repeat(4096);

        let zero = Ternary::try_from_unbalanced(&format!("-{padding}")).unwrap();
        assert_eq!(zero.to_string(), "0");

        let value = Ternary::try_from_unbalanced(&format!("+{padding}12")).unwrap();
        assert_eq!(value.to_string(), "+--");
        assert_eq!(value.to_unbalanced(), "12");
    }

    #[test]
    #[should_panic(expected = "invalid unbalanced ternary string")]
    fn unbalanced_panicking_constructor_rejects_invalid_input() {
        let _ = Ternary::from_unbalanced("1023");
    }
}
