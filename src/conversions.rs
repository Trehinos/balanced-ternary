//! This module provides conversions between `Digit` and `Ternary` types and common Rust types such as `char`, `&str`, and `String`.
//!
//! # Overview
//! The module defines `impl From` traits for seamless conversions:
//!
//! - `Digit` <-> `char`: Converts digits to and from their character representation.
//! - `Digit` <-> `i8`: Converts digits to and from their byte representation.
//! - `Ternary` <-> `&str` / `String`: Allows parsing and generating ternary numbers from strings.
//! - `Ternary` <-> `i64`: Converts ternary numbers from/to decimal numbers.
//!
//! The primary goal of these conversions is to simplify working with `Digit` and `Ternary` types by leveraging Rust's `From` and `Into` traits.

use crate::Digit;

#[cfg(feature = "ternary-string")]
use alloc::string::{String, ToString};

#[cfg(feature = "ternary-string")]
use crate::Ternary;

impl From<char> for Digit {
    fn from(value: char) -> Self {
        Self::from_char(value)
    }
}

impl From<i8> for Digit {
    fn from(value: i8) -> Self {
        Self::from_i8(value)
    }
}

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

#[cfg(feature = "ternary-string")]
impl From<&str> for Ternary {
    fn from(value: &str) -> Self {
        Self::parse(value)
    }
}

#[cfg(feature = "ternary-string")]
impl From<String> for Ternary {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
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
