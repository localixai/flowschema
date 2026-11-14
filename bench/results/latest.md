# The latest run

2026-11-14, flowschema d5bd463 with changes not yet committed, on Apple M5 (macOS 26.6.2), rustc 1.100.0, against jsonschema 0.60.
Each check's time is the median of 31 rounds; allocations are counted exactly.

## Typical checks

| check | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict, valid | 110 ns (0 alloc) | 165 ns (0 alloc) | 1.50x |
| review verdict, invalid | 16 ns (0 alloc) | 36 ns (0 alloc) | 2.26x |
| 1,000 rows | 24.32 µs (0 alloc) | 43.92 µs (0 alloc) | 1.81x |
| 500 objects × 20 fields | 20.96 µs (0 alloc) | 169.77 µs (0 alloc) | 8.10x |
| tree of 511 nodes by `$ref` | 4.69 µs (0 alloc) | 10.04 µs (7 alloc) | 2.14x |
| 64 tool calls, `oneOf` of 8 | 1.66 µs (0 alloc) | 2.70 µs (0 alloc) | 1.63x |
| `uniqueItems`, 1,000 strings | 2.63 µs (1 alloc) | 4.55 µs (1 alloc) | 1.73x |
| `enum` of 1,000, the last | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.32x |
| `format: date-time` | 6 ns (0 alloc) | 10 ns (0 alloc) | 1.62x |
| `format: email` | 20 ns (0 alloc) | 79 ns (1 alloc) | 3.91x |
| `format: uuid` | 3 ns (0 alloc) | 4 ns (0 alloc) | 1.33x |
| `format: ipv4` | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.47x |
| `format: uri` | 13 ns (0 alloc) | 20 ns (0 alloc) | 1.52x |
| `format: regex`, `^[a-z]+\d$` | 52 ns (0 alloc) | 60 ns (3 alloc) | 1.16x |
| `format: regex`, `^\p{L}+$` | 47 ns (0 alloc) | 53 ns (3 alloc) | 1.14x |
| `format: regex`, `^\P{ASCII}+$` | 52 ns (0 alloc) | 62 ns (3 alloc) | 1.21x |
| `format: regex`, `[\P{Letter}]` | 45 ns (0 alloc) | 75 ns (3 alloc) | 1.68x |
| `format: regex`, `(?=a)b` | 51 ns (0 alloc) | 81 ns (4 alloc) | 1.57x |
| `format: regex`, `(a)\1` | 56 ns (0 alloc) | 82 ns (4 alloc) | 1.47x |
| `format: regex`, `(?<n>a)\k<n>` | 71 ns (0 alloc) | 127 ns (7 alloc) | 1.78x |
| `pattern: ^[A-Z]{3}$` | 5 ns (0 alloc) | 15 ns (0 alloc) | 2.77x |
| `pattern: ^\d{4}-\d{2}-\d{2}$` | 10 ns (0 alloc) | 17 ns (0 alloc) | 1.75x |
| `pattern: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$` | 10 ns (0 alloc) | 19 ns (0 alloc) | 1.90x |
| `pattern: ^(small\|medium\|large)$` | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.27x |

| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict | 1.17 µs | 6.51 µs | 5.57x |
| tool calls | 6.20 µs | 27.22 µs | 4.39x |

| one request | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict: compiled, one answer checked | 1.24 µs | 6.43 µs | 5.18x |
| one tool call: compiled, one answer checked | 518 ns | 3.95 µs | 7.62x |
| nested models: compiled, one answer checked | 2.50 µs | 10.68 µs | 4.27x |

Every message of an invalid verdict: flowschema 614 ns, jsonschema 831 ns (1.35x).

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
| compiling every schema | 125.0 ms | 461.4 ms (flowschema 3.69x faster) |
| checking every answer of the 9,779 schemas both check whole | 2.59 ms | 4.23 ms (flowschema 1.63x faster) |

The two agree on 99.93% of the answers (`corpus-disagreements.tsv`). 21 schemas (0.19%) ask something not checked here: `pattern` (19), `$ref outside the schema` (1), `format` (1). flowschema is slower by over 5% on 306 schemas (`corpus-slower.tsv`).

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
