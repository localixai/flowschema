# The latest run

2026-11-14, flowschema 176b1ce with changes not yet committed, on Apple M5 (macOS 26.6.2), rustc 1.100.0, against jsonschema 0.60.
Each check's time is the median of 31 rounds; allocations are counted exactly.

## Typical checks

| check | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict, valid | 96 ns (0 alloc) | 174 ns (0 alloc) | 1.81x |
| review verdict, invalid | 16 ns (0 alloc) | 36 ns (0 alloc) | 2.26x |
| 1,000 rows | 21.62 µs (0 alloc) | 49.64 µs (0 alloc) | 2.30x |
| 500 objects × 20 fields | 27.24 µs (0 alloc) | 177.81 µs (0 alloc) | 6.53x |
| tree of 511 nodes by `$ref` | 4.55 µs (0 alloc) | 12.45 µs (7 alloc) | 2.73x |
| 64 tool calls, `oneOf` of 8 | 1.60 µs (0 alloc) | 3.07 µs (0 alloc) | 1.91x |
| `uniqueItems`, 1,000 strings | 3.14 µs (1 alloc) | 5.07 µs (1 alloc) | 1.61x |
| `enum` of 1,000, the last | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.23x |
| `format: date-time` | 7 ns (0 alloc) | 10 ns (0 alloc) | 1.51x |
| `format: email` | 21 ns (0 alloc) | 91 ns (1 alloc) | 4.28x |
| `format: uuid` | 3 ns (0 alloc) | 5 ns (0 alloc) | 1.33x |
| `format: ipv4` | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.40x |
| `format: uri` | 14 ns (0 alloc) | 23 ns (0 alloc) | 1.62x |
| `format: regex`, `^[a-z]+\d$` | 57 ns (0 alloc) | 67 ns (3 alloc) | 1.17x |
| `format: regex`, `^\p{L}+$` | 53 ns (0 alloc) | 66 ns (3 alloc) | 1.25x |
| `format: regex`, `^\P{ASCII}+$` | 57 ns (0 alloc) | 69 ns (3 alloc) | 1.21x |
| `format: regex`, `[\P{Letter}]` | 49 ns (0 alloc) | 73 ns (3 alloc) | 1.49x |
| `format: regex`, `(?=a)b` | 57 ns (0 alloc) | 90 ns (4 alloc) | 1.57x |
| `format: regex`, `(a)\1` | 60 ns (0 alloc) | 87 ns (4 alloc) | 1.45x |
| `format: regex`, `(?<n>a)\k<n>` | 76 ns (0 alloc) | 138 ns (7 alloc) | 1.80x |
| `pattern: ^[A-Z]{3}$` | 6 ns (0 alloc) | 16 ns (0 alloc) | 2.71x |
| `pattern: ^\d{4}-\d{2}-\d{2}$` | 7 ns (0 alloc) | 17 ns (0 alloc) | 2.24x |
| `pattern: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$` | 9 ns (0 alloc) | 18 ns (0 alloc) | 2.03x |
| `pattern: ^(small\|medium\|large)$` | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.24x |

| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict | 1.28 µs | 6.88 µs | 5.38x |
| tool calls | 6.46 µs | 28.77 µs | 4.46x |

| one request | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict: compiled, one answer checked | 1.27 µs | 6.62 µs | 5.21x |
| one tool call: compiled, one answer checked | 533 ns | 4.16 µs | 7.80x |
| nested models: compiled, one answer checked | 2.74 µs | 11.01 µs | 4.02x |

Every message of an invalid verdict: flowschema 643 ns, jsonschema 828 ns (1.29x).

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
| compiling every schema | 128.7 ms | 474.0 ms (flowschema 3.68x faster) |
| checking every answer of the 9,779 schemas both check whole | 2.39 ms | 4.30 ms (flowschema 1.79x faster) |

The two agree on 99.93% of the answers (`corpus-disagreements.tsv`). 21 schemas (0.19%) ask something not checked here: `pattern` (19), `$ref outside the schema` (1), `format` (1). flowschema is slower by over 5% on 149 schemas (`corpus-slower.tsv`).

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
