#!/usr/bin/env bash
# crates/scan/data/platforms-database.json is a copy of droidtop-platforms' platforms-database.json at the commit crates/scan/data/UPSTREAM names, so every program
# reading it says the same about a folder (droidtop SPEC 7e2b). This fails when the copy is not that commit's
# file, so a hand edit, or a copy refreshed without moving the pin, cannot pass. To refresh: copy the file
# from the new commit and write its hash in UPSTREAM. (From droidtop-agent's scripts/check-platform-data.sh.)
set -euo pipefail
cd "$(dirname "$0")/.."
sha="$(tr -d '[:space:]' < crates/scan/data/UPSTREAM)"
[[ "$sha" =~ ^[0-9a-f]{40}$ ]] || { echo "crates/scan/data/UPSTREAM must hold a full commit hash" >&2; exit 1; }
upstream="$(mktemp)"
curl -fsS --retry 3 -o "$upstream" "https://raw.githubusercontent.com/Droidtop/droidtop-platforms/$sha/platforms-database.json"
if cmp -s "$upstream" "crates/scan/data/platforms-database.json"; then
    echo "platforms-database.json is droidtop-platforms' at $sha"
else
    echo "crates/scan/data/platforms-database.json is not droidtop-platforms' platforms-database.json at $sha" >&2
    exit 1
fi
