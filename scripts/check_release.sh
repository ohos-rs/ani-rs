#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
scripts/header.sh --check
scripts/check_example_ets.sh

RUSTDOCFLAGS="-D warnings" cargo doc \
  -p ani -p ani-derive -p ani-sys -p ani-cli \
  --all-features --no-deps

for package in ani-sys ani-derive ani ani-cli; do
  cargo package -p "$package" --no-verify
done

pnpm --dir website check
pnpm --dir website build

echo "OK: release checks and packages completed."
