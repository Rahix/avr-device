#!/usr/bin/env bash
# Generate a lookup table function for interrupts of all supported chips
set -e

echo     "// Autogenerated.  Do not edit."
echo     "pub fn lookup_vector(chip: &str, intr: &str) -> Option<usize> {"
echo     "    match chip {"

for intr_path in "$@"; do
    chip="$(basename "${intr_path%%.*}")"
    echo "        \"$chip\" => match intr {"

    svd interrupts --no-gaps $intr_path | awk '{print "            \""substr($2, 1, length($2)-1)"\"" " => Some(" $1"),"}'
    echo "            _ => None,"
    echo "        },"
done

echo     "        _ => None,"
echo     "    }"
echo     "}"
