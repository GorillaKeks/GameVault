# ADR-001: Steam Authentication

**Status:** Proposed
**Date:** 2026-11-12

## Context

GameVault is a local-only Windows Tauri desktop app; the owner has explicitly ruled out a GameVault website/server. Steam OpenID documentation describes a website redirect flow that returns a SteamID. The reviewed Valve docs do not specify a native desktop return URI, loopback callback or Tauri integration. OpenID identity does not authorize private player data.

## Problem

How can GameVault link an account without collecting Steam credentials and while validating identity safely?

## Considered alternatives

- Direct desktop OpenID callback: no supported callback registration/mechanism found in reviewed Steam docs; reject as an assumed design.
- Website OpenID relying party: documented web pattern; would require a GameVault site/domain and secure return endpoint, which conflicts with the confirmed no-server constraint.
- Ask for Steam password/cookie/Guard code: prohibited by project requirements and Steam terms; rejected.
- User enters SteamID manually: avoids a new login protocol but does not authenticate ownership or grant private data access.

## Decision

**Recommendation:** do not implement Steam OpenID in v1 under the no-GameVault-server constraint unless Valve documents a safe desktop relying-party flow that can be fully validated locally. A manually supplied SteamID is an identifier only. The remaining account-data approach to investigate is a user-provided individual Web API key and SteamID, subject to explicit Valve confirmation and owner approval; this ADR does not authorize collecting a key or implementing sign-in.

OpenID relying party must pin Steam's OP endpoint, use a maintained library, bind state to the initiating request, verify signature and all required fields, enforce exact `return_to`, check discovery and replay-resistant nonce, require TLS, and avoid leaking login/session material through logs or referrers. A SteamID from an unverified URL is untrusted.

## Consequences

- The website approach would contradict the owner requirement and is not selected.
- OpenID establishes Steam identity only. Separate API authorization/access requirements remain.
- Without a permitted no-server identity/key flow, account-connected features remain blocked; do not quietly drop the mandatory library/achievement requirements.

## Risks

Valve docs do not establish a native desktop callback, and OpenID identity alone does not authorize data. A manual SteamID can refer to someone other than the local user. No login is implemented.

## References

- https://partner.steamgames.com/doc/features/auth
- https://steamcommunity.com/dev
- https://openid.net/specs/openid-authentication-2_0.html
- https://steamcommunity.com/dev/apiterms
- `../FEASIBILITY_REVIEW.md`
