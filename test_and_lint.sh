#!/bin/bash
#
if [ -z "$1" ]; then
    cargo test --all 
else
    if [ "$1" == "--release" ]; then
        shift
        cargo test --all --release "$@"
        ./run_linter.sh --all --release "$@"
        exit
    fi
    module=$1
    shift
    cargo test -p "$module" "$@" 
    ./run_linter.sh "$module" "$@"
fi


