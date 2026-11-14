//! A call still being written: whether what came so far can still come to match. Only what
//! more input cannot change is asked: the kind of a value, a field the schema refuses, a value
//! written whole that does not match its place, a string that no option begins with. What an
//! ending may still mend (a field not yet there, a count not yet reached) is not, and neither
//! is what only the whole value can tell (a union, a `not`): the answer errs towards yes.

use serde_json::{Map, Value};

use super::{DEPTH, Walk};
use crate::cut::Step;
use crate::node::{Extra, Id, Node, Rules};

impl Walk<'_> {
    /// Whether `value`, still written along `path` (down to a string cut midway where `text`),
    /// can come to match node `id`: false only where nothing written after it mends it.
    pub(crate) fn may_hold(
        &self,
        id: Id,
        value: &Value,
        path: &[Step],
        text: bool,
        refs: usize,
    ) -> bool {
        let rules = match self.nodes.get(id) {
            Some(Node::Rules(rules)) => rules,
            // The kind of a value does not change as it is written.
            Some(Node::Types(types, _)) => return self.has_type(*types, value),
            Some(Node::Never) => return false,
            Some(Node::Any) | None => return true,
        };
        if let Some(target) = rules.alias {
            return refs >= DEPTH || self.may_hold(target, value, path, text, refs + 1);
        }
        if rules
            .types
            .is_some_and(|types| !self.has_type(types, value))
        {
            return false;
        }
        match value {
            Value::String(written) => !(text && path.is_empty()) || begins(rules, written),
            Value::Object(fields) => self.fields_may_hold(rules, fields, path, text),
            Value::Array(items) => self.items_may_hold(rules, items, path, text),
            Value::Null | Value::Bool(_) | Value::Number(_) => true,
        }
    }

    /// An object still open: each field written whole holds its place, the one still written
    /// may come to, and none is refused.
    fn fields_may_hold(
        &self,
        rules: &Rules,
        fields: &Map<String, Value>,
        path: &[Step],
        text: bool,
    ) -> bool {
        let Some(objects) = &rules.object else {
            return true;
        };
        if (objects.max_fields).is_some_and(|most| fields.len() as u64 > most) {
            return false;
        }
        fields.iter().all(|(name, value)| {
            let named = objects.properties.iter().find(|p| &*p.name == name);
            let id = match (named, &objects.extra) {
                (Some(property), _) => property.id,
                // A pattern may take the name, and what it asks is not read here.
                (None, _) if !objects.patterns.is_empty() => return true,
                (None, Extra::Refused) => return false,
                (None, Extra::Checked(id)) => *id,
                (None, Extra::Absent | Extra::Allowed) => return true,
            };
            match path.split_first() {
                Some((Step::Key(open), rest)) if open == name => {
                    self.may_hold(id, value, rest, text, 0)
                }
                Some(_) | None => self.holds(id, value, 0),
            }
        })
    }

    /// An array still open: each item written whole holds its place, and the one still written
    /// may come to.
    fn items_may_hold(&self, rules: &Rules, items: &[Value], path: &[Step], text: bool) -> bool {
        let Some(arrays) = &rules.array else {
            return true;
        };
        if (arrays.max_items).is_some_and(|most| items.len() as u64 > most) {
            return false;
        }
        items.iter().enumerate().all(|(place, item)| {
            let Some(id) = arrays.prefix.get(place).copied().or(arrays.rest) else {
                return true;
            };
            match path.split_first() {
                Some((Step::Index(open), rest)) if *open == place => {
                    self.may_hold(id, item, rest, text, 0)
                }
                Some(_) | None => self.holds(id, item, 0),
            }
        })
    }
}

/// Whether a string cut midway can still come to what `rules` ask of it: it is no longer than
/// allowed, and where the rules name their values, one of them begins with it.
fn begins(rules: &Rules, written: &str) -> bool {
    let fits = |option: &Value| option.as_str().is_some_and(|o| o.starts_with(written));
    (rules.max_length).is_none_or(|most| written.chars().count() as u64 <= most)
        && (rules.constant.as_ref()).is_none_or(fits)
        && (rules.options.as_ref().and_then(Value::as_array))
            .is_none_or(|options| options.iter().any(fits))
}

#[cfg(test)]
#[path = "partial_tests.rs"]
mod tests;
