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
