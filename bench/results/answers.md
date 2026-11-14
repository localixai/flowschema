| answers of MaskBench | answers | instructions, jsonschema's over flowschema's | flowschema runs over 5% fewer on | over 5% more on |
|---|---|---|---|---|
| that match | 13,937 | 1.64x | 13,353 (95.8%) | 238 (1.7%) |
| that do not | 22,979 | 1.62x | 21,072 (91.7%) | 960 (4.2%) |

Of the 9,779 schemas both check whole that have answers, flowschema runs over 5% fewer instructions on the answers of 9,407 (96.2%) and over 5% more on those of 147 (1.5%). An answer is one `matches` and one `is_valid`, counted in the processor's instructions: the same on every run of one build, and not a time.
