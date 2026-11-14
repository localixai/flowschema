//! The leaves the walk settles where it stands (`Walk::leaf_holds`), found as each node is
//! built: an `enum` of strings, an integer between bounds, a string of a length. With the nodes
//! of types alone, they are most of an agent's fields, so a valid one costs a comparison, not a
//! walk.

use serde_json::Value;

use crate::node::{
    BOOLEAN, HAS_ARRAY, HAS_COMBINED, HAS_KIND, HAS_NUMBER, HAS_OBJECT, HAS_STRING, INTEGER, Lane,
    Leaf, NULL, Rules, STRING,
};

/// What `rules` ask, where they are a leaf.
pub(super) fn leaf(rules: &Rules) -> Leaf {
    let plain = rules.reference.is_none()
        && rules.only_types.is_none()
        && rules.unevaluated_fields.is_none()
        && rules.unevaluated_items.is_none();
    let allows = |bits: u8| rules.types.is_none_or(|types| types & bits != 0);
    let kind_alone = rules.has & !HAS_KIND == 0;
    if !plain {
        return Leaf::Walked;
    }
    if let Some(known) = &rules.constant {
        // A number is left to the walk: whether `1.0` is the integer `1` is the draft's to say.
        let kind = match known {
            Value::String(_) => STRING,
            Value::Bool(_) => BOOLEAN,
            Value::Null => NULL,
            Value::Number(_) | Value::Array(_) | Value::Object(_) => 0,
        };
        return match kind_alone && rules.options.is_none() && kind != 0 && allows(kind) {
            true => Leaf::Const,
            false => Leaf::Walked,
        };
    }
    if kind_alone && rules.options.is_some() && rules.option_kinds == STRING && allows(STRING) {
        return Leaf::Strings;
    }
    if rules.options.is_some() {
        return Leaf::Walked;
    }
    let whole = (rules.limits.iter()).all(|(limit, _)| limit.as_i64().is_some());
    if rules.has & !(HAS_KIND | HAS_NUMBER) == 0
        && rules.multiple.is_none()
        && allows(INTEGER)
        && whole
    {
        return Leaf::Range;
    }
    let text_alone = rules.has & !(HAS_KIND | HAS_STRING) == 0;
    match text_alone && allows(STRING) {
        true => Leaf::Text,
        false => Leaf::Walked,
    }
}

/// The lane the quick walk takes through `rules`.
pub(super) fn lane(rules: &Rules) -> Lane {
    let bare = rules.reference.is_none()
        && rules.options.is_none()
        && rules.constant.is_none()
        && rules.unevaluated_fields.is_none()
        && rules.unevaluated_items.is_none();
    let own = rules.has & !HAS_KIND;
    let object = (rules.object.as_ref()).is_some_and(|objects| objects.simple);
    let array = (rules.array.as_ref()).is_some_and(|arrays| {
        arrays.prefix.is_empty() && !arrays.unique && arrays.contains.is_none()
    });
    // One of the two unions, with no keyword beside it.
    let union = rules.has == HAS_COMBINED
        && rules.all_of.is_empty()
        && rules.not.is_none()
        && rules.condition.is_none()
        && rules.any_of.is_empty() != rules.one_of.is_empty();
    match (bare, own) {
        (true, HAS_OBJECT) if object => Lane::Object,
        (true, HAS_ARRAY) if array => Lane::Array,
        (true, HAS_COMBINED) if union => Lane::Union,
        (true | false, _) => Lane::Walk,
    }
}

#[cfg(test)]
#[path = "leaves_tests.rs"]
mod tests;
