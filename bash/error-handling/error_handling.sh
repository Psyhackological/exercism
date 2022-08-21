#!/usr/bin/env bash

main() {
	# number of arguments not equal to 1
	if [[ "$#" -ne 1 ]]; then
		# otherwise print an error message and exit with a non-zero status.
		# $0 - name of the script
		echo "Usage: $0 <person>"
		exit 1
	else
		echo "Hello, $1"
	fi

}

# call main with all of the positional arguments
main "$@"
