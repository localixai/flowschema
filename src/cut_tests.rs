use serde_json::json;

use super::*;

#[test]
fn a_whole_text_is_itself() {
    assert_eq!(whole(r#"{"a": [1, 2]}"#), Some(json!({"a": [1, 2]})));
    assert_eq!(whole("12"), Some(json!(12)));
}

#[test]
fn what_was_written_before_the_cut_is_kept() {
    for (text, kept) in [
        // Inside a value's string: kept as far as it came.
        (r#"{"path": "src/li"#, json!({"path": "src/li"})),
        (
            r#"{"a": 1, "text": "two\nlin"#,
            json!({"a": 1, "text": "two\nlin"}),
        ),
        // Half an escape is not a character.
        (r#"{"text": "caf\u00"#, json!({"text": "caf"})),
        (r#"{"text": "a\"#, json!({"text": "a"})),
        // A name with no value, and a value not begun, are left out.
        (r#"{"a": 1, "b"#, json!({"a": 1})),
        (r#"{"a": 1, "b":"#, json!({"a": 1})),
        (r#"{"a": 1,"#, json!({"a": 1})),
        // A number or a word may go on.
        (r#"{"a": 1, "b": 12"#, json!({"a": 1})),
        (r#"{"a": 1, "b": tru"#, json!({"a": 1})),
        (r#"{"a": 1, "b": true"#, json!({"a": 1, "b": true})),
        // What is open is closed, as deep as it is.
        (
            r#"{"issues": [{"file": "a.rs", "line": 3}, {"file": "b"#,
            json!({"issues": [{"file": "a.rs", "line": 3}, {"file": "b"}]}),
        ),
        (r#"[[1, 2], [3"#, json!([[1, 2], []])),
        (r#"{"a": {"#, json!({"a": {}})),
    ] {
        assert_eq!(whole(text), Some(kept), "{text}");
    }
}

#[test]
fn what_is_no_start_of_json_is_nothing() {
    for text in ["", "hello", "} {"] {
        assert_eq!(whole(text), None, "{text}");
    }
}

#[test]
fn where_the_text_was_still_writing_is_told() {
    let key = |name: &str| Step::Key(name.to_string());
    for (text, path, in_text) in [
        // Inside a string, down to it; after a whole value, down to what holds it.
        (r#"{"a": {"b": "x"#, vec![key("a"), key("b")], true),
        (r#"{"a": {"b": true"#, vec![key("a")], false),
        (
            r#"{"a": [1, {"b\"c": "x"#,
            vec![key("a"), Step::Index(1), key("b\"c")],
            true,
        ),
        // What the cut left unfinished is left out, and with it the step into it.
        (r#"{"a": [1, 2"#, vec![key("a")], false),
        (r#"{"a": 1, "b"#, vec![], false),
        (r#""x"#, vec![], true),
    ] {
        let cut = cut(text).unwrap();
        assert_eq!(
            (cut.path, cut.text, cut.whole),
            (Some(path), in_text, false),
            "{text}"
        );
    }
    let cut = cut(r#"{"a": 1}"#).unwrap();
    assert!(cut.whole && !cut.text);
}
