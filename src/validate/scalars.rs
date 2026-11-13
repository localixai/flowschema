//! A value's own keywords where it is a number or a string, and a node of types alone: the
//! checks that read the value and nothing below it.

use serde_json::Value;

use super::Walk;
use crate::node::{Rules, TypeList};
use crate::number;
use crate::report::{At, Report, said, short, type_of};

impl Walk<'_> {
    /// A string's keywords: `minLength` and `maxLength` in characters, `pattern`, `format`.
    pub(super) fn text<const F: bool>(
        &self,
        rules: &Rules,
        text: &str,
        at: &At<'_>,
        r: &mut Report<'_, F>,
    ) -> bool {
        if !length(rules, text, at, r) {
            return false;
        }
        if let Some((source, Some(pattern))) = rules.pattern.and_then(|p| self.patterns.get(p))
            && !pattern.is_match(text)
            && !r.no(|| {
                said!(
                    "{at} must match the pattern {source:?} (is {})",
                    short_text(text)
                )
            })
        {
            return false;
        }
        match &rules.format {
            Some((name, check)) if !check(text.as_bytes()) => {
                r.no(|| said!("{at} must be a valid {name} (is {})", short_text(text)))
            }
            Some(_) | None => true,
        }
    }

    /// A node of types alone, said as the rules' `type` says it (`kind`). Apart from `node`, so
    /// the walk's every call does not pay for it: most such nodes are settled before (`fits`).
    #[inline(never)]
    pub(super) fn typed<const F: bool>(
        &self,
        types: u8,
        names: TypeList,
        value: &Value,
        at: &At<'_>,
        mut r: Report<'_, F>,
    ) -> bool {
        let kind = self.kind_for(types, value);
        if types & kind != 0 {
            return true;
        }
        r.no(|| {
            said!(
                "{at} must be {} (is {})",
                names.names().join(" or "),
                type_of(kind)
            )
        });
        false
    }
}

/// A string as a message names it.
fn short_text(text: &str) -> String {
    short(&Value::String(text.to_string()))
}

/// The number bounds, then `multipleOf` (read only where the node has either: `HAS_NUMBER`).
pub(super) fn bounds<const F: bool>(
    rules: &Rules,
    n: &serde_json::Number,
    at: &At<'_>,
    r: &mut Report<'_, F>,
) -> bool {
    for (limit, sign) in &rules.limits {
        if !sign.holds(n, limit) && !r.no(|| said!("{at} must be {} {limit} (is {n})", sign.said()))
        {
            return false;
        }
    }
    match &rules.multiple {
        Some(m) if !number::multiple(n, m) => {
            r.no(|| said!("{at} must be a multiple of {m} (is {n})"))
        }
        Some(_) | None => true,
    }
}

/// `minLength` and `maxLength`, in characters (counted only as far as the bound needs).
fn length<const F: bool>(rules: &Rules, text: &str, at: &At<'_>, r: &mut Report<'_, F>) -> bool {
    let (least, most) = (rules.min_length, rules.max_length);
    if least.is_none() && most.is_none() {
        return true;
    }
    // A character takes one to four bytes: the byte count settles most bounds unread.
    let bytes = text.len() as u64;
    if most.is_none_or(|most| bytes <= most) && least.is_none_or(|least| bytes / 4 >= least) {
        return true;
    }
    // Past four bytes a character, a quick walk knows the text is too long without counting.
    if !r.full() && most.is_some_and(|most| bytes > most.saturating_mul(4)) {
        return r.no(String::new);
    }
    let len = text.chars().count() as u64;
    if let Some(least) = least.filter(|least| len < *least)
        && !r.no(|| said!("{at} must have at least {least} characters (has {len})"))
    {
        return false;
    }
    match most.filter(|most| len > *most) {
        Some(most) => r.no(|| said!("{at} must have at most {most} characters (has {len})")),
        None => true,
    }
}
