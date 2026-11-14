//! What the walk settles without walking: a leaf that asks only a type the value has (or a
//! `$ref` to one), and the branches of a tagged union its value can match.

use serde_json::Value;

use super::Walk;
use crate::compile::tags::Tag;
use crate::node::{
    ALL, ARRAY, BOOLEAN, HAS_KIND, HAS_STRING, Id, Items, Leaf, NULL, NUMBER, Node, OBJECT,
    PlainArray, Rules, STRING, kind_of,
};
use crate::report::{At, Report, said};
use crate::text::same_text;

/// Whether `types` take a number with a fraction.
fn fractions(types: u8) -> bool {
    types & NUMBER != 0
}

impl Walk<'_> {
    /// A field's `name` held to `propertyNames`' node `id`, read where it is, with no string
    /// made of it: where the node asks only a string's keywords (a pattern, a length, `false`);
    /// `None` where it asks more.
    pub(super) fn name<const F: bool>(
        &self,
        id: Id,
        name: &str,
        at: &At<'_>,
        mut r: Report<'_, F>,
    ) -> Option<bool> {
        let rules = match self.nodes.get(id) {
            Some(Node::Any) => return Some(true),
            Some(Node::Never) => {
                r.no(|| said!("{at} is not allowed"));
                return Some(false);
            }
            None => return None,
            // A name is a string: one asked to be a string matches, one asked not to is walked.
            Some(Node::Types(types, _)) => return (types & STRING != 0).then_some(true),
            Some(Node::Rules(rules)) => rules,
        };
        let types_only = rules.has & !(HAS_KIND | HAS_STRING) == 0
            && rules.reference.is_none()
            && rules.options.is_none()
            && rules.constant.is_none();
        if !types_only || rules.types.is_some_and(|types| types & STRING == 0) {
            return None;
        }
        Some(rules.has & HAS_STRING == 0 || self.text(rules, name, at, &mut r))
    }

    /// Whether `value` surely matches node `id` without walking it: the node allows anything,
    /// or asks only for types the value has. `false` says only that the walk must decide.
    #[inline]
    pub(super) fn fits(&self, id: Id, value: &Value) -> bool {
        self.types_of(id)
            .is_some_and(|types| self.has_type(types, value))
    }

    /// An item of a list walked against node `id`, its failures said in `r`. A walk that says
    /// them asks the quick walk first: of a long list that fails, most items hold, and what
    /// holds is told with nothing written and no place named.
    #[inline]
    pub(super) fn said<const F: bool>(
        &self,
        id: Id,
        value: &Value,
        at: &At<'_>,
        r: Report<'_, F>,
    ) -> bool {
        self.holds(id, value, 0) || (F && self.node(id, value, at, r, 0))
    }

    /// A value inside the data walked against node `id`: by the quick walk's lanes where
    /// nothing is said (`holds`), as every node is where its failures are (`node`).
    #[inline]
    pub(super) fn child<const F: bool>(
        &self,
        id: Id,
        value: &Value,
        at: &At<'_>,
        r: Report<'_, F>,
    ) -> bool {
        match F {
            true => self.node(id, value, at, r, 0),
            false => self.holds(id, value, 0),
        }
    }

    /// Whether node `id` is a leaf that surely holds `value` (`Leaf`), asked of a field's value
    /// where `fits` could not say: out of the walk's way, so its every call does not grow by it.
    pub(crate) fn leaf_holds(&self, id: Id, value: &Value) -> bool {
        self.leaf(id, value) == Some(true)
    }

    /// What the leaf node `id` says of `value`: that it holds it, that it surely does not (a
    /// quick walk then fails with nothing walked), or nothing, where the walk must decide (a
    /// node that is no leaf, a fraction, a string to count by characters).
    #[inline(never)]
    pub(crate) fn leaf(&self, id: Id, value: &Value) -> Option<bool> {
        match self.nodes.get(id) {
            Some(Node::Rules(rules)) => self.settles(rules, value),
            Some(Node::Any | Node::Never | Node::Types(..)) | None => None,
        }
    }

    /// What the leaf `rules` say of `value` (`leaf`).
    #[inline]
    pub(super) fn settles(&self, rules: &Rules, value: &Value) -> Option<bool> {
        match rules.leaf {
            Leaf::Walked => None,
            // A value that is no string is none of the strings.
            Leaf::Strings => {
                Some(
                    value
                        .as_str()
                        .is_some_and(|text| match (&rules.table, &rules.options) {
                            (Some(table), Some(Value::Array(options))) => {
                                table.holds_text(options, text)
                            }
                            (None, Some(Value::Array(options))) => (options.iter()).any(|option| {
                                option.as_str().is_some_and(|known| same_text(known, text))
                            }),
                            (Some(_) | None, Some(_) | None) => false,
                        }),
                )
            }
            Leaf::Const => rules.constant.as_ref().map(|known| {
                match (known.as_str(), value.as_str()) {
                    (Some(known), Some(text)) => same_text(known, text),
                    // A boolean or null: itself alone.
                    (None, None) => known == value,
                    (Some(_), None) | (None, Some(_)) => false,
                }
            }),
            // An integer written as one is settled, and a fraction where any number is taken
            // (whether `1.0` is an integer is the draft's to read): beside bounds a float holds
            // exactly, it is compared as the float it is.
            Leaf::Range => {
                match (value.as_i64(), value.as_f64()) {
                    (Some(n), _) => Some((rules.limits.iter()).all(|(limit, sign)| {
                        limit.as_i64().is_some_and(|l| sign.holds_whole(n, l))
                    })),
                    (None, Some(f)) if value.is_f64() && rules.types.is_none_or(fractions) => {
                        let small = |limit: &serde_json::Number| {
                            limit
                                .as_i64()
                                .and_then(|l| i32::try_from(l).ok())
                                .map(f64::from)
                        };
                        (rules.limits.iter())
                            .map(|(limit, sign)| small(limit).map(|l| sign.holds_whole(f, l)))
                            .try_fold(true, |all, holds| holds.map(|holds| all && holds))
                    }
                    (None, Some(_) | None) => None,
                }
            }
            Leaf::Text => {
                let text = value.as_str()?;
                let bytes = text.len() as u64;
                let length = rules.max_length.is_none_or(|most| bytes <= most)
                    && rules.min_length.is_none_or(|least| bytes / 4 >= least);
                // A length the count of bytes does not settle is counted by the walk.
                length.then(|| {
                    rules.pattern.is_none_or(|place| self.matches(place, text))
                        && (rules.format.as_ref()).is_none_or(|(_, check)| check(text.as_bytes()))
                })
            }
        }
    }

    /// How many of a union's branches `value` matches, counted up to `most` (one settles an
    /// `anyOf`, two a `oneOf`): of the branches its tag leaves, or of them all, each tried where
    /// it stands before it is walked.
    pub(super) fn matched(
        &self,
        branches: &[Id],
        tag: Option<&Tag>,
        value: &Value,
        refs: usize,
        most: usize,
    ) -> usize {
        let mut count = 0;
        let mut enough = |id: Id| {
            let holds = self.holds(id, value, refs);
            count += usize::from(holds);
            count >= most
        };
        match tag.and_then(|tag| tag.picks(value)) {
            Some((tagged, untagged)) => {
                for place in tagged.iter().chain(untagged) {
                    let id = usize::try_from(*place).ok().and_then(|at| branches.get(at));
                    if id.is_some_and(|id| enough(*id)) {
                        break;
                    }
                }
            }
            None => {
                for id in branches {
                    if enough(*id) {
                        break;
                    }
                }
            }
        }
        count
    }

    /// Whether `value` matches the array `plan` describes: its type, its count, its items' type.
    #[inline]
    pub(super) fn plain(&self, plan: &PlainArray, value: &Value) -> bool {
        match value {
            Value::Array(items) => {
                let len = items.len() as u64;
                plan.types & ARRAY != 0
                    && (plan.min..=plan.max).contains(&len)
                    && match plan.items {
                        Items::Types(ALL) => true,
                        Items::Types(types) => items.iter().all(|item| self.has_type(types, item)),
                        Items::Plain(id) => match self.nodes.get(id) {
                            Some(Node::Rules(rules)) => {
                                rules.plain_array.as_ref().is_some_and(|inner| {
                                    items.iter().all(|item| self.plain(inner, item))
                                })
                            }
                            Some(Node::Any | Node::Never | Node::Types(..)) | None => false,
                        },
                    }
            }
            Value::Null
            | Value::Bool(_)
            | Value::Number(_)
            | Value::String(_)
            | Value::Object(_) => self.has_type(plan.types, value),
        }
    }

    /// The types node `id` asks for, where that is all it asks (a `$ref` that is all a node
    /// asks was given its target's types as it compiled); every type for one that allows all.
    #[inline(always)]
    pub(super) fn types_of(&self, id: Id) -> Option<u8> {
        match self.nodes.get(id) {
            Some(Node::Any) => Some(ALL),
            Some(Node::Types(types, _)) => Some(*types),
            Some(Node::Rules(rules)) => rules.only_types,
            Some(Node::Never) | None => None,
        }
    }

    /// Whether `value` is of one of `types`.
    #[inline(always)]
    pub(super) fn has_type(&self, types: u8, value: &Value) -> bool {
        types & self.kind_for(types, value) != 0
    }

    /// The bits of `value`'s kind as `types` reads it: whether a number is whole is read only
    /// where the types tell an integer from a number (a number asked to be any number is one).
    #[inline(always)]
    pub(super) fn kind_for(&self, types: u8, value: &Value) -> u8 {
        match value {
            Value::Null => NULL,
            Value::Bool(_) => BOOLEAN,
            Value::String(_) => STRING,
            Value::Array(_) => ARRAY,
            Value::Object(_) => OBJECT,
            Value::Number(_) if types & NUMBER != 0 => NUMBER,
            Value::Number(_) => kind_of(value, self.strict),
        }
    }
}
