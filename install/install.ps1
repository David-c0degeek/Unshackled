# Build and install the Unshackled CLI from source on Windows.
#
# Usage: ./install/install.ps1
#requires -Version 5
$ErrorActionPreference = 'Stop'

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "cargo (the Rust toolchain) is required. Install it from https://rustup.rs and re-run."
}

$root = Split-Path -Parent $PSScriptRoot
Write-Host "building and installing the unshackled CLI from $root ..."
cargo install --path (Join-Path $root 'crates/unshackled-cli') --locked

Write-Host ""
Write-Host "installed 'unshackled'. verify with:"
Write-Host "    unshackled doctor"
