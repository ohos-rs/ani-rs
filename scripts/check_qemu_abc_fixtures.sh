#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
manifest="$repo_root/tests/qemu-abc-fixtures.sha256"

sha256_file() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  elif command -v shasum >/dev/null 2>&1; then
    LC_ALL=C LANG=C shasum -a 256 "$1" | awk '{print $1}'
  elif command -v openssl >/dev/null 2>&1; then
    openssl dgst -sha256 "$1" | awk '{print $NF}'
  else
    echo "missing SHA-256 implementation" >&2
    return 127
  fi
}

if [[ ! -f "$manifest" ]]; then
  echo "missing QEMU ABC fixture manifest: $manifest" >&2
  exit 1
fi

temp_root="$(mktemp -d "${TMPDIR:-/tmp}/ani-rs-abc-fixtures.XXXXXX")"
trap 'rm -rf "$temp_root"' EXIT
hash_list="$temp_root/hashes"
: > "$hash_list"

fixture_count=0
while IFS= read -r ets_file; do
  abc_file="${ets_file%.ets}.abc"
  if [[ ! -f "$abc_file" ]]; then
    echo "missing QEMU ABC fixture for ${ets_file#"$repo_root/"}: $abc_file" >&2
    exit 1
  fi
  ets_relative="${ets_file#"$repo_root/"}"
  abc_relative="${abc_file#"$repo_root/"}"
  printf '%s  %s\n' "$(sha256_file "$ets_file")" "$ets_relative" >> "$hash_list"
  printf '%s  %s\n' "$(sha256_file "$abc_file")" "$abc_relative" >> "$hash_list"
  ((fixture_count += 1))
done < <(LC_ALL=C find "$repo_root/examples" -mindepth 2 -maxdepth 2 \
  -name arkvm_test.ets -type f | LC_ALL=C sort)

if [[ "$fixture_count" -ne 52 ]]; then
  echo "expected 52 QEMU ABC fixtures, found $fixture_count" >&2
  exit 1
fi
abc_count=0
while IFS= read -r _abc_file; do
  ((abc_count += 1))
done < <(LC_ALL=C find "$repo_root/examples" -mindepth 2 -maxdepth 2 \
  -name arkvm_test.abc -type f | LC_ALL=C sort)
if [[ "$abc_count" -ne "$fixture_count" ]]; then
  echo "expected exactly $fixture_count example ABC files, found $abc_count" >&2
  exit 1
fi

for relative in \
  scripts/ohos_qemu_abc_launcher.ets \
  scripts/ohos_qemu_abc_launcher.abc; do
  fixture="$repo_root/$relative"
  if [[ ! -f "$fixture" ]]; then
    echo "missing QEMU launcher fixture: $fixture" >&2
    exit 1
  fi
  printf '%s  %s\n' "$(sha256_file "$fixture")" "$relative" >> "$hash_list"
done

expected="$(awk '$1 !~ /^#/ && NF == 2 { print $1; exit }' "$manifest")"
actual="$(sha256_file "$hash_list")"
if [[ -z "$expected" || "$actual" != "$expected" ]]; then
  echo "QEMU ABC fixtures do not match their ArkTS sources." >&2
  echo "expected aggregate: ${expected:-missing}" >&2
  echo "actual aggregate:   $actual" >&2
  echo "Regenerate every fixture with the matching OpenHarmony es2panda, then update $manifest." >&2
  exit 1
fi

echo "OK: $fixture_count QEMU ABC fixtures and the launcher match $expected."
