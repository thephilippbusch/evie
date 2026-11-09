#!/bin/sh
# Validates the agent files: skills, subagents, AGENTS.md and CLAUDE.md files.
#
# Usage: validate-agent-files.sh   (run from the repository root)
#
# Checks every "CI: yes" row of the Budgets table in
# .agents/skills/authoring-skills/SKILL.md:
#
#   - each skill in .agents/skills/ passes `skills-ref validate` (frontmatter
#     keys, name, description, compatibility) and its name is ASCII only
#   - SKILL.md files are under 500 lines, AGENTS.md files under 200 lines and
#     subagent files in .claude/agents/ under 100 lines
#   - each subagent has a valid name that equals its file name, and a
#     non-empty description
#   - relative links in skills, subagents and AGENTS.md files resolve
#   - symlinks resolve; every CLAUDE.md is a symlink to the AGENTS.md beside
#     it, every AGENTS.md has one, and .claude/skills points to .agents/skills
#
# Files are those git tracks plus untracked files that aren't ignored.
# Needs git, awk and uvx (uv is pinned in .prototools).

set -eu

# skills-ref, the Agent Skills reference validator, runs from a pinned commit
# of the agentskills repository. --exclude-newer pins its dependencies to the
# versions published before that date, so every run resolves the same ones.
skills_ref_commit='f130f348f502d9804278a617f86929846896d2e9'
skills_ref_deps_date='2026-11-01T00:00:00Z'

skills_dir='.agents/skills'
agents_dir='.claude/agents'
name_pattern='^[a-z0-9]+(-[a-z0-9]+)*$'

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
errors=0

# error FILE MESSAGE
error() {
  printf '%s: %s\n' "$1" "$2" >&2
  errors=$((errors + 1))
}

# frontmatter_field FILE KEY: prints the value of a top-level YAML key from
# the frontmatter, unquoted, with block scalars (| and >) joined to one line.
frontmatter_field() {
  awk -v key="$2" '
    NR == 1 { if ($0 != "---") exit; next }
    $0 == "---" { exit }
    found && /^[ \t]/ { sub(/^[ \t]+/, ""); value = value " " $0; next }
    found { exit }
    index($0, key ":") == 1 {
      found = 1
      value = substr($0, length(key) + 2)
      sub(/^[ \t]+/, "", value)
      if (value ~ /^[|>][-+0-9]*[ \t]*$/) value = ""
    }
    END {
      sub(/^[ \t]+/, "", value); sub(/[ \t]+$/, "", value)
      if (value ~ /^".*"$/ || value ~ /^\047.*\047$/) value = substr(value, 2, length(value) - 2)
      print value
    }
  ' "$1"
}

# has_frontmatter FILE: the file starts with a --- line and has a closing one.
has_frontmatter() {
  awk 'NR == 1 && $0 != "---" { exit 1 } NR > 1 && $0 == "---" { found = 1; exit } END { exit !found }' "$1"
}

# check_lines FILE LIMIT: the file has fewer than LIMIT lines (wc -l).
check_lines() {
  lines=$(wc -l < "$1" | tr -d ' ')
  if [ "$lines" -ge "$2" ]; then
    error "$1" "has $lines lines; the budget is under $2"
  fi
}

# relative_links FILE: prints "LINE<TAB>TARGET" for each relative link
# outside code blocks and code spans: inline links, images and reference
# definitions. URLs, other schemes and in-page anchors are skipped.
relative_links() {
  awk '
    # Removes code spans: a run of backticks up to the next run of the same length.
    function strip_code(s,    out, i, n, rest, j) {
      out = ""
      while ((i = index(s, "`")) > 0) {
        out = out substr(s, 1, i - 1)
        n = 0
        while (substr(s, i + n, 1) == "`") n++
        rest = substr(s, i + n)
        j = index(rest, substr(s, i, n))
        if (j == 0) return out substr(s, i)
        s = substr(rest, j + n)
      }
      return out s
    }
    function emit(target) {
      sub(/^[ \t]+/, "", target)
      if (target ~ /^</) { sub(/^</, "", target); sub(/>.*/, "", target) }
      else sub(/[ \t].*/, "", target)
      sub(/[#?].*/, "", target)
      gsub(/%20/, " ", target)
      if (target == "" || target ~ /^[A-Za-z][A-Za-z0-9+.-]*:/) return
      print NR "\t" target
    }
    /^ ? ? ?(```|~~~)/ {
      match($0, /(```|~~~)/)
      fence = substr($0, RSTART, 3)
      if (open == "") { open = fence; next }
      if (fence == open) { open = ""; next }
    }
    open != "" { next }
    /^ ? ? ?\[[^]]+\]:/ { line = $0; sub(/^[^]]*\]:/, "", line); emit(line); next }
    {
      line = strip_code($0)
      while (match(line, /\]\([^)]*\)/)) {
        emit(substr(line, RSTART + 2, RLENGTH - 3))
        line = substr(line, RSTART + RLENGTH)
      }
    }
  ' "$1"
}

# check_links FILE: every relative link in FILE points to an existing path,
# resolved from the file's directory, or from the repo root for /paths.
check_links() {
  base=$(dirname "$1")
  relative_links "$1" > "$tmp/links"
  while IFS="$(printf '\t')" read -r line target; do
    case "$target" in
      /*) path=".$target" ;;
      *) path="$base/$target" ;;
    esac
    if [ ! -e "$path" ]; then
      error "$1:$line" "broken link to $target"
    fi
  done < "$tmp/links"
}

# The files to check: tracked, plus untracked files that aren't ignored.
git -c core.quotePath=false ls-files --cached --others --exclude-standard | sort -u > "$tmp/all"
: > "$tmp/files"
while IFS= read -r f; do
  if [ -e "$f" ] || [ -L "$f" ]; then printf '%s\n' "$f" >> "$tmp/files"; fi
done < "$tmp/all"

# Markdown files are read only if they exist; broken symlinks are reported below.
markdown() {
  grep -E "$1" "$tmp/files" | while IFS= read -r f; do
    if [ -f "$f" ]; then printf '%s\n' "$f"; fi
  done
}
markdown '(^|/)AGENTS\.md$' > "$tmp/agents-md"
markdown "^$skills_dir/.+\.md$" > "$tmp/skill-md"
markdown "^$agents_dir/.+\.md$" > "$tmp/subagents"
grep -E '(^|/)CLAUDE\.md$' "$tmp/files" > "$tmp/claude-md" || true

# Skills: the spec validator, plus evie's stricter ASCII-only names.
if ! command -v uvx > /dev/null 2>&1; then
  printf 'uvx is not available. In the repository root, run: proto trust && proto install\n' >&2
  exit 1
fi

for dir in "$skills_dir"/*/; do
  [ -d "$dir" ] || continue
  dir=${dir%/}
  if output=$(uvx --quiet --exclude-newer "$skills_ref_deps_date" \
    --from "git+https://github.com/agentskills/agentskills@$skills_ref_commit#subdirectory=skills-ref" \
    skills-ref validate "$dir" 2>&1); then
    name=$(frontmatter_field "$dir/SKILL.md" name)
    if ! printf '%s\n' "$name" | grep -Eq "$name_pattern"; then
      error "$dir/SKILL.md" "skill name '$name' may only use a-z, 0-9 and single hyphens"
    fi
  elif printf '%s\n' "$output" | grep -q '^Validation failed for '; then
    printf '%s\n' "$output" | sed -n 's/^  - //p' > "$tmp/skill-errors"
    while IFS= read -r message; do
      error "$dir/SKILL.md" "$message"
    done < "$tmp/skill-errors"
  else
    printf '%s\n' "$output" >&2
    error "$dir" "skills-ref could not run (see the output above)"
  fi
  if [ -f "$dir/SKILL.md" ]; then
    check_lines "$dir/SKILL.md" 500
  fi
done

while IFS= read -r f; do
  check_links "$f"
done < "$tmp/skill-md"

# Subagents.
while IFS= read -r f; do
  check_lines "$f" 100
  check_links "$f"
  if ! has_frontmatter "$f"; then
    error "$f" "has no YAML frontmatter between --- lines"
    continue
  fi
  name=$(frontmatter_field "$f" name)
  expected=$(basename "$f" .md)
  if [ -z "$name" ]; then
    error "$f" "frontmatter is missing 'name'"
  elif ! printf '%s\n' "$name" | grep -Eq "$name_pattern" || [ "${#name}" -gt 64 ]; then
    error "$f" "subagent name '$name' must be 1-64 chars of a-z, 0-9 and single hyphens"
  elif [ "$name" != "$expected" ]; then
    error "$f" "subagent name '$name' must equal the file name '$expected'"
  fi
  if [ -z "$(frontmatter_field "$f" description)" ]; then
    error "$f" "frontmatter is missing a non-empty 'description'"
  fi
done < "$tmp/subagents"

# AGENTS.md files and their CLAUDE.md symlinks.
while IFS= read -r f; do
  check_lines "$f" 200
  check_links "$f"
  case "$f" in
    */*) claude="${f%/*}/CLAUDE.md" ;;
    *) claude='CLAUDE.md' ;;
  esac
  if [ ! -L "$claude" ] && [ ! -e "$claude" ]; then
    error "$f" "has no CLAUDE.md beside it; add one with: ln -s AGENTS.md $claude"
  fi
done < "$tmp/agents-md"

while IFS= read -r f; do
  if [ ! -L "$f" ]; then
    error "$f" "must be a symlink to AGENTS.md, not a regular file"
  elif [ "$(readlink "$f")" != 'AGENTS.md' ]; then
    error "$f" "must point to AGENTS.md in the same directory, not $(readlink "$f")"
  elif [ ! -f "$(dirname "$f")/AGENTS.md" ]; then
    error "$f" "points to AGENTS.md, which doesn't exist"
  fi
done < "$tmp/claude-md"

# Symlinks.
while IFS= read -r f; do
  if [ -L "$f" ] && [ ! -e "$f" ]; then
    error "$f" "broken symlink to $(readlink "$f")"
  fi
done < "$tmp/files"

if [ ! -L .claude/skills ]; then
  error .claude/skills "must be a symlink to ../$skills_dir"
elif [ -e .claude/skills ] && [ "$(cd -P .claude/skills && pwd)" != "$(cd -P "$skills_dir" && pwd)" ]; then
  error .claude/skills "must point to $skills_dir, not $(readlink .claude/skills)"
fi

if [ "$errors" -gt 0 ]; then
  printf '\n%s problem(s) found. See the budgets in %s/authoring-skills/SKILL.md.\n' "$errors" "$skills_dir" >&2
  exit 1
fi
printf 'Agent files are valid.\n'
