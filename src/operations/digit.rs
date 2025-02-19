use alloc::vec;
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Sub};
use crate::{Digit, Ternary};

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

impl Digit {
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