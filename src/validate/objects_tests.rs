use serde_json::{Map, Value, json};

use crate::Schema;

/// Every subset of `names`, as lists.
fn subsets<'n>(names: &[&'n str]) -> Vec<Vec<&'n str>> {
    (0..1_u32 << names.len())
        .map(|mask| {
            (names.iter().enumerate())
                .filter(|(at, _)| mask & (1 << at) != 0)
                .map(|(_, name)| *name)
                .collect()
        })
        .collect()
}

#[test]
fn the_walk_in_order_judges_as_the_walk_by_cursor_does() {
    // Names before, between and after the properties, one of them sharing a property's start.
    let fields = ["a", "b", "bb", "c", "d", "e"];
    let properties = json!({"b": {"type": "integer"}, "c": {"enum": ["x", "y"]},
        "d": {"type": "object", "required": ["k"]}});
    let extras = [
        None,
        Some(json!(true)),
        Some(json!(false)),
        Some(json!({"type": "string"})),
    ];
    let values = |name: &str, odd: bool| match (name, odd) {
        ("b", false) => json!(1),
        ("b", true) => json!("one"),
        ("c", false) => json!("x"),
        ("c", true) => json!("z"),
        ("d", false) => json!({"k": 1}),
        ("d", true) => json!({}),
        (_, false) => json!("text"),
        (_, true) => json!(7),
    };
    let mut judged = 0;
    for required in subsets(&["b", "c", "d", "e"]) {
        for extra in &extras {
            let mut schema = json!({"properties": properties, "required": required});
            if let (Some(extra), Some(rules)) = (extra, schema.as_object_mut()) {
                rules.insert("additionalProperties".into(), extra.clone());
            }
            // The same schema with a count that asks nothing: walked by the cursor, not in order.
            let mut by_cursor = schema.clone();
            if let Some(rules) = by_cursor.as_object_mut() {
                rules.insert("minProperties".into(), json!(0));
            }
            let (quick, slow) = (Schema::new(&schema), Schema::new(&by_cursor));
            for present in subsets(&fields) {
                for odd in 0..1_u32 << present.len().min(3) {
                    let data: Map<String, Value> = (present.iter().enumerate())
                        .map(|(at, name)| ((*name).to_string(), values(name, odd & (1 << at) != 0)))
                        .collect();
                    let data = Value::Object(data);
                    assert_eq!(quick.matches(&data), slow.matches(&data), "{data} {schema}");
                    assert_eq!(quick.matches(&data), quick.mismatches(&data).is_empty());
                    judged += 1;
                }
            }
        }
    }
    assert!(judged > 10_000, "{judged}");
}
