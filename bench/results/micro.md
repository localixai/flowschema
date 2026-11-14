| check | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict, valid | 98 ns (0 alloc) | 157 ns (0 alloc) | 1.60x |
| review verdict, invalid | 16 ns (0 alloc) | 60 ns (0 alloc) | 3.80x |
| 1,000 rows | 22.23 µs (0 alloc) | 56.15 µs (0 alloc) | 2.53x |
| 500 objects × 20 fields | 23.18 µs (0 alloc) | 174.81 µs (0 alloc) | 7.54x |
| tree of 511 nodes by `$ref` | 4.46 µs (0 alloc) | 11.54 µs (7 alloc) | 2.59x |
| 64 tool calls, `oneOf` of 8 | 1.46 µs (0 alloc) | 2.89 µs (0 alloc) | 1.98x |
| `uniqueItems`, 1,000 strings | 3.24 µs (1 alloc) | 5.07 µs (1 alloc) | 1.56x |
| `enum` of 1,000, the last | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.31x |
| `format: date-time` | 7 ns (0 alloc) | 10 ns (0 alloc) | 1.51x |
| `format: email` | 22 ns (0 alloc) | 87 ns (1 alloc) | 3.94x |
| `format: uuid` | 4 ns (0 alloc) | 4 ns (0 alloc) | 1.25x |
| `format: ipv4` | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.47x |
| `format: uri` | 14 ns (0 alloc) | 22 ns (0 alloc) | 1.58x |
| `format: regex`, `^[a-z]+\d$` | 58 ns (0 alloc) | 66 ns (3 alloc) | 1.13x |
| `format: regex`, `^\p{L}+$` | 52 ns (0 alloc) | 61 ns (3 alloc) | 1.18x |
| `format: regex`, `^\P{ASCII}+$` | 57 ns (0 alloc) | 67 ns (3 alloc) | 1.17x |
| `format: regex`, `[\P{Letter}]` | 49 ns (0 alloc) | 67 ns (3 alloc) | 1.36x |
| `format: regex`, `(?=a)b` | 56 ns (0 alloc) | 88 ns (4 alloc) | 1.57x |
| `format: regex`, `(a)\1` | 61 ns (0 alloc) | 88 ns (4 alloc) | 1.45x |
| `format: regex`, `(?<n>a)\k<n>` | 79 ns (0 alloc) | 137 ns (7 alloc) | 1.73x |
| `pattern: ^[A-Z]{3}$` | 6 ns (0 alloc) | 15 ns (0 alloc) | 2.68x |
| `pattern: ^\d{4}-\d{2}-\d{2}$` | 8 ns (0 alloc) | 17 ns (0 alloc) | 2.22x |
| `pattern: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$` | 9 ns (0 alloc) | 19 ns (0 alloc) | 2.01x |
| `pattern: ^(small\|medium\|large)$` | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.12x |

| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict | 1.21 µs | 6.76 µs | 5.61x |
| tool calls | 6.27 µs | 27.17 µs | 4.33x |

| one request | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict: compiled, one answer checked | 1.26 µs | 6.50 µs | 5.18x |
| one tool call: compiled, one answer checked | 516 ns | 3.98 µs | 7.71x |
| nested models: compiled, one answer checked | 2.67 µs | 10.47 µs | 3.92x |

Every message of an invalid verdict: flowschema 609 ns, jsonschema 758 ns (1.25x).
