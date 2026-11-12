# ADR-005: GameVault Project License

**Status:** Proposed (open-source model owner-confirmed; exact license pending)
**Date:** 2026-11-12

## Context

The owner confirmed that GameVault must be free and open source. The public repository still has no license file; public availability alone grants no general permission to reuse GameVault code. No third-party runtime dependency has been added.

## Problem

Which license should govern GameVault source distribution, downstream reuse, contributions and commercial distribution?

## Considered alternatives

- MIT: short permissive license; retain copyright and permission notices.
- Apache-2.0: permissive, express contributor patent license, NOTICE and modified-file requirements.
- GPL-3.0-only: reciprocal terms and corresponding-source requirements for covered conveyed works; scope requires legal review.
- MPL-2.0: file-level copyleft for covered files; larger work can use other terms, with covered-source obligations.
- Proprietary: restrict redistribution/reuse; requires owner-controlled terms and contribution rights.

## Decision

**Recommendation:** Apache-2.0, because the owner requires FOSS and the project targets commercial redistribution and community contribution with permissive reuse and an express contributor patent grant. MIT is the simpler alternative; GPL-3.0-only or MPL-2.0 fit if the owner wants stronger reciprocal obligations. The exact Apache-2.0 text and copyright holder remain pending explicit approval. A non-operative candidate marker is prepared in `../license-candidate/`; do not add a root `LICENSE` until approval.

## Consequences

Before distribution, inventory exact dependency and asset licenses, notices, fonts, artwork and bundled native components. Tauri's code is MIT and/or Apache-2.0 by component, React is MIT, and SQLite core is public domain; wrappers and optional components must be reviewed separately.

## Risks

This is not legal advice. Compatibility depends on actual versions and distribution form. The project's name, Steam branding and game artwork rights are separate from the source license.

## References

- https://opensource.org/license/mit/
- https://www.apache.org/licenses/LICENSE-2.0.txt
- https://www.gnu.org/licenses/gpl-3.0.html
- https://www.mozilla.org/en-US/MPL/2.0/
- https://github.com/tauri-apps/tauri
- https://github.com/facebook/react
- https://www.sqlite.org/copyright.html
- `../LICENSING.md`
- `../OPEN_SOURCE_LICENSE_RECOMMENDATION.md`
