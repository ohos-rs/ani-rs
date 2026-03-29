#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

image="${ARKVM_DOCKER_IMAGE:-docker.m.daocloud.io/library/ubuntu:latest}"
arkvm_dir_default="$repo_root/.cache/arkvm/arkvm_static_linux_x64/x64_linux_static"
arkvm_dir="${ARKVM_DIR:-$arkvm_dir_default}"
arkvm_tarball="${ARKVM_TARBALL:-}"
ark_src_root="${ARK_SRC_ROOT:-/tmp/arkcompiler_runtime_core}"
arkvm_test_module_name="${ARKVM_TEST_MODULE_NAME:-arkvm_test}"
report_file="$repo_root/examples/arkvm_report.txt"
result_tsv="$repo_root/examples/arkvm_report.tsv"

cleanup_arkvm_dir=""

if [[ -n "$arkvm_tarball" ]]; then
  cleanup_arkvm_dir="$(mktemp -d /tmp/ani-rs-arkvm.XXXXXX)"
  trap '[[ -n "$cleanup_arkvm_dir" ]] && rm -rf "$cleanup_arkvm_dir"' EXIT
  tar -xzf "$arkvm_tarball" -C "$cleanup_arkvm_dir"

  if [[ -d "$cleanup_arkvm_dir/x64_linux_static" ]]; then
    arkvm_dir="$cleanup_arkvm_dir/x64_linux_static"
  else
    first_subdir="$(find "$cleanup_arkvm_dir" -mindepth 1 -maxdepth 1 -type d | head -n 1)"
    if [[ -n "$first_subdir" ]]; then
      arkvm_dir="$first_subdir"
    fi
  fi
fi

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

check_nonempty_tree() {
  local path="$1"
  if [[ -z "$(find "$path" -type f -print -quit 2>/dev/null)" ]]; then
    echo "ARK_SRC_EMPTY: $path"
    exit 1
  fi
}

check_nonempty_tree "$ark_src_root/static_core/plugins/ets/stdlib/std"
check_nonempty_tree "$ark_src_root/static_core/plugins/ets/stdlib/escompat"
check_nonempty_tree "$ark_src_root/static_core/plugins/ets/sdk/api"
check_nonempty_tree "$ark_src_root/static_core/plugins/ets/sdk/arkts"

host_rustup_cache="${ARKVM_RUSTUP_CACHE:-/tmp/ani-rs-arkvm-rustup}"
host_cargo_home_cache="${ARKVM_CARGO_HOME_CACHE:-/tmp/ani-rs-arkvm-cargo-home}"
mkdir -p "$host_rustup_cache" "$host_cargo_home_cache"

echo "Running in docker image: $image"
echo "Using ARKVM_DIR: $arkvm_dir"
if [[ -n "$arkvm_tarball" ]]; then
  echo "Using ARKVM_TARBALL: $arkvm_tarball"
fi
echo "Using ARK_SRC_ROOT: $ark_src_root"
echo "Using ARKVM_RUSTUP_CACHE: $host_rustup_cache"
echo "Using ARKVM_CARGO_HOME_CACHE: $host_cargo_home_cache"

docker run --rm --platform linux/amd64 \
  -v "$repo_root":/work \
  -v "$arkvm_dir":/arkvm:ro \
  -v "$ark_src_root":/arkcompiler_runtime_core:ro \
  -v "$host_rustup_cache":/root/.rustup \
  -v "$host_cargo_home_cache":/root/.cargo \
  -e ANI_TEST_MODULE_NAME="$arkvm_test_module_name" \
  -e ANI_DEBUG_REGISTER="${ANI_DEBUG_REGISTER:-0}" \
  -e ARKVM_RUSTUP_DIST_SERVER="${ARKVM_RUSTUP_DIST_SERVER:-}" \
  -e ARKVM_RUSTUP_UPDATE_ROOT="${ARKVM_RUSTUP_UPDATE_ROOT:-}" \
  -e ARKVM_DIR_IN_CONTAINER=/arkvm \
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

use_rustup_server() {
  local dist_server="$1"
  local update_root="$2"
  if [[ -n "$dist_server" ]]; then
    export RUSTUP_DIST_SERVER="$dist_server"
    export RUSTUP_UPDATE_ROOT="$update_root"
    echo "rustup source: $dist_server"
  else
    unset RUSTUP_DIST_SERVER
    unset RUSTUP_UPDATE_ROOT
    echo "rustup source: default"
  fi
}

install_stable_toolchain() {
  local mirrors=()
  if [[ -n "${ARKVM_RUSTUP_DIST_SERVER:-}" ]]; then
    mirrors+=("${ARKVM_RUSTUP_DIST_SERVER}|${ARKVM_RUSTUP_UPDATE_ROOT:-${ARKVM_RUSTUP_DIST_SERVER%/}/rustup}")
  fi
  mirrors+=("|")
  mirrors+=("https://static.rust-lang.org|https://static.rust-lang.org/rustup")
  mirrors+=("https://rsproxy.cn|https://rsproxy.cn/rustup")

  local mirror
  for mirror in "${mirrors[@]}"; do
    local dist_server="${mirror%%|*}"
    local update_root="${mirror#*|}"
    use_rustup_server "$dist_server" "$update_root"
    if rustup toolchain install stable --profile minimal --no-self-update; then
      rustup default stable
      return 0
    fi
  done

  echo "failed to install rust stable from all configured sources" >&2
  return 1
}

if [[ ! -x /root/.cargo/bin/rustup ]]; then
  unset RUSTUP_DIST_SERVER
  unset RUSTUP_UPDATE_ROOT
  curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain none
fi

source /root/.cargo/env
rustup set auto-self-update disable >/dev/null 2>&1 || true
set -- $(rustc --version 2>/dev/null || true)
installed_rustc_version="${2:-}"
if [[ -z "$installed_rustc_version" ]]; then
  set -- $(rustup run stable rustc --version 2>/dev/null || true)
  installed_rustc_version="${2:-}"
  if [[ -n "$installed_rustc_version" ]]; then
    rustup default stable
    source /root/.cargo/env
  fi
fi
if [[ -z "$installed_rustc_version" ]] || [[ "$(printf '%s\n' "1.85.0" "$installed_rustc_version" | sort -V | head -n1)" != "1.85.0" ]]; then
  if rustup run stable rustc --version >/dev/null 2>&1; then
    rustup toolchain uninstall stable-x86_64-unknown-linux-gnu >/dev/null 2>&1 || true
  fi
  install_stable_toolchain
  source /root/.cargo/env
fi

rustc --version
cargo --version

report_file=/work/examples/arkvm_report.txt
result_tsv=/work/examples/arkvm_report.tsv
arkvm_dir="${ARKVM_DIR_IN_CONTAINER:-/work/.cache/arkvm/arkvm_static_linux_x64/x64_linux_static}"
arkts_cfg=/tmp/arktsconfig.ani-rs.json
mkdir -p /work/examples

cat > "$arkts_cfg" <<EOF
{
  "compilerOptions": {
    "baseUrl": "/arkcompiler_runtime_core/static_core",
    "paths": {
      "std": ["/arkcompiler_runtime_core/static_core/plugins/ets/stdlib/std"],
      "escompat": ["/arkcompiler_runtime_core/static_core/plugins/ets/stdlib/escompat"],
      "arkruntime": ["/arkcompiler_runtime_core/static_core/plugins/ets/stdlib/arkruntime"],
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
  cargo clean -p "$pkg" >/tmp/"${pkg}".clean.log 2>&1 || true
  if cargo build -p "$pkg" >/tmp/"${pkg}".build.log 2>&1; then
    echo "BUILD_OK: $pkg" >> "$report_file"
  else
    echo "BUILD_FAIL: $pkg" >> "$report_file"
    tail -n 120 /tmp/"${pkg}".build.log >> "$report_file"
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
