use serde_json::json;

use crate::Schema;

#[test]
fn what_a_model_wrote_as_a_string_is_read_as_it_meant() {
    let schema = Schema::new(&json!({"type": "object", "properties": {
        "limit": {"type": "integer"}, "ratio": {"type": "number"}, "deep": {"type": "boolean"},
        "name": {"type": "string"}, "either": {"type": ["string", "integer"]},
        "tags": {"type": "array", "items": {"type": "integer"}},
        "where": {"$ref": "#/$defs/place"}},
        "$defs": {"place": {"type": "object", "properties": {"line": {"type": "integer"}}}}}));
    let mut data = json!({"limit": "30", "ratio": " 0.5 ", "deep": "true", "name": "12",
        "either": "7", "tags": "[1, \"2\"]", "where": "{\"line\": \"4\"}"});
    assert!(!schema.matches(&data));
    let said = schema.mend(&mut data);
    assert_eq!(
        data,
        json!({"limit": 30, "ratio": 0.5, "deep": true, "name": "12",
        "either": "7", "tags": [1, 2], "where": {"line": 4}})
    );
    assert!(schema.matches(&data));
    assert_eq!(said.len(), 7, "{said:#?}");
    assert!(
        said.contains(&r#"data.limit: "30" read as 30"#.to_string()),
        "{said:#?}"
    );
}

#[test]
fn nothing_is_guessed() {
    let schema = Schema::new(&json!({"properties": {"limit": {"type": "integer"},
        "any": {}, "one": {"anyOf": [{"type": "integer"}, {"type": "boolean"}]}}}));
    // Not the number asked for, no type asked, and a union: each left as written.
    let mut data = json!({"limit": "3.5", "any": "1", "one": "1", "other": "true"});
    let written = data.clone();
    assert_eq!(schema.mend(&mut data), Vec::<String>::new());
    assert_eq!(data, written);
}
