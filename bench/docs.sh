#!/bin/sh
# The tables of docs/ between `<!-- bench:<name> -->` and `<!-- /bench:<name> -->`, written again
# from results/<name>.md, so no figure there is typed by hand. run.sh calls it after a run; CI runs
# it on the committed results and fails when a page changes (a table not refreshed).
set -eu
cd "$(dirname "$0")"
for page in ../docs/*.md; do
    awk -v dir=results '
        /^<!-- bench:[a-z-]+ -->$/ {
            print; name = $0; sub(/^<!-- bench:/, "", name); sub(/ -->$/, "", name)
            file = dir "/" name ".md"
            if ((getline line < file) <= 0) { print "docs.sh: no " file > "/dev/stderr"; exit 1 }
            print ""; print line
            while ((getline line < file) > 0) print line
            close(file); print ""; skip = 1; next
        }
        /^<!-- \/bench:[a-z-]+ -->$/ { skip = 0; print; next }
        !skip { print }' "$page" > "$page.new"
    mv "$page.new" "$page"
done
