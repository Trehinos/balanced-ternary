use crate::Digit;
use alloc::vec::Vec;

pub trait DigitOperate {
    fn to_digits(&self) -> Vec<Digit>;
    fn digit(&self, index: usize) -> Option<Digit>;
    fn each(&self, f: impl Fn(Digit) -> Digit) -> Self
    where
        Self: Sized;
    fn each_with(&self, f: impl Fn(Digit, Digit) -> Digit, other: Digit) -> Self
    where
        Self: Sized;
    fn each_zip(&self, f: impl Fn(Digit, Digit) -> Digit, other: Self) -> Self
    where
        Self: Sized;
    fn each_zip_carry(&self, f: impl Fn(Digit, Digit, Digit) -> (Digit, Digit), other: Self) -> Self
    where
        Self: Sized;
}
