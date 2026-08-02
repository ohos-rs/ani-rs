#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
verifier="$repo_root/scripts/verify_ohos_qemu_guest.sh"
fake_hdc="$repo_root/tests/qemu/fake_hdc.sh"
baseline_flags='fpu sse sse2 xsave avx avx2'

run_verifier() {
  local vendor="$1"
  local flags="$2"

  env \
    HDC_TARGET=127.0.0.1:5558 \
    HDC_BIN="$fake_hdc" \
    OHOS_QEMU_GUEST_ARCH=x86_64 \
    OHOS_QEMU_X86_CPU_MODEL=Haswell-v2 \
    FAKE_CPU_VENDOR="$vendor" \
    FAKE_CPU_FLAGS="$flags" \
    "$verifier"
}

for vendor in GenuineIntel AuthenticAMD; do
  output="$(run_verifier "$vendor" "$baseline_flags")"
  if [[ "$output" != *"x86 CPU: $vendor"* ]]; then
    echo "expected $vendor guest CPU verification to pass" >&2
    exit 1
  fi
done

if output="$(run_verifier AuthenticAMD "$baseline_flags avx512f" 2>&1)"; then
  echo "expected AVX-512 guest CPU verification to fail" >&2
  exit 1
fi
if [[ "$output" != *"guest unexpectedly exposes avx512f"* ]]; then
  echo "AVX-512 rejection did not report the offending feature" >&2
  exit 1
fi

if output="$(run_verifier CentaurHauls "$baseline_flags" 2>&1)"; then
  echo "expected unknown x86 CPU vendor verification to fail" >&2
  exit 1
fi
if [[ "$output" != *"unsupported CPU vendor CentaurHauls"* ]]; then
  echo "unknown CPU vendor rejection did not report the vendor" >&2
  exit 1
fi

echo "OK: Intel and AMD Haswell-v2 guests pass; AVX-512 and unknown vendors fail."
