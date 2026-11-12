extern crate std;

use crate::Digit;

#[test]
fn digit_arithmetic_exhaustive() {
    let digits = [Digit::Neg, Digit::Zero, Digit::Pos];
    let wrap = |value: i8| (value + 1).rem_euclid(3) - 1;

    for a in digits {
        assert_eq!((-a).to_i8(), -a.to_i8());

        for b in digits {
            let x = a.to_i8();
            let y = b.to_i8();

            assert_eq!((a + b).to_i8(), wrap(x + y));
            assert_eq!((a - b).to_i8(), wrap(x - y));
            assert_eq!((a * b).to_i8(), x * y);

            if y == 0 {
                assert!(std::panic::catch_unwind(|| a / b).is_err());
            } else {
                assert_eq!((a / b).to_i8(), x / y);
            }
        }
    }
}

#[test]
fn digit_logic_truth_tables() {
    use Digit::{Neg, Pos, Zero};

    let digits = [Neg, Zero, Pos];
    let and = [[Neg, Neg, Neg], [Neg, Zero, Zero], [Neg, Zero, Pos]];
    let or = [[Neg, Zero, Pos], [Zero, Zero, Pos], [Pos, Pos, Pos]];
    let xor = [[Neg, Zero, Pos], [Zero, Zero, Zero], [Pos, Zero, Neg]];

    for (i, a) in digits.into_iter().enumerate() {
        assert_eq!(!a, -a);

        for (j, b) in digits.into_iter().enumerate() {
            assert_eq!(a & b, and[i][j]);
            assert_eq!(a | b, or[i][j]);
            assert_eq!(a ^ b, xor[i][j]);
        }
    }
}

#[cfg(feature = "ternary-string")]
mod numeric {
    extern crate std;

    use crate::concepts::DigitOperate;
    use crate::{Digit, Ternary};
    use alloc::vec;
    use alloc::vec::Vec;
    use std::collections::{BTreeSet, HashSet};
    use std::hash::{DefaultHasher, Hash, Hasher};

    fn next(seed: &mut u64) -> u64 {
        *seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        *seed
    }

    fn random_ternary(seed: &mut u64, width: usize) -> Ternary {
        let mut digits: Vec<_> = (0..width)
            .map(|_| match (next(seed) >> 32) % 3 {
                0 => Digit::Neg,
                1 => Digit::Zero,
                _ => Digit::Pos,
            })
            .collect();

        // Exercise the requested significant width.
        if let Some(first) = digits.first_mut() {
            *first = if next(seed) & 1 == 0 {
                Digit::Neg
            } else {
                Digit::Pos
            };
        }

        Ternary::new(digits)
    }

    fn evaluate(value: &Ternary) -> i128 {
        value.to_digit_slice().iter().fold(0i128, |acc, digit| {
            acc.checked_mul(3)
                .and_then(|v| v.checked_add(i128::from(digit.to_i8())))
                .expect("Reference evaluation exceeded i128")
        })
    }

    fn assert_canonical(value: &Ternary) {
        let digits = value.to_digit_slice();

        assert!(!digits.is_empty());
        assert!(
            digits.len() == 1 || digits[0] != Digit::Zero,
            "Noncanonical result: {value}"
        );
    }

    fn magnitude(value: &Ternary) -> Ternary {
        if value < &Ternary::from_dec(0) {
            -value
        } else {
            value.clone()
        }
    }

    fn hash(value: &Ternary) -> u64 {
        let mut state = DefaultHasher::new();
        value.hash(&mut state);
        state.finish()
    }

    fn add_digits(a: Digit, b: Digit, carry: Digit) -> (Digit, Digit) {
        let sum = a.to_i8() + b.to_i8() + carry.to_i8();
        let next_carry = (sum + 1).div_euclid(3);

        (
            Digit::from_i8(next_carry),
            Digit::from_i8(sum - 3 * next_carry),
        )
    }

    fn check_zip<T: DigitOperate + Clone>(a: &T, b: &T) {
        let a_digits = a.to_digits();
        let b_digits = b.to_digits();
        let width = a_digits.len().max(b_digits.len());

        let mut left = vec![Digit::Zero; width];
        let mut right = vec![Digit::Zero; width];
        left[width - a_digits.len()..].copy_from_slice(&a_digits);
        right[width - b_digits.len()..].copy_from_slice(&b_digits);

        let expected: Vec<_> = left
            .iter()
            .zip(&right)
            .map(|(x, y)| {
                let value = x.to_i8() - y.to_i8();
                Digit::from_i8((value + 1).rem_euclid(3) - 1)
            })
            .collect();

        let result = a.each_zip(|x, y| x - y, b.clone());
        assert_eq!(result.to_digits(), expected);
    }

    #[test]
    fn arithmetic_matches_i128_reference() {
        let boundaries = [
            i64::MIN,
            i64::MIN + 1,
            -243,
            -1,
            0,
            1,
            243,
            i64::MAX - 1,
            i64::MAX,
        ];

        let mut pairs = Vec::new();
        for a in boundaries {
            for b in boundaries {
                pairs.push((a, b));
            }
        }

        let mut seed = 0x9E37_79B9_7F4A_7C15;
        for _ in 0..256 {
            pairs.push((next(&mut seed) as i64, next(&mut seed) as i64));
        }

        for (a, b) in pairs {
            let x = Ternary::from_dec(a).with_length(80);
            let y = Ternary::from_dec(b).with_length(47);
            let ai = i128::from(a);
            let bi = i128::from(b);

            assert_eq!(evaluate(&x), ai);
            assert_eq!(i64::try_from(&x), Ok(a));
            assert_eq!(x.cmp(&y), a.cmp(&b));

            for (actual, expected) in [(&x + &y, ai + bi), (&x - &y, ai - bi), (&x * &y, ai * bi)] {
                assert_eq!(evaluate(&actual), expected, "a={a}, b={b}");
                assert_canonical(&actual);
            }

            if b != 0 {
                let (q, r) = x.div_rem(&y);
                assert_eq!(evaluate(&q), ai / bi, "a={a}, b={b}");
                assert_eq!(evaluate(&r), ai % bi, "a={a}, b={b}");
                assert_eq!(&x / &y, q);
                assert_eq!(&x % &y, r);
                assert_canonical(&q);
                assert_canonical(&r);
            }

            check_zip(&x, &y);
            check_zip(&y, &x);
            assert_eq!(evaluate(&x.each_zip_carry(add_digits, y.clone())), ai + bi);

            #[cfg(feature = "tryte")]
            {
                let word = crate::Tryte::<41>::try_from(&x).expect("Every i64 fits in 41 trits");
                assert_eq!(i64::try_from(word), Ok(a));
            }

            #[cfg(feature = "ternary-store")]
            {
                let storage = crate::DataTernary::from_ternary(x.clone());
                assert_eq!(i64::try_from(&storage), Ok(a));
            }
        }
    }

    #[test]
    fn arbitrary_precision_properties() {
        let mut seed = 0xD1B5_4A32_D192_ED03;
        let zero = Ternary::from_dec(0);

        for width in [0, 1, 5, 41, 128, 257] {
            for _ in 0..8 {
                let a = random_ternary(&mut seed, width);
                let b = random_ternary(&mut seed, width / 2 + 1).with_length(width + 9);
                let c = random_ternary(&mut seed, width / 3 + 1);

                let sum = &a + &b;
                let difference = &a - &b;
                let product = &a * &b;

                assert_eq!(&sum - &b, a);
                assert_eq!(&difference + &b, a);
                assert_eq!(sum, &b + &a);
                assert_eq!(product, &b * &a);
                assert_eq!(&a * &(&b + &c), &(&a * &b) + &(&a * &c));
                assert_eq!(&product / &b, a);
                assert_eq!(&product % &b, zero);

                let (q, r) = a.div_rem(&b);
                assert_eq!(&(&q * &b) + &r, a);
                assert!(magnitude(&r) < magnitude(&b));

                if r != zero {
                    assert_eq!(r.cmp(&zero), a.cmp(&zero));
                }

                for result in [&sum, &difference, &product, &q, &r] {
                    assert_canonical(result);
                }

                check_zip(&a, &b);
                check_zip(&b, &a);
                assert_eq!(a.each_zip_carry(add_digits, b.clone()), sum);

                #[cfg(feature = "ternary-store")]
                {
                    use crate::DataTernary;

                    let padded = a.with_length(width + 11);
                    let storage = DataTernary::from_ternary(padded.clone());
                    let stored_width = padded.log().div_ceil(5) * 5;
                    let expected = padded.with_length(stored_width);
                    let fixed = storage.to_fixed_ternary();

                    assert_eq!(fixed.to_digit_slice(), expected.to_digit_slice());
                    assert_eq!(storage.to_ternary(), a);
                    assert_eq!(DataTernary::from_ternary(fixed), storage);
                }
            }
        }
    }

    #[test]
    fn numeric_equality_order_and_hash_agree() {
        let large = Ternary::parse(&"+0-".repeat(128));
        let expected = vec![
            -&large,
            Ternary::from_dec(-1),
            Ternary::from_dec(0),
            Ternary::from_dec(1),
            large,
        ];

        let mut hashed = HashSet::new();
        let mut ordered = BTreeSet::new();

        for value in &expected {
            for width in [0, 5, 40, 512] {
                let padded = value.with_length(width);

                assert_eq!(&padded, value);
                assert_eq!(padded.cmp(value), std::cmp::Ordering::Equal);
                assert_eq!(hash(&padded), hash(value));

                hashed.insert(padded.clone());
                ordered.insert(padded);
            }
        }

        let empty = Ternary::parse("");
        let zero = Ternary::from_dec(0);

        assert_eq!(hash(&empty), hash(&zero));
        assert!(!hashed.insert(empty.clone()));
        assert!(!ordered.insert(empty));

        assert_eq!(hashed.len(), expected.len());
        assert_eq!(ordered.into_iter().collect::<Vec<_>>(), expected);
    }

    #[cfg(feature = "tryte")]
    #[test]
    fn fixed_width_boundaries_match_i128() {
        use crate::Tryte;

        fn check<const N: usize>() {
            let modulus = 3i128.pow(N as u32);
            let max = (modulus - 1) / 2;
            let values = [-max, -max + 1, -1, 0, 1, max - 1, max];

            for a in values {
                for b in values {
                    let x = Tryte::<N>::from_i64(a as i64);
                    let y = Tryte::<N>::from_i64(b as i64);

                    for (wrapped, checked, saturated, exact) in [
                        (x + y, x.checked_add(y), x.saturating_add(y), a + b),
                        (x - y, x.checked_sub(y), x.saturating_sub(y), a - b),
                        (x * y, x.checked_mul(y), x.saturating_mul(y), a * b),
                    ] {
                        let expected = (exact + max).rem_euclid(modulus) - max;
                        let fits = (-max..=max).contains(&exact);

                        assert_eq!(i128::from(wrapped.to_i64()), expected);
                        assert_eq!(
                            checked.map(|v| i128::from(v.to_i64())),
                            fits.then_some(exact)
                        );
                        assert_eq!(i128::from(saturated.to_i64()), exact.clamp(-max, max));
                    }

                    assert_eq!(
                        x.checked_div(y).map(|v| i128::from(v.to_i64())),
                        if b == 0 { None } else { Some(a / b) }
                    );

                    let (result, carry) = x.each_zip_carry_full(add_digits, y);
                    assert_eq!(
                        i128::from(result.to_i64()) + i128::from(carry.to_i8()) * modulus,
                        a + b
                    );
                    assert_eq!(x.each_zip_carry(add_digits, y), result);
                    assert_eq!(result.to_digit_slice().len(), N);
                    check_zip(&x, &y);
                }
            }
        }

        check::<1>();
        check::<2>();
        check::<6>();
        check::<40>();
    }

    #[cfg(feature = "ternary-store")]
    #[test]
    fn ter40_boundaries_match_i128() {
        use crate::Ter40;

        let max = i128::from(Ter40::MAX.to_dec());
        let modulus = 2 * max + 1;
        let values = [-max, -max + 1, -1, 0, 1, max - 1, max];

        for a in values {
            for b in values {
                let x = Ter40::from_dec(a as i64);
                let y = Ter40::from_dec(b as i64);

                for (actual, exact) in [
                    (x.checked_add(y), a + b),
                    (x.checked_sub(y), a - b),
                    (x.checked_mul(y), a * b),
                ] {
                    assert_eq!(
                        actual.map(|v| i128::from(v.to_dec())),
                        (-max..=max).contains(&exact).then_some(exact)
                    );
                }

                assert_eq!(
                    x.checked_div(y).map(|v| i128::from(v.to_dec())),
                    if b == 0 { None } else { Some(a / b) }
                );

                let (result, carry) = x.each_zip_carry_full(add_digits, y);
                assert_eq!(
                    i128::from(result.to_dec()) + i128::from(carry.to_i8()) * modulus,
                    a + b
                );
                assert_eq!(x.each_zip_carry(add_digits, y), result);
                assert_eq!(result.to_digits().len(), 40);
                check_zip(&x, &y);
            }
        }
    }
}
