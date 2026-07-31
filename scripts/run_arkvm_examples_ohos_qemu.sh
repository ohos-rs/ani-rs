#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

hdc_target="${HDC_TARGET:-127.0.0.1:5557}"
guest_arch="${OHOS_QEMU_GUEST_ARCH:-arm64}"
ohos_source_root="${OHOS_SOURCE_ROOT:-/tmp/openharmony}"
deveco_sdk_root="${DEVECO_SDK_ROOT:-/Applications/DevEco-Studio.app/Contents/sdk/default/openharmony}"
work_root="${OHOS_QEMU_WORK_ROOT:-$repo_root/target/ohos-qemu}"
remote_root="${OHOS_QEMU_REMOTE_ROOT:-/data/local/tmp/ani-rs-qemu}"
case_timeout="${OHOS_QEMU_CASE_TIMEOUT:-45}"
case_attempts="${OHOS_QEMU_CASE_ATTEMPTS:-3}"
package_filter="${OHOS_QEMU_PACKAGE_FILTER:-}"
runner_asan="${OHOS_QEMU_RUNNER_ASAN:-0}"

case "$guest_arch" in
  arm64)
    rust_target="aarch64-unknown-linux-ohos"
    clang_triple="aarch64-unknown-linux-ohos"
    expected_uname='aarch64|arm64'
    expected_elf_machine='AArch64'
    ;;
  x86_64)
    rust_target="x86_64-unknown-linux-ohos"
    clang_triple="x86_64-unknown-linux-ohos"
    expected_uname='x86_64'
    expected_elf_machine='Advanced Micro Devices X86-64'
    ;;
  armv7a)
    rust_target="armv7-unknown-linux-ohos"
    clang_triple="armv7-unknown-linux-ohos"
    expected_uname='armv7l|armv7|arm'
    expected_elf_machine='ARM'
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
es2panda="$es2panda_dir/es2panda"
arktsconfig="$es2panda_dir/arktsconfig.json"
runner="$work_root/ani_abc_runner"
launcher_abc="$work_root/ohos_qemu_abc_launcher.abc"
report="$work_root/report.tsv"

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

if ! "$hdc_bin" -t "$hdc_target" shell true >/dev/null 2>&1; then
  echo "HDC target is not connected: $hdc_target" >&2
  exit 1
fi

device_arch="$("$hdc_bin" -t "$hdc_target" shell uname -m | tr -d '\r[:space:]')"
if [[ ! "$device_arch" =~ ^($expected_uname)$ ]]; then
  echo "HDC target architecture mismatch: requested $guest_arch, device reports $device_arch" >&2
  exit 1
fi

mkdir -p "$work_root"

runner_cxxflags=(-O2)
runner_runtime_env=""
if [[ "$runner_asan" == "1" ]]; then
  runner_cxxflags=(-O1 -g -fno-omit-frame-pointer -fsanitize=address)
  runner_runtime_env="ASAN_OPTIONS=detect_leaks=0:halt_on_error=1"
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

if [[ "${OHOS_QEMU_SKIP_BUILD:-0}" != "1" ]]; then
  env \
    ANI_TEST_MODULE_NAME=arkvm_test \
    "$rust_target_env=$clang_bin/$clang_triple-clang" \
    "$cc_target_env=$clang_bin/$clang_triple-clang" \
    "$cxx_target_env=$clang_bin/$clang_triple-clang++" \
    cargo build --workspace --target "$rust_target"
fi

"$hdc_bin" -t "$hdc_target" shell mkdir -p "$remote_root"
"$hdc_bin" -t "$hdc_target" file send "$runner" "$remote_root/ani_abc_runner" >/dev/null
"$hdc_bin" -t "$hdc_target" file send \
  "$launcher_abc" "$remote_root/ohos_qemu_abc_launcher.abc" >/dev/null
"$hdc_bin" -t "$hdc_target" shell chmod 755 "$remote_root/ani_abc_runner"

printf 'arch\tpackage\tcross_build\telf_abi\tabc_compile\tqemu_runtime\tassert_pass\tassert_fail\tstatus\n' > "$report"

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

  "$hdc_bin" -t "$hdc_target" file send "$native_lib" "$remote_root/lib${base}.so" >/dev/null
  "$hdc_bin" -t "$hdc_target" file send "$abc_file" "$remote_root/arkvm_test.abc" >/dev/null
  runtime_ok=0
  assert_pass=0
  assert_fail=0
  for ((attempt = 1; attempt <= case_attempts; attempt += 1)); do
    "$hdc_bin" -t "$hdc_target" shell hilog -r >/dev/null
    "$hdc_bin" -t "$hdc_target" shell \
      "$runner_runtime_env ANI_TEST_MODULE_NAME=arkvm_test LD_LIBRARY_PATH=/system/lib64:$remote_root timeout -k 5 $case_timeout $remote_root/ani_abc_runner $remote_root/ohos_qemu_abc_launcher.abc $remote_root/arkvm_test.abc arkvm_test.ETSGLOBAL main $remote_root" \
      >"$run_log" 2>&1 || true
    "$hdc_bin" -t "$hdc_target" shell \
      "hilog -x | grep -E '\\[arkvm\\]|\\[ASSERT PASS\\]|\\[ASSERT FAIL\\]|\\[QEMU ERROR\\]'" \
      >"$hilog_file" 2>&1 || true

    assert_pass="$(awk '/\[ASSERT PASS\]/{count += 1} END{print count + 0}' "$hilog_file")"
    assert_fail="$(awk '/\[ASSERT FAIL\]/{count += 1} END{print count + 0}' "$hilog_file")"
    if grep -q 'ANI_ABC_RUNNER_OK' "$run_log" &&
      grep -q '\[arkvm\] smoke done:' "$hilog_file" &&
      [[ "$assert_fail" == "0" ]]; then
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
done < <(find examples -maxdepth 2 -name Cargo.toml | sort)

echo "QEMU_RESULT: $passed/$total"
echo "REPORT: $report"

[[ "$passed" == "$total" ]]
