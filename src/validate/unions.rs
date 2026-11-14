//! The quick walk's ways through a union, one for each way a union is told: by its list of
//! names, by its tag, or branch by branch, an `anyOf` and a `oneOf` each by a function of
//! their own. A branch that cannot hold a value of the value's kind (`Rules::kinds`) is not
//! walked.

use serde_json::Value;

use super::Walk;
use super::lanes::{Way, object};
use super::objects::seen;
use crate::node::{
    ALL, ARRAY, BOOLEAN, INTEGER, Id, NULL, NUMBER, OBJECT, Rules, STRING, TYPES_ALONE,
};

/// The way through the one union of `rules`.
pub(super) fn way(rules: &Rules) -> Way {
    let tagged = rules.any_tag.is_some() || rules.one_tag.is_some();
    match (&rules.listed, tagged, rules.any_of.is_empty()) {
        (Some(_), _, _) => listed,
        (None, true, _) => by_tag,
        (None, false, false) => any,
        (None, false, true) => one,
    }
}

/// The bits a branch's kinds are asked for `value` by.
#[inline(always)]
fn kind(value: &Value) -> u8 {
    match value {
        Value::Null => NULL,
        Value::Bool(_) => BOOLEAN,
        Value::Number(_) => NUMBER | INTEGER,
        Value::String(_) => STRING,
        Value::Array(_) => ARRAY,
        Value::Object(_) => OBJECT,
    }
}

/// Whether the branch `id` at `place` holds `value`, of `kind`: not walked where it cannot hold
/// a value of that kind, nor where it asks only for types and the value is no number (whether
/// a number is whole is the branch's to say).
#[inline(always)]
fn held(
    walk: &Walk<'_>,
    rules: &Rules,
    at: (usize, &Id),
    value: &Value,
    kind: u8,
    refs: usize,
) -> bool {
    let kinds = rules.kinds.get(at.0).copied().unwrap_or(ALL);
    kinds & kind != 0
        && ((kinds & TYPES_ALONE != 0 && kind & NUMBER == 0) || walk.holds(*at.1, value, refs))
}

/// A union of names holds the strings of its list and nothing else.
fn listed(_: &Walk<'_>, _: Id, rules: &Rules, value: &Value, _: usize) -> bool {
    let listed = rules.listed.as_ref();
    listed.is_some_and(|listed| value.as_str().is_some_and(|text| listed.holds(text)))
}

/// A union whose branch is picked by its tag (`Walk::matched`).
fn by_tag(walk: &Walk<'_>, _: Id, rules: &Rules, value: &Value, refs: usize) -> bool {
    match rules.any_of.is_empty() {
        false => walk.matched(&rules.any_of, rules.any_tag.as_deref(), value, refs, 1) > 0,
        true => walk.matched(&rules.one_of, rules.one_tag.as_deref(), value, refs, 2) == 1,
    }
}

/// An `anyOf` with no tag: each branch tried in turn, the first that holds settling it.
fn any(walk: &Walk<'_>, _: Id, rules: &Rules, value: &Value, refs: usize) -> bool {
    let kind = kind(value);
    (rules.any_of.iter().enumerate()).any(|at| held(walk, rules, at, value, kind, refs))
}

/// A `oneOf` with no tag: each branch tried in turn, a second that holds settling it.
fn one(walk: &Walk<'_>, _: Id, rules: &Rules, value: &Value, refs: usize) -> bool {
    let kind = kind(value);
    let mut held =
        (rules.one_of.iter().enumerate()).filter(|at| held(walk, rules, *at, value, kind, refs));
    held.next().is_some() && held.next().is_none()
}

/// An object as `object` reads it (or a type alone), and the one union beside it: where its
/// branches only require names of the object's properties (`Objects::branches`), each is told
/// by the properties the walk met, with nothing looked up.
pub(super) fn both(walk: &Walk<'_>, id: Id, rules: &Rules, value: &Value, refs: usize) -> bool {
    if let (Value::Object(fields), Some(objects)) = (value, &rules.object)
        && !objects.branches.is_empty()
        && walk.sorted
    {
        // The object first, as a union is tried after it: one that does not hold is not read
        // twice.
        if !object(walk, id, rules, value, refs) {
            return false;
        }
        let met = seen(objects, fields);
        let held = (objects.branches.iter()).filter(|mask| met & **mask == **mask);
        return match rules.any_of.is_empty() {
            true => held.count() == 1,
            false => held.count() > 0,
        };
    }
    object(walk, id, rules, value, refs) && way(rules)(walk, id, rules, value, refs)
}
