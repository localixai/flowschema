//! Where a `$ref` can land in one document: each resource by its URI (`$id`, or `id` in draft 4,
//! resolved against the one around it), each anchor (`$anchor`, `$dynamicAnchor`, or before
//! 2019-09 an id that is only a fragment), and then a JSON pointer inside. Only subschemas are
//! walked: an `$id` inside data (`enum`, `default`) or an unknown keyword names nothing.

use std::collections::HashMap;

use serde_json::Value;

use crate::draft::Draft;
use crate::uri::{self, BASE};

/// A document's resources and anchors.
pub(crate) struct Index<'j> {
    resources: HashMap<String, &'j Value>,
    anchors: HashMap<String, &'j Value>,
    /// How many resources give each `$dynamicAnchor` (2020-12), and how many say
    /// `$recursiveAnchor: true` (2019-09): with one, a dynamic reference lands where a plain one
    /// does.
    pub(crate) dynamic: HashMap<String, usize>,
    pub(crate) recursive: usize,
}

impl<'j> Index<'j> {
    /// The index of `root` and of the other `documents` it may refer to, each by its URI.
    pub(crate) fn build(root: &'j Value, documents: &'j [(String, Value)], draft: Draft) -> Self {
        let mut index = Index {
            resources: HashMap::new(),
            anchors: HashMap::new(),
            dynamic: HashMap::new(),
            recursive: 0,
        };
        for (uri, document) in documents {
            let named = uri::split(uri).0;
            index.resources.insert(named.to_string(), document);
            let own = match document.get("$schema") {
                Some(_) => Draft::of(document),
                None => draft,
            };
            index.walk(document, named, own);
        }
        index.resources.insert(BASE.to_string(), root);
        index.walk(root, BASE, draft);
        index
    }

    fn walk(&mut self, schema: &'j Value, base: &str, draft: Draft) {
        let Value::Object(rules) = schema else {
            return;
        };
        // What names a resource or an anchor, read in one pass over the fields.
        let (mut referred, mut id, mut anchor, mut dynamic, mut recursive) =
            (false, None, None, None, false);
        for (key, value) in rules {
            match key.as_str() {
                "$ref" => referred = true,
                "$id" if draft != Draft::Four => id = value.as_str(),
                "id" if draft == Draft::Four => id = value.as_str(),
                "$anchor" if draft >= Draft::Nineteen => anchor = value.as_str(),
                "$dynamicAnchor" if draft == Draft::Twenty => dynamic = value.as_str(),
                "$recursiveAnchor" if draft == Draft::Nineteen => {
                    recursive = *value == Value::Bool(true);
                }
                _ => {}
            }
        }
        let id = id.filter(|_| !(draft.ref_alone() && referred));
        // A new base only where an id names a resource: no copy of the base for each schema.
        let mut named = None;
        if let Some(id) = id {
            let resolved = uri::resolve(base, id);
            let (resource, fragment) = uri::split(&resolved);
            match (fragment.is_empty(), draft >= Draft::Nineteen) {
                (true, _) => {
                    self.resources.insert(resource.to_string(), schema);
                    named = Some(resource.to_string());
                }
                // Before 2019-09 an id that is only a fragment is an anchor.
                (false, false) if !fragment.starts_with('/') => {
                    self.anchors.insert(resolved.clone(), schema);
                }
                (false, true | false) => {}
            }
        }
        let here = named.as_deref().unwrap_or(base);
        if let Some(name) = anchor {
            self.anchors.insert(format!("{here}#{name}"), schema);
        }
        if let Some(name) = dynamic {
            self.anchors.insert(format!("{here}#{name}"), schema);
            *self.dynamic.entry(name.to_string()).or_default() += 1;
        }
        if recursive {
            self.recursive += 1;
        }
        for (key, value) in rules {
            match (key.as_str(), value) {
                // Keywords whose value is a list of schemas.
                ("allOf" | "anyOf" | "oneOf" | "prefixItems" | "items", Value::Array(items)) => {
                    items.iter().for_each(|item| self.walk(item, here, draft));
                }
                // Keywords whose value maps names to schemas.
                (
                    "properties" | "patternProperties" | "$defs" | "definitions"
                    | "dependentSchemas" | "dependencies",
                    Value::Object(named),
                ) => named.values().for_each(|item| self.walk(item, here, draft)),
                // Keywords whose value is a schema.
                (
                    "additionalProperties"
                    | "additionalItems"
                    | "items"
                    | "contains"
                    | "propertyNames"
                    | "not"
                    | "if"
                    | "then"
                    | "else"
                    | "unevaluatedProperties"
                    | "unevaluatedItems"
                    | "contentSchema"
                    | "$recursiveRef",
                    _,
                ) => self.walk(value, here, draft),
                _ => {}
            }
        }
    }

    /// Whether the metaschema `root` names, where it is one of the documents, takes the
    /// validation vocabulary: one that lists its `$vocabulary` without it asks for none of
    /// `type`, `minimum`, `required` and the like. The documents are looked in by their URIs;
    /// only a metaschema outside json-schema.org that none of them is (one a schema holds inside
    /// it) needs the documents walked.
    pub(crate) fn validates(
        root: &'j Value,
        documents: &'j [(String, Value)],
        draft: Draft,
    ) -> bool {
        let named = root.get("$schema").and_then(Value::as_str).unwrap_or("");
        if named.is_empty() {
            return true;
        }
        let wanted = uri::split(named).0;
        let given = (documents.iter()).find(|(uri, _)| uri::split(uri).0 == wanted);
        let meta = match given {
            Some((_, document)) => Some(document),
            None if named.contains("json-schema.org") => None,
            None => Index::build(root, documents, draft)
                .resources
                .get(wanted)
                .copied(),
        };
        let listed = meta
            .and_then(|meta| meta.get("$vocabulary"))
            .and_then(Value::as_object);
        listed.is_none_or(|listed| listed.keys().any(|key| key.ends_with("/vocab/validation")))
    }

    /// What the absolute reference `target` lands on, with the base it is read with.
    pub(crate) fn find(&self, target: &str) -> Option<(&'j Value, String)> {
        let (resource, fragment) = uri::split(target);
        if !fragment.is_empty() && !fragment.starts_with('/') {
            return Some((*self.anchors.get(target)?, resource.to_string()));
        }
        let mut here = *self.resources.get(resource)?;
        for part in fragment.split('/').skip(1) {
            let part = decode(part)?;
            here = match here {
                Value::Object(map) => map.get(&part)?,
                Value::Array(list) => list.get(part.parse::<usize>().ok()?)?,
                Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => return None,
            };
        }
        Some((here, resource.to_string()))
    }
}

/// A pointer's segment as a name: `%XX` decoded, then `~1` and `~0`.
fn decode(part: &str) -> Option<String> {
    let mut bytes = Vec::with_capacity(part.len());
    let mut rest = part.bytes();
    while let Some(byte) = rest.next() {
        match byte {
            b'%' => {
                let high = char::from(rest.next()?).to_digit(16)?;
                let low = char::from(rest.next()?).to_digit(16)?;
                bytes.push(u8::try_from(high * 16 + low).ok()?);
            }
            other => bytes.push(other),
        }
    }
    let text = String::from_utf8(bytes).ok()?;
    Some(text.replace("~1", "/").replace("~0", "~"))
}

#[cfg(test)]
#[path = "index_tests.rs"]
mod tests;
