//! A union of names: an `anyOf` or a `oneOf` whose every branch is an `enum` of strings, a
//! `const` that is a string, or such a union again (a list of scopes, an `enum` whose values
//! each carry a title) holds the strings of one list and nothing else. The list is found once
//! here, each string a branch names tried against the union as it is walked, and a quick walk
//! asks the list (`Walk::holds`). A walk that says what is wrong still goes through the
//! branches, and says the same.

use serde_json::Value;

use crate::hash::{Listed, Table};
use crate::node::{ALL, INTEGER, Id, Lane, Leaf, NUMBER, Node, OBJECT, kind_of};
use crate::validate::Walk;

/// The most strings a union's branches may name between them for its list to be made.
const MOST: usize = 512;
/// How deep a union's branches are followed.
const DEPTH: usize = 8;

#[cold]
pub(super) fn settle(nodes: &mut [Node]) {
    for id in 0..nodes.len() {
        let Some(Node::Rules(rules)) = nodes.get(id) else {
            continue;
        };
        if !matches!(rules.lane, Lane::Union | Lane::Both) {
            continue;
        }
        let branches = rules.any_of.iter().chain(&*rules.one_of);
        let kinds: Box<[u8]> = branches.map(|branch| kinds(nodes, *branch)).collect();
        if let Some(Node::Rules(rules)) = nodes.get_mut(id) {
            rules.kinds = kinds;
        }
    }
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
    for id in 0..nodes.len() {
        if let Some(branches) = required(nodes, id)
            && let Some(Node::Rules(rules)) = nodes.get_mut(id)
            && let Some(objects) = &mut rules.object
        {
            for (property, bit) in objects.properties.iter_mut().zip(&branches.0) {
                property.bit = *bit;
            }
            objects.branches = branches.1;
        }
    }
}

/// The kinds of value branch `id` can hold at all, read off its types, its `enum` and its
/// `const`; every kind where it says none of them. A number of either sort stands for both.
fn kinds(nodes: &[Node], id: Id) -> u8 {
    let wide = |kinds: u8| match kinds & (NUMBER | INTEGER) {
        0 => kinds,
        _ => kinds | NUMBER | INTEGER,
    };
    let mut id = id;
    for _ in 0..DEPTH {
        let rules = match nodes.get(id) {
            Some(Node::Rules(rules)) => rules,
            Some(Node::Types(types, _)) => return wide(*types),
            Some(Node::Never) => return 0,
            Some(Node::Any) | None => return ALL,
        };
        match (rules.only_types, rules.alias) {
            (Some(types), _) => return wide(types),
            (None, Some(target)) => id = target,
            (None, None) => {
                let listed = matches!(rules.options, Some(Value::Array(_)));
                let options = if listed {
                    wide(rules.option_kinds)
                } else {
                    ALL
                };
                let constant =
                    (rules.constant.as_ref()).map_or(ALL, |one| wide(kind_of(one, false)));
                return rules.types.map_or(ALL, wide) & options & constant;
            }
        }
    }
    ALL
}

/// For an object with one union beside it whose every branch only requires names the object's
/// `properties` hold: each property's bit (its own where `required` gave it one, a free one
/// where a branch names it) and each branch's names as those bits. `None` for any other node,
/// or where the bits run out.
fn required(nodes: &[Node], id: Id) -> Option<(Vec<u64>, Box<[u64]>)> {
    let Some(Node::Rules(rules)) = nodes.get(id) else {
        return None;
    };
    let objects = rules.object.as_ref().filter(|_| rules.lane == Lane::Both)?;
    let mut bits: Vec<u64> = objects.properties.iter().map(|p| p.bit).collect();
    let mut used = bits.iter().fold(0, |all, bit| all | bit);
    let mut branches = vec![];
    for branch in rules.any_of.iter().chain(&*rules.one_of) {
        let Some(Node::Rules(branch)) = nodes.get(*branch) else {
            return None;
        };
        let names = branch.object.as_ref().filter(|names| {
            branch.lane == Lane::Required
                && !names.closing
                && branch.leaf == Leaf::Walked
                && branch.types.is_none_or(|types| types & OBJECT != 0)
        })?;
        let mut mask = 0;
        for name in names.required.iter().map(|it| names.required_name(it)) {
            let place = (objects.properties)
                .binary_search_by(|property| (*property.name).cmp(name))
                .ok()?;
            let bit = bits.get_mut(place)?;
            if *bit == 0 {
                // The lowest bit no property has yet; none left is no list of branches.
                *bit = 1_u64.checked_shl(used.trailing_ones())?;
                used |= *bit;
            }
            mask |= *bit;
        }
        branches.push(mask);
    }
    Some((bits, branches.into_boxed_slice()))
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
