| check | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict, valid | 96 ns (0 alloc) | 174 ns (0 alloc) | 1.81x |
| review verdict, invalid | 16 ns (0 alloc) | 36 ns (0 alloc) | 2.26x |
| 1,000 rows | 21.62 µs (0 alloc) | 49.64 µs (0 alloc) | 2.30x |
| 500 objects × 20 fields | 27.24 µs (0 alloc) | 177.81 µs (0 alloc) | 6.53x |
| tree of 511 nodes by `$ref` | 4.55 µs (0 alloc) | 12.45 µs (7 alloc) | 2.73x |
| 64 tool calls, `oneOf` of 8 | 1.60 µs (0 alloc) | 3.07 µs (0 alloc) | 1.91x |
| `uniqueItems`, 1,000 strings | 3.14 µs (1 alloc) | 5.07 µs (1 alloc) | 1.61x |
| `enum` of 1,000, the last | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.23x |
| `format: date-time` | 7 ns (0 alloc) | 10 ns (0 alloc) | 1.51x |
| `format: email` | 21 ns (0 alloc) | 91 ns (1 alloc) | 4.28x |
| `format: uuid` | 3 ns (0 alloc) | 5 ns (0 alloc) | 1.33x |
| `format: ipv4` | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.40x |
| `format: uri` | 14 ns (0 alloc) | 23 ns (0 alloc) | 1.62x |
| `format: regex`, `^[a-z]+\d$` | 57 ns (0 alloc) | 67 ns (3 alloc) | 1.17x |
| `format: regex`, `^\p{L}+$` | 53 ns (0 alloc) | 66 ns (3 alloc) | 1.25x |
| `format: regex`, `^\P{ASCII}+$` | 57 ns (0 alloc) | 69 ns (3 alloc) | 1.21x |
| `format: regex`, `[\P{Letter}]` | 49 ns (0 alloc) | 73 ns (3 alloc) | 1.49x |
| `format: regex`, `(?=a)b` | 57 ns (0 alloc) | 90 ns (4 alloc) | 1.57x |
| `format: regex`, `(a)\1` | 60 ns (0 alloc) | 87 ns (4 alloc) | 1.45x |
| `format: regex`, `(?<n>a)\k<n>` | 76 ns (0 alloc) | 138 ns (7 alloc) | 1.80x |
| `pattern: ^[A-Z]{3}$` | 6 ns (0 alloc) | 16 ns (0 alloc) | 2.71x |
| `pattern: ^\d{4}-\d{2}-\d{2}$` | 7 ns (0 alloc) | 17 ns (0 alloc) | 2.24x |
| `pattern: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$` | 9 ns (0 alloc) | 18 ns (0 alloc) | 2.03x |
| `pattern: ^(small\|medium\|large)$` | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.24x |

| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict | 1.28 µs | 6.88 µs | 5.38x |
| tool calls | 6.46 µs | 28.77 µs | 4.46x |

| one request | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict: compiled, one answer checked | 1.27 µs | 6.62 µs | 5.21x |
| one tool call: compiled, one answer checked | 533 ns | 4.16 µs | 7.80x |
| nested models: compiled, one answer checked | 2.74 µs | 11.01 µs | 4.02x |

Every message of an invalid verdict: flowschema 643 ns, jsonschema 828 ns (1.29x).
