#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

hdc_target="${HDC_TARGET:-127.0.0.1:5558}"
guest_arch="${OHOS_QEMU_GUEST_ARCH:-arm64}"
ohos_source_root="${OHOS_SOURCE_ROOT:-/tmp/openharmony}"
deveco_sdk_root="${DEVECO_SDK_ROOT:-/Applications/DevEco-Studio.app/Contents/sdk/default/openharmony}"
qemu_packages_root="${QEMU_PACKAGES_ROOT:-}"
require_package_process="${OHOS_QEMU_REQUIRE_PACKAGE_PROCESS:-0}"
hap_input="${OHOS_QEMU_HAP:-}"
work_root="${OHOS_QEMU_WORK_ROOT:-$repo_root/target/ohos-qemu}"
remote_root="${OHOS_QEMU_REMOTE_ROOT:-/data/local/tmp/ani-rs-qemu}"
case_timeout="${OHOS_QEMU_CASE_TIMEOUT:-45}"
case_attempts="${OHOS_QEMU_CASE_ATTEMPTS:-3}"
hdc_timeout="${OHOS_QEMU_HDC_TIMEOUT:-60}"
hdc_runtime_timeout="${OHOS_QEMU_HDC_RUNTIME_TIMEOUT:-}"
package_filter="${OHOS_QEMU_PACKAGE_FILTER:-}"
runner_asan="${OHOS_QEMU_RUNNER_ASAN:-0}"
rust_asan="${OHOS_QEMU_RUST_ASAN:-0}"
lsan="${OHOS_QEMU_LSAN:-0}"
runner_iterations="${OHOS_QEMU_ITERATIONS:-1}"
memory_sample="${OHOS_QEMU_MEMORY_SAMPLE:-${OHOS_QEMU_MEMORY_SAMPLE_EVERY:-0}}"
max_pss_growth_kb="${OHOS_QEMU_MAX_PSS_GROWTH_KB:-}"
max_per_iteration_us="${OHOS_QEMU_MAX_PER_ITERATION_US:-}"

for numeric in \
  "$runner_iterations" \
  "$memory_sample" \
  "$case_timeout" \
  "$case_attempts" \
  "$hdc_timeout"; do
  if [[ ! "$numeric" =~ ^[0-9]+$ ]]; then
    echo "QEMU iteration, attempt, and timeout values must be non-negative integers" >&2
    exit 2
  fi
done
if [[ -n "$hdc_runtime_timeout" && ! "$hdc_runtime_timeout" =~ ^[0-9]+$ ]]; then
  echo "OHOS_QEMU_HDC_RUNTIME_TIMEOUT must be a non-negative integer" >&2
  exit 2
fi
if [[ -n "$max_pss_growth_kb" && ! "$max_pss_growth_kb" =~ ^[0-9]+$ ]]; then
  echo "OHOS_QEMU_MAX_PSS_GROWTH_KB must be a non-negative integer" >&2
  exit 2
fi
if [[ -n "$max_per_iteration_us" && ! "$max_per_iteration_us" =~ ^[0-9]+$ ]]; then
  echo "OHOS_QEMU_MAX_PER_ITERATION_US must be a non-negative integer" >&2
  exit 2
fi
if [[ "$runner_iterations" == "0" ]]; then
  echo "OHOS_QEMU_ITERATIONS must be greater than zero" >&2
  exit 2
fi
if [[ -z "$hdc_runtime_timeout" ]]; then
  hdc_runtime_timeout=$((case_timeout + 30))
fi
if [[ "$case_timeout" == "0" || "$case_attempts" == "0" || \
  "$hdc_timeout" == "0" || "$hdc_runtime_timeout" == "0" ]]; then
  echo "QEMU case, attempt, and HDC timeout values must be greater than zero" >&2
  exit 2
fi

case "$guest_arch" in
  arm64)
    rust_target="aarch64-unknown-linux-ohos"
    clang_triple="aarch64-unknown-linux-ohos"
    expected_uname='aarch64|arm64'
    expected_elf_machine='AArch64'
    qemu_package='openharmony-qemu-arm64-arm64_virt'
    hap_abi='arm64-v8a'
    ;;
  x86_64)
    rust_target="x86_64-unknown-linux-ohos"
    clang_triple="x86_64-unknown-linux-ohos"
    expected_uname='x86_64'
    expected_elf_machine='Advanced Micro Devices X86-64'
    qemu_package='openharmony-qemu-x86_64-x86_64_virt'
    hap_abi='x86_64'
    ;;
  armv7a)
    rust_target="armv7-unknown-linux-ohos"
    clang_triple="armv7-unknown-linux-ohos"
    expected_uname='armv7l|armv7|arm'
    expected_elf_machine='ARM'
    qemu_package='openharmony-qemu-armv7a-armv7a_virt'
    hap_abi='armeabi-v7a'
    ;;
  *)
    echo "unsupported OHOS_QEMU_GUEST_ARCH: $guest_arch (expected arm64, x86_64, or armv7a)" >&2
    exit 2
    ;;
esac

rust_target_env="$(printf '%s' "$rust_target" | tr '[:lower:]-' '[:upper:]_')"
rust_target_env="CARGO_TARGET_${rust_target_env}_LINKER"
cc_target_env="CC_$(printf '%s' "$rust_target" | tr '-' '_')"
cxx_target_env="CXX_$(printf '%s' "$rust_target" | tr '-' '_')"

if [[ "$work_root" != /* ]]; then
  work_root="$repo_root/$work_root"
fi

clang_bin="$deveco_sdk_root/native/llvm/bin"
sysroot="$deveco_sdk_root/native/sysroot"
hdc_bin="${HDC_BIN:-}"
es2panda_dir="$ohos_source_root/out/arm64_virt/clang_x64/arkcompiler/ets_frontend"
es2panda="${OHOS_ES2PANDA:-$es2panda_dir/es2panda}"
arktsconfig="${OHOS_ARKTSCONFIG:-$es2panda_dir/arktsconfig.json}"
if [[ -z "${OHOS_ES2PANDA:-}" && ! -f "$es2panda" ]]; then
  host_tools="$ohos_source_root/out/x86_64_virt/clang_x64"
  candidate="$host_tools/exe.unstripped/clang_x64/arkcompiler/ets_frontend/es2panda"
  [[ -f "$candidate" ]] && es2panda="$candidate"
fi
if [[ -z "${OHOS_ARKTSCONFIG:-}" && ! -f "$arktsconfig" ]]; then
  candidate="$ohos_source_root/out/x86_64_virt/clang_x64/arkcompiler/ets_frontend/arktsconfig.json"
  [[ -f "$candidate" ]] && arktsconfig="$candidate"
fi
runner="$work_root/ani_abc_runner"
launcher_abc="$work_root/ohos_qemu_abc_launcher.abc"
report="$work_root/report.tsv"
memory_report="$work_root/memory.tsv"
performance_report="$work_root/performance.tsv"

if [[ -n "$qemu_packages_root" ]]; then
  package_root="$qemu_packages_root/$qemu_package"
  manifest="$package_root/manifest.json"
  if [[ ! -f "$manifest" ]] ||
    ! grep -q "\"guest_arch\": \"$guest_arch\"" "$manifest"; then
    echo "QEMU package does not match $guest_arch: $package_root" >&2
    exit 1
  fi
  if [[ "$require_package_process" == "1" ]] &&
    ! ps -ax -o command= | grep -F "$package_root/images/" | grep -v grep >/dev/null; then
    echo "the requested QEMU package is not running: $package_root" >&2
    exit 1
  fi
  echo "QEMU_PACKAGE: $package_root"
fi

for required in \
  "$clang_bin/$clang_triple-clang" \
  "$clang_bin/$clang_triple-clang++" \
  "$clang_bin/llvm-readelf" \
  "$es2panda" \
  "$arktsconfig"; do
  if [[ ! -f "$required" ]]; then
    echo "missing required tool/file: $required" >&2
    exit 1
  fi
done

if [[ -z "$hdc_bin" ]]; then
  if command -v hdc >/dev/null 2>&1; then
    hdc_bin="$(command -v hdc)"
  elif [[ -x "$deveco_sdk_root/toolchains/hdc" ]]; then
    hdc_bin="$deveco_sdk_root/toolchains/hdc"
  else
    echo "missing hdc; set HDC_BIN or add hdc to PATH" >&2
    exit 1
  fi
fi

if [[ ! -x "$hdc_bin" ]]; then
  echo "HDC_BIN is not executable: $hdc_bin" >&2
  exit 1
fi

run_with_timeout() {
  local timeout_seconds="$1"
  shift

  if command -v timeout >/dev/null 2>&1; then
    timeout -k 2 "$timeout_seconds" "$@"
    return
  fi
  if command -v gtimeout >/dev/null 2>&1; then
    gtimeout -k 2 "$timeout_seconds" "$@"
    return
  fi
  if ! command -v perl >/dev/null 2>&1; then
    echo "missing timeout, gtimeout, or perl for the host HDC watchdog" >&2
    return 127
  fi

  "$@" &
  local command_pid=$!
  LC_ALL=C LANG=C perl -e '
    my ($timeout, $pid) = @ARGV;
    sleep $timeout;
    if (kill 0, $pid) {
      warn "host timeout after ${timeout}s\n";
      kill 15, $pid;
      sleep 2;
      kill 9, $pid if kill 0, $pid;
    }
  ' "$timeout_seconds" "$command_pid" &
  local watchdog_pid=$!
  local status=0

  wait "$command_pid" || status=$?
  kill "$watchdog_pid" 2>/dev/null || true
  wait "$watchdog_pid" 2>/dev/null || true
  return "$status"
}

target_connected() {
  run_with_timeout "$hdc_timeout" "$hdc_bin" list targets |
    tr -d '\r' |
    grep -Fxq "$hdc_target"
}

for _ in $(seq 1 30); do
  target_connected && break
  run_with_timeout "$hdc_timeout" "$hdc_bin" tconn "$hdc_target" \
    >/dev/null 2>&1 || true
  sleep 1
done
if ! target_connected; then
  echo "HDC target is not connected: $hdc_target" >&2
  exit 1
fi

device_arch="$(run_with_timeout "$hdc_timeout" \
  "$hdc_bin" -t "$hdc_target" shell uname -m | tr -d '\r[:space:]')"
if [[ ! "$device_arch" =~ ^($expected_uname)$ ]]; then
  echo "HDC target architecture mismatch: requested $guest_arch, device reports $device_arch" >&2
  exit 1
fi

mkdir -p "$work_root"

hap_extract_root="$work_root/hap"
if [[ -n "$hap_input" ]]; then
  if [[ ! -f "$hap_input" ]]; then
    echo "HAP does not exist: $hap_input" >&2
    exit 1
  fi
  "$repo_root/scripts/verify_hap.sh" "$hap_input" "$guest_arch"
  package_filter='^ani-example-new-basic$'
  mkdir -p "$hap_extract_root"
  unzip -p "$hap_input" resources/rawfile/ani_rs_smoke.abc \
    >"$hap_extract_root/arkvm_test.abc"
  unzip -p "$hap_input" "libs/$hap_abi/libani_example_new_basic.so" \
    >"$hap_extract_root/libani_example_new_basic.so"
  echo "HAP_INPUT: $hap_input"
fi

runner_cxxflags=(-O2)
runner_runtime_env="ANI_QEMU_ITERATIONS=$runner_iterations"
if [[ "$memory_sample" != "0" ]]; then
  runner_runtime_env+=" ANI_QEMU_MEMORY_SAMPLE=1"
fi
if [[ "${OHOS_QEMU_DISABLE_JIT:-0}" == "1" ]]; then
  runner_runtime_env+=" ANI_QEMU_DISABLE_JIT=1"
fi
if [[ "$runner_asan" == "1" ]]; then
  runner_cxxflags=(-O1 -g -fno-omit-frame-pointer -fsanitize=address)
  runner_runtime_env+=" ASAN_OPTIONS=detect_leaks=$lsan:halt_on_error=1"
fi

"$clang_bin/$clang_triple-clang++" \
  --sysroot="$sysroot" \
  -std=c++17 \
  "${runner_cxxflags[@]}" \
  -I "$repo_root/include" \
  "$repo_root/scripts/ohos_ani_abc_runner.cpp" \
  -ldl \
  -o "$runner"

docker run --rm --platform linux/amd64 \
  -v "$ohos_source_root:$ohos_source_root:ro" \
  -v "$repo_root:/repo:ro" \
  -v "$work_root:/work" \
  ubuntu:22.04 \
  "$es2panda" \
  --extension=ets \
  --arktsconfig "$arktsconfig" \
  --output /work/ohos_qemu_abc_launcher.abc \
  /repo/scripts/ohos_qemu_abc_launcher.ets

if [[ -z "$hap_input" && "${OHOS_QEMU_SKIP_BUILD:-0}" != "1" ]]; then
  cargo_command=(cargo)
  rustflags="${RUSTFLAGS:-}"
  if [[ "$rust_asan" == "1" ]]; then
    if [[ "$runner_asan" != "1" ]]; then
      echo "OHOS_QEMU_RUST_ASAN requires OHOS_QEMU_RUNNER_ASAN so the executable supplies the ASAN runtime" >&2
      exit 2
    fi
    cargo_command=(cargo +nightly)
    rustflags="${rustflags:+$rustflags }-Zsanitizer=address -Cforce-frame-pointers=yes"
  fi
  env \
    ANI_TEST_MODULE_NAME=arkvm_test \
    RUSTFLAGS="$rustflags" \
    "$rust_target_env=$clang_bin/$clang_triple-clang" \
    "$cc_target_env=$clang_bin/$clang_triple-clang" \
    "$cxx_target_env=$clang_bin/$clang_triple-clang++" \
    "${cargo_command[@]}" build --workspace --target "$rust_target"
fi

run_with_timeout "$hdc_timeout" \
  "$hdc_bin" -t "$hdc_target" shell mkdir -p "$remote_root"
run_with_timeout "$hdc_timeout" \
  "$hdc_bin" -t "$hdc_target" file send \
  "$runner" "$remote_root/ani_abc_runner" >/dev/null
run_with_timeout "$hdc_timeout" "$hdc_bin" -t "$hdc_target" file send \
  "$launcher_abc" "$remote_root/ohos_qemu_abc_launcher.abc" >/dev/null
run_with_timeout "$hdc_timeout" \
  "$hdc_bin" -t "$hdc_target" shell chmod 755 "$remote_root/ani_abc_runner"

printf 'arch\tpackage\tcross_build\telf_abi\tabc_compile\tqemu_runtime\tassert_pass\tassert_fail\tstatus\n' > "$report"
printf 'arch\tpackage\titerations\tstart_pss_kb\tend_pss_kb\tgrowth_pss_kb\tlimit_pss_kb\tstatus\n' > "$memory_report"
printf 'arch\tpackage\titerations\telapsed_us\tper_iteration_us\tlimit_us\tstatus\n' > "$performance_report"

total=0
passed=0

while IFS= read -r cargo_toml; do
  package="$(sed -n 's/^name[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$cargo_toml" | head -n1)"
  [[ -z "$package" ]] && continue
  if [[ -n "$package_filter" && ! "$package" =~ $package_filter ]]; then
    continue
  fi

  ((total += 1))
  base="${package//-/_}"
  example_dir="$(dirname "$cargo_toml")"
  test_ets="$example_dir/arkvm_test.ets"
  native_lib="$repo_root/target/$rust_target/debug/lib${base}.so"
  case_dir="$work_root/cases/$base"
  abc_file="$case_dir/arkvm_test.abc"
  run_log="$case_dir/runtime.log"
  hilog_file="$case_dir/hilog.log"

  mkdir -p "$case_dir"

  if [[ -n "$hap_input" ]]; then
    native_lib="$hap_extract_root/libani_example_new_basic.so"
    cp "$hap_extract_root/arkvm_test.abc" "$abc_file"
  fi

  if [[ ! -f "$native_lib" ]]; then
    printf '%s\t%s\tFAIL\tSKIP\tSKIP\tSKIP\t0\t0\tFAIL\n' \
      "$guest_arch" "$package" >> "$report"
    echo "FAIL $package: missing $native_lib"
    continue
  fi

  elf_header="$("$clang_bin/llvm-readelf" -h "$native_lib")"
  if ! grep -Fq "Machine:                           $expected_elf_machine" <<<"$elf_header"; then
    printf '%s\t%s\tOK\tFAIL\tSKIP\tSKIP\t0\t0\tFAIL\n' \
      "$guest_arch" "$package" >> "$report"
    echo "FAIL $package: ELF architecture is not $expected_elf_machine"
    continue
  fi

  if [[ -z "$hap_input" ]]; then
    if ! docker run --rm --platform linux/amd64 \
      -v "$ohos_source_root:$ohos_source_root:ro" \
      -v "$repo_root:/repo:ro" \
      -v "$work_root:/work" \
      ubuntu:22.04 \
      "$es2panda" \
      --extension=ets \
      --arktsconfig "$arktsconfig" \
      --output "/work/cases/$base/arkvm_test.abc" \
      "/repo/$test_ets" \
      >"$case_dir/es2panda.log" 2>&1; then
      printf '%s\t%s\tOK\tOK\tFAIL\tSKIP\t0\t0\tFAIL\n' \
        "$guest_arch" "$package" >> "$report"
      echo "FAIL $package: ABC compile"
      continue
    fi
  fi

  run_with_timeout "$hdc_timeout" "$hdc_bin" -t "$hdc_target" file send \
    "$native_lib" "$remote_root/lib${base}.so" >/dev/null
  run_with_timeout "$hdc_timeout" "$hdc_bin" -t "$hdc_target" file send \
    "$abc_file" "$remote_root/arkvm_test.abc" >/dev/null
  runtime_ok=0
  assert_pass=0
  assert_fail=0
  start_pss=-1
  end_pss=-1
  growth_pss=0
  memory_status=SKIP
  elapsed_us=-1
  per_iteration_us=-1
  performance_status=FAIL
  for ((attempt = 1; attempt <= case_attempts; attempt += 1)); do
    run_with_timeout "$hdc_timeout" \
      "$hdc_bin" -t "$hdc_target" shell hilog -r >/dev/null
    case_runtime_env="$runner_runtime_env ANI_QEMU_DESTRUCTOR_LIBRARY=$remote_root/lib${base}.so"
    set +e
    run_with_timeout "$hdc_runtime_timeout" "$hdc_bin" -t "$hdc_target" shell \
      "$case_runtime_env ANI_TEST_MODULE_NAME=arkvm_test LD_LIBRARY_PATH=/system/lib64:$remote_root timeout -k 5 $case_timeout $remote_root/ani_abc_runner $remote_root/ohos_qemu_abc_launcher.abc $remote_root/arkvm_test.abc arkvm_test.ETSGLOBAL main $remote_root" \
      >"$run_log" 2>&1
    device_exit=$?
    set -e
    printf 'ANI_HDC_SHELL_EXIT=%s\n' "$device_exit" >>"$run_log"
    run_with_timeout "$hdc_timeout" "$hdc_bin" -t "$hdc_target" shell \
      "hilog -x | grep -E '\\[arkvm\\]|\\[ASSERT PASS\\]|\\[ASSERT FAIL\\]|\\[QEMU ERROR\\]'" \
      >"$hilog_file" 2>&1 || true
    cp "$run_log" "$case_dir/runtime.attempt-$attempt.log"
    cp "$hilog_file" "$case_dir/hilog.attempt-$attempt.log"

    assert_pass="$(awk '/\[ASSERT PASS\]/{count += 1} END{print count + 0}' "$hilog_file")"
    assert_fail="$(awk '/\[ASSERT FAIL\]/{count += 1} END{print count + 0}' "$hilog_file")"
    elapsed_us="$(awk -F'elapsed_us=' '/ANI_PERF_SAMPLE/{split($2, v, " "); value=v[1]} END{if (value != "") print value}' "$run_log")"
    per_iteration_us="$(awk -F'per_iteration_us=' '/ANI_PERF_SAMPLE/{split($2, v, " "); value=v[1]} END{if (value != "") print value}' "$run_log")"
    elapsed_us="${elapsed_us:--1}"
    per_iteration_us="${per_iteration_us:--1}"
    performance_status=PASS
    if ((elapsed_us < 0 || per_iteration_us < 0)); then
      performance_status=FAIL
    elif [[ -n "$max_per_iteration_us" ]] &&
      ((per_iteration_us > max_per_iteration_us)); then
      performance_status=FAIL
    fi
    if [[ "$memory_sample" != "0" ]]; then
      sample_count="$(awk '/ANI_MEMORY_SAMPLE/{count += 1} END{print count + 0}' "$run_log")"
      start_pss="$(awk -F'pss_kb=' '/ANI_MEMORY_SAMPLE/{split($2, v, " "); print v[1]; exit}' "$run_log")"
      end_pss="$(awk -F'pss_kb=' '/ANI_MEMORY_SAMPLE/{split($2, v, " "); value=v[1]} END{if (value != "") print value}' "$run_log")"
      start_pss="${start_pss:--1}"
      end_pss="${end_pss:--1}"
      if ((sample_count >= 2 && start_pss >= 0 && end_pss >= 0)); then
        growth_pss=$((end_pss - start_pss))
        memory_status=PASS
        if [[ -n "$max_pss_growth_kb" ]] && ((growth_pss > max_pss_growth_kb)); then
          memory_status=FAIL
        fi
      else
        memory_status=FAIL
      fi
    fi
    if grep -q 'ANI_ABC_RUNNER_OK' "$run_log" &&
      grep -q '\[arkvm\] smoke done:' "$hilog_file" &&
      [[ "$assert_fail" == "0" ]] &&
      { [[ "$package" != "ani-example-init-lifecycle" ]] ||
        grep -q 'ANI_FINALIZER_OK count=1' "$run_log"; } &&
      [[ "$memory_status" != "FAIL" ]] &&
      [[ "$performance_status" != "FAIL" ]]; then
      runtime_ok=1
      break
    fi
    if ((attempt < case_attempts)); then
      echo "RETRY $package: QEMU runtime attempt $attempt/$case_attempts"
      sleep 1
    fi
  done

  if [[ "$runtime_ok" == "1" ]]; then
    printf '%s\t%s\tOK\tOK\tOK\tOK\t%s\t%s\tPASS\n' \
      "$guest_arch" "$package" "$assert_pass" "$assert_fail" >> "$report"
    ((passed += 1))
    echo "PASS $package ($assert_pass assertions)"
  else
    printf '%s\t%s\tOK\tOK\tOK\tFAIL\t%s\t%s\tFAIL\n' \
      "$guest_arch" "$package" "$assert_pass" "$assert_fail" >> "$report"
    echo "FAIL $package: QEMU runtime ($assert_pass pass, $assert_fail fail)"
  fi
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
    "$guest_arch" "$package" "$runner_iterations" "$start_pss" "$end_pss" \
    "$growth_pss" "${max_pss_growth_kb:-none}" "$memory_status" >> "$memory_report"
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
    "$guest_arch" "$package" "$runner_iterations" "$elapsed_us" \
    "$per_iteration_us" "${max_per_iteration_us:-none}" "$performance_status" \
    >> "$performance_report"
done < <(find examples -maxdepth 2 -name Cargo.toml | sort)

echo "QEMU_RESULT: $passed/$total"
echo "REPORT: $report"
echo "MEMORY_REPORT: $memory_report"
echo "PERFORMANCE_REPORT: $performance_report"

[[ "$passed" == "$total" ]]
