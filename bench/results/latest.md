# The latest run

2026-11-15, flowschema 7355095 with changes not yet committed, on Apple M5 (macOS 26.6.2), rustc 1.100.0, against jsonschema 0.60.
Each check's time is the median of 31 rounds; allocations are counted exactly.

## Typical checks

| check | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict, valid | 98 ns (0 alloc) | 157 ns (0 alloc) | 1.60x |
| review verdict, invalid | 16 ns (0 alloc) | 60 ns (0 alloc) | 3.80x |
| 1,000 rows | 22.23 µs (0 alloc) | 56.15 µs (0 alloc) | 2.53x |
| 500 objects × 20 fields | 23.18 µs (0 alloc) | 174.81 µs (0 alloc) | 7.54x |
| tree of 511 nodes by `$ref` | 4.46 µs (0 alloc) | 11.54 µs (7 alloc) | 2.59x |
| 64 tool calls, `oneOf` of 8 | 1.46 µs (0 alloc) | 2.89 µs (0 alloc) | 1.98x |
| `uniqueItems`, 1,000 strings | 3.24 µs (1 alloc) | 5.07 µs (1 alloc) | 1.56x |
| `enum` of 1,000, the last | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.31x |
| `format: date-time` | 7 ns (0 alloc) | 10 ns (0 alloc) | 1.51x |
| `format: email` | 22 ns (0 alloc) | 87 ns (1 alloc) | 3.94x |
| `format: uuid` | 4 ns (0 alloc) | 4 ns (0 alloc) | 1.25x |
| `format: ipv4` | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.47x |
| `format: uri` | 14 ns (0 alloc) | 22 ns (0 alloc) | 1.58x |
| `format: regex`, `^[a-z]+\d$` | 58 ns (0 alloc) | 66 ns (3 alloc) | 1.13x |
| `format: regex`, `^\p{L}+$` | 52 ns (0 alloc) | 61 ns (3 alloc) | 1.18x |
| `format: regex`, `^\P{ASCII}+$` | 57 ns (0 alloc) | 67 ns (3 alloc) | 1.17x |
| `format: regex`, `[\P{Letter}]` | 49 ns (0 alloc) | 67 ns (3 alloc) | 1.36x |
| `format: regex`, `(?=a)b` | 56 ns (0 alloc) | 88 ns (4 alloc) | 1.57x |
| `format: regex`, `(a)\1` | 61 ns (0 alloc) | 88 ns (4 alloc) | 1.45x |
| `format: regex`, `(?<n>a)\k<n>` | 79 ns (0 alloc) | 137 ns (7 alloc) | 1.73x |
| `pattern: ^[A-Z]{3}$` | 6 ns (0 alloc) | 15 ns (0 alloc) | 2.68x |
| `pattern: ^\d{4}-\d{2}-\d{2}$` | 8 ns (0 alloc) | 17 ns (0 alloc) | 2.22x |
| `pattern: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$` | 9 ns (0 alloc) | 19 ns (0 alloc) | 2.01x |
| `pattern: ^(small\|medium\|large)$` | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.12x |

| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict | 1.21 µs | 6.76 µs | 5.61x |
| tool calls | 6.27 µs | 27.17 µs | 4.33x |

| one request | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict: compiled, one answer checked | 1.26 µs | 6.50 µs | 5.18x |
| one tool call: compiled, one answer checked | 516 ns | 3.98 µs | 7.71x |
| nested models: compiled, one answer checked | 2.67 µs | 10.47 µs | 3.92x |

Every message of an invalid verdict: flowschema 609 ns, jsonschema 758 ns (1.25x).

## The official test suite

| draft | flowschema | jsonschema 0.60 | tests left out (a keyword not checked here) |
|---|---|---|---|
| draft4 | 928/928 | 928/928 | 20 tests |
| draft6 | 1256/1256 | 1256/1256 | 20 tests |
| draft7 | 1584/1584 | 1584/1584 | 145 tests |
| draft2019-09 | 1944/1944 | 1944/1944 | 185 tests |
| draft2020-12 | 1963/1963 | 1963/1963 | 188 tests |

Tests left out, by what they ask that is not checked: `format` (375), `pattern` (76), `$dynamicRef across resources` (73), `$ref outside the schema` (34).

Failures: 0.

## MaskBench

| | flowschema | jsonschema 0.60 |
|---|---|---|
| schemas compiled | 11,306 of 11,306 (0 panics) | 11,298 (8 refused) |
| answers judged as labelled | 36,984 of 36,990 (99.98%) | 36,970 (99.95%) |
| compiling every schema | 138.2 ms | 490.1 ms (flowschema 3.55x faster) |
| checking every answer of the 9,779 schemas both check whole | 2.41 ms | 4.50 ms (flowschema 1.87x faster) |

The two agree on 99.93% of the answers (`corpus-disagreements.tsv`). 21 schemas (0.19%) ask something not checked here: `pattern` (19), `$ref outside the schema` (1), `format` (1). flowschema is slower by over 5% on 132 schemas (`corpus-slower.tsv`).

## Size

| added to a release binary built for size | size |
|---|---|
| flowschema, all of it | 309 KB |
| of which its own code | 260 KB |
| of which regex-lite | 49 KB |
| jsonschema 0.60 | 1,884 KB |

| crates added to the build, itself included | crates |
|---|---|
| flowschema | 2 |
| jsonschema 0.60 | 57 |

Over a program that reads the JSON alone (312 KB), stripped, `opt-level = "z"`, fat LTO. regex-lite is measured alone, as a program that uses it pays for it.
