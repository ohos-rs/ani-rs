#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

echo "[1/4] Cleaning stale .d.ets artifacts under examples/*/target/ani-ets"
find examples -path '*/target/ani-ets/*.d.ets' -delete

echo "[2/4] Building all example packages to emit ETS files"
pkgs="$(rg -n '^name[[:space:]]*=' examples/*/Cargo.toml | sed -E 's/.*"([^"]+)".*/\1/')"
while IFS= read -r pkg; do
  [[ -z "$pkg" ]] && continue
  echo "  - cargo check -p $pkg"
  cargo check -p "$pkg" >/tmp/"${pkg}".log 2>&1 || {
    echo "FAILED: $pkg"
    tail -n 80 /tmp/"${pkg}".log
    exit 1
  }
done <<< "$pkgs"

echo "[3/4] Verifying every example has expected .ets output"
issues=0
while IFS= read -r cargo; do
  [[ -z "$cargo" ]] && continue

  dir="$(dirname "$cargo")"
  name="$(sed -n 's/^name[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$cargo" | head -n1)"
  if [[ -z "$name" ]]; then
    echo "NO_NAME: $cargo"
    issues=1
    continue
  fi

  base="${name//-/_}"
  ets="$dir/target/ani-ets/${base}.ets"
  dets="$dir/target/ani-ets/${base}.d.ets"

  if [[ ! -f "$ets" ]]; then
    echo "MISSING_ETS: $ets"
    issues=1
    continue
  fi
  if [[ -f "$dets" ]]; then
    echo "UNEXPECTED_D_ETS: $dets"
    issues=1
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

echo "[4/4] Checking no .d.ets remains in examples outputs"
if find examples -path '*/target/ani-ets/*.d.ets' | grep -q .; then
  echo "UNEXPECTED_D_ETS_FILES_FOUND"
  find examples -path '*/target/ani-ets/*.d.ets' | sort
  issues=1
fi

if [[ "$issues" -ne 0 ]]; then
  echo "FAILED: ETS output validation did not pass."
  exit 1
fi

echo "OK: all examples generated ANI-style .ets outputs."
