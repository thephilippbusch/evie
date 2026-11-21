#!/bin/sh
# Fails when gen/ differs from what `moon run proto:codegen` writes, including
# deleted and extra files. Generates into a temporary folder, so it writes
# nothing in the checkout and passes before fresh output is committed.
#
# Usage: check-codegen.sh, from libs/proto. Run by the check-codegen task.

set -eu

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

cargo run --quiet --locked -p proto-codegen -- "$tmp"

if ! diff -r gen "$tmp/gen"; then
  echo 'gen/ is stale or was edited by hand. Run `moon run proto:codegen` and commit the result.' >&2
  exit 1
fi
