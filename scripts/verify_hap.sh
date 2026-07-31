#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: scripts/verify_hap.sh <signed-or-unsigned.hap> [arm64|x86_64|armv7a]

Checks that a HAP contains ABC bytecode and correctly-typed ANI shared
libraries. Set DEVECO_SDK_ROOT when the SDK is not at the default macOS path.
EOF
}

if [[ $# -lt 1 || $# -gt 2 ]]; then
  usage >&2
  exit 2
fi

hap_file="$1"
requested_arch="${2:-}"
sdk_root="${DEVECO_SDK_ROOT:-/Applications/DevEco-Studio.app/Contents/sdk/default/openharmony}"
llvm_bin="$sdk_root/native/llvm/bin"

if [[ ! -f "$hap_file" ]]; then
  echo "HAP does not exist: $hap_file" >&2
  exit 1
fi
for tool in unzip "$llvm_bin/llvm-readelf" "$llvm_bin/llvm-nm"; do
  if [[ "$tool" == "unzip" ]]; then
    command -v unzip >/dev/null || {
      echo "missing command: unzip" >&2
      exit 1
    }
  elif [[ ! -x "$tool" ]]; then
    echo "missing SDK tool: $tool" >&2
    exit 1
  fi
done

case "$requested_arch" in
  "")
    requested_abi=""
    requested_machine=""
    ;;
  arm64)
    requested_abi="arm64-v8a"
    requested_machine="AArch64"
    ;;
  x86_64)
    requested_abi="x86_64"
    requested_machine="Advanced Micro Devices X86-64"
    ;;
  armv7a)
    requested_abi="armeabi-v7a"
    requested_machine="ARM"
    ;;
  *)
    echo "unsupported architecture: $requested_arch" >&2
    exit 2
    ;;
esac

entries="$(unzip -Z1 "$hap_file")"
if ! grep -Eq '(^|/).+\.abc$' <<<"$entries"; then
  echo "HAP contains no .abc bytecode: $hap_file" >&2
  exit 1
fi

libraries=()
while IFS= read -r entry; do
  [[ -n "$entry" ]] && libraries+=("$entry")
done < <(grep -E '(^|/)libs/[^/]+/lib[^/]+\.so$' <<<"$entries" || true)

if [[ ${#libraries[@]} -eq 0 ]]; then
  echo "HAP contains no native libraries under libs/<abi>/: $hap_file" >&2
  exit 1
fi

temp_root="$(mktemp -d "${TMPDIR:-/tmp}/ani-rs-hap.XXXXXX")"
trap 'rm -rf "$temp_root"' EXIT

checked=0
for entry in "${libraries[@]}"; do
  abi="$(sed -E 's#^(.*/)?libs/([^/]+)/.*$#\2#' <<<"$entry")"
  case "$abi" in
    arm64-v8a) expected_machine="AArch64" ;;
    x86_64) expected_machine="Advanced Micro Devices X86-64" ;;
    armeabi-v7a) expected_machine="ARM" ;;
    *)
      echo "unsupported HAP ABI directory: $abi ($entry)" >&2
      exit 1
      ;;
  esac
  if [[ -n "$requested_abi" && "$abi" != "$requested_abi" ]]; then
    continue
  fi
  if [[ -n "$requested_machine" && "$expected_machine" != "$requested_machine" ]]; then
    echo "internal architecture mapping mismatch for $entry" >&2
    exit 1
  fi

  extracted="$temp_root/$(basename "$entry").$abi"
  unzip -p "$hap_file" "$entry" > "$extracted"
  elf_header="$("$llvm_bin/llvm-readelf" -h "$extracted")"
  if ! grep -Fq "Machine:                           $expected_machine" <<<"$elf_header"; then
    echo "wrong ELF machine for $entry; expected $expected_machine" >&2
    exit 1
  fi
  dynamic_symbols="$("$llvm_bin/llvm-nm" -D --defined-only "$extracted")"
  if ! grep -Eq '[[:space:]]ANI_Constructor$' <<<"$dynamic_symbols"; then
    echo "ANI_Constructor is not exported by $entry" >&2
    exit 1
  fi
  ((checked += 1))
done

if [[ "$checked" -eq 0 ]]; then
  echo "HAP contains no libraries for requested architecture $requested_arch" >&2
  exit 1
fi

echo "OK: $hap_file contains ABC bytecode and $checked valid ANI library/libraries${requested_arch:+ for $requested_arch}."
