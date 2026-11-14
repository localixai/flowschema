# The latest run

2026-11-14, flowschema fa8f64c with changes not yet committed, on Apple M5 (macOS 26.6.2), rustc 1.100.0, against jsonschema 0.60.
Each check's time is the median of 31 rounds; allocations are counted exactly.

## Typical checks

| check | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict, valid | 102 ns (0 alloc) | 159 ns (0 alloc) | 1.56x |
| review verdict, invalid | 17 ns (0 alloc) | 31 ns (0 alloc) | 1.87x |
| 1,000 rows | 23.66 µs (0 alloc) | 46.62 µs (0 alloc) | 1.97x |
| 500 objects × 20 fields | 24.30 µs (0 alloc) | 168.94 µs (0 alloc) | 6.95x |
| tree of 511 nodes by `$ref` | 4.56 µs (0 alloc) | 10.42 µs (7 alloc) | 2.29x |
| 64 tool calls, `oneOf` of 8 | 1.57 µs (0 alloc) | 2.65 µs (0 alloc) | 1.68x |
| `uniqueItems`, 1,000 strings | 2.64 µs (1 alloc) | 4.54 µs (1 alloc) | 1.72x |
| `enum` of 1,000, the last | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.31x |
| `format: date-time` | 6 ns (0 alloc) | 10 ns (0 alloc) | 1.66x |
| `format: email` | 19 ns (0 alloc) | 78 ns (1 alloc) | 4.05x |
| `format: uuid` | 3 ns (0 alloc) | 4 ns (0 alloc) | 1.33x |
| `format: ipv4` | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.42x |
| `format: uri` | 13 ns (0 alloc) | 20 ns (0 alloc) | 1.54x |
| `format: regex`, `^[a-z]+\d$` | 52 ns (0 alloc) | 59 ns (3 alloc) | 1.13x |
| `format: regex`, `^\p{L}+$` | 46 ns (0 alloc) | 57 ns (3 alloc) | 1.24x |
| `format: regex`, `^\P{ASCII}+$` | 51 ns (0 alloc) | 63 ns (3 alloc) | 1.23x |
| `format: regex`, `[\P{Letter}]` | 44 ns (0 alloc) | 60 ns (3 alloc) | 1.36x |
| `format: regex`, `(?=a)b` | 51 ns (0 alloc) | 75 ns (4 alloc) | 1.46x |
| `format: regex`, `(a)\1` | 54 ns (0 alloc) | 79 ns (4 alloc) | 1.45x |
| `format: regex`, `(?<n>a)\k<n>` | 70 ns (0 alloc) | 125 ns (7 alloc) | 1.79x |
| `pattern: ^[A-Z]{3}$` | 6 ns (0 alloc) | 15 ns (0 alloc) | 2.47x |
| `pattern: ^\d{4}-\d{2}-\d{2}$` | 7 ns (0 alloc) | 16 ns (0 alloc) | 2.12x |
| `pattern: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$` | 8 ns (0 alloc) | 16 ns (0 alloc) | 1.96x |
| `pattern: ^(small\|medium\|large)$` | 4 ns (0 alloc) | 5 ns (0 alloc) | 1.20x |

| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict | 1.06 µs | 6.10 µs | 5.73x |
| tool calls | 5.68 µs | 25.05 µs | 4.41x |

| one request | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict: compiled, one answer checked | 1.20 µs | 5.95 µs | 4.97x |
| one tool call: compiled, one answer checked | 471 ns | 3.66 µs | 7.78x |
| nested models: compiled, one answer checked | 2.37 µs | 9.79 µs | 4.14x |

Every message of an invalid verdict: flowschema 572 ns, jsonschema 696 ns (1.22x).

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
| compiling every schema | 125.9 ms | 460.7 ms (flowschema 3.66x faster) |
| checking every answer of the 9,779 schemas both check whole | 2.68 ms | 4.24 ms (flowschema 1.59x faster) |

The two agree on 99.93% of the answers (`corpus-disagreements.tsv`). 21 schemas (0.19%) ask something not checked here: `pattern` (19), `$ref outside the schema` (1), `format` (1). flowschema is slower by over 5% on 381 schemas (`corpus-slower.tsv`).

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
