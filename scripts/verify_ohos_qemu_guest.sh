#!/usr/bin/env bash
set -euo pipefail

hdc_target="${HDC_TARGET:-}"
expected_api_level="${OHOS_QEMU_EXPECTED_API_LEVEL:-26}"
expected_kernel="${OHOS_QEMU_EXPECTED_KERNEL:-6.6.101}"
guest_arch="${OHOS_QEMU_GUEST_ARCH:-}"
x86_cpu_baseline="${OHOS_QEMU_X86_CPU_MODEL:-Haswell-v2}"

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

if [[ "$guest_arch" == "x86_64" ]]; then
  if [[ "$x86_cpu_baseline" != "Haswell-v2" ]]; then
    echo "::error::Unsupported x86_64 CPU baseline: $x86_cpu_baseline." >&2
    exit 2
  fi

  cpuinfo="$(
    "$hdc_bin" -t "$hdc_target" shell cat /proc/cpuinfo | tr -d '\r'
  )"
  cpu_vendor="$(
    awk -F: '/^vendor_id[[:space:]]*:/ {
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", $2)
      print $2
      exit
    }' <<<"$cpuinfo"
  )"
  cpu_family="$(
    awk -F: '/^cpu family[[:space:]]*:/ {
      gsub(/[[:space:]]/, "", $2)
      print $2
      exit
    }' <<<"$cpuinfo"
  )"
  cpu_model_id="$(
    awk -F: '/^model[[:space:]]*:/ {
      gsub(/[[:space:]]/, "", $2)
      print $2
      exit
    }' <<<"$cpuinfo"
  )"
  cpu_model_name="$(
    awk -F: '/^model name[[:space:]]*:/ {
      sub(/^[^:]*:[[:space:]]*/, "")
      print
      exit
    }' <<<"$cpuinfo"
  )"
  cpu_flags="$(
    awk -F: '/^flags[[:space:]]*:/ {
      sub(/^[^:]*:[[:space:]]*/, "")
      print
      exit
    }' <<<"$cpuinfo"
  )"

  if [[ "$cpu_vendor" != "GenuineIntel" || "$cpu_family" != "6" ||
    "$cpu_model_id" != "60" ]]; then
    echo "::error::Expected $x86_cpu_baseline CPUID GenuineIntel/6/60, got ${cpu_vendor:-unknown}/${cpu_family:-unknown}/${cpu_model_id:-unknown}." >&2
    exit 1
  fi
  for required_flag in xsave avx avx2; do
    if [[ " $cpu_flags " != *" $required_flag "* ]]; then
      echo "::error::$x86_cpu_baseline guest CPU is missing required flag $required_flag." >&2
      exit 1
    fi
  done
  for cpu_flag in $cpu_flags; do
    if [[ "$cpu_flag" == avx512* ]]; then
      echo "::error::$x86_cpu_baseline guest unexpectedly exposes $cpu_flag." >&2
      exit 1
    fi
  done

  echo "x86 CPU: $cpu_model_name ($x86_cpu_baseline, AVX/AVX2 enabled, AVX-512 disabled)"
fi
