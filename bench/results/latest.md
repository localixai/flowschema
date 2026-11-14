# The latest run

2026-11-14, flowschema 584f9e1 with changes not yet committed, on Apple M5 (macOS 26.6.2), rustc 1.100.0, against jsonschema 0.60.
Each check's time is the median of 31 rounds; allocations are counted exactly.

## Typical checks

| check | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict, valid | 88 ns (0 alloc) | 151 ns (0 alloc) | 1.71x |
| review verdict, invalid | 13 ns (0 alloc) | 38 ns (0 alloc) | 2.85x |
| 1,000 rows | 20.26 µs (0 alloc) | 45.19 µs (0 alloc) | 2.23x |
| 500 objects × 20 fields | 26.18 µs (0 alloc) | 186.67 µs (0 alloc) | 7.13x |
| tree of 511 nodes by `$ref` | 4.38 µs (0 alloc) | 11.30 µs (7 alloc) | 2.58x |
| 64 tool calls, `oneOf` of 8 | 1.60 µs (0 alloc) | 2.83 µs (0 alloc) | 1.77x |
| `uniqueItems`, 1,000 strings | 2.85 µs (1 alloc) | 4.88 µs (1 alloc) | 1.71x |
| `enum` of 1,000, the last | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.29x |
| `format: date-time` | 7 ns (0 alloc) | 10 ns (0 alloc) | 1.58x |
| `format: email` | 21 ns (0 alloc) | 88 ns (1 alloc) | 4.23x |
| `format: uuid` | 3 ns (0 alloc) | 4 ns (0 alloc) | 1.25x |
| `format: ipv4` | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.40x |
| `format: uri` | 15 ns (0 alloc) | 21 ns (0 alloc) | 1.43x |
| `format: regex`, `^[a-z]+\d$` | 56 ns (0 alloc) | 63 ns (3 alloc) | 1.13x |
| `format: regex`, `^\p{L}+$` | 50 ns (0 alloc) | 60 ns (3 alloc) | 1.20x |
| `format: regex`, `^\P{ASCII}+$` | 54 ns (0 alloc) | 66 ns (3 alloc) | 1.22x |
| `format: regex`, `[\P{Letter}]` | 48 ns (0 alloc) | 67 ns (3 alloc) | 1.40x |
| `format: regex`, `(?=a)b` | 54 ns (0 alloc) | 80 ns (4 alloc) | 1.46x |
| `format: regex`, `(a)\1` | 57 ns (0 alloc) | 88 ns (4 alloc) | 1.53x |
| `format: regex`, `(?<n>a)\k<n>` | 74 ns (0 alloc) | 135 ns (7 alloc) | 1.83x |
| `pattern: ^[A-Z]{3}$` | 5 ns (0 alloc) | 15 ns (0 alloc) | 2.71x |
| `pattern: ^\d{4}-\d{2}-\d{2}$` | 7 ns (0 alloc) | 16 ns (0 alloc) | 2.24x |
| `pattern: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$` | 10 ns (0 alloc) | 18 ns (0 alloc) | 1.85x |
| `pattern: ^(small\|medium\|large)$` | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.20x |

| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict | 1.19 µs | 6.30 µs | 5.30x |
| tool calls | 6.11 µs | 27.33 µs | 4.47x |

| one request | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict: compiled, one answer checked | 1.25 µs | 6.42 µs | 5.15x |
| one tool call: compiled, one answer checked | 522 ns | 3.76 µs | 7.20x |
| nested models: compiled, one answer checked | 2.49 µs | 10.10 µs | 4.05x |

Every message of an invalid verdict: flowschema 568 ns, jsonschema 756 ns (1.33x).

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
| compiling every schema | 128.2 ms | 467.5 ms (flowschema 3.65x faster) |
| checking every answer of the 9,779 schemas both check whole | 2.39 ms | 4.34 ms (flowschema 1.82x faster) |

The two agree on 99.93% of the answers (`corpus-disagreements.tsv`). 21 schemas (0.19%) ask something not checked here: `pattern` (19), `$ref outside the schema` (1), `format` (1). flowschema is slower by over 5% on 146 schemas (`corpus-slower.tsv`).

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
