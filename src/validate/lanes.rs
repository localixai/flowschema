//! The quick walk's lanes: most nodes that are no leaf are an object or an array and no more
//! (`Lane`), and a quick walk goes through them with nothing else looked at: no report, no
//! place, no keyword asked that the node does not have. Whatever asks more is walked as every
//! node is (`Walk::node`), and says the same.

use serde_json::Value;

use super::scalars::bounds;
use super::{DEPTH, Walk};
use crate::node::{
    ARRAY, Extra, HAS_ARRAY, HAS_COMBINED, HAS_NUMBER, HAS_OBJECT, HAS_STRING, Id, Lane, Leaf,
    Node, OBJECT, Objects, Rules,
};
use crate::report::{At, Report};

impl Walk<'_> {
    /// Whether `value` matches node `id`, saying nothing: what `node` answers a quick walk, by
    /// the shortest way the node allows.
    pub(crate) fn holds(&self, id: Id, value: &Value, refs: usize) -> bool {
        let rules = match self.nodes.get(id) {
            Some(Node::Rules(rules)) => rules,
            Some(Node::Types(types, _)) => return self.has_type(*types, value),
            Some(Node::Never) => return false,
            Some(Node::Any) | None => return true,
        };
        if let Some(types) = rules.only_types {
            return self.has_type(types, value);
        }
        if let Some(plan) = &rules.plain_array {
            return self.plain(plan, value);
        }
        // A `$ref` and nothing else is what it lands on.
        if let Some(target) = rules.alias {
            return refs < DEPTH && self.holds(target, value, refs + 1);
        }
        // A leaf says so where it stands, held or surely not.
        if rules.leaf != Leaf::Walked
            && let Some(holds) = self.settles(rules, value)
        {
            return holds;
        }
        let typed = |bit: u8| rules.types.is_none_or(|types| types & bit != 0);
        match (rules.lane, value, &rules.object, &rules.array) {
            (Lane::Object, Value::Object(fields), Some(objects), _)
                if self.sorted && in_order(objects, fields.len()) =>
            {
                typed(OBJECT) && self.in_order(objects, fields)
            }
            (Lane::Array, Value::Array(items), _, Some(arrays)) => {
                let len = items.len() as u64;
                typed(ARRAY)
                    && arrays.min_items.is_none_or(|least| len >= least)
                    && arrays.max_items.is_none_or(|most| len <= most)
                    && (arrays.rest)
                        .is_none_or(|rest| items.iter().all(|item| self.holds(rest, item, 0)))
            }
            (Lane::Union, _, _, _) => match (&rules.listed, rules.any_of.is_empty()) {
                // A union of names holds the strings of its list and nothing else.
                (Some(listed), _) => value.as_str().is_some_and(|text| listed.holds(text)),
                (None, false) => {
                    self.matched(&rules.any_of, rules.any_tag.as_deref(), value, refs, 1) > 0
                }
                (None, true) => {
                    self.matched(&rules.one_of, rules.one_tag.as_deref(), value, refs, 2) == 1
                }
            },
            (Lane::Walk | Lane::Object | Lane::Array, _, _, _) => {
                let unevaluated =
                    rules.unevaluated_fields.is_some() || rules.unevaluated_items.is_some();
                match rules.reference.is_some() || unevaluated {
                    true => self.node(id, value, &At::Root, Report::quiet(), refs),
                    false => self.direct(rules, value, refs),
                }
            }
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
