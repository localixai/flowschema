# The latest run

2026-11-14, flowschema de1a71c with changes not yet committed, on Apple M5 (macOS 26.6.2), rustc 1.100.0, against jsonschema 0.60.
Each check's time is the median of 31 rounds; allocations are counted exactly.

## Typical checks

| check | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict, valid | 92 ns (0 alloc) | 167 ns (0 alloc) | 1.82x |
| review verdict, invalid | 14 ns (0 alloc) | 34 ns (0 alloc) | 2.46x |
| 1,000 rows | 20.14 µs (0 alloc) | 47.38 µs (0 alloc) | 2.35x |
| 500 objects × 20 fields | 33.66 µs (0 alloc) | 171.79 µs (0 alloc) | 5.10x |
| tree of 511 nodes by `$ref` | 4.22 µs (0 alloc) | 11.40 µs (7 alloc) | 2.70x |
| 64 tool calls, `oneOf` of 8 | 1.52 µs (0 alloc) | 2.73 µs (0 alloc) | 1.80x |
| `uniqueItems`, 1,000 strings | 2.99 µs (1 alloc) | 5.06 µs (1 alloc) | 1.69x |
| `enum` of 1,000, the last | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.23x |
| `format: date-time` | 7 ns (0 alloc) | 10 ns (0 alloc) | 1.52x |
| `format: email` | 22 ns (0 alloc) | 88 ns (1 alloc) | 4.03x |
| `format: uuid` | 4 ns (0 alloc) | 5 ns (0 alloc) | 1.33x |
| `format: ipv4` | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.40x |
| `format: uri` | 14 ns (0 alloc) | 22 ns (0 alloc) | 1.55x |
| `format: regex`, `^[a-z]+\d$` | 58 ns (0 alloc) | 77 ns (3 alloc) | 1.33x |
| `format: regex`, `^\p{L}+$` | 50 ns (0 alloc) | 60 ns (3 alloc) | 1.20x |
| `format: regex`, `^\P{ASCII}+$` | 57 ns (0 alloc) | 66 ns (3 alloc) | 1.15x |
| `format: regex`, `[\P{Letter}]` | 50 ns (0 alloc) | 66 ns (3 alloc) | 1.34x |
| `format: regex`, `(?=a)b` | 57 ns (0 alloc) | 89 ns (4 alloc) | 1.55x |
| `format: regex`, `(a)\1` | 59 ns (0 alloc) | 89 ns (4 alloc) | 1.51x |
| `format: regex`, `(?<n>a)\k<n>` | 76 ns (0 alloc) | 139 ns (7 alloc) | 1.83x |
| `pattern: ^[A-Z]{3}$` | 6 ns (0 alloc) | 16 ns (0 alloc) | 2.68x |
| `pattern: ^\d{4}-\d{2}-\d{2}$` | 7 ns (0 alloc) | 17 ns (0 alloc) | 2.22x |
| `pattern: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$` | 9 ns (0 alloc) | 18 ns (0 alloc) | 2.05x |
| `pattern: ^(small\|medium\|large)$` | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.17x |

| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict | 1.24 µs | 6.77 µs | 5.45x |
| tool calls | 6.46 µs | 26.93 µs | 4.17x |

| one request | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict: compiled, one answer checked | 1.22 µs | 6.45 µs | 5.30x |
| one tool call: compiled, one answer checked | 534 ns | 3.96 µs | 7.42x |
| nested models: compiled, one answer checked | 2.54 µs | 10.63 µs | 4.19x |

Every message of an invalid verdict: flowschema 594 ns, jsonschema 770 ns (1.30x).

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
| compiling every schema | 129.8 ms | 471.8 ms (flowschema 3.64x faster) |
| checking every answer of the 9,779 schemas both check whole | 2.41 ms | 4.32 ms (flowschema 1.79x faster) |

The two agree on 99.93% of the answers (`corpus-disagreements.tsv`). 21 schemas (0.19%) ask something not checked here: `pattern` (19), `$ref outside the schema` (1), `format` (1). flowschema is slower by over 5% on 175 schemas (`corpus-slower.tsv`).

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
