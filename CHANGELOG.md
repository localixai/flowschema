# Changelog

## Unreleased

## 0.1.0

- The first release, as its own repository: the JSON Schema validator Localix CLI wrote for
  `llx exec --schema`, with its history. Drafts 4, 6, 7, 2019-09 and 2020-12, every keyword of
  validation and every applicator, `format` asserted, ECMA-262 patterns, `$ref` into the documents
  given; `Schema::new`, `Schema::with_documents`, `matches`, `mismatches` and `unchecked`.
- The benchmarks write their results to `bench/results/` and are held to `bench/budgets.json`
  (`cargo run --release --bin check`); `bench/size.sh` measures what the library adds to a
  binary and to a build; `bench/charts.sh` draws the README's chart from the results.
