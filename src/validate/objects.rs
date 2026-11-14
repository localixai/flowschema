//! The walks of an object that read less than `fields` does: where only `properties` look at
//! a field and the data has more of them, where only `additionalProperties` does, and the quick
//! walk of most objects, whose fields come in the order of their names.

use std::cmp::Ordering;

use serde_json::{Map, Value};

use super::Walk;
use crate::node::{Extra, Objects, Property};
use crate::report::{At, Report};
use crate::text::{field, order, same_bytes};

impl Walk<'_> {
    /// Where only `properties` look at a field and the data has more fields than it names: each
    /// property looked up, the other fields never read.
    pub(super) fn named<const F: bool>(
        &self,
        objects: &Objects,
        object: &Value,
        fields: &Map<String, Value>,
        at: &At<'_>,
        r: &mut Report<'_, F>,
        bits: Option<u64>,
    ) -> bool {
        let mut met = 0;
        for property in &objects.properties {
            let (name, id) = (&*property.name, property.id);
            let Some(value) = field(fields, name) else {
                // A required field looked up and not there: no need to read on.
                if bits.is_some_and(|required| property.bit & required != 0) {
                    return r.no(String::new);
                }
                continue;
            };
            met |= property.bit;
            let ok = self.has_type(property.types, value)
                || self.fits(id, value)
                || self.said(id, value, &At::Key(at, name), r.part());
            if !r.took(ok) {
                return false;
            }
        }
        if bits.is_some_and(|bits| met & bits != bits) {
            return r.no(String::new);
        }
        !objects.closing || self.closing(objects, object, fields, at, r)
    }

    /// The quick walk of an object held to `properties`, `required` and `additionalProperties`
    /// alone, whose fields come in the order of their names (`Walk::sorted`): each field is the
    /// next property or none of them, told in one pass with nothing looked up and nothing kept
    /// of the last one, and a required property gone past is missing at once. It says nothing,
    /// so it names no place.
    pub(super) fn in_order(&self, objects: &Objects, fields: &Map<String, Value>) -> bool {
        let properties = &*objects.properties;
        // A required field `properties` does not name is looked up; the rest are met in order.
        let required = match objects.required_bits {
            Some(bits) => bits,
            None => {
                let mut wanted = (objects.required.iter()).map(|it| objects.required_name(it));
                if !wanted.all(|name| field(fields, name).is_some()) {
                    return false;
                }
                0
            }
        };
        // Nothing looks at a field: none is read.
        if properties.is_empty() && matches!(objects.extra, Extra::Absent | Extra::Allowed) {
            return true;
        }
        let (mut next, mut met) = (0, 0);
        for (name, value) in fields {
            let wanted = name.as_bytes();
            let mut found = None;
            while let Some(property) = properties.get(next) {
                let known = property.name.as_bytes();
                // Most fields are the next property: told so with no walk over their bytes.
                if same_bytes(known, wanted) {
                    (found, next) = (Some(property), next + 1);
                    break;
                }
                // Two names that are not the same mostly differ at their first byte: told
                // there, with the rest unread.
                let stands = match known.first().cmp(&wanted.first()) {
                    Ordering::Equal => order(known, wanted),
                    apart @ (Ordering::Less | Ordering::Greater) => apart,
                };
                match stands {
                    Ordering::Less if property.bit & required != 0 => return false,
                    Ordering::Less => next += 1,
                    Ordering::Equal => {
                        (found, next) = (Some(property), next + 1);
                        break;
                    }
                    Ordering::Greater => break,
                }
            }
            let ok = match found {
                Some(property) => {
                    met |= property.bit;
                    let walked = || self.holds(property.id, value, 0);
                    // A node of types alone is settled where the field stands; any other goes
                    // by the way chosen for it, a leaf's being its own short function.
                    match property.types {
                        0 => walked(),
                        types => self.has_type(types, value),
                    }
                }
                None => match objects.extra {
                    Extra::Absent | Extra::Allowed => true,
                    Extra::Refused => false,
                    Extra::Checked(id) => self.holds(id, value, 0),
                },
            };
            if !ok {
                return false;
            }
        }
        met & required == required
    }
}

impl Walk<'_> {
    /// The quick walk of one or two fields against a few properties: each field is looked for
    /// among them, a comparison of lengths telling most apart, and no property is gone past.
    pub(super) fn few(&self, objects: &Objects, fields: &Map<String, Value>) -> bool {
        let mut met = 0;
        for (name, value) in fields {
            let named =
                |property: &&Property| same_bytes(property.name.as_bytes(), name.as_bytes());
            let ok = match (objects.properties.iter().find(named), &objects.extra) {
                (Some(property), _) => {
                    met |= property.bit;
                    match property.types {
                        0 => self.holds(property.id, value, 0),
                        types => self.has_type(types, value),
                    }
                }
                (None, Extra::Absent | Extra::Allowed) => true,
                (None, Extra::Refused) => false,
                (None, Extra::Checked(id)) => self.holds(*id, value, 0),
            };
            if !ok {
                return false;
            }
        }
        match objects.required_bits {
            Some(required) => met & required == required,
            None => (objects.required.iter())
                .all(|it| field(fields, objects.required_name(it)).is_some()),
        }
    }
}

impl Walk<'_> {
    /// The quick walk of an object that names far more properties than the value has fields:
    /// each field is found by hash (`Objects::lookup`), with nothing said and no property
    /// gone past. What `in_order` answers, for a value it would walk every property for.
    pub(super) fn by_hash(
        &self,
        objects: &Objects,
        lookup: &crate::hash::Names,
        fields: &Map<String, Value>,
    ) -> bool {
        let properties = &*objects.properties;
        let required = objects.required_bits.unwrap_or(0);
        let mut wanted = (objects.required.iter()).map(|it| objects.required_name(it));
        if objects.required_bits.is_none() && !wanted.all(|name| field(fields, name).is_some()) {
            return false;
        }
        let mut met = 0;
        for (name, value) in fields {
            let found = (lookup.find(properties, name)).and_then(|place| properties.get(place));
            let ok = match (found, &objects.extra) {
                (Some(property), _) => {
                    met |= property.bit;
                    match property.types {
                        0 => self.holds(property.id, value, 0),
                        types => self.has_type(types, value),
                    }
                }
                (None, Extra::Absent | Extra::Allowed) => true,
                (None, Extra::Refused) => false,
                (None, Extra::Checked(id)) => self.holds(*id, value, 0),
            };
            if !ok {
                return false;
            }
        }
        met & required == required
    }
}

/// The bits of the properties among `fields`: what a union of required names beside an object
/// is told by (`Objects::branches`), with nothing looked up. Both are in the order of their
/// names and are met in one pass; where the properties are many (`Objects::lookup`), only
/// those a branch names (`Objects::named`) are looked for.
pub(super) fn seen(objects: &Objects, fields: &Map<String, Value>) -> u64 {
    if objects.lookup.is_some() {
        return (objects
            .named
            .iter()
            .filter_map(|place| objects.properties.get(*place)))
        .filter(|property| field(fields, &property.name).is_some())
        .fold(0, |bits, property| bits | property.bit);
    }
    let (mut next, mut bits) = (0, 0);
    for name in fields.keys() {
        while let Some(property) = objects.properties.get(next) {
            match order(property.name.as_bytes(), name.as_bytes()) {
                Ordering::Less => next += 1,
                Ordering::Equal => {
                    (bits, next) = (bits | property.bit, next + 1);
                    break;
                }
                Ordering::Greater => break,
            }
        }
    }
    bits
}

#[cfg(test)]
#[path = "objects_tests.rs"]
mod tests;
