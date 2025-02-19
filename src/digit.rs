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
//! #### `BitAnd` for `Digit`
//!
//! Performs a bitwise AND operation between two `Digit` values.
//!
//! - `Digit::Neg & other` → `Digit::Neg`
//! - `Digit::Zero & Digit::Neg` → `Digit::Neg`
//! - `Digit::Zero & other` → `Digit::Zero`
//! - `Digit::Pos & other` → `other`
//!
//! #### `BitOr` for `Digit`
//!
//! Performs a bitwise OR operation between two `Digit` values.
//!
//! - `Digit::Neg | other` → `other`
//! - `Digit::Zero | Digit::Pos` → `Digit::Pos`
//! - `Digit::Zero | other` → `Digit::Zero`
//! - `Digit::Pos | other` → `Digit::Pos`
//!
//! #### `BitXor` for `Digit`
//!
//! Performs a bitwise XOR operation between two `Digit` values.
//!
//! - `Digit::Neg ^ other` → `other`
//! - `Digit::Zero ^ other` → `Digit::Zero`
//! - `Digit::Pos ^ other` → `-other`

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
    pub fn to_char(&self) -> char {
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
    pub fn from_char(c: char) -> Digit {
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
    pub fn to_i8(&self) -> i8 {
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
    pub fn from_i8(i: i8) -> Digit {
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
    pub fn possibly(&self) -> Self {
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
    pub fn necessary(&self) -> Self {
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
    pub fn contingently(&self) -> Self {
        match self {
            Digit::Neg => Digit::Neg,
            Digit::Zero => Digit::Pos,
            Digit::Pos => Digit::Neg,
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
    pub fn not_negative(&self) -> Self {
        match self {
            Digit::Neg => Digit::Zero,
            Digit::Zero => Digit::Pos,
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
    pub fn positive(&self) -> Self {
        match self {
            Digit::Neg => Digit::Zero,
            Digit::Zero => Digit::Zero,
            Digit::Pos => Digit::Pos,
        }
    }

    /// Determines the condition of non-positivity for the current `Digit`.
    ///
    /// - Returns:
    ///     - `Digit::Pos` for `Digit::Neg`
    ///     - `Digit::Pos` for `Digit::Zero`
    ///     - `Digit::Zero` for `Digit::Pos`
    ///
    /// This method complements the `positive` condition and captures
    /// states that are not strictly positive.
    pub fn not_positive(&self) -> Self {
        match self {
            Digit::Neg => Digit::Pos,
            Digit::Zero => Digit::Pos,
            Digit::Pos => Digit::Zero,
        }
    }

    /// Determines the strictly negative condition for the current `Digit`.
    ///
    /// - Returns:
    ///     - `Digit::Pos` for `Digit::Neg`
    ///     - `Digit::Zero` for `Digit::Zero`
    ///     - `Digit::Zero` for `Digit::Pos`
    ///
    /// This method calculates strictly negative states
    /// in association with ternary logic.
    pub fn negative(&self) -> Self {
        match self {
            Digit::Neg => Digit::Pos,
            Digit::Zero => Digit::Zero,
            Digit::Pos => Digit::Zero,
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
    pub fn k3_imply(&self, other: Self) -> Self {
        match self {
            Digit::Neg => Digit::Pos,
            Digit::Zero => other.positive(),
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
    pub fn l3_imply(&self, other: Self) -> Self {
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
    pub fn rm3_imply(&self, other: Self) -> Self {
        match self {
            Digit::Neg => Digit::Pos,
            Digit::Zero => other,
            Digit::Pos => other.necessary(),
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
    pub fn ht_imply(&self, other: Self) -> Self {
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
    pub fn ht_not(&self) -> Self {
        match self {
            Digit::Neg => Digit::Pos,
            Digit::Zero => Digit::Neg,
            Digit::Pos => Digit::Neg,
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
    fn add(self, other: Digit) -> Self::Output {
        match self {
            Digit::Neg => match other {
                Digit::Neg => Ternary::parse("-+"),
                Digit::Zero => Ternary::parse("-"),
                Digit::Pos => Ternary::parse("0"),
            },
            Digit::Zero => Ternary::new(vec![other]),
            Digit::Pos => match other {
                Digit::Neg => Ternary::parse("0"),
                Digit::Zero => Ternary::parse("+"),
                Digit::Pos => Ternary::parse("+-"),
            },
        }
    }
}

impl Sub<Digit> for Digit {
    type Output = Ternary;
    fn sub(self, other: Digit) -> Self::Output {
        match self {
            Digit::Neg => match other {
                Digit::Neg => Ternary::parse("0"),
                Digit::Zero => Ternary::parse("-"),
                Digit::Pos => Ternary::parse("-+"),
            },
            Digit::Zero => Ternary::new(vec![-other]),
            Digit::Pos => match other {
                Digit::Neg => Ternary::parse("+-"),
                Digit::Zero => Ternary::parse("+"),
                Digit::Pos => Ternary::parse("0"),
            },
        }
    }
}

impl Mul<Digit> for Digit {
    type Output = Digit;

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

    fn div(self, other: Digit) -> Self::Output {
        match self {
            Digit::Neg => match other {
                Digit::Neg => Digit::Pos,
                Digit::Zero => panic!("Cannot divide by zero."),
                Digit::Pos => Digit::Neg,
            },
            Digit::Zero => match other {
                Digit::Neg => Digit::Zero,
                Digit::Zero => panic!("Cannot divide by zero."),
                Digit::Pos => Digit::Zero,
            },
            Digit::Pos => match other {
                Digit::Neg => Digit::Neg,
                Digit::Zero => panic!("Cannot divide by zero."),
                Digit::Pos => Digit::Pos,
            },
        }
    }
}

impl BitAnd for Digit {
    type Output = Self;
    fn bitand(self, other: Self) -> Self::Output {
        match self {
            Digit::Neg => Digit::Neg,
            Digit::Zero => match other {
                Digit::Neg => Digit::Neg,
                _ => Digit::Zero,
            },
            Digit::Pos => other,
        }
    }
}

impl BitOr for Digit {
    type Output = Self;
    fn bitor(self, other: Self) -> Self::Output {
        match self {
            Digit::Neg => other,
            Digit::Zero => match other {
                Digit::Pos => Digit::Pos,
                _ => Digit::Zero,
            },
            Digit::Pos => Digit::Pos,
        }
    }
}

impl BitXor for Digit {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match self {
            Digit::Neg => rhs,
            Digit::Zero => Digit::Zero,
            Digit::Pos => -rhs,
        }
    }
}
