#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

image="${ARKVM_DOCKER_IMAGE:-docker.m.daocloud.io/library/ubuntu:latest}"
arkvm_dir_default="$repo_root/.cache/arkvm/arkvm_static_linux_x64/x64_linux_static"
arkvm_dir="${ARKVM_DIR:-$arkvm_dir_default}"
ark_src_root="${ARK_SRC_ROOT:-/tmp/arkcompiler_runtime_core}"
arkvm_test_module_name="${ARKVM_TEST_MODULE_NAME:-arkvm_test}"
report_file="$repo_root/examples/arkvm_report.txt"
result_tsv="$repo_root/examples/arkvm_report.tsv"

if [[ ! -x "$arkvm_dir/es2panda" ]]; then
  echo "ARKVM_NOT_FOUND: $arkvm_dir/es2panda"
  exit 1
fi
if [[ ! -x "$arkvm_dir/ark" ]]; then
  echo "ARKVM_NOT_FOUND: $arkvm_dir/ark"
  exit 1
fi
if [[ ! -f "$arkvm_dir/etsstdlib.abc" ]]; then
  echo "ARKVM_NOT_FOUND: $arkvm_dir/etsstdlib.abc"
  exit 1
fi
if [[ ! -d "$ark_src_root/static_core/plugins/ets/stdlib/std" ]]; then
  echo "ARK_SRC_NOT_FOUND: $ark_src_root/static_core/plugins/ets/stdlib/std"
  exit 1
fi
if [[ ! -d "$ark_src_root/static_core/plugins/ets/stdlib/escompat" ]]; then
  echo "ARK_SRC_NOT_FOUND: $ark_src_root/static_core/plugins/ets/stdlib/escompat"
  exit 1
fi
if [[ ! -d "$ark_src_root/static_core/plugins/ets/sdk/api" ]]; then
  echo "ARK_SRC_NOT_FOUND: $ark_src_root/static_core/plugins/ets/sdk/api"
  exit 1
fi
if [[ ! -d "$ark_src_root/static_core/plugins/ets/sdk/arkts" ]]; then
  echo "ARK_SRC_NOT_FOUND: $ark_src_root/static_core/plugins/ets/sdk/arkts"
  exit 1
fi

host_rustup_cache="${ARKVM_RUSTUP_CACHE:-/tmp/ani-rs-arkvm-rustup}"
host_cargo_home_cache="${ARKVM_CARGO_HOME_CACHE:-/tmp/ani-rs-arkvm-cargo-home}"
mkdir -p "$host_rustup_cache" "$host_cargo_home_cache"

echo "Running in docker image: $image"
echo "Using ARKVM_DIR: $arkvm_dir"
echo "Using ARK_SRC_ROOT: $ark_src_root"
echo "Using ARKVM_RUSTUP_CACHE: $host_rustup_cache"
echo "Using ARKVM_CARGO_HOME_CACHE: $host_cargo_home_cache"

docker run --rm --platform linux/amd64 \
  -v "$repo_root":/work \
  -v "$ark_src_root":/arkcompiler_runtime_core:ro \
  -v "$host_rustup_cache":/root/.rustup \
  -v "$host_cargo_home_cache":/root/.cargo \
  -e ANI_TEST_MODULE_NAME="$arkvm_test_module_name" \
  -e ANI_DEBUG_REGISTER="${ANI_DEBUG_REGISTER:-0}" \
  -w /work \
  "$image" \
  /bin/bash -lc '
set -euo pipefail

export DEBIAN_FRONTEND=noninteractive

apt-get update
apt-get install -y --no-install-recommends \
  ca-certificates \
  curl \
  build-essential \
  libatomic1 \
  pkg-config \
  git \
  ripgrep
update-ca-certificates || true

if [[ ! -x /root/.cargo/bin/rustc ]]; then
  curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain stable
fi

source /root/.cargo/env
rustc --version
cargo --version

report_file=/work/examples/arkvm_report.txt
result_tsv=/work/examples/arkvm_report.tsv
arkvm_dir=/work/.cache/arkvm/arkvm_static_linux_x64/x64_linux_static
arkts_cfg=/tmp/arktsconfig.ani-rs.json
mkdir -p /work/examples

cat > "$arkts_cfg" <<EOF
{
  "compilerOptions": {
    "baseUrl": "/arkcompiler_runtime_core/static_core",
    "paths": {
      "std": ["/arkcompiler_runtime_core/static_core/plugins/ets/stdlib/std"],
      "escompat": ["/arkcompiler_runtime_core/static_core/plugins/ets/stdlib/escompat"],
      "api": ["/arkcompiler_runtime_core/static_core/plugins/ets/sdk/api"],
      "arkts": ["/arkcompiler_runtime_core/static_core/plugins/ets/sdk/arkts"]
    }
  }
}
EOF

echo "ANI examples arkvm report" > "$report_file"
echo "image: '"$image"'" >> "$report_file"
echo "date: $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$report_file"
echo >> "$report_file"
echo -e "example\tbuild\tabc_compile\truntime" > "$result_tsv"

pkgs="$(rg -n "^name[[:space:]]*=" examples/*/Cargo.toml | sed -E "s/.*\"([^\"]+)\".*/\1/")"

for pkg in $pkgs; do
  echo "[build] $pkg"
  if cargo build -p "$pkg" >/tmp/"${pkg}".build.log 2>&1; then
    echo "BUILD_OK: $pkg" >> "$report_file"
  else
    echo "BUILD_FAIL: $pkg" >> "$report_file"
    tail -n 120 /tmp/"${pkg}".build.log >> "$report_file"
    echo -e "${pkg}\tFAIL\tSKIP\tSKIP" >> "$result_tsv"
  fi
done

find /work/examples -path "*/target/ani-ets/*.d.ets" -delete || true
bash ./scripts/generate_arkvm_smoke_ets.sh

while IFS= read -r cargo_file; do
  [[ -z "$cargo_file" ]] && continue
  example_dir="$(dirname "$cargo_file")"
  pkg="$(sed -n "s/^name[[:space:]]*=[[:space:]]*\"\\([^\"]*\\)\".*/\\1/p" "$cargo_file" | head -n1)"
  [[ -z "$pkg" ]] && continue

  abc_file="$example_dir/arkvm_test.abc"
  ets_file="$example_dir/arkvm_test.ets"
  module_name="$(basename "${ets_file%.ets}")"
  entrypoint="${module_name}/ETSGLOBAL::main"
  log_file="$example_dir/arkvm_test.log"
  build_log="/tmp/${pkg}.build.log"
  run_status="OK"
  abc_status="OK"
  build_status="OK"

  if grep -q "^BUILD_FAIL: ${pkg}$" "$report_file"; then
    build_status="FAIL"
    abc_status="SKIP"
    run_status="SKIP"
    echo -e "${pkg}\t${build_status}\t${abc_status}\t${run_status}" >> "$result_tsv"
    continue
  fi

  echo "[abc] $pkg"
  if ! "$arkvm_dir/es2panda" \
      --extension ets \
      --arktsconfig "$arkts_cfg" \
      --output "$abc_file" \
      "$ets_file" >"$log_file" 2>&1; then
    abc_status="FAIL"
    run_status="SKIP"
    {
      echo "ABC_FAIL: $pkg"
      tail -n 120 "$log_file"
    } >> "$report_file"
    echo -e "${pkg}\t${build_status}\t${abc_status}\t${run_status}" >> "$result_tsv"
    continue
  fi

  echo "[run] $pkg"
  if ! LD_LIBRARY_PATH="/work/target/debug:$arkvm_dir" \
      "$arkvm_dir/ark" \
      --boot-panda-files="$arkvm_dir/etsstdlib.abc" \
      --load-runtimes=ets \
      --native-library-path=/work/target/debug \
      --ets.native-library-path=/work/target/debug \
      "$abc_file" \
      "$entrypoint" >>"$log_file" 2>&1; then
    run_status="FAIL"
    {
      echo "RUN_FAIL: $pkg"
      tail -n 120 "$log_file"
    } >> "$report_file"
    echo -e "${pkg}\t${build_status}\t${abc_status}\t${run_status}" >> "$result_tsv"
    continue
  fi

  echo "RUN_OK: $pkg" >> "$report_file"
  echo -e "${pkg}\t${build_status}\t${abc_status}\t${run_status}" >> "$result_tsv"
done < <(find examples -maxdepth 2 -name Cargo.toml | sort)

echo >> "$report_file"
echo "TSV summary: $result_tsv" >> "$report_file"
'

echo
echo "Done. Summary files:"
echo "  - $report_file"
echo "  - $result_tsv"
