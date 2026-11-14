//! The walks of an object that read less than `fields` does: where only `properties` look at
//! a field and the data has more of them, where only `additionalProperties` does, and the quick
//! walk of most objects, whose fields come in the order of their names.

use std::cmp::Ordering;

use serde_json::{Map, Value};

use super::Walk;
use crate::node::{Extra, Objects};
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
                if bits.is_some() && property.bit != 0 {
                    return r.no(String::new);
                }
                continue;
            };
            met |= property.bit;
            let ok = self.has_type(property.types, value)
                || self.fits(id, value)
                || (property.leaf && self.leaf_holds(id, value))
                || self.node(id, value, &At::Key(at, name), r.part(), 0);
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
                match order(known, wanted) {
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
                    let walked = || self.node(property.id, value, &At::Root, Report::quiet(), 0);
                    // A node of types alone, and most leaves, are settled either way where
                    // the field stands: a value that surely fails is not walked to fail.
                    if property.types != 0 {
                        self.has_type(property.types, value)
                    } else if property.leaf {
                        self.leaf(property.id, value).unwrap_or_else(walked)
                    } else {
                        walked()
                    }
                }
                None => match objects.extra {
                    Extra::Absent | Extra::Allowed => true,
                    Extra::Refused => false,
                    Extra::Checked(id) => {
                        self.fits(id, value) || self.node(id, value, &At::Root, Report::quiet(), 0)
                    }
                },
            };
            if !ok {
                return false;
            }
        }
        met & required == required
    }

    /// Where only `additionalProperties` asks anything of a field: each held to it, nothing
    /// looked up; `None` where other keywords name fields.
    pub(super) fn only_extra<const F: bool>(
        &self,
        objects: &Objects,
        object: &Value,
        fields: &Map<String, Value>,
        at: &At<'_>,
        r: &mut Report<'_, F>,
    ) -> Option<bool> {
        let Extra::Checked(id) = objects.extra else {
            return None;
        };
        if !objects.properties.is_empty() || !objects.patterns.is_empty() || objects.names.is_some()
        {
            return None;
        }
        for (name, value) in fields {
            let ok = self.fits(id, value) || self.node(id, value, &At::Key(at, name), r.part(), 0);
            if !r.took(ok) {
                return Some(false);
            }
        }
        Some(!objects.closing || self.closing(objects, object, fields, at, r))
    }
}

#[cfg(test)]
#[path = "objects_tests.rs"]
mod tests;
