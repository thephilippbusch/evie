#!/bin/sh
# Formats the staged files with dprint and stages the result.
#
# Files that also have unstaged changes are only checked, never rewritten, so
# the commit never picks up changes that weren't staged. dprint decides which
# files it formats (see dprint.json); it skips the rest.

set -eu

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

git -c core.quotePath=false diff --cached --name-only --diff-filter=ACMR > "$tmp/staged"
[ -s "$tmp/staged" ] || exit 0

if ! dprint --version > /dev/null 2>&1; then
  printf 'dprint is not available. In the repository root, run: proto trust && proto install\n' >&2
  exit 1
fi

git -c core.quotePath=false diff --name-only > "$tmp/unstaged"
grep -Fxv -f "$tmp/unstaged" "$tmp/staged" > "$tmp/format" || true
grep -Fx -f "$tmp/unstaged" "$tmp/staged" > "$tmp/check" || true

if [ -s "$tmp/format" ]; then
  dprint fmt --allow-no-files --stdin-files < "$tmp/format"
  git --literal-pathspecs add --pathspec-from-file="$tmp/format"
fi

if [ -s "$tmp/check" ] && ! dprint check --allow-no-files --stdin-files < "$tmp/check"; then
  printf '\nThese files are partly staged, so they were checked but not formatted.\n' >&2
  printf 'Run `moon run root:format` (or `dprint fmt <file>`), stage the result and commit again.\n' >&2
  exit 1
fi
