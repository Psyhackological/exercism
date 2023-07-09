#!/bin/bash

if [ $# -ne 1 ]; then
	echo "Error: Please provide exactly one argument - the directory path."
	exit 1
elif [ ! -d "$1" ]; then
	echo "Error: The specified directory does not exist."
	exit 2
fi

dir="${1%/}"
echo "Directory exists: $dir"
echo

echo "Exercise: $dir"
cat "$dir/README.md"
echo

echo 'Tests:'
for test_file in "$dir"/tests/*.rs; do
	echo "$test_file"
	cat "$test_file"
	echo
done

if [ -f "$dir/Cargo.toml" ]; then
	echo "Cargo.toml:"
	cat "$dir/Cargo.toml"
	echo
fi

echo "Solutions' files:"
for solution_file in "$dir"/src/*.rs; do
	echo "$solution_file"
	cat "$solution_file"
	echo
done

echo 'clippy, rustfmt, and tests did go through.'
