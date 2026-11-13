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
