# Licensing, Attribution, and Branding

**Status:** no license chosen; no third-party runtime dependencies adopted. **Research date:** 2026-11-12. This comparison is informational, not legal advice.

## Practical project-license options

| Option | Practical effect if GameVault is distributed | Trade-off |
|---|---|---|
| **MIT** | Permissive; downstream users may reuse/modify/redistribute, including in proprietary products, while retaining copyright and permission notices. | Short and familiar. The text does not include the detailed contributor patent license found in Apache-2.0. |
| **Apache-2.0** | Permissive commercial reuse and redistribution with preservation of license/copyright notices, marking modified files, and any applicable NOTICE; includes an express contributor patent license. | More detailed notice/compliance requirements. It does not grant trademark rights. |
| **GPL-3.0-only** | Allows distribution and modification, but conveyed derivative/combined works are subject to GPLv3 source and same-license conditions, with applicable notices and corresponding-source obligations. | Strong copyleft may be a deliberate community goal, but affects how derivatives and dependencies can be combined/distributed. |

Primary license texts: [MIT](https://opensource.org/license/mit/), [Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0.txt), and [GNU GPL-3.0](https://www.gnu.org/licenses/gpl-3.0.html). Review the exact text before adoption.

### Recommendation (not a decision)

If the goal is an openly licensed desktop app that can be reused in both open and proprietary downstream software, **Apache-2.0 is a reasonable default recommendation** because it is permissive and contains an express patent grant. MIT is a simpler alternative if minimal conditions matter more. GPL-3.0-only is appropriate if the owner deliberately wants reciprocal copyleft for distributed derivatives.

The owner must choose. This document does not grant permission to add a `LICENSE` file or license the project. Confirm intended commercial use, contribution model, and desired downstream freedoms before deciding.

## Third-party reuse and notices

- Track exact source URL, revision/version, author, license text, notices, modifications, and distribution obligations for every code, package, font, icon, artwork, and generated asset.
- Review direct and transitive dependencies and bundled native libraries. Do not infer an asset's license from the code repository license.
- SAM is under zlib terms in its current upstream `LICENSE.txt`; its README also attributes icons to Fugue Icons. No SAM code or icon asset was copied. See [SAM/ASF audit](SAM_AUDIT.md).
- ASF is under Apache-2.0, but it is not approved as a dependency. Its dependency and account-automation surface requires an independent review; Apache-2.0 does not make it fit GameVault or grant Steam-related permissions.
- No Steamworks SDK, SAM, ASF, SteamKit2, game artwork, or Steam logo has been copied into this repository.

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
