#!/bin/bash

ALLOWED_LINTS=("must_use_candidate" "missing-errors-doc" "cast_possible_truncation" "cast_possible_wrap" "missing_panics_doc" "cast_sign_loss" "unused_self" "module-name-repetitions" "mutable_key_type")


format_lints(){
    for lint in "${ALLOWED_LINTS[@]}"; do
        printf '%s' "-A"
        printf " %s " "clippy::$lint"
    done
}


cargo clippy "$@" -- -W "clippy::pedantic" -D warnings $(format_lints)
