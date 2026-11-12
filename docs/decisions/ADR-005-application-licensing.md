# ADR-005: GameVault Project License

**Status:** Proposed
**Date:** 2026-11-12

## Context

The public repository has no license file. Public availability alone grants no general permission to reuse GameVault code. No third-party runtime dependency has been added.

## Problem

Which license should govern GameVault source distribution, downstream reuse, contributions and commercial distribution?

## Considered alternatives

- MIT: short permissive license; retain copyright and permission notices.
- Apache-2.0: permissive, express contributor patent license, NOTICE and modified-file requirements.
- GPL-3.0-only: reciprocal terms and corresponding-source requirements for covered conveyed works; scope requires legal review.
- MPL-2.0: file-level copyleft for covered files; larger work can use other terms, with covered-source obligations.
- Proprietary: restrict redistribution/reuse; requires owner-controlled terms and contribution rights.

## Decision

**Recommendation:** if the owner wants openly licensed and permissively reusable code, Apache-2.0 is a reasonable default; MIT is simpler. No selection is made until the owner confirms open-source versus closed-source goals and contribution model. Do not create a `LICENSE` file or imply a license in the README yet.

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
