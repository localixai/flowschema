//! Data checked against a JSON Schema given at run time, tuned for what a language model writes
//! (a tool call's arguments, a structured answer): drafts 4, 6, 7, 2019-09 and 2020-12, read by
//! the document's `$schema` (`draft.rs`), compiled once (`compile.rs`, its references through
//! `index.rs` and `uri.rs`) and walked for each check (`validate.rs`), valid data with no
//! allocation. Formats are checked, not only noted (`format.rs`), and a pattern by its own
//! matcher (`pattern.rs`). Equality is JSON's, numbers compare exactly (`number.rs`, `hash.rs`).
//! What a schema asks that is not checked here is named by [`Schema::unchecked`], never dropped
//! unsaid.
//!
//! The README follows, its example run as a test.
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

/// The examples of `docs/`, run as tests so the pages cannot drift from the API.
#[cfg(doctest)]
#[doc = include_str!("../docs/agents.md")]
#[doc = include_str!("../docs/api.md")]
struct Docs;

mod compile;
mod cut;
mod draft;
mod fit;
mod format;
mod hash;
mod index;
mod mend;
mod node;
mod number;
mod pattern;
mod report;
mod text;
mod uri;
mod validate;

use std::sync::OnceLock;

use serde_json::Value;

pub use cut::whole;
pub use fit::{Fitted, Provider, fit};
use node::Node;
use pattern::Pattern;
use report::{At, Report};
use validate::Walk;

/// A JSON Schema compiled: an object, or `true` or `false`, read once and ready to check data
/// against. It keeps nothing of the document it was read from, which stays the caller's.
pub struct Schema {
    /// The schema compiled, its root the first node.
    nodes: Vec<Node>,
    /// Its patterns: each source, and its matcher where it is checked here.
    patterns: Vec<(Box<str>, Option<Pattern>)>,
    /// What it asks that is not checked here.
    notes: Vec<&'static str>,
    /// Draft 4's integers.
    strict: bool,
    /// Whether a data object's fields come in the order of their names (`sorted_maps`).
    sorted: bool,
    /// Whether the schema is one leaf, settled with no walk (`Walk::leaf`): asked of every
    /// check only where it can be.
    leaf: bool,
}

impl Schema {
    /// `json` compiled, its draft read from `$schema` (2020-12 when it names none). The
    /// document is read where it is, and not copied: a schema that comes with a request is
    /// compiled from the request. It never fails: what it asks that is not checked here is
    /// named by [`Schema::unchecked`].
    pub fn new(json: &Value) -> Self {
        Self::with_documents(json, &[])
    }

    /// A schema whose `$ref`s may land in other documents, each given with its URI
    /// (`https://example.com/defs.json`): no network is ever reached.
    pub fn with_documents(json: &Value, documents: &[(String, Value)]) -> Self {
        let compiled = compile::compile(json, documents);
        let mut notes = compiled.notes;
        let patterns: Vec<(Box<str>, Option<Pattern>)> = (compiled.patterns.into_iter())
            .map(|source| {
                let matcher = Pattern::new(&source).ok();
                (source, matcher)
            })
            .collect();
        if patterns.iter().any(|(_, matcher)| matcher.is_none()) {
            notes.push("pattern");
        }
        let leaf = match compiled.nodes.first() {
            Some(Node::Rules(rules)) => rules.leaf != node::Leaf::Walked,
            Some(Node::Any | Node::Never | Node::Types(..)) | None => false,
        };
        Self {
            leaf,
            nodes: compiled.nodes,
            patterns,
            notes,
            strict: compiled.draft.strict_integers(),
            sorted: sorted_maps(),
        }
    }

    fn walk(&self) -> Walk<'_> {
        Walk {
            nodes: &self.nodes,
            patterns: &self.patterns,
            strict: self.strict,
            sorted: self.sorted,
        }
    }

    /// Whether `data` matches: the quick check, which stops at the first failure and says nothing.
    pub fn matches(&self, data: &Value) -> bool {
        // A schema that is one leaf is settled where it stands, as a field's value is.
        let walk = self.walk();
        match self.leaf.then(|| walk.leaf(0, data)).flatten() {
            Some(holds) => holds,
            None => walk.holds(0, data, 0),
        }
    }

    /// Every way `data` does not match, the first twenty, its places written from `data`
    /// (`data.legs[0] is missing "from"`); none when it matches.
    pub fn mismatches(&self, data: &Value) -> Vec<String> {
        // Valid data is the common case: the quick walk says so, and only a failure is walked
        // again for every message.
        if self.matches(data) {
            return vec![];
        }
        let mut out = vec![];
        (self.walk()).node(0, data, &At::Root, Report::said(&mut out), 0);
        out
    }

    /// `data` read as the model meant it, where the schema leaves no doubt: a number, a boolean
    /// or null written as a string, an object or an array sent as a string of JSON, each where
    /// the schema asks for the one and not for a string. Mended in place, each change said with
    /// where (`data.limit: "30" read as 30`); check it after.
    pub fn mend(&self, data: &mut Value) -> Vec<String> {
        let mut said = vec![];
        mend::mend(&self.nodes, 0, data, "data", &mut said);
        said
    }

    /// Whether a call still being written can still come to match: `written` is the text so
    /// far, and the answer is false only where no ending mends it (a field the schema refuses,
    /// a value written whole that is not what its field asks, a string no option begins with).
    /// A stream is stopped at the first false; what only the whole value tells (a union, a
    /// field still to come) is left to `matches`.
    pub fn may_match(&self, written: &str) -> bool {
        let Some(cut) = cut::cut(written) else {
            // Not yet the start of a value (a word half written), or never one.
            return !written.contains(['{', '[']);
        };
        match (cut.whole, &cut.value, cut.path) {
            // A number at the root may go on.
            (true, Value::Number(_), _) => true,
            (true, value, _) => self.matches(value),
            (false, value, Some(path)) => self.walk().may_hold(0, value, &path, cut.text, 0),
            (false, _, None) => true,
        }
    }

    /// What the schema asks that is not checked here, each named once.
    pub fn unchecked(&self) -> Vec<&'static str> {
        self.notes.clone()
    }
}

/// Whether a data object's fields come in the order of their names: so with serde_json's own
/// map, and not where a crate of the program turns its `preserve_order` feature on. A map is
/// asked once, as a schema compiles: no program has both, and a check allocates nothing.
fn sorted_maps() -> bool {
    static SORTED: OnceLock<bool> = OnceLock::new();
    *SORTED.get_or_init(|| {
        let mut map = serde_json::Map::new();
        map.insert("b".to_string(), Value::Null);
        map.insert("a".to_string(), Value::Null);
        map.keys().next().is_some_and(|first| first == "a")
    })
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
