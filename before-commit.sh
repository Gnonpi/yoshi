#!/usr/bin/env bash

# Temporary checks before setting a true CI/CD

set -e

cargoSubcommand() {
    echo "> Running 'cargo"${1}"'"
    cargo ${1}
}

cargoSubcommand check
cargoSubcommand test
cargoSubcommand clippy
cargoSubcommand fmt

echo "> Done running checks, you can commit!"
