#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
arch="${1:-arm64}"
work_root="${ANI_HAP_REPRO_WORK_ROOT:-$repo_root/target/hap-reproducible-$arch}"

for run in first second; do
  ANI_HAP_WORK_ROOT="$work_root/$run" \
    "$repo_root/scripts/build_hap_smoke.sh" "$arch"
  hap="$work_root/$run/project/entry/build/default/outputs/default/entry-default-unsigned.hap"
  rm -rf "$work_root/$run/unpacked"
  mkdir -p "$work_root/$run/unpacked"
  unzip -qq "$hap" -d "$work_root/$run/unpacked"
done

if ! diff -qr "$work_root/first/unpacked" "$work_root/second/unpacked"; then
  echo "HAP_REPRODUCIBILITY: FAIL" >&2
  exit 1
fi

abc="$work_root/first/unpacked/resources/rawfile/ani_rs_smoke.abc"
echo "HAP_CONTENT_SHA256: $(sha256sum "$abc" | awk '{print $1}')"
echo "HAP_REPRODUCIBILITY: PASS"
