| answers of MaskBench | answers | instructions, jsonschema's over flowschema's | flowschema runs over 5% fewer on | over 5% more on |
|---|---|---|---|---|
| that match | 13,937 | 1.55x | 12,856 (92.2%) | 483 (3.5%) |
| that do not | 22,979 | 1.55x | 20,307 (88.4%) | 1,296 (5.6%) |

Of the 9,779 schemas both check whole that have answers, flowschema runs over 5% fewer instructions on the answers of 9,056 (92.6%) and over 5% more on those of 280 (2.9%). An answer is one `matches` and one `is_valid`, counted in the processor's instructions: the same on every run of one build, and not a time.
