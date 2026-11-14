| | flowschema | jsonschema 0.60 |
|---|---|---|
| schemas compiled | 11,306 of 11,306 (0 panics) | 11,298 (8 refused) |
| answers judged as labelled | 36,984 of 36,990 (99.98%) | 36,970 (99.95%) |
| compiling every schema | 123.8 ms | 467.2 ms (flowschema 3.77x faster) |
| checking every answer of the 9,779 schemas both check whole | 3.10 ms | 4.25 ms (flowschema 1.37x faster) |

The two agree on 99.93% of the answers (`corpus-disagreements.tsv`). 21 schemas (0.19%) ask something not checked here: `pattern` (19), `$ref outside the schema` (1), `format` (1). flowschema is slower by over 5% on 1224 schemas (`corpus-slower.tsv`).
