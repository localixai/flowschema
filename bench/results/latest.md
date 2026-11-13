# The latest run

2026-11-13, flowschema 173072e with changes not yet committed, on Apple M5 (macOS 26.6.2), rustc 1.100.0, against jsonschema 0.60.
Each check's time is the median of 31 rounds; allocations are counted exactly.

## Typical checks

| check | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict, valid | 398 ns (0 alloc) | 603 ns (0 alloc) | 1.52x |
| review verdict, invalid | 53 ns (0 alloc) | 108 ns (0 alloc) | 2.05x |
| 1,000 rows | 91.90 µs (0 alloc) | 152.67 µs (0 alloc) | 1.66x |
| 500 objects × 20 fields | 96.56 µs (0 alloc) | 376.46 µs (0 alloc) | 3.90x |
| tree of 511 nodes by `$ref` | 16.76 µs (0 alloc) | 26.35 µs (7 alloc) | 1.57x |
| 64 tool calls, `oneOf` of 8 | 6.00 µs (0 alloc) | 7.70 µs (0 alloc) | 1.28x |
| `uniqueItems`, 1,000 strings | 6.43 µs (1 alloc) | 8.56 µs (1 alloc) | 1.33x |
| `enum` of 1,000, the last | 11 ns (0 alloc) | 21 ns (0 alloc) | 1.95x |
| `format: date-time` | 10 ns (0 alloc) | 19 ns (0 alloc) | 1.97x |
| `format: email` | 32 ns (0 alloc) | 166 ns (1 alloc) | 5.20x |
| `format: uuid` | 8 ns (0 alloc) | 10 ns (0 alloc) | 1.26x |
| `format: ipv4` | 9 ns (0 alloc) | 14 ns (0 alloc) | 1.53x |
| `format: uri` | 26 ns (0 alloc) | 46 ns (0 alloc) | 1.75x |
| `format: regex`, `^[a-z]+\d$` | 99 ns (0 alloc) | 154 ns (3 alloc) | 1.56x |
| `format: regex`, `^\p{L}+$` | 87 ns (0 alloc) | 136 ns (3 alloc) | 1.56x |
| `format: regex`, `^\P{ASCII}+$` | 105 ns (0 alloc) | 143 ns (3 alloc) | 1.37x |
| `format: regex`, `[\P{Letter}]` | 93 ns (0 alloc) | 146 ns (3 alloc) | 1.58x |
| `format: regex`, `(?=a)b` | 75 ns (0 alloc) | 147 ns (4 alloc) | 1.95x |
| `format: regex`, `(a)\1` | 77 ns (0 alloc) | 147 ns (4 alloc) | 1.90x |
| `format: regex`, `(?<n>a)\k<n>` | 105 ns (0 alloc) | 239 ns (7 alloc) | 2.28x |
| `pattern: ^[A-Z]{3}$` | 10 ns (0 alloc) | 39 ns (0 alloc) | 3.71x |
| `pattern: ^\d{4}-\d{2}-\d{2}$` | 13 ns (0 alloc) | 40 ns (0 alloc) | 3.18x |
| `pattern: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$` | 14 ns (0 alloc) | 41 ns (0 alloc) | 3.01x |
| `pattern: ^(small\|medium\|large)$` | 10 ns (0 alloc) | 14 ns (0 alloc) | 1.42x |

| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict | 6.95 µs | 15.09 µs | 2.17x |
| tool calls | 29.12 µs | 65.56 µs | 2.25x |

Every message of an invalid verdict: flowschema 1.49 µs, jsonschema 1.66 µs (1.11x).

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
| compiling every schema | 520.8 ms | 1059.7 ms (flowschema 2.03x faster) |
| checking every answer of the 9,779 schemas both check whole | 8.73 ms | 12.47 ms (flowschema 1.43x faster) |

The two agree on 99.93% of the answers (`corpus-disagreements.tsv`). 21 schemas (0.19%) ask something not checked here: `pattern` (19), `$ref outside the schema` (1), `format` (1). flowschema is slower by over 5% on 350 schemas (`corpus-slower.tsv`).

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
