#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Exercise representative synchronous calls, typed buffers, callbacks and the
# shared async runtime in the real guest. The upstream v20260731 release
# baseline supports repeated RuntimeKernel restart under JIT, so both phases
# use the same iteration count unless a runner explicitly overrides the async
# phase.
sync_iterations="${OHOS_QEMU_PERF_ITERATIONS:-10}"
async_iterations="${OHOS_QEMU_PERF_ASYNC_ITERATIONS:-$sync_iterations}"
export OHOS_QEMU_MAX_PER_ITERATION_US="${OHOS_QEMU_MAX_PER_ITERATION_US:-15000000}"
export OHOS_QEMU_CASE_TIMEOUT="${OHOS_QEMU_CASE_TIMEOUT:-300}"
export OHOS_QEMU_MEMORY_SAMPLE=0
export OHOS_QEMU_DISABLE_JIT="${OHOS_QEMU_PERF_DISABLE_JIT:-0}"
export OHOS_QEMU_SKIP_BUILD="${OHOS_QEMU_SKIP_BUILD:-1}"

if [[ -n "${OHOS_QEMU_PERFORMANCE_WORK_ROOT:-}" ]]; then
  base_work_root="$OHOS_QEMU_PERFORMANCE_WORK_ROOT"
elif [[ -n "${OHOS_QEMU_WORK_ROOT:-}" ]]; then
  base_work_root="$OHOS_QEMU_WORK_ROOT-performance"
else
  base_work_root="$repo_root/target/ohos-qemu-performance"
fi
if [[ "$base_work_root" != /* ]]; then
  base_work_root="$repo_root/$base_work_root"
fi

run_phase() {
  local name="$1"
  local filter="$2"
  local iterations="$3"
  export OHOS_QEMU_PACKAGE_FILTER="$filter"
  export OHOS_QEMU_ITERATIONS="$iterations"
  export OHOS_QEMU_WORK_ROOT="$base_work_root/$name"
  "$repo_root/scripts/run_arkvm_examples_ohos_qemu.sh"
}

run_phase sync '^ani-example-(arraybuffer|function|new-basic)$' "$sync_iterations"
run_phase async '^ani-example-async-wrapper$' "$async_iterations"

mkdir -p "$base_work_root"
for report in report.tsv memory.tsv performance.tsv; do
  head -n 1 "$base_work_root/sync/$report" >"$base_work_root/$report"
  tail -n +2 "$base_work_root/sync/$report" >>"$base_work_root/$report"
  tail -n +2 "$base_work_root/async/$report" >>"$base_work_root/$report"
done

echo "QEMU_PERFORMANCE_REPORT: $base_work_root/performance.tsv"
echo "QEMU_PERFORMANCE_RESULT: PASS"
