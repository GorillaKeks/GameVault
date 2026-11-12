# Licensing, Attribution, and Branding

**Status:** the owner requires free and open-source software; the exact license is unselected. No third-party runtime dependencies have been adopted. **Research date:** 2026-11-12. This comparison is informational, not legal advice.

## Practical project-license options

| Option | Practical effect if GameVault is distributed | Trade-off |
|---|---|---|
| **MIT** | Permissive; downstream users may reuse/modify/redistribute, including in proprietary products, while retaining copyright and permission notices. | Short and familiar. The text does not include the detailed contributor patent license found in Apache-2.0. |
| **Apache-2.0** | Permissive commercial reuse and redistribution with preservation of license/copyright notices, marking modified files, and any applicable NOTICE; includes an express contributor patent license. | More detailed notice/compliance requirements. It does not grant trademark rights. |
| **GPL-3.0-only** | Allows distribution and modification, but conveyed derivative/combined works are subject to GPLv3 source and same-license conditions, with applicable notices and corresponding-source obligations. | Strong copyleft may be a deliberate community goal, but affects how derivatives and dependencies can be combined/distributed. |
| **MPL-2.0** | File-level copyleft for covered files and their modifications; a larger work may contain separately licensed files, subject to MPL notice/source obligations. | Middle ground between permissive and whole-work copyleft; requires file-level compliance tracking. |
| **Proprietary** | Owner can reserve rights and restrict redistribution, subject to valid rights ownership and applicable law. | No open-source permission; requires owner-controlled license terms, contribution rights and distribution terms. |

Primary license texts: [MIT](https://opensource.org/license/mit/), [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0.txt), and [GNU GPL-3.0](https://www.gnu.org/licenses/gpl-3.0.html). Review the exact text before adoption.

### Recommendation (not a decision)

Given the confirmed free/open-source requirement, **Apache-2.0 is recommended** for permissive commercial reuse and its express contributor patent grant. MIT is simpler if minimal conditions matter more. GPL-3.0-only or MPL-2.0 is preferable only if the owner wants stronger reciprocal obligations for derivatives.

The exact license text and copyright holder still require explicit owner approval. This document does not grant permission to add a root `LICENSE` file. See [`OPEN_SOURCE_LICENSE_RECOMMENDATION.md`](OPEN_SOURCE_LICENSE_RECOMMENDATION.md) and the [non-operative proposed candidate marker](license-candidate/LICENSE-APACHE-2.0.proposed.txt).

The project is required to remain free and open source; a proprietary license is no longer under consideration. The specific Apache-2.0 choice remains Proposed. See [`decisions/ADR-005-application-licensing.md`](decisions/ADR-005-application-licensing.md).

## Third-party reuse and notices

- Track exact source URL, revision/version, author, license text, notices, modifications, and distribution obligations for every code, package, font, icon, artwork, and generated asset.
- Review direct and transitive dependencies and bundled native libraries. Do not infer an asset's license from the code repository license.
- SAM is under zlib terms in its current upstream `LICENSE.txt`; its README also attributes icons to Fugue Icons. No SAM code or icon asset was copied. See [SAM/ASF audit](SAM_AUDIT.md).
- ASF is under Apache-2.0, but it is not approved as a dependency. Its dependency and account-automation surface requires an independent review; Apache-2.0 does not make it fit GameVault or grant Steam-related permissions.
- No Steamworks SDK, SAM, ASF, SteamKit2, game artwork, or Steam logo has been copied into this repository.
- Tauri's upstream code uses MIT and/or Apache-2.0 by component, React is MIT, and SQLite core deliverable code is public domain. These do not establish the license of wrappers, optional SQLite components, build tools, dependencies, or artwork; inventory exact selected versions before distribution.

## Steam/Valve branding, data, and artwork

Steam Web API Terms (currently marked as last updated July 2010) contain conditions for API access and Steam Data presentation, user request/consent, confidentiality, privacy policy, storage-country disclosure, and call volume. Recheck the current terms before implementation/release: [Steam Web API Terms](https://steamcommunity.com/dev/apiterms).

Valve's [Steam Branding Guidelines](https://partner.steamgames.com/doc/marketing/branding) say use of the logo must not imply endorsement/affiliation and specify approved artwork and legal attribution rules. The guidelines can change. GameVault should remain visually independent and carry a clear non-affiliation statement.

The Web API terms and branding guidance reviewed do not establish blanket redistribution rights for every game icon, capsule, or store image. Do not package Steam/game art. For any runtime display or cache, identify the exact documented source and verify the applicable terms, attribution, and retention before implementation.

## Product name and domain

“GameVault” and “Your Gaming Progress. One Place.” remain working choices. No trademark, product-name, domain, or confusingly similar project clearance has been performed. Recheck before publishing the app, website, or store listing; no legal conclusion is made here.

## Release checklist

- [ ] Owner selects a project license; add exact upstream text and license identifier.
- [ ] Third-party notices generated/reviewed from the actual dependency and asset inventory.
- [ ] Steam API key/app registration and data use comply with current Valve terms.
- [ ] Privacy notice states requested data, storage, location/country, retention, and deletion.
- [ ] Game artwork, Steam logo and trademark use reviewed against current guidance.
- [ ] Product name/domain clearance completed.
- [ ] Windows signing/updater key handling and release notices reviewed.
