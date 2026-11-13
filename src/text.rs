//! Strings compared where they stand, with no call to the system's `memcmp`: a field's name, an
//! `enum`'s option and a tag's value are short, and a call for each costs more than the
//! comparison. Equality is told by the length and two loads that overlap, order eight bytes at a
//! time, and a field of a small data object is found by walking its names.

use std::cmp::Ordering;

use serde_json::{Map, Value};

/// Whether two strings are the same (`same_bytes`).
#[inline]
pub(crate) fn same_text(a: &str, b: &str) -> bool {
    same_bytes(a.as_bytes(), b.as_bytes())
}

/// Whether two byte strings are the same, inline and with no loop up to 16 bytes: the length,
/// then two loads that overlap and cover every byte (the first and last four, or eight). A
/// field's name and an `enum`'s option are short, and a call to the system's `memcmp` for each
/// costs more than the comparison.
#[inline(always)]
pub(crate) fn same_bytes(a: &[u8], b: &[u8]) -> bool {
    let n = a.len();
    if n != b.len() {
        return false;
    }
    match n {
        0 => true,
        1..=3 => a.first() == b.first() && a.get(n / 2) == b.get(n / 2) && a.last() == b.last(),
        4..=7 => {
            a.first_chunk::<4>() == b.first_chunk::<4>()
                && a.last_chunk::<4>() == b.last_chunk::<4>()
        }
        8..=16 => {
            a.first_chunk::<8>() == b.first_chunk::<8>()
                && a.last_chunk::<8>() == b.last_chunk::<8>()
        }
        _ => a == b,
    }
}

/// Two names in byte order, eight bytes at a time and inline: a field's name is short, and a
/// call to the system's `memcmp` for each costs more than the comparison.
#[inline]
pub(crate) fn order(mut a: &[u8], mut b: &[u8]) -> Ordering {
    while let (Some((x, rest_a)), Some((y, rest_b))) =
        (a.split_first_chunk::<8>(), b.split_first_chunk::<8>())
    {
        let (x, y) = (u64::from_be_bytes(*x), u64::from_be_bytes(*y));
        if x != y {
            return x.cmp(&y);
        }
        (a, b) = (rest_a, rest_b);
    }
    for (x, y) in a.iter().zip(b) {
        if x != y {
            return x.cmp(y);
        }
    }
    a.len().cmp(&b.len())
}

/// Past this many fields, a field of a data object is looked up in its map rather than walked to.
const WALKED: usize = 16;

/// The field `name` of a data object: walked to in a small one, each name told apart by its
/// length first; looked up in a large one.
#[inline]
pub(crate) fn field<'v>(fields: &'v Map<String, Value>, name: &str) -> Option<&'v Value> {
    if fields.len() > WALKED {
        return fields.get(name);
    }
    (fields.iter())
        .find(|(known, _)| same_text(known, name))
        .map(|(_, value)| value)
}

#[cfg(test)]
#[path = "text_tests.rs"]
mod tests;
