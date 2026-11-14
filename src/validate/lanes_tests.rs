use serde_json::{Value, json};

use crate::Schema;

/// Each shape of node has its own way through the quick walk: whatever the way, it answers
/// what the walk that says what is wrong answers, of a value that matches and of one of every
/// other kind.
#[test]
fn every_way_answers_as_the_walk_that_says_what_is_wrong() {
    let values = [
        json!(null),
        json!(true),
        json!(0),
        json!(7),
        json!(2.5),
        json!(-1),
        json!(""),
        json!("ab"),
        json!("x-long-enough"),
        json!([]),
        json!([1, 2, 2]),
        json!(["a", "b"]),
        json!({}),
        json!({"a": 1}),
        json!({"a": "s", "b": 2}),
        json!({"x-a": "s", "x-b": "t"}),
        json!({"x-a": 1, "other": true}),
        json!({"a": 1, "b": 2, "c": 3, "d": 4, "e": 5, "f": 6}),
    ];
    let schemas = [
        // A leaf of each kind, and what a leaf leaves open.
        json!({"enum": ["ab", "cd"]}),
        json!({"const": "ab"}),
        json!({"type": "integer", "minimum": 0, "maximum": 7}),
        json!({"type": "number", "minimum": 0.5, "multipleOf": 0.5}),
        json!({"type": "string", "minLength": 2, "maxLength": 5}),
        json!({"minLength": 3}),
        json!({"enum": [7, "ab", null]}),
        json!({"const": 2.5}),
        // Objects: in order, by required names alone, by a pattern, by any keyword, with more
        // fields than properties.
        json!({"type": "object", "required": ["a"], "properties": {"a": {"type": "integer"}}}),
        json!({"required": ["a", "b"]}),
        json!({"type": "object", "patternProperties": {"^x-": {"type": "string"}},
            "additionalProperties": false}),
        json!({"patternProperties": {"^x-": {"type": "string"}}, "required": ["x-a"],
            "properties": {"other": {"type": "boolean"}}}),
        json!({"type": "object", "minProperties": 2, "propertyNames": {"maxLength": 3}}),
        json!({"type": "object", "properties": {"b": {"type": "integer"}}}),
        json!({"type": "object", "additionalProperties": {"type": "integer"}}),
        // Arrays: one schema for every item, and any keyword.
        json!({"type": "array", "items": {"type": "string"}, "minItems": 1}),
        json!({"type": "array", "uniqueItems": true, "items": {"type": "integer"}}),
        // Combinations alone, and beside a type.
        json!({"not": {"type": "string"}}),
        json!({"allOf": [{"type": "object"}, {"required": ["a"]}]}),
        json!({"anyOf": [{"required": ["a"]}, {"required": ["x-a"]}]}),
        json!({"oneOf": [{"type": "integer"}, {"type": "number"}]}),
        json!({"type": "object", "not": {"required": ["b"]}}),
        json!({"if": {"required": ["a"]}, "then": {"required": ["b"]}}),
        json!({"$ref": "#/$defs/n", "$defs": {"n": {"type": "integer", "minimum": 1}}}),
    ];
    for schema in &schemas {
        let compiled = Schema::new(schema);
        for value in &values {
            let said: Vec<String> = compiled.mismatches(value);
            assert_eq!(
                compiled.matches(value),
                said.is_empty(),
                "{schema} of {value}: {said:?}"
            );
        }
    }
    // The answers themselves, of one schema of each family.
    let holds = |schema: Value, value: Value| Schema::new(&schema).matches(&value);
    assert!(holds(
        json!({"required": ["a", "b"]}),
        json!({"a": 1, "b": 2})
    ));
    assert!(!holds(json!({"required": ["a", "b"]}), json!({"a": 1})));
    assert!(holds(json!({"not": {"type": "string"}}), json!(1)));
    assert!(!holds(json!({"not": {"type": "string"}}), json!("s")));
    assert!(
        holds(json!({"minLength": 3}), json!(1)) && !holds(json!({"minLength": 3}), json!("ab"))
    );
}
