#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

first_target="$(mktemp -d "${TMPDIR:-/tmp}/ani-rs-ets-check-first.XXXXXX")"
second_target="$(mktemp -d "${TMPDIR:-/tmp}/ani-rs-ets-check-second.XXXXXX")"

cleanup() {
  local directory
  for directory in "$first_target" "$second_target"; do
    if [[ -d "$directory" && "$(basename "$directory")" == ani-rs-ets-check-* ]]; then
      rm -rf -- "$directory"
    fi
  done
}
trap cleanup EXIT

validate_output_tree() {
  local target_root="$1"
  local issues=0
  local cargo name base ets

  while IFS= read -r cargo; do
    [[ -z "$cargo" ]] && continue
    name="$(sed -n 's/^name[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$cargo" | head -n1)"
    if [[ -z "$name" ]]; then
      echo "NO_NAME: $cargo"
      issues=1
      continue
    fi

    base="${name//-/_}"
    ets="$target_root/ani-ets/${base}.ets"
    if [[ ! -f "$ets" ]]; then
      echo "MISSING_ETS: $ets"
      issues=1
      continue
    fi
    if grep -q 'declare ' "$ets"; then
      echo "HAS_DECLARE: $ets"
      issues=1
    fi
    if ! grep -q 'native ' "$ets"; then
      echo "NO_NATIVE: $ets"
      issues=1
    fi
    if ! grep -q 'loadLibrary(' "$ets"; then
      echo "NO_LOAD_LIBRARY: $ets"
      issues=1
    fi
  done < <(find examples -maxdepth 2 -name Cargo.toml | sort)

  if find "$target_root/ani-ets" -name '*.d.ets' -print -quit | grep -q .; then
    echo "UNEXPECTED_D_ETS_FILES_FOUND: $target_root/ani-ets"
    issues=1
  fi

  [[ "$issues" -eq 0 ]]
}

echo "[1/4] Fresh workspace build for ETS generation"
CARGO_TARGET_DIR="$first_target" cargo check --workspace --all-features
validate_output_tree "$first_target"

echo "[2/4] Independent fresh build for reproducibility"
CARGO_TARGET_DIR="$second_target" cargo check --workspace --all-features
validate_output_tree "$second_target"

echo "[3/4] Comparing generated ETS bytes"
issues=0
while IFS= read -r first_ets; do
  relative="${first_ets#"$first_target"/}"
  second_ets="$second_target/$relative"
  if [[ ! -f "$second_ets" ]] || ! cmp -s "$first_ets" "$second_ets"; then
    echo "NON_DETERMINISTIC_ETS: $relative"
    if [[ -f "$second_ets" ]]; then
      diff -u "$first_ets" "$second_ets" || true
    fi
    issues=1
  fi
done < <(find "$first_target/ani-ets" -type f -name '*.ets' | sort)

echo "[4/4] Checking for leaked atomic temporary files"
if find "$first_target/ani-ets" "$second_target/ani-ets" -type f -name '*.tmp' -print -quit |
  grep -q .; then
  echo "LEAKED_ETS_TEMP_FILE"
  issues=1
fi

if [[ "$issues" -ne 0 ]]; then
  echo "FAILED: ETS output validation did not pass."
  exit 1
fi

echo "OK: all examples generated deterministic, atomic ANI-style .ets outputs."
