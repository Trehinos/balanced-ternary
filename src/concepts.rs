use crate::Digit;
use alloc::vec::Vec;

/// Digit-wise transformations on balanced ternary representations.
pub trait DigitOperate {
    /// Returns the stored digits in most-significant-first order.
    fn to_digits(&self) -> Vec<Digit>;

    /// Returns the digit at `index`, counting from the least significant trit at index zero.
    fn digit(&self, index: usize) -> Option<Digit>;

    /// Maps each digit through `f`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "ternary-string")]
    /// # {
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
    /// # }
    /// ```
    fn each(&self, f: impl Fn(Digit) -> Digit) -> Self
    where
        Self: Sized;

    /// Maps each digit `d` through `f(d, other)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "ternary-string")]
    /// # {
    /// use std::ops::Mul;
    /// use balanced_ternary::{Ternary, Digit};
    /// use balanced_ternary::concepts::DigitOperate;
    ///
    /// let original = Ternary::parse("+-0");
    /// let transformed = original.each_with(Digit::mul, Digit::Neg);
    /// assert_eq!(transformed.to_string(), "-+0");
    /// # }
    /// ```
    fn each_with(&self, f: impl Fn(Digit, Digit) -> Digit, other: Digit) -> Self
    where
        Self: Sized;

    /// Combines corresponding digits as `f(left, right)`. `Ternary` aligns the
    /// operands at the least significant digit and zero-extends the shorter one.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "ternary-string")]
    /// # {
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
    /// # }
    /// ```
    fn each_zip(&self, f: impl Fn(Digit, Digit) -> Digit, other: Self) -> Self
    where
        Self: Sized;

    /// Combines corresponding digits from least to most significant. The initial
    /// carry is zero. The callback receives `(left, right, carry)` and returns
    /// `(next_carry, result_digit)`.
    ///
    /// `Ternary` zero-extends the shorter input and prepends a nonzero final
    /// carry. `Tryte<N>` and `Ter40` retain their width and discard that carry.
    /// Use their inherent `each_zip_carry_full` method to retrieve it.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "ternary-string")]
    /// # {
    /// use balanced_ternary::{Digit, Ternary};
    /// use balanced_ternary::concepts::DigitOperate;
    ///
    /// let ternary1 = Ternary::parse("+-0");
    /// let ternary2 = Ternary::parse("-+0");
    ///
    /// // Transformation function that adds digits with a carry digit
    /// let combine = |d1: Digit, d2: Digit, carry: Digit| -> (Digit, Digit) {
    ///     let sum = d1.to_i8() + d2.to_i8() + carry.to_i8();
    ///
    ///     let next_carry = (sum + 1).div_euclid(3);
    ///     let result = sum - next_carry * 3;
    ///
    ///     (
    ///         Digit::from_i8(next_carry),
    ///         Digit::from_i8(result),
    ///     )
    /// };
    ///
    /// let result = ternary1.each_zip_carry(combine, ternary2.clone()).trim();
    /// assert_eq!(result.to_string(), (&ternary1 + &ternary2).to_string());
    /// # }
    /// ```
    fn each_zip_carry(
        &self,
        f: impl Fn(Digit, Digit, Digit) -> (Digit, Digit),
        other: Self,
    ) -> Self
    where
        Self: Sized;
}
