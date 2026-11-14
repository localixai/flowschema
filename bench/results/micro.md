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
