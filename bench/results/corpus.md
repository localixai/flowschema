| | flowschema | jsonschema 0.60 |
|---|---|---|
| schemas compiled | 11,306 of 11,306 (0 panics) | 11,298 (8 refused) |
| answers judged as labelled | 36,984 of 36,990 (99.98%) | 36,970 (99.95%) |
| compiling every schema | 125.4 ms | 461.8 ms (flowschema 3.68x faster) |
| checking every answer of the 9,779 schemas both check whole | 2.74 ms | 4.23 ms (flowschema 1.55x faster) |

The two agree on 99.93% of the answers (`corpus-disagreements.tsv`). 21 schemas (0.19%) ask something not checked here: `pattern` (19), `$ref outside the schema` (1), `format` (1). flowschema is slower by over 5% on 508 schemas (`corpus-slower.tsv`).
