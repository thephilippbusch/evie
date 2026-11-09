#!/usr/bin/env bash
# Claude Code PreToolUse guard, registered in .claude/settings.json.
#
# Blocks a tool call (exit 2, reason on stderr) when it would:
#   - edit a path listed in protected-paths.txt (Edit, Write, MultiEdit, NotebookEdit),
#   - force-push or push to `main` (Bash),
#   - run `drizzle-kit push` (Bash; humans run it in local dev),
#   - read or touch `.env` files, keys or other secrets (Bash),
#   - write to a protected path through the shell (Bash).
# It backs up the deny rules in .claude/settings.json, which match only the
# command text as written; this script also parses compound commands,
# `sh -c`, `git -C`, refspecs and the current branch.
#
# Requires bash 3.2+ and jq. Without jq it blocks every call it guards.
# Try it:
#   echo '{"tool_name":"Bash","tool_input":{"command":"git push -f"},"cwd":"."}' \
#     | .claude/hooks/pre-tool-use.sh; echo "exit $?"

HOOK_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)
PROTECTED_LIST="$HOOK_DIR/protected-paths.txt"

block() {
  printf 'Blocked by .claude/hooks/pre-tool-use.sh: %s\n' "$1" >&2
  exit 2
}

MSG_FORCE="force-pushing is not allowed (AGENTS.md rules). Push new commits instead of rewriting pushed history. If a force-push is really needed, ask the user to run it."
MSG_MAIN="pushing to \`main\` is not allowed (AGENTS.md rules). Work on a branch, push that branch and open a PR."
MSG_DRIZZLE="\`drizzle-kit push\` is for humans in local dev only. Generate a migration, review it and apply it with \`moon run :migrate\`, or ask the user to push their local database."
MSG_ASK="If this file really needs a change, stop and ask the user."

if ! command -v jq >/dev/null 2>&1; then
  block "jq is not installed, so the guardrails cannot check this call. Ask the user to install jq (for example \`brew install jq\`)."
fi

input=$(cat)
tool=$(jq -r '.tool_name // empty' <<<"$input")
cwd=$(jq -r '.cwd // empty' <<<"$input")
if [ -z "$cwd" ] || [ ! -d "$cwd" ]; then cwd=${CLAUDE_PROJECT_DIR:-$PWD}; fi

# ---------------------------------------------------------------------------
# Paths

# abspath PATH BASE: absolute path with ".", ".." and "//" removed and the
# deepest existing directory resolved through symlinks (/tmp -> /private/tmp).
abspath() {
  local p=$1 part d rest= real
  local -a parts out=()
  case $p in /*) ;; '~'/*) p="$HOME/${p#'~'/}" ;; *) p="$2/$p" ;; esac
  IFS=/ read -ra parts <<<"$p"
  for part in "${parts[@]}"; do
    case $part in
      '' | .) ;;
      ..) [ ${#out[@]} -gt 0 ] && unset "out[$((${#out[@]} - 1))]" ;;
      *) out+=("$part") ;;
    esac
  done
  p=$(IFS=/; printf '/%s' "${out[*]}")
  d=$p
  while [ "$d" != / ] && [ ! -d "$d" ]; do
    rest="/${d##*/}$rest"
    d=${d%/*}
    [ -n "$d" ] || d=/
  done
  real=$(cd "$d" 2>/dev/null && pwd -P) || real=$d
  [ "$real" = / ] && real=
  printf '%s%s' "$real" "$rest"
}

# protected ABSPATH: succeeds when the path matches protected-paths.txt and
# sets MATCH to the message for the agent.
protected() {
  local abs=$1 d top rel name line pattern reason hit zero
  [ -r "$PROTECTED_LIST" ] || return 1
  d=${abs%/*}
  while [ -n "$d" ] && [ ! -d "$d" ]; do d=${d%/*}; done
  top=$(git -C "${d:-/}" rev-parse --show-toplevel 2>/dev/null) || return 1
  case $abs in "$top"/*) rel=${abs#"$top"/} ;; *) return 1 ;; esac
  name=${rel##*/}
  shopt -s nocasematch
  while IFS= read -r line || [ -n "$line" ]; do
    line=${line#"${line%%[![:space:]]*}"}
    case $line in '' | '#'*) continue ;; esac
    pattern=${line%%[[:space:]]*}
    reason=${line#"$pattern"}
    reason=${reason#"${reason%%[![:space:]]*}"}
    pattern=${pattern#/}
    hit=0
    case $pattern in
      */) [[ $rel == ${pattern}* ]] && hit=1 ;;
      */*) # "**/" also matches zero directories
        zero=${pattern#'**/'}
        while [[ $zero == *'/**/'* ]]; do zero="${zero%%'/**/'*}/${zero#*'/**/'}"; done
        [[ $rel == $pattern || $rel == $zero ]] && hit=1
        ;;
      *) [[ $name == $pattern ]] && hit=1 ;;
    esac
    if [ $hit = 1 ]; then
      shopt -u nocasematch
      MATCH="\`$rel\` is protected (rule \`$pattern\` in .claude/hooks/protected-paths.txt). ${reason:-It must not be edited by hand.} $MSG_ASK"
      return 0
    fi
  done <"$PROTECTED_LIST"
  shopt -u nocasematch
  return 1
}

# secret WORD: succeeds when a shell word names a .env file, key or credentials.
secret() {
  local t=$1 b
  case $t in *'\'*) return 1 ;; esac # a regex such as '\.env', not a path
  t=${t##*=}
  t=${t#@}
  b=${t##*/}
  case $b in
    .env.example | .env.sample | .env.template | .env.dist | *.env.example) return 1 ;;
    .env* | *.env | *.pem | *.key | *.p12 | *.pfx | id_rsa* | id_ed25519* | id_ecdsa*) return 0 ;;
  esac
  case $t in
    secrets/* | */secrets/* | */secrets | .ssh | .ssh/* | */.ssh | */.ssh/* | */.aws | */.aws/*) return 0 ;;
  esac
  return 1
}

check_secret() {
  secret "$1" && block "\`$1\` looks like a secret (.env file, key or credentials). Agents never read, print, copy or edit secrets. Use \`.env.example\` for variable names, or ask the user."
}

check_write() {
  protected "$(abspath "$1" "$seg_cwd")" && block "$MATCH"
}

# ---------------------------------------------------------------------------
# Bash: split the command into simple commands and words, honouring quotes,
# heredocs and comments, then check each simple command.

flush() {
  if [ "$have" = 1 ]; then toks+=("$tok"); fi
  tok=
  have=0
}

segment() {
  [ ${#toks[@]} -gt 0 ] && check_segment "${toks[@]}"
  toks=()
}

# analyze STRING DEPTH
analyze() {
  local s=$1 depth=$2 n i c q= tok= have=0 hd= rest pre
  local -a toks=()
  [ "$depth" -gt 4 ] && return
  n=${#s}
  for ((i = 0; i < n; i++)); do
    c=${s:i:1}
    if [ -n "$q" ]; then
      if [ "$c" = "$q" ]; then
        q=
      elif [ "$q" = '"' ] && [ "$c" = '\' ]; then
        i=$((i + 1))
        tok+=${s:i:1}
      else
        tok+=$c
      fi
      continue
    fi
    case $c in
      "'" | '"') q=$c have=1 ;;
      '\')
        i=$((i + 1))
        [ "${s:i:1}" = $'\n' ] || tok+=${s:i:1}
        have=1
        ;;
      ' ' | $'\t') flush ;;
      '#')
        if [ "$have" = 0 ]; then
          while [ $i -lt $n ] && [ "${s:i+1:1}" != $'\n' ]; do i=$((i + 1)); done
        else
          tok+=$c
        fi
        ;;
      $'\n')
        flush
        segment
        if [ -n "$hd" ]; then # skip the heredoc body
          rest=${s:i+1}
          if [ "${rest%%$'\n'*}" = "$hd" ]; then
            i=$((i + ${#hd} + 1))
          else
            pre=${rest%%$'\n'"$hd"*}
            if [ "$pre" = "$rest" ]; then i=$n; else i=$((i + 1 + ${#pre} + 1 + ${#hd})); fi
          fi
          hd=
        fi
        ;;
      ';' | '|' | '(' | ')' | '`')
        flush
        segment
        ;;
      '&')
        flush
        [ "${s:i+1:1}" = '>' ] || segment
        ;;
      '$')
        if [ "${s:i+1:1}" = '(' ]; then
          flush
          segment
          i=$((i + 1))
        else
          tok+=$c
          have=1
        fi
        ;;
      '>' | '<')
        if [[ $tok =~ ^[0-9]+$ ]]; then tok= have=0; else flush; fi
        if [ "$c$c" = '<<' ] && [ "${s:i+1:1}" = '<' ]; then
          if [ "${s:i+2:1}" = '<' ]; then # here-string: the next word is data
            i=$((i + 2))
            toks+=('<<<')
          else # heredoc: remember the delimiter, skip the body at the next newline
            i=$((i + 2))
            [ "${s:i:1}" = - ] && i=$((i + 1))
            while [ "${s:i:1}" = ' ' ]; do i=$((i + 1)); done
            hd=
            while [ $i -lt $n ]; do
              c=${s:i:1}
              case $c in ' ' | $'\t' | $'\n' | ';' | '|' | '&' | ')') break ;; "'" | '"') ;; *) hd+=$c ;; esac
              i=$((i + 1))
            done
            i=$((i - 1))
          fi
          continue
        fi
        while [[ ${s:i+1:1} == [\>\|] ]]; do i=$((i + 1)); done
        if [ "${s:i+1:1}" = '&' ]; then
          i=$((i + 1))
          toks+=("$c&")
        else
          toks+=("$c")
        fi
        ;;
      *)
        tok+=$c
        have=1
        ;;
    esac
  done
  flush
  segment
}

seg_cwd=$cwd

check_segment() {
  local -a a=("$@")
  local m=$# k j w cmd dk=0
  # Every word: secrets, redirect targets, nested command substitutions.
  for ((j = 0; j < m; j++)); do
    w=${a[j]}
    case $w in
      '>') check_write "${a[j + 1]}" ;;
      '>&' | '<&' | '<<<' | '<') ;;
      *$'\n'* | *' '* | *$'\t'*) ;;
      *) check_secret "$w" ;;
    esac
    case $w in
      *'$('*) analyze "${w#*\$(}" $((depth + 1)) ;;
      *'`'*) analyze "${w#*\`}" $((depth + 1)) ;;
    esac
  done
  # Skip assignments, keywords and wrappers to find the command.
  k=0
  while [ $k -lt $m ]; do
    w=${a[k]}
    if [[ $w =~ ^[A-Za-z_][A-Za-z0-9_]*= ]]; then
      k=$((k + 1))
      continue
    fi
    case ${w##*/} in
      sudo | env | nohup | nice | time | timeout | command | builtin | noglob | exec | xargs | stdbuf | \
        if | then | else | elif | do | while | until | '!' | '{')
        k=$((k + 1))
        while [ $k -lt $m ] && [[ ${a[k]} == -* || ${a[k]} =~ ^[0-9.]+[smhd]?$ ]]; do k=$((k + 1)); done
        continue
        ;;
    esac
    break
  done
  [ $k -lt $m ] || return 0
  cmd=${a[k]##*/}
  case $cmd in
    cd | pushd)
      w=$(abspath "${a[k + 1]:-$HOME}" "$seg_cwd")
      [ -d "$w" ] && seg_cwd=$w
      ;;
    git) check_git ;;
    sh | bash | zsh | dash | ksh)
      for ((j = k + 1; j < m; j++)); do
        if [[ ${a[j]} =~ ^-[a-zA-Z]*c[a-zA-Z]*$ ]]; then
          analyze "${a[j + 1]}" $((depth + 1))
          break
        fi
      done
      ;;
    eval) analyze "${a[*]:k+1}" $((depth + 1)) ;;
    tee | truncate | rm | mv)
      for ((j = k + 1; j < m; j++)); do
        case ${a[j]} in -* | '>' | '>&' | '<' | '<&' | '<<<') ;; *) check_write "${a[j]}" ;; esac
      done
      ;;
    cp | install | ln) check_write "${a[m - 1]}" ;;
    sed | perl)
      for ((j = k + 1; j < m; j++)); do
        case ${a[j]} in -i* | --in-place*) w=1 && break ;; esac
      done
      if [ $j -lt $m ]; then
        for ((j = k + 1; j < m; j++)); do
          case ${a[j]} in -*) ;; *) check_write "${a[j]}" ;; esac
        done
      fi
      ;;
  esac
  # drizzle-kit push, also through bunx, npx, bun x, pnpm dlx, ...
  for ((j = k; j < m; j++)); do
    case ${a[j]##*/} in
      drizzle-kit | drizzle-kit@*) dk=1 ;;
      push) [ $dk = 1 ] && block "$MSG_DRIZZLE" ;;
    esac
  done
  return 0
}

# check_git: uses a, m and k from check_segment.
check_git() {
  local j=$((k + 1)) gdir=$seg_cwd w spec dst branch upstream force=0 all=0 opts=1
  local -a pos=()
  while [ $j -lt $m ]; do # global options before the subcommand
    case ${a[j]} in
      -C)
        gdir=$(abspath "${a[j + 1]}" "$seg_cwd")
        j=$((j + 2))
        ;;
      -c | --git-dir | --work-tree | --namespace | --config-env) j=$((j + 2)) ;;
      -*) j=$((j + 1)) ;;
      *) break ;;
    esac
  done
  [ "${a[j]}" = push ] || return 0
  for ((j = j + 1; j < m; j++)); do
    w=${a[j]}
    if [ $opts = 1 ]; then
      case $w in
        --) opts=0 && continue ;;
        --force | --force-with-lease | --force-with-lease=* | --force-if-includes) force=1 && continue ;;
        --mirror | --all | --branches) all=1 && continue ;;
        -o | --push-option | --repo | --receive-pack | --exec) j=$((j + 1)) && continue ;;
        '>' | '>&' | '<' | '<&' | '<<<') j=$((j + 1)) && continue ;;
        --*) continue ;;
        -*)
          [[ $w == *f* ]] && force=1
          continue
          ;;
      esac
    fi
    pos+=("$w")
  done
  [ $force = 1 ] && block "$MSG_FORCE"
  for spec in "${pos[@]:1}"; do
    [[ $spec == +* ]] && block "$MSG_FORCE (a \`+\` refspec forces the update)"
  done
  [ $all = 1 ] && block "$MSG_MAIN (\`--all\`, \`--branches\` and \`--mirror\` push \`main\` too)"
  branch=$(git -C "$gdir" branch --show-current 2>/dev/null)
  if [ ${#pos[@]} -le 1 ]; then
    [ "$branch" = main ] && block "$MSG_MAIN (you are on \`main\`, so a plain \`git push\` pushes it)"
    upstream=$(git -C "$gdir" rev-parse --abbrev-ref '@{push}' 2>/dev/null)
    [[ $upstream == */main ]] && block "$MSG_MAIN (this branch pushes to \`$upstream\`; push with an explicit branch name)"
    return 0
  fi
  for spec in "${pos[@]:1}"; do
    dst=${spec#+}
    dst=${dst##*:}
    case $dst in HEAD | @) dst=$branch ;; esac
    dst=${dst#refs/heads/}
    [ "$dst" = main ] && block "$MSG_MAIN"
  done
  return 0
}

# ---------------------------------------------------------------------------

case $tool in
  Edit | Write | MultiEdit | NotebookEdit)
    path=$(jq -r '.tool_input.file_path // .tool_input.notebook_path // empty' <<<"$input")
    [ -n "$path" ] || exit 0
    protected "$(abspath "$path" "$cwd")" && block "$MATCH"
    ;;
  Bash)
    analyze "$(jq -r '.tool_input.command // empty' <<<"$input")" 0
    ;;
esac
exit 0
