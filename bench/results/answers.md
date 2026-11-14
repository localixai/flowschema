| answers of MaskBench | answers | instructions, jsonschema's over flowschema's | flowschema runs over 5% fewer on | over 5% more on |
|---|---|---|---|---|
| that match | 13,937 | 1.32x | 10,577 (75.9%) | 1,962 (14.1%) |
| that do not | 22,979 | 1.28x | 14,702 (64.0%) | 5,125 (22.3%) |

Of the 9,779 schemas both check whole that have answers, flowschema runs over 5% fewer instructions on the answers of 7,288 (74.5%) and over 5% more on those of 1,478 (15.1%). An answer is one `matches` and one `is_valid`, counted in the processor's instructions: the same on every run of one build, and not a time.
