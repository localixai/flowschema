# The latest run

2026-11-15, flowschema d1a5ed3 with changes not yet committed, on Apple M5 (macOS 26.6.2), rustc 1.100.0, against jsonschema 0.60.
Each check's time is the median of 31 rounds; allocations are counted exactly.

## Typical checks

| check | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict, valid | 88 ns (0 alloc) | 158 ns (0 alloc) | 1.78x |
| review verdict, invalid | 15 ns (0 alloc) | 32 ns (0 alloc) | 2.18x |
| 1,000 rows | 21.01 µs (0 alloc) | 43.46 µs (0 alloc) | 2.07x |
| 500 objects × 20 fields | 23.69 µs (0 alloc) | 166.00 µs (0 alloc) | 7.01x |
| tree of 511 nodes by `$ref` | 4.44 µs (0 alloc) | 10.53 µs (7 alloc) | 2.37x |
| 64 tool calls, `oneOf` of 8 | 1.41 µs (0 alloc) | 2.93 µs (0 alloc) | 2.08x |
| `uniqueItems`, 1,000 strings | 2.85 µs (1 alloc) | 4.88 µs (1 alloc) | 1.71x |
| `enum` of 1,000, the last | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.25x |
| `format: date-time` | 7 ns (0 alloc) | 10 ns (0 alloc) | 1.48x |
| `format: email` | 21 ns (0 alloc) | 83 ns (1 alloc) | 3.97x |
| `format: uuid` | 3 ns (0 alloc) | 4 ns (0 alloc) | 1.25x |
| `format: ipv4` | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.42x |
| `format: uri` | 14 ns (0 alloc) | 22 ns (0 alloc) | 1.64x |
| `format: regex`, `^[a-z]+\d$` | 57 ns (0 alloc) | 63 ns (3 alloc) | 1.12x |
| `format: regex`, `^\p{L}+$` | 49 ns (0 alloc) | 58 ns (3 alloc) | 1.19x |
| `format: regex`, `^\P{ASCII}+$` | 56 ns (0 alloc) | 63 ns (3 alloc) | 1.13x |
| `format: regex`, `[\P{Letter}]` | 48 ns (0 alloc) | 66 ns (3 alloc) | 1.38x |
| `format: regex`, `(?=a)b` | 54 ns (0 alloc) | 86 ns (4 alloc) | 1.59x |
| `format: regex`, `(a)\1` | 58 ns (0 alloc) | 83 ns (4 alloc) | 1.45x |
| `format: regex`, `(?<n>a)\k<n>` | 73 ns (0 alloc) | 130 ns (7 alloc) | 1.78x |
| `pattern: ^[A-Z]{3}$` | 5 ns (0 alloc) | 15 ns (0 alloc) | 3.01x |
| `pattern: ^\d{4}-\d{2}-\d{2}$` | 8 ns (0 alloc) | 16 ns (0 alloc) | 2.10x |
| `pattern: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$` | 10 ns (0 alloc) | 18 ns (0 alloc) | 1.80x |
| `pattern: ^(small\|medium\|large)$` | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.14x |

| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict | 1.11 µs | 6.36 µs | 5.71x |
| tool calls | 5.84 µs | 26.59 µs | 4.55x |

| one request | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict: compiled, one answer checked | 1.16 µs | 6.31 µs | 5.43x |
| one tool call: compiled, one answer checked | 508 ns | 3.89 µs | 7.65x |
| nested models: compiled, one answer checked | 2.46 µs | 10.34 µs | 4.20x |

Every message of an invalid verdict: flowschema 577 ns, jsonschema 704 ns (1.22x).

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
| compiling every schema | 137.8 ms | 484.2 ms (flowschema 3.51x faster) |
| checking every answer of the 9,779 schemas both check whole | 2.42 ms | 4.41 ms (flowschema 1.83x faster) |

The two agree on 99.93% of the answers (`corpus-disagreements.tsv`). 21 schemas (0.19%) ask something not checked here: `pattern` (19), `$ref outside the schema` (1), `format` (1). flowschema is slower by over 5% on 127 schemas (`corpus-slower.tsv`).

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
