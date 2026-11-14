# Benchmark

What it costs to check what a model wrote, and how often the check is right, measured the same
way for flowschema and for the [`jsonschema`](https://github.com/Stranger6667/jsonschema) crate,
the reference in Rust. This is what the library is held to: [`budgets.json`](budgets.json)
states the rules, `cargo run --release --bin check` fails when a run breaks one, and every run's
results are committed under [`results/`](results/).

The latest numbers: [`results/latest.md`](results/latest.md).

## What is measured

| Benchmark | What it runs | Results |
| --- | --- | --- |
| `micro` | Typical checks of an agent's output, each on fixed schemas and data: a review's verdict valid and not, a thousand rows, wide objects, a tree by `$ref`, a union of tool calls, `uniqueItems`, a long `enum`, each `format`, common patterns; the time to compile two schemas; what a request costs, its schema compiled and one answer checked (a structured answer, and a tool call shaped as a coding agent's are: a flat object of a few fields, most of them strings); the time to write every message of an invalid answer | `micro.json`, `micro.md` |
| `suite` | The official [JSON-Schema-Test-Suite](https://github.com/json-schema-org/JSON-Schema-Test-Suite): every required test of each draft from 4 to 2020-12, the format tests and ECMA-262's regular expressions, the suite's remote documents given to both | `suite.json`, `suite.md` |
| `corpus` | [MaskBench](https://github.com/guidance-ai/jsonschemabench) (guidance-ai): the schemas of real LLM tasks (function calling, GitHub's, Kubernetes', JSON Schema Store's), each with answers labelled valid or not | `corpus.json`, `corpus.md`, `corpus-disagreements.tsv`, `corpus-slower.tsv` |
| `keywords` | What MaskBench's schemas ask, by where they come from: the share of the function-calling schemas, of the Model Context Protocol's own and of the documents that say each keyword | `keywords.json`, `keywords.md` |
| `size.sh` | One small program built with no validator, with flowschema, with jsonschema and with regex-lite alone: what each adds to the binary, and the crates each adds to the build | `size.json`, `size.md` |
| `charts.sh` | The README's chart, drawn from the results above as SVG (`charts`) and rasterized by headless Chrome | `charts/compare.svg`, `charts/compare.png` |

## How

- **Built for speed.** Both validators are built as `cargo build --release` builds them
  (`opt-level = 3`), with fat LTO and one codegen unit, and `jsonschema` asserts `format`, as
  flowschema does, so both do the same work. The size is measured apart, built for size.
- **Time.** `micro` calls a check until about 0.2 ms have passed, then runs 31 rounds of that many
  calls and keeps the median. A compile's time has the copy of its schema taken off. `corpus`
  times each schema by its best of five rounds, which is steadier where thousands are each timed
  once, and compares check times only on the schemas both check whole.
- **Allocations** are counted exactly, by a global allocator that counts each call. The inputs
  are fixed, so the counts are the same on every run.
- **Verdicts.** `micro` asserts that both validators agree on every row. `suite` counts a group
  whose schema uses a keyword flowschema does not check (`Schema::unchecked`) apart, rather than
  as passed. `corpus` judges every answer against its label and the two validators against each
  other, and lists every answer they judge apart.
- **Size.** `size.sh` strips the binaries and subtracts the one with no validator; the crates
  added are those `cargo tree` lists beyond the build that only reads JSON.
- **The corpora** are fetched by `fetch.sh` at the commits the results were taken at; `run.sh`
  fetches them when they are missing.

Timings depend on the machine; `results/latest.md` says which one ran them. The ratios between the
two validators move far less than the times do.

## Run

```sh
./run.sh                                   # everything, results/ written, then held to budgets.json
(cd quick && cargo run --release)          # the quick loop: this tree against the run saved last
cargo run --release --bin micro            # one benchmark; also suite, corpus
cargo run --release --bin check            # the results in results/, held to budgets.json again
cargo run --release --bin schema -- data/jsonschemabench/maskbench/data/<file>.json
```

`schema` times one corpus file answer by answer, for a schema `corpus-slower.tsv` lists.

## The quick loop

A change to how the library checks is measured first by `quick/`, a crate of its own with the
library and `serde_json` alone, so a change rebuilds in about ten seconds and runs in about one:

```sh
cd quick && cargo run --release                     # this tree against the run saved last
cargo run --release -- --save                       # keep this run as the one to compare with
cargo run --release -- --save-as start              # or under a name: compare with --against start
../profile.sh compile agents                        # where the time goes (also check, messages)
```

It measures what `micro` does (its fixtures are `src/fixtures.rs`, shared), the compile of two
schemas, and a fixed sample of MaskBench, every eighth file of each source in name order, in two
sets: what an agent sends (function calling, tool definitions, hand-written and synthesized
answers) and the documents (GitHub's, Kubernetes', JSON Schema Store's and the rest), each
compiled, checked and said. Each row is counted, not timed:

- **Instructions retired**, as the kernel counts them for the process (`proc_pid_rusage` on
  macOS): the same work retires the same instructions on every run, so a change of a tenth of a
  percent is a change of the code, not of the machine.
- **CPU cycles**, by the same count: they pay for missed caches and branches too, and move a few
  percent from run to run. The fewest of several rounds of a fixed number of calls is kept, on
  the fast cores (QoS user-interactive).
- **Allocations**, counted exactly.
- **A fingerprint** of every verdict and message: a change that answers otherwise says so, and
  the run exits with 2.

A change is kept when it does less work, checks no less, and the fingerprint holds; the
published figures still come from `run.sh`. Elsewhere than macOS, the clock stands in for both
counters.

The count is exact for one build, not across two: a change anywhere moves what the compiler
inlines elsewhere, and a row the change does not touch can move by a percent or two. A change
smaller than that is not told from it; one that moves its own rows by more, and the rest within
it, is.

## Budgets

| Rule | Holds |
| --- | --- |
| `no-allocation` | `matches` allocates nothing, valid data or not; a `uniqueItems` keeps one set of its items' hashes |
| `never-slower` | No typical check, compile, request or message is slower than jsonschema's |
| `suite` | No test of the official suite that flowschema checks fails |
| `corpus-labels` | At least 99.95% of MaskBench's answers are judged as labelled |
| `corpus-panics` | No schema of MaskBench makes it panic |
| `corpus-speed` | On MaskBench, both built for speed, it compiles at least 3x and checks at least 1.1x faster |
| `size` | It adds at most 300 KB to a binary built for size |

A number changes only together with its reason, in the change that moves it.
