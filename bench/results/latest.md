# The latest run

2026-11-14, flowschema 2d558d7 with changes not yet committed, on Apple M5 (macOS 26.6.2), rustc 1.100.0, against jsonschema 0.60.
Each check's time is the median of 31 rounds; allocations are counted exactly.

## Typical checks

| check | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict, valid | 110 ns (0 alloc) | 157 ns (0 alloc) | 1.43x |
| review verdict, invalid | 18 ns (0 alloc) | 32 ns (0 alloc) | 1.83x |
| 1,000 rows | 25.41 µs (0 alloc) | 49.60 µs (0 alloc) | 1.95x |
| 500 objects × 20 fields | 26.47 µs (0 alloc) | 152.38 µs (0 alloc) | 5.76x |
| tree of 511 nodes by `$ref` | 5.19 µs (0 alloc) | 10.28 µs (7 alloc) | 1.98x |
| 64 tool calls, `oneOf` of 8 | 1.67 µs (0 alloc) | 2.97 µs (0 alloc) | 1.77x |
| `uniqueItems`, 1,000 strings | 3.57 µs (1 alloc) | 4.90 µs (1 alloc) | 1.37x |
| `enum` of 1,000, the last | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.26x |
| `format: date-time` | 7 ns (0 alloc) | 10 ns (0 alloc) | 1.45x |
| `format: email` | 21 ns (0 alloc) | 85 ns (1 alloc) | 4.09x |
| `format: uuid` | 4 ns (0 alloc) | 4 ns (0 alloc) | 1.17x |
| `format: ipv4` | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.27x |
| `format: uri` | 14 ns (0 alloc) | 21 ns (0 alloc) | 1.43x |
| `format: regex`, `^[a-z]+\d$` | 55 ns (0 alloc) | 68 ns (3 alloc) | 1.22x |
| `format: regex`, `^\p{L}+$` | 49 ns (0 alloc) | 58 ns (3 alloc) | 1.17x |
| `format: regex`, `^\P{ASCII}+$` | 54 ns (0 alloc) | 63 ns (3 alloc) | 1.17x |
| `format: regex`, `[\P{Letter}]` | 48 ns (0 alloc) | 68 ns (3 alloc) | 1.42x |
| `format: regex`, `(?=a)b` | 55 ns (0 alloc) | 83 ns (4 alloc) | 1.52x |
| `format: regex`, `(a)\1` | 57 ns (0 alloc) | 85 ns (4 alloc) | 1.48x |
| `format: regex`, `(?<n>a)\k<n>` | 74 ns (0 alloc) | 130 ns (7 alloc) | 1.75x |
| `pattern: ^[A-Z]{3}$` | 6 ns (0 alloc) | 15 ns (0 alloc) | 2.28x |
| `pattern: ^\d{4}-\d{2}-\d{2}$` | 8 ns (0 alloc) | 16 ns (0 alloc) | 2.16x |
| `pattern: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$` | 9 ns (0 alloc) | 18 ns (0 alloc) | 1.90x |
| `pattern: ^(small\|medium\|large)$` | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.17x |

| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict | 1.15 µs | 6.28 µs | 5.45x |
| tool calls | 5.68 µs | 25.94 µs | 4.57x |

| one request | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict: compiled, one answer checked | 1.17 µs | 6.08 µs | 5.19x |
| one tool call: compiled, one answer checked | 483 ns | 3.66 µs | 7.57x |
| nested models: compiled, one answer checked | 2.32 µs | 9.97 µs | 4.31x |

Every message of an invalid verdict: flowschema 559 ns, jsonschema 720 ns (1.29x).

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
| compiling every schema | 124.7 ms | 464.5 ms (flowschema 3.72x faster) |
| checking every answer of the 9,779 schemas both check whole | 3.00 ms | 4.25 ms (flowschema 1.42x faster) |

The two agree on 99.93% of the answers (`corpus-disagreements.tsv`). 21 schemas (0.19%) ask something not checked here: `pattern` (19), `$ref outside the schema` (1), `format` (1). flowschema is slower by over 5% on 1120 schemas (`corpus-slower.tsv`).

## Size

| added to a release binary built for size | size |
|---|---|
| flowschema, all of it | 293 KB |
| of which its own code | 244 KB |
| of which regex-lite | 49 KB |
| jsonschema 0.60 | 1,884 KB |

| crates added to the build, itself included | crates |
|---|---|
| flowschema | 2 |
| jsonschema 0.60 | 57 |

Over a program that reads the JSON alone (312 KB), stripped, `opt-level = "z"`, fat LTO. regex-lite is measured alone, as a program that uses it pays for it.
