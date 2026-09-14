#!/usr/bin/env bash
set -euo pipefail

export LC_ALL=C
export LANG=C

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "$SCRIPT_DIR/.." && pwd)"
HEADER_PATH="$REPO_ROOT/include/ani.h"
HEADER_HASH_PATH="$REPO_ROOT/include/ani.h.sha256"
SYS_RS_PATH="$REPO_ROOT/crates/sys/src/lib.rs"
BINDGEN_VERSION="bindgen 0.72.1"

usage() {
  echo "usage: scripts/header.sh --check [ANI_HEADER_OR_SOURCE_ROOT]" >&2
  echo "       scripts/header.sh --update ANI_HEADER_OR_SOURCE_ROOT" >&2
}

resolve_header() {
  local source="$1"
  local candidate

  for candidate in \
    "$source" \
    "$source/ani/ani.h" \
    "$source/interface/sdk_c/ani/ani.h" \
    "$source/static_core/plugins/ets/runtime/ani/ani.h"; do
    if [[ -f "$candidate" ]]; then
      printf '%s\n' "$candidate"
      return 0
    fi
  done

  echo "error: ani.h not found from source: $source" >&2
  return 1
}

sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

generate_bindings() {
  local input="$1"
  local output="$2"

  if ! command -v bindgen >/dev/null 2>&1; then
    echo "error: bindgen is required ($BINDGEN_VERSION)" >&2
    return 1
  fi
  if [[ "$(bindgen --version)" != "$BINDGEN_VERSION" ]]; then
    echo "error: expected $BINDGEN_VERSION, got $(bindgen --version)" >&2
    return 1
  fi

  bindgen "$input" \
    --output "$output" \
    --allowlist-type 'ani_.*|__ani_.*' \
    --allowlist-function 'ANI_.*' \
    --allowlist-var 'ANI_.*' \
    --raw-line '#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]' \
    --raw-line '#![allow(rustdoc::broken_intra_doc_links)]' \
    --no-layout-tests
}

validate_openharmony_7_release() {
  local symbol
  for symbol in \
    Primitive_Box_Boolean Primitive_Unbox_Boolean \
    Primitive_Box_Byte Primitive_Unbox_Byte \
    Primitive_Box_Char Primitive_Unbox_Char \
    Primitive_Box_Short Primitive_Unbox_Short \
    Primitive_Box_Int Primitive_Unbox_Int \
    Primitive_Box_Long Primitive_Unbox_Long \
    Primitive_Box_Float Primitive_Unbox_Float \
    Primitive_Box_Double Primitive_Unbox_Double \
    FixedArray_New_Int FixedArray_GetRegion_Int FixedArray_SetRegion_Int \
    Any_InstanceOf Any_Call Any_New; do
    if ! grep -q "$symbol" "$HEADER_PATH"; then
      echo "error: pinned header is missing OpenHarmony 7.0 Release ANI symbol: $symbol" >&2
      return 1
    fi
  done
}

mode="${1:---check}"
source_arg="${2:-}"
case "$mode" in
  --check)
    if [[ -n "$source_arg" ]]; then
      source_header="$(resolve_header "$source_arg")"
      if ! cmp -s "$source_header" "$HEADER_PATH"; then
        echo "error: checked-in include/ani.h differs from $source_header" >&2
        exit 1
      fi
    fi

    if [[ ! -f "$HEADER_HASH_PATH" ]]; then
      echo "error: missing include/ani.h.sha256" >&2
      exit 1
    fi
    expected_hash="$(awk 'NR == 1 { print $1 }' "$HEADER_HASH_PATH")"
    actual_hash="$(sha256 "$HEADER_PATH")"
    if [[ "$actual_hash" != "$expected_hash" ]]; then
      echo "error: include/ani.h checksum drifted" >&2
      echo "expected: $expected_hash" >&2
      echo "actual:   $actual_hash" >&2
      exit 1
    fi

    validate_openharmony_7_release
    generated="$(mktemp "${TMPDIR:-/tmp}/ani-sys.XXXXXX.rs")"
    trap 'rm -f "$generated"' EXIT
    generate_bindings "$HEADER_PATH" "$generated"
    if ! cmp -s "$generated" "$SYS_RS_PATH"; then
      echo "error: crates/sys/src/lib.rs is stale; run scripts/header.sh --update <source>" >&2
      diff -u "$SYS_RS_PATH" "$generated" || true
      exit 1
    fi
    echo "ANI header and Rust bindings are reproducible ($actual_hash)"
    ;;
  --update)
    if [[ -z "$source_arg" ]]; then
      usage
      exit 2
    fi
    source_header="$(resolve_header "$source_arg")"
    install -m 0644 "$source_header" "$HEADER_PATH"
    validate_openharmony_7_release

    generated="$(mktemp "$REPO_ROOT/crates/sys/src/lib.rs.XXXXXX")"
    trap 'rm -f "$generated"' EXIT
    generate_bindings "$HEADER_PATH" "$generated"
    mv "$generated" "$SYS_RS_PATH"
    trap - EXIT

    actual_hash="$(sha256 "$HEADER_PATH")"
    printf '%s  ani.h\n' "$actual_hash" > "$HEADER_HASH_PATH"
    echo "Updated OpenHarmony 7.0 Release ANI header and bindings ($actual_hash)"
    ;;
  *)
    usage
    exit 2
    ;;
esac
