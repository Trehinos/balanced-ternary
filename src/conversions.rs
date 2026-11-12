//! Conversions between balanced ternary types and common Rust types.
//!
//! `TryFrom` provides fallible conversions from `char` and `i8` to `Digit`,
//! and from `&str` and `String` to `Ternary`.
//!
//! Conversions from `Digit` to `char` or `i8`, from `i64` to `Ternary`,
//! and from `Ternary` to `String` are infallible.
//!
//! Conversions from borrowed or owned `Ternary` values to `i64`
//! use `TryFrom` and return `TernaryConversionError::OutOfRange`
//! when the numeric value cannot be represented.

use crate::{Digit, DigitConversionError};

#[cfg(feature = "ternary-string")]
use alloc::string::{String, ToString};

#[cfg(feature = "ternary-string")]
use crate::{ParseTernaryError, Ternary, TernaryConversionError};

impl From<Digit> for char {
    fn from(value: Digit) -> Self {
        value.to_char()
    }
}

impl From<Digit> for i8 {
    fn from(value: Digit) -> Self {
        value.to_i8()
    }
}
impl TryFrom<char> for Digit {
    type Error = DigitConversionError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Self::try_from_char(value)
    }
}

impl TryFrom<i8> for Digit {
    type Error = DigitConversionError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        Self::try_from_i8(value)
    }
}

#[cfg(feature = "ternary-string")]
impl TryFrom<&str> for Ternary {
    type Error = ParseTernaryError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_parse(value)
    }
}

#[cfg(feature = "ternary-string")]
impl TryFrom<String> for Ternary {
    type Error = ParseTernaryError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_parse(value.as_str())
    }
}

#[cfg(feature = "ternary-string")]
impl From<i64> for Ternary {
    fn from(value: i64) -> Self {
        Self::from_dec(value)
    }
}

#[cfg(feature = "ternary-string")]
impl From<Ternary> for String {
    fn from(value: Ternary) -> Self {
        value.to_string()
    }
}

#[cfg(feature = "ternary-string")]
impl TryFrom<&Ternary> for i64 {
    type Error = TernaryConversionError;
    fn try_from(value: &Ternary) -> Result<Self, Self::Error> {
        value
            .checked_to_dec()
            .ok_or(TernaryConversionError::OutOfRange)
    }
}

#[cfg(feature = "ternary-string")]
impl TryFrom<Ternary> for i64 {
    type Error = TernaryConversionError;
    fn try_from(value: Ternary) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

#[cfg(all(test, feature = "ternary-string"))]
mod tests {
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
}
