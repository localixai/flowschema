| answers of MaskBench | answers | instructions, jsonschema's over flowschema's | flowschema runs over 5% fewer on | over 5% more on |
|---|---|---|---|---|
| that match | 13,937 | 1.60x | 13,085 (93.9%) | 333 (2.4%) |
| that do not | 22,979 | 1.58x | 20,488 (89.2%) | 1,134 (4.9%) |

Of the 9,779 schemas both check whole that have answers, flowschema runs over 5% fewer instructions on the answers of 9,173 (93.8%) and over 5% more on those of 205 (2.1%). An answer is one `matches` and one `is_valid`, counted in the processor's instructions: the same on every run of one build, and not a time.
