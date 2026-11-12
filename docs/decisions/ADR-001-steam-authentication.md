# ADR-001: Steam Authentication

**Status:** Proposed
**Date:** 2026-11-12

## Context

GameVault is planned as a Windows Tauri desktop app. Steam OpenID documentation describes a website redirect flow that returns a SteamID. The reviewed Valve docs do not specify a native desktop return URI, loopback callback or Tauri integration. OpenID identity does not authorize private player data.

## Problem

How can GameVault link an account without collecting Steam credentials and while validating identity safely?

## Considered alternatives

- Direct desktop OpenID callback: no supported callback registration/mechanism found in reviewed Steam docs; reject as an assumed design.
- Website OpenID relying party: documented web pattern; requires a GameVault site/domain and secure return endpoint.
- Ask for Steam password/cookie/Guard code: prohibited by project requirements and Steam terms; rejected.
- No account linking: compatible with a reduced offline product, but owner must change current product goals.

## Decision

**Recommendation:** investigate website OpenID on a GameVault-controlled domain, with the site validating Steam's assertion and issuing a short-lived, revocable GameVault session to the desktop through a carefully reviewed handoff. This remains proposed and is blocked on backend acceptability and Valve/application registration confirmation. Do not implement a native callback by assumption.

OpenID relying party must pin Steam's OP endpoint, use a maintained library, bind state to the initiating request, verify signature and all required fields, enforce exact `return_to`, check discovery and replay-resistant nonce, require TLS, and avoid leaking login/session material through logs or referrers. A SteamID from an unverified URL is untrusted.

## Consequences

- A website/service, domain, session lifecycle, abuse controls, privacy notice, hosting country and support operations are introduced.
- OpenID establishes Steam identity only. Separate API authorization/access requirements remain.
- If hosting is unacceptable, account-connected v1 must be deferred or re-scoped by owner approval.

## Risks

Valve docs do not establish a desktop callback or confirm the proposed service/key relationship. OpenID 2.0 is an older protocol and must use a well-maintained implementation and current security review. Incorrect handoff could expose or bind sessions to the wrong Steam account.

## References

- https://partner.steamgames.com/doc/features/auth
- https://steamcommunity.com/dev
- https://openid.net/specs/openid-authentication-2_0.html
- https://steamcommunity.com/dev/apiterms
- `../FEASIBILITY_REVIEW.md`
