#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

baseline_rev="${ANI_SEMVER_BASELINE_REV:-origin/master}"

# cargo-semver-checks analyzes Rust library metadata. Proc-macro-only and
# binary-only crates have no supported rustdoc API surface, so ani-derive and
# ani-cli remain covered by the compile, package, and generated-ETS gates.
for package_manifest in crates/sys/Cargo.toml crates/ani/Cargo.toml; do
  package="$(sed -n 's/^name[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$package_manifest" | head -n1)"
  current_version="$(sed -n 's/^version[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$package_manifest" | head -n1)"
  baseline_version="$(git show "$baseline_rev:$package_manifest" | sed -n 's/^version[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' | head -n1)"
  release_args=()
  if [[ -n "${ANI_SEMVER_RELEASE_TYPE:-}" ]]; then
    release_args=(--release-type "$ANI_SEMVER_RELEASE_TYPE")
  elif [[ "$current_version" == "$baseline_version" ]]; then
    # cargo-semver-checks otherwise treats an unchanged prerelease version as
    # a major release, which would silently skip the compatibility gate.
    release_args=(--release-type patch)
  fi
  if ((${#release_args[@]} > 0)); then
    cargo semver-checks \
      --package "$package" \
      --baseline-rev "$baseline_rev" \
      "${release_args[@]}"
  else
    cargo semver-checks \
      --package "$package" \
      --baseline-rev "$baseline_rev"
  fi
done
