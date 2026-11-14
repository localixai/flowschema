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

/// Whether two byte strings are the same, inline and with no loop up to 32 bytes: the length,
/// then two loads that overlap and cover every byte (the first and last 4, 8 or 16). A
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
        17..=32 => {
            a.first_chunk::<16>() == b.first_chunk::<16>()
                && a.last_chunk::<16>() == b.last_chunk::<16>()
        }
        _ => a == b,
    }
}

/// Two names in byte order, inline: a field's name is short, and a call to the system's
/// `memcmp` for each costs more than the comparison.
#[inline]
pub(crate) fn order(a: &[u8], b: &[u8]) -> Ordering {
    let n = a.len().min(b.len());
    let (Some(x), Some(y)) = (a.get(..n), b.get(..n)) else {
        return Ordering::Equal;
    };
    // What both have, read as few numbers as hold it: the last one may lie over bytes already
    // read, which are the same in both and so say nothing of their order.
    let shared = match n {
        0 => Ordering::Equal,
        1..=3 => {
            let at = |name: &[u8]| {
                (
                    name.first().copied(),
                    name.get(n / 2).copied(),
                    name.last().copied(),
                )
            };
            at(x).cmp(&at(y))
        }
        4..=7 => {
            let read = |four: Option<&[u8; 4]>| four.map(|four| u32::from_be_bytes(*four));
            (read(x.first_chunk()), read(x.last_chunk()))
                .cmp(&(read(y.first_chunk()), read(y.last_chunk())))
        }
        _ => {
            let read = |eight: Option<&[u8; 8]>| eight.map(|eight| u64::from_be_bytes(*eight));
            let (mut p, mut q) = (x, y);
            while let (Some((c, rest_p)), Some((d, rest_q)), true) = (
                p.split_first_chunk::<8>(),
                q.split_first_chunk::<8>(),
                p.len() > 8,
            ) {
                if c != d {
                    return read(Some(c)).cmp(&read(Some(d)));
                }
                (p, q) = (rest_p, rest_q);
            }
            read(x.last_chunk()).cmp(&read(y.last_chunk()))
        }
    };
    shared.then(a.len().cmp(&b.len()))
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
