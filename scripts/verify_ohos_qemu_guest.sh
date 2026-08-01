#!/usr/bin/env bash
set -euo pipefail

hdc_target="${HDC_TARGET:-}"
expected_api_level="${OHOS_QEMU_EXPECTED_API_LEVEL:-26}"
expected_kernel="${OHOS_QEMU_EXPECTED_KERNEL:-6.6.101}"

if [[ -z "$hdc_target" ]]; then
  echo "HDC_TARGET is required" >&2
  exit 2
fi

if [[ -n "${HDC_BIN:-}" ]]; then
  hdc_bin="$HDC_BIN"
elif command -v hdc >/dev/null 2>&1; then
  hdc_bin="$(command -v hdc)"
elif [[ -n "${DEVECO_SDK_ROOT:-}" ]]; then
  hdc_bin="$DEVECO_SDK_ROOT/toolchains/hdc"
else
  echo "unable to find hdc; set HDC_BIN or DEVECO_SDK_ROOT" >&2
  exit 2
fi

if [[ ! -x "$hdc_bin" ]]; then
  echo "hdc is not executable: $hdc_bin" >&2
  exit 2
fi

api_source="parameter service"
api_level="$(
  "$hdc_bin" -t "$hdc_target" shell 'param get const.ohos.apiversion' \
    2>/dev/null | tr -d '\r[:space:]' || true
)"

# The v20260731 ARMv7A image contains the API property in its immutable system
# parameter file, but its parameter service does not publish the nested
# ohos_const property. Read the same signed image metadata as a fallback.
if [[ ! "$api_level" =~ ^[0-9]+$ ]]; then
  api_source="system parameter file"
  parameter_contents="$(
    "$hdc_bin" -t "$hdc_target" shell '
for parameter_file in \
  /usr/etc/param/ohos_const/ohos.para \
  /system/etc/param/ohos_const/ohos.para \
  /usr/system/etc/param/ohos_const/ohos.para \
  /etc/param/ohos_const/ohos.para
do
  if [ -r "$parameter_file" ]; then
    cat "$parameter_file"
    exit 0
  fi
done
exit 1
' 2>/dev/null || true
  )"
  api_level="$(
    printf '%s\n' "$parameter_contents" | tr -d '\r' | awk -F= '
      $1 == "const.ohos.apiversion" && $2 ~ /^[0-9]+$/ {
        print $2
        exit
      }
    '
  )"
fi

kernel="$(
  "$hdc_bin" -t "$hdc_target" shell uname -r | tr -d '\r[:space:]'
)"
version="$(
  "$hdc_bin" -t "$hdc_target" shell \
    'param get const.product.software.version' | tr -d '\r'
)"

if [[ ! "$api_level" =~ ^[0-9]+$ ]]; then
  echo "OpenHarmony: ${version:-unknown} (API unavailable, kernel ${kernel:-unknown})"
  echo "::error::Unable to determine the OpenHarmony API level from the parameter service or system parameter file." >&2
  exit 1
fi

echo "OpenHarmony: $version (API $api_level via $api_source, kernel $kernel)"

if [[ "$api_level" != "$expected_api_level" ]]; then
  echo "::error::Expected OpenHarmony API $expected_api_level, got $api_level." >&2
  exit 1
fi
if [[ "$kernel" != "$expected_kernel" ]]; then
  echo "::error::Expected OpenHarmony kernel $expected_kernel, got $kernel." >&2
  exit 1
fi
