# AGENTS.md

Instructions for AI coding agents working on this repository.

flowschema is JSON Schema for agents: the tool calls and the structured output a model writes,
checked against a schema given at run time, with messages a model can mend its answer from. A Rust
library with two dependencies. [CONTRIBUTING.md](CONTRIBUTING.md) is what a change has to keep,
and it binds an agent as it binds a person. This file says where things are and how to work here.

## Layout

- `src/`: the library, one idea per file. `lib.rs` is the API (`Schema`); `draft.rs` reads the
  draft, `compile.rs` and `compile/` build the nodes (`node.rs`), `index.rs` and `uri.rs` resolve
  references, `validate.rs` and `validate/` walk the data, `format.rs`, `pattern.rs`, `number.rs`
  and `hash.rs` check what their names say, `report.rs` writes the messages. Each `x.rs` has its
  tests in `x_tests.rs` beside it.
- `tests/repo.rs`: the repository's own rules, checked on its files (F3, F4, F10, F12).
- `bench/`: the benchmarks against the `jsonschema` crate, their budgets (`budgets.json`), and
  the results every published figure comes from (`results/`). A crate of its own: `jsonschema`
  and the corpora never reach the library's build. Its method: [bench/README.md](bench/README.md).
- `docs/`: the documentation, one page per file.

## Commands

```sh
scripts/check                    # every check CI runs: fmt, clippy, tests, the wasm build
cargo test --locked              # the tests; one: cargo test --locked <name>
bench/run.sh                     # every benchmark, results/ written, then held to budgets.json
cargo run --release --bin check --manifest-path bench/Cargo.toml   # the last run, held again
```

## Rules

Each rule has an id; cite it in a commit or a review that touches it. A rule marked _checked_
fails the checks when broken.

- **F1** _checked_: no panic in working code: no `unwrap`, `expect`, `panic!`, `unreachable!`,
  `todo!`, no string sliced inside a character, no number cut by `as`. What a model or a user wrote
  never crashes the program that checks it. A panic any corpus finds is a bug, fixed with a test.
- **F2** _checked_ (`bench`, `no-allocation`): `matches` allocates nothing, valid data or not;
  a `uniqueItems` keeps one set of its items' hashes.
- **F3** _checked_ (`tests/repo.rs`): at most 300 lines of working code per file and 100 per
  function; at most 7,500 in all. A change that does not fit removes code first.
- **F4** _checked_: every file opens with a `//!` that says what it is; `x.rs` sits beside its
  directory `x/`, never `x/mod.rs`; unit tests live in `x_tests.rs`, wired by
  `#[cfg(test)] #[path = "x_tests.rs"] mod tests;`; English only. Comments say why.
- **F5** _checked_: the lints in `Cargo.toml` are denied and never turned off there. An exception is
  an `expect` with its `reason`, on the smallest item.
- **F6**: two dependencies, `serde_json` and `regex-lite`. A change that needs another is
  redesigned, or the dependency is argued for in its own change.
- **F7** _checked_: no `unsafe` (`#![forbid(unsafe_code)]`).
- **F8**: nothing is checked less than it says. A keyword that is not checked is named by
  `Schema::unchecked`; a schema is never taken as stricter than it is checked, nor looser in
  silence.
- **F9**: every message names where the value is (`data.issues[0].line`), the rule it breaks and
  what the data has, so a model can mend its answer from the message alone.
- **F10** _checked_ (`tests/repo.rs`): no I/O. Nothing is fetched (a `$ref` reaches only the
  documents given), and no file, socket, process, thread, clock or environment is named in working
  code. The crate builds for `wasm32-unknown-unknown` too (`scripts/check`).
- **F11** _checked_: the crate's own enums are matched without a `_` arm.
- **F12** _checked_ (`tests/repo.rs`, lints): a compiled `Schema` is `Send + Sync` and does not
  change; `pub` is only the API another crate uses (`unreachable_pub`).
- **Conformance** _checked_ (`bench`, `suite`): every test of the official JSON-Schema-Test-Suite
  that the library checks passes, on every draft.
- **Budgets hold.** Raise a number of `bench/budgets.json` only with its reason in the change.
- **No figure is typed by hand.** A number in the README or `docs/` comes from `bench/results/`:
  change the benchmark or rerun it, then copy the figure.

## Releases

- A change a user can see adds a line under `## Unreleased` in [CHANGELOG.md](CHANGELOG.md).
- A release moves those lines under the version, which is the one in `Cargo.toml`; a tag
  `v<version>` makes the GitHub release ([release workflow](.github/workflows/release.yml)).
  Programs take flowschema as a git dependency on the tag; it is not on crates.io yet.
- Never commit, tag or push unasked.

## Before you say it is done

`scripts/check`, and `bench/run.sh` when code that checks changed. A failure names the file and
the rule: fix what it names, never the check.
