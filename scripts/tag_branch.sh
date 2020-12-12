#!/bin/bash
set -e

TOML_VERSION=$(cat ./Cargo.toml | grep version | head -1 | grep -oP '\d{1,2}\.\d{1,2}\.\d{1,2}')
echo ${TOML_VERSION}
echo ${TRAVIS_COMMIT_MESSAGE}
# git tag -a ${TOML_VERSION} -m ${TRAVIS_COMMIT_MESSAGE}
