#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

baseline_rev="${ANI_SEMVER_BASELINE_REV:-origin/master}"
if ! git cat-file -e "$baseline_rev^{commit}" 2>/dev/null; then
  echo "ABI baseline revision is unavailable: $baseline_rev" >&2
  exit 2
fi

work_root="$(mktemp -d "${TMPDIR:-/tmp}/ani-rs-abi.XXXXXX")"
baseline_root="$work_root/baseline"
current_target="$work_root/current-target"
baseline_target="$work_root/baseline-target"

cleanup() {
  if [[ -d "$work_root" && "$(basename "$work_root")" == ani-rs-abi.* ]]; then
    rm -rf -- "$work_root"
  fi
}
trap cleanup EXIT

mkdir -p "$baseline_root"
git archive "$baseline_rev" | tar -x -C "$baseline_root"

echo "[1/4] Generate current ETS/ANI declarations"
CARGO_TARGET_DIR="$current_target" cargo check --workspace --all-features >/dev/null

echo "[2/4] Generate baseline ETS/ANI declarations ($baseline_rev)"
(
  cd "$baseline_root"
  CARGO_TARGET_DIR="$baseline_target" cargo check --workspace --all-features >/dev/null
)

normalize_abi() {
  local root="$1"
  local output="$2"
  : >"$output"
  while IFS= read -r ets; do
    local relative="${ets#"$root"/ani-ets/}"
    # Native declarations are the ETS-facing ABI: symbol name, parameter
    # direction and ANI-lowered public types must all remain compatible.
    sed -n \
      -e '/native /p' \
      -e '/^export type /p' \
      "$ets" |
      sed -e 's/[[:space:]]\+/ /g' -e 's/^ /\t/' |
      while IFS= read -r declaration; do
        printf '%s\t%s\n' "$relative" "$declaration"
      done >>"$output"
  done < <(find "$root/ani-ets" -type f -name '*.ets' | sort)
  LC_ALL=C sort -u -o "$output" "$output"
}

normalize_abi "$current_target" "$work_root/current-ets-abi.tsv"
normalize_abi "$baseline_target" "$work_root/baseline-ets-abi.tsv"

echo "[3/4] Reject removed or changed ETS/ANI signatures"
missing="$work_root/missing-ets-abi.tsv"
comm -23 "$work_root/baseline-ets-abi.tsv" "$work_root/current-ets-abi.tsv" >"$missing"
if [[ -s "$missing" ]]; then
  echo "Breaking ETS/ANI declarations relative to $baseline_rev:" >&2
  cat "$missing" >&2
  exit 1
fi

echo "[4/4] Verify native image lifecycle symbols"
cargo build -p ani-example-new-basic >/dev/null
(
  cd "$baseline_root"
  cargo build -p ani-example-new-basic --target-dir "$baseline_target/native" >/dev/null
)
current_lib="$repo_root/target/debug/libani_example_new_basic.$([[ "$(uname)" == Darwin ]] && echo dylib || echo so)"
baseline_lib="$baseline_target/native/debug/libani_example_new_basic.$([[ "$(uname)" == Darwin ]] && echo dylib || echo so)"
nm_flags=(-g)
if [[ "$(uname)" != Darwin ]]; then
  nm_flags=(-D --defined-only)
fi

has_symbol() {
  local library="$1"
  local expected="$2"
  # Mach-O prefixes C symbols with `_`; ELF does not. Compare the normalized
  # final nm column so the gate has identical semantics on macOS and Linux.
  nm "${nm_flags[@]}" "$library" |
    awk -v expected="$expected" '
      {
        name = $NF
        sub(/^_/, "", name)
        if (name == expected) found = 1
      }
      END { exit(found ? 0 : 1) }
    '
}

for symbol in ANI_Constructor ANI_Destructor; do
  if ! has_symbol "$current_lib" "$symbol"; then
    echo "Current image removed required symbol: $symbol" >&2
    exit 1
  fi
  # The first release introducing this gate may not yet have had a destructor.
  # Once present in a baseline it remains protected by the same current-image
  # requirement above.
  if has_symbol "$baseline_lib" "$symbol"; then
    echo "  preserved $symbol"
  else
    echo "  introduced $symbol (not present in $baseline_rev)"
  fi
done

echo "OK: Rust semver is complemented by ETS declarations, ANI signatures, and native symbols."
