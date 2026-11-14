#!/bin/sh
# The session the site's hero plays: the install, the program, its run. Typed, then run for real.
say() {
  printf '\033[1;32m$\033[0m '
  i=1
  while [ "$i" -le "${#1}" ]; do
    printf '%s' "$(printf '%s' "$1" | cut -c"$i")"
    sleep 0.035
    i=$((i + 1))
  done
  sleep 0.35
  printf '\n'
}
run() { say "$1"; sh -c "$1"; sleep "${2:-1.2}"; }
sleep 0.4
run 'cargo add flowschema --git https://github.com/localixai/flowschema' 1.4
clear
run 'cat src/main.rs' 3.5
clear
run 'cargo run --quiet' 4
