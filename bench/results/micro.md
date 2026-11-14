| check | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict, valid | 88 ns (0 alloc) | 158 ns (0 alloc) | 1.78x |
| review verdict, invalid | 15 ns (0 alloc) | 32 ns (0 alloc) | 2.18x |
| 1,000 rows | 21.01 µs (0 alloc) | 43.46 µs (0 alloc) | 2.07x |
| 500 objects × 20 fields | 23.69 µs (0 alloc) | 166.00 µs (0 alloc) | 7.01x |
| tree of 511 nodes by `$ref` | 4.44 µs (0 alloc) | 10.53 µs (7 alloc) | 2.37x |
| 64 tool calls, `oneOf` of 8 | 1.41 µs (0 alloc) | 2.93 µs (0 alloc) | 2.08x |
| `uniqueItems`, 1,000 strings | 2.85 µs (1 alloc) | 4.88 µs (1 alloc) | 1.71x |
| `enum` of 1,000, the last | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.25x |
| `format: date-time` | 7 ns (0 alloc) | 10 ns (0 alloc) | 1.48x |
| `format: email` | 21 ns (0 alloc) | 83 ns (1 alloc) | 3.97x |
| `format: uuid` | 3 ns (0 alloc) | 4 ns (0 alloc) | 1.25x |
| `format: ipv4` | 5 ns (0 alloc) | 7 ns (0 alloc) | 1.42x |
| `format: uri` | 14 ns (0 alloc) | 22 ns (0 alloc) | 1.64x |
| `format: regex`, `^[a-z]+\d$` | 57 ns (0 alloc) | 63 ns (3 alloc) | 1.12x |
| `format: regex`, `^\p{L}+$` | 49 ns (0 alloc) | 58 ns (3 alloc) | 1.19x |
| `format: regex`, `^\P{ASCII}+$` | 56 ns (0 alloc) | 63 ns (3 alloc) | 1.13x |
| `format: regex`, `[\P{Letter}]` | 48 ns (0 alloc) | 66 ns (3 alloc) | 1.38x |
| `format: regex`, `(?=a)b` | 54 ns (0 alloc) | 86 ns (4 alloc) | 1.59x |
| `format: regex`, `(a)\1` | 58 ns (0 alloc) | 83 ns (4 alloc) | 1.45x |
| `format: regex`, `(?<n>a)\k<n>` | 73 ns (0 alloc) | 130 ns (7 alloc) | 1.78x |
| `pattern: ^[A-Z]{3}$` | 5 ns (0 alloc) | 15 ns (0 alloc) | 3.01x |
| `pattern: ^\d{4}-\d{2}-\d{2}$` | 8 ns (0 alloc) | 16 ns (0 alloc) | 2.10x |
| `pattern: ^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$` | 10 ns (0 alloc) | 18 ns (0 alloc) | 1.80x |
| `pattern: ^(small\|medium\|large)$` | 5 ns (0 alloc) | 6 ns (0 alloc) | 1.14x |

| compiled once | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict | 1.11 µs | 6.36 µs | 5.71x |
| tool calls | 5.84 µs | 26.59 µs | 4.55x |

| one request | flowschema | jsonschema 0.60 | flowschema faster by |
|---|---|---|---|
| review verdict: compiled, one answer checked | 1.16 µs | 6.31 µs | 5.43x |
| one tool call: compiled, one answer checked | 508 ns | 3.89 µs | 7.65x |
| nested models: compiled, one answer checked | 2.46 µs | 10.34 µs | 4.20x |

Every message of an invalid verdict: flowschema 577 ns, jsonschema 704 ns (1.22x).
