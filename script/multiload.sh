#!/bin/env bash

set -eu

MULTICPU=0
PROGNAME=$0

usage() {
  exec >&2
  echo "Usage: $PROGNAME [-m] <NUMBER OF PROCESSES>

Options:
 -m: enable multiple CPU mode"
  exit 1
}

while getopts "m" OPT; do
  case $OPT in
  m)
    MULTICPU=1
    ;;
  \?)
    usage
    ;;
  esac
done

shift $((OPTIND - 1))

if [ $# -lt 1 ]; then
  usage
fi

CONCURRENCY=$1

if [ $MULTICPU -eq 0 ]; then
  taskset -p -c 0 $$ >/dev/null
fi

cargo build --quiet --bin load

for ((i = 0; i < CONCURRENCY; i++)); do
  time cargo run --quiet --bin load &
done

for ((i = 0; i < CONCURRENCY; i++)); do
  wait
done
