#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

hdc_target="${HDC_TARGET:-127.0.0.1:5557}"
ohos_source_root="${OHOS_SOURCE_ROOT:-/tmp/openharmony}"
deveco_sdk_root="${DEVECO_SDK_ROOT:-/Applications/DevEco-Studio.app/Contents/sdk/default/openharmony}"
work_root="${OHOS_QEMU_WORK_ROOT:-$repo_root/target/ohos-qemu}"
remote_root="${OHOS_QEMU_REMOTE_ROOT:-/data/local/tmp/ani-rs-qemu}"
case_timeout="${OHOS_QEMU_CASE_TIMEOUT:-45}"
package_filter="${OHOS_QEMU_PACKAGE_FILTER:-}"

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
  "$clang_bin/aarch64-unknown-linux-ohos-clang" \
  "$clang_bin/aarch64-unknown-linux-ohos-clang++" \
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

mkdir -p "$work_root"

"$clang_bin/aarch64-unknown-linux-ohos-clang++" \
  --sysroot="$sysroot" \
  -std=c++17 \
  -O2 \
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
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_OHOS_LINKER="$clang_bin/aarch64-unknown-linux-ohos-clang" \
    CC_aarch64_unknown_linux_ohos="$clang_bin/aarch64-unknown-linux-ohos-clang" \
    CXX_aarch64_unknown_linux_ohos="$clang_bin/aarch64-unknown-linux-ohos-clang++" \
    cargo build --workspace --target aarch64-unknown-linux-ohos
fi

"$hdc_bin" -t "$hdc_target" shell mkdir -p "$remote_root"
"$hdc_bin" -t "$hdc_target" file send "$runner" "$remote_root/ani_abc_runner" >/dev/null
"$hdc_bin" -t "$hdc_target" file send \
  "$launcher_abc" "$remote_root/ohos_qemu_abc_launcher.abc" >/dev/null
"$hdc_bin" -t "$hdc_target" shell chmod 755 "$remote_root/ani_abc_runner"

printf 'package\tcross_build\tabc_compile\tqemu_runtime\tassert_pass\tassert_fail\tstatus\n' > "$report"

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
  native_lib="$repo_root/target/aarch64-unknown-linux-ohos/debug/lib${base}.so"
  case_dir="$work_root/cases/$base"
  abc_file="$case_dir/arkvm_test.abc"
  run_log="$case_dir/runtime.log"
  hilog_file="$case_dir/hilog.log"

  mkdir -p "$case_dir"

  if [[ ! -f "$native_lib" ]]; then
    printf '%s\tFAIL\tSKIP\tSKIP\t0\t0\tFAIL\n' "$package" >> "$report"
    echo "FAIL $package: missing $native_lib"
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
    printf '%s\tOK\tFAIL\tSKIP\t0\t0\tFAIL\n' "$package" >> "$report"
    echo "FAIL $package: ABC compile"
    continue
  fi

  "$hdc_bin" -t "$hdc_target" file send "$native_lib" "$remote_root/lib${base}.so" >/dev/null
  "$hdc_bin" -t "$hdc_target" file send "$abc_file" "$remote_root/arkvm_test.abc" >/dev/null
  "$hdc_bin" -t "$hdc_target" shell hilog -r >/dev/null

  "$hdc_bin" -t "$hdc_target" shell \
    "ANI_TEST_MODULE_NAME=arkvm_test LD_LIBRARY_PATH=/system/lib64:$remote_root timeout -k 5 $case_timeout $remote_root/ani_abc_runner $remote_root/ohos_qemu_abc_launcher.abc $remote_root/arkvm_test.abc arkvm_test.ETSGLOBAL main $remote_root" \
    >"$run_log" 2>&1 || true
  "$hdc_bin" -t "$hdc_target" shell \
    "hilog -x | grep -E '\\[arkvm\\]|\\[ASSERT PASS\\]|\\[ASSERT FAIL\\]|\\[QEMU ERROR\\]'" \
    >"$hilog_file" 2>&1 || true

  assert_pass="$(awk '/\[ASSERT PASS\]/{count += 1} END{print count + 0}' "$hilog_file")"
  assert_fail="$(awk '/\[ASSERT FAIL\]/{count += 1} END{print count + 0}' "$hilog_file")"

  if grep -q 'ANI_ABC_RUNNER_OK' "$run_log" &&
    grep -q '\[arkvm\] smoke done:' "$hilog_file" &&
    [[ "$assert_fail" == "0" ]]; then
    printf '%s\tOK\tOK\tOK\t%s\t%s\tPASS\n' \
      "$package" "$assert_pass" "$assert_fail" >> "$report"
    ((passed += 1))
    echo "PASS $package ($assert_pass assertions)"
  else
    printf '%s\tOK\tOK\tFAIL\t%s\t%s\tFAIL\n' \
      "$package" "$assert_pass" "$assert_fail" >> "$report"
    echo "FAIL $package: QEMU runtime ($assert_pass pass, $assert_fail fail)"
  fi
done < <(find examples -maxdepth 2 -name Cargo.toml | sort)

echo "QEMU_RESULT: $passed/$total"
echo "REPORT: $report"

[[ "$passed" == "$total" ]]
