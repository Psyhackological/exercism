#!/usr/bin/env bash

main() {
	# https://stackoverflow.com/questions/11461625/reverse-the-order-of-characters-in-a-string
	rev <<<"$1" | tac
}

main "$@"
