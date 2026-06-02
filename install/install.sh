#!/bin/sh
# Build and install the Unshackled CLI from source on Linux or macOS.
#
# Usage: ./install/install.sh
set -eu

if ! command -v cargo >/dev/null 2>&1; then
    echo "error: cargo (the Rust toolchain) is required." >&2
    echo "       install it from https://rustup.rs and re-run this script." >&2
    exit 1
fi

root="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
echo "building and installing the unshackled CLI from $root ..."
cargo install --path "$root/crates/unshackled-cli" --locked

echo
echo "installed 'unshackled'. verify with:"
echo "    unshackled doctor"
