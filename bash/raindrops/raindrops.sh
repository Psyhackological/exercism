#!/usr/bin/env bash

main() {
	result=""
	number=$1
	if [[ $((number % 3)) -eq 0 ]]; then
		result="${result}Pling"
	fi

	if [[ $((number % 5)) -eq 0 ]]; then
		result="${result}Plang"
	fi

	if [[ $((number % 7)) -eq 0 ]]; then
		result="${result}Plong"
	fi

	if [[ -z "$result" ]]; then
		echo "$number"
	else
		echo "$result"
	fi
}

main "$@"
