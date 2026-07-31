#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
arch="${2:-${OHOS_QEMU_GUEST_ARCH:-arm64}}"
hap="${1:-$repo_root/target/hap-smoke-$arch/project/entry/build/default/outputs/default/entry-default-unsigned.hap}"

export OHOS_QEMU_GUEST_ARCH="$arch"
export OHOS_QEMU_HAP="$hap"
if [[ -n "${OHOS_QEMU_HAP_QEMU_WORK_ROOT:-}" ]]; then
  export OHOS_QEMU_WORK_ROOT="$OHOS_QEMU_HAP_QEMU_WORK_ROOT"
elif [[ -n "${OHOS_QEMU_WORK_ROOT:-}" ]]; then
  export OHOS_QEMU_WORK_ROOT="$OHOS_QEMU_WORK_ROOT-hap-$arch"
else
  export OHOS_QEMU_WORK_ROOT="$repo_root/target/ohos-qemu-hap-$arch"
fi

"$repo_root/scripts/run_arkvm_examples_ohos_qemu.sh"
