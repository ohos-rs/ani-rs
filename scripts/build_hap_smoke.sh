#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
arch="${1:-arm64}"
sdk_root="${DEVECO_SDK_ROOT:-/Applications/DevEco-Studio.app/Contents/sdk/default/openharmony}"
deveco_root="${DEVECO_STUDIO_ROOT:-/Applications/DevEco-Studio.app/Contents}"
work_root="${ANI_HAP_WORK_ROOT:-$repo_root/target/hap-smoke-$arch}"
ohos_source_root="${OHOS_SOURCE_ROOT:-/Volumes/PSSD/qemu/openharmony}"
if [[ "$work_root" != /* ]]; then
  work_root="$repo_root/$work_root"
fi
project_root="$work_root/project"
native_target="$work_root/native-target"
sdk_base="$work_root/sdk-base"
node_path="$work_root/node-path"

case "$arch" in
  arm64)
    rust_target="aarch64-unknown-linux-ohos"
    clang_triple="aarch64-unknown-linux-ohos"
    abi="arm64-v8a"
    ;;
  x86_64)
    rust_target="x86_64-unknown-linux-ohos"
    clang_triple="x86_64-unknown-linux-ohos"
    abi="x86_64"
    ;;
  armv7a)
    rust_target="armv7-unknown-linux-ohos"
    clang_triple="armv7-unknown-linux-ohos"
    abi="armeabi-v7a"
    ;;
  *)
    echo "unsupported architecture: $arch" >&2
    exit 2
    ;;
esac

node="$deveco_root/tools/node/bin/node"
hvigor="$deveco_root/tools/hvigor/hvigor/bin/hvigor.js"
clang="$sdk_root/native/llvm/bin/$clang_triple-clang"
clangxx="$sdk_root/native/llvm/bin/$clang_triple-clang++"
es2panda="$ohos_source_root/out/arm64_virt/clang_x64/arkcompiler/ets_frontend/es2panda"
arktsconfig="$ohos_source_root/out/arm64_virt/clang_x64/arkcompiler/ets_frontend/arktsconfig.json"
for required in "$node" "$hvigor" "$clang" "$clangxx" "$es2panda"; do
  if [[ ! -x "$required" ]]; then
    echo "missing executable: $required" >&2
    exit 1
  fi
done

mkdir -p "$work_root"
if [[ -d "$project_root" ]]; then
  rm -rf -- "$project_root"
fi
mkdir -p "$project_root" "$native_target" "$sdk_base"
cp -R "$repo_root/tests/hap-smoke/." "$project_root/"
if [[ -L "$sdk_base/24" ]]; then
  rm -- "$sdk_base/24"
fi
mkdir -p "$sdk_base/24"
for component in native previewer js ets toolchains; do
  ln -sfn "$sdk_root/$component" "$sdk_base/24/$component"
done
cp "$repo_root/tests/hap-smoke-sdk-pkg.json" \
  "$sdk_base/24/sdk-pkg.json"
mkdir -p "$project_root/node_modules/@ohos"
ln -sfn "$deveco_root/tools/hvigor/hvigor-ohos-plugin" \
  "$project_root/node_modules/@ohos/hvigor-ohos-plugin"
mkdir -p "$node_path/@ohos"
ln -sfn "$deveco_root/tools/hvigor/hvigor" "$node_path/@ohos/hvigor"
ln -sfn "$deveco_root/tools/hvigor/hvigor-ohos-plugin" \
  "$node_path/@ohos/hvigor-ohos-plugin"

ets_output="$native_target/ani-ets/ani_example_new_basic.ets"
target_env="$(printf '%s' "$rust_target" | tr '[:lower:]-' '[:upper:]_')"
cc_env="$(printf '%s' "$rust_target" | tr '-' '_')"
mkdir -p "$(dirname "$ets_output")"

# Proc-macro output paths are compile-time inputs that Cargo cannot track.
# Recompile the leaf cdylib so a deleted project copy can never leave stale or
# missing ETS declarations behind.
cargo clean --manifest-path "$repo_root/Cargo.toml" \
  --target-dir "$native_target" --target "$rust_target" \
  --package ani-example-new-basic

env \
  ANI_MODULE_DESCRIPTOR=arkvm_test \
  ANI_ETS_OUTPUT="$ets_output" \
  ANI_ETS_LIBRARY=ani_example_new_basic \
  CARGO_TARGET_DIR="$native_target" \
  "CARGO_TARGET_${target_env}_LINKER=$clang" \
  "CC_${cc_env}=$clang" \
  "CXX_${cc_env}=$clangxx" \
  cargo build --manifest-path "$repo_root/Cargo.toml" \
    --package ani-example-new-basic --target "$rust_target"

mkdir -p "$project_root/entry/libs/$abi"
cp "$native_target/$rust_target/debug/libani_example_new_basic.so" \
  "$project_root/entry/libs/$abi/libani_example_new_basic.so"

mkdir -p "$project_root/entry/src/main/resources/rawfile"
docker run --rm --platform linux/amd64 \
  -v "$ohos_source_root:$ohos_source_root:ro" \
  -v "$repo_root:/repo:ro" \
  -v "$work_root:/work" \
  ubuntu:22.04 \
  "$es2panda" \
  --extension=ets \
  --arktsconfig "$arktsconfig" \
  --output /work/project/entry/src/main/resources/rawfile/ani_rs_smoke.abc \
  /repo/examples/new_basic/arkvm_test.ets

(
  cd "$project_root"
  NODE_PATH="$node_path" OHOS_BASE_SDK_HOME="$sdk_base" "$node" "$hvigor" \
    --mode module \
    -p product=default \
    -p module=entry@default \
    -p buildMode=debug \
    --no-daemon --no-incremental --no-parallel \
    assembleHap
)

hap="$project_root/entry/build/default/outputs/default/entry-default-unsigned.hap"
"$repo_root/scripts/verify_hap.sh" "$hap" "$arch"
echo "HAP_OUTPUT: $hap"
