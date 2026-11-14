//! The leaves the walk settles where it stands (`Walk::leaf_holds`), found as each node is
//! built: an `enum` of strings, an integer between bounds, a string of a length. With the nodes
//! of types alone, they are most of an agent's fields, so a valid one costs a comparison, not a
//! walk.

use serde_json::Value;

use crate::node::{
    BOOLEAN, Extra, HAS_ARRAY, HAS_COMBINED, HAS_KIND, HAS_NUMBER, HAS_OBJECT, HAS_STRING, INTEGER,
    Lane, Leaf, NULL, NUMBER, Rules, STRING,
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
        // A number beside `integer` is left to the walk: whether `1.0` is the integer `1` is
        // the draft's to say. Beside `number`, or no type, every number equal to it is held.
        let kind = match known {
            Value::String(_) => STRING,
            Value::Bool(_) => BOOLEAN,
            Value::Null => NULL,
            Value::Number(_) => NUMBER,
            Value::Array(_) | Value::Object(_) => 0,
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
/// An object's keywords with a combination beside them.
const BOTH: u8 = HAS_OBJECT | HAS_COMBINED;

pub(super) fn lane(rules: &Rules) -> Lane {
    let unevaluated = rules.unevaluated_fields.is_some() || rules.unevaluated_items.is_some();
    let bare = rules.reference.is_none()
        && rules.options.is_none()
        && rules.constant.is_none()
        && !unevaluated;
    let own = rules.has & !HAS_KIND;
    let object = (rules.object.as_ref()).is_some_and(|objects| objects.simple);
    let patterned = (rules.object.as_ref()).is_some_and(|objects| {
        !objects.patterns.is_empty() && objects.names.is_none() && !objects.closing
    });
    // Held to nothing after its fields but how many they are: `simple`, or that beside.
    let counted = (rules.object.as_ref()).is_some_and(|objects| objects.counted);
    // Names no property, and holds every field to one schema.
    let map = (rules.object.as_ref()).is_some_and(|objects| {
        objects.properties.is_empty() && matches!(objects.extra, Extra::Checked(_))
    });
    // Held to the names it requires and to nothing of its fields.
    let named = (rules.object.as_ref()).is_some_and(|objects| {
        objects.properties.is_empty() && matches!(objects.extra, Extra::Absent | Extra::Allowed)
    });
    let array = (rules.array.as_ref()).is_some_and(|arrays| {
        arrays.prefix.is_empty() && !arrays.unique && arrays.contains.is_none()
    });
    // One of the two unions and no other combination: alone, or beside an object's keywords.
    let one_union = rules.all_of.is_empty()
        && rules.not.is_none()
        && rules.condition.is_none()
        && rules.any_of.is_empty() != rules.one_of.is_empty();
    let union = one_union && rules.has & !(HAS_KIND | BOTH) == 0;
    // One combination alone, with not even a `type` beside it.
    let alone = rules.has == HAS_COMBINED && rules.condition.is_none();
    let unions = rules.any_of.is_empty() && rules.one_of.is_empty();
    let (all, not) = (!rules.all_of.is_empty(), rules.not.is_some());
    match (bare, own) {
        (true, HAS_OBJECT) if counted && named => Lane::Required,
        (true, HAS_OBJECT) if counted && map => Lane::Map,
        (true, HAS_OBJECT) if object => Lane::Object,
        (true, HAS_OBJECT) if patterned => Lane::Patterned,
        (true, HAS_OBJECT) => Lane::Fields,
        (true, HAS_ARRAY) if array => Lane::Array,
        (true, HAS_ARRAY) => Lane::Items,
        (true, HAS_COMBINED) if union && rules.has == HAS_COMBINED => Lane::Union,
        (true, HAS_COMBINED) if alone && unions && not && !all => Lane::Not,
        (true, HAS_COMBINED) if alone && unions && all && !not => Lane::All,
        (true, HAS_NUMBER) => Lane::Number,
        (true, HAS_STRING) => Lane::Text,
        (true, HAS_COMBINED | BOTH) if union && (object || rules.object.is_none()) => Lane::Both,
        (false, 0) if rules.reference.is_none() && !unevaluated => Lane::Kind,
        (true | false, _) => Lane::Walk,
    }
}

#[cfg(test)]
#[path = "leaves_tests.rs"]
mod tests;
