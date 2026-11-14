# The latest run

2026-11-14, flowschema 437f72a with changes not yet committed, on Apple M5 (macOS 26.6.2), rustc 1.100.0, against jsonschema 0.60.
Each check's time is the median of 31 rounds; allocations are counted exactly.

## Typical checks

| check | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict, valid | 119 ns (0 alloc) | 166 ns (0 alloc) | 1.39x |
| review verdict, invalid | 17 ns (0 alloc) | 43 ns (0 alloc) | 2.55x |
| 1,000 rows | 29.37 µs (0 alloc) | 54.10 µs (0 alloc) | 1.84x |
| 500 objects × 20 fields | 25.43 µs (0 alloc) | 169.62 µs (0 alloc) | 6.67x |
| tree of 511 nodes by `$ref` | 5.96 µs (0 alloc) | 11.58 µs (7 alloc) | 1.94x |
| 64 tool calls, `oneOf` of 8 | 1.87 µs (0 alloc) | 2.92 µs (0 alloc) | 1.56x |
| `uniqueItems`, 1,000 strings | 2.89 µs (1 alloc) | 5.06 µs (1 alloc) | 1.75x |
| `enum` of 1,000, the last | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.34x |
| `format: date-time` | 7 ns (0 alloc) | 11 ns (0 alloc) | 1.42x |
| `format: email` | 23 ns (0 alloc) | 86 ns (1 alloc) | 3.81x |
| `format: uuid` | 4 ns (0 alloc) | 4 ns (0 alloc) | 1.11x |
| `format: ipv4` | 6 ns (0 alloc) | 7 ns (0 alloc) | 1.13x |
| `format: uri` | 15 ns (0 alloc) | 21 ns (0 alloc) | 1.38x |
| `format: regex`, `^[a-z]+\d$` | 55 ns (0 alloc) | 65 ns (3 alloc) | 1.17x |
| `format: regex`, `^\p{L}+$` | 50 ns (0 alloc) | 57 ns (3 alloc) | 1.15x |
| `format: regex`, `^\P{ASCII}+$` | 55 ns (0 alloc) | 65 ns (3 alloc) | 1.20x |
| `format: regex`, `[\P{Letter}]` | 48 ns (0 alloc) | 65 ns (3 alloc) | 1.36x |
| `format: regex`, `(?=a)b` | 54 ns (0 alloc) | 84 ns (4 alloc) | 1.57x |
| `format: regex`, `(a)\1` | 59 ns (0 alloc) | 84 ns (4 alloc) | 1.44x |
| `format: regex`, `(?<n>a)\k<n>` | 75 ns (0 alloc) | 130 ns (7 alloc) | 1.74x |
| `pattern: ^[A-Z]{3}$` | 7 ns (0 alloc) | 14 ns (0 alloc) | 2.15x |
| `pattern: ^\d{4}-\d{2}-\d{2}$` | 8 ns (0 alloc) | 16 ns (0 alloc) | 2.09x |
| `pattern: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$` | 10 ns (0 alloc) | 18 ns (0 alloc) | 1.85x |
| `pattern: ^(small\|medium\|large)$` | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.17x |

| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict | 1.19 µs | 6.60 µs | 5.55x |
| tool calls | 6.10 µs | 27.07 µs | 4.44x |

| one request | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict: compiled, one answer checked | 1.17 µs | 6.28 µs | 5.39x |
| one tool call: compiled, one answer checked | 493 ns | 3.89 µs | 7.89x |
| nested models: compiled, one answer checked | 2.33 µs | 10.53 µs | 4.52x |

Every message of an invalid verdict: flowschema 616 ns, jsonschema 763 ns (1.24x).

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
| compiling every schema | 123.8 ms | 467.2 ms (flowschema 3.77x faster) |
| checking every answer of the 9,779 schemas both check whole | 3.10 ms | 4.25 ms (flowschema 1.37x faster) |

The two agree on 99.93% of the answers (`corpus-disagreements.tsv`). 21 schemas (0.19%) ask something not checked here: `pattern` (19), `$ref outside the schema` (1), `format` (1). flowschema is slower by over 5% on 1224 schemas (`corpus-slower.tsv`).

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
