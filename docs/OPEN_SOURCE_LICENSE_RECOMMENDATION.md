# GameVault Open-Source License Recommendation

**Owner constraint:** GameVault must be free and open source.
**Recommendation:** Apache License 2.0.
**Status:** Proposed; the owner has not approved an exact license text. **Date:** 2026-11-12.

## Comparison

| License | Commercial redistribution and derivatives | Contributions | Dependencies / Tauri fit | Future plugins and adoption |
|---|---|---|---|---|
| MIT | Permissive; downstream commercial and closed-source derivatives allowed with copyright/permission notice | Very simple; no explicit contribution patent grant in the standard text | Very compatible with React, Rust/Tauri and permissive crates | Lowest friction and broad familiarity; permits proprietary forks/plugins |
| Apache-2.0 | Permissive; commercial and closed-source derivatives allowed with notices, modified-file marking and applicable NOTICE | Explicit contributor patent license and patent-termination clauses; more compliance text | Strong compatibility with Tauri code and many Rust dependencies, but exact dependency licenses still require inventory | Permissive plugin ecosystem and broad adoption; does not itself guarantee plugins remain open |
| GPL-3.0-only | Commercial sale is allowed, but conveying covered derivatives/combined works brings corresponding-source and same-license obligations subject to GPL terms | Strong copyleft encourages reciprocal distributed changes; contributor/CLA terms still needed | Many dependencies can be GPL-compatible, but distribution/linking boundaries require careful review; not the easiest default for mixed permissive stack | Encourages a shared codebase, but may discourage proprietary plugin integrations; interpret linking carefully with counsel |
| MPL-2.0 | Commercial use allowed; copyleft applies file-by-file to covered files/modifications, with larger works permitted under other terms | Contributors grant rights; notices/source duties attach to covered files | Often compatible with larger permissively licensed applications, but exact file/module boundaries and dependency licenses matter | Moderate adoption; permits proprietary surrounding code/plugins while requiring modified MPL-covered files to remain available |

The project's public GitHub visibility does not grant reuse permission. A license is distinct from Steam/Valve branding, game artwork, fonts, dependencies and user data. Each dependency must be inventoried by exact version and bundled asset.

## Recommendation rationale

**Apache-2.0 is the recommended default** for an explicitly free/open-source desktop application intended for commercial redistribution and community contributions. It is permissive, includes an express contributor patent grant, is used by components in the Tauri ecosystem, and supports broad adoption. This means downstream closed-source forks remain possible under Apache's terms; if the owner wants reciprocal openness, GPL-3.0-only or MPL-2.0 may better match that goal. MIT is the simplest alternative if minimal license text is the priority and the owner accepts its patent-grant trade-off.

The exact SQLite/Rust packages, frontend packages, fonts, icon set and art source are not selected. No framework-level statement can replace a version-specific dependency and notice audit. A future plugin interface also needs its own API stability, trust and contribution policy; license choice alone cannot secure plugins.

## Proposed license file (not effective)

Candidate file: [`license-candidate/LICENSE-APACHE-2.0.proposed.txt`](license-candidate/LICENSE-APACHE-2.0.proposed.txt).

It is a draft marker, **not a grant of rights and not the root project license**. On explicit owner approval, copy the exact current [Apache-2.0 license text](https://www.apache.org/licenses/LICENSE-2.0.txt) to the repository root `LICENSE`, add copyright holder/year, set SPDX identifiers in new source files, and keep applicable `NOTICE` and third-party notices. Until then, the repository remains without a selected license.

## Approval checklist

- [ ] Owner confirms Apache-2.0 as the exact project license.
- [ ] Confirm copyright holder(s) and contribution ownership/CLA policy.
- [ ] Audit exact direct/transitive code, generated assets and bundled native components.
- [ ] Add root `LICENSE` text and third-party notices only after approval.
- [ ] Keep Steam/game art and trademarks outside any assumed license grant.

## Primary license references (checked 2026-11-12)

- [Apache-2.0 official text](https://www.apache.org/licenses/LICENSE-2.0.txt)
- [MIT license](https://opensource.org/license/mit/)
- [GNU GPLv3](https://www.gnu.org/licenses/gpl-3.0.html)
- [Mozilla Public License 2.0](https://www.mozilla.org/en-US/MPL/2.0/)
- [Tauri project license information](https://github.com/tauri-apps/tauri)
- [React project license](https://github.com/facebook/react)
- [SQLite public-domain status](https://www.sqlite.org/copyright.html)
