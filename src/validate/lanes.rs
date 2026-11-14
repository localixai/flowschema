//! The quick walk's lanes: most nodes that are no leaf are an object or an array and no more
//! (`Lane`), and a quick walk goes through them with nothing else looked at: no report, no
//! place, no keyword asked that the node does not have. Whatever asks more is walked as every
//! node is (`Walk::node`), and says the same.

use serde_json::Value;

use super::{DEPTH, Walk};
use crate::node::{ARRAY, Extra, Id, Lane, Leaf, Node, OBJECT, Objects};
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
            (Lane::Union, _, _, _) => match rules.any_of.is_empty() {
                false => self.matched(&rules.any_of, rules.any_tag.as_deref(), value, refs, 1) > 0,
                true => self.matched(&rules.one_of, rules.one_tag.as_deref(), value, refs, 2) == 1,
            },
            (Lane::Walk | Lane::Object | Lane::Array, _, _, _) => {
                self.node(id, value, &At::Root, Report::quiet(), refs)
            }
        }
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
