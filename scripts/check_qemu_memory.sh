#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Exercise ownership-heavy and asynchronous paths repeatedly in one ANI VM.
# The threshold covers lazy Ark runtime initialization while still catching
# unbounded native/global-reference growth.
export OHOS_QEMU_PACKAGE_FILTER='ani-example-(arraybuffer|async-wrapper|wrap-ptr)'
short_iterations="${OHOS_QEMU_SHORT_ITERATIONS:-50}"
long_iterations="${OHOS_QEMU_LONG_ITERATIONS:-100}"
if [[ -n "${OHOS_QEMU_MEMORY_WORK_ROOT:-}" ]]; then
  base_work_root="$OHOS_QEMU_MEMORY_WORK_ROOT"
elif [[ -n "${OHOS_QEMU_WORK_ROOT:-}" ]]; then
  base_work_root="$OHOS_QEMU_WORK_ROOT-memory"
else
  base_work_root="$repo_root/target/ohos-qemu-memory"
fi
max_slope_kb="${OHOS_QEMU_MAX_PSS_SLOPE_KB:-8192}"
export OHOS_QEMU_MEMORY_SAMPLE=1
export OHOS_QEMU_MAX_PSS_GROWTH_KB="${OHOS_QEMU_MAX_PSS_GROWTH_KB:-32768}"
export OHOS_QEMU_CASE_TIMEOUT="${OHOS_QEMU_CASE_TIMEOUT:-120}"
# Repeated reflective calls currently hit an Ark runtime JIT code-cache crash
# after roughly two dozen async invocations on the 20260728 image. Leak checks
# exercise the same ANI/native ownership paths under the interpreter so that a
# platform JIT failure cannot hide a native memory regression.
export OHOS_QEMU_DISABLE_JIT="${OHOS_QEMU_DISABLE_JIT:-1}"

for iterations in "$short_iterations" "$long_iterations"; do
  export OHOS_QEMU_ITERATIONS="$iterations"
  export OHOS_QEMU_WORK_ROOT="$base_work_root/run-$iterations"
  "$repo_root/scripts/run_arkvm_examples_ohos_qemu.sh"
done

mkdir -p "$base_work_root"
head -n 1 "$base_work_root/run-$short_iterations/memory.tsv" >"$base_work_root/memory.tsv"
tail -n +2 "$base_work_root/run-$short_iterations/memory.tsv" >>"$base_work_root/memory.tsv"
tail -n +2 "$base_work_root/run-$long_iterations/memory.tsv" >>"$base_work_root/memory.tsv"
head -n 1 "$base_work_root/run-$short_iterations/report.tsv" >"$base_work_root/report.tsv"
tail -n +2 "$base_work_root/run-$short_iterations/report.tsv" >>"$base_work_root/report.tsv"
tail -n +2 "$base_work_root/run-$long_iterations/report.tsv" >>"$base_work_root/report.tsv"

if awk -F '\t' -v short="$short_iterations" -v long="$long_iterations" \
  -v limit="$max_slope_kb" '
    NR == 1 { next }
    $8 != "PASS" { failed = 1 }
    $3 == short { short_growth[$2] = $6 }
    $3 == long { long_growth[$2] = $6 }
    END {
      for (package in short_growth) {
        if (!(package in long_growth) ||
            long_growth[package] - short_growth[package] > limit) {
          failed = 1
        }
      }
      exit failed
    }
  ' "$base_work_root/memory.tsv"; then
  echo "QEMU_MEMORY_REPORT: $base_work_root/memory.tsv"
  echo "QEMU_MEMORY_RESULT: PASS"
else
  echo "QEMU_MEMORY_REPORT: $base_work_root/memory.tsv" >&2
  echo "QEMU_MEMORY_RESULT: FAIL" >&2
  exit 1
fi
