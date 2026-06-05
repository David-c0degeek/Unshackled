# 00 - Tooling Research And Readiness

## Goal

Confirm the current repository state, implementation surfaces, baseline
verification, official references, applicable skills, and local tooling before
starting code changes. This subject turns the review findings into concrete
implementation constraints and updates the rest of the plan if the code has
drifted since `docs/codex-review.md` was written.

## Boxes

- [x] **00.1** (agent) Read repo instructions, `docs/codex-review.md`, and the
      authoritative docs listed in the main plan; list blocking constraints,
      clean-room rules, and existing local conventions.
- [x] **00.2** (agent) Inventory touched crates, modules, CI workflows,
      dependency manifests, and test files for all eight findings.
- [x] **00.3** (agent) Run or intentionally defer the baseline gate:
      `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`,
      `cargo test --workspace`, and `cargo check --workspace`; record exact
      blockers and reproduction commands.
- [x] **00.4** (agent) Reproduce the review's supply-chain failures with local
      tool versions recorded: `cargo audit`, `cargo machete`, and
      `cargo deny check`.
- [x] **00.5** (agent) Research only official or primary sources needed for
      provider quota/rate-limit behavior, Rust dependency advisories, and any
      crate/tool APIs touched by implementation.
- [x] **00.6** (agent) Review applicable repo skills and local tooling; classify
      each as adopt/defer/reject with rationale, trust notes, permissions, and
      setup cost.
- [x] **00.7** (agent) Decide whether the local read-only behavior reference is
      needed. If used, record only high-level behavior questions and add the
      required provenance note; otherwise record that repo docs were sufficient.
- [x] **00.8** (agent) Bake findings into this plan: update subject boxes,
      cross-cutting rules, gates, decision log, manual actions, and lessons as
      needed. End with an implementation-readiness summary.

## Hindsight checkpoint

- [x] Captain Hindsight review recorded
- [x] Verdict is `CLOSE`

## Progress log

> One line per slice. Date - slice number - box IDs touched - what shipped -
> how verified - checkpoint commit/push status.

- 2026-06-05 - 1 - 00.1-00.8 - Read repo instructions, review findings,
  durable docs, workflow files, manifests, and relevant skills; confirmed repo
  docs were sufficient and the read-only behavior reference was not used.
  Baseline showed normal Rust gates healthy and supply-chain checks failing on
  the known audit/machete findings. Checkpoint not committed/pushed by agent.

## Captain Hindsight

1. Keep: Treating the task as Tier L was correct because the work spans harness,
   sandbox, provider runtime, MCP, docs, supply-chain tooling, and a submodule.
2. Fix before closing: None.
3. Record: Repo docs were sufficient; no local behavior reference was used, so
   no provenance note is required beyond this plan record.
4. Risk: Supply-chain advisory remediation is constrained by the repository's
   current Rust/Cargo 1.82 MSRV.
5. Verdict: CLOSE.
