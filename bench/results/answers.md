| answers of MaskBench | answers | instructions, jsonschema's over flowschema's | flowschema runs over 5% fewer on | over 5% more on |
|---|---|---|---|---|
| that match | 13,937 | 1.48x | 12,380 (88.8%) | 772 (5.5%) |
| that do not | 22,979 | 1.46x | 19,401 (84.4%) | 1,960 (8.5%) |

Of the 9,779 schemas both check whole that have answers, flowschema runs over 5% fewer instructions on the answers of 8,735 (89.3%) and over 5% more on those of 472 (4.8%). An answer is one `matches` and one `is_valid`, counted in the processor's instructions: the same on every run of one build, and not a time.
