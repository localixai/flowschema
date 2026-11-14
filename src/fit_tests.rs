use serde_json::{Value, json};

use super::*;
use crate::Schema;

/// A tool's parameters with something of everything a provider may not take.
fn written() -> Value {
    json!({"type": "object", "required": ["path"],
        "properties": {
            "path": {"type": "string", "minLength": 1, "format": "uri"},
            "limit": {"type": "integer", "minimum": 1, "maximum": 100},
            "mode": {"oneOf": [{"const": "read"}, {"const": "write"}]},
            "tags": {"type": "array", "minItems": 2, "uniqueItems": true,
                "items": {"type": "string", "pattern": "^[a-z]+$"}}}})
}

#[test]
fn openai_closes_objects_requires_every_property_and_keeps_bounds() {
    let Fitted { schema, changes } = fit(&written(), Provider::OpenAi);
    assert_eq!(schema["additionalProperties"], json!(false));
    assert_eq!(schema["required"], json!(["path", "limit", "mode", "tags"]));
    assert_eq!(
        schema["properties"]["limit"],
        json!({"type": ["integer", "null"],
        "minimum": 1, "maximum": 100})
    );
    // `oneOf` is not held: sent as `anyOf`, and nullable as the property was not required.
    assert_eq!(
        schema["properties"]["mode"]["anyOf"][0]["anyOf"][1],
        json!({"const": "write"})
    );
    assert_eq!(schema["properties"]["path"], json!({"type": "string"}));
    assert_eq!(schema["properties"]["tags"]["items"]["pattern"], "^[a-z]+$");
    for said in [
        "schema.properties.path: `minLength` is not held: taken out",
        "schema.properties.path: `format` is not held: taken out",
        "schema.properties.mode: `oneOf` sent as `anyOf`",
        "schema.properties.tags: `uniqueItems` is not held: taken out",
        "schema: `additionalProperties` set to false, as the provider asks",
        "schema.properties.limit: made required and nullable, as the provider asks",
    ] {
        assert!(
            changes.iter().any(|change| change == said),
            "{said}: {changes:#?}"
        );
    }
}

#[test]
fn anthropic_takes_out_bounds_and_gemini_patterns() {
    let anthropic = fit(&written(), Provider::Anthropic).schema;
    assert_eq!(anthropic["properties"]["limit"], json!({"type": "integer"}));
    assert_eq!(
        anthropic["properties"]["path"],
        json!({"type": "string", "format": "uri"})
    );
    assert!(anthropic["properties"]["tags"].get("minItems").is_none());
    assert_eq!(anthropic["required"], json!(["path"]));
    assert_eq!(anthropic["additionalProperties"], json!(false));
    let gemini = fit(&written(), Provider::Gemini).schema;
    assert_eq!(gemini["properties"]["limit"]["maximum"], 100);
    assert_eq!(
        gemini["properties"]["tags"]["items"],
        json!({"type": "string"})
    );
    assert_eq!(
        gemini["properties"]["mode"]["anyOf"][0],
        json!({"enum": ["read"]})
    );
    assert!(gemini.get("additionalProperties").is_none());
}

#[test]
fn a_schema_that_fits_comes_back_as_it_is() {
    for provider in [Provider::OpenAi, Provider::Anthropic, Provider::Gemini] {
        let once = fit(&written(), provider);
        let twice = fit(&once.schema, provider);
        assert_eq!(twice.schema, once.schema, "{provider:?}");
        assert_eq!(twice.changes, Vec::<String>::new(), "{provider:?}");
    }
}

#[test]
fn what_fits_the_schema_as_written_fits_what_is_sent() {
    // Every field given, as a provider that requires them all has the model write them.
    let answer = json!({"path": "https://example.com/a", "limit": 5, "mode": "read",
        "tags": ["a", "b"]});
    assert!(Schema::new(&written()).matches(&answer));
    for provider in [Provider::OpenAi, Provider::Anthropic, Provider::Gemini] {
        let sent = fit(&written(), provider).schema;
        assert!(Schema::new(&sent).matches(&answer), "{provider:?}");
    }
    // What the provider does not hold is still held here: by the schema as written.
    let loose = json!({"path": "https://example.com/a", "limit": 500, "mode": "read",
        "tags": ["a", "a"]});
    assert!(Schema::new(&fit(&written(), Provider::Anthropic).schema).matches(&loose));
    assert!(!Schema::new(&written()).matches(&loose));
}
