//! A union of names: an `anyOf` or a `oneOf` whose every branch is an `enum` of strings, a
//! `const` that is a string, or such a union again (a list of scopes, an `enum` whose values
//! each carry a title) holds the strings of one list and nothing else. The list is found once
//! here, each string a branch names tried against the union as it is walked, and a quick walk
//! asks the list (`Walk::holds`). A walk that says what is wrong still goes through the
//! branches, and says the same.

use serde_json::Value;

use crate::hash::{Listed, Table};
use crate::node::{Id, Lane, Leaf, Node};
use crate::validate::Walk;

/// The most strings a union's branches may name between them for its list to be made.
const MOST: usize = 512;
/// How deep a union's branches are followed.
const DEPTH: usize = 8;

#[cold]
pub(super) fn settle(nodes: &mut [Node]) {
    let mut lists = vec![];
    for (id, node) in nodes.iter().enumerate() {
        let union = matches!(node, Node::Rules(rules) if rules.lane == Lane::Union);
        let mut names = vec![];
        if !union || !named(nodes, id, 0, &mut names) || names.len() > MOST {
            continue;
        }
        names.sort_unstable();
        names.dedup();
        // Strings alone are tried: neither the draft's integers nor a map's order is read.
        let (patterns, strict, sorted) = (&[][..], false, true);
        let walk = Walk {
            nodes,
            patterns,
            strict,
            sorted,
        };
        let held = (names
            .into_iter()
            .map(|name| Value::String(name.to_string())))
        .filter(|name| walk.holds(id, name, 0))
        .collect::<Vec<Value>>();
        lists.push((id, held));
    }
    for (id, options) in lists {
        if let Some(Node::Rules(rules)) = nodes.get_mut(id) {
            let table = Table::of(&options);
            rules.listed = Some(Box::new(Listed { options, table }));
        }
    }
}

/// Whether node `id` holds strings it names and nothing else, each of them added to `names`.
fn named<'n>(nodes: &'n [Node], id: Id, depth: usize, names: &mut Vec<&'n str>) -> bool {
    let Some(Node::Rules(rules)) = nodes.get(id).filter(|_| depth < DEPTH) else {
        return false;
    };
    if let Some(target) = rules.alias {
        return named(nodes, target, depth + 1, names);
    }
    match (rules.leaf, rules.lane, &rules.options, &rules.constant) {
        (Leaf::Strings, _, Some(Value::Array(options)), _) => {
            names.extend(options.iter().filter_map(Value::as_str));
            true
        }
        (Leaf::Const, _, _, Some(Value::String(only))) => {
            names.push(only);
            true
        }
        (Leaf::Walked, Lane::Union, _, _) => (rules.any_of.iter().chain(&*rules.one_of))
            .all(|branch| named(nodes, *branch, depth + 1, names)),
        (Leaf::Walked | Leaf::Strings | Leaf::Const | Leaf::Range | Leaf::Text, _, _, _) => false,
    }
}

#[cfg(test)]
#[path = "unions_tests.rs"]
mod tests;
