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
