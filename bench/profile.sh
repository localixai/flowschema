#!/bin/sh
# Where the time goes: one part of the quick loop run over and over (`quick/`, `--spin`), watched by
# macOS's own sampler for four seconds, and the functions it was found in most, by their own time.
#
#   ./profile.sh compile agents      # also: check, messages; documents
#   ./profile.sh fixture "64 tool calls, \`oneOf\` of 8"   # one check of micro's
set -eu
cd "$(dirname "$0")"
what=${1:-compile}
set_=${2:-agents}
cargo build --quiet --release --manifest-path quick/Cargo.toml
if [ "$what" = fixture ]; then
    quick/target/release/flowschema-quick --spin fixture --fixture "$set_" &
else
    quick/target/release/flowschema-quick --spin "$what" "$set_" &
fi
pid=$!
sleep 1.5
out=$(mktemp)
sample "$pid" 4 -mayDie -file "$out" > /dev/null 2>&1 || true
wait "$pid" || true
# The sampler's own summary: each function by the samples where it was on top of the stack.
awk '/^Sort by top of stack/ { on = 1; next } on && NF { print } /^Binary Images/ { exit }' "$out" \
    | grep -v '^Binary Images' | head -40
rm -f "$out"
