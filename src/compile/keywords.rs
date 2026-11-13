//! Keywords read into a node's plain values: `type`'s bits, the number bounds as the draft says
//! them, counts and lists of names.

use serde_json::{Number, Value};

use crate::draft::Draft;
use crate::node::{
    ARRAY, BOOLEAN, INTEGER, NULL, NUMBER, OBJECT, STRING, Sign, TypeList, TypeNames,
};
use crate::number::exact;

use super::read::Keywords;

/// `type` as a node of types alone: its bits and its names packed; none where a name is not a
/// type's, which only rules can keep for a message.
pub(super) fn alone(kind: &Value) -> Option<(u8, TypeList)> {
    let (bits, names) = types(Some(kind));
    match names {
        TypeNames::Packed(list) => Some((bits?, list)),
        TypeNames::Written(_) => None,
    }
}

/// `type`: its bits (every kind for a name it does not know, none for one that is not a name),
/// and its names.
pub(super) fn types(kind: Option<&Value>) -> (Option<u8>, TypeNames) {
    let listed = match kind {
        None => return (None, TypeNames::default()),
        Some(Value::Array(many)) => many.as_slice(),
        Some(one) => std::slice::from_ref(one),
    };
    let names = listed.iter().filter_map(Value::as_str);
    let bits = names.clone().fold(0, |all, name| {
        all | match name {
            "null" => NULL,
            "boolean" => BOOLEAN,
            "integer" => INTEGER,
            "number" => NUMBER | INTEGER,
            "string" => STRING,
            "array" => ARRAY,
            "object" => OBJECT,
            _ => u8::MAX,
        }
    });
    (Some(bits), TypeNames::of(names))
}

/// The number bounds, in the order a message says them: `minimum`, `exclusiveMinimum`, `maximum`,
/// `exclusiveMaximum`; in draft 4, `exclusiveMinimum: true` makes `minimum` strict instead.
pub(super) fn limits(k: &Keywords<'_>, draft: Draft) -> Box<[(Number, Sign)]> {
    if k.minimum.is_none()
        && k.maximum.is_none()
        && k.exclusive_minimum.is_none()
        && k.exclusive_maximum.is_none()
    {
        return Box::default();
    }
    let number = |value: Option<&Value>| match value {
        Some(Value::Number(n)) => Some(n.clone()),
        Some(_) | None => None,
    };
    let flag = |value: Option<&Value>| draft.flag_bounds() && value == Some(&Value::Bool(true));
    let low = match flag(k.exclusive_minimum) {
        true => Sign::Above,
        false => Sign::AtLeast,
    };
    let high = match flag(k.exclusive_maximum) {
        true => Sign::Below,
        false => Sign::AtMost,
    };
    let exclusive = |value: Option<&Value>| number(value).filter(|_| !draft.flag_bounds());
    [
        number(k.minimum).map(|m| (m, low)),
        exclusive(k.exclusive_minimum).map(|m| (m, Sign::Above)),
        number(k.maximum).map(|m| (m, high)),
        exclusive(k.exclusive_maximum).map(|m| (m, Sign::Below)),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub(super) fn count(value: Option<&Value>) -> Option<u64> {
    match value? {
        // A count written `2.0` is the count 2.
        Value::Number(n) => n.as_u64().or_else(|| {
            let whole = exact(n.as_f64()?)?;
            // A count past u64 is one no data reaches: the most there is.
            Some(u64::try_from(whole.max(0)).unwrap_or(u64::MAX))
        }),
        Value::Null | Value::Bool(_) | Value::String(_) | Value::Array(_) | Value::Object(_) => {
            None
        }
    }
}
