//! Truth tables from `digit-operations.png` in the README.
//! Inputs, rows (left operand), and columns (right operand) use the order
//! `Neg`, `Zero`, `Pos`. The three zero-divisor cells have separate panic tests.

use crate::{Digit, Neg, Pos, Zero};
use core::ops::{Add, BitAnd, BitOr, BitXor, Mul, Not, Sub};

const DIGITS: [Digit; 3] = [Neg, Zero, Pos];

fn expected_digit(symbol: u8) -> Digit {
    match symbol {
        b'-' => Neg,
        b'0' => Zero,
        b'+' => Pos,
        _ => panic!("invalid digit in expected table: {}", char::from(symbol)),
    }
}

macro_rules! unary_table {
    ($operation:ident, $row:literal) => {
        #[test]
        fn $operation() {
            let expected: &[u8; 3] = $row;
            for (input, &symbol) in DIGITS.into_iter().zip(expected) {
                assert_eq!(
                    Digit::$operation(input),
                    expected_digit(symbol),
                    "input = {input:?}"
                );
            }
        }
    };
}

fn check_binary(operation: fn(Digit, Digit) -> Digit, rows: [&[u8; 3]; 3]) {
    for (left, row) in DIGITS.into_iter().zip(rows) {
        for (right, &symbol) in DIGITS.into_iter().zip(row) {
            assert_eq!(
                operation(left, right),
                expected_digit(symbol),
                "left = {left:?}, right = {right:?}"
            );
        }
    }
}

macro_rules! binary_table {
    ($operation:ident, $neg:literal, $zero:literal, $pos:literal) => {
        #[test]
        fn $operation() {
            check_binary(Digit::$operation, [$neg, $zero, $pos]);
        }
    };
}

unary_table!(not, b"+0-");
unary_table!(possibly, b"-++");
unary_table!(necessary, b"--+");
unary_table!(contingently, b"-+-");
unary_table!(ht_not, b"+--");
unary_table!(post, b"0+-");
unary_table!(pre, b"+-0");
unary_table!(absolute_positive, b"+0+");
unary_table!(positive, b"00+");
unary_table!(not_negative, b"0++");
unary_table!(not_positive, b"--0");
unary_table!(negative, b"-00");
unary_table!(absolute_negative, b"-0-");

binary_table!(bitand, b"---", b"-00", b"-0+");
binary_table!(add, b"+-0", b"-0+", b"0+-");

binary_table!(bitor, b"-0+", b"00+", b"+++");
binary_table!(sub, b"0-+", b"+0-", b"-+0");

binary_table!(bitxor, b"-0+", b"000", b"+0-");
binary_table!(mul, b"+0-", b"000", b"-0+");
binary_table!(graded_diff, b"-0+", b"0-0", b"+0-");

binary_table!(k3_imply, b"+++", b"00+", b"-0+");
binary_table!(bi3_and, b"-0-", b"000", b"-0+");
binary_table!(bi3_imply, b"+0+", b"000", b"-0+");
binary_table!(para_imply, b"+++", b"-0+", b"-0+");
binary_table!(rm3_fusion, b"---", b"-0+", b"-++");

binary_table!(k3_equiv, b"+0-", b"000", b"-0+");
binary_table!(bi3_or, b"-0+", b"000", b"+0+");
binary_table!(l3_imply, b"+++", b"0++", b"-0+");
binary_table!(ht_imply, b"+++", b"-++", b"-0+");
binary_table!(rm3_imply, b"+++", b"-0+", b"--+");

binary_table!(l3_strong_and, b"---", b"--0", b"-0+");
binary_table!(l3_strong_or, b"-0+", b"0++", b"+++");
binary_table!(consensus, b"-00", b"000", b"00+");
binary_table!(accept_anything, b"--0", b"-0+", b"0++");
binary_table!(unknown_consensus, b"-0-", b"0+0", b"-0-");

binary_table!(compare, b"0--", b"+0-", b"++0");
binary_table!(equal, b"+--", b"-+-", b"--+");
binary_table!(different, b"-++", b"+-+", b"++-");
binary_table!(greater, b"---", b"+--", b"++-");
binary_table!(greater_or_equal, b"+--", b"++-", b"+++");

binary_table!(duel, b"-0-", b"00+", b"-++");
binary_table!(mc_and, b"---", b"000", b"-0+");
binary_table!(mc_or, b"-0+", b"000", b"+++");
binary_table!(lesser, b"-++", b"--+", b"---");
binary_table!(lesser_or_equal, b"+++", b"-++", b"--+");

binary_table!(coalesce, b"---", b"-0+", b"+++");
binary_table!(compatible, b"++-", b"+++", b"-++");
binary_table!(step_toward, b"-00", b"-0+", b"00+");
binary_table!(inhibit, b"0-0", b"000", b"0+0");
binary_table!(step_back, b"00+", b"-0+", b"-00");

binary_table!(third, b"-+0", b"+0-", b"0-+");
binary_table!(exclusive_merge, b"0-0", b"-0+", b"0+0");
binary_table!(is_successor, b"-+-", b"--+", b"+--");
binary_table!(enable, b"-0-", b"000", b"+0+");
binary_table!(is_predecessor, b"--+", b"+--", b"-+-");

#[test]
fn div_nonzero() {
    use Digit::{Neg, Pos, Zero};

    for (left, by_neg, by_pos) in [(Neg, Pos, Neg), (Zero, Zero, Zero), (Pos, Neg, Pos)] {
        assert_eq!(left / Neg, by_neg, "{left:?} / Neg");
        assert_eq!(left / Pos, by_pos, "{left:?} / Pos");
    }
}

#[test]
#[should_panic(expected = "Cannot divide by zero")]
fn div_neg_by_zero() {
    let _ = Neg / Zero;
}

#[test]
#[should_panic(expected = "Cannot divide by zero")]
fn div_zero_by_zero() {
    let _ = Zero / Zero;
}

#[test]
#[should_panic(expected = "Cannot divide by zero")]
fn div_pos_by_zero() {
    let _ = Pos / Zero;
}

#[test]
fn digit_bool_conversions() {
    assert_eq!(Neg.try_to_bool(), Some(false));
    assert_eq!(Zero.try_to_bool(), None);
    assert_eq!(Pos.try_to_bool(), Some(true));

    assert!(!Neg.ht_bool());
    assert!(Pos.ht_bool());
}

#[test]
#[should_panic(expected = "Cannot convert Digit::Zero to bool. Use try_to_bool() to handle Zero.")]
fn ht_bool_panics_on_zero() {
    let _ = Zero.ht_bool();
}