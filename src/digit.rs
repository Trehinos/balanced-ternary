use core::error::Error;
use core::fmt::{Display, Formatter};
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Sub};

#[cfg(feature = "ternary-string")]
use crate::Ternary;

/// Error returned when a value cannot represent a balanced ternary digit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DigitConversionError {
    /// The character is invalid for the selected digit representation.
    InvalidCharacter(char),
    /// The integer is not -1, 0, or 1.
    InvalidValue(i8),
}

impl Display for DigitConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidCharacter(character) => {
                write!(f, "invalid balanced ternary character {character:?}")
            }
            Self::InvalidValue(value) => write!(
                f,
                "invalid balanced ternary digit value {value}; \
                 expected -1, 0, or 1"
            ),
        }
    }
}

impl Error for DigitConversionError {}

/// One balanced trit. Addition and subtraction wrap modulo three.
/// Division by zero panics.
///
/// The bitwise operators implement digit logic. Named methods provide
/// additional truth functions, such as [`Self::k3_imply`] and [`Self::ht_imply`].
///
/// ![Digit truth tables](https://raw.githubusercontent.com/Trehinos/balanced-ternary/refs/heads/master/digit-operations.png)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Digit {
    /// Represents -1
    Neg,
    /// Represents 0
    Zero,
    /// Represents +1
    Pos,
}

impl Digit {
    pub const MIN: Digit = Digit::Neg;
    pub const MAX: Digit = Digit::Pos;
    pub const ZERO: Digit = Digit::Zero;

    /// Converts the `Digit` into a character representation.
    ///
    /// - Returns:
    ///     - `Θ` for `Digit::Neg`
    ///     - `0` for `Digit::Zero`
    ///     - `1` for `Digit::Pos`
    pub const fn to_char_theta(&self) -> char {
        match self {
            Digit::Neg => 'Θ',
            Digit::Zero => '0',
            Digit::Pos => '1',
        }
    }

    /// Converts the `Digit` into a character representation.
    ///
    /// - Returns:
    ///     - `Z` for `Digit::Neg`
    ///     - `0` for `Digit::Zero`
    ///     - `1` for `Digit::Pos`
    pub const fn to_char_z(&self) -> char {
        match self {
            Digit::Neg => 'Z',
            Digit::Zero => '0',
            Digit::Pos => '1',
        }
    }

    /// Converts the `Digit` into a character representation.
    ///
    /// - Returns:
    ///     - `T` for `Digit::Neg`
    ///     - `0` for `Digit::Zero`
    ///     - `1` for `Digit::Pos`
    pub const fn to_char_t(&self) -> char {
        match self {
            Digit::Neg => 'T',
            Digit::Zero => '0',
            Digit::Pos => '1',
        }
    }

    /// Attempts to convert 'Θ', '0', or '1' into a balanced ternary digit.
    ///
    /// Returns an error containing the invalid character on failure.
    pub const fn try_from_char_theta(c: char) -> Result<Self, DigitConversionError> {
        match c {
            'Θ' => Ok(Self::Neg),
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::Pos),
            _ => Err(DigitConversionError::InvalidCharacter(c)),
        }
    }

    /// Creates a `Digit` from a character representation.
    ///
    /// - Accepts:
    ///     - `Θ` for `Digit::Neg`
    ///     - `0` for `Digit::Zero`
    ///     - `1` for `Digit::Pos`
    ///
    /// # Panics
    ///
    /// Panics if the input character is invalid.
    /// Use [`Self::try_from_char_theta`] for a fallible conversion.
    pub const fn from_char_theta(c: char) -> Digit {
        match Self::try_from_char_theta(c) {
            Ok(digit) => digit,
            Err(_) => panic!("Invalid value. Expected 'Θ', '0', or '1'."),
        }
    }

    /// Attempts to convert 'Z', '0', or '1' into a balanced ternary digit.
    ///
    /// Returns an error containing the invalid character on failure.
    pub const fn try_from_char_z(c: char) -> Result<Self, DigitConversionError> {
        match c {
            'Z' => Ok(Self::Neg),
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::Pos),
            _ => Err(DigitConversionError::InvalidCharacter(c)),
        }
    }

    /// Creates a `Digit` from a character representation.
    ///
    /// - Accepts:
    ///     - `Z` for `Digit::Neg`
    ///     - `0` for `Digit::Zero`
    ///     - `1` for `Digit::Pos`
    ///
    /// # Panics
    ///
    /// Panics if the input character is invalid.
    /// Use [`Self::try_from_char_z`] for a fallible conversion.
    pub const fn from_char_z(c: char) -> Digit {
        match Self::try_from_char_z(c) {
            Ok(digit) => digit,
            Err(_) => panic!("Invalid value. Expected 'Z', '0', or '1'."),
        }
    }

    /// Attempts to convert 'T', '0', or '1' into a balanced ternary digit.
    ///
    /// Returns an error containing the invalid character on failure.
    pub const fn try_from_char_t(c: char) -> Result<Self, DigitConversionError> {
        match c {
            'T' => Ok(Self::Neg),
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::Pos),
            _ => Err(DigitConversionError::InvalidCharacter(c)),
        }
    }

    /// Creates a `Digit` from a character representation.
    ///
    /// - Accepts:
    ///     - `T` for `Digit::Neg`
    ///     - `0` for `Digit::Zero`
    ///     - `1` for `Digit::Pos`
    ///
    /// # Panics
    ///
    /// Panics if the input character is invalid.
    /// Use [`Self::try_from_char_t`] for a fallible conversion.
    pub const fn from_char_t(c: char) -> Digit {
        match Self::try_from_char_t(c) {
            Ok(digit) => digit,
            Err(_) => panic!("Invalid value. Expected 'T', '0', or '1'."),
        }
    }

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

    /// Attempts to convert '+', '0', or '-' into a balanced ternary digit.
    ///
    /// Returns an error containing the invalid character on failure.
    pub const fn try_from_char(c: char) -> Result<Self, DigitConversionError> {
        match c {
            '-' => Ok(Self::Neg),
            '0' => Ok(Self::Zero),
            '+' => Ok(Self::Pos),
            _ => Err(DigitConversionError::InvalidCharacter(c)),
        }
    }

    /// Converts '+', '0', or '-' into a balanced ternary digit.
    ///
    /// # Panics
    ///
    /// Panics if the character is invalid.
    /// Use [`Self::try_from_char`] for a fallible conversion.
    pub const fn from_char(c: char) -> Self {
        match Self::try_from_char(c) {
            Ok(digit) => digit,
            Err(_) => panic!("Invalid value. Expected '-', '0' or '+'."),
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

    /// Attempts to convert -1, 0, or 1 into a balanced ternary digit.
    ///
    /// This conversion is strict: values are not reduced modulo three.
    pub const fn try_from_i8(value: i8) -> Result<Self, DigitConversionError> {
        match value {
            -1 => Ok(Self::Neg),
            0 => Ok(Self::Zero),
            1 => Ok(Self::Pos),
            _ => Err(DigitConversionError::InvalidValue(value)),
        }
    }

    /// Converts -1, 0, or 1 into a balanced ternary digit.
    ///
    /// # Panics
    ///
    /// Panics if the integer is invalid.
    /// Use [`Self::try_from_i8`] for a fallible conversion.
    pub const fn from_i8(value: i8) -> Self {
        match Self::try_from_i8(value) {
            Ok(digit) => digit,
            Err(_) => panic!("Invalid value. A Ternary must be either -1, 0 or +1."),
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

    /// Converts `Neg` to `false` and `Pos` to `true`.
    ///
    /// # Panics
    ///
    /// Panics on `Zero`. Use [`Self::try_to_bool`] to preserve the unknown value,
    /// [`Self::possibly`] to resolve it to `Pos`, or [`Self::necessary`] to
    /// resolve it to `Neg`.
    pub const fn ht_bool(self) -> bool {
        self.try_to_bool()
            .expect("Cannot convert Digit::Zero to bool. Use try_to_bool() to handle Zero.")
    }

    /// Converts a known digit to a boolean.
    ///
    /// Returns `Some(false)` for `Neg`, `None` for `Zero`, and `Some(true)` for `Pos`.
    pub const fn try_to_bool(self) -> Option<bool> {
        match self {
            Digit::Neg => Some(false),
            Digit::Zero => None,
            Digit::Pos => Some(true),
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

    /// Grades the distance between two digits.
    ///
    /// Equal digits yield `Neg`, adjacent digits yield `Zero`, and opposite
    /// nonzero digits yield `Pos`. Numerically, the result is `|a - b| - 1`.
    pub const fn graded_diff(self, other: Self) -> Self {
        let distance = (self.to_i8() - other.to_i8()).abs();
        Self::from_i8(distance - 1)
    }

    /// Computes RM3 fusion.
    ///
    /// `Neg` is absorbing and `Zero` is the identity. In particular,
    /// fusing `Pos` with `Neg` yields `Neg` in either operand order.
    pub const fn rm3_fusion(self, other: Self) -> Self {
        match self {
            Self::Neg => Self::Neg,
            Self::Zero => other,
            Self::Pos => other.possibly(),
        }
    }

    /// Computes Łukasiewicz strong conjunction.
    ///
    /// For balanced digit values `a` and `b`, the result is
    /// `max(-1, a + b - 1)`. `Pos` is the identity, and combining
    /// two `Zero` digits yields `Neg`.
    pub const fn l3_strong_and(self, other: Self) -> Self {
        match self {
            Self::Neg => Self::Neg,
            Self::Zero => other.not_positive(),
            Self::Pos => other,
        }
    }

    /// Computes Łukasiewicz strong disjunction.
    ///
    /// For balanced digit values `a` and `b`, the result is
    /// `min(1, a + b + 1)`. `Neg` is the identity, and combining
    /// two `Zero` digits yields `Pos`.
    pub const fn l3_strong_or(self, other: Self) -> Self {
        match self {
            Self::Neg => other,
            Self::Zero => other.not_negative(),
            Self::Pos => Self::Pos,
        }
    }

    /// Returns the shared digit when both operands agree, or `Zero` otherwise.
    ///
    /// The result is `Neg` only for `(Neg, Neg)` and `Pos` only for
    /// `(Pos, Pos)`.
    pub const fn consensus(self, other: Self) -> Self {
        if self.to_i8() == other.to_i8() {
            self
        } else {
            Self::Zero
        }
    }

    /// Adds the two digits with saturation at `Neg` and `Pos`.
    ///
    /// `Zero` is the identity. Opposite signs cancel, and adding two
    /// identical nonzero digits preserves their sign.
    pub const fn accept_anything(self, other: Self) -> Self {
        match self {
            Self::Neg => other.not_positive(),
            Self::Zero => other,
            Self::Pos => other.not_negative(),
        }
    }

    /// Grades the number of unknown operands, treating `Zero` as unknown.
    ///
    /// Returns `Pos` when both operands are unknown, `Zero` when exactly
    /// one is unknown, and `Neg` when both are known.
    pub const fn unknown_consensus(self, other: Self) -> Self {
        match (self, other) {
            (Self::Zero, Self::Zero) => Self::Pos,
            (Self::Zero, _) | (_, Self::Zero) => Self::Zero,
            _ => Self::Neg,
        }
    }

    /// Compares the digits in the order `Neg < Zero < Pos`.
    ///
    /// Returns `Neg` when `self` is less than `other`, `Zero` when they
    /// are equal, and `Pos` when `self` is greater than `other`.
    pub const fn compare(self, other: Self) -> Self {
        Self::from_i8((self.to_i8() - other.to_i8()).signum())
    }

    /// Returns `Pos` when the digits are equal and `Neg` when they differ.
    ///
    /// Two `Zero` digits compare equal and therefore yield `Pos`.
    pub const fn equal(self, other: Self) -> Self {
        if self.to_i8() == other.to_i8() {
            Self::Pos
        } else {
            Self::Neg
        }
    }

    /// Returns `Pos` when the digits differ and `Neg` when they are equal.
    ///
    /// Two `Zero` digits therefore yield `Neg`.
    pub const fn different(self, other: Self) -> Self {
        if self.to_i8() != other.to_i8() {
            Self::Pos
        } else {
            Self::Neg
        }
    }

    /// Returns `Pos` if `self` is strictly greater than `other` in the order
    /// `Neg < Zero < Pos`, and `Neg` otherwise.
    pub const fn greater(self, other: Self) -> Self {
        if self.to_i8() > other.to_i8() {
            Self::Pos
        } else {
            Self::Neg
        }
    }

    /// Returns `Pos` if `self` is greater than or equal to `other` in the
    /// order `Neg < Zero < Pos`, and `Neg` otherwise.
    pub const fn greater_or_equal(self, other: Self) -> Self {
        if self.to_i8() >= other.to_i8() {
            Self::Pos
        } else {
            Self::Neg
        }
    }

    /// Resolves a rock-paper-scissors duel between two digits.
    ///
    /// The winning digit is returned: `Zero` beats `Neg`, `Pos` beats `Zero`,
    /// and `Neg` beats `Pos`. A tie returns the shared digit.
    pub const fn duel(self, other: Self) -> Self {
        match self {
            Self::Neg => other.absolute_negative(),
            Self::Zero => other.positive(),
            Self::Pos => other.possibly(),
        }
    }

    /// Computes McCarthy's left-sequential conjunction.
    ///
    /// A left operand of `Pos` selects `other`. A left operand of `Neg` or
    /// `Zero` determines the result regardless of `other`.
    pub const fn mc_and(self, other: Self) -> Self {
        match self {
            Self::Pos => other,
            _ => self,
        }
    }

    /// Computes McCarthy's left-sequential disjunction.
    ///
    /// A left operand of `Neg` selects `other`. A left operand of `Zero` or
    /// `Pos` determines the result regardless of `other`.
    pub const fn mc_or(self, other: Self) -> Self {
        match self {
            Self::Neg => other,
            _ => self,
        }
    }

    /// Returns `Pos` if `self` is strictly less than `other` in the order
    /// `Neg < Zero < Pos`, and `Neg` otherwise.
    pub const fn lesser(self, other: Self) -> Self {
        if self.to_i8() < other.to_i8() {
            Self::Pos
        } else {
            Self::Neg
        }
    }

    /// Returns `Pos` if `self` is less than or equal to `other` in the
    /// order `Neg < Zero < Pos`, and `Neg` otherwise.
    pub const fn lesser_or_equal(self, other: Self) -> Self {
        if self.to_i8() <= other.to_i8() {
            Self::Pos
        } else {
            Self::Neg
        }
    }

    /// Returns the first nonzero digit, checking `self` before `other`.
    ///
    /// If both digits are zero, the result is `Zero`.
    pub const fn coalesce(self, other: Self) -> Self {
        match self {
            Self::Zero => other,
            _ => self,
        }
    }

    /// Returns `Neg` when the digits have opposite nonzero signs and `Pos`
    /// otherwise. `Zero` is compatible with every digit, including itself.
    pub const fn compatible(self, other: Self) -> Self {
        match (self, other) {
            (Self::Neg, Self::Pos) | (Self::Pos, Self::Neg) => Self::Neg,
            _ => Self::Pos,
        }
    }

    /// Moves `self` one step toward `other` in the order `Neg < Zero < Pos`.
    ///
    /// Equal digits stay in place. Adjacent digits reach `other` in one step.
    /// Moving between `Neg` and `Pos` in either direction yields `Zero`.
    #[deprecated(
        since = "3.2.0",
        note = "Use `toward` instead. Will be removed in 4.0.0."
    )]
    pub const fn step_toward(self, other: Self) -> Self {
        self.toward(other)
    }

    /// Moves `self` one step toward `other` in the order `Neg < Zero < Pos`.
    ///
    /// Equal digits stay in place. Adjacent digits reach `other` in one step.
    /// Moving between `Neg` and `Pos` in either direction yields `Zero`.
    pub const fn toward(self, other: Self) -> Self {
        Self::from_i8(self.to_i8() + other.compare(self).to_i8())
    }

    /// Uses `other` to inhibit `self`.
    ///
    /// A zero control digit passes `self` through. Either nonzero control
    /// digit clears the result to `Zero`.
    pub const fn inhibit(self, other: Self) -> Self {
        match other {
            Self::Zero => self,
            _ => Self::Zero,
        }
    }

    /// Returns `other` when the digits differ, or `Zero` when they agree.
    ///
    /// For opposite signs, the result takes the sign of `other`.
    #[deprecated(
        since = "3.2.0",
        note = "Use `changed` instead. Will be removed in 4.0.0."
    )]
    pub const fn step_back(self, other: Self) -> Self {
        self.change(other)
    }

    /// Returns `other` when the digits differ, or `Zero` when they agree.
    ///
    /// For opposite signs, the result takes the sign of `other`.
    pub const fn change(self, other: Self) -> Self {
        if self.to_i8() == other.to_i8() {
            Self::Zero
        } else {
            other
        }
    }

    /// Returns the shared digit for a tie, or the third digit for a distinct pair.
    ///
    /// For example, `Neg` and `Zero` yield `Pos` in either operand order.
    pub const fn third(self, other: Self) -> Self {
        if self.to_i8() == other.to_i8() {
            self
        } else {
            // The three distinct digit values sum to zero.
            Self::from_i8(-self.to_i8() - other.to_i8())
        }
    }

    /// Returns the sole nonzero digit when exactly one operand is nonzero.
    /// If both operands are zero or both are nonzero, returns `Zero`.
    pub const fn exclusive_merge(self, other: Self) -> Self {
        match (self, other) {
            (Self::Zero, digit) | (digit, Self::Zero) => digit,
            _ => Self::Zero,
        }
    }

    /// Returns `Pos` if `other` immediately follows `self` in the cycle
    /// `Neg → Zero → Pos → Neg`, and `Neg` otherwise.
    pub const fn is_successor(self, other: Self) -> Self {
        self.post().equal(other)
    }

    /// Uses `other` to enable `self`.
    ///
    /// Either nonzero control digit passes `self` through. A zero control
    /// digit clears the result to `Zero`.
    pub const fn enable(self, other: Self) -> Self {
        match other {
            Self::Zero => Self::Zero,
            _ => self,
        }
    }

    /// Returns `Pos` if `other` immediately precedes `self` in the cycle
    /// `Neg → Zero → Pos → Neg`, and `Neg` otherwise.
    pub const fn is_predecessor(self, other: Self) -> Self {
        self.pre().equal(other)
    }

    /// Encodes this trit as its numeric value plus one.
    ///
    /// Maps `Neg` to `0`, `Zero` to `1`, and `Pos` to `2`.
    /// This is an offset encoding, not a numeric base conversion.
    pub const fn to_unbalanced(self) -> u8 {
        match self {
            Digit::Neg => 0,
            Digit::Zero => 1,
            Digit::Pos => 2,
        }
    }

    /// Decodes a trit from its offset encoding.
    ///
    /// Maps `0` to `Neg`, `1` to `Zero`, and `2` to `Pos`.
    /// This is the inverse of [`Self::to_unbalanced`].
    ///
    /// # Panics
    ///
    /// Panics if `u` is outside `0..=2`.
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
    #[cfg(feature = "ternary-string")]
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
    #[cfg(feature = "ternary-string")]
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
    type Output = Digit;

    /// Returns the sum modulo three.
    fn add(self, other: Digit) -> Self::Output {
        match self {
            Digit::Neg => other.pre(),
            Digit::Zero => other,
            Digit::Pos => other.post(),
        }
    }
}

impl Sub<Digit> for Digit {
    type Output = Digit;

    /// Returns the difference modulo three.
    fn sub(self, other: Digit) -> Self::Output {
        self + (-other)
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

    /// Divides by a single trit.
    ///
    /// # Panics
    ///
    /// Panics if the divisor is zero.
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
