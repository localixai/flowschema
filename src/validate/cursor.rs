//! The fields `properties` names, met as a data object's fields come: both in the order of
//! their names, so a cursor finds each where it left the last.

use std::cmp::Ordering;

use serde_json::{Map, Value};

use crate::node::{Id, Property};
use crate::text::{field, order, same_bytes};

/// The fields `properties` names, met in the order of their names: a data object's fields come
/// in that order (serde_json's map), so each is found where the last one was left, most in one
/// comparison; one out of that order is searched for.
pub(super) struct Cursor<'p, 'd> {
    properties: &'p [Property],
    next: usize,
    /// The `required` bits of the properties gone past with no field met.
    pub(super) passed: u64,
    /// The last field's name: a name after it comes in order, and is not behind the cursor.
    last: &'d str,
}

impl<'p, 'd> Cursor<'p, 'd> {
    /// A cursor before the first of `properties`.
    pub(super) fn new(properties: &'p [Property]) -> Self {
        Cursor {
            properties,
            next: 0,
            passed: 0,
            last: "",
        }
    }

    /// The node of the field `name`, with its `required` bit.
    pub(super) fn find(&mut self, name: &'d str) -> Option<(Id, u64)> {
        let wanted = name.as_bytes();
        let last = std::mem::replace(&mut self.last, name);
        let properties = self.properties;
        while let Some(known) = properties.get(self.next) {
            let bytes = known.name.as_bytes();
            // Most fields are the next property: told so with no walk over their bytes.
            if same_bytes(bytes, wanted) {
                self.next += 1;
                return Some((known.id, known.bit));
            }
            match order(bytes, wanted) {
                Ordering::Less => {
                    self.passed |= known.bit;
                    self.next += 1;
                }
                Ordering::Equal => {
                    self.next += 1;
                    return Some((known.id, known.bit));
                }
                Ordering::Greater => break,
            }
        }
        // In order, every property behind the cursor is before an earlier name: none is this one.
        if order(wanted, last.as_bytes()) == Ordering::Greater {
            return None;
        }
        let place =
            (properties.binary_search_by(|known| order(known.name.as_bytes(), wanted))).ok()?;
        properties.get(place).map(|known| (known.id, known.bit))
    }
}

impl Cursor<'_, '_> {
    /// Whether a required property gone past unmet is not in `fields` (it may have come out of
    /// order); those found are not asked again.
    pub(super) fn missing(&mut self, fields: &Map<String, Value>, met: u64) -> bool {
        let lost = self.passed & !met;
        self.passed &= met;
        (self.properties.iter())
            .filter(|known| lost & known.bit != 0)
            .any(|known| field(fields, &known.name).is_none())
    }
}

#[cfg(test)]
#[path = "cursor_tests.rs"]
mod tests;
