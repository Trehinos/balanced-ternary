use crate::{Ternary, TernaryConversionError};

fn check_i64_conversion(value: Ternary, expected: Result<i64, TernaryConversionError>) {
    assert_eq!(i64::try_from(&value), expected);
    assert_eq!(i64::try_from(value.clone()), expected);

    #[cfg(feature = "ternary-store")]
    {
        let storage = crate::DataTernary::from_ternary(value);

        assert_eq!(storage.checked_to_dec(), expected.ok());
        assert_eq!(i64::try_from(&storage), expected);
        assert_eq!(i64::try_from(storage), expected);
    }
}

#[test]
fn i64_conversion_boundaries_and_padding() {
    for value in [
        i64::MIN,
        i64::MIN + 1,
        -244,
        -243,
        -122,
        -121,
        -1,
        0,
        1,
        121,
        122,
        243,
        244,
        i64::MAX - 1,
        i64::MAX,
    ] {
        let ternary = Ternary::from_dec(value);

        check_i64_conversion(ternary.clone(), Ok(value));
        check_i64_conversion(ternary.with_length(137), Ok(value));
    }
}

#[test]
fn i64_conversion_small_range() {
    for value in -1000i64..=1000 {
        check_i64_conversion(Ternary::from_dec(value), Ok(value));
    }
}

#[test]
fn i64_conversion_rejects_overflow() {
    let one = Ternary::from_dec(1);

    let above_max = &Ternary::from_dec(i64::MAX) + &one;
    let below_min = &Ternary::from_dec(i64::MIN) - &one;

    for value in [
        above_max,
        below_min,
        Ternary::parse(&"+".repeat(4096)),
        Ternary::parse(&"-".repeat(4096)),
    ] {
        check_i64_conversion(value.clone(), Err(TernaryConversionError::OutOfRange));
        check_i64_conversion(
            value.with_length(4200),
            Err(TernaryConversionError::OutOfRange),
        );
    }
}

#[test]
fn i64_conversion_accepts_empty_and_long_zero() {
    for length in [0, 1, 5, 6, 137, 4096] {
        check_i64_conversion(Ternary::parse(&"0".repeat(length)), Ok(0));
    }
}
