//! Children named where they land, once every node is built: a field's or an item's schema that
//! is a `$ref` and nothing else (`Rules::alias`) is replaced, in the node that names it, by the
//! node the reference lands on, so the walk steps from a value straight to its schema, with no
//! node in between; and each property is told the types its node asks, where they are all it
//! asks (`Property::types`), now that unions of types are folded.

use crate::node::{ALL, Extra, Id, Node, Rules};

/// How many references in a row are followed: a longer chain, or one that turns on itself, is
/// left as it is, for the walk to count.
const CHAIN: usize = 8;

/// Every child of `nodes` named where it lands (where any node was `referred` to), and every
/// property told what its node asks.
pub(crate) fn settle(nodes: &mut [Node], referred: bool) {
    if referred {
        let lands: Vec<Id> = (0..nodes.len()).map(|id| landing(nodes, id)).collect();
        let land = |id: &mut Id| *id = lands.get(*id).copied().unwrap_or(*id);
        for node in nodes.iter_mut() {
            if let Node::Rules(rules) = node {
                children(rules, &land);
            }
        }
    }
    for at in 0..nodes.len() {
        let mut place = 0;
        while let Some(child) = property(nodes, at, place) {
            let types = told(nodes, child);
            if let Some(Node::Rules(rules)) = nodes.get_mut(at)
                && let Some(objects) = &mut rules.object
                && let Some(property) = objects.properties.get_mut(place)
            {
                property.types = types;
            }
            place += 1;
        }
        if let Some(extra) = extra(nodes, at) {
            let types = told(nodes, extra);
            if let Some(Node::Rules(rules)) = nodes.get_mut(at)
                && let Some(objects) = &mut rules.object
            {
                objects.extra_types = types;
            }
        }
    }
}

/// The node every field node `at` does not name is held to.
fn extra(nodes: &[Node], at: Id) -> Option<Id> {
    match nodes.get(at)? {
        Node::Rules(rules) => match rules.object.as_ref()?.extra {
            Extra::Checked(id) => Some(id),
            Extra::Absent | Extra::Allowed | Extra::Refused => None,
        },
        Node::Any | Node::Never | Node::Types(..) => None,
    }
}

/// Where node `id` lands: itself, or the end of the references it only is.
fn landing(nodes: &[Node], id: Id) -> Id {
    let mut at = id;
    for _ in 0..CHAIN {
        match nodes.get(at) {
            Some(Node::Rules(rules)) => match rules.alias {
                Some(next) => at = next,
                None => return at,
            },
            Some(Node::Any | Node::Never | Node::Types(..)) | None => return at,
        }
    }
    id
}

/// Every child `rules` hold a value inside the data to (a field's, an item's, a name), given
/// to `land`. A child held to the same value (`$ref`, `allOf`, `anyOf`, `not`, `if`) keeps its
/// reference: the walk counts those it follows with no step into the data, and a schema that
/// refers to itself ends there.
fn children(rules: &mut Rules, land: &impl Fn(&mut Id)) {
    let single = [&mut rules.unevaluated_fields, &mut rules.unevaluated_items];
    single.into_iter().flatten().for_each(land);
    if let Some(arrays) = &mut rules.array {
        (arrays.prefix.iter_mut())
            .chain(&mut arrays.rest)
            .for_each(land);
        if let Some((contains, _, _)) = &mut arrays.contains {
            land(contains);
        }
    }
    if let Some(objects) = &mut rules.object {
        (objects.properties.iter_mut()).for_each(|property| land(&mut property.id));
        (objects.patterns.iter_mut()).for_each(|(_, id)| land(id));
        objects.names.iter_mut().for_each(land);
        if let Extra::Checked(id) = &mut objects.extra {
            land(id);
        }
    }
}

/// The node of the property at `place` of node `at`.
fn property(nodes: &[Node], at: Id, place: usize) -> Option<Id> {
    match nodes.get(at)? {
        Node::Rules(rules) => Some(rules.object.as_ref()?.properties.get(place)?.id),
        Node::Any | Node::Never | Node::Types(..) => None,
    }
}

/// The types node `id` asks, where they are all it asks (every type for one that takes
/// anything): what a property is told of its node.
fn told(nodes: &[Node], id: Id) -> u8 {
    match nodes.get(id) {
        Some(Node::Rules(rules)) => rules.only_types.unwrap_or(0),
        Some(Node::Types(types, _)) => *types,
        Some(Node::Any) => ALL,
        Some(Node::Never) | None => 0,
    }
}

#[cfg(test)]
#[path = "flatten_tests.rs"]
mod tests;
