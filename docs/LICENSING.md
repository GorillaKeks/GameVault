# Licensing, Attribution, and Branding

**Status:** unresolved; no project license or third-party runtime dependencies have been adopted. **Research date:** 2026-11-12.

## Decisions required

1. Select GameVault's own source-code license before accepting external contributions or distributing source/binaries. This is a product-owner decision; do not infer it from an empty GitHub repository.
2. Add the chosen license and any contribution policy only after review of intended distribution, dependencies, and contribution model.
3. Maintain a third-party notice inventory for Rust crates, npm packages, fonts, icons, artwork, generated assets, and copied snippets.
4. Review trademark/name/domain collisions for “GameVault” before public release. The project name and tagline are working labels only.

## Third-party reuse rules

- Record exact source URL, revision/version, author, license text, copyright notices, modifications, and distribution obligations before reuse.
- Preserve upstream copyright and license text. Do not copy code just because it is public on GitHub.
- Review all transitive dependencies and bundled native libraries for license compatibility and redistribution rules.
- No Steamworks SDK, SAM, ASF, SteamKit2, game artwork, or Steam branding assets have been copied into this repository.

## Steam and Valve data/assets

Steam Web API Terms allow access and distribution of Steam Data subject to their conditions, including a privacy policy for nonpublic user data, user-requested retrieval, disclosure of storage and countries, “as is” presentation and a confidential API key. They prohibit presenting the app as endorsed/affiliated and prohibit certain misuse. Terms currently show a 2010 update and must be rechecked before implementation or release: [Steam Web API Terms](https://steamcommunity.com/dev/apiterms).

Use Steam/Valve marks descriptively, follow current brand guidance, and state that GameVault is independent. Do not redistribute game images/assets from a source unless the applicable rights and caching rules have been verified. A Steam API license does not automatically grant rights to every game's artwork or other assets.

## External audit reference: ASF

The requested ArchiSteamFarm repository currently links an Apache-2.0 license and its `LICENSE.txt` contains Apache License 2.0. Reuse would require retaining license/copyright notices, including a copy of the license and handling any upstream NOTICE file and modifications as Apache-2.0 requires. This is not legal advice. See [ASF repository](https://github.com/JustArchiNET/ArchiSteamFarm) and [ASF license](https://github.com/JustArchiNET/ArchiSteamFarm/blob/main/LICENSE.txt).

This ASF observation does not establish the license or status of Steam Achievement Manager (SAM); they are distinct projects. See [SAM/ASF audit](SAM_AUDIT.md).

## Release checklist

- [ ] Project license selected and present.
- [ ] Third-party notices generated/reviewed from actual dependency set.
- [ ] Steam API key and data use match current Valve terms and app registration.
- [ ] Privacy policy describes collection, storage location/country, retention and deletion.
- [ ] Game/Steam artwork and trademarks reviewed.
- [ ] Product name/trademark/domain review completed.
- [ ] Windows signing, updater key handling and notices reviewed.
