# Changelog

## Unreleased

- A schema compiles close to three times faster: every keyword of a schema object is read in one
  pass over its fields, the reference index is built only for a schema that has a `$ref`, a
  leaf that asks only for a type (`{"type": "string", "description": "…"}`, most of an agent's)
  is held with no rules allocated, and `type`'s names and `required`'s fields are kept without a
  string of their own. Every verdict and message is unchanged.
- Checks are faster where an agent's answers spend them: a field that is a common leaf (a type
  alone, an `enum` of strings, an integer between bounds, a string of a length) is settled where
  it stands, with no walk of its rules; an `enum`'s strings, a field's name and a tag's value are
  compared inline, with no call to `memcmp`; a field of a small data object is found by walking
  it.
- `bench/quick/`: the loop a change is measured in first, in instructions, cycles and
  allocations, the same on every run, with a fingerprint of every verdict and message;
  `bench/profile.sh` says where the time goes. The benchmarks are built for speed
  (`opt-level = 3`), as `cargo build --release` builds; the size is still measured built for size.

## 0.1.0

- The first release, as its own repository: the JSON Schema validator Localix CLI wrote for
  `llx exec --schema`, with its history. Drafts 4, 6, 7, 2019-09 and 2020-12, every keyword of
  validation and every applicator, `format` asserted, ECMA-262 patterns, `$ref` into the documents
  given; `Schema::new`, `Schema::with_documents`, `matches`, `mismatches` and `unchecked`.
- The benchmarks write their results to `bench/results/` and are held to `bench/budgets.json`
  (`cargo run --release --bin check`); `bench/size.sh` measures what the library adds to a
  binary and to a build; `bench/charts.sh` draws the README's chart from the results.
