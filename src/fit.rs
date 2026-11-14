//! A schema fitted to a provider's strict mode. Each provider holds a model to its own part of
//! JSON Schema and refuses or ignores the rest; `fit` gives the schema to send (what the
//! provider does not take, taken out; what it asks of every object, set) and says each change,
//! so the program knows what the provider will not hold. The answer is still checked against the
//! schema as it was written (`Schema`): that is where what was taken out is held.
//!
//! The rules are the providers' documentation as it read in November 2026, and move with it.

use serde_json::{Map, Value, json};

/// A provider whose strict mode holds a model to part of JSON Schema.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Provider {
    /// OpenAI's structured outputs and strict function calling.
    OpenAi,
    /// Anthropic's structured outputs and strict tool use.
    Anthropic,
    /// Gemini's structured output.
    Gemini,
}

/// A schema as a provider's strict mode takes it.
#[derive(Clone, Debug, PartialEq)]
pub struct Fitted {
    /// The schema to send.
    pub schema: Value,
    /// Every change made to fit it, each with where in the schema (`schema.properties.age`):
    /// what the provider will not hold, and what it asked to be set.
    pub changes: Vec<String>,
}

/// What a provider's strict mode takes.
struct Rules {
    provider: Provider,
    /// The keywords it refuses or ignores, beside those none takes (`COMMON`): taken out.
    dropped: &'static [&'static str],
    /// The formats it holds: any other is taken out.
    formats: &'static [&'static str],
}

impl Rules {
    /// Whether every object must refuse fields it does not name.
    fn closed(&self) -> bool {
        matches!(self.provider, Provider::OpenAi | Provider::Anthropic)
    }

    /// Whether every property must be required: one that was not may be null instead.
    fn all_required(&self) -> bool {
        self.provider == Provider::OpenAi
    }

    /// Whether `minItems` is held only up to one.
    fn small_min_items(&self) -> bool {
        self.provider == Provider::Anthropic
    }

    /// Whether `const` is sent as an `enum` of one value.
    fn const_as_enum(&self) -> bool {
        self.provider == Provider::Gemini
    }
}

/// Keywords none of the three holds.
const COMMON: [&str; 16] = [
    "not",
    "if",
    "then",
    "else",
    "dependentRequired",
    "dependentSchemas",
    "dependencies",
    "minLength",
    "maxLength",
    "uniqueItems",
    "contains",
    "minContains",
    "maxContains",
    "patternProperties",
    "propertyNames",
    "minProperties",
];

impl Provider {
    fn rules(self) -> Rules {
        match self {
            Provider::OpenAi => Rules {
                provider: self,
                dropped: &[
                    "allOf",
                    "maxProperties",
                    "unevaluatedProperties",
                    "unevaluatedItems",
                    "prefixItems",
                ],
                formats: &[
                    "date-time",
                    "time",
                    "date",
                    "duration",
                    "email",
                    "hostname",
                    "ipv4",
                    "ipv6",
                    "uuid",
                ],
            },
            Provider::Anthropic => Rules {
                provider: self,
                dropped: &[
                    "minimum",
                    "maximum",
                    "exclusiveMinimum",
                    "exclusiveMaximum",
                    "multipleOf",
                    "maxItems",
                    "maxProperties",
                    "unevaluatedProperties",
                    "unevaluatedItems",
                    "prefixItems",
                ],
                formats: &[
                    "date-time",
                    "time",
                    "date",
                    "duration",
                    "email",
                    "hostname",
                    "uri",
                    "ipv4",
                    "ipv6",
                    "uuid",
                ],
            },
            Provider::Gemini => Rules {
                provider: self,
                dropped: &[
                    "allOf",
                    "exclusiveMinimum",
                    "exclusiveMaximum",
                    "multipleOf",
                    "pattern",
                    "maxProperties",
                    "unevaluatedProperties",
                    "unevaluatedItems",
                ],
                formats: &["date-time", "date", "time"],
            },
        }
    }
}

/// `schema` as `provider`'s strict mode takes it, with every change made to fit it. A schema
/// that already fits comes back as it is, with no change.
pub fn fit(schema: &Value, provider: Provider) -> Fitted {
    let mut changes = vec![];
    let schema = fitted(schema, &provider.rules(), "schema", &mut changes);
    Fitted { schema, changes }
}

/// The schema at `at`, fitted.
fn fitted(schema: &Value, rules: &Rules, at: &str, changes: &mut Vec<String>) -> Value {
    let Value::Object(said) = schema else {
        return schema.clone();
    };
    let mut out = Map::new();
    for (key, value) in said {
        let kept = match key.as_str() {
            _ if rules.dropped.contains(&key.as_str()) || COMMON.contains(&key.as_str()) => None,
            "format"
                if !value
                    .as_str()
                    .is_some_and(|name| rules.formats.contains(&name)) =>
            {
                None
            }
            "minItems" if rules.small_min_items() && value.as_u64().is_some_and(|n| n > 1) => None,
            "properties" | "$defs" | "definitions" => Some(each(value, rules, at, key, changes)),
            "items" | "additionalProperties" => {
                Some(fitted(value, rules, &format!("{at}.{key}"), changes))
            }
            "anyOf" | "oneOf" | "allOf" | "prefixItems" => {
                Some(list(value, rules, at, key, changes))
            }
            _ => Some(value.clone()),
        };
        match (key.as_str(), kept) {
            (_, None) => changes.push(format!("{at}: `{key}` is not held: taken out")),
            ("oneOf", Some(branches)) if !said.contains_key("anyOf") => {
                changes.push(format!("{at}: `oneOf` sent as `anyOf`"));
                out.insert("anyOf".into(), branches);
            }
            ("oneOf", Some(_)) => {
                changes.push(format!(
                    "{at}: `oneOf` beside `anyOf` is not held: taken out"
                ));
            }
            ("const", Some(only)) if rules.const_as_enum() => {
                changes.push(format!("{at}: `const` sent as an `enum` of one value"));
                out.insert("enum".into(), json!([only]));
            }
            (_, Some(kept)) => {
                out.insert(key.clone(), kept);
            }
        }
    }
    if is_object(&out) {
        close(&mut out, rules, at, changes);
        require(&mut out, rules, at, changes);
    }
    Value::Object(out)
}

/// Each schema of a map of them (`properties`, `$defs`), fitted.
fn each(map: &Value, rules: &Rules, at: &str, key: &str, changes: &mut Vec<String>) -> Value {
    let Value::Object(named) = map else {
        return map.clone();
    };
    let fit = |(name, schema): (&String, &Value)| {
        let inner = fitted(schema, rules, &format!("{at}.{key}.{name}"), changes);
        (name.clone(), inner)
    };
    Value::Object(named.iter().map(fit).collect())
}

/// Each schema of a list of them (`anyOf`), fitted.
fn list(list: &Value, rules: &Rules, at: &str, key: &str, changes: &mut Vec<String>) -> Value {
    let Value::Array(schemas) = list else {
        return list.clone();
    };
    let fit = |(place, schema): (usize, &Value)| {
        fitted(schema, rules, &format!("{at}.{key}[{place}]"), changes)
    };
    Value::Array(schemas.iter().enumerate().map(fit).collect())
}

/// Whether the keywords are an object's: it says `properties`, or its `type` is or lists
/// `object`.
fn is_object(said: &Map<String, Value>) -> bool {
    let typed = match said.get("type") {
        Some(Value::String(name)) => name == "object",
        Some(Value::Array(names)) => names.iter().any(|name| name == "object"),
        Some(_) | None => false,
    };
    typed || said.contains_key("properties")
}

/// Where the provider asks every object to refuse fields it does not name: so set.
fn close(out: &mut Map<String, Value>, rules: &Rules, at: &str, changes: &mut Vec<String>) {
    if !rules.closed() || out.get("additionalProperties") == Some(&Value::Bool(false)) {
        return;
    }
    let was = match out.get("additionalProperties") {
        None | Some(Value::Bool(_)) => "set to false, as the provider asks",
        Some(_) => "was a schema, which is not held: set to false",
    };
    changes.push(format!("{at}: `additionalProperties` {was}"));
    out.insert("additionalProperties".into(), Value::Bool(false));
}

/// Where the provider asks every property to be required: one that was not is, and may be null.
fn require(out: &mut Map<String, Value>, rules: &Rules, at: &str, changes: &mut Vec<String>) {
    let names: Vec<String> = match (rules.all_required(), out.get("properties")) {
        (true, Some(Value::Object(properties))) => properties.keys().cloned().collect(),
        (true | false, Some(_) | None) => return,
    };
    let mut required: Vec<Value> = match out.get("required") {
        Some(Value::Array(listed)) => listed.clone(),
        Some(_) | None => vec![],
    };
    for name in names {
        if required.iter().any(|listed| *listed == name.as_str()) {
            continue;
        }
        changes.push(format!(
            "{at}.properties.{name}: made required and nullable, as the provider asks"
        ));
        if let Some(Value::Object(properties)) = out.get_mut("properties")
            && let Some(property) = properties.get_mut(&name)
        {
            *property = nullable(property.take());
        }
        required.push(Value::String(name));
    }
    out.insert("required".into(), Value::Array(required));
}

/// `schema`, or null: `null` added to its `type` where it says one, else an `anyOf` with it.
fn nullable(schema: Value) -> Value {
    let wrapped = |schema: Value| json!({"anyOf": [schema, {"type": "null"}]});
    let Value::Object(mut said) = schema else {
        return wrapped(schema);
    };
    match said.get("type").cloned() {
        Some(Value::String(name)) if name != "null" => {
            said.insert("type".into(), json!([name, "null"]));
        }
        Some(Value::String(_)) => {}
        Some(Value::Array(mut names)) => {
            if !names.iter().any(|name| name == "null") {
                names.push(json!("null"));
            }
            said.insert("type".into(), Value::Array(names));
        }
        Some(_) | None => return wrapped(Value::Object(said)),
    }
    // An `enum` beside the type must allow null too.
    if let Some(Value::Array(options)) = said.get_mut("enum")
        && !options.contains(&Value::Null)
    {
        options.push(Value::Null);
    }
    Value::Object(said)
}

#[cfg(test)]
#[path = "fit_tests.rs"]
mod tests;
