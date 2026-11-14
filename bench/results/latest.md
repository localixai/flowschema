# The latest run

2026-11-14, flowschema f11f666 with changes not yet committed, on Apple M5 (macOS 26.6.2), rustc 1.100.0, against jsonschema 0.60.
Each check's time is the median of 31 rounds; allocations are counted exactly.

## Typical checks

| check | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict, valid | 101 ns (0 alloc) | 150 ns (0 alloc) | 1.49x |
| review verdict, invalid | 15 ns (0 alloc) | 32 ns (0 alloc) | 2.20x |
| 1,000 rows | 21.52 µs (0 alloc) | 46.58 µs (0 alloc) | 2.16x |
| 500 objects × 20 fields | 22.82 µs (0 alloc) | 152.65 µs (0 alloc) | 6.69x |
| tree of 511 nodes by `$ref` | 4.36 µs (0 alloc) | 10.36 µs (7 alloc) | 2.38x |
| 64 tool calls, `oneOf` of 8 | 1.52 µs (0 alloc) | 2.66 µs (0 alloc) | 1.75x |
| `uniqueItems`, 1,000 strings | 2.72 µs (1 alloc) | 4.53 µs (1 alloc) | 1.67x |
| `enum` of 1,000, the last | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.12x |
| `format: date-time` | 6 ns (0 alloc) | 9 ns (0 alloc) | 1.43x |
| `format: email` | 20 ns (0 alloc) | 78 ns (1 alloc) | 3.91x |
| `format: uuid` | 4 ns (0 alloc) | 4 ns (0 alloc) | 1.06x |
| `format: ipv4` | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.29x |
| `format: uri` | 13 ns (0 alloc) | 19 ns (0 alloc) | 1.45x |
| `format: regex`, `^[a-z]+\d$` | 52 ns (0 alloc) | 60 ns (3 alloc) | 1.16x |
| `format: regex`, `^\p{L}+$` | 46 ns (0 alloc) | 56 ns (3 alloc) | 1.22x |
| `format: regex`, `^\P{ASCII}+$` | 50 ns (0 alloc) | 59 ns (3 alloc) | 1.17x |
| `format: regex`, `[\P{Letter}]` | 44 ns (0 alloc) | 64 ns (3 alloc) | 1.46x |
| `format: regex`, `(?=a)b` | 50 ns (0 alloc) | 78 ns (4 alloc) | 1.56x |
| `format: regex`, `(a)\1` | 53 ns (0 alloc) | 79 ns (4 alloc) | 1.49x |
| `format: regex`, `(?<n>a)\k<n>` | 69 ns (0 alloc) | 119 ns (7 alloc) | 1.73x |
| `pattern: ^[A-Z]{3}$` | 6 ns (0 alloc) | 13 ns (0 alloc) | 2.26x |
| `pattern: ^\d{4}-\d{2}-\d{2}$` | 7 ns (0 alloc) | 14 ns (0 alloc) | 2.08x |
| `pattern: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$` | 9 ns (0 alloc) | 17 ns (0 alloc) | 1.88x |
| `pattern: ^(small\|medium\|large)$` | 4 ns (0 alloc) | 5 ns (0 alloc) | 1.21x |

| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict | 1.09 µs | 6.04 µs | 5.56x |
| tool calls | 5.68 µs | 25.59 µs | 4.50x |

| one request | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict: compiled, one answer checked | 1.12 µs | 5.87 µs | 5.25x |
| one tool call: compiled, one answer checked | 480 ns | 3.64 µs | 7.57x |
| nested models: compiled, one answer checked | 2.22 µs | 9.91 µs | 4.47x |

Every message of an invalid verdict: flowschema 554 ns, jsonschema 663 ns (1.20x).

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
| compiling every schema | 125.4 ms | 461.8 ms (flowschema 3.68x faster) |
| checking every answer of the 9,779 schemas both check whole | 2.74 ms | 4.23 ms (flowschema 1.55x faster) |

The two agree on 99.93% of the answers (`corpus-disagreements.tsv`). 21 schemas (0.19%) ask something not checked here: `pattern` (19), `$ref outside the schema` (1), `format` (1). flowschema is slower by over 5% on 508 schemas (`corpus-slower.tsv`).

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
