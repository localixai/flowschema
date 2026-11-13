# The latest run

2026-11-14, flowschema ce1eb88 with changes not yet committed, on Apple M5 (macOS 26.6.2), rustc 1.100.0, against jsonschema 0.60.
Each check's time is the median of 31 rounds; allocations are counted exactly.

## Typical checks

| check | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict, valid | 160 ns (0 alloc) | 166 ns (0 alloc) | 1.04x |
| review verdict, invalid | 34 ns (0 alloc) | 34 ns (0 alloc) | 0.99x |
| 1,000 rows | 45.70 µs (0 alloc) | 42.42 µs (0 alloc) | 0.93x |
| 500 objects × 20 fields | 51.34 µs (0 alloc) | 168.85 µs (0 alloc) | 3.29x |
| tree of 511 nodes by `$ref` | 9.62 µs (0 alloc) | 11.52 µs (7 alloc) | 1.20x |
| 64 tool calls, `oneOf` of 8 | 3.50 µs (0 alloc) | 3.85 µs (0 alloc) | 1.10x |
| `uniqueItems`, 1,000 strings | 4.22 µs (1 alloc) | 4.52 µs (1 alloc) | 1.07x |
| `enum` of 1,000, the last | 6 ns (0 alloc) | 7 ns (0 alloc) | 1.14x |
| `format: date-time` | 9 ns (0 alloc) | 9 ns (0 alloc) | 1.06x |
| `format: email` | 20 ns (0 alloc) | 79 ns (1 alloc) | 3.85x |
| `format: uuid` | 4 ns (0 alloc) | 4 ns (0 alloc) | 0.92x |
| `format: ipv4` | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.31x |
| `format: uri` | 14 ns (0 alloc) | 20 ns (0 alloc) | 1.43x |
| `format: regex`, `^[a-z]+\d$` | 54 ns (0 alloc) | 102 ns (3 alloc) | 1.88x |
| `format: regex`, `^\p{L}+$` | 50 ns (0 alloc) | 60 ns (3 alloc) | 1.21x |
| `format: regex`, `^\P{ASCII}+$` | 55 ns (0 alloc) | 69 ns (3 alloc) | 1.25x |
| `format: regex`, `[\P{Letter}]` | 48 ns (0 alloc) | 70 ns (3 alloc) | 1.47x |
| `format: regex`, `(?=a)b` | 55 ns (0 alloc) | 84 ns (4 alloc) | 1.52x |
| `format: regex`, `(a)\1` | 57 ns (0 alloc) | 87 ns (4 alloc) | 1.53x |
| `format: regex`, `(?<n>a)\k<n>` | 74 ns (0 alloc) | 135 ns (7 alloc) | 1.83x |
| `pattern: ^[A-Z]{3}$` | 6 ns (0 alloc) | 15 ns (0 alloc) | 2.33x |
| `pattern: ^\d{4}-\d{2}-\d{2}$` | 8 ns (0 alloc) | 16 ns (0 alloc) | 2.17x |
| `pattern: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$` | 9 ns (0 alloc) | 18 ns (0 alloc) | 1.95x |
| `pattern: ^(small\|medium\|large)$` | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.17x |

| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict | 1.13 µs | 6.51 µs | 5.78x |
| tool calls | 6.01 µs | 27.46 µs | 4.57x |

Every message of an invalid verdict: flowschema 750 ns, jsonschema 815 ns (1.09x).

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
| compiling every schema | 133.0 ms | 497.6 ms (flowschema 3.74x faster) |
| checking every answer of the 9,779 schemas both check whole | 5.03 ms | 4.48 ms (flowschema 0.89x faster) |

The two agree on 99.93% of the answers (`corpus-disagreements.tsv`). 21 schemas (0.19%) ask something not checked here: `pattern` (19), `$ref outside the schema` (1), `format` (1). flowschema is slower by over 5% on 6828 schemas (`corpus-slower.tsv`).

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
