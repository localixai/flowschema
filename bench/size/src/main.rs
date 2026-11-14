//! Reads a schema and data as one JSON document on stdin and says whether the data matches: with
//! the validator its feature names, or with none, the baseline that only reads the JSON. With
//! `regex-lite`, the schema's `pattern` alone is matched against the data, so that crate's share
//! of flowschema's size is measured as a program that uses it pays for it.

use std::io::Read;

fn main() {
    let mut text = String::new();
    let _ = std::io::stdin().read_to_string(&mut text);
    let input: serde_json::Value = serde_json::from_str(&text).unwrap_or_default();
    let (schema, data) = (&input["schema"], &input["data"]);
    #[cfg(feature = "flowschema")]
    let matches = flowschema::Schema::new(&schema.clone()).matches(data);
    #[cfg(feature = "jsonschema")]
    let matches = jsonschema::options()
        .should_validate_formats(true)
        .build(schema)
        .is_ok_and(|validator| validator.is_valid(data));
    #[cfg(feature = "regex-lite")]
    let matches = (schema["pattern"].as_str())
        .and_then(|pattern| regex_lite::Regex::new(pattern).ok())
        .is_some_and(|regex| regex.is_match(data.as_str().unwrap_or("")));
    #[cfg(not(any(feature = "flowschema", feature = "jsonschema", feature = "regex-lite")))]
    let matches = schema.is_object() && !data.is_null();
    println!("{matches}");
}
