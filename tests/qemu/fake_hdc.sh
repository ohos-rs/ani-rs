#!/usr/bin/env bash
set -euo pipefail

command_line="$*"
case "$command_line" in
  *"param get const.ohos.apiversion"*)
    echo 26
    ;;
  *"param get const.product.software.version"*)
    echo "OpenHarmony 7.0.0.32"
    ;;
  *"uname -r"*)
    echo "6.6.101"
    ;;
  *"cat /proc/cpuinfo"*)
    printf '%s\n' \
      'processor : 0' \
      "vendor_id : ${FAKE_CPU_VENDOR:?FAKE_CPU_VENDOR is required}" \
      'cpu family : 6' \
      'model : 60' \
      'model name : QEMU Haswell-v2 test CPU' \
      "flags : ${FAKE_CPU_FLAGS:?FAKE_CPU_FLAGS is required}"
    ;;
  *)
    echo "unexpected fake hdc command: $command_line" >&2
    exit 1
    ;;
esac
