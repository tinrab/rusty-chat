#!/bin/bash

function lint() {
    cargo fmt --all -- --check && cargo clippy --all-targets --all-features -- -D warnings \
        -D unsafe_code \
        -D missing_copy_implementations \
        -D trivial_casts \
        -D trivial_numeric_casts \
        -D unused_extern_crates \
        -D unused_import_braces \
        -D unused_qualifications \
        -D unreachable_pub
}

function test() {
    cargo test
}

function build() {
    cargo build --release
}

function help() {
    echo "Usage: $(basename "$0") [OPTIONS]

Commands:
  lint   Run lints
  test   Run all tests
  build  Build release binaries
  help   Show help
"
}

if [[ $1 =~ ^(test|build|lint|help)$ ]]; then
    "$@"
else
    echo "Invalid subcommand '$1'" >&2
    exit 1
fi
