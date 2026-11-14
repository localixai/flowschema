use super::*;

/// `names` as `properties` in order, each its place as its node and its bit.
fn named(names: &[&str]) -> Vec<Property> {
    (names.iter().zip(0..))
        .map(|(name, id)| Property {
            name: (*name).into(),
            id,
            bit: 1 << id,
            leaf: false,
            types: 0,
        })
        .collect()
}

/// A property as its node and its bit.
fn found(property: Option<&Property>) -> Option<(usize, u64)> {
    property.map(|p| (p.id, p.bit))
}

#[test]
fn a_field_out_of_order_is_still_found() {
    let properties = named(&["a", "b", "c", "d"]);
    let mut cursor = Cursor::new(&properties);
    // In order: each where the last was left.
    assert_eq!(found(cursor.find("a")), Some((0, 1)));
    assert_eq!(found(cursor.find("c")), Some((2, 4)));
    // Out of order: behind the cursor, and a name it never meets.
    assert_eq!(found(cursor.find("b")), Some((1, 2)));
    assert_eq!(found(cursor.find("zz")), None);
    assert_eq!(found(cursor.find("d")), Some((3, 8)));
    assert_eq!(found(cursor.find("0")), None);
}

#[test]
fn a_required_field_gone_past_is_confirmed_missing_by_one_lookup() {
    let properties = named(&["a", "b", "c"]);
    let mut cursor = Cursor::new(&properties);
    assert_eq!(found(cursor.find("c")), Some((2, 4)));
    assert_eq!(cursor.passed, 0b011, "a and b went past unmet");
    let fields = |names: &[&str]| -> Map<String, Value> {
        names
            .iter()
            .map(|name| ((*name).to_string(), Value::Null))
            .collect()
    };
    // "a" came out of order: it is there; "b" is not.
    assert!(cursor.missing(&fields(&["c", "a"]), 0));
    assert_eq!(cursor.passed, 0, "what was asked is not asked again");
    cursor.passed = 0b011;
    assert!(!cursor.missing(&fields(&["a", "b", "c"]), 0));
}
