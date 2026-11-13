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
| `micro` | Typical checks of an agent's output, each on fixed schemas and data: a review's verdict valid and not, a thousand rows, wide objects, a tree by `$ref`, a union of tool calls, `uniqueItems`, a long `enum`, each `format`, common patterns; the time to compile two schemas; the time to write every message of an invalid answer | `micro.json`, `micro.md` |
| `suite` | The official [JSON-Schema-Test-Suite](https://github.com/json-schema-org/JSON-Schema-Test-Suite): every required test of each draft from 4 to 2020-12, the format tests and ECMA-262's regular expressions, the suite's remote documents given to both | `suite.json`, `suite.md` |
| `corpus` | [MaskBench](https://github.com/guidance-ai/jsonschemabench) (guidance-ai): the schemas of real LLM tasks (function calling, GitHub's, Kubernetes', JSON Schema Store's), each with answers labelled valid or not | `corpus.json`, `corpus.md`, `corpus-disagreements.tsv`, `corpus-slower.tsv` |
| `size.sh` | One small program built three times, with no validator, with flowschema and with jsonschema: what each adds | `size.json`, `size.md` |

## How

- **Built as a CLI ships.** Both validators are built with `opt-level = "z"`, fat LTO and one
  codegen unit, and `jsonschema` asserts `format`, as flowschema does, so both do the same work.
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
- **Size.** `size.sh` strips the three binaries and subtracts the one with no validator.
- **The corpora** are fetched by `fetch.sh` at the commits the results were taken at; `run.sh`
  fetches them when they are missing.

Timings depend on the machine; `results/latest.md` says which one ran them. The ratios between the
two validators move far less than the times do.

## Run

```sh
./run.sh                                   # everything, results/ written, then held to budgets.json
cargo run --release --bin micro            # one benchmark; also suite, corpus
cargo run --release --bin check            # the results in results/, held to budgets.json again
cargo run --release --bin schema -- data/jsonschemabench/maskbench/data/<file>.json
```

`schema` times one corpus file answer by answer, for a schema `corpus-slower.tsv` lists.

## Budgets

| Rule | Holds |
| --- | --- |
| `no-allocation` | `matches` allocates nothing, valid data or not; a `uniqueItems` keeps one set of its items' hashes |
| `never-slower` | No typical check, compile or message is slower than jsonschema's |
| `suite` | No test of the official suite that flowschema checks fails |
| `corpus-labels` | At least 99.95% of MaskBench's answers are judged as labelled |
| `corpus-panics` | No schema of MaskBench makes it panic |
| `corpus-speed` | On MaskBench it compiles at least 1.5x and checks at least 1.2x faster |
| `size` | It adds at most 300 KB to a binary built for size |

A number changes only together with its reason, in the change that moves it.
