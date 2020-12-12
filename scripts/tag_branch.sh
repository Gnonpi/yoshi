#!/bin/bash
set -e

git config credential.helper "store --file=.git/credentials"
echo "https://${GH_TOKEN}:@github.com" > .git/credentials

TOML_VERSION=$(cat ./Cargo.toml | grep version | head -1 | grep -oP '\d{1,2}\.\d{1,2}\.\d{1,2}')
git tag -a ${TOML_VERSION} -m "${TRAVIS_COMMIT_MESSAGE}"
git push origin --tags
