use std::cmp::Ordering;

use serde_json::{Number, Value, json};

use super::*;

#[test]
fn a_whole_float_is_read_as_its_integer_exactly() {
    assert_eq!(exact(0.0), Some(0));
    assert_eq!(exact(-0.0), Some(0));
    assert_eq!(exact(3.0), Some(3));
    assert_eq!(exact(-1024.0), Some(-1024));
    assert_eq!(exact(9_007_199_254_740_992.0), Some(9_007_199_254_740_992));
    assert_eq!(
        exact(1.0e30),
        Some(1_000_000_000_000_000_019_884_624_838_656)
    );
    assert_eq!(exact(-(2f64.powi(126))), Some(-(1_i128 << 126)));
    assert_eq!(exact(1.5), None);
    assert_eq!(exact(f64::INFINITY), None);
    assert_eq!(exact(f64::NAN), None);
    assert_eq!(exact(1.0e39), None, "past an i128");
}

#[test]
fn an_integer_and_a_float_compare_without_rounding() {
    let n = |value: Value| -> Number { serde_json::from_value(value).unwrap() };
    let cmp = |a, b| compare(&n(a), &n(b));
    assert_eq!(
        cmp(
            json!(9_007_199_254_740_993_u64),
            json!(9_007_199_254_740_992.0)
        ),
        Some(Ordering::Greater)
    );
    assert_eq!(cmp(json!(2), json!(2.5)), Some(Ordering::Less));
    assert_eq!(cmp(json!(3), json!(2.5)), Some(Ordering::Greater));
    assert_eq!(cmp(json!(-3), json!(-2.5)), Some(Ordering::Less));
    assert_eq!(cmp(json!(2), json!(2.0)), Some(Ordering::Equal));
    assert_eq!(cmp(json!(u64::MAX), json!(1.0e300)), Some(Ordering::Less));
    assert_eq!(
        cmp(json!(i64::MIN), json!(-1.0e300)),
        Some(Ordering::Greater)
    );
    assert_eq!(cmp(json!(1.5), json!(2.5)), Some(Ordering::Less));
}

/// A fraction held to a small divisor with no spelling of it says what the spelled way says:
/// each divisor's multiples, their neighbours a bit away, and numbers of every length.
#[test]
fn a_fraction_is_held_to_a_divisor_unspelled_as_it_is_spelled() {
    let divisors = [
        0.01, 0.1, 0.5, 0.25, 1.5, 0.001, 2.5, 0.0075, 1e-15, 3.0, 7.0, 0.3, 1e-9, 12.5, 0.05,
    ];
    let mut tried = 0;
    for said in divisors {
        let number = serde_json::Number::from_f64(said).unwrap();
        let quick = Divisor::new(&number);
        let spelled = Divisor {
            small: None,
            ..Divisor::new(&number)
        };
        let mut check = |n: f64| {
            let n = serde_json::Number::from_f64(n).unwrap();
            assert_eq!(quick.divides(&n), spelled.divides(&n), "{n} by {said}");
            tried += 1;
        };
        for k in (-40_i32..400).chain([12_345, 1_234_567, 99_999_999, 2_000_000_011]) {
            let multiple = f64::from(k) * said;
            for bits in [0_i64, 1, -1, 2, -2, 17] {
                let near = f64::from_bits(multiple.to_bits().wrapping_add_signed(bits));
                for n in [near, near + 0.5, near / 3.0, near * 1e-7, near * 1e9] {
                    if n.is_finite() {
                        check(n);
                    }
                }
            }
        }
        for n in [
            12345.67,
            0.1 + 0.2,
            1e-300,
            5e-324,
            1e300,
            0.30000000000000004,
            1e15 + 0.5,
        ] {
            check(n);
        }
    }
    assert!(tried > 100_000);
}
