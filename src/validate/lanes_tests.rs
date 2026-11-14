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

/// `enum` and `const` by their short way say what the whole walk says, for values of every
/// kind and numbers written both ways.
#[test]
fn enums_and_constants_are_held_as_the_whole_walk_holds_them() {
    let schemas = [
        json!({"enum": [1, 2, 3, 5, 8]}),
        json!({"type": "integer", "enum": [1, 2, 3]}),
        json!({"type": "number", "enum": [1, 2.5]}),
        json!({"enum": ["a", 1, null, true, [1], {"a": 1}]}),
        json!({"type": "boolean", "enum": [true]}),
        json!({"const": 1}),
        json!({"const": 1.0}),
        json!({"type": "number", "const": 2.5}),
        json!({"type": "integer", "const": 1}),
        json!({"const": false}),
        json!({"const": [1, 2]}),
        json!({"$schema": "http://json-schema.org/draft-04/schema#", "type": "integer", "enum": [1, 2]}),
    ];
    let values = [
        json!(1),
        json!(1.0),
        json!(2.5),
        json!(3),
        json!(8),
        json!(9),
        json!(-1),
        json!("a"),
        json!("1"),
        json!(null),
        json!(true),
        json!(false),
        json!([1]),
        json!([1, 2]),
        json!({"a": 1}),
        json!({"a": 1.0}),
        json!(0),
    ];
    for schema in &schemas {
        let compiled = Schema::new(schema);
        for value in &values {
            let said = compiled.mismatches(value).is_empty();
            assert_eq!(compiled.matches(value), said, "{schema} with {value}");
        }
    }
}

/// A union of branches that only require names, beside the properties that hold them, is told
/// by the properties the walk met: it says what the whole walk says, whichever names come.
#[test]
fn a_union_of_required_names_is_told_by_the_properties_met() {
    let properties = json!({"a": {"type": "integer"}, "b": {"type": "integer"}, "c": {}, "d": {}});
    let schemas = [
        json!({"type": "object", "properties": properties, "anyOf": [{"required": ["a"]}, {"required": ["b", "c"]}]}),
        json!({"type": "object", "properties": properties, "oneOf": [{"required": ["a"]}, {"required": ["b"]}, {"required": ["c", "d"]}]}),
        json!({"properties": properties, "required": ["d"], "additionalProperties": false, "oneOf": [{"required": ["a"]}, {"required": ["d"]}]}),
        json!({"properties": properties, "required": ["z"], "anyOf": [{"required": ["a"]}, {"required": []}]}),
        // A name no property holds, a branch that asks more: told branch by branch.
        json!({"properties": properties, "anyOf": [{"required": ["a"]}, {"required": ["x"]}]}),
        json!({"properties": properties, "oneOf": [{"required": ["a"]}, {"required": ["b"], "minProperties": 2}]}),
        json!({"properties": properties, "oneOf": [{"type": "array", "required": ["a"]}, {"required": ["b"]}]}),
    ];
    let names = ["a", "b", "c", "d", "x", "z"];
    for schema in &schemas {
        let compiled = Schema::new(schema);
        // Every set of the six names, and a value that is no object.
        for set in 0_u32..64 {
            let fields: serde_json::Map<String, Value> = (names.iter().enumerate())
                .filter(|(place, _)| set >> place & 1 == 1)
                .map(|(_, name)| ((*name).to_string(), json!(1)))
                .collect();
            let value = Value::Object(fields);
            let said = compiled.mismatches(&value).is_empty();
            assert_eq!(compiled.matches(&value), said, "{schema} with {value}");
        }
        let other = json!("no object");
        assert_eq!(
            compiled.matches(&other),
            compiled.mismatches(&other).is_empty(),
            "{schema}"
        );
    }
}

/// An object held to a count of fields beside its required names, or beside one schema for
/// every field, by its short way says what the whole walk says.
#[test]
fn counted_objects_are_held_as_the_whole_walk_holds_them() {
    let schemas = [
        json!({"type": "object", "minProperties": 1}),
        json!({"minProperties": 2, "maxProperties": 3, "required": ["a"]}),
        json!({"type": "object", "maxProperties": 0}),
        json!({"type": "object", "minProperties": 1, "additionalProperties": {"type": "string"}}),
        json!({"maxProperties": 2, "additionalProperties": {"enum": ["x", "y"]}, "required": ["b"]}),
        json!({"minProperties": 2, "properties": {"a": {"type": "string"}, "c": {"const": "x"}}, "additionalProperties": false}),
        json!({"type": "object", "maxProperties": 2, "properties": {"a": {"type": "string"}}, "required": ["a"]}),
    ];
    let values = [
        json!({}),
        json!({"a": "x"}),
        json!({"a": "x", "b": "y"}),
        json!({"a": 1, "b": "y"}),
        json!({"a": "x", "b": "y", "c": "x"}),
        json!({"a": "x", "b": "y", "c": "x", "d": "y"}),
        json!({"b": "x"}),
        json!([]),
        json!("a"),
        json!(null),
    ];
    for schema in &schemas {
        let compiled = Schema::new(schema);
        for value in &values {
            let said = compiled.mismatches(value).is_empty();
            assert_eq!(compiled.matches(value), said, "{schema} with {value}");
        }
    }
}

/// A union's branch that cannot hold a value of some kind is not walked for it: the union
/// still says what the whole walk says, of values of every kind.
#[test]
fn unions_skip_the_branches_that_cannot_hold_a_kind_and_say_the_same() {
    let schemas = [
        json!({"oneOf": [{"type": "string"}, {"enum": [false]}]}),
        json!({"oneOf": [{"type": "integer"}, {"type": "number", "minimum": 2}, {"const": 1.5}]}),
        json!({"anyOf": [{"type": "number", "multipleOf": 2}, {"enum": [1, "a", null]}, {"const": [1]}]}),
        json!({"oneOf": [{"enum": "not a list"}, {"type": "null"}]}),
        json!({"oneOf": [{"$ref": "#/definitions/a"}, {"type": ["array", "object"], "minItems": 1}], "definitions": {"a": {"type": "string", "minLength": 2}}}),
        json!({"$schema": "http://json-schema.org/draft-04/schema#", "oneOf": [{"type": "integer"}, {"enum": [1.0, 2.5]}]}),
        json!({"type": "object", "properties": {"a": {}}, "anyOf": [{"type": "array"}, {"required": ["a"], "minProperties": 1}]}),
    ];
    let values = [
        json!(null),
        json!(true),
        json!(false),
        json!(1),
        json!(1.0),
        json!(1.5),
        json!(2),
        json!(2.5),
        json!(3),
        json!(4.0),
        json!("a"),
        json!("ab"),
        json!([]),
        json!([1]),
        json!({}),
        json!({"a": 1}),
    ];
    for schema in &schemas {
        let compiled = Schema::new(schema);
        for value in &values {
            let said = compiled.mismatches(value).is_empty();
            assert_eq!(compiled.matches(value), said, "{schema} with {value}");
        }
    }
}

/// An object that names far more properties than a value has fields finds each field by hash,
/// and says what the whole walk says: of fields it names, of others, of required ones missing.
#[test]
fn an_object_of_many_properties_finds_few_fields_by_hash() {
    let mut properties = serde_json::Map::new();
    for n in 0..40 {
        let kind = if n % 2 == 0 {
            json!({"type": "string"})
        } else {
            json!({"enum": ["x", 1]})
        };
        properties.insert(format!("field_{n}"), kind);
    }
    let schemas = [
        json!({"type": "object", "properties": properties}),
        json!({"properties": properties, "additionalProperties": false, "required": ["field_2"]}),
        json!({"properties": properties, "additionalProperties": {"type": "integer"}, "required": ["field_3", "other"]}),
        json!({"properties": properties, "minProperties": 2, "required": ["field_2", "field_3"]}),
    ];
    let values = [
        json!({}),
        json!({"field_2": "a"}),
        json!({"field_2": 1}),
        json!({"field_3": "x", "field_2": "b"}),
        json!({"field_3": 2}),
        json!({"other": 1, "field_3": 1}),
        json!({"other": "a", "field_3": "x"}),
        json!({"field_2": "a", "zzz": 1}),
        json!({"field_2": "a", "zzz": "b"}),
        json!([]),
        json!(1),
    ];
    for schema in &schemas {
        let compiled = Schema::new(schema);
        for value in &values {
            let said = compiled.mismatches(value).is_empty();
            assert_eq!(compiled.matches(value), said, "{schema} with {value}");
        }
    }
}
