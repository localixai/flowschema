use serde_json::{Value, json};

use crate::Schema;
use crate::compile::compile;
use crate::node::{Node, STRING};

/// What the property `name` of the root of `schema` was told: its node, leaf and types.
fn told(schema: &Value, name: &str) -> Option<(usize, bool, u8)> {
    let nodes = compile(schema, &[]).nodes;
    match nodes.first() {
        Some(Node::Rules(rules)) => (rules.object.as_ref()?.properties.iter())
            .find(|property| &*property.name == name)
            .map(|property| (property.id, property.leaf, property.types)),
        Some(Node::Any | Node::Never | Node::Types(..)) | None => None,
    }
}

#[test]
fn a_property_by_reference_is_named_where_it_lands() {
    let schema = json!({"$defs": {"a": {"$ref": "#/$defs/b"}, "b": {"type": "string"},
            "c": {"enum": ["x"]}, "maybe": {"anyOf": [{"type": "string"}, {"type": "null"}]}},
        "properties": {"text": {"$ref": "#/$defs/a"}, "kind": {"$ref": "#/$defs/c"},
            "note": {"$ref": "#/$defs/maybe"}}});
    let (_, leaf, types) = told(&schema, "text").unwrap();
    assert_eq!((leaf, types), (false, STRING));
    assert_eq!(told(&schema, "kind").map(|(_, leaf, _)| leaf), Some(true));
    // A union of types, folded, is told as its types.
    assert!(told(&schema, "note").is_some_and(|(_, _, types)| types & STRING != 0));
}

#[test]
fn references_that_turn_on_themselves_are_left_to_the_walk() {
    let schema = json!({"$defs": {"a": {"$ref": "#/$defs/b"}, "b": {"$ref": "#/$defs/a"}},
        "properties": {"x": {"$ref": "#/$defs/a"}}});
    let looped = Schema::new(&schema);
    assert!(!looped.matches(&json!({"x": 1})));
    assert_eq!(looped.mismatches(&json!({"x": 1})).len(), 1);
    assert!(looped.matches(&json!({})));
}

#[test]
fn a_schema_that_applies_itself_to_the_same_value_still_ends() {
    for schema in [
        json!({"$defs": {"a": {"allOf": [{"$ref": "#/$defs/a"}]}}, "$ref": "#/$defs/a"}),
        json!({"$defs": {"a": {"anyOf": [{"$ref": "#/$defs/a"}]}},
            "properties": {"x": {"$ref": "#/$defs/a"}}}),
        json!({"$defs": {"a": {"not": {"$ref": "#/$defs/a"}}}, "items": {"$ref": "#/$defs/a"}}),
    ] {
        let looped = Schema::new(&schema);
        for data in [json!({"x": 1}), json!([1]), json!(1)] {
            // Whatever it says, it says so: no walk without end.
            assert_eq!(looped.matches(&data), looped.mismatches(&data).is_empty());
        }
    }
}
