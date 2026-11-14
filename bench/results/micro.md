| check | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict, valid | 93 ns (0 alloc) | 162 ns (0 alloc) | 1.74x |
| review verdict, invalid | 16 ns (0 alloc) | 52 ns (0 alloc) | 3.34x |
| 1,000 rows | 22.35 µs (0 alloc) | 52.65 µs (0 alloc) | 2.36x |
| 500 objects × 20 fields | 23.68 µs (0 alloc) | 169.00 µs (0 alloc) | 7.14x |
| tree of 511 nodes by `$ref` | 5.45 µs (0 alloc) | 13.33 µs (7 alloc) | 2.45x |
| 64 tool calls, `oneOf` of 8 | 1.77 µs (0 alloc) | 3.17 µs (0 alloc) | 1.79x |
| `uniqueItems`, 1,000 strings | 3.28 µs (1 alloc) | 5.59 µs (1 alloc) | 1.71x |
| `enum` of 1,000, the last | 6 ns (0 alloc) | 8 ns (0 alloc) | 1.44x |
| `format: date-time` | 8 ns (0 alloc) | 12 ns (0 alloc) | 1.61x |
| `format: email` | 24 ns (0 alloc) | 95 ns (1 alloc) | 4.00x |
| `format: uuid` | 4 ns (0 alloc) | 5 ns (0 alloc) | 1.25x |
| `format: ipv4` | 5 ns (0 alloc) | 8 ns (0 alloc) | 1.40x |
| `format: uri` | 15 ns (0 alloc) | 23 ns (0 alloc) | 1.57x |
| `format: regex`, `^[a-z]+\d$` | 60 ns (0 alloc) | 71 ns (3 alloc) | 1.18x |
| `format: regex`, `^\p{L}+$` | 54 ns (0 alloc) | 67 ns (3 alloc) | 1.23x |
| `format: regex`, `^\P{ASCII}+$` | 60 ns (0 alloc) | 70 ns (3 alloc) | 1.18x |
| `format: regex`, `[\P{Letter}]` | 52 ns (0 alloc) | 83 ns (3 alloc) | 1.60x |
| `format: regex`, `(?=a)b` | 60 ns (0 alloc) | 113 ns (4 alloc) | 1.87x |
| `format: regex`, `(a)\1` | 62 ns (0 alloc) | 94 ns (4 alloc) | 1.50x |
| `format: regex`, `(?<n>a)\k<n>` | 80 ns (0 alloc) | 141 ns (7 alloc) | 1.76x |
| `pattern: ^[A-Z]{3}$` | 6 ns (0 alloc) | 16 ns (0 alloc) | 2.77x |
| `pattern: ^\d{4}-\d{2}-\d{2}$` | 8 ns (0 alloc) | 18 ns (0 alloc) | 2.25x |
| `pattern: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$` | 10 ns (0 alloc) | 20 ns (0 alloc) | 2.01x |
| `pattern: ^(small\|medium\|large)$` | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.37x |

| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict | 1.33 µs | 6.99 µs | 5.25x |
| tool calls | 6.62 µs | 31.76 µs | 4.79x |

| one request | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict: compiled, one answer checked | 1.49 µs | 7.04 µs | 4.73x |
| one tool call: compiled, one answer checked | 569 ns | 4.32 µs | 7.60x |
| nested models: compiled, one answer checked | 2.54 µs | 11.06 µs | 4.36x |

Every message of an invalid verdict: flowschema 619 ns, jsonschema 843 ns (1.36x).
