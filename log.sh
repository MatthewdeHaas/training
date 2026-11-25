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
