#!/usr/bin/env bash
# xeli-stats.sh — print daily download counts across npm and GitHub releases.
#
# Usage:
#   ./scripts/xeli-stats.sh                 # human-readable
#   ./scripts/xeli-stats.sh --json          # machine-readable JSON
#   SLACK_WEBHOOK=https://... ./scripts/xeli-stats.sh --slack   # post to Slack
#
# Deps: curl, jq, gh (authenticated for the GitHub release counts).

set -euo pipefail

NPM_PKG="@josharsh/xeli"
NPM_PKG_ENCODED="%40josharsh%2Fxeli"
REPO="josharsh/xeli"

fail() { echo "error: $*" >&2; exit 1; }
command -v curl >/dev/null || fail "curl is required"
command -v jq >/dev/null || fail "jq is required"
command -v gh >/dev/null || fail "gh is required (brew install gh && gh auth login)"

# --- npm downloads (last day / week / month) ---
npm_count() {
  # npm's downloads API returns 404 for packages with no recorded downloads in
  # the range (including very fresh publishes) — treat as 0 instead of failing.
  local range="$1"
  curl -fs "https://api.npmjs.org/downloads/point/${range}/${NPM_PKG}" 2>/dev/null \
    | jq -r '.downloads // 0' 2>/dev/null \
    || echo 0
}

NPM_DAY=$(npm_count last-day || echo 0)
NPM_WEEK=$(npm_count last-week || echo 0)
NPM_MONTH=$(npm_count last-month || echo 0)

# --- GitHub release downloads (sum across all assets + releases) ---
# This is the closest proxy for Homebrew and manual installs since they pull
# the binary tarballs from the GitHub release.
GH_TOTAL=$(gh api "repos/${REPO}/releases" --paginate \
  --jq '[.[] | .assets[] | .download_count] | add // 0')

# Per-release breakdown
GH_BREAKDOWN=$(gh api "repos/${REPO}/releases" --paginate \
  --jq '.[] | "\(.tag_name): \(.assets | map(.download_count) | add // 0)"')

TOTAL=$((NPM_MONTH + GH_TOTAL))  # rough composite — npm last-month + GH all-time

if [[ "${1:-}" == "--json" ]]; then
  jq -n \
    --argjson npm_day "$NPM_DAY" \
    --argjson npm_week "$NPM_WEEK" \
    --argjson npm_month "$NPM_MONTH" \
    --argjson gh_total "$GH_TOTAL" \
    --argjson total "$TOTAL" \
    '{
      npm: { last_day: $npm_day, last_week: $npm_week, last_month: $npm_month },
      github_releases: { all_time: $gh_total },
      composite: { rough_total: $total }
    }'
  exit 0
fi

if [[ "${1:-}" == "--slack" ]]; then
  [[ -n "${SLACK_WEBHOOK:-}" ]] || fail "SLACK_WEBHOOK env var not set"
  TEXT="*xeli stats* \n\
• npm (24h): *${NPM_DAY}* | week: ${NPM_WEEK} | month: ${NPM_MONTH}\n\
• GitHub release downloads (all-time): *${GH_TOTAL}*\n\
• Composite rough total: *${TOTAL}*"
  curl -fsS -X POST -H 'Content-Type: application/json' \
    -d "$(jq -n --arg t "$TEXT" '{text: $t}')" \
    "$SLACK_WEBHOOK" >/dev/null
  echo "Posted to Slack."
  exit 0
fi

# Default: pretty terminal output
printf '\n  \033[1;35mxeli stats\033[0m\n'
printf '  \033[2m─────────────────────────────\033[0m\n'
printf '  npm (last 24h): \033[1;32m%s\033[0m\n' "$NPM_DAY"
printf '  npm (last 7d):  %s\n' "$NPM_WEEK"
printf '  npm (last 30d): %s\n' "$NPM_MONTH"
printf '  GitHub releases (all-time): \033[1;32m%s\033[0m\n' "$GH_TOTAL"
printf '  \033[2m─────────────────────────────\033[0m\n'
printf '  Composite rough total: \033[1;33m%s\033[0m\n\n' "$TOTAL"
printf '  Per-release breakdown:\n%s\n\n' "$(echo "$GH_BREAKDOWN" | sed 's/^/    /')"
