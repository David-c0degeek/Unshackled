# Lessons

- LocalMind needs memory-management APIs at the store boundary; host adapters
  should not manipulate LocalMind database tables directly.
- LocalMind's lockfile can select newer transitive crates than the Unshackled
  MSRV supports; keep the vendored lockfile aligned with Cargo 1.82-compatible
  dependency versions when adding store APIs.
- Context injection lookups must be read-only. A missing LocalMind project
  should return no context instead of initializing `.localmind` as a side effect.
- The Windows GNU TUI build on rustc 1.82 is sensitive to transitive
  `parking_lot_core` updates. Keep a smoke test that starts
  `cargo run --features tui -- --help` so startup crashes are caught directly.
