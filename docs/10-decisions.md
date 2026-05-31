# Architecture Decision Records

This file starts the decision log. Add new records at the top.

## ADR-0004: No Private Endpoint Adapters

Status: accepted

Unshackled will not implement adapters for private, undocumented, or
consumer-product endpoints. Provider integrations must use official APIs, local
servers, or explicit user-owned custom endpoints.

Reason:

- reduces legal and account risk
- keeps provider contracts stable
- avoids brittle reverse-engineered behavior
- preserves trust in the project

## ADR-0003: Project Files Are Harness Source of Truth

Status: accepted

The harness treats `brief.md` and `PROGRESS.md` as authoritative. Transcripts
are helpful context but not authoritative state.

Reason:

- users can inspect and edit plans
- sessions can resume after crashes
- implementation remains auditable

## ADR-0002: Provider-Neutral Core

Status: accepted

The core crate must not depend on provider-specific APIs or payload shapes.

Reason:

- avoids coupling the product to one vendor
- makes local models first-class
- keeps tests independent of network access

## ADR-0001: Rust Workspace with Narrow Crates

Status: accepted

Unshackled is split into narrow crates rather than one large binary crate.

Reason:

- clearer boundaries
- easier clean-room review
- smaller test surfaces
- easier future embedding

