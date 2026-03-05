#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/.." && pwd)"
HEADER_PATH="$REPO_ROOT/include/ani.h"
SYS_RS_PATH="$REPO_ROOT/crates/sys/src/lib.rs"

# Usage:
#   scripts/header.sh [ARKCOMPILER_RUNTIME_CORE_DIR]
# Default source dir matches local dev setup.
ARK_SRC_ROOT="${1:-/tmp/arkcompiler_runtime_core}"
ARK_HEADER_REL="static_core/plugins/ets/runtime/ani/ani.h"

if [[ -d "$ARK_SRC_ROOT/.git" ]]; then
  git -C "$ARK_SRC_ROOT" show "HEAD:$ARK_HEADER_REL" > "$HEADER_PATH"
elif [[ -f "$ARK_SRC_ROOT/$ARK_HEADER_REL" ]]; then
  cp "$ARK_SRC_ROOT/$ARK_HEADER_REL" "$HEADER_PATH"
else
  echo "error: ani.h not found in source root: $ARK_SRC_ROOT" >&2
  exit 1
fi

bindgen "$HEADER_PATH" \
  --output "$SYS_RS_PATH" \
  --allowlist-type 'ani_.*|__ani_.*' \
  --allowlist-function 'ANI_.*' \
  --allowlist-var 'ANI_.*' \
  --raw-line '#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]' \
  --no-layout-tests
