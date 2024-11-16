#!/usr/bin/env bash

is_leap_year() {
	# int regex
	re='^[0-9]+$'

	# if arguments not equal to 1
	# or regex fails which means this is not a integer
	# https://www.codegrepper.com/code-examples/shell/bash+check+if+argument+is+integer
	if [[ "$#" -ne 1 || ! "$1" =~ $re ]]; then
		echo "Usage: leap.sh <year>"
		# -1 error code
		exit 1
	fi

	# variable to make it easier to read
	year="$1"

	# divisible by 4
	# and
	# not divisible by 100
	# or
	# divisible by 400
	if [[ "$((year % 4))" -eq 0 && "$((year % 100))" -ne 0 || "$((year % 400))" == 0 ]]; then
		echo "true"
	else
		echo "false"
	fi
}

is_leap_year "$@"
