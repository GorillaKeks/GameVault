# ADR-003: Local Data Storage

**Status:** Proposed
**Date:** 2026-11-12

## Context

The product brief proposes a local-first Windows app. No application schema, library, cache or preferences exist. Data sources and retention remain gated by Steam API and privacy review.

## Problem

Where should approved local preferences and any permitted cached Steam responses live, and how should access be constrained?

## Considered alternatives

- SQLite behind Rust repository/migrations: matches embedded local app and narrow native boundary.
- Tauri SQL plugin with frontend SQL: simplifies setup but gives WebView database authority and expands command surface.
- Files/JSON: simpler for settings but weaker for relational library/cache query and migration needs.
- No persistence: simplest but cannot support preferences/offline cache envisioned in product brief.

## Decision

**Recommendation:** use SQLite in per-user application data, accessed only through a Rust-owned repository with parameterized statements, explicit DTOs, ordered migrations and tested backup/restore. Store only owner-approved local preferences and minimum API cache, with source and fetch time; no production DB/schema is authorized in this phase. Select an exact Rust crate/bundling strategy during scaffold planning after license, MSRV and Windows build review.

## Consequences

The WebView receives domain commands rather than arbitrary SQL. Cache remains local and can support offline display, but must be clearly stale and must follow verified retention/deletion terms. SQLite CLI absence does not block an embedded library.

## Risks

SQLite wrapper/native bundling licenses may differ from SQLite core's public-domain status. Destructive migration and backup correctness require tests once code exists. Data retention policy cannot be finalized until API use is approved.

## References

- https://www.sqlite.org/whentouse.html
- https://www.sqlite.org/copyright.html
- https://v2.tauri.app/plugin/sql/
- `../ARCHITECTURE.md`
- `../SECURITY.md`
