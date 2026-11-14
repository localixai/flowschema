use serde_json::json;

use crate::Schema;

fn tool() -> Schema {
    Schema::new(
        &json!({"type": "object", "required": ["path"], "additionalProperties": false,
        "properties": {
            "path": {"type": "string"},
            "mode": {"enum": ["read", "write"]},
            "limit": {"type": "integer", "maximum": 100},
            "tags": {"type": "array", "maxItems": 2, "items": {"type": "string"}},
            "where": {"$ref": "#/$defs/place"}},
        "$defs": {"place": {"type": "object", "properties": {"line": {"type": "integer"}}}}}),
    )
}

#[test]
fn what_an_ending_can_still_mend_may_match() {
    let schema = tool();
    for written in [
        "",
        "{",
        r#"{"pa"#,
        r#"{"path": "src/li"#,
        // A number may go on, and a required field may still come.
        r#"{"limit": 12"#,
        r#"{"mode": "re"#,
        r#"{"mode": "read", "tags": ["a", "b"#,
        r#"{"where": {"line": 4"#,
        r#"{"where": {"line": 4, "other": "x"#,
        // A name is not known until it is whole.
        r#"{"mode": "read", "extr"#,
        r#"{"path": "a", "mode": "write"}"#,
    ] {
        assert!(schema.may_match(written), "{written}");
    }
}

#[test]
fn what_no_ending_mends_cannot_match() {
    let schema = tool();
    for written in [
        // A value written whole that is not what its field asks.
        r#"{"path": 12,"#,
        r#"{"limit": 500,"#,
        r#"{"where": {"line": "x"#,
        r#"{"tags": ["a", 1,"#,
        // No option begins so, and a third item is one too many.
        r#"{"mode": "x"#,
        r#"{"tags": ["a", "b", "c"#,
        // A field the schema refuses, its name written whole.
        r#"{"path": "a", "extra": "v"#,
        r#"{"path": "a", "extra": 1,"#,
        // The wrong kind at the root, and a whole call that does not match.
        "[",
        r#"{"mode": "read"}"#,
        // Not the start of JSON.
        "} {",
    ] {
        assert!(!schema.may_match(written), "{written}");
    }
}

#[test]
fn what_only_the_whole_value_tells_is_left_to_it() {
    // A union, and a pattern that may take any name: nothing is said before the end.
    let union = Schema::new(&json!({"anyOf": [
        {"type": "object", "required": ["a"], "additionalProperties": false,
            "properties": {"a": {"type": "integer"}}},
        {"type": "string"}]}));
    assert!(union.may_match(r#"{"b": 1,"#));
    assert!(!union.matches(&json!({"b": 1})));
    let patterned = Schema::new(&json!({"type": "object", "additionalProperties": false,
        "patternProperties": {"^x-": {"type": "integer"}}}));
    assert!(patterned.may_match(r#"{"x-a": "no","#));
    // A word or a number at the root is not known until it ends.
    let flag = Schema::new(&json!({"type": "boolean"}));
    assert!(flag.may_match("tru") && flag.may_match("12"));
    assert!(flag.may_match("true") && !flag.may_match(r#""tru"#));
}

#[test]
fn every_beginning_of_a_call_that_matches_may_match() {
    // The one promise: no text that can still end in a match is refused. Each call here
    // matches, so each of its beginnings must be let through, cut wherever it is.
    let cases = [
        (
            json!({"type": "object", "required": ["path", "edits"], "additionalProperties": false,
                "properties": {
                    "path": {"type": "string", "minLength": 3, "maxLength": 40},
                    "mode": {"enum": ["read", "write", "write-new"]},
                    "kind": {"const": "patch"},
                    "ratio": {"type": "number", "minimum": 0, "maximum": 1},
                    "dry": {"type": ["boolean", "null"]},
                    "edits": {"type": "array", "minItems": 1, "maxItems": 3, "items": {
                        "type": "object", "required": ["line", "text"],
                        "additionalProperties": false,
                        "properties": {"line": {"type": "integer", "minimum": 1},
                            "text": {"type": "string"}}}}}}),
            json!({"path": "src/\"lib\".rs", "mode": "write-new", "kind": "patch",
                "ratio": 0.25, "dry": null, "edits": [{"line": 12, "text": "caf\u{e9} \\ {[\n"},
                    {"line": 7, "text": "\u{1f600}"}]}),
        ),
        (
            json!({"anyOf": [{"type": "array", "prefixItems": [{"type": "integer"}],
                "items": {"$ref": "#/$defs/name"}}, {"type": "null"}],
                "$defs": {"name": {"type": "string", "pattern": "^[a-z]+$"}}}),
            json!([3, "ab", "c"]),
        ),
        (
            json!({"type": "object", "patternProperties": {"^x-": {"type": "integer"}},
                "additionalProperties": {"type": "string"}, "minProperties": 2}),
            json!({"x-a": 1, "note": "n", "other": "o"}),
        ),
    ];
    for (schema, data) in cases {
        let schema = Schema::new(&schema);
        assert!(schema.matches(&data), "{data}");
        let pretty = serde_json::to_string_pretty(&data).unwrap();
        for text in [data.to_string(), pretty] {
            for (end, _) in text.char_indices() {
                let written = text.get(..end).unwrap();
                assert!(schema.may_match(written), "{written}");
            }
            assert!(schema.may_match(&text), "{text}");
        }
    }
}
