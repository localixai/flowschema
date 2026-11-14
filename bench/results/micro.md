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
