#!/bin/sh
# What each validator adds to a release binary built for size: size/ built with no validator, with
# flowschema, with jsonschema, and with regex-lite alone (the part of flowschema's size that is its
# dependency), the baseline subtracted. Writes results/size.{json,md}.
set -eu
cd "$(dirname "$0")"
mkdir -p results
built() { # <features> -> bytes of the stripped binary
    cargo build --quiet --release --manifest-path size/Cargo.toml --target-dir size/target \
        ${1:+--features "$1"}
    wc -c < size/target/release/flowschema-size | tr -d ' '
}
none=$(built "")
ours=$(built flowschema)
theirs=$(built jsonschema)
regex=$(built regex-lite)
added_ours=$((ours - none))
added_theirs=$((theirs - none))
added_regex=$((regex - none))
own=$((added_ours - added_regex))
kb() { # bytes -> "1,884 KB"
    awk -v b="$1" 'BEGIN {
        n = sprintf("%.0f", b / 1024); s = ""
        while (length(n) > 3) { s = "," substr(n, length(n) - 2) s; n = substr(n, 1, length(n) - 3) }
        printf "%s%s KB", n, s }'
}
printf '{\n  "baseline": %s,\n  "flowschema": %s,\n  "regexLite": %s,\n  "jsonschema": %s\n}\n' \
    "$none" "$added_ours" "$added_regex" "$added_theirs" > results/size.json
{
    echo "| added to a release binary built for size | size |"
    echo "|---|---|"
    echo "| flowschema, all of it | $(kb "$added_ours") |"
    echo "| of which its own code | $(kb "$own") |"
    echo "| of which regex-lite | $(kb "$added_regex") |"
    echo "| jsonschema 0.60 | $(kb "$added_theirs") |"
    echo
    echo "Over a program that reads the JSON alone ($(kb "$none")), stripped, \`opt-level = \"z\"\`," \
        "fat LTO. regex-lite is measured alone, as a program that uses it pays for it."
} > results/size.md
cat results/size.md
