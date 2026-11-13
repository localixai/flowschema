# Security

flowschema checks what a model wrote against a schema that may come from anywhere: a tool's
definition, a user, another model. Neither may make the program that checks them do anything but
answer. It fetches nothing (a `$ref` reaches only the documents it is given), runs every pattern in
time linear in the string (no matcher backtracks), cuts a `$ref` loop that never steps into the
data, and holds no `unsafe` code (`#![forbid(unsafe_code)]`). Its lints forbid `unwrap`, `expect`,
`panic!` and slicing a string inside a character in working code, and the benchmarks check that
no schema of the official test suite or of MaskBench makes it panic. A panic on any input is a bug,
reported as below.

## Reporting a problem

Report it privately, not in an issue:
[open a security advisory](https://github.com/localixai/flowschema/security/advisories/new).
Say what schema and data you checked, and what it did. The latest release is the one that gets the
fix.
