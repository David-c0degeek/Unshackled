# Installing Unshackled (alpha)

Unshackled is a Rust-native, provider-neutral coding-agent harness for Windows,
Linux, and macOS (all tier-1).

## From source (recommended for alpha)

Requires the Rust toolchain (`cargo`, MSRV 1.82) from <https://rustup.rs>.

```sh
# Linux / macOS
./install/install.sh

# Windows (PowerShell)
./install/install.ps1
```

Both wrap `cargo install --path crates/unshackled-cli --locked`. After install:

```sh
unshackled doctor
```

`doctor` reports your platform, the config search paths, which provider
credentials are present (never their values), tool availability, and workspace
trust state.

## From a release archive

Each tagged release publishes per-platform archives that contain the
`unshackled` binary plus `LICENSE-MIT`. Download the archive for your platform,
extract it, and put the binary on your `PATH`.

## From crates.io

```sh
cargo install unshackled
```

(Available once the crate is published; the source build above always works.)

## Next steps

- Configure a provider — see [providers.md](providers.md).
- Read the security model — see [security.md](security.md).
