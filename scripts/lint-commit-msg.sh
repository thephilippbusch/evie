#!/bin/sh
# Checks that a commit message is a Conventional Commit.
#
# Usage: lint-commit-msg.sh [FILE]
#   FILE  the commit message file (as passed to the commit-msg hook).
#         Without FILE, or with "-", the message is read from stdin.
#
# Allowed types are the standard Conventional Commit types plus `ai` for
# agent configuration. Messages that git or GitHub write themselves (merges,
# reverts, fixup!/squash!/amend!) are accepted as they are.

set -eu

types='feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert|ai'
max_header_length=100

if [ "${1:--}" = "-" ]; then
  raw=$(cat)
else
  raw=$(cat "$1")
fi

# Drop everything below the scissors line of `git commit -v` and all comment
# lines, as git does when it cleans up the message.
message=$(printf '%s\n' "$raw" | sed -e '/^# -* >8 -*$/,$d' -e '/^#/d')

# The header is the first non-empty line; the line after it must be empty.
header=$(printf '%s\n' "$message" | sed -n '/[^[:space:]]/{p;q;}')
after_header=$(printf '%s\n' "$message" | sed -n '/[^[:space:]]/{n;p;q;}')

fail() {
  printf 'Invalid commit message: %s\n\n' "$1" >&2
  printf '  %s\n\n' "$header" >&2
  printf 'Expected a Conventional Commit: <type>[(scope)][!]: <description>\n' >&2
  printf 'Allowed types: %s\n' "$(printf '%s' "$types" | sed 's/|/, /g')" >&2
  printf 'Example: fix(auth): reject expired sessions\n' >&2
  exit 1
}

if [ -z "$header" ]; then
  header='(empty)'
  fail 'the message is empty'
fi

case "$header" in
  'Merge '* | 'Revert "'* | 'fixup! '* | 'squash! '* | 'amend! '*) exit 0 ;;
esac

if ! printf '%s\n' "$header" | grep -Eq "^($types)(\([^()[:space:]]+\))?!?: [^[:space:]]"; then
  fail 'the header does not match the format'
fi

if [ "${#header}" -gt "$max_header_length" ]; then
  fail "the header is longer than $max_header_length characters"
fi

if [ -n "$after_header" ]; then
  fail 'the header must be followed by an empty line'
fi
