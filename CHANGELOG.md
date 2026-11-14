# Changelog

## Unreleased

- **`Schema::new` and `Schema::with_documents` take the schema by reference** (`&Value`) and keep
  nothing of it: a schema that comes with a request is compiled where it is, with no copy.
  `Schema::json` is gone; the document stays the caller's.
- **`fit(schema, Provider)`**: a schema as a provider's strict mode takes it (OpenAI, Anthropic,
  Gemini), with what the provider does not hold taken out, what it asks of every object set,
  and every change said; the answer is still checked against the schema as written.
- **`Schema::may_match(written)`**: whether a call still being written can still end in a
  match, false only where no ending mends it, so a stream is stopped at the first false.
- **`whole(text)`**: what a JSON text the output limit cut off wrote whole, the string it stops
  in closed, what is open closed, what the cut left unfinished left out.
- **`Schema::mend(&mut data)`**: what a model wrote read as it meant it where the schema leaves
  no doubt (a number, a boolean or null written as a string, an object or an array sent as a
  string of JSON), each change said.
- A schema compiles close to three times faster: every keyword of a schema object is read in one
  pass over its fields, the reference index is built only for a schema that has a `$ref`, a
  leaf that asks only for a type (`{"type": "string", "description": "…"}`, most of an agent's)
  is held with no rules allocated, a schema that says nothing (`{}`) is `true`, and `type`'s
  names and `required`'s fields are kept without a string of their own.
- Checks are faster where an agent's answers spend them. An object held to `properties`,
  `required` and `additionalProperties` alone is read in one pass over its fields in the order
  of their names, nothing looked up. A field that is a common leaf (a type alone, an `enum` of
  strings, a `const`, an integer between bounds, a string of a length, a pattern or a format) is
  settled where it stands, held or surely not, with no walk of its rules, through a `$ref` too.
  A node that is an object, an array or a union and no more is walked by a lane of its own,
  with no report and no keyword asked that it does not have. A `$ref` in a schema of draft 7 or
  earlier is followed as directly as in a later one. A union picks its branch by its tag with no
  iterator in between. A union of names (an `anyOf` or a `oneOf` of `enum`s of strings and
  string `const`s, as a list of scopes or an `enum` with titles is written) is one list a quick
  check looks its string up in. A `multipleOf` spells its divisor once, as the schema is
  compiled, and a whole number not at all. A fraction beside whole bounds is compared as the
  float it is; a node no short way is kept for is told by its type first, then by what the
  value's kind asks alone, and an `allOf`, a `not` and an `if` go through the short ways too. An `enum`'s strings, a field's name and a
  tag's value are compared inline, with no call to `memcmp`.
- Every verdict and message is unchanged: the official suite, MaskBench and the quick loop's
  fingerprint say so.
- `bench/quick/`: the loop a change is measured in first, in instructions, cycles and
  allocations, the same on every run, with a fingerprint of every verdict and message;
  `bench/profile.sh` says where the time goes. The benchmarks are built for speed
  (`opt-level = 3`), as `cargo build --release` builds; the size is still measured built for
  size. They now time a request too: a schema compiled and one answer checked.

## 0.1.0

- The first release, as its own repository: the JSON Schema validator Localix CLI wrote for
  `llx exec --schema`, with its history. Drafts 4, 6, 7, 2019-09 and 2020-12, every keyword of
  validation and every applicator, `format` asserted, ECMA-262 patterns, `$ref` into the documents
  given; `Schema::new`, `Schema::with_documents`, `matches`, `mismatches` and `unchecked`.
- The benchmarks write their results to `bench/results/` and are held to `bench/budgets.json`
  (`cargo run --release --bin check`); `bench/size.sh` measures what the library adds to a
  binary and to a build; `bench/charts.sh` draws the README's chart from the results.
