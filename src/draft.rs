//! Which JSON Schema a document is written to (its `$schema`; 2020-12 when it names none), and
//! what that changes: whether a `$ref` hides the keywords beside it (up to draft 7), whether `1.0`
//! is an integer (not in draft 4), and how the bounds are written. The keywords each draft knows,
//! and the one that names a resource (`id` before draft 6), are read in `compile/read.rs`; one
//! it does not know is a word of the schema's own, and asks nothing.

use serde_json::Value;

/// The drafts, in the order they came.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) enum Draft {
    Four,
    Six,
    Seven,
    Nineteen,
    Twenty,
}

impl Draft {
    /// The draft `root`'s `$schema` names; 2020-12 for none, or one it does not know.
    pub(crate) fn of(root: &Value) -> Self {
        let named = root.get("$schema").and_then(Value::as_str).unwrap_or("");
        [
            ("draft-03", Draft::Four),
            ("draft-04", Draft::Four),
            ("draft-06", Draft::Six),
            ("draft-07", Draft::Seven),
            ("2019-09", Draft::Nineteen),
        ]
        .into_iter()
        .find(|(name, _)| named.contains(name))
        .map_or(Draft::Twenty, |(_, draft)| draft)
    }

    /// Whether a `$ref` hides every keyword beside it.
    pub(crate) fn ref_alone(self) -> bool {
        self <= Draft::Seven
    }

    /// Whether only a number written without a fraction is an integer (`1.0` is not).
    pub(crate) fn strict_integers(self) -> bool {
        self == Draft::Four
    }

    /// Whether `exclusiveMinimum` and `exclusiveMaximum` are flags on `minimum` and `maximum`.
    pub(crate) fn flag_bounds(self) -> bool {
        self == Draft::Four
    }
}

#[cfg(test)]
#[path = "draft_tests.rs"]
mod tests;
