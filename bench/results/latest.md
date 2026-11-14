# The latest run

2026-11-14, flowschema 9648627 with changes not yet committed, on Apple M5 (macOS 26.6.2), rustc 1.100.0, against jsonschema 0.60.
Each check's time is the median of 31 rounds; allocations are counted exactly.

## Typical checks

| check | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict, valid | 115 ns (0 alloc) | 162 ns (0 alloc) | 1.42x |
| review verdict, invalid | 20 ns (0 alloc) | 34 ns (0 alloc) | 1.70x |
| 1,000 rows | 25.57 µs (0 alloc) | 48.00 µs (0 alloc) | 1.88x |
| 500 objects × 20 fields | 30.10 µs (0 alloc) | 167.67 µs (0 alloc) | 5.57x |
| tree of 511 nodes by `$ref` | 7.55 µs (0 alloc) | 10.17 µs (7 alloc) | 1.35x |
| 64 tool calls, `oneOf` of 8 | 2.05 µs (0 alloc) | 2.70 µs (0 alloc) | 1.32x |
| `uniqueItems`, 1,000 strings | 2.73 µs (1 alloc) | 4.54 µs (1 alloc) | 1.66x |
| `enum` of 1,000, the last | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.28x |
| `format: date-time` | 6 ns (0 alloc) | 10 ns (0 alloc) | 1.57x |
| `format: email` | 20 ns (0 alloc) | 77 ns (1 alloc) | 3.89x |
| `format: uuid` | 4 ns (0 alloc) | 4 ns (0 alloc) | 1.06x |
| `format: ipv4` | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.26x |
| `format: uri` | 14 ns (0 alloc) | 21 ns (0 alloc) | 1.52x |
| `format: regex`, `^[a-z]+\d$` | 52 ns (0 alloc) | 66 ns (3 alloc) | 1.26x |
| `format: regex`, `^\p{L}+$` | 48 ns (0 alloc) | 58 ns (3 alloc) | 1.22x |
| `format: regex`, `^\P{ASCII}+$` | 52 ns (0 alloc) | 62 ns (3 alloc) | 1.19x |
| `format: regex`, `[\P{Letter}]` | 46 ns (0 alloc) | 82 ns (3 alloc) | 1.78x |
| `format: regex`, `(?=a)b` | 56 ns (0 alloc) | 88 ns (4 alloc) | 1.59x |
| `format: regex`, `(a)\1` | 61 ns (0 alloc) | 86 ns (4 alloc) | 1.41x |
| `format: regex`, `(?<n>a)\k<n>` | 76 ns (0 alloc) | 165 ns (7 alloc) | 2.19x |
| `pattern: ^[A-Z]{3}$` | 6 ns (0 alloc) | 15 ns (0 alloc) | 2.36x |
| `pattern: ^\d{4}-\d{2}-\d{2}$` | 8 ns (0 alloc) | 17 ns (0 alloc) | 2.20x |
| `pattern: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$` | 9 ns (0 alloc) | 18 ns (0 alloc) | 1.93x |
| `pattern: ^(small\|medium\|large)$` | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.15x |

| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict | 1.08 µs | 6.43 µs | 5.93x |
| tool calls | 6.02 µs | 27.18 µs | 4.51x |

| one request | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict: compiled, one answer checked | 1.18 µs | 6.31 µs | 5.34x |
| one tool call: compiled, one answer checked | 483 ns | 3.88 µs | 8.04x |

Every message of an invalid verdict: flowschema 613 ns, jsonschema 754 ns (1.23x).

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
| compiling every schema | 123.4 ms | 466.9 ms (flowschema 3.78x faster) |
| checking every answer of the 9,779 schemas both check whole | 3.41 ms | 4.25 ms (flowschema 1.25x faster) |

The two agree on 99.93% of the answers (`corpus-disagreements.tsv`). 21 schemas (0.19%) ask something not checked here: `pattern` (19), `$ref outside the schema` (1), `format` (1). flowschema is slower by over 5% on 1812 schemas (`corpus-slower.tsv`).

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
