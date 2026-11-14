| answers of MaskBench | answers | instructions, jsonschema's over flowschema's | flowschema runs over 5% fewer on | over 5% more on |
|---|---|---|---|---|
| that match | 13,937 | 1.56x | 13,017 (93.4%) | 375 (2.7%) |
| that do not | 22,979 | 1.56x | 20,404 (88.8%) | 1,178 (5.1%) |

Of the 9,779 schemas both check whole that have answers, flowschema runs over 5% fewer instructions on the answers of 9,133 (93.4%) and over 5% more on those of 229 (2.3%). An answer is one `matches` and one `is_valid`, counted in the processor's instructions: the same on every run of one build, and not a time.
