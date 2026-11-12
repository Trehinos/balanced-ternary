//! Conversions between balanced ternary types and common Rust types.
//!
//! `TryFrom` provides fallible conversions from `char` and `i8` to `Digit`,
//! and from `&str` and `String` to `Ternary`.
//!
//! Conversions from `Digit` to `char` or `i8`, from `i64` to `Ternary`,
//! and from `Ternary` to `String` are infallible.
//!
//! The legacy `From<Ternary> for i64` conversion still panics on overflow.
//! Use `Ternary::checked_to_dec()` when the value may exceed the i64 range.

use crate::{DigitConversionError, Digit};

#[cfg(feature = "ternary-string")]
use alloc::string::{String, ToString};

#[cfg(feature = "ternary-string")]
use crate::{ParseTernaryError, Ternary};

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
impl From<Ternary> for i64 {
    fn from(value: Ternary) -> Self {
        value.to_dec()
    }
}
