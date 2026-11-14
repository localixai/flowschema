# The latest run

2026-11-14, flowschema d0de540 with changes not yet committed, on Apple M5 (macOS 26.6.2), rustc 1.100.0, against jsonschema 0.60.
Each check's time is the median of 31 rounds; allocations are counted exactly.

## Typical checks

| check | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict, valid | 124 ns (0 alloc) | 180 ns (0 alloc) | 1.46x |
| review verdict, invalid | 19 ns (0 alloc) | 36 ns (0 alloc) | 1.90x |
| 1,000 rows | 26.80 µs (0 alloc) | 51.15 µs (0 alloc) | 1.91x |
| 500 objects × 20 fields | 26.48 µs (0 alloc) | 185.96 µs (0 alloc) | 7.02x |
| tree of 511 nodes by `$ref` | 5.62 µs (0 alloc) | 11.18 µs (7 alloc) | 1.99x |
| 64 tool calls, `oneOf` of 8 | 1.87 µs (0 alloc) | 2.84 µs (0 alloc) | 1.52x |
| `uniqueItems`, 1,000 strings | 2.83 µs (1 alloc) | 6.01 µs (1 alloc) | 2.12x |
| `enum` of 1,000, the last | 6 ns (0 alloc) | 7 ns (0 alloc) | 1.17x |
| `format: date-time` | 9 ns (0 alloc) | 11 ns (0 alloc) | 1.27x |
| `format: email` | 25 ns (0 alloc) | 97 ns (1 alloc) | 3.93x |
| `format: uuid` | 5 ns (0 alloc) | 5 ns (0 alloc) | 1.06x |
| `format: ipv4` | 7 ns (0 alloc) | 8 ns (0 alloc) | 1.22x |
| `format: uri` | 16 ns (0 alloc) | 23 ns (0 alloc) | 1.42x |
| `format: regex`, `^[a-z]+\d$` | 64 ns (0 alloc) | 74 ns (3 alloc) | 1.15x |
| `format: regex`, `^\p{L}+$` | 56 ns (0 alloc) | 64 ns (3 alloc) | 1.15x |
| `format: regex`, `^\P{ASCII}+$` | 65 ns (0 alloc) | 77 ns (3 alloc) | 1.19x |
| `format: regex`, `[\P{Letter}]` | 55 ns (0 alloc) | 72 ns (3 alloc) | 1.31x |
| `format: regex`, `(?=a)b` | 60 ns (0 alloc) | 94 ns (4 alloc) | 1.56x |
| `format: regex`, `(a)\1` | 64 ns (0 alloc) | 95 ns (4 alloc) | 1.48x |
| `format: regex`, `(?<n>a)\k<n>` | 82 ns (0 alloc) | 141 ns (7 alloc) | 1.73x |
| `pattern: ^[A-Z]{3}$` | 6 ns (0 alloc) | 16 ns (0 alloc) | 2.46x |
| `pattern: ^\d{4}-\d{2}-\d{2}$` | 8 ns (0 alloc) | 18 ns (0 alloc) | 2.09x |
| `pattern: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$` | 10 ns (0 alloc) | 19 ns (0 alloc) | 1.95x |
| `pattern: ^(small\|medium\|large)$` | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.31x |

| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict | 1.26 µs | 6.97 µs | 5.51x |
| tool calls | 6.71 µs | 28.60 µs | 4.27x |

| one request | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict: compiled, one answer checked | 1.33 µs | 7.09 µs | 5.31x |
| one tool call: compiled, one answer checked | 554 ns | 4.18 µs | 7.56x |
| nested models: compiled, one answer checked | 2.69 µs | 10.90 µs | 4.05x |

Every message of an invalid verdict: flowschema 652 ns, jsonschema 800 ns (1.23x).

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
| compiling every schema | 126.6 ms | 474.5 ms (flowschema 3.75x faster) |
| checking every answer of the 9,779 schemas both check whole | 3.09 ms | 4.33 ms (flowschema 1.40x faster) |

The two agree on 99.93% of the answers (`corpus-disagreements.tsv`). 21 schemas (0.19%) ask something not checked here: `pattern` (19), `$ref outside the schema` (1), `format` (1). flowschema is slower by over 5% on 1193 schemas (`corpus-slower.tsv`).

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
