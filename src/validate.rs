//! Data walked against the compiled nodes. The quick walk stops at the first failure and says
//! nothing; the full one, run only once the quick one failed, says every failure in the order the
//! keywords are read: `$ref`, `type`, `enum`, `const`, the combinations, then the value's own.
//! Valid data costs no allocation and no keyword read.

mod containers;
mod cursor;
mod lanes;
mod objects;
mod partial;
mod scalars;
mod shortcut;
mod unevaluated;

use serde_json::Value;

use crate::node::{
    HAS_ARRAY, HAS_COMBINED, HAS_KIND, HAS_NUMBER, HAS_OBJECT, HAS_STRING, Id, Node, Rules,
};
use crate::number::json_eq;
use crate::pattern::Pattern;
use crate::report::{At, Report, said, short, type_of};
use crate::text::same_text;
use scalars::bounds;

/// The most `$ref`s followed in a row with no step into the data: a schema that refers to
/// itself ends there.
pub(crate) const DEPTH: usize = 32;

/// The compiled schema a walk reads.
pub(crate) struct Walk<'s> {
    pub(crate) nodes: &'s [Node],
    /// The schema's patterns, each its source and its matcher (none where it is not checked).
    pub(crate) patterns: &'s [(Box<str>, Option<Pattern>)],
    /// Draft 4's integers: only those written without a fraction.
    pub(crate) strict: bool,
    /// Whether a data object's fields come in the order of their names (serde_json's own map).
    pub(crate) sorted: bool,
}

impl Walk<'_> {
    /// Whether `value`, at `at`, matches node `id`; its failures said in `r`. `refs`: the
    /// `$ref`s followed since the walk last stepped into the data.
    pub(crate) fn node<const F: bool>(
        &self,
        id: Id,
        value: &Value,
        at: &At<'_>,
        mut r: Report<'_, F>,
        refs: usize,
    ) -> bool {
        let rules = match self.nodes.get(id) {
            Some(Node::Rules(rules)) => rules,
            Some(Node::Types(types, names)) => return self.typed(*types, *names, value, at, r),
            Some(Node::Never) => {
                r.no(|| said!("{at} is not allowed"));
                return false;
            }
            Some(Node::Any) | None => return true,
        };
        if !F && let Some(plan) = &rules.plain_array {
            return self.plain(plan, value);
        }
        // A `$ref` and nothing else is what it lands on: walked there, with nothing after.
        if let Some(target) = rules.alias {
            if refs >= DEPTH {
                r.no(|| said!("{at}: the schema's $ref nests deeper than {DEPTH}"));
                return false;
            }
            return self.node(target, value, at, r, refs + 1);
        }
        if let Some(target) = rules.reference {
            if refs >= DEPTH {
                r.no(|| said!("{at}: the schema's $ref nests deeper than {DEPTH}"));
                return false;
            }
            let ok = self.node(target, value, at, r.part(), refs + 1);
            if !r.took(ok) {
                return false;
            }
        }
        // A walk that says nothing has one way through a node's own keywords (`direct`), kept
        // once for both this walk and the lanes.
        if !F {
            return self.direct(rules, value, refs)
                && self.unevaluated(rules, value, at, &mut r, refs);
        }
        // Each part stops the rest once quick and failed: `r.ok()` says how it went.
        let has = |group: u8| rules.has & group != 0;
        // A quick walk reads the value's own keywords (a count fails at once) before the
        // combinations; a full one says the combinations' failures first.
        let full = r.full();
        // What no other keyword looked at is worked out from all of them: last, and only when
        // the rest held, if quick.
        let _ = (!has(HAS_KIND) || self.kind(rules, value, at, &mut r))
            && (full || self.own(rules, value, at, &mut r))
            && (!has(HAS_COMBINED) || self.combined(rules, value, at, &mut r, refs))
            && (!full || self.own(rules, value, at, &mut r))
            && self.unevaluated(rules, value, at, &mut r, refs);
        r.ok()
    }

    /// `unevaluatedItems` of an array, `unevaluatedProperties` of an object. Inlined in `node`,
    /// its one caller, so a node that has neither pays for no call.
    #[inline(always)]
    fn unevaluated<const F: bool>(
        &self,
        rules: &Rules,
        value: &Value,
        at: &At<'_>,
        r: &mut Report<'_, F>,
        refs: usize,
    ) -> bool {
        match value {
            Value::Array(items) if rules.unevaluated_items.is_some() && !rules.every_item_seen => {
                self.unevaluated_items(rules, value, items, at, r, refs)
            }
            Value::Object(fields)
                if rules.unevaluated_fields.is_some() && !rules.every_field_seen =>
            {
                self.unevaluated_fields(rules, value, fields, at, r, refs)
            }
            Value::Null
            | Value::Bool(_)
            | Value::Number(_)
            | Value::String(_)
            | Value::Array(_)
            | Value::Object(_) => true,
        }
    }

    /// The keywords of the value's own kind: a number's bounds, a string's, an array's items, an
    /// object's fields.
    fn own<const F: bool>(
        &self,
        rules: &Rules,
        value: &Value,
        at: &At<'_>,
        r: &mut Report<'_, F>,
    ) -> bool {
        let has = |group: u8| rules.has & group != 0;
        match value {
            Value::Number(n) => !has(HAS_NUMBER) || bounds(rules, n, at, r),
            Value::String(text) => !has(HAS_STRING) || self.text(rules, text, at, r),
            Value::Array(_) if !has(HAS_ARRAY) => true,
            Value::Object(_) if !has(HAS_OBJECT) => true,
            Value::Array(items) => {
                (rules.array.as_ref()).is_none_or(|arrays| self.items(arrays, items, at, r))
            }
            Value::Object(fields) => (rules.object.as_ref())
                .is_none_or(|objects| self.fields(objects, value, fields, at, r)),
            Value::Null | Value::Bool(_) => true,
        }
    }

    /// Whether `value` matches node `id`, saying nothing.
    fn quick(&self, id: Id, value: &Value, refs: usize) -> bool {
        self.holds(id, value, refs)
    }

    /// `allOf`, `anyOf`, `oneOf`, `not`, and `if` with its `then` and `else`.
    fn combined<const F: bool>(
        &self,
        rules: &Rules,
        value: &Value,
        at: &At<'_>,
        r: &mut Report<'_, F>,
        refs: usize,
    ) -> bool {
        for id in &rules.all_of {
            // A quick walk takes each part by its shortest way (`holds`).
            let ok = match F {
                true => self.fits(*id, value) || self.node(*id, value, at, r.part(), refs),
                false => self.holds(*id, value, refs),
            };
            if !r.took(ok) {
                return false;
            }
        }
        let any = &rules.any_of;
        if !any.is_empty()
            && self.matched(any, rules.any_tag.as_deref(), value, refs, 1) == 0
            && !r.no(|| said!("{at} matches none of the {} schemas of anyOf", any.len()))
        {
            return false;
        }
        let one = &rules.one_of;
        if !one.is_empty() {
            let matched = self.matched(one, rules.one_tag.as_deref(), value, refs, 2);
            if matched != 1
                && !r.no(|| {
                    let matches = if matched == 0 {
                        "none"
                    } else {
                        "more than one"
                    };
                    said!("{at} must match exactly one schema of oneOf (matches {matches})")
                })
            {
                return false;
            }
        }
        if let Some(not) = rules.not
            && self.quick(not, value, refs)
            && !r.no(|| said!("{at} must not match the schema of not"))
        {
            return false;
        }
        let Some((when, then, otherwise)) = rules.condition else {
            return true;
        };
        let branch = match self.quick(when, value, refs) {
            true => then,
            false => otherwise,
        };
        match branch {
            Some(id) => {
                let ok = match F {
                    true => self.node(id, value, at, r.part(), refs),
                    false => self.holds(id, value, refs),
                };
                r.took(ok)
            }
            None => true,
        }
    }
}

impl Walk<'_> {
    /// `type`, `enum` and `const`. Inlined in `node`, its one caller: most nodes ask a type, and
    /// a call for each would cost more than the check.
    #[inline(always)]
    fn kind<const F: bool>(
        &self,
        rules: &Rules,
        value: &Value,
        at: &At<'_>,
        r: &mut Report<'_, F>,
    ) -> bool {
        let kind = self.kind_for(rules.types.unwrap_or(0), value);
        if let Some(types) = rules.types
            && types & kind == 0
            && !r.no(|| {
                let wanted = rules.type_names.names();
                said!(
                    "{at} must be {} (is {})",
                    wanted.join(" or "),
                    type_of(kind)
                )
            })
        {
            return false;
        }
        if let Some(whole @ Value::Array(options)) = &rules.options
            && !(rules.option_kinds & kind != 0
                && match (&rules.table, value) {
                    (Some(table), _) => table.holds(options, value),
                    // A string is only another string (`json_eq`), compared where it stands.
                    (None, Value::String(text)) => options.iter().any(
                        |option| matches!(option, Value::String(known) if same_text(known, text)),
                    ),
                    (None, _) => options.iter().any(|option| json_eq(option, value)),
                })
            && !r.no(|| said!("{at} must be one of {} (is {})", short(whole), short(value)))
        {
            return false;
        }
        match &rules.constant {
            Some(one) if !json_eq(one, value) => {
                r.no(|| said!("{at} must be {} (is {})", short(one), short(value)))
            }
            Some(_) | None => true,
        }
    }

    /// Whether `text` matches the pattern at `place` (one not checked here matches anything).
    pub(super) fn matches(&self, place: usize, text: &str) -> bool {
        match self.patterns.get(place) {
            Some((_, Some(pattern))) => pattern.is_match(text),
            Some((_, None)) | None => true,
        }
    }
}
