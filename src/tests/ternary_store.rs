use crate::{DataTernary, TernaryConversionError};
use alloc::string::ToString;
use alloc::vec;

#[test]
fn single_chunk_creation() {
    use crate::Ternary;

    let ternary = Ternary::parse("+-0-+");
    let data = DataTernary::from_ternary(ternary.clone());

    assert_eq!(data.chunks.len(), 1);
    assert_eq!(data.to_ternary(), ternary);
}

#[test]
fn round_trip() {
    use crate::Ternary;

    let ternary = Ternary::parse("+0-0++-");
    let data = DataTernary::from_ternary(ternary.clone());

    assert_eq!(data.to_ternary(), ternary);
}

#[test]
fn data_ternary_round_trip_zero_chunks() {
    use crate::Ternary;

    let cases = [
        ("+", "0000+"),
        ("+00000", "0000+00000"),
        ("-00000", "0000-00000"),
        ("+0000000000", "0000+0000000000"),
        ("0000000000", "0000000000"),
    ];

    for (original, expected_fixed) in cases {
        let data = DataTernary::from_ternary(Ternary::parse(original));

        assert_eq!(data.to_fixed_ternary().to_string(), expected_fixed);

        assert_eq!(
            data.to_ternary().to_string(),
            Ternary::parse(original).trim().to_string()
        );
    }
}

#[test]
fn ter40_checked_construction() {
    use crate::Ter40;

    for value in [Ter40::MIN.to_dec(), -1, 0, 1, Ter40::MAX.to_dec()] {
        assert_eq!(Ter40::checked_from_dec(value), Some(Ter40::from_dec(value)));
        assert_eq!(Ter40::try_from(value).unwrap().to_dec(), value);
    }

    for value in [
        i64::MIN,
        Ter40::MIN.to_dec() - 1,
        Ter40::MAX.to_dec() + 1,
        i64::MAX,
    ] {
        assert_eq!(Ter40::checked_from_dec(value), None);
    }
}

#[test]
fn ter40_checked_ternary_conversion() {
    use crate::{Digit, Ter40, Ternary};

    // More than 40 digits, but a representable value.
    let mut digits = vec![Digit::Zero; 100];
    digits.push(Digit::Pos);

    let padded = Ternary::new(digits);

    assert_eq!(
        Ter40::checked_from_ternary(&padded),
        Some(Ter40::from_dec(1))
    );

    // Values outside the signed 40-trit range.
    let positive = Ternary::parse(&"+".repeat(41));
    let negative = Ternary::parse(&"-".repeat(41));

    assert_eq!(Ter40::checked_from_ternary(&positive), None);
    assert_eq!(Ter40::checked_from_ternary(&negative), None);

    // Every valid boundary fits in exactly 40 trits.
    for value in [Ter40::MIN, Ter40::MAX] {
        let ternary = value.to_ternary();

        assert_eq!(ternary.log(), 40);
        assert_eq!(Ter40::checked_from_ternary(&ternary), Some(value));
    }
}

#[test]
#[should_panic(expected = "Ter40 value out of range")]
fn ter40_constructor_overflow() {
    use crate::Ter40;
    let _ = Ter40::from_dec(i64::MAX);
}

#[test]
fn ter40_from_i64_overflow() {
    use crate::Ter40;
    assert_eq!(
        Ter40::try_from(i64::MIN),
        Err(TernaryConversionError::OutOfRange)
    );
}

#[test]
fn ter40_from_ternary_overflow() {
    use crate::{Ter40, Ternary};
    assert_eq!(
        Ter40::try_from(Ternary::from_dec(i64::MAX)),
        Err(TernaryConversionError::OutOfRange)
    );
}

#[test]
#[should_panic(expected = "Ter40 addition overflow")]
fn ter40_add_overflow() {
    use crate::Ter40;
    let _ = Ter40::MAX + Ter40::from_dec(1);
}

#[test]
#[should_panic(expected = "Ter40 subtraction overflow")]
fn ter40_sub_overflow() {
    use crate::Ter40;
    let _ = Ter40::MIN - Ter40::from_dec(1);
}

#[test]
#[should_panic(expected = "Ter40 multiplication overflow")]
fn ter40_mul_overflow() {
    use crate::Ter40;
    let _ = Ter40::MAX * Ter40::from_dec(2);
}

#[test]
#[should_panic(expected = "Ter40 division by zero")]
fn ter40_div_by_zero() {
    use crate::Ter40;
    let _ = Ter40::from_dec(1) / Ter40::ZERO;
}

#[test]
fn ter40_direct_digit_access() {
    use crate::concepts::DigitOperate;
    use crate::{Digit, Ter40, Ternary};

    let values = [
        Ter40::MIN.to_dec(),
        -1_000_000,
        -1,
        0,
        1,
        1_000_000,
        Ter40::MAX.to_dec(),
    ];

    for value in values {
        let word = Ter40::from_dec(value);
        let reference = Ternary::from_dec(value).with_length(40);
        assert_eq!(word.to_digits(), reference.to_digit_slice());
        assert_eq!(
            word.to_ternary().to_digit_slice(),
            reference.to_digit_slice()
        );

        for index in 0..40 {
            assert_eq!(
                word.digit(index),
                reference.get_digit(index).copied(),
                "Failed for value {value} at index {index}"
            );
        }
        assert_eq!(word.digit(40), None);
        assert_eq!(word.digit(100), None);
    }
    assert_eq!(Ter40::ZERO.digit(39), Some(Digit::Zero));
    assert_eq!(Ter40::MAX.digit(39), Some(Digit::Pos));
    assert_eq!(Ter40::MIN.digit(39), Some(Digit::Neg));
}

#[test]
fn ter40_direct_display() {
    use crate::{Ter40, Ternary};
    use alloc::string::ToString;

    for value in [Ter40::MIN.to_dec(), -42, -1, 0, 1, 42, Ter40::MAX.to_dec()] {
        let word = Ter40::from_dec(value);
        let expected = Ternary::from_dec(value).with_length(40).to_string();
        assert_eq!(word.to_string(), expected);
        assert_eq!(word.to_string().len(), 40);
    }
    assert_eq!(Ter40::ZERO.to_string(), "0".repeat(40));
    assert_eq!(Ter40::MAX.to_string(), "+".repeat(40));
    assert_eq!(Ter40::MIN.to_string(), "-".repeat(40));
}

#[test]
fn ter40_direct_conversion_exhaustive() {
    use crate::concepts::DigitOperate;
    use crate::{Ter40, Ternary};

    for value in -1000i64..=1000 {
        let word = Ter40::from_dec(value);
        let reference = Ternary::from_dec(value).with_length(40);
        assert_eq!(
            word.to_ternary().to_digit_slice(),
            reference.to_digit_slice()
        );
        assert_eq!(word.to_digits(), reference.to_digit_slice());
        assert_eq!(word.to_ternary().to_dec(), value);
    }
}

#[test]
fn ternary_r07_regression() {
    use crate::{Digit, Ternary};
    use alloc::string::ToString;

    let value = Ternary::parse("000+-0");
    assert_eq!(value.to_string(), "000+-0");
    assert_eq!(value.get_digit(0), Some(&Digit::Zero));
    assert_eq!(value.get_digit(1), Some(&Digit::Neg));
    assert_eq!(value.get_digit(2), Some(&Digit::Pos));
    assert_eq!(value.get_digit(6), None);
    assert_eq!(value.get_digit(usize::MAX), None);
    assert_eq!(value.with_length(8).to_string(), "00000+-0");
    assert_eq!(value.with_length(2).to_string(), "000+-0");
    assert_eq!(value.concat(&Ternary::parse("-+")).to_string(), "000+-0-+");
    assert_eq!(value.to_string_repr(Digit::to_char_theta), "0001Θ0");

    let empty = Ternary::parse("");
    assert_eq!(empty.to_string(), "");
    assert_eq!(empty.with_length(0).to_string(), "");
    assert_eq!(empty.with_length(3).to_string(), "000");
}

#[test]
fn trits_chunk_direct_decoding() {
    use crate::{Ternary, TritsChunk};

    for value in -121i8..=121 {
        let chunk = TritsChunk::from_dec(value);
        let expected = Ternary::from_dec(value as i64).with_length(5);
        assert_eq!(chunk.to_digits(), expected.to_digit_slice());
        assert_eq!(chunk.to_fixed_ternary(), expected);
    }
}

#[test]
fn data_ternary_direct_encoding() {
    use crate::{DataTernary, Ternary};
    use alloc::string::ToString;

    let cases = [
        "",
        "0",
        "000",
        "+",
        "-",
        "+-0-+",
        "+00000",
        "-00000",
        "0000000000",
        "000+--",
        "+0-0++-",
    ];

    for input in cases {
        let original = Ternary::parse(input);
        let padding = (5 - input.len() % 5) % 5;
        let expected_fixed = original.with_length(input.len() + padding);
        let storage = DataTernary::from_ternary(original.clone());

        assert_eq!(
            storage.to_fixed_ternary(),
            expected_fixed,
            "Fixed representation failed for {input}"
        );
        assert_eq!(
            storage.to_ternary(),
            original.trim(),
            "Numeric representation failed for {input}"
        );
        assert_eq!(storage.to_digits(), original.trim().to_digit_slice());
        assert_eq!(storage.to_string(), expected_fixed.to_string());
    }
}

#[test]
fn data_ternary_direct_numeric_conversion() {
    use crate::{DataTernary, Ternary};

    let boundaries = [
        i64::MIN,
        -1_000_000,
        -243,
        -122,
        -121,
        -1,
        0,
        1,
        121,
        122,
        243,
        1_000_000,
        i64::MAX,
    ];

    for value in boundaries {
        let optimized = DataTernary::from_dec(value);
        let reference = DataTernary::from_ternary(Ternary::from_dec(value));
        assert_eq!(optimized, reference);
        assert_eq!(optimized.to_dec(), value);
    }
    for value in -10_000i64..=10_000 {
        assert_eq!(DataTernary::from_dec(value).to_dec(), value);
    }
}

#[test]
fn data_ternary_large_and_empty_values() {
    use crate::{DataTernary, Digit, Ternary};
    use alloc::string::ToString;

    let input = "+-0".repeat(100);
    let original = Ternary::parse(&input);
    let storage = DataTernary::from_ternary(original.clone());
    assert_eq!(storage.to_ternary(), original);
    assert_eq!(storage.to_string(), input);
    assert_eq!(storage.to_digits().len(), input.len());

    let empty = DataTernary::default();
    assert_eq!(empty.to_fixed_ternary().to_string(), "");
    assert_eq!(empty.to_ternary().to_string(), "0");
    assert_eq!(empty.to_digits(), vec![Digit::Zero]);
    assert_eq!(empty.to_string(), "");
    assert_eq!(empty.to_dec(), 0);
}

#[test]
fn chunk_conversions_exhaustive() {
    use crate::{Ternary, TernaryConversionError, TritsChunk};

    for value in i8::MIN..=i8::MAX {
        let expected = if (-121..=121).contains(&value) {
            Ok(value)
        } else {
            Err(TernaryConversionError::OutOfRange)
        };

        let ternary = Ternary::from_dec(value as i64).with_length(80);

        assert_eq!(
            TritsChunk::try_from(value).map(|chunk| chunk.to_dec()),
            expected
        );
        assert_eq!(
            TritsChunk::try_from(&ternary).map(|chunk| chunk.to_dec()),
            expected
        );
        assert_eq!(
            TritsChunk::try_from(ternary.clone()).map(|chunk| chunk.to_dec()),
            expected
        );
        assert_eq!(
            TritsChunk::checked_from_dec(value).map(|chunk| chunk.to_dec()),
            expected.ok()
        );
        assert_eq!(
            TritsChunk::checked_from_ternary(&ternary).map(|chunk| chunk.to_dec()),
            expected.ok()
        );

        if let Ok(value) = expected {
            assert_eq!(TritsChunk::from_ternary(ternary).to_dec(), value);
        }
    }
}

#[test]
fn bounded_storage_accepts_empty_and_padded_zero() {
    use crate::{Ter40, Ternary, TritsChunk};

    for input in ["", "0", "0000000000"] {
        let value = Ternary::parse(input);

        assert_eq!(TritsChunk::try_from(&value), Ok(TritsChunk::ZERO));
        assert_eq!(Ter40::try_from(&value), Ok(Ter40::ZERO));
    }
}

#[test]
fn ter40_conversion_traits_check_boundaries() {
    use crate::{Ter40, Ternary, TernaryConversionError};

    let min = Ter40::MIN.to_dec();
    let max = Ter40::MAX.to_dec();

    for value in [i64::MIN, min - 1, min, -1, 0, 1, max, max + 1, i64::MAX] {
        let expected = if (min..=max).contains(&value) {
            Ok(value)
        } else {
            Err(TernaryConversionError::OutOfRange)
        };

        let ternary = Ternary::from_dec(value).with_length(100);

        assert_eq!(
            Ter40::try_from(value).map(|number| number.to_dec()),
            expected
        );
        assert_eq!(
            Ter40::try_from(&ternary).map(|number| number.to_dec()),
            expected
        );
        assert_eq!(
            Ter40::try_from(ternary).map(|number| number.to_dec()),
            expected
        );
    }
}

#[test]
fn bounded_storage_rejects_arbitrary_precision_overflow() {
    use crate::{Ter40, Ternary, TernaryConversionError, TritsChunk};

    for symbol in ["+", "-"] {
        let value = Ternary::parse(&symbol.repeat(4096));

        assert_eq!(
            TritsChunk::try_from(&value),
            Err(TernaryConversionError::OutOfRange)
        );
        assert_eq!(
            Ter40::try_from(&value),
            Err(TernaryConversionError::OutOfRange)
        );
    }
}

#[test]
fn data_checked_conversion_from_decimal() {
    for value in [
        i64::MIN,
        i64::MIN + 1,
        -243,
        -122,
        -121,
        -1,
        0,
        1,
        121,
        122,
        243,
        i64::MAX - 1,
        i64::MAX,
    ] {
        let storage = DataTernary::from_dec(value);

        assert_eq!(storage.checked_to_dec(), Some(value));
        assert_eq!(i64::try_from(&storage), Ok(value));
        assert_eq!(i64::try_from(storage), Ok(value));
    }
}

#[test]
fn data_zero_storage_preserves_padded_width() {
    use crate::{Digit, Ternary};

    for length in [0usize, 1, 4, 5, 6, 10, 137] {
        let input = Ternary::parse(&"0".repeat(length));
        let storage = DataTernary::from_ternary(input);
        let expected_chunks = length / 5 + usize::from(length % 5 != 0);
        let expected_width = expected_chunks * 5;

        assert_eq!(storage.chunks.len(), expected_chunks);
        assert_eq!(storage.to_string(), "0".repeat(expected_width));
        assert_eq!(
            storage.to_fixed_ternary().to_digit_slice(),
            vec![Digit::Zero; expected_width].as_slice()
        );

        assert_eq!(storage.to_ternary().to_digit_slice(), &[Digit::Zero]);
        assert_eq!(storage.to_digits(), vec![Digit::Zero]);
        assert_eq!(storage.checked_to_dec(), Some(0));

        let restored = DataTernary::from_ternary(storage.to_fixed_ternary());

        // Compare the representation directly, independently of Eq semantics.
        assert_eq!(restored.chunks.as_slice(), storage.chunks.as_slice());
    }
}

#[test]
fn data_empty_numeric_and_fixed_round_trips() {
    let empty = DataTernary::default();
    let numeric_zero = DataTernary::from_dec(0);

    assert!(empty.chunks.is_empty());
    assert_eq!(numeric_zero.chunks.len(), 1);

    assert_eq!(empty.to_string(), "");
    assert_eq!(numeric_zero.to_string(), "00000");

    assert_eq!(empty.checked_to_dec(), Some(0));
    assert_eq!(numeric_zero.checked_to_dec(), Some(0));
    assert_eq!(i64::try_from(&empty), Ok(0));
    assert_eq!(i64::try_from(empty.clone()), Ok(0));

    let fixed_round_trip = DataTernary::from_ternary(empty.to_fixed_ternary());
    assert!(fixed_round_trip.chunks.is_empty());

    let numeric_round_trip = DataTernary::from_ternary(empty.to_ternary());
    assert_eq!(numeric_round_trip.chunks.len(), 1);
    assert_eq!(numeric_round_trip.to_string(), "00000");
}

#[test]
#[should_panic(expected = "Ternary value does not fit in i64")]
fn data_to_dec_panics_on_overflow() {
    let value = crate::Ternary::parse(&"+".repeat(100));
    let storage = DataTernary::from_ternary(value);

    let _ = storage.to_dec();
}

#[test]
fn data_equality_compares_stored_chunks() {
    use crate::Ternary;

    for suffix in ["+", "-", "+-0"] {
        let short = DataTernary::from_ternary(Ternary::parse(suffix));
        let same = DataTernary::from_ternary(Ternary::parse(suffix).with_length(5));
        let wider = DataTernary::from_ternary(Ternary::parse(suffix).with_length(6));

        assert_eq!(short, same);
        assert_ne!(short, wider);
        assert_eq!(short.to_ternary(), wider.to_ternary());

        let restored = DataTernary::from_ternary(wider.to_fixed_ternary());
        assert_eq!(restored, wider);
    }

    // Equal widths alone do not imply equal storage.
    assert_ne!(DataTernary::from_dec(1), DataTernary::from_dec(-1));
}

#[test]
fn data_zero_equality_preserves_storage_width() {
    use crate::Ternary;

    let empty = DataTernary::default();
    let one_chunk = DataTernary::from_dec(0);
    let two_chunks = DataTernary::from_ternary(Ternary::parse("000000"));

    assert_eq!(empty, DataTernary::from_ternary(Ternary::parse("")));

    for input in ["0", "000", "00000"] {
        assert_eq!(one_chunk, DataTernary::from_ternary(Ternary::parse(input)));
    }

    assert_ne!(empty, one_chunk);
    assert_ne!(empty, two_chunks);
    assert_ne!(one_chunk, two_chunks);

    for storage in [&empty, &one_chunk, &two_chunks] {
        assert_eq!(storage.checked_to_dec(), Some(0));
    }
}

#[test]
fn data_hash_collections_use_structural_equality() {
    extern crate std;

    use crate::Ternary;
    use std::collections::{HashMap, HashSet};

    let short = DataTernary::from_ternary(Ternary::parse("+"));
    let same = DataTernary::from_ternary(Ternary::parse("0000+"));
    let wider = DataTernary::from_ternary(Ternary::parse("00000+"));

    let mut set = HashSet::new();

    assert!(set.insert(short.clone()));
    assert!(!set.insert(same.clone()));
    assert!(set.insert(wider.clone()));
    assert!(set.insert(DataTernary::default()));
    assert!(set.insert(DataTernary::from_dec(0)));
    assert!(set.insert(DataTernary::from_ternary(Ternary::parse("000000"))));
    assert_eq!(set.len(), 5);

    let mut map = HashMap::new();

    assert_eq!(map.insert(short.clone(), 10), None);
    assert_eq!(map.insert(same, 20), Some(10));
    assert_eq!(map.insert(wider.clone(), 30), None);
    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&short), Some(&20));
    assert_eq!(map.get(&wider), Some(&30));
}
