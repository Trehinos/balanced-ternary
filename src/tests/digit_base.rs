use crate::Digit::{Neg, Pos, Zero};
use crate::{Digit, DigitConversionError};

#[test]
fn subtraction_modulo_three() {
    let cases = [
        (Neg, Neg, Zero),
        (Neg, Zero, Neg),
        (Neg, Pos, Pos),
        (Zero, Neg, Pos),
        (Zero, Zero, Zero),
        (Zero, Pos, Neg),
        (Pos, Neg, Neg),
        (Pos, Zero, Pos),
        (Pos, Pos, Zero),
    ];

    for (a, b, expected) in cases {
        assert_eq!(a - b, expected, "{a:?} - {b:?}");
    }
}

#[test]
fn try_from_char_valid_and_invalid() {
    assert_eq!(Digit::try_from_char('-'), Ok(Neg));
    assert_eq!(Digit::try_from_char('0'), Ok(Zero));
    assert_eq!(Digit::try_from_char('+'), Ok(Pos));
    assert_eq!(
        Digit::try_from_char('x'),
        Err(DigitConversionError::InvalidCharacter('x'))
    );

    assert_eq!(Digit::from_char('-'), Neg);
    assert_eq!(Digit::from_char('0'), Zero);
    assert_eq!(Digit::from_char('+'), Pos);
}

#[test]
#[should_panic(expected = "Invalid value. Expected '-', '0' or '+'.")]
fn from_char_panic() {
    Digit::from_char('1');
}

#[test]
fn try_from_char_theta_valid_and_invalid() {
    assert_eq!(Digit::try_from_char_theta('Θ'), Ok(Neg));
    assert_eq!(Digit::try_from_char_theta('0'), Ok(Zero));
    assert_eq!(Digit::try_from_char_theta('1'), Ok(Pos));
    assert_eq!(
        Digit::try_from_char_theta('x'),
        Err(DigitConversionError::InvalidCharacter('x'))
    );

    assert_eq!(Digit::from_char_theta('Θ'), Neg);
    assert_eq!(Digit::from_char_theta('0'), Zero);
    assert_eq!(Digit::from_char_theta('1'), Pos);
}

#[test]
#[should_panic(expected = "Invalid value. Expected 'Θ', '0', or '1'.")]
fn from_char_theta_panic() {
    Digit::from_char_theta('-');
}

#[test]
fn try_from_char_z_valid_and_invalid() {
    assert_eq!(Digit::try_from_char_z('Z'), Ok(Neg));
    assert_eq!(Digit::try_from_char_z('0'), Ok(Zero));
    assert_eq!(Digit::try_from_char_z('1'), Ok(Pos));
    assert_eq!(
        Digit::try_from_char_z('x'),
        Err(DigitConversionError::InvalidCharacter('x'))
    );

    assert_eq!(Digit::from_char_z('Z'), Neg);
    assert_eq!(Digit::from_char_z('0'), Zero);
    assert_eq!(Digit::from_char_z('1'), Pos);
}

#[test]
#[should_panic(expected = "Invalid value. Expected 'Z', '0', or '1'.")]
fn from_char_z_panic() {
    Digit::from_char_z('-');
}

#[test]
fn try_from_char_t_valid_and_invalid() {
    assert_eq!(Digit::try_from_char_t('T'), Ok(Neg));
    assert_eq!(Digit::try_from_char_t('0'), Ok(Zero));
    assert_eq!(Digit::try_from_char_t('1'), Ok(Pos));
    assert_eq!(
        Digit::try_from_char_t('x'),
        Err(DigitConversionError::InvalidCharacter('x'))
    );

    assert_eq!(Digit::from_char_t('T'), Neg);
    assert_eq!(Digit::from_char_t('0'), Zero);
    assert_eq!(Digit::from_char_t('1'), Pos);
}

#[test]
#[should_panic(expected = "Invalid value. Expected 'T', '0', or '1'.")]
fn from_char_t_panic() {
    Digit::from_char_t('-');
}
#[test]
fn digit_character_conversions() {
    use crate::{Digit, DigitConversionError};

    for (character, expected) in [('-', Neg), ('0', Zero), ('+', Pos)] {
        assert_eq!(Digit::try_from_char(character), Ok(expected));
        assert_eq!(Digit::try_from(character), Ok(expected));
        assert_eq!(Digit::from_char(character), expected);
        assert_eq!(char::from(expected), character);
    }

    for character in ['1', '2', 'x', ' ', '\n', '\0', '−', 'Θ', '🙂'] {
        let expected = Err(DigitConversionError::InvalidCharacter(character));

        assert_eq!(Digit::try_from_char(character), expected);
        assert_eq!(Digit::try_from(character), expected);
    }
}

#[test]
fn digit_integer_conversions_exhaustive() {
    use crate::{Digit, DigitConversionError};

    for value in i8::MIN..=i8::MAX {
        let expected = match value {
            -1 => Ok(Neg),
            0 => Ok(Zero),
            1 => Ok(Pos),
            _ => Err(DigitConversionError::InvalidValue(value)),
        };

        assert_eq!(Digit::try_from_i8(value), expected);
        assert_eq!(Digit::try_from(value), expected);

        if let Ok(digit) = expected {
            assert_eq!(Digit::from_i8(value), digit);
            assert_eq!(i8::from(digit), value);
        }
    }
}

#[test]
fn digit_conversions_are_const() {
    use crate::{Digit, DigitConversionError};

    const POSITIVE: Result<Digit, DigitConversionError> = Digit::try_from_char('+');
    const NEGATIVE: Result<Digit, DigitConversionError> = Digit::try_from_i8(-1);
    const INVALID: Result<Digit, DigitConversionError> = Digit::try_from_i8(2);
    const ZERO_FROM_CHAR: Digit = Digit::from_char('0');
    const ZERO_FROM_INTEGER: Digit = Digit::from_i8(0);

    assert_eq!(POSITIVE, Ok(Pos));
    assert_eq!(NEGATIVE, Ok(Neg));
    assert_eq!(INVALID, Err(DigitConversionError::InvalidValue(2)));
    assert_eq!(ZERO_FROM_CHAR, Zero);
    assert_eq!(ZERO_FROM_INTEGER, Zero);
}

#[test]
#[should_panic(expected = "Invalid value")]
fn digit_from_char_rejects_invalid_input() {
    let _ = crate::Digit::from_char('x');
}

#[test]
#[should_panic(expected = "Invalid value")]
fn digit_from_i8_rejects_invalid_input() {
    let _ = crate::Digit::from_i8(2);
}
