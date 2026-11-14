| answers of MaskBench | answers | instructions, jsonschema's over flowschema's | flowschema runs over 5% fewer on | over 5% more on |
|---|---|---|---|---|
| that match | 13,937 | 1.54x | 12,750 (91.5%) | 575 (4.1%) |
| that do not | 22,979 | 1.54x | 20,375 (88.7%) | 1,397 (6.1%) |

Of the 9,779 schemas both check whole that have answers, flowschema runs over 5% fewer instructions on the answers of 8,967 (91.7%) and over 5% more on those of 344 (3.5%). An answer is one `matches` and one `is_valid`, counted in the processor's instructions: the same on every run of one build, and not a time.
