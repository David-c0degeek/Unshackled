# Build and install the Unshackled CLI from source on Windows.
#
# Usage:
#   ./install/install.ps1                         # full build (tui + learning)
#   ./install/install.ps1 -Features learning      # no interactive TUI
#   ./install/install.ps1 -Toolchain stable       # force a toolchain
#requires -Version 5
param(
    [string]$Features = 'tui,learning',
    [string]$Toolchain = ''
)
$ErrorActionPreference = 'Stop'

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "cargo (the Rust toolchain) is required. Install it from https://rustup.rs and re-run."
}

$root = Split-Path -Parent $PSScriptRoot
$cli = Join-Path $root 'crates/unshackled-cli'

# The LocalMind learning engine is a git submodule; the `learning` feature needs
# it checked out.
if ((Test-Path (Join-Path $root '.gitmodules')) -and (Get-Command git -ErrorAction SilentlyContinue)) {
    Write-Host "updating submodules ..."
    git -C $root submodule update --init --recursive
}

# The interactive TUI (crossterm) is unstable under the windows-gnu toolchain;
# prefer the MSVC toolchain when building with the `tui` feature.
if (-not $Toolchain -and ($Features -match 'tui') -and (Get-Command rustup -ErrorAction SilentlyContinue)) {
    if ((rustup toolchain list) -match 'msvc') {
        $Toolchain = 'stable-x86_64-pc-windows-msvc'
        Write-Host "using the MSVC toolchain for a stable 'chat' (TUI) build."
    } else {
        Write-Warning "the 'tui' feature (chat) is unstable on the windows-gnu toolchain."
        Write-Warning "install MSVC for a working 'chat':  rustup toolchain install stable-x86_64-pc-windows-msvc"
        Write-Warning "or skip it:  ./install/install.ps1 -Features learning"
    }
}

Write-Host "building and installing the unshackled CLI (features: $Features) ..."
if ($Toolchain) {
    cargo "+$Toolchain" install --path $cli --features $Features --locked
} else {
    cargo install --path $cli --features $Features --locked
}

Write-Host ""
Write-Host "installed 'unshackled'. verify with:"
Write-Host "    unshackled doctor"
