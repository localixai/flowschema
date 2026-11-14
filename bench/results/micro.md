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
