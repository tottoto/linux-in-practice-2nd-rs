#!/bin/env bash

set -euo pipefail

PROGNAME=$0
WORK_DIR=$(pwd)
SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)
MEASURE_OPT=""

usage() {
  exec >&2
  echo "Usage: $PROGNAME [-m] <MAX NUMBER OF PROCESSES>

Options:
 -m: enable -m option to multiload.sh"
  exit 1
}

measure() {
  local nproc=$1
  local opt=${2:-}
  bash -c "time ${SCRIPT_DIR}/multiload.sh $opt $nproc" 2>&1 |
    grep real |
    sed -n -e 's/^.*0m\([.0-9]*\)s$/\1/p' |
    awk -v nproc="$nproc" -f "${SCRIPT_DIR}/cpuperf.awk"
}

while getopts "m" OPT; do
  case $OPT in
  m)
    MEASURE_OPT="-m"
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

OUTPUT_FILE="${WORK_DIR}/cpuperf.data"
rm "$OUTPUT_FILE"
MAX_NPROC=$1
for ((i = 1; i <= MAX_NPROC; i++)); do
  measure $i $MEASURE_OPT >>"$OUTPUT_FILE"
done
