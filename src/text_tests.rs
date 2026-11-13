use serde_json::{Map, Value, json};

use super::*;

#[test]
fn names_are_ordered_as_their_bytes_are() {
    use std::cmp::Ordering::{Equal, Greater, Less};
    for (a, b) in [
        ("a", "b"),
        ("ab", "abc"),
        ("field_09", "field_10"),
        ("", "x"),
        ("abcdefgh", "abcdefgi"),
        ("abcdefghij", "abcdefgh"),
        ("é", "z"),
        ("same-name-here", "same-name-here"),
    ] {
        assert_eq!(order(a.as_bytes(), b.as_bytes()), a.cmp(b), "{a} {b}");
        assert_eq!(order(b.as_bytes(), a.as_bytes()), b.cmp(a), "{b} {a}");
    }
    assert_eq!(
        (order(b"x", b"x"), order(b"a", b"b"), order(b"b", b"a")),
        (Equal, Less, Greater)
    );
}

#[test]
fn bytes_are_the_same_as_slices_say_at_every_length() {
    let text = "abcdefghijklmnopqrstuvwxyz0123456789";
    for len in 0..=text.len() {
        let a = &text.as_bytes()[..len];
        for at in 0..len {
            let mut b = a.to_vec();
            b[at] ^= 1;
            assert!(!same_bytes(a, &b), "{len} {at}");
        }
        assert!(same_bytes(a, a), "{len}");
        assert!(!same_bytes(a, &text.as_bytes()[..len.saturating_sub(1)]) || len == 0);
    }
}

#[test]
fn a_field_is_found_walked_to_or_looked_up() {
    let small: Map<String, Value> = (0..4).map(|n| (format!("f{n}"), json!(n))).collect();
    assert_eq!(field(&small, "f3"), Some(&json!(3)));
    assert_eq!(field(&small, "f4"), None);
    assert_eq!(field(&small, "f"), None);
    let large: Map<String, Value> = (0..40).map(|n| (format!("f{n}"), json!(n))).collect();
    assert_eq!(field(&large, "f39"), Some(&json!(39)));
    assert_eq!(field(&large, "f40"), None);
}
