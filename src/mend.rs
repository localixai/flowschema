//! What a model wrote read as it meant it, where the schema leaves no doubt: a number, a
//! boolean or null written as a string, and an object or an array sent as a string of JSON
//! (a call's arguments often are), where the schema asks for the one and not for a string. Each
//! is made what it was meant to be, and said, before the data is checked; nothing is guessed
//! where a string is allowed, and nothing inside a union.

use serde_json::Value;

use crate::node::{ARRAY, BOOLEAN, INTEGER, Id, NULL, NUMBER, Node, OBJECT, STRING, kind_of};
use crate::report::short;

/// How many `$ref`s that are only a `$ref` are followed to a node.
const CHAIN: usize = 8;

/// `value`, held to node `id`, mended in place; each change said in `said`, with where.
pub(crate) fn mend(nodes: &[Node], id: Id, value: &mut Value, at: &str, said: &mut Vec<String>) {
    let mut id = id;
    let rules = 'found: {
        for _ in 0..CHAIN {
            match nodes.get(id) {
                Some(Node::Types(types, _)) => return reread(*types, value, at, said),
                Some(Node::Rules(rules)) => match rules.alias {
                    Some(target) => id = target,
                    None => break 'found rules,
                },
                Some(Node::Any | Node::Never) | None => return,
            }
        }
        return;
    };
    if let Some(types) = rules.types {
        reread(types, value, at, said);
    }
    match value {
        Value::Object(fields) => {
            let properties = rules
                .object
                .iter()
                .flat_map(|objects| objects.properties.iter());
            for property in properties {
                if let Some(inner) = fields.get_mut(&*property.name) {
                    mend(
                        nodes,
                        property.id,
                        inner,
                        &format!("{at}.{}", property.name),
                        said,
                    );
                }
            }
        }
        Value::Array(items) => {
            let Some(arrays) = &rules.array else { return };
            for (place, item) in items.iter_mut().enumerate() {
                if let Some(id) = arrays.prefix.get(place).copied().or(arrays.rest) {
                    mend(nodes, id, item, &format!("{at}[{place}]"), said);
                }
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {}
    }
}

/// A string where none of `types` is a string, read as the value it writes, where that is one
/// of them.
fn reread(types: u8, value: &mut Value, at: &str, said: &mut Vec<String>) {
    let Value::String(text) = &*value else { return };
    if types & STRING != 0 {
        return;
    }
    let Ok(meant) = serde_json::from_str::<Value>(text.trim()) else {
        return;
    };
    // The kind it writes, an integer told from a number as the drafts after 4 tell it.
    let wanted = types & (INTEGER | NUMBER | BOOLEAN | NULL | OBJECT | ARRAY);
    if kind_of(&meant, false) & wanted == 0 {
        return;
    }
    said.push(format!("{at}: {} read as {}", short(value), short(&meant)));
    *value = meant;
}

#[cfg(test)]
#[path = "mend_tests.rs"]
mod tests;
