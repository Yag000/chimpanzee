#!/bin/bash

ALLOWED_LINTS=("must_use_candidate" "missing-errors-doc" "cast_possible_truncation" "cast_possible_wrap" "missing_panics_doc" "cast_sign_loss" "unused_self")


format_lints(){
    for lint in "${ALLOWED_LINTS[@]}"; do
        printf '%s' "-A" 
        printf " %s " "clippy::$lint"
    done
}


if [ -z "$1" ]; then
    cargo clippy --all -- -W "clippy::pedantic"  $(format_lints)
else
    module=$1
    shift
    cargo clippy -p "$module" "$@" -- -W "clippy::pedantic"  $(format_lints)
fi
