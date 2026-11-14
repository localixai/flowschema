//! The compiled schema's nodes: what each schema object asks, in the shape a walk reads with no
//! keyword looked up (`compile.rs` builds them, `validate.rs` walks them).

use std::cmp::Ordering;

use serde_json::{Number, Value};

use crate::hash::Table;
use crate::number;

/// A node's place in the compiled schema.
pub(crate) type Id = usize;

/// Field names, as `required` and `dependentRequired` list them.
pub(crate) type Names = Box<[Box<str>]>;

/// A schema: `true` (or anything that is not an object), `false`, an object that asks only for
/// types, or an object's keywords.
pub(crate) enum Node {
    Any,
    Never,
    /// An object whose one keyword is `type` (annotations aside): its types' bits, and their
    /// names as written. Most leaves of an agent's schema are one (`{"type": "string",
    /// "description": "…"}`): no rules are made for it, and the walk checks it where it stands.
    Types(u8, TypeList),
    Rules(Box<Rules>),
}

/// What one schema object asks. Each keyword it lacks is empty here, and costs nothing.
#[derive(Default)]
pub(crate) struct Rules {
    /// Which groups of keywords this node has (`HAS_*`): a walk skips the rest unread.
    pub(crate) has: u8,
    pub(crate) reference: Option<Id>,
    /// `type` as `kind_of`'s bits; `None` with no `type`.
    pub(crate) types: Option<u8>,
    /// `type`'s names, as a message says them.
    pub(crate) type_names: TypeNames,
    /// `enum` as written: its options, and the whole for a message; with many options, the table
    /// a value is looked up in.
    pub(crate) options: Option<Value>,
    pub(crate) table: Option<Table>,
    /// The kinds among `enum`'s options: a value of no such kind is none of them, unread.
    pub(crate) option_kinds: u8,
    pub(crate) constant: Option<Value>,
    pub(crate) all_of: Box<[Id]>,
    pub(crate) any_of: Box<[Id]>,
    pub(crate) one_of: Box<[Id]>,
    pub(crate) not: Option<Id>,
    /// `if`, with `then` and `else`.
    pub(crate) condition: Option<(Id, Option<Id>, Option<Id>)>,
    /// The number bounds in the order they are said, and `multipleOf`.
    pub(crate) limits: Box<[(Number, Sign)]>,
    pub(crate) multiple: Option<number::Divisor>,
    pub(crate) min_length: Option<u64>,
    pub(crate) max_length: Option<u64>,
    /// `pattern`: its place among the schema's patterns.
    pub(crate) pattern: Option<usize>,
    /// `format`, where it is one this check knows: its name, for a message, and its check.
    pub(crate) format: Option<(Box<str>, crate::format::Check)>,
    pub(crate) array: Option<Box<Arrays>>,
    pub(crate) object: Option<Box<Objects>>,
    /// `unevaluatedProperties` and `unevaluatedItems`: what nothing else here looked at is held to.
    pub(crate) unevaluated_fields: Option<Id>,
    pub(crate) unevaluated_items: Option<Id>,
    /// The tags of `anyOf` and `oneOf`, where a field settles their branches.
    pub(crate) any_tag: Option<Box<crate::compile::tags::Tag>>,
    pub(crate) one_tag: Option<Box<crate::compile::tags::Tag>>,
    /// The types, where they are all the schema asks (`{"type": "string"}`, most leaves): the
    /// walk checks such a value where it stands, with no call.
    pub(crate) only_types: Option<u8>,
    /// What the node asks, where it is a leaf the walk settles where it stands (`Leaf`).
    pub(crate) leaf: Leaf,
    /// The lane the quick walk takes through the node (`Lane`).
    pub(crate) lane: Lane,
    /// The strings a union of names holds, where the node is one (`compile/unions.rs`).
    pub(crate) listed: Option<Box<crate::hash::Listed>>,
    /// The node this one only refers to, where a `$ref` is all it asks.
    pub(crate) alias: Option<Id>,
    /// An array that asks only its type, its count and its items' types: the quick walk reads
    /// it in one loop (`compile/kinds.rs`).
    pub(crate) plain_array: Option<PlainArray>,
    /// Whether what the node applies to every value it allows takes every field, or every
    /// item: `unevaluated*` then finds nothing left (`compile/evaluated.rs`).
    pub(crate) every_field_seen: bool,
    pub(crate) every_item_seen: bool,
}

/// The names of JSON Schema's types, in the order of their codes in `TypeList`.
const TYPE_NAMES: [&str; 7] = [
    "null", "boolean", "integer", "number", "string", "array", "object",
];

/// Names of types as `type` writes them, packed in a word: three bits a name, in the order
/// written, so a message says them as the schema did and keeping them allocates nothing.
#[derive(Clone, Copy, Default)]
pub(crate) struct TypeList {
    codes: u32,
    len: u8,
}

impl TypeList {
    /// `names` packed; none where one is not a type's name, or past ten names.
    pub(crate) fn of<'n>(names: impl Iterator<Item = &'n str>) -> Option<Self> {
        let mut list = TypeList::default();
        for name in names {
            let code = TYPE_NAMES.iter().position(|known| *known == name)?;
            let code = u32::try_from(code).ok().filter(|_| list.len < 10)?;
            list.codes |= code << (3 * u32::from(list.len));
            list.len += 1;
        }
        Some(list)
    }

    /// The names, in the order written.
    pub(crate) fn names(self) -> Vec<&'static str> {
        (0..u32::from(self.len))
            .filter_map(|at| {
                let code = usize::try_from((self.codes >> (3 * at)) & 0b111).ok()?;
                TYPE_NAMES.get(code).copied()
            })
            .collect()
    }
}

/// `type`'s names, for a message: packed where they all name types, else as written.
pub(crate) enum TypeNames {
    Packed(TypeList),
    Written(Names),
}

impl Default for TypeNames {
    fn default() -> Self {
        TypeNames::Packed(TypeList::default())
    }
}

impl TypeNames {
    /// `names` as `type` lists them.
    pub(crate) fn of<'n>(names: impl Iterator<Item = &'n str> + Clone) -> Self {
        match TypeList::of(names.clone()) {
            Some(list) => TypeNames::Packed(list),
            None => TypeNames::Written(names.map(Into::into).collect()),
        }
    }

    /// The names, in the order written.
    pub(crate) fn names(&self) -> Vec<&str> {
        match self {
            TypeNames::Packed(list) => list.names(),
            TypeNames::Written(names) => names.iter().map(|name| &**name).collect(),
        }
    }
}

/// A leaf the walk settles where it stands (`Walk::leaf_holds`), found as the node is built
/// (`compile/leaves.rs`). It is sure only where it says a value holds: one it cannot settle so
/// (a fraction, a long string to count by characters) is walked, and said as the rules say.
/// It holds nothing of its own (the rules have it), so the rules grow by a byte.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub(crate) enum Leaf {
    #[default]
    Walked,
    /// An `enum` of strings alone, with a `type` that allows strings or none.
    Strings,
    /// A `const` alone that is a string, a boolean or null, with a `type` that allows it or
    /// none: the field a union of tool calls is told apart by.
    Const,
    /// Integers within bounds that are integers themselves (`limits`), with a `type` that
    /// allows integers or none.
    Range,
    /// Strings held to a string's keywords alone, with `type` string or none: the length where
    /// the count of bytes says it holds (`min_length`, `max_length`: at least four bytes a
    /// character for the least, at most one for the most), then `pattern` and `format`.
    Text,
}

/// The lane the quick walk takes through a node (`Walk::holds`): most nodes that are no leaf
/// are an object or an array and no more, and are walked with nothing else looked at.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub(crate) enum Lane {
    /// Every keyword the node may have is read (`Walk::node`).
    #[default]
    Walk,
    /// `type`, and an object held to `properties`, `required` and `additionalProperties`.
    Object,
    /// `type`, and an array held to its count and to one schema for every item.
    Array,
    /// An `anyOf` or a `oneOf` and nothing else.
    Union,
}

/// What an array that asks only its type, its count and its items' types asks.
pub(crate) struct PlainArray {
    /// The node's own types (every one where it says none).
    pub(crate) types: u8,
    pub(crate) min: u64,
    pub(crate) max: u64,
    pub(crate) items: Items,
}

/// What each item of a plain array must be.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Items {
    /// Of these types (every one where `items` says nothing).
    Types(u8),
    /// A plain array itself, as the node's plan says.
    Plain(Id),
}

impl Rules {
    /// These rules with every keyword of the validation vocabulary taken out, for a schema whose
    /// metaschema leaves that vocabulary out: they then ask only what the applicators do.
    pub(crate) fn without_validation(&mut self) {
        (self.types, self.type_names) = (None, TypeNames::default());
        (self.options, self.table, self.option_kinds, self.constant) = (None, None, 0, None);
        (self.limits, self.multiple) = (Box::default(), None);
        (self.min_length, self.max_length, self.pattern) = (None, None, None);
        if let Some(arrays) = &mut self.array {
            (arrays.min_items, arrays.max_items, arrays.unique) = (None, None, false);
            if let Some(contains) = &mut arrays.contains {
                (contains.1, contains.2) = (1, None);
            }
        }
        if let Some(objects) = &mut self.object {
            (objects.required, objects.required_bits) = (Box::default(), None);
            (objects.min_fields, objects.max_fields) = (None, None);
            objects.dependent = Box::default();
            objects.closing = !objects.dependent_schemas.is_empty();
            objects.simple =
                objects.names.is_none() && objects.patterns.is_empty() && !objects.closing;
        }
        self.has = self.groups();
    }
}

/// The groups of keywords a node may have.
pub(crate) const HAS_KIND: u8 = 1;
pub(crate) const HAS_COMBINED: u8 = 1 << 1;
pub(crate) const HAS_NUMBER: u8 = 1 << 2;
pub(crate) const HAS_STRING: u8 = 1 << 3;
pub(crate) const HAS_ARRAY: u8 = 1 << 4;
pub(crate) const HAS_OBJECT: u8 = 1 << 5;

impl Rules {
    /// The groups of keywords these rules have.
    pub(crate) fn groups(&self) -> u8 {
        let kind = self.types.is_some() || self.options.is_some() || self.constant.is_some();
        let combined = !self.all_of.is_empty()
            || !self.any_of.is_empty()
            || !self.one_of.is_empty()
            || self.not.is_some()
            || self.condition.is_some();
        let number = !self.limits.is_empty() || self.multiple.is_some();
        let string = self.min_length.is_some()
            || self.max_length.is_some()
            || self.pattern.is_some()
            || self.format.is_some();
        let array = self.array.is_some() || self.unevaluated_items.is_some();
        let object = self.object.is_some() || self.unevaluated_fields.is_some();
        [
            (kind, HAS_KIND),
            (combined, HAS_COMBINED),
            (number, HAS_NUMBER),
            (string, HAS_STRING),
            (array, HAS_ARRAY),
            (object, HAS_OBJECT),
        ]
        .into_iter()
        .filter(|(present, _)| *present)
        .fold(0, |all, (_, bit)| all | bit)
    }
}

/// How a number must stand to a bound.
#[derive(Clone, Copy)]
pub(crate) enum Sign {
    AtLeast,
    Above,
    AtMost,
    Below,
}

impl Sign {
    /// Whether `n` stands so to `limit`, compared exactly.
    pub(crate) fn holds(self, n: &Number, limit: &Number) -> bool {
        let Some(order) = number::compare(n, limit) else {
            return false;
        };
        match self {
            Sign::AtLeast => order != Ordering::Less,
            Sign::Above => order == Ordering::Greater,
            Sign::AtMost => order != Ordering::Greater,
            Sign::Below => order == Ordering::Less,
        }
    }

    /// Whether the integer `n` stands so to the integer `limit`.
    #[inline]
    pub(crate) fn holds_whole(self, n: i64, limit: i64) -> bool {
        match self {
            Sign::AtLeast => n >= limit,
            Sign::Above => n > limit,
            Sign::AtMost => n <= limit,
            Sign::Below => n < limit,
        }
    }

    pub(crate) fn said(self) -> &'static str {
        match self {
            Sign::AtLeast => ">=",
            Sign::Above => ">",
            Sign::AtMost => "<=",
            Sign::Below => "<",
        }
    }
}

/// The keywords of an array.
pub(crate) struct Arrays {
    pub(crate) prefix: Box<[Id]>,
    pub(crate) rest: Option<Id>,
    /// Whether `rest` is a leaf the walk settles where it stands (`Leaf`), told here as a
    /// property is told it (`compile/flatten.rs`).
    pub(crate) rest_leaf: bool,
    pub(crate) min_items: Option<u64>,
    pub(crate) max_items: Option<u64>,
    pub(crate) unique: bool,
    /// `contains`, with `minContains` (1 when not said) and `maxContains`.
    pub(crate) contains: Option<(Id, u64, Option<u64>)>,
}

/// A field `properties` names: its name, its node, and its bit in `required` (none when it is
/// not required, or past the 64th).
pub(crate) struct Property {
    pub(crate) name: Box<str>,
    pub(crate) id: Id,
    pub(crate) bit: u64,
    /// Whether its node is a leaf the walk settles where it stands (`Leaf`): told here, where
    /// the walk already reads, so a field that is none costs no call to ask.
    pub(crate) leaf: bool,
    /// The types its node asks, where it is a node of types alone (every type for one that
    /// allows anything); none where it asks more. Most fields are settled by it, with no read
    /// of their node.
    pub(crate) types: u8,
}

/// A field `required` names: most are named by `properties` too, and are their place there, so
/// their names are not kept twice.
pub(crate) enum Required {
    Property(usize),
    Other(Box<str>),
}

/// The keywords of an object.
pub(crate) struct Objects {
    pub(crate) required: Box<[Required]>,
    /// `properties` in the order of their names, each with its bit in `required`: a check that
    /// meets them all in the data looks none of `required` up.
    pub(crate) properties: Box<[Property]>,
    /// `properties` by hash, where there are many.
    pub(crate) lookup: Option<crate::hash::Names>,
    /// The bits of every required field `properties` names, when they are all of `required`.
    pub(crate) required_bits: Option<u64>,
    /// `patternProperties`: each pattern's place among the schema's patterns, and its node.
    pub(crate) patterns: Box<[(usize, Id)]>,
    pub(crate) extra: Extra,
    /// `propertyNames`.
    pub(crate) names: Option<Id>,
    pub(crate) min_fields: Option<u64>,
    pub(crate) max_fields: Option<u64>,
    pub(crate) dependent: Box<[(Box<str>, Names)]>,
    /// `dependentSchemas` (or draft 7's `dependencies` given a schema).
    pub(crate) dependent_schemas: Box<[(Box<str>, Id)]>,
    /// Whether any of the counts or the dependencies is said: an object is held to them after
    /// its fields.
    pub(crate) closing: bool,
    /// Whether it is held to `properties`, `required` and `additionalProperties` alone (no
    /// `propertyNames`, no `patternProperties`, nothing `closing`): most objects, which the
    /// quick walk reads in one pass (`Walk::in_order`).
    pub(crate) simple: bool,
}

impl Objects {
    /// The name of a required field.
    pub(crate) fn required_name<'o>(&'o self, field: &'o Required) -> &'o str {
        match field {
            Required::Property(at) => self.properties.get(*at).map_or("", |p| &p.name),
            Required::Other(name) => name,
        }
    }
}

/// What a field neither `properties` nor `patternProperties` names is held to.
pub(crate) enum Extra {
    /// No `additionalProperties`: anything, and it looks at none of them.
    Absent,
    /// `additionalProperties: true`: anything, and it looks at all of them.
    Allowed,
    Refused,
    Checked(Id),
}

impl Extra {
    pub(crate) fn applies(&self) -> bool {
        match self {
            Extra::Absent => false,
            Extra::Allowed | Extra::Refused | Extra::Checked(_) => true,
        }
    }
}

/// The types of JSON Schema as bits of a value's kind.
pub(crate) const NULL: u8 = 1;
pub(crate) const BOOLEAN: u8 = 1 << 1;
pub(crate) const INTEGER: u8 = 1 << 2;
pub(crate) const NUMBER: u8 = 1 << 3;
pub(crate) const STRING: u8 = 1 << 4;
pub(crate) const ARRAY: u8 = 1 << 5;
pub(crate) const OBJECT: u8 = 1 << 6;
/// Every type's bit: what a node that allows anything allows.
pub(crate) const ALL: u8 = NULL | BOOLEAN | INTEGER | NUMBER | STRING | ARRAY | OBJECT;

/// The bits of `value`'s kinds: a number with no fraction is an integer too (only one written
/// so, where `strict`: draft 4).
pub(crate) fn kind_of(value: &Value, strict: bool) -> u8 {
    match value {
        Value::Null => NULL,
        Value::Bool(_) => BOOLEAN,
        Value::Number(_) if strict && number::is_written_integer(value) => NUMBER | INTEGER,
        Value::Number(_) if !strict && number::is_integer(value) => NUMBER | INTEGER,
        Value::Number(_) => NUMBER,
        Value::String(_) => STRING,
        Value::Array(_) => ARRAY,
        Value::Object(_) => OBJECT,
    }
}
