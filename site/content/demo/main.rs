use flowschema::{Schema, whole};
use serde_json::json;

fn main() {
    // A tool's parameters, as the request brings them.
    let read = Schema::new(&json!({
        "type": "object",
        "required": ["path"],
        "additionalProperties": false,
        "properties": {
            "path": {"type": "string"},
            "limit": {"type": "integer", "minimum": 1}
        }
    }));

    // The call the model wrote: what to send back for it to fix.
    let call = json!({"path": 7, "limit": 0, "force": true});
    for message in read.mismatches(&call) {
        println!("{message}");
    }

    // A call still streaming: can it still match? One cut off: what came whole?
    println!("{}", read.may_match(r#"{"path": "src/lib.rs", "lim"#));
    println!("{}", whole(r#"{"path": "src/lib.rs", "limit": 2"#).unwrap_or_default());
}
