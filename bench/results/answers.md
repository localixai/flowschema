| answers of MaskBench | answers | instructions, jsonschema's over flowschema's | flowschema runs over 5% fewer on | over 5% more on |
|---|---|---|---|---|
| that match | 13,937 | 1.43x | 11,988 (86.0%) | 1,002 (7.2%) |
| that do not | 22,979 | 1.40x | 18,222 (79.3%) | 2,739 (11.9%) |

Of the 9,779 schemas both check whole that have answers, flowschema runs over 5% fewer instructions on the answers of 8,392 (85.8%) and over 5% more on those of 708 (7.2%). An answer is one `matches` and one `is_valid`, counted in the processor's instructions: the same on every run of one build, and not a time.
