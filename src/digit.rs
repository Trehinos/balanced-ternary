//! ## Module: Balanced Ternary `Digit`
//!
//! This module defines the `Digit` type for the balanced ternary numeral system,
//! along with its associated operations and functionality.
//!
//! ### Key Features
//!
//! - **`Digit` Type**: Represents a digit in the balanced ternary numeral system.
//!     - Possible values: `Neg` (-1), `Zero` (0), `Pos` (+1).
//!     - Provides utility functions for converting between characters, integers, and other formats.
//! - **Arithmetic Operators**: Implements arithmetic operations for digits, including:
//!     - Negation (`Neg`) and Bitwise Not (`Not`).
//!     - Addition (`Add`) and Subtraction (`Sub`).
//!     - Multiplication (`Mul`) and Division (`Div`), with safe handling of divisors (division by zero panics).
//! - **Logical Operators**: Supports bitwise logical operations (AND, OR, XOR) based on ternary logic rules.
//! - **Custom Methods**: Additional utility methods implementing balanced ternary logic principles.
//!
//! ### Supported Use Cases
//!
//! - Arithmetic in balanced ternary numeral systems.
//! - Logic operations in custom numeral systems.
//! - Conversion between balanced ternary representation and more common formats like integers and characters.
//!
//! ## `Digit` type arithmetical and logical operations
//!
//! - `Neg` and `Not` for `Digit`: Negates the digit value, adhering to balanced ternary rules.
//! - `Add<Digit>` for `Digit`: Adds two `Digit` values and returns a `Ternary`.
//! - `Sub<Digit>` for `Digit`: Subtracts one `Digit` from another and returns a `Ternary`.
//! - `Mul<Digit>` for `Digit`: Multiplies two `Digit` values and returns a `Digit`.
//! - `Div<Digit>` for `Digit`: Divides one `Digit` by another and returns a `Digit`. Division by zero panics.
//!
//! ### Logical Operations for `Digit`
//!
//! The `Digit` type supports bitwise logical operations, which are implemented according to logical rules applicable to balanced ternary digits.
//!
//! ### Digits operators
//!
//! #### Unary operators
//!
//! These operators can be applied for `Ternary` with `Ternary::each(operator)`:
//!
//! | Unary operators       | - | 0 | + |
//! |-----------------------|---|---|---|
//! | possibly              | - | + | + |
//! | necessary             | - | - | + |
//! | contingently          | - | + | - |
//! | ht_not                | + | - | - |
//! | post                  | 0 | + | - |
//! | pre                   | + | - | 0 |
//! | `!` (not) / `-` (neg) | + | 0 | - |
//! | absolute_positive     | + | 0 | + |
//! | positive              | 0 | 0 | + |
//! | not_negative          | 0 | + | + |
//! | not_positive          | - | - | 0 |
//! | negative              | - | 0 | 0 |
//! | absolute_negative     | - | 0 | - |
//!
//! #### Binary operators
//!
//! These operators can be applied for `Ternary` with:
//!
//! - `Ternary::each_with(operator, with)`, or,
//! - `Ternary::each_zip(operator, other)`:
//!
//! | Binary operators | -<br>- | -<br>0 | -<br>+ | 0<br>- | 0<br>0 | 0<br>+ | +<br>- | +<br>0 | +<br>+ |
//! |------------------|--------|--------|--------|--------|--------|--------|--------|--------|--------|
//! | `+` (add)        | -+     | -      | 0      | -      | 0      | +      | 0      | +      | +-     |
//! | `-` (sub)        | 0      | -      | -+     | +      | 0      | -      | +-     | +      | 0      |
//! | `/` (div)        | +      |        | -      | 0      |        | 0      | -      |        | +      |
//! | `*` (mul)        | +      | 0      | -      | 0      | 0      | 0      | -      | 0      | +      |
//! | `&` (bitand)     | -      | -      | -      | -      | 0      | 0      | -      | 0      | +      |
//! | bi3_and          | -      | 0      | -      | 0      | 0      | 0      | -      | 0      | +      |
//! | `\|` (bitor)     | -      | 0      | +      | 0      | 0      | +      | +      | +      | +      |
//! | bi3_or           | -      | 0      | +      | 0      | 0      | 0      | +      | 0      | +      |
//! | `^` (bitxor)     | -      | 0      | +      | 0      | 0      | 0      | +      | 0      | -      |
//! | k3_equiv         | +      | 0      | -      | 0      | 0      | 0      | -      | 0      | +      |
//! | k3_imply         | +      | +      | +      | 0      | 0      | +      | -      | 0      | +      |
//! | bi3_imply        | +      | 0      | +      | 0      | 0      | 0      | -      | 0      | +      |
//! | l3_imply         | +      | +      | +      | 0      | +      | +      | -      | 0      | +      |
//! | rm3_imply        | +      | +      | +      | -      | 0      | +      | -      | -      | +      |
//! | para_imply       | +      | +      | +      | -      | 0      | +      | -      | 0      | +      |
//! | ht_imply         | +      | +      | +      | -      | +      | +      | -      | 0      | +      |

use crate::Ternary;
use alloc::vec;
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Sub};

/// Represents a digit in the balanced ternary numeral system.
///
/// A digit can have one of three values:
/// - `Neg` (-1): Represents the value -1 in the balanced ternary system.
/// - `Zero` (0): Represents the value 0 in the balanced ternary system.
/// - `Pos` (+1): Represents the value +1 in the balanced ternary system.
///
/// Provides utility functions for converting to/from characters, integers, and negation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Digit {
    /// Represents -1
    Neg,
    /// Represents 0
    Zero,
    /// Represents +1
    Pos,
}

impl Digit {
    /// Converts the `Digit` into its character representation.
    ///
    /// - Returns:
    ///     - `-` for `Digit::Neg`
    ///     - `0` for `Digit::Zero`
    ///     - `+` for `Digit::Pos`
    pub const fn to_char(&self) -> char {
        match self {
            Digit::Neg => '-',
            Digit::Zero => '0',
            Digit::Pos => '+',
        }
    }

    /// Creates a `Digit` from its character representation.
    ///
    /// - Accepts:
    ///     - `-` for `Digit::Neg`
    ///     - `0` for `Digit::Zero`
    ///     - `+` for `Digit::Pos`
    /// - Panics if the input character is invalid.
    pub const fn from_char(c: char) -> Digit {
        match c {
            '-' => Digit::Neg,
            '0' => Digit::Zero,
            '+' => Digit::Pos,
            _ => panic!("Invalid value. A Ternary must be either -, 0 or +."),
        }
    }

    /// Converts the `Digit` into its integer representation.
    ///
    /// - Returns:
    ///     - -1 for `Digit::Neg`
    ///     - 0 for `Digit::Zero`
    ///     - 1 for `Digit::Pos`
    pub const fn to_i8(&self) -> i8 {
        match self {
            Digit::Neg => -1,
            Digit::Zero => 0,
            Digit::Pos => 1,
        }
    }

    /// Creates a `Digit` from its integer representation.
    ///
    /// - Accepts:
    ///     - -1 for `Digit::Neg`
    ///     - 0 for `Digit::Zero`
    ///     - 1 for `Digit::Pos`
    /// - Panics if the input integer is invalid.
    pub const fn from_i8(i: i8) -> Digit {
        match i {
            -1 => Digit::Neg,
            0 => Digit::Zero,
            1 => Digit::Pos,
            _ => panic!("Invalid value. A Ternary must be either -1, 0 or +1."),
        }
    }

    /// Returns the corresponding possible value of the current `Digit`.
    ///
    /// - Returns:
    ///     - `Digit::Neg` for `Digit::Neg`
    ///     - `Digit::Pos` for `Digit::Zero`
    ///     - `Digit::Pos` for `Digit::Pos`
    pub const fn possibly(self) -> Self {
        match self {
            Digit::Neg => Digit::Neg,
            Digit::Zero => Digit::Pos,
            Digit::Pos => Digit::Pos,
        }
    }

    /// Determines the condition of necessity for the current `Digit`.
    ///
    /// - Returns:
    ///     - `Digit::Neg` for `Digit::Neg`
    ///     - `Digit::Neg` for `Digit::Zero`
    ///     - `Digit::Pos` for `Digit::Pos`
    ///
    /// This method is used to calculate necessity as part
    /// of balanced ternary logic systems.
    pub const fn necessary(self) -> Self {
        match self {
            Digit::Neg => Digit::Neg,
            Digit::Zero => Digit::Neg,
            Digit::Pos => Digit::Pos,
        }
    }

    /// Determines the condition of contingency for the current `Digit`.
    ///
    /// - Returns:
    ///     - `Digit::Neg` for `Digit::Neg`
    ///     - `Digit::Pos` for `Digit::Zero`
    ///     - `Digit::Neg` for `Digit::Pos`
    ///
    /// This method represents contingency in balanced ternary logic,
    /// which defines the specific alternation of `Digit` values.
    pub const fn contingently(self) -> Self {
        match self {
            Digit::Neg => Digit::Neg,
            Digit::Zero => Digit::Pos,
            Digit::Pos => Digit::Neg,
        }
    }

    /// Returns the absolute positive value of the current `Digit`.
    ///
    /// - Returns:
    ///     - `Digit::Pos` for `Digit::Neg`
    ///     - `Digit::Zero` for `Digit::Zero`
    ///     - `Digit::Pos` for `Digit::Pos`
    pub const fn absolute_positive(self) -> Self {
        match self {
            Digit::Neg => Digit::Pos,
            Digit::Zero => Digit::Zero,
            Digit::Pos => Digit::Pos,
        }
    }

    /// Determines the strictly positive condition for the current `Digit`.
    ///
    /// - Returns:
    ///     - `Digit::Zero` for `Digit::Neg`
    ///     - `Digit::Zero` for `Digit::Zero`
    ///     - `Digit::Pos` for `Digit::Pos`
    ///
    /// This method is used to calculate strictly positive states
    /// in association with ternary logic.
    pub const fn positive(self) -> Self {
        match self {
            Digit::Neg => Digit::Zero,
            Digit::Zero => Digit::Zero,
            Digit::Pos => Digit::Pos,
        }
    }

    /// Determines the condition of non-negativity for the current `Digit`.
    ///
    /// - Returns:
    ///     - `Digit::Zero` for `Digit::Neg`
    ///     - `Digit::Pos` for `Digit::Zero`
    ///     - `Digit::Pos` for `Digit::Pos`
    ///
    /// This method is used to filter out negative conditions
    /// in computations with balanced ternary representations.
    pub const fn not_negative(self) -> Self {
        match self {
            Digit::Neg => Digit::Zero,
            Digit::Zero => Digit::Pos,
            Digit::Pos => Digit::Pos,
        }
    }

    /// Determines the condition of non-positivity for the current `Digit`.
    ///
    /// - Returns:
    ///     - `Digit::Neg` for `Digit::Neg`
    ///     - `Digit::Neg` for `Digit::Zero`
    ///     - `Digit::Zero` for `Digit::Pos`
    ///
    /// This method complements the `positive` condition and captures
    /// states that are not strictly positive.
    pub const fn not_positive(self) -> Self {
        match self {
            Digit::Neg => Digit::Neg,
            Digit::Zero => Digit::Neg,
            Digit::Pos => Digit::Zero,
        }
    }

    /// Determines the strictly negative condition for the current `Digit`.
    ///
    /// - Returns:
    ///     - `Digit::Neg` for `Digit::Neg`
    ///     - `Digit::Zero` for `Digit::Zero`
    ///     - `Digit::Zero` for `Digit::Pos`
    ///
    /// This method calculates strictly negative states
    /// in association with ternary logic.
    pub const fn negative(self) -> Self {
        match self {
            Digit::Neg => Digit::Neg,
            Digit::Zero => Digit::Zero,
            Digit::Pos => Digit::Zero,
        }
    }

    /// Returns the absolute negative value of the current `Digit`.
    ///
    /// - Returns:
    ///     - `Digit::Neg` for `Digit::Neg`
    ///     - `Digit::Zero` for `Digit::Zero`
    ///     - `Digit::Neg` for `Digit::Pos`
    pub const fn absolute_negative(self) -> Self {
        match self {
            Digit::Neg => Digit::Neg,
            Digit::Zero => Digit::Zero,
            Digit::Pos => Digit::Neg,
        }
    }

    /// Performs Kleene implication with the current `Digit` as `self` and another `Digit`.
    ///
    /// - `self`: The antecedent of the implication.
    /// - `other`: The consequent of the implication.
    ///
    /// - Returns:
    ///     - `Digit::Pos` when `self` is `Digit::Neg`.
    ///     - The positive condition of `other` when `self` is `Digit::Zero`.
    ///     - `other` when `self` is `Digit::Pos`.
    ///
    /// Implements Kleene ternary implication logic, which includes
    /// determining the logical result based on antecedent and consequent.
    pub const fn k3_imply(self, other: Self) -> Self {
        match self {
            Digit::Neg => Digit::Pos,
            Digit::Zero => other.positive(),
            Digit::Pos => other,
        }
    }

    /// Apply a ternary equivalence operation for the current `Digit` and another `Digit`.
    ///
    /// - `self`: The first operand of the equivalence operation.
    /// - `other`: The second operand of the equivalence operation.
    ///
    /// - Returns:
    ///     - The negation of `other` when `self` is `Digit::Neg`.
    ///     - `Digit::Zero` when `self` is `Digit::Zero`.
    ///     - `other` when `self` is `Digit::Pos`.
    ///
    /// This method implements a ternary logic equivalence, which captures the relationship between
    /// two balanced ternary `Digit`s based on their logical equivalence.
    pub const fn k3_equiv(self, other: Self) -> Self {
        match self {
            Digit::Neg => match other {
                Digit::Neg => Digit::Pos,
                Digit::Zero => Digit::Zero,
                Digit::Pos => Digit::Neg,
            },
            Digit::Zero => Digit::Zero,
            Digit::Pos => other,
        }
    }

    /// Performs a ternary AND operation for the current `Digit` and another `Digit`.
    ///
    /// - `self`: The first operand of the AND operation.
    /// - `other`: The second operand of the AND operation.
    ///
    /// - Returns:
    ///     - `Digit::Neg` if `self` is `Digit::Neg` and `other` is not `Digit::Zero`.
    ///     - `Digit::Zero` if either `self` or `other` is `Digit::Zero`.
    ///     - `other` if `self` is `Digit::Pos`.
    ///
    /// This method implements Bochvar's internal three-valued logic in balanced ternary AND operation,
    /// which evaluates the logical conjunction of two `Digit`s in the ternary logic system.
    pub const fn bi3_and(self, other: Self) -> Self {
        match self {
            Digit::Neg => other.absolute_negative(),
            Digit::Zero => Digit::Zero,
            Digit::Pos => other,
        }
    }

    /// Performs a ternary OR operation for the current `Digit` and another `Digit`.
    ///
    /// - `self`: The first operand of the OR operation.
    /// - `other`: The second operand of the OR operation.
    ///
    /// - Returns:
    ///     - `other` if `self` is `Digit::Neg`.
    ///     - `Digit::Zero` if `self` is `Digit::Zero`.
    ///     - `Digit::Pos` if `self` is `Digit::Pos` and `other` is not `Digit::Zero`.
    ///
    /// This method implements Bochvar's three-valued internal ternary logic for the OR operation,
    /// determining the logical disjunction of two balanced ternary `Digit`s.
    pub const fn bi3_or(self, other: Self) -> Self {
        match self {
            Digit::Neg => other,
            Digit::Zero => Digit::Zero,
            Digit::Pos => other.absolute_positive(),
        }
    }

    /// Performs Bochvar's internal three-valued implication with the current `Digit` as `self`
    /// and another `Digit` as the consequent.
    ///
    /// - `self`: The antecedent of the implication.
    /// - `other`: The consequent of the implication.
    ///
    /// - Returns:
    ///     - `Digit::Zero` if `self` is `Digit::Neg` and `other` is `Digit::Zero`.
    ///     - `Digit::Pos` if `self` is `Digit::Neg` and `other` is not `Digit::Zero`.
    ///     - `Digit::Zero` if `self` is `Digit::Zero`.
    ///     - `other` if `self` is `Digit::Pos`.
    ///
    /// This method implements Bochvar's internal implication logic, which evaluates
    /// the logical consequence, between two balanced ternary `Digit`s in a manner
    /// consistent with three-valued logic principles.
    pub const fn bi3_imply(self, other: Self) -> Self {
        match self {
            Digit::Neg => other.absolute_positive(),
            Digit::Zero => Digit::Zero,
            Digit::Pos => other,
        }
    }

    /// Performs Łukasiewicz implication with the current `Digit` as `self` and another `Digit`.
    ///
    /// - `self`: The antecedent of the implication.
    /// - `other`: The consequent of the implication.
    ///
    /// - Returns:
    ///     - `Digit::Pos` when `self` is `Digit::Neg`.
    ///     - The non-negative condition of `other` when `self` is `Digit::Zero`.
    ///     - `other` when `self` is `Digit::Pos`.
    ///
    /// Implements Łukasiewicz ternary implication logic, which
    /// evaluates an alternative approach for implication compared to Kleene logic.
    pub const fn l3_imply(self, other: Self) -> Self {
        match self {
            Digit::Neg => Digit::Pos,
            Digit::Zero => other.not_negative(),
            Digit::Pos => other,
        }
    }

    /// Performs RM3 implication with the current `Digit` as `self` and another `Digit`.
    ///
    /// - `self`: The antecedent of the implication.
    /// - `other`: The consequent of the implication.
    ///
    /// - Returns:
    ///     - `Digit::Pos` when `self` is `Digit::Neg`.
    ///     - `other` when `self` is `Digit::Zero`.
    ///     - The necessary condition of `other` when `self` is `Digit::Pos`.
    ///
    /// Implements RM3 ternary implication logic, which defines a unique
    /// perspective for implication operations in balanced ternary systems.
    pub const fn rm3_imply(self, other: Self) -> Self {
        match self {
            Digit::Neg => Digit::Pos,
            Digit::Zero => other,
            Digit::Pos => other.necessary(),
        }
    }

    /// Performs the paraconsistent-logic implication with the current `Digit` as `self` and another `Digit`.
    ///
    /// - `self`: The antecedent of the implication.
    /// - `other`: The consequent of the implication.
    ///
    /// - Returns:
    ///     - `Digit::Pos` when `self` is `Digit::Neg`.
    ///     - `other` otherwise.
    pub const fn para_imply(self, other: Self) -> Self {
        match self {
            Digit::Neg => Digit::Pos,
            _ => other,
        }
    }

    /// Performs HT implication with the current `Digit` as `self` and another `Digit`.
    ///
    /// - `self`: The antecedent of the implication.
    /// - `other`: The consequent of the implication.
    ///
    /// - Returns:
    ///     - `Digit::Pos` when `self` is `Digit::Neg`.
    ///     - The possibility condition of `other` when `self` is `Digit::Zero`.
    ///     - `other` when `self` is `Digit::Pos`.
    ///
    /// This method computes HT ternary implication based on heuristic logic.
    pub const fn ht_imply(self, other: Self) -> Self {
        match self {
            Digit::Neg => Digit::Pos,
            Digit::Zero => other.possibly(),
            Digit::Pos => other,
        }
    }

    /// Performs HT logical negation of the current `Digit`.
    ///
    /// - Returns:
    ///     - `Digit::Pos` when `self` is `Digit::Neg`.
    ///     - `Digit::Neg` when `self` is `Digit::Zero` or `Digit::Pos`.
    ///
    /// This method evaluates the HT negation result using heuristic ternary logic.
    pub const fn ht_not(self) -> Self {
        match self {
            Digit::Neg => Digit::Pos,
            Digit::Zero => Digit::Neg,
            Digit::Pos => Digit::Neg,
        }
    }

    /// Converts the `Digit` to a `bool` in HT logic.
    ///
    /// - Returns:
    ///     - `true` when `self` is `Digit::Pos`.
    ///     - `false` when `self` is `Digit::Neg`.
    ///
    /// - Panics:
    ///     - Panics if `self` is `Digit::Zero`, as `Digit::Zero` cannot be directly
    ///       converted to a boolean value.
    ///       > To ensure `Pos` or `Neg` value, use one of :
    ///       > * [Digit::possibly]
    ///       > * [Digit::necessary]
    ///       > * [Digit::contingently]
    ///       > * [Digit::ht_not]
    ///
    pub const fn ht_bool(self) -> bool {
        match self {
            Digit::Neg => false,
            Digit::Zero => panic!(
                "Cannot convert a Digit::Zero to a bool. \
                 Use Digit::possibly()->to_bool() or Digit::necessary()->to_bool() instead."
            ),
            Digit::Pos => true,
        }
    }

    /// Performs Post's negation of the current `Digit`.
    ///
    /// - Returns:
    ///     - `Digit::Zero` when `self` is `Digit::Neg`.
    ///     - `Digit::Pos` when `self` is `Digit::Zero`.
    ///     - `Digit::Neg` when `self` is `Digit::Pos`.
    ///
    /// This method evaluates the negation based on Post's logic in ternary systems,
    /// which differs from standard negation logic.
    pub const fn post(self) -> Self {
        match self {
            Digit::Neg => Digit::Zero,
            Digit::Zero => Digit::Pos,
            Digit::Pos => Digit::Neg,
        }
    }

    /// Performs the inverse operation from the Post's negation of the current `Digit`.
    ///
    /// - Returns:
    ///     - `Digit::Pos` when `self` is `Digit::Neg`.
    ///     - `Digit::Neg` when `self` is `Digit::Zero`.
    ///     - `Digit::Zero` when `self` is `Digit::Pos`.
    pub const fn pre(self) -> Self {
        match self {
            Digit::Neg => Digit::Pos,
            Digit::Zero => Digit::Neg,
            Digit::Pos => Digit::Zero,
        }
    }

    /// This method maps this `Digit` value to its corresponding unbalanced ternary
    /// integer representation.
    ///
    /// - Returns:
    ///     - `0` for `Digit::Neg`.
    ///     - `1` for `Digit::Zero`.
    ///     - `2` for `Digit::Pos`.
    ///
    pub const fn to_unbalanced(self) -> u8 {
        match self {
            Digit::Neg => 0,
            Digit::Zero => 1,
            Digit::Pos => 2,
        }
    }

    /// Creates a `Digit` from an unbalanced ternary integer representation.
    ///
    /// # Arguments:
    /// - `u`: An unsigned 8-bit integer representing an unbalanced ternary value.
    ///
    /// # Returns:
    /// - `Digit::Neg` for `0`.
    /// - `Digit::Zero` for `1`.
    /// - `Digit::Pos` for `2`.
    ///
    /// # Panics:
    /// - Panics if the provided value is not `0`, `1`, or `2`, as these are the
    ///   only valid representations of unbalanced ternary values.
    pub const fn from_unbalanced(u: u8) -> Digit {
        match u {
            0 => Digit::Neg,
            1 => Digit::Zero,
            2 => Digit::Pos,
            _ => panic!("Invalid value. A unbalanced ternary value must be either 0, 1 or 2."),
        }
    }

    /// Increments the `Digit` value and returns a `Ternary` result.
    ///
    /// - The rules for incrementing are based on ternary arithmetic:
    ///   - For `Digit::Neg`:
    ///     - Incrementing results in `Digit::Zero` (`Ternary::parse("0")`).
    ///   - For `Digit::Zero`:
    ///     - Incrementing results in `Digit::Pos` (`Ternary::parse("+")`).
    ///   - For `Digit::Pos`:
    ///     - Incrementing results in "overflow" (`Ternary::parse("+-")`).
    ///
    /// - Returns:
    ///   - A `Ternary` instance representing the result of the increment operation.
    pub fn inc(self) -> Ternary {
        match self {
            Digit::Neg => Ternary::parse("0"),
            Digit::Zero => Ternary::parse("+"),
            Digit::Pos => Ternary::parse("+-"),
        }
    }

    /// Decrements the `Digit` value and returns a `Ternary` result.
    ///
    /// - The rules for decrementing are based on ternary arithmetic:
    ///   - For `Digit::Neg`:
    ///     - Decrementing results in "underflow" (`Ternary::parse("-+")`).
    ///   - For `Digit::Zero`:
    ///     - Decrementing results in `Digit::Neg` (`Ternary::parse("-")`).
    ///   - For `Digit::Pos`:
    ///     - Decrementing results in `Digit::Zero` (`Ternary::parse("0")`).
    ///
    /// - Returns:
    ///   - A `Ternary` instance representing the result of the decrement operation.
    pub fn dec(self) -> Ternary {
        match self {
            Digit::Neg => Ternary::parse("-+"),
            Digit::Zero => Ternary::parse("-"),
            Digit::Pos => Ternary::parse("0"),
        }
    }
}

impl Neg for Digit {
    type Output = Self;

    /// Returns the negation of the `Digit`.
    ///
    /// - `Digit::Neg` becomes `Digit::Pos`
    /// - `Digit::Pos` becomes `Digit::Neg`
    /// - `Digit::Zero` remains `Digit::Zero`
    fn neg(self) -> Self::Output {
        match self {
            Digit::Neg => Digit::Pos,
            Digit::Zero => Digit::Zero,
            Digit::Pos => Digit::Neg,
        }
    }
}

impl Not for Digit {
    type Output = Self;
    fn not(self) -> Self::Output {
        -self
    }
}

impl Add<Digit> for Digit {
    type Output = Ternary;

    /// Adds two `Digit` values together and returns a `Ternary` result.
    ///
    /// - The rules for addition are based on ternary arithmetic:
    ///   - For `Digit::Neg`:
    ///     - Adding `Digit::Neg` results in "underflow" (`Ternary::parse("-+")`).
    ///     - Adding `Digit::Zero` keeps the result as `Digit::Neg` (`Ternary::parse("-")`).
    ///     - Adding `Digit::Pos` results in a balance (`Ternary::parse("0")`).
    ///   - For `Digit::Zero`:
    ///     - Simply returns the other operand wrapped in a `Ternary` object.
    ///   - For `Digit::Pos`:
    ///     - Adding `Digit::Neg` results in balance (`Ternary::parse("0")`).
    ///     - Adding `Digit::Zero` keeps the result as `Digit::Pos` (`Ternary::parse("+")`).
    ///     - Adding `Digit::Pos` results in "overflow" (`Ternary::parse("+-")`).
    ///
    /// - Returns:
    ///   - A `Ternary` instance that holds the result of the addition.
    ///
    /// - Panics:
    ///   - This method does not panic under any circumstances.
    fn add(self, other: Digit) -> Self::Output {
        match self {
            Digit::Neg => other.dec(),
            Digit::Zero => Ternary::new(vec![other]),
            Digit::Pos => other.inc(),
        }
    }
}

impl Sub<Digit> for Digit {
    type Output = Ternary;

    /// Subtracts two `Digit` values and returns a `Ternary` result.
    ///
    /// - The rules for subtraction are based on ternary arithmetic:
    ///   - For `Digit::Neg`:
    ///     - Subtracting `Digit::Neg` results in balance (`Ternary::parse("0")`).
    ///     - Subtracting `Digit::Zero` keeps the result as `Digit::Neg` (`Ternary::parse("-")`).
    ///     - Subtracting `Digit::Pos` results in "underflow" (`Ternary::parse("-+")`).
    ///   - For `Digit::Zero`:
    ///     - Simply negates the other operand and returns it wrapped in a `Ternary` object.
    ///   - For `Digit::Pos`:
    ///     - Subtracting `Digit::Neg` results in "overflow" (`Ternary::parse("+-")`).
    ///     - Subtracting `Digit::Zero` keeps the result as `Digit::Pos` (`Ternary::parse("+")`).
    ///     - Subtracting `Digit::Pos` results in balance (`Ternary::parse("0")`).
    ///
    /// - Returns:
    ///   - A `Ternary` instance that holds the result of the subtraction.
    ///
    /// - Panics:
    ///   - This method does not panic under any circumstances.
    fn sub(self, other: Digit) -> Self::Output {
        match self {
            Digit::Neg => other.inc(),
            Digit::Zero => Ternary::new(vec![-other]),
            Digit::Pos => other.dec(),
        }
    }
}

impl Mul<Digit> for Digit {
    type Output = Digit;

    /// Multiplies two `Digit` values together and returns the product as a `Digit`.
    ///
    /// - The rules for multiplication in this implementation are straightforward:
    ///   - `Digit::Neg` multiplied by:
    ///     - `Digit::Neg` results in `Digit::Pos`.
    ///     - `Digit::Zero` results in `Digit::Zero`.
    ///     - `Digit::Pos` results in `Digit::Neg`.
    ///   - `Digit::Zero` multiplied by any `Digit` always results in `Digit::Zero`.
    ///   - `Digit::Pos` multiplied by:
    ///     - `Digit::Neg` results in `Digit::Neg`.
    ///     - `Digit::Zero` results in `Digit::Zero`.
    ///     - `Digit::Pos` results in `Digit::Pos`.
    ///
    /// - Returns:
    ///   - A `Digit` instance representing the result of the multiplication.
    fn mul(self, other: Digit) -> Self::Output {
        match self {
            Digit::Neg => -other,
            Digit::Zero => Digit::Zero,
            Digit::Pos => other,
        }
    }
}

impl Div<Digit> for Digit {
    type Output = Digit;

    /// Divides one `Digit` value by another and returns the result as a `Digit`.
    ///
    /// # Rules for division:
    /// - For `Digit::Neg`:
    ///   - Dividing `Digit::Neg` by `Digit::Neg` results in `Digit::Pos`.
    ///   - Dividing `Digit::Neg` by `Digit::Zero` will panic with "Cannot divide by zero."
    ///   - Dividing `Digit::Neg` by `Digit::Pos` results in `Digit::Neg`.
    /// - For `Digit::Zero`:
    ///   - Dividing `Digit::Zero` by `Digit::Neg` results in `Digit::Zero`.
    ///   - Dividing `Digit::Zero` by `Digit::Zero` will panic with "Cannot divide by zero."
    ///   - Dividing `Digit::Zero` by `Digit::Pos` results in `Digit::Zero`.
    /// - For `Digit::Pos`:
    ///   - Dividing `Digit::Pos` by `Digit::Neg` results in `Digit::Neg`.
    ///   - Dividing `Digit::Pos` by `Digit::Zero` will panic with "Cannot divide by zero."
    ///   - Dividing `Digit::Pos` by `Digit::Pos` results in `Digit::Pos`.
    ///
    /// # Returns:
    /// - A `Digit` value representing the result of the division.
    ///
    /// # Panics:
    /// - Panics with "Cannot divide by zero." if the `other` operand is `Digit::Zero`.
    fn div(self, other: Digit) -> Self::Output {
        if other == Digit::Zero {
            panic!("Cannot divide by zero.");
        }
        self * other
    }
}

impl BitAnd for Digit {
    type Output = Self;

    /// Performs a bitwise AND operation between two `Digit` values and returns the result.
    ///
    /// - The rules for the bitwise AND (`&`) operation are:
    ///   - If `self` is `Digit::Neg`, the result is always `Digit::Neg`.
    ///   - If `self` is `Digit::Zero`, the result depends on the value of `other`:
    ///     - `Digit::Neg` results in `Digit::Neg`.
    ///     - Otherwise, the result is `Digit::Zero`.
    ///   - If `self` is `Digit::Pos`, the result is simply `other`.
    ///
    /// # Returns:
    /// - A `Digit` value that is the result of the bitwise AND operation.
    ///
    /// # Examples:
    /// ```
    /// use balanced_ternary::Digit;
    /// use Digit::{Neg, Pos, Zero};
    ///
    /// assert_eq!(Neg & Pos, Neg);
    /// assert_eq!(Zero & Neg, Neg);
    /// assert_eq!(Zero & Pos, Zero);
    /// assert_eq!(Pos & Zero, Zero);
    /// ```
    fn bitand(self, other: Self) -> Self::Output {
        match self {
            Digit::Neg => Digit::Neg,
            Digit::Zero => other.negative(),
            Digit::Pos => other,
        }
    }
}

impl BitOr for Digit {
    type Output = Self;

    /// Performs a bitwise OR operation between two `Digit` values and returns the result.
    ///
    /// - The rules for the bitwise OR (`|`) operation are as follows:
    ///   - If `self` is `Digit::Neg`, the result is always the value of `other`.
    ///   - If `self` is `Digit::Zero`, the result depends on the value of `other`:
    ///     - `Digit::Pos` results in `Digit::Pos`.
    ///     - Otherwise, the result is `Digit::Zero`.
    ///   - If `self` is `Digit::Pos`, the result is always `Digit::Pos`.
    ///
    /// # Returns:
    /// - A `Digit` value that is the result of the bitwise OR operation.
    ///
    /// # Examples:
    /// ```
    /// use balanced_ternary::Digit;
    /// use Digit::{Neg, Pos, Zero};
    ///
    /// assert_eq!(Neg | Pos, Pos);
    /// assert_eq!(Zero | Neg, Zero);
    /// assert_eq!(Zero | Pos, Pos);
    /// assert_eq!(Pos | Zero, Pos);
    /// ```
    fn bitor(self, other: Self) -> Self::Output {
        match self {
            Digit::Neg => other,
            Digit::Zero => other.positive(),
            Digit::Pos => Digit::Pos,
        }
    }
}

impl BitXor for Digit {
    type Output = Self;

    /// Performs a bitwise XOR (exclusive OR) operation between two `Digit` values.
    ///
    /// - The rules for the bitwise XOR (`^`) operation are as follows:
    ///   - If `self` is `Digit::Neg`, the result is always the value of `rhs`.
    ///   - If `self` is `Digit::Zero`, the result is always `Digit::Zero`.
    ///   - If `self` is `Digit::Pos`, the result is the negation of `rhs`:
    ///     - `Digit::Neg` becomes `Digit::Pos`.
    ///     - `Digit::Zero` becomes `Digit::Zero`.
    ///     - `Digit::Pos` becomes `Digit::Neg`.
    ///
    /// # Returns:
    /// - A `Digit` value that is the result of the bitwise XOR operation.
    ///
    /// # Examples:
    /// ```
    /// use balanced_ternary::Digit;
    /// use Digit::{Neg, Pos, Zero};
    ///
    /// assert_eq!(Neg ^ Pos, Pos);
    /// assert_eq!(Zero ^ Neg, Zero);
    /// assert_eq!(Pos ^ Pos, Neg);
    /// ```
    fn bitxor(self, rhs: Self) -> Self::Output {
        match self {
            Digit::Neg => rhs,
            Digit::Zero => Digit::Zero,
            Digit::Pos => -rhs,
        }
    }
}
