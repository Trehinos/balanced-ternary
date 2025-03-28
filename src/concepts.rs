use crate::Digit;
use alloc::vec::Vec;

pub trait DigitOperate {
    /// Returns every individual [Digit] of this [DigitOperate] object.
    fn to_digits(&self) -> Vec<Digit>;

    /// Returns one individual [Digit] ot this [DigitOperate] object if it exists.
    fn digit(&self, index: usize) -> Option<Digit>;

    /// Applies a transformation function to each digit of the balanced ternary number,
    /// returning a new `DigitOperate` object with the transformed digits.
    ///
    /// ### Arguments
    ///
    /// * `f` - A closure or function that takes a `Digit` and returns a transformed `Digit`.
    ///
    /// ### Returns
    ///
    /// * `Self` - A new `DigitOperate` object containing the transformed digits.
    ///
    /// ### Digit transformations
    ///
    /// These methods (unary operators) from the [Digit] type can be called directly.
    ///
    /// * Returns either `Pos` or `Neg`:
    ///     * [Digit::possibly]
    ///     * [Digit::necessary]
    ///     * [Digit::contingently]
    ///     * [Digit::ht_not]
    /// * Returns either `Zero` or `Pos` or `Neg`.
    ///     * [Digit::pre]
    ///     * [Digit::post]
    ///     * `Digit::not`
    ///     * `Digit::neg`
    ///     * [Digit::absolute_positive]
    ///     * [Digit::positive]
    ///     * [Digit::not_negative]
    ///     * [Digit::not_positive]
    ///     * [Digit::negative]
    ///     * [Digit::absolute_negative]
    ///
    /// ### Examples with [crate::Ternary]
    /// ```
    /// use balanced_ternary::{Ternary, Digit};
    /// use balanced_ternary::concepts::DigitOperate;
    ///
    /// let orig_ternary = Ternary::parse("+0-");
    /// let transformed = orig_ternary.each(Digit::necessary);
    /// assert_eq!(transformed.to_string(), "+--");
    /// let transformed = orig_ternary.each(Digit::positive);
    /// assert_eq!(transformed.to_string(), "+00");
    /// let transformed = orig_ternary.each(Digit::not_negative);
    /// assert_eq!(transformed.to_string(), "++0");
    /// let transformed = orig_ternary.each(Digit::absolute_negative);
    /// assert_eq!(transformed.to_string(), "-0-");
    /// ```
    fn each(&self, f: impl Fn(Digit) -> Digit) -> Self
    where
        Self: Sized;

    /// Applies a transformation function to each digit of the balanced ternary number,
    /// using an additional parameter for the transformation process, returning a new `DigitOperate`
    /// object with the transformed digits.
    ///
    /// ### Arguments
    ///
    /// * `f` - A closure or function that takes a `Digit` and an additional `Digit`,
    ///         and returns a transformed `Digit`.
    /// * `other` - An additional `Digit` to be passed to the transformation function `f`.
    ///
    /// ### Returns
    ///
    /// * `Self` - A new `DigitOperate` object containing the transformed digits.
    ///
    /// ### Digit transformations
    ///
    /// These methods (binary operators) from the [Digit] type can be called directly.
    ///
    /// * `Digit::add`
    /// * `Digit::sub`
    /// * `Digit::mul`
    /// * `Digit::div`
    /// * `Digit::bitand` (k3/l3 and)
    /// * [Digit::bi3_and]
    /// * `Digit::bitor`  (k3/l3 or)
    /// * [Digit::bi3_or]
    /// * `Digit::bitxor` (k3/l3 xor)
    /// * [Digit::k3_imply]
    /// * [Digit::k3_equiv]
    /// * [Digit::bi3_imply]
    /// * [Digit::l3_imply]
    /// * [Digit::rm3_imply]
    /// * [Digit::ht_imply]
    ///
    /// ### Examples with [crate::Ternary]
    /// ```
    /// use std::ops::Mul;
    /// use balanced_ternary::{Ternary, Digit};
    /// use balanced_ternary::concepts::DigitOperate;
    ///
    /// let original = Ternary::parse("+-0");
    /// let transformed = original.each_with(Digit::mul, Digit::Neg);
    /// assert_eq!(transformed.to_string(), "-+0");
    /// ```
    fn each_with(&self, f: impl Fn(Digit, Digit) -> Digit, other: Digit) -> Self
    where
        Self: Sized;

    /// Applies a transformation function to each digit of the balanced ternary number,
    /// along with a corresponding digit from another `DigitOperate` number.
    ///
    /// ### Arguments
    ///
    /// * `f` - A closure or function that takes two arguments:
    ///     * a `Digit` from the current `Ternary`,
    ///     * a `Digit` from the corresponding position in the `other` `Ternary`.
    ///     * The function must return a transformed `Digit`.
    /// * `other` - A `DigitOperate` object with digits to process alongside the digits of the current object.
    ///
    /// ### Returns
    ///
    /// * `Self` - A new `DigitOperate` object containing the transformed digits.
    ///
    /// ### Examples with [crate::Ternary]
    ///
    /// ```
    /// use std::ops::Mul;
    /// use balanced_ternary::{Ternary, Digit};
    /// use balanced_ternary::concepts::DigitOperate;
    ///
    /// let ternary1 = Ternary::parse("-+0-+0-+0");
    /// let ternary2 = Ternary::parse("---000+++");
    ///
    /// let result = ternary1.each_zip(Digit::mul, ternary2.clone());
    /// assert_eq!(result.to_string(), "+-0000-+0");
    ///
    /// let result = ternary1.each_zip(Digit::k3_imply, ternary2.clone());
    /// assert_eq!(result.to_string(), "+-0+00+++");
    /// let result = ternary1.each_zip(Digit::bi3_imply, ternary2.clone());
    /// assert_eq!(result.to_string(), "+-0000++0");
    /// let result = ternary1.each_zip(Digit::ht_imply, ternary2.clone());
    /// assert_eq!(result.to_string(), "+--+0++++");
    /// ```
    fn each_zip(&self, f: impl Fn(Digit, Digit) -> Digit, other: Self) -> Self
    where
        Self: Sized;

    /// Applies a transformation function to each digit of the balanced ternary number,
    /// along with a corresponding digit from another `DigitOperate` number, and a carry digit.
    ///
    /// ### Arguments
    ///
    /// * `f` - A closure or function that takes three arguments:
    ///     * a `Digit` from the current `DigitOperate`,
    ///     * a `Digit` from the corresponding position in the `other` `DigitOperate`,
    ///     * and the current carry `Digit`.
    ///     * The function must return a tuple containing `(carry: Digit, transformed: Digit)`.
    /// * `other` - A `DigitOperate` object with digits to process alongside the digits of the current object.
    ///
    /// ### Returns
    ///
    /// * `Self` - A new `DigitOperate` object containing the transformed digits.
    ///
    /// ### Examples with [crate::Ternary]
    ///
    /// ```
    /// use balanced_ternary::{Digit, Ternary};
    /// use balanced_ternary::concepts::DigitOperate;
    ///
    /// let ternary1 = Ternary::parse("+-0");
    /// let ternary2 = Ternary::parse("-+0");
    ///
    /// // Transformation function that adds digits with a carry digit
    /// let combine = |d1: Digit, d2: Digit, carry: Digit| -> (Digit, Digit) {
    ///     // Simple example operation: this just illustrates transforming with carry.
    ///     // Replace with meaningful logic as needed for your application.
    ///     let sum = d1.to_i8() + d2.to_i8() + carry.to_i8();
    ///     (Digit::from_i8(sum / 3), Digit::from_i8(sum % 3))
    /// };
    ///
    /// let result = ternary1.each_zip_carry(combine, ternary2.clone()).trim();
    /// assert_eq!(result.to_string(), (&ternary1 + &ternary2).to_string());
    /// ```
    fn each_zip_carry(
        &self,
        f: impl Fn(Digit, Digit, Digit) -> (Digit, Digit),
        other: Self,
    ) -> Self
    where
        Self: Sized;
}
