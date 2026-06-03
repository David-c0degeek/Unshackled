#!/bin/sh
# Build and install the Unshackled CLI from source on Linux or macOS.
#
# Usage:
#   ./install/install.sh                       # full build (tui + learning)
#   UNSHACKLED_FEATURES=learning ./install/install.sh   # no interactive TUI
set -eu

if ! command -v cargo >/dev/null 2>&1; then
    echo "error: cargo (the Rust toolchain) is required." >&2
    echo "       install it from https://rustup.rs and re-run this script." >&2
    exit 1
fi

features="${UNSHACKLED_FEATURES:-tui,learning}"
root="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"

# The LocalMind learning engine is a git submodule; the `learning` feature needs
# it checked out.
if [ -f "$root/.gitmodules" ] && command -v git >/dev/null 2>&1; then
    echo "updating submodules ..."
    git -C "$root" submodule update --init --recursive
fi

echo "building and installing the unshackled CLI (features: $features) ..."
cargo install --path "$root/crates/unshackled-cli" --features "$features" --locked

echo
echo "installed 'unshackled'. verify with:"
echo "    unshackled doctor"
