| answers of MaskBench | answers | instructions, jsonschema's over flowschema's | flowschema runs over 5% fewer on | over 5% more on |
|---|---|---|---|---|
| that match | 13,937 | 1.32x | 10,591 (76.0%) | 1,951 (14.0%) |
| that do not | 22,979 | 1.29x | 14,717 (64.0%) | 5,104 (22.2%) |

Of the 9,779 schemas both check whole that have answers, flowschema runs over 5% fewer instructions on the answers of 7,295 (74.6%) and over 5% more on those of 1,472 (15.1%). An answer is one `matches` and one `is_valid`, counted in the processor's instructions: the same on every run of one build, and not a time.
