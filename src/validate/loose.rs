//! The quick walk of an object held to `patternProperties` beside its properties: each field is
//! the next property or none (as `Walk::in_order` reads them), then tried against each
//! pattern, with nothing said and no place named. What `Walk::fields` answers a quick walk of
//! such an object, by a shorter way: a map keyed by a pattern is the commonest shape of object
//! after the plain one.

use std::cmp::Ordering;

use serde_json::{Map, Value};

use super::Walk;
use crate::node::{Extra, Objects};
use crate::report::{At, Report};
use crate::text::{field, order};

impl Walk<'_> {
    /// Whether the object `value` matches `objects`, which are held to patterns (and to no
    /// `propertyNames`, count or dependency: `Lane::Patterned`).
    pub(super) fn patterned(
        &self,
        objects: &Objects,
        value: &Value,
        fields: &Map<String, Value>,
    ) -> bool {
        let sparse = objects.properties.len() > fields.len().saturating_mul(4);
        // Fields out of the order of their names, or far fewer than the properties named,
        // are read as any object's are.
        if !self.sorted || (sparse && objects.lookup.is_some()) {
            return self.fields(objects, value, fields, &At::Root, &mut Report::quiet());
        }
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
        let (mut next, mut met) = (0, 0);
        for (name, value) in fields {
            let mut found = None;
            while let Some(property) = properties.get(next) {
                match order(property.name.as_bytes(), name.as_bytes()) {
                    Ordering::Less if property.bit & required != 0 => return false,
                    Ordering::Less => next += 1,
                    Ordering::Equal => {
                        (found, next) = (Some(property), next + 1);
                        break;
                    }
                    Ordering::Greater => break,
                }
            }
            let mut named = false;
            if let Some(property) = found {
                (met, named) = (met | property.bit, true);
                let ok = match property.types {
                    0 => self.holds(property.id, value, 0),
                    types => self.has_type(types, value),
                };
                if !ok {
                    return false;
                }
            }
            for (place, id) in &objects.patterns {
                if self.matches(*place, name) {
                    named = true;
                    if !self.holds(*id, value, 0) {
                        return false;
                    }
                }
            }
            let ok = match (named, &objects.extra) {
                (true, _) | (false, Extra::Absent | Extra::Allowed) => true,
                (false, Extra::Refused) => false,
                (false, Extra::Checked(id)) => self.holds(*id, value, 0),
            };
            if !ok {
                return false;
            }
        }
        met & required == required
    }
}
