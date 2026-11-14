//! A schema object's keywords, read in one pass over its fields: each kept where the draft knows
//! it (the arms' guards say which draft knows which keyword, and which names a resource), so the
//! compiler asks a field of `Keywords` for a keyword, not the map, and compares no keyword's name
//! twice. A keyword a draft does not know is a word of the schema's own, and asks nothing.

use serde_json::{Map, Value};

use crate::draft::Draft;

/// What one schema object says, keyword by keyword: `None` where it says nothing, or the draft
/// does not know the word.
#[derive(Default)]
pub(super) struct Keywords<'j> {
    /// How many keywords the object says, of those below.
    said: u8,
    pub(super) reference: Option<&'j Value>,
    /// `$id`, or `id` in draft 4.
    pub(super) id: Option<&'j Value>,
    /// `$dynamicRef` (2020-12) or `$recursiveRef` (2019-09).
    pub(super) dynamic: Option<&'j Value>,
    pub(super) kind: Option<&'j Value>,
    pub(super) options: Option<&'j Value>,
    pub(super) constant: Option<&'j Value>,
    pub(super) format: Option<&'j Value>,
    pub(super) pattern: Option<&'j Value>,
    pub(super) all_of: Option<&'j Value>,
    pub(super) any_of: Option<&'j Value>,
    pub(super) one_of: Option<&'j Value>,
    pub(super) not: Option<&'j Value>,
    pub(super) when: Option<&'j Value>,
    pub(super) then: Option<&'j Value>,
    pub(super) otherwise: Option<&'j Value>,
    pub(super) multiple_of: Option<&'j Value>,
    pub(super) minimum: Option<&'j Value>,
    pub(super) exclusive_minimum: Option<&'j Value>,
    pub(super) maximum: Option<&'j Value>,
    pub(super) exclusive_maximum: Option<&'j Value>,
    pub(super) min_length: Option<&'j Value>,
    pub(super) max_length: Option<&'j Value>,
    pub(super) unevaluated_properties: Option<&'j Value>,
    pub(super) unevaluated_items: Option<&'j Value>,
    pub(super) items: Option<&'j Value>,
    pub(super) prefix_items: Option<&'j Value>,
    /// Kept in every draft: 2020-12 reads it too beside a list of `items`, as an LLM's schema
    /// often writes it (`compile/containers.rs`).
    pub(super) additional_items: Option<&'j Value>,
    pub(super) min_items: Option<&'j Value>,
    pub(super) max_items: Option<&'j Value>,
    pub(super) unique_items: Option<&'j Value>,
    pub(super) contains: Option<&'j Value>,
    pub(super) min_contains: Option<&'j Value>,
    pub(super) max_contains: Option<&'j Value>,
    pub(super) required: Option<&'j Value>,
    pub(super) properties: Option<&'j Value>,
    pub(super) pattern_properties: Option<&'j Value>,
    pub(super) additional_properties: Option<&'j Value>,
    pub(super) property_names: Option<&'j Value>,
    pub(super) min_properties: Option<&'j Value>,
    pub(super) max_properties: Option<&'j Value>,
    pub(super) dependent_required: Option<&'j Value>,
    pub(super) dependent_schemas: Option<&'j Value>,
    pub(super) dependencies: Option<&'j Value>,
}

impl<'j> Keywords<'j> {
    /// The keywords of `rules`, as `draft` reads them.
    pub(super) fn of(rules: &'j Map<String, Value>, draft: Draft) -> Self {
        use Draft::{Four, Nineteen, Seven, Six, Twenty};
        let mut k = Keywords::default();
        for (key, value) in rules {
            let slot = match key.as_str() {
                "$ref" => &mut k.reference,
                "$id" if draft != Four => &mut k.id,
                "id" if draft == Four => &mut k.id,
                "$dynamicRef" if draft == Twenty => &mut k.dynamic,
                "$recursiveRef" if draft == Nineteen => &mut k.dynamic,
                "type" => &mut k.kind,
                "enum" => &mut k.options,
                "const" if draft >= Six => &mut k.constant,
                "format" => &mut k.format,
                "pattern" => &mut k.pattern,
                "allOf" => &mut k.all_of,
                "anyOf" => &mut k.any_of,
                "oneOf" => &mut k.one_of,
                "not" => &mut k.not,
                "if" if draft >= Seven => &mut k.when,
                "then" if draft >= Seven => &mut k.then,
                "else" if draft >= Seven => &mut k.otherwise,
                "multipleOf" => &mut k.multiple_of,
                "minimum" => &mut k.minimum,
                "exclusiveMinimum" => &mut k.exclusive_minimum,
                "maximum" => &mut k.maximum,
                "exclusiveMaximum" => &mut k.exclusive_maximum,
                "minLength" => &mut k.min_length,
                "maxLength" => &mut k.max_length,
                "unevaluatedProperties" if draft >= Nineteen => &mut k.unevaluated_properties,
                "unevaluatedItems" if draft >= Nineteen => &mut k.unevaluated_items,
                "items" => &mut k.items,
                "prefixItems" if draft == Twenty => &mut k.prefix_items,
                "additionalItems" => &mut k.additional_items,
                "minItems" => &mut k.min_items,
                "maxItems" => &mut k.max_items,
                "uniqueItems" => &mut k.unique_items,
                "contains" if draft >= Six => &mut k.contains,
                "minContains" if draft >= Nineteen => &mut k.min_contains,
                "maxContains" if draft >= Nineteen => &mut k.max_contains,
                "required" => &mut k.required,
                "properties" => &mut k.properties,
                "patternProperties" => &mut k.pattern_properties,
                "additionalProperties" => &mut k.additional_properties,
                "propertyNames" if draft >= Six => &mut k.property_names,
                "minProperties" => &mut k.min_properties,
                "maxProperties" => &mut k.max_properties,
                "dependentRequired" if draft >= Nineteen => &mut k.dependent_required,
                "dependentSchemas" if draft >= Nineteen => &mut k.dependent_schemas,
                "dependencies" if draft <= Seven => &mut k.dependencies,
                _ => continue,
            };
            *slot = Some(value);
            k.said = k.said.saturating_add(1);
        }
        k
    }

    /// Whether the object says no keyword at all: `{}`, or annotations alone. It allows anything.
    pub(super) fn silent(&self) -> bool {
        self.said == 0
    }

    /// `type`, where it is the one keyword the object says.
    pub(super) fn only_type(&self) -> Option<&'j Value> {
        self.kind.filter(|_| self.said == 1)
    }

    /// Whether any keyword of arrays is said (`additionalItems` only where the draft knows it).
    pub(super) fn of_arrays(&self, draft: Draft) -> bool {
        self.items.is_some()
            || self.prefix_items.is_some()
            || (self.additional_items.is_some() && draft <= Draft::Nineteen)
            || self.min_items.is_some()
            || self.max_items.is_some()
            || self.unique_items.is_some()
            || self.contains.is_some()
            || self.min_contains.is_some()
            || self.max_contains.is_some()
    }

    /// Whether any keyword of objects is said.
    pub(super) fn of_objects(&self) -> bool {
        self.required.is_some()
            || self.properties.is_some()
            || self.pattern_properties.is_some()
            || self.additional_properties.is_some()
            || self.property_names.is_some()
            || self.min_properties.is_some()
            || self.max_properties.is_some()
            || self.dependent_required.is_some()
            || self.dependent_schemas.is_some()
            || self.dependencies.is_some()
            || self.unevaluated_properties.is_some()
    }
}

#[cfg(test)]
#[path = "read_tests.rs"]
mod tests;
