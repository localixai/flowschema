use serde_json::{Value, json};

use crate::Schema;
use crate::compile::compile;
use crate::node::{Leaf, Node};

/// The leaf the root of `schema` was found to be.
fn leaf(schema: &Value) -> Option<Leaf> {
    match compile(schema, &[]).nodes.first() {
        Some(Node::Rules(rules)) => Some(rules.leaf),
        Some(Node::Any | Node::Never | Node::Types(..)) | None => None,
    }
}

#[test]
fn common_leaves_are_found_and_the_rest_are_walked() {
    let cases = [
        (json!({"enum": ["a", "b"]}), Leaf::Strings),
        (
            json!({"type": ["string", "null"], "enum": ["a", null]}),
            Leaf::Walked,
        ),
        (
            json!({"type": "integer", "minimum": 1, "exclusiveMaximum": 10}),
            Leaf::Range,
        ),
        (json!({"type": "integer", "minimum": 1.5}), Leaf::Walked),
        (json!({"type": "number", "multipleOf": 2}), Leaf::Walked),
        (json!({"type": "string", "maxLength": 5}), Leaf::Text),
        (json!({"type": "string", "pattern": "^a"}), Leaf::Text),
        (json!({"format": "uuid"}), Leaf::Text),
        (
            json!({"type": "string", "pattern": "^a", "enum": ["a"]}),
            Leaf::Walked,
        ),
        (json!({"type": "string", "minimum": 1}), Leaf::Walked),
        (json!({"const": "read"}), Leaf::Const),
        (json!({"type": "string", "const": "read"}), Leaf::Const),
        (json!({"const": null}), Leaf::Const),
        (json!({"const": 1}), Leaf::Walked),
        (json!({"type": "integer", "const": "read"}), Leaf::Walked),
        (json!({"const": "read", "enum": ["read"]}), Leaf::Walked),
    ];
    for (schema, wanted) in cases {
        assert_eq!(leaf(&schema), Some(wanted), "{schema}");
    }
}

#[test]
fn a_leaf_judges_as_its_rules_do() {
    let cases = [
        (
            json!({"enum": ["low", "high"]}),
            vec![json!("low"), json!("mid"), json!(1)],
        ),
        (
            json!({"type": "integer", "minimum": 1, "maximum": 3}),
            vec![
                json!(0),
                json!(1),
                json!(3),
                json!(4),
                json!(2.0),
                json!(2.5),
                json!("2"),
            ],
        ),
        (
            json!({"type": "string", "minLength": 2, "maxLength": 3}),
            vec![
                json!("a"),
                json!("ab"),
                json!("abcd"),
                json!("ééé"),
                json!("éééé"),
                json!(5),
            ],
        ),
        (
            json!({"type": "string", "pattern": "^a+$", "format": "email"}),
            vec![json!("aa"), json!("ab"), json!("a@b.c"), json!(1)],
        ),
        (json!({"format": "uuid"}), vec![json!("x"), json!(1)]),
        (
            json!({"const": "read"}),
            vec![json!("read"), json!("write"), json!(1), json!(null)],
        ),
        (
            json!({"const": true}),
            vec![json!(true), json!(false), json!("true"), json!(1)],
        ),
    ];
    for (schema, values) in cases {
        // In an object, so the walk settles each field where it stands.
        let wrapped = Schema::new(&json!({"properties": {"x": schema}}));
        let alone = Schema::new(&json!({"allOf": [schema]}));
        for value in values {
            let data = json!({"x": value});
            assert_eq!(wrapped.matches(&data), alone.matches(&value), "{data}");
            assert_eq!(
                wrapped.mismatches(&data).is_empty(),
                alone.matches(&value),
                "{data}"
            );
        }
    }
}
