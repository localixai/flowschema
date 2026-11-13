use serde_json::{Value, json};

use super::*;

/// The keywords of `schema` as `draft` reads them.
fn read(draft: Draft, schema: &Value) -> Keywords<'_> {
    Keywords::of(schema.as_object().unwrap(), draft)
}

#[test]
fn a_draft_reads_only_the_keywords_it_knows() {
    let schema = json!({
        "if": true, "dependencies": {}, "prefixItems": [], "properties": {}, "const": 1,
        "$dynamicRef": "#a", "$recursiveRef": "#", "unevaluatedItems": false,
    });
    let four = read(Draft::Four, &schema);
    assert!(four.when.is_none() && four.constant.is_none() && four.properties.is_some());
    let seven = read(Draft::Seven, &schema);
    assert!(seven.when.is_some() && seven.dependencies.is_some());
    assert!(seven.prefix_items.is_none() && seven.unevaluated_items.is_none());
    let nineteen = read(Draft::Nineteen, &schema);
    assert!(nineteen.prefix_items.is_none() && nineteen.dependencies.is_none());
    assert_eq!(nineteen.dynamic, Some(&json!("#")));
    let twenty = read(Draft::Twenty, &schema);
    assert!(twenty.prefix_items.is_some() && twenty.unevaluated_items.is_some());
    assert_eq!(twenty.dynamic, Some(&json!("#a")));
}

#[test]
fn draft_4_names_a_resource_by_id_and_the_others_by_dollar_id() {
    let schema = json!({"id": "a", "$id": "b"});
    assert_eq!(read(Draft::Four, &schema).id, Some(&json!("a")));
    assert_eq!(read(Draft::Six, &schema).id, Some(&json!("b")));
}

#[test]
fn additional_items_is_kept_in_every_draft_but_counts_for_arrays_until_2019() {
    let schema = json!({"additionalItems": false});
    let twenty = read(Draft::Twenty, &schema);
    assert!(twenty.additional_items.is_some());
    assert!(!twenty.of_arrays(Draft::Twenty));
    assert!(read(Draft::Seven, &schema).of_arrays(Draft::Seven));
}
