| answers of MaskBench | answers | instructions, jsonschema's over flowschema's | flowschema runs over 5% fewer on | over 5% more on |
|---|---|---|---|---|
| that match | 13,937 | 1.50x | 12,517 (89.8%) | 703 (5.0%) |
| that do not | 22,979 | 1.50x | 19,830 (86.3%) | 1,683 (7.3%) |

Of the 9,779 schemas both check whole that have answers, flowschema runs over 5% fewer instructions on the answers of 8,819 (90.2%) and over 5% more on those of 430 (4.4%). An answer is one `matches` and one `is_valid`, counted in the processor's instructions: the same on every run of one build, and not a time.
