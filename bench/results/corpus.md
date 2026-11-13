| | flowschema | jsonschema 0.60 |
|---|---|---|
| schemas compiled | 11,306 of 11,306 (0 panics) | 11,298 (8 refused) |
| answers judged as labelled | 36,984 of 36,990 (99.98%) | 36,970 (99.95%) |
| compiling every schema | 520.8 ms | 1059.7 ms (flowschema 2.03x faster) |
| checking every answer of the 9,779 schemas both check whole | 8.73 ms | 12.47 ms (flowschema 1.43x faster) |

The two agree on 99.93% of the answers (`corpus-disagreements.tsv`). 21 schemas (0.19%) ask something not checked here: `pattern` (19), `$ref outside the schema` (1), `format` (1). flowschema is slower by over 5% on 350 schemas (`corpus-slower.tsv`).
