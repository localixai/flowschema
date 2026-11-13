#!/bin/sh
# results/latest.md: what the last run measured and on what (the date, the commit, the machine,
# the compiler), then each benchmark's tables. run.sh calls it after the benchmarks.
set -eu
cd "$(dirname "$0")"
machine=$( (sysctl -n machdep.cpu.brand_string 2>/dev/null || uname -m) | tr -s ' ')
system=$( (sw_vers -productName && sw_vers -productVersion) 2>/dev/null | paste -sd ' ' -)
[ -n "$system" ] || system=$(uname -sr)
commit=$(git describe --always --dirty=' with changes not yet committed' 2>/dev/null || echo '?')
{
    echo "# The latest run"
    echo
    echo "$(date +%Y-%m-%d), flowschema $commit, on $machine ($system)," \
        "$(rustc --version | cut -d' ' -f1-2), against jsonschema 0.60."
    echo "Each check's time is the median of 31 rounds; allocations are counted exactly."
    for part in micro suite corpus size; do
        echo
        case $part in
            micro) echo "## Typical checks" ;;
            suite) echo "## The official test suite" ;;
            corpus) echo "## MaskBench" ;;
            size) echo "## Size" ;;
        esac
        echo
        cat "results/$part.md"
    done
} > results/latest.md
