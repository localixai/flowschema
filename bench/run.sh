#!/bin/sh
# Every benchmark, in one run: the corpora fetched if they are missing, then micro, suite, corpus,
# keywords and size, each writing its results/; then results/latest.md (latest.sh), the tables of
# docs/ (docs.sh), the README's chart (charts.sh), and the run held to budgets.json. A figure the
# README gives is copied from results/ by hand; nothing else is. Nothing else should run meanwhile.
set -eu
cd "$(dirname "$0")"
[ -d data/suite/.git ] && [ -d data/jsonschemabench/.git ] || ./fetch.sh
cargo build --quiet --release --bins
for bench in micro suite corpus answers keywords; do
    cargo run --quiet --release --bin "$bench" > /dev/null
done
./size.sh > /dev/null
./latest.sh
./docs.sh
./charts.sh
cargo run --quiet --release --bin check
