#!/bin/bash

projects=(
    "first-rust-proj"
    "next-rust-proj"
    "last-rust-prof/level2/rust-proj"
)

for dir in "${projects[@]}"; do
    echo "building $dir"
    if ! (cd "$dir" && cargo build); then
    echo "error in "$dir"
    fi
    done
    echo "all done"