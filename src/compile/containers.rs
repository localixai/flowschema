//! The keywords of arrays and objects compiled, as the draft spells them: `items` a list with
//! `additionalItems` before 2020-12 (and taken so in 2020-12 too, where an LLM's schema often
//! writes it), `prefixItems` in 2020-12; `dependencies` before 2019-09 split by what it holds.

use serde_json::Value;

use super::Compiler;
use super::keywords::count;
use super::read::Keywords;
use crate::node::{Arrays, Extra, Id, Node, Objects, Property, Required};

impl<'j> Compiler<'j> {
    pub(super) fn arrays(&mut self, k: &Keywords<'j>, base: &str) -> Option<Box<Arrays>> {
        if !k.of_arrays(self.draft) {
            return None;
        }
        let (prefix, rest) = match (k.prefix_items, k.items) {
            (Some(prefix @ Value::Array(_)), rest) => (Some(prefix), rest),
            (None | Some(_), Some(prefix @ Value::Array(_))) => (Some(prefix), k.additional_items),
            (None | Some(_), rest) => (None, rest),
        };
        let contains = k.contains.map(|schema| {
            let (least, most) = (count(k.min_contains), count(k.max_contains));
            (self.node(schema, base), least.unwrap_or(1), most)
        });
        Some(Box::new(Arrays {
            prefix: self.many(prefix, base),
            rest: rest.map(|schema| self.node(schema, base)),
            min_items: count(k.min_items),
            max_items: count(k.max_items),
            unique: k.unique_items == Some(&Value::Bool(true)),
            contains,
        }))
    }

    /// `properties` in the order of their names, each with its bit in `required`, and
    /// `patternProperties` as each pattern's place and node.
    fn named(&mut self, k: &Keywords<'j>, base: &str) -> (Vec<Property>, Vec<(usize, Id)>) {
        let wanted = listed(k.required);
        let bit = |name: &str| {
            let place = wanted.clone().position(|field| field == name);
            place
                .filter(|place| *place < 64)
                .map_or(0, |place| 1 << place)
        };
        let named = k.properties.and_then(Value::as_object);
        let mut properties = Vec::with_capacity(named.map_or(0, serde_json::Map::len));
        for (name, schema) in named.into_iter().flatten() {
            // What its node asks is told once every node is built (`flatten::settle`).
            properties.push(Property {
                name: name.as_str().into(),
                id: self.node(schema, base),
                bit: bit(name),
                types: 0,
            });
        }
        let mut patterns = vec![];
        for (source, schema) in k
            .pattern_properties
            .and_then(Value::as_object)
            .into_iter()
            .flatten()
        {
            // A pattern that is one whole name (`^name$`) is a property of that name, found as
            // one and not tried against every field; unless a property has the name already.
            match crate::pattern::exact(source)
                .filter(|name| !properties.iter().any(|p| *p.name == **name))
            {
                Some(name) => properties.push(Property {
                    id: self.node(schema, base),
                    bit: bit(&name),
                    types: 0,
                    name: name.into(),
                }),
                None => {
                    let place = self.pattern(source);
                    patterns.push((place, self.node(schema, base)));
                }
            }
        }
        // In order, as a data object's fields come: a walk meets both in one pass.
        properties.sort_unstable_by(|a, b| a.name.cmp(&b.name));
        (properties, patterns)
    }

    pub(super) fn objects(&mut self, k: &Keywords<'j>, base: &str) -> Option<Box<Objects>> {
        if !k.of_objects() {
            return None;
        }
        let wanted = listed(k.required);
        let (properties, patterns) = self.named(k, base);
        let required: Box<[Required]> = (wanted)
            .map(
                |field| match properties.binary_search_by(|p| (*p.name).cmp(field)) {
                    Ok(at) => Required::Property(at),
                    Err(_) => Required::Other(field.into()),
                },
            )
            .collect();
        let bits = properties.iter().fold(0, |all, p| all | p.bit);
        let all_named = required.len() <= 64 && bits.count_ones() as usize == required.len();
        let extra = match k.additional_properties {
            None => Extra::Absent,
            Some(Value::Bool(true)) => Extra::Allowed,
            Some(Value::Bool(false)) => Extra::Refused,
            Some(schema) => match self.node(schema, base) {
                // A schema that allows anything is `true`: no field is held to it.
                id if matches!(self.nodes.get(id), Some(Node::Any)) => Extra::Allowed,
                id => Extra::Checked(id),
            },
        };
        let (mut dependent, mut dependent_schemas) = (vec![], vec![]);
        let deps = [k.dependent_required, k.dependent_schemas, k.dependencies];
        for (name, need) in deps
            .into_iter()
            .flatten()
            .filter_map(Value::as_object)
            .flatten()
        {
            match need {
                Value::Array(_) => {
                    let needs = listed(Some(need)).map(Into::into).collect();
                    dependent.push((name.as_str().into(), needs));
                }
                Value::Object(_) | Value::Bool(_) => {
                    dependent_schemas.push((name.as_str().into(), self.node(need, base)));
                }
                Value::Null | Value::Number(_) | Value::String(_) => {}
            }
        }
        let min_fields = count(k.min_properties);
        let max_fields = count(k.max_properties);
        let closing = min_fields.is_some()
            || max_fields.is_some()
            || !dependent.is_empty()
            || !dependent_schemas.is_empty();
        let names = k.property_names.map(|schema| self.node(schema, base));
        let ordered = names.is_none() && patterns.is_empty();
        Some(Box::new(Objects {
            closing,
            simple: ordered && !closing,
            ordered,
            lookup: crate::hash::Names::of(&properties),
            required,
            properties: properties.into_boxed_slice(),
            required_bits: all_named.then_some(bits),
            patterns: patterns.into_boxed_slice(),
            extra,
            extra_types: 0,
            branches: Box::default(),
            named: Box::default(),
            names,
            min_fields,
            max_fields,
            dependent: dependent.into_boxed_slice(),
            dependent_schemas: dependent_schemas.into_boxed_slice(),
        }))
    }
}

/// The strings a list holds (`required`, an entry of `dependentRequired`), in order.
fn listed(list: Option<&Value>) -> impl Iterator<Item = &str> + Clone {
    let list = list
        .and_then(Value::as_array)
        .map_or(&[][..], Vec::as_slice);
    list.iter().filter_map(Value::as_str)
}
