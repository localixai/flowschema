//! The leaves the walk settles where it stands (`Walk::leaf_holds`), found as each node is
//! built: an `enum` of strings, an integer between bounds, a string of a length. With the nodes
//! of types alone, they are most of an agent's fields, so a valid one costs a comparison, not a
//! walk.

use crate::node::{HAS_KIND, HAS_NUMBER, HAS_STRING, INTEGER, Leaf, Rules, STRING};

/// What `rules` ask, where they are a leaf.
pub(super) fn leaf(rules: &Rules) -> Leaf {
    let plain = rules.reference.is_none()
        && rules.constant.is_none()
        && rules.only_types.is_none()
        && rules.unevaluated_fields.is_none()
        && rules.unevaluated_items.is_none();
    let allows = |bits: u8| rules.types.is_none_or(|types| types & bits != 0);
    let kind_alone = rules.has & !HAS_KIND == 0;
    if !plain {
        return Leaf::Walked;
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

#[cfg(test)]
#[path = "leaves_tests.rs"]
mod tests;
