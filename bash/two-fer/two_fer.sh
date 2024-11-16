#!/usr/bin/env bash

main() {
	# if 0 arguments were given
	if [[ "$#" -eq 0 ]]; then
		echo "One for you, one for me."
	# if 1 arguments were given or has_space (the argument was passed with by using "")
	# https://stackoverflow.com/questions/1473981/how-to-check-if-a-string-has-spaces-in-bash-shell
	elif [[ "$#" -eq 1 ]] || [[ "$1" =~ \  ]]; then
		echo "One for $1, one for me."
	# if greater than 1 arguments without a space were given
	else
		echo "One for $*, one for me."
	fi
}

# call main with all of the positional arguments
main "$@"
