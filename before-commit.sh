#!/usr/bin/env bash

# Temporary checks before setting a true CI/CD

set -e

export RUST_LOG=debug

cargoSubcommand() {
    echo "> Running 'cargo "${1}"'"
    cargo ${1}
}

cargoSubcommand build
cargoSubcommand check
cargoSubcommand test
cargoSubcommand clippy
cargoSubcommand fmt

echo "> Done running checks, you can commit!"
