//! The quick walk's lanes: most nodes that are no leaf are an object or an array and no more
//! (`Lane`), and a quick walk goes through them with nothing else looked at: no report, no
//! place, no keyword asked that the node does not have. Whatever asks more is walked as every
//! node is (`Walk::node`), and says the same.

use serde_json::Value;

use super::scalars::bounds;
use super::shortcut;
use super::{DEPTH, Walk};
use crate::node::{
    ARRAY, Extra, HAS_ARRAY, HAS_COMBINED, HAS_NUMBER, HAS_OBJECT, HAS_STRING, Id, Lane, Leaf,
    Node, OBJECT, Objects, Rules,
};
use crate::report::{At, Report};

impl Walk<'_> {
    /// Whether `value` matches node `id`, saying nothing: what `node` answers a quick walk, by
    /// the shortest way the node allows. A node of rules is walked by the way chosen for it as
    /// the schema was compiled (`way`): one call, with no question asked here of what the node
    /// is, so a way added for one shape of node costs no other node anything.
    #[inline(always)]
    pub(crate) fn holds(&self, id: Id, value: &Value, refs: usize) -> bool {
        match self.nodes.get(id) {
            Some(Node::Rules(rules)) => (rules.way.0)(self, id, rules, value, refs),
            Some(node) => self.plain_node(node, value),
            None => true,
        }
    }

    /// `holds` for a node that is no node of rules: a call of its own, so that `holds` is no
    /// more where it is called than a look at the node and a call.
    #[inline(never)]
    fn plain_node(&self, node: &Node, value: &Value) -> bool {
        match node {
            Node::Types(types, _) => self.has_type(*types, value),
            Node::Never => false,
            Node::Any | Node::Rules(_) => true,
        }
    }

    /// A node no lane takes, with no `$ref` beside its keywords and nothing unevaluated,
    /// walked with nothing said: its type first, then only what the value's kind and the
    /// node's keywords ask, each gone to directly. What `node` answers, by a shorter way.
    #[inline(never)]
    pub(super) fn direct(&self, rules: &Rules, value: &Value, refs: usize) -> bool {
        if rules
            .types
            .is_some_and(|types| !self.has_type(types, value))
        {
            return false;
        }
        let has = |group: u8| rules.has & group != 0;
        let mut quiet = Report::quiet();
        let (at, r) = (&At::Root, &mut quiet);
        let listed = rules.options.is_none() && rules.constant.is_none();
        (listed || self.kind(rules, value, at, r))
            && match value {
                Value::Number(n) => !has(HAS_NUMBER) || bounds(rules, n, at, r),
                Value::String(text) => !has(HAS_STRING) || self.text(rules, text, at, r),
                Value::Array(items) => (rules.array.as_ref())
                    .is_none_or(|arrays| !has(HAS_ARRAY) || self.items(arrays, items, at, r)),
                Value::Object(fields) => (rules.object.as_ref()).is_none_or(|objects| {
                    !has(HAS_OBJECT) || self.fields(objects, value, fields, at, r)
                }),
                Value::Null | Value::Bool(_) => true,
            }
            && (!has(HAS_COMBINED) || self.combined(rules, value, at, r, refs))
    }
}

/// A way through a node of rules: the node's place, its rules, the value, the `$ref`s followed.
pub(crate) type Way = fn(&Walk<'_>, Id, &Rules, &Value, usize) -> bool;

/// The way a quick walk takes through `rules`, chosen once every pass over the nodes is done.
pub(crate) fn way(rules: &Rules) -> Way {
    match (
        rules.only_types,
        &rules.plain_array,
        rules.alias,
        rules.leaf,
    ) {
        (Some(_), _, _, _) => types,
        (None, Some(_), _, _) => plain,
        (None, None, Some(_), _) => alias,
        (None, None, None, Leaf::Strings) => strings,
        (None, None, None, Leaf::Const) => constant,
        (None, None, None, Leaf::Range) => range,
        (None, None, None, Leaf::Text) => string,
        (None, None, None, Leaf::Walked) => by_lane(rules.lane),
    }
}

/// The way of each lane.
fn by_lane(lane: Lane) -> Way {
    match lane {
        Lane::Object => object,
        Lane::Array => array,
        Lane::Union => union,
        Lane::Not => not,
        Lane::All => all,
        Lane::Number => number,
        Lane::Text => text,
        Lane::Fields => object,
        Lane::Items => items,
        Lane::Kind => kind,
        Lane::Required => required,
        Lane::Patterned => patterned,
        Lane::Map => map,
        Lane::Both => both,
        Lane::Walk => rest,
    }
}

fn types(walk: &Walk<'_>, _: Id, rules: &Rules, value: &Value, _: usize) -> bool {
    (rules.only_types).is_none_or(|types| walk.has_type(types, value))
}

fn plain(walk: &Walk<'_>, _: Id, rules: &Rules, value: &Value, _: usize) -> bool {
    (rules.plain_array.as_ref()).is_none_or(|plan| walk.plain(plan, value))
}

/// A `$ref` and nothing else is what it lands on.
fn alias(walk: &Walk<'_>, _: Id, rules: &Rules, value: &Value, refs: usize) -> bool {
    (rules.alias).is_none_or(|target| refs < DEPTH && walk.holds(target, value, refs + 1))
}

/// A leaf says so where it stands, held or surely not, each kind by its own function; what
/// one leaves open (a fraction where integers are asked, an integer past what a float holds)
/// goes by its lane.
fn strings(_: &Walk<'_>, _: Id, rules: &Rules, value: &Value, _: usize) -> bool {
    shortcut::strings(rules, value)
}

fn constant(walk: &Walk<'_>, id: Id, rules: &Rules, value: &Value, refs: usize) -> bool {
    shortcut::constant(rules, value)
        .unwrap_or_else(|| by_lane(rules.lane)(walk, id, rules, value, refs))
}

fn range(walk: &Walk<'_>, id: Id, rules: &Rules, value: &Value, refs: usize) -> bool {
    Walk::range(rules, value).unwrap_or_else(|| by_lane(rules.lane)(walk, id, rules, value, refs))
}

fn string(walk: &Walk<'_>, id: Id, rules: &Rules, value: &Value, refs: usize) -> bool {
    (walk.string(rules, value)).unwrap_or_else(|| by_lane(rules.lane)(walk, id, rules, value, refs))
}

fn not(walk: &Walk<'_>, _: Id, rules: &Rules, value: &Value, refs: usize) -> bool {
    rules.not.is_none_or(|not| !walk.holds(not, value, refs))
}

fn all(walk: &Walk<'_>, _: Id, rules: &Rules, value: &Value, refs: usize) -> bool {
    (rules.all_of.iter()).all(|part| walk.holds(*part, value, refs))
}

/// A number held to its type and its own keywords; a value of another kind, to the type alone.
fn number(walk: &Walk<'_>, _: Id, rules: &Rules, value: &Value, _: usize) -> bool {
    rules.types.is_none_or(|types| walk.has_type(types, value))
        && match value {
            Value::Number(n) => bounds(rules, n, &At::Root, &mut Report::quiet()),
            Value::Null
            | Value::Bool(_)
            | Value::String(_)
            | Value::Array(_)
            | Value::Object(_) => true,
        }
}

/// A string held to its type and its own keywords; a value of another kind, to the type alone.
fn text(walk: &Walk<'_>, _: Id, rules: &Rules, value: &Value, _: usize) -> bool {
    rules.types.is_none_or(|types| walk.has_type(types, value))
        && match value {
            Value::String(text) => walk.text(rules, text, &At::Root, &mut Report::quiet()),
            Value::Null
            | Value::Bool(_)
            | Value::Number(_)
            | Value::Array(_)
            | Value::Object(_) => true,
        }
}

/// An object held to its type and its own keywords: in the order of its fields where it can
/// be (`Walk::in_order`), by `Walk::fields` where not; a value of another kind, to the type.
fn object(walk: &Walk<'_>, _: Id, rules: &Rules, value: &Value, _: usize) -> bool {
    match (value, &rules.object) {
        (Value::Object(fields), Some(objects)) => {
            let ordered = walk.sorted && objects.simple && in_order(objects, fields.len());
            rules.types.is_none_or(|types| types & OBJECT != 0)
                && match ordered {
                    true => walk.in_order(objects, fields),
                    false => walk.fields(objects, value, fields, &At::Root, &mut Report::quiet()),
                }
        }
        (_, _) => rules.types.is_none_or(|types| walk.has_type(types, value)),
    }
}

/// An object held to the names it requires, each looked up; no field of it is read.
fn required(walk: &Walk<'_>, _: Id, rules: &Rules, value: &Value, _: usize) -> bool {
    match (value, &rules.object) {
        (Value::Object(fields), Some(objects)) => {
            rules.types.is_none_or(|types| types & OBJECT != 0)
                && (objects.required.iter())
                    .all(|it| fields.contains_key(objects.required_name(it)))
        }
        (_, _) => rules.types.is_none_or(|types| walk.has_type(types, value)),
    }
}

/// A map: every field held to one schema, by its types where they are all the schema asks.
fn map(walk: &Walk<'_>, _: Id, rules: &Rules, value: &Value, _: usize) -> bool {
    match (value, &rules.object) {
        (Value::Object(fields), Some(objects)) => {
            let held = |value: &Value| match (objects.extra_types, &objects.extra) {
                (0, Extra::Checked(id)) => walk.holds(*id, value, 0),
                (0, Extra::Absent | Extra::Allowed | Extra::Refused) => true,
                (types, _) => walk.has_type(types, value),
            };
            rules.types.is_none_or(|types| types & OBJECT != 0)
                && (objects.required.iter())
                    .all(|it| fields.contains_key(objects.required_name(it)))
                && fields.values().all(held)
        }
        (_, _) => rules.types.is_none_or(|types| walk.has_type(types, value)),
    }
}

/// An object as `object` reads it (or a type alone), and the one union beside it.
fn both(walk: &Walk<'_>, id: Id, rules: &Rules, value: &Value, refs: usize) -> bool {
    object(walk, id, rules, value, refs) && union(walk, id, rules, value, refs)
}

/// An object held to patterns for its names (`Walk::patterned`).
fn patterned(walk: &Walk<'_>, _: Id, rules: &Rules, value: &Value, _: usize) -> bool {
    match (value, &rules.object) {
        (Value::Object(fields), Some(objects)) => {
            rules.types.is_none_or(|types| types & OBJECT != 0)
                && walk.patterned(objects, value, fields)
        }
        (_, _) => rules.types.is_none_or(|types| walk.has_type(types, value)),
    }
}

/// An array held to its type, its count and one schema for every item.
fn array(walk: &Walk<'_>, _: Id, rules: &Rules, value: &Value, _: usize) -> bool {
    match (value, &rules.array) {
        (Value::Array(items), Some(arrays)) => {
            let len = items.len() as u64;
            rules.types.is_none_or(|types| types & ARRAY != 0)
                && arrays.min_items.is_none_or(|least| len >= least)
                && arrays.max_items.is_none_or(|most| len <= most)
                && (arrays.rest)
                    .is_none_or(|rest| items.iter().all(|item| walk.holds(rest, item, 0)))
        }
        (_, _) => rules.types.is_none_or(|types| walk.has_type(types, value)),
    }
}

/// An array held to its type and any keyword of an array (`Walk::items`).
fn items(walk: &Walk<'_>, _: Id, rules: &Rules, value: &Value, _: usize) -> bool {
    match (value, &rules.array) {
        (Value::Array(items), Some(arrays)) => {
            rules.types.is_none_or(|types| types & ARRAY != 0)
                && walk.items(arrays, items, &At::Root, &mut Report::quiet())
        }
        (_, _) => rules.types.is_none_or(|types| walk.has_type(types, value)),
    }
}

/// `type`, `enum` and `const` (`Walk::kind`).
fn kind(walk: &Walk<'_>, _: Id, rules: &Rules, value: &Value, _: usize) -> bool {
    walk.kind(rules, value, &At::Root, &mut Report::quiet())
}

fn union(walk: &Walk<'_>, _: Id, rules: &Rules, value: &Value, refs: usize) -> bool {
    let tagged = rules.any_tag.is_some() || rules.one_tag.is_some();
    match (&rules.listed, rules.any_of.is_empty(), tagged) {
        // A union of names holds the strings of its list and nothing else.
        (Some(listed), _, _) => value.as_str().is_some_and(|text| listed.holds(text)),
        // With no tag to pick a branch by, each is tried in turn: the first that holds
        // settles an `anyOf`, a second one a `oneOf`.
        (None, false, false) => (rules.any_of.iter()).any(|id| walk.holds(*id, value, refs)),
        (None, true, false) => {
            let mut held = (rules.one_of.iter()).filter(|id| walk.holds(**id, value, refs));
            held.next().is_some() && held.next().is_none()
        }
        (None, false, true) => {
            walk.matched(&rules.any_of, rules.any_tag.as_deref(), value, refs, 1) > 0
        }
        (None, true, true) => {
            walk.matched(&rules.one_of, rules.one_tag.as_deref(), value, refs, 2) == 1
        }
    }
}

/// A node no lane takes: by `direct`, or by `node` where a `$ref` stands beside keywords or
/// what was not evaluated is asked.
pub(crate) fn rest(walk: &Walk<'_>, id: Id, rules: &Rules, value: &Value, refs: usize) -> bool {
    let unevaluated = rules.unevaluated_fields.is_some() || rules.unevaluated_items.is_some();
    match rules.reference.is_some() || unevaluated {
        true => walk.node(id, value, &At::Root, Report::quiet(), refs),
        false => walk.direct(rules, value, refs),
    }
}

/// Whether an object of `given` fields is walked in the order of its fields: so most are,
/// unless there are far more properties than fields (found by hash) or far more fields than
/// properties look at (each property looked up).
pub(super) fn in_order(objects: &Objects, given: usize) -> bool {
    let known = objects.properties.len();
    let hashed = objects.lookup.is_some() && known > given.saturating_mul(4);
    let open = matches!(objects.extra, Extra::Absent | Extra::Allowed);
    let looked_up = open && known > 0 && known.saturating_mul(4) <= given;
    !hashed && !looked_up
}

#[cfg(test)]
#[path = "lanes_tests.rs"]
mod tests;
