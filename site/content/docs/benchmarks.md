# Benchmarks

flowschema is measured against the `jsonschema` crate on the checks an agent makes, on real LLM schemas and answers, and for what it adds to a program. Every figure here is the latest run's in the repository's `bench/results`, on an Apple M5 with both crates built for speed (`opt-level = 3`), `jsonschema` 0.60 with formats asserted as flowschema asserts them.

## A request

A schema that comes with a request is compiled, one answer is checked, and the schema is dropped. This is the cost of that whole path.

| | flowschema | `jsonschema` | |
| --- | --- | --- | --- |
| a tool call | 516 ns | 3.98 µs | 7.7× |
| a structured answer (a review's verdict) | 1.26 µs | 6.50 µs | 5.2× |
| nested models (`$defs`, `anyOf` with null) | 2.67 µs | 10.47 µs | 3.9× |

## Once compiled

Where the schema is known ahead and compiled once, a check allocates nothing.

| | flowschema | `jsonschema` | |
| --- | --- | --- | --- |
| a review's verdict | 98 ns | 157 ns | 1.6× |
| a union of 64 tool calls | 1.46 µs | 2.89 µs | 2.0× |

No typical check, compile, request or message of the benchmark is slower than `jsonschema`'s.

## Real LLM tasks

[MaskBench](https://github.com/guidance-ai/jsonschemabench): 11,306 schemas of function calling, tool servers and documents, with answers labelled as matching or not.

| | flowschema | `jsonschema` | |
| --- | --- | --- | --- |
| compiling every schema | 138.2 ms | 490.1 ms | 3.5× |
| checking every answer of the 9,779 schemas both check whole | 2.41 ms | 4.50 ms | 1.9× |
| answers judged as labelled | 99.98% | 99.95% | |

Over all of MaskBench flowschema is ahead; schema by schema it is not everywhere. Counted in the processor's instructions, which are the same on every run, it runs over 5% fewer than `jsonschema` on the answers of 96.2% of the schemas and over 5% more on those of 1.5%. Where it runs more, it is mostly on an answer that does not match and is wrong in more than one place: each validator stops at the first failure it meets, the two read an object's fields in different orders, and the one that meets the cheap failure first is done first.

## Weight

| | flowschema | `jsonschema` |
| --- | --- | --- |
| added to a release binary built for size | 309 KB | 1,884 KB |
| crates added to a build, itself included | 2 | 57 |
| allocations on a check | 0 | up to 7 |

Of the 309 KB, 49 KB are `regex-lite`.

## How it is measured

- **Time**: the median of 31 rounds of each check, after a warm-up; allocations are counted exactly.
- **Instructions**: where a time of a few nanoseconds is mostly noise, the processor's own count of instructions is used: it is the same on every run.
- **Budgets**: `bench/budgets.json` holds a run to what the README claims (nothing slower, nothing allocated on a check, no failure of the official suite, the size), and CI fails when the committed results break one.

The method, every table and the schemas flowschema is slower on are in the repository: [the comparison](https://github.com/localixai/flowschema/blob/main/docs/comparison.md) and [`bench/`](https://github.com/localixai/flowschema/tree/main/bench).
