#!/bin/sh
# The README's chart: results/charts/compare.svg drawn from the results (`charts`), then
# rasterized at twice its size into compare.png by headless Chrome, its rounded corners left
# transparent so the card sits on any page. CHROME names the browser where it is not the
# macOS app's place or on PATH as google-chrome or chromium.
set -eu
cd "$(dirname "$0")"
size=$(cargo run --quiet --release --bin charts)
width=${size% *}
height=${size#* }
chrome=${CHROME:-}
for candidate in "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" google-chrome chromium; do
    [ -n "$chrome" ] && break
    if [ -x "$candidate" ] || command -v "$candidate" > /dev/null 2>&1; then chrome=$candidate; fi
done
[ -n "$chrome" ] || { echo "charts.sh: no Chrome; set CHROME" >&2; exit 1; }
page=$(mktemp -d)
trap 'rm -rf "$page"' EXIT
{ printf '<!doctype html><html><body style="margin:0;background:transparent">'
  cat results/charts/compare.svg
  printf '</body></html>'; } > "$page/compare.html"
"$chrome" --headless --disable-gpu --hide-scrollbars --default-background-color=00000000 \
    --force-device-scale-factor=2 --window-size="${width%.*},${height%.*}" \
    --user-data-dir="$page/profile" --screenshot="$PWD/results/charts/compare.png" \
    "file://$page/compare.html" > /dev/null 2>&1
echo "wrote results/charts/compare.png ($((${width%.*} * 2))×$((${height%.*} * 2)))"
