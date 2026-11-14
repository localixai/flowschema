# Cut-off and streaming calls

What a model writes does not always arrive whole or as the schema spells it: a reply stops at the output limit, a call's arguments come in pieces, a number comes as a string. This page covers the three.

## A call the output limit cut off

A reply that reaches the output limit stops midway, and so do the arguments of the call it was writing. `whole` gives what was written whole: the string it stops in closed, every object and array still open closed, and what the cut left unfinished left out (a name with no value, a number that may go on, half an escape).

```rust
use flowschema::whole;
use serde_json::json;

let cut = r#"{"path": "src/lib.rs", "lines": [1, 2"#;
assert_eq!(whole(cut), Some(json!({"path": "src/lib.rs", "lines": [1]})));

let cut = r#"{"path": "src/lib.rs", "content": "fn main() {"#;
assert_eq!(whole(cut), Some(json!({"path": "src/lib.rs", "content": "fn main() {"})));
```

What it gives is a value like any other: check it against the schema to learn which fields came whole and which are still missing.

## A call as it streams

A call's arguments arrive in pieces. `may_match` says, of the text so far, whether it can still end in a match: false only where no ending mends it, so a stream can be stopped at the first false and the model told why, without waiting for the rest.

```rust
use flowschema::Schema;
use serde_json::json;

let tool = Schema::new(&json!({"type": "object", "required": ["path"],
    "additionalProperties": false,
    "properties": {"path": {"type": "string"}, "mode": {"enum": ["read", "write"]}}}));

// Still open: the path may yet come, and "re" may become "read".
assert!(tool.may_match(r#"{"mode": "re"#));
// No ending mends these: no option begins so, and the schema refuses the field.
assert!(!tool.may_match(r#"{"mode": "x"#));
assert!(!tool.may_match(r#"{"path": "a", "force": true,"#));
```

It asks only what more text cannot change: the kind of a value, a field the schema refuses, a value written whole that is not what its place asks, a string no option begins with, a list already too long. What an ending may mend (a required field still to come) and what only the whole value tells (a union, a `not`, a pattern over a field's name) are left to `matches`: the answer errs towards yes, and every beginning of a call that matches is let through.

## What a model wrote, read as it meant it

A model writes `"30"` where the schema asks for an integer, or sends an object as a string of JSON. Where the schema leaves no doubt, `mend` makes the value what it was meant to be and says so; where a string is allowed, or the schema is a union, nothing is guessed.

```rust
use flowschema::Schema;
use serde_json::json;

let schema = Schema::new(&json!({"type": "object", "properties": {
    "limit": {"type": "integer"},
    "paths": {"type": "array", "items": {"type": "string"}}}}));
let mut call = json!({"limit": "30", "paths": "[\"src\", \"tests\"]"});
assert!(!schema.matches(&call));

let said = schema.mend(&mut call);
assert_eq!(call, json!({"limit": 30, "paths": ["src", "tests"]}));
assert_eq!(said[0], "data.limit: \"30\" read as 30");
assert!(schema.matches(&call));
```

Check the value after: `mend` changes only what the schema makes certain, and says each change with where it was made.
