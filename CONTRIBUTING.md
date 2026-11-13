# Contributing

```sh
scripts/check        # formatting, lints, tests, the WebAssembly build
bench/run.sh         # the benchmarks, then the run held to bench/budgets.json
```

## What a change has to keep

- **Correct on the official suite.** Every test of the
  [JSON-Schema-Test-Suite](https://github.com/json-schema-org/JSON-Schema-Test-Suite) that the
  library checks passes, on every draft from 4 to 2020-12 (`bench`, `suite`). A keyword it does not
  check is named by `Schema::unchecked`, never taken in silence.
- **No allocation on a check.** `matches` allocates nothing, valid data or not; the benchmarks
  count every allocation exactly and fail a run that makes one (`no-allocation`).
- **Never slower than the reference.** No typical check, compile or message is slower than the
  `jsonschema` crate's (`never-slower`), and MaskBench's real schemas compile and check faster
  (`corpus-speed`). The budgets are in [bench/budgets.json](bench/budgets.json).
- **Small.** It adds at most 300 KB to a binary built for size (`size`), with two dependencies.
  At most 300 lines of working code per file and 7,500 in all (`tests/repo.rs`).
- **Messages a model can act on.** Each names where the value is, the rule and what the data has.
- **No panic, no `unsafe`, no I/O.** The lints in `Cargo.toml` hold the first, the crate's
  attribute the second, and `tests/repo.rs` the third: no working file names a file, a socket, a
  process, a thread, the clock or the environment.

The rules, with their ids, are in [AGENTS.md](AGENTS.md).

## Figures in the documentation

No number in the README or in [docs/](docs) is typed by hand: each comes from
[bench/results](bench/results), which `bench/run.sh` writes. Change the benchmark or rerun it,
then copy the figure. The method is in [bench/README.md](bench/README.md).

## Releases

A change a user can see adds a line under `## Unreleased` in [CHANGELOG.md](CHANGELOG.md). A tag
`v<version>`, the version in `Cargo.toml`, makes the release
([release workflow](.github/workflows/release.yml)).
