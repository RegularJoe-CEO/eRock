#!/usr/bin/env bash
set -euo pipefail
: "${GITHUB_TOKEN:?Please export GITHUB_TOKEN before running this script}"
echo "Token present in env (not in repo)."
# ... your logic using $GITHUB_TOKEN ...
