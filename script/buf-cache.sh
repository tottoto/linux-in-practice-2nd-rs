#!/bin/env bash

set -eu

OUTPUT_FILE="$(pwd)/testfile"

echo "Shows the total system memory usage before a file is created."
echo ""
free

echo ""
echo "Creates a new 1GB file, which will cause the kernel to allocate 1GB of page cache space in memory."
echo ""
dd if=/dev/zero of="${OUTPUT_FILE}" bs=1M count=1K

echo ""
echo "Shows the total system memory usage after the file is created."
echo ""
free

echo ""
echo "Displays the total system memory usage after deleting the file, that is deleting the page cache."
echo ""
rm "${OUTPUT_FILE}"
free
