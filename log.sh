#!/usr/bin/env zsh

set -euo pipefail

LOG_FILE="${TRAINING_LOG:-$HOME/Training/log.md}"
EDITOR="${EDITOR:-nvim}"

mkdir -p "${LOG_FILE:h}"
touch "$LOG_FILE"

case "${1:-add}" in
  recent)
    tail -n 30 "$LOG_FILE"
    exit 0
    ;;
  today)
    out=$(awk -v d="$(date +%Y-%m-%d)" '/^## / { p = ($2 == d) } p' "$LOG_FILE")
    if [[ -n "$out" ]]; then
      print -r -- "$out"
    else
      echo "no entries"
    fi
    exit 0
    ;;
  week)
    cutoff=$(date -v-7d +%Y-%m-%d) 
    awk -v cutoff="$cutoff" '/^## / { d = $2 } d >= cutoff' "$LOG_FILE"
    exit 0
    ;;
  edit)
    ${=EDITOR} "$LOG_FILE"
    exit 0
    ;;
  add) ;;
  *)
    echo "usage: log [add|recent|today|week|edit]" >&2
    exit 1
    ;;
esac

# Exercise type
read -r "session_type?Type: "
session_type="${session_type:-...}"

# Description
read -r "summary?Summary: "
if [[ -z "$summary" ]]; then
  echo "summary required" >&2
  exit 1
fi

# Details
details_file=$(mktemp)
trap 'rm -f "$details_file"' EXIT
cat >"$details_file" <<EOF
# Details for: $summary
# Lines starting with # are ignored. Save and quit when done.

EOF
${=EDITOR} "$details_file"

details=$({ grep -v '^#' "$details_file" || true; } | sed '/./,$!d')
if [[ -z "$details" ]]; then
  details="(nothing to note)"
fi

# Append the entry
timestamp="$(date '+%Y-%m-%d %a %H:%M')"
printf '\n## %s • %s\n%s\n\nDetails:\n%s\n\n---\n' \
  "$timestamp" "$session_type" "$summary" "$details" >> "$LOG_FILE"

echo "logged -> $LOG_FILE"


# Sync to git (the entry is already saved, so failures here only warn)
repo_dir="${LOG_FILE:h}"
if git -C "$repo_dir" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  git -C "$repo_dir" add -- "$LOG_FILE"
  if git -C "$repo_dir" diff --cached --quiet -- "$LOG_FILE"; then
    echo "git: nothing to commit"
  else
    commit_msg="log: ${session_type}: ${summary}"
    # Commit only the log file, even if other files are staged
    git -C "$repo_dir" commit -q -m "${commit_msg[1,72]}" -- "$LOG_FILE"
    echo "git: committed"
    if git -C "$repo_dir" push -q; then
      echo "git: pushed"
    else
      echo "git: push failed (entry is committed locally)" >&2
    fi
  fi
else
  echo "git: $repo_dir is not a git repo, skipping" >&2
fi


