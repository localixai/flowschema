use serde_json::{Value, json};

use crate::Schema;

/// The quick walk and the one that says what is wrong agree on every value tried.
fn agree(schema: &Value, held: &[&str], refused: &[Value]) {
    let schema = Schema::new(schema);
    for name in held {
        assert!(schema.matches(&json!(name)), "{name}");
        assert!(schema.mismatches(&json!(name)).is_empty(), "{name}");
    }
    for value in refused {
        assert!(!schema.matches(value), "{value}");
        assert!(!schema.mismatches(value).is_empty(), "{value}");
    }
}

#[test]
fn a_union_of_names_holds_the_strings_it_would_branch_by_branch() {
    // A name two branches of a `oneOf` share is none of its names.
    let scopes = json!({"oneOf": [
        {"type": "string", "enum": ["read", "write"]},
        {"enum": ["write", "admin"]},
        {"const": "none"}]});
    agree(
        &scopes,
        &["read", "admin", "none"],
        &[json!("write"), json!("other"), json!(1)],
    );
    // An `anyOf` takes it; unions inside unions are followed, a `$ref` to one too.
    let nested = json!({"anyOf": [
        {"oneOf": [{"enum": ["a", "b"]}, {"enum": ["b", "c"]}]},
        {"$ref": "#/$defs/more"}],
        "$defs": {"more": {"anyOf": [{"const": "b"}, {"const": "d"}]}}});
    agree(
        &nested,
        &["a", "b", "c", "d"],
        &[json!("e"), json!(null), json!(["a"])],
    );
    // An `enum` whose values each carry a title, as a form's choices are written.
    let titled = json!({"oneOf": [{"const": "low", "title": "Low"},
        {"const": "high", "title": "High"}]});
    agree(&titled, &["low", "high"], &[json!("Low"), json!(true)]);
}

#[test]
fn a_union_that_holds_more_than_names_is_walked() {
    // Any string, a number, a `const` that is no string: each makes the union more than a
    // list, and it is told branch by branch as before.
    let open = json!({"anyOf": [{"enum": ["a"]}, {"type": "string", "minLength": 3}]});
    agree(&open, &["a", "long"], &[json!("no"), json!(1)]);
    let mixed = json!({"oneOf": [{"enum": ["a"]}, {"const": true}, {"const": 1}]});
    let mixed = Schema::new(&mixed);
    assert!(mixed.matches(&json!("a")) && mixed.matches(&json!(true)) && mixed.matches(&json!(1)));
    assert!(!mixed.matches(&json!("b")));
}

#[test]
fn what_is_wrong_is_said_of_the_branches_as_before() {
    let scopes = Schema::new(&json!({"oneOf": [{"enum": ["a", "b"]}, {"enum": ["b"]}]}));
    assert_eq!(
        scopes.mismatches(&json!("b")),
        ["data must match exactly one schema of oneOf (matches more than one)"]
    );
}
