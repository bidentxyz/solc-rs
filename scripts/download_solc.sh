#!/usr/bin/env bash
# Download every released solc version (linux-amd64, no nightlies) from
# solc-bin into .solc/, verifying each download against its sha256 from
# list.json. Existing files are left untouched, so re-runs only fetch new
# versions. Downloads go to a .partial file first and are only moved into
# place after the checksum matches.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DEST="$ROOT/.solc"
BASE_URL="https://binaries.soliditylang.org/linux-amd64"

list_json="$(mktemp)"
manifest="$(mktemp)"
trap 'rm -f "$list_json" "$manifest"; find "$DEST" -name "*.partial" -delete' EXIT

mkdir -p "$DEST"

echo "Fetching $BASE_URL/list.json"
curl -sfL --retry 2 "$BASE_URL/list.json" -o "$list_json"

# The releases map only contains stable releases; nightly and prerelease
# builds appear only in the builds array, so iterating releases excludes them.
# Emit "path<TAB>sha256" lines for every release.
python3 - "$list_json" "$manifest" <<'PYEOF'
import json
import sys

data = json.load(open(sys.argv[1]))
by_path = {b["path"]: b.get("sha256", "") for b in data["builds"]}
with open(sys.argv[2], "w", encoding="utf-8") as f:
    for path in sorted(data["releases"].values()):
        sha256 = by_path.get(path, "")
        if not sha256:
            print(f"warning: no sha256 for {path}", file=sys.stderr)
            continue
        f.write(f"{path}\t{sha256}\n")
PYEOF

downloaded=0
skipped=0
failed=0
while IFS=$'\t' read -r path sha256; do
    target="$DEST/$path"
    if [ -f "$target" ]; then
        echo "skip  $path"
        skipped=$((skipped + 1))
        continue
    fi
    tmp="$target.partial"
    echo "fetch $path"
    if ! curl -sfL --retry 2 "$BASE_URL/$path" -o "$tmp"; then
        echo "error: download failed for $path" >&2
        rm -f "$tmp"
        failed=$((failed + 1))
        continue
    fi
    got="$(sha256sum "$tmp" | awk '{print $1}')"
    want="${sha256#0x}"
    if [ "$got" != "$want" ]; then
        echo "error: sha256 mismatch for $path" >&2
        echo "  expected $want" >&2
        echo "  got      $got" >&2
        rm -f "$tmp"
        failed=$((failed + 1))
        continue
    fi
    chmod +x "$tmp"
    mv "$tmp" "$target"
    downloaded=$((downloaded + 1))
done < "$manifest"

echo "done: downloaded $downloaded, skipped $skipped, failed $failed"
[ "$failed" -eq 0 ]
