import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { detectLanguage, languageNames, languages, localizeSteamError, translate, type Language } from "./i18n";
import "./App.css";

type GameCandidate = { app_id: string; name: string; schema_available: boolean; icon_jpeg: number[] | null };
type Inspection = {
  installation_found: boolean;
  games: GameCandidate[];
  client_session: string;
  achievements: string;
  note: string;
  scan_incomplete: boolean;
};
type Achievement = { id: string; name: string; description: string; achieved: boolean; unlock_time: number | null };
type AchievementReadResult = { status: "available" | "unavailable"; achievements: Achievement[]; message: string | null };

const localeFor: Record<Language, string> = { en: "en-US", de: "de-DE", fr: "fr-FR", es: "es-ES", pt: "pt-PT" };

function GameIcon({ game }: { game: GameCandidate }) {
  const [imageFailed, setImageFailed] = useState(false);

  if (!game.icon_jpeg || imageFailed) {
    return <div className="game-icon" aria-hidden="true">{game.name.slice(0, 1).toUpperCase()}</div>;
  }

  let binary = "";
  for (const byte of game.icon_jpeg) binary += String.fromCharCode(byte);
  const imageUrl = `data:image/jpeg;base64,${btoa(binary)}`;

  return <div className="game-icon" aria-hidden="true"><img src={imageUrl} alt="" onError={() => setImageFailed(true)} /></div>;
}

function App() {
  const [language, setLanguage] = useState<Language>(detectLanguage);
  const [result, setResult] = useState<Inspection | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");
  const [achievementResults, setAchievementResults] = useState<Record<string, AchievementReadResult>>({});
  const [achievementBusy, setAchievementBusy] = useState<string | null>(null);
  const t = (key: Parameters<typeof translate>[1], values?: Record<string, string | number>) => translate(language, key, values);

  useEffect(() => {
    document.documentElement.lang = language;
    localStorage.setItem("gamevault-language", language);
  }, [language]);

  async function inspect() {
    setBusy(true);
    setError("");
    try {
      setResult(await invoke<Inspection>("inspect_local_steam"));
    } catch {
      setError(t("inspectError"));
    } finally {
      setBusy(false);
    }
  }

  async function readAchievements(appId: string) {
    setAchievementBusy(appId);
    try {
      const response = await invoke<AchievementReadResult>("read_game_achievements", { appId });
      setAchievementResults((previous) => ({ ...previous, [appId]: response }));
    } catch {
      setAchievementResults((previous) => ({
        ...previous,
        [appId]: { status: "unavailable", achievements: [], message: null },
      }));
    } finally {
      setAchievementBusy(null);
    }
  }

  const readCount = Object.values(achievementResults).filter((entry) => entry.status === "available").length;

  return (
    <main className="shell">
      <aside className="rail" aria-label="GameVault">
        <div className="brand-mark">G</div>
        <span className="rail-active" aria-label={t("libraryTitle")}>▦</span>
        <span aria-hidden="true">◷</span><span aria-hidden="true">⚙</span>
        <div className="rail-bottom">GK</div>
      </aside>
      <section className="content">
        <header>
          <div><p className="eyebrow">{t("tagline")}</p><h1>GameVault</h1></div>
          <div className="header-actions">
            <label className="language-picker"><span>{t("language")}</span>
              <select value={language} onChange={(event) => setLanguage(event.target.value as Language)} aria-label={t("language")}>
                {languages.map((code) => <option key={code} value={code}>{languageNames[code]}</option>)}
              </select>
            </label>
            <span className="local-pill"><i /> {t("localOnly")}</span>
          </div>
        </header>
        <section className="welcome">
          <div><p className="eyebrow">{t("steamLibrary")}</p><h2>{t("headline")}</h2><p>{t("intro")}</p></div>
          <button onClick={inspect} disabled={busy}>{busy ? t("scanning") : t("scan")}<span>→</span></button>
        </section>
        <div className="section-heading">
          <div><h2>{t("overview")}</h2><p>{t("localReadOnly")}</p></div>
          <span className="muted">{result ? t("gamesFound", { count: new Intl.NumberFormat(localeFor[language]).format(result.games.length) }) : t("notScanned")}</span>
        </div>
        {error && <div className="error" role="alert">{error}</div>}
        {!result ? <div className="empty"><div className="empty-icon">▤</div><h3>{t("waitingTitle")}</h3><p>{t("waitingText")}</p></div> : <>
          <div className="status-grid">
            <article className="status-card"><span>{t("steamInstall")}</span><strong>{result.installation_found ? t("found") : t("notFound")}</strong><small>{result.installation_found ? t("commonLocations") : t("noSteam")}</small></article>
            <article className="status-card"><span>{t("steamSession")}</span><strong>{t("notConnected")}</strong><small>{t("sessionDetail")}</small></article>
            <article className="status-card"><span>{t("achievements")}</span><strong>{readCount > 0 ? t("dataRead") : Object.keys(achievementResults).length > 0 ? t("unavailable") : t("notRead")}</strong><small>{t("achievementsDetail")}</small></article>
          </div>
          <div className="notice">{result.installation_found ? t("detectedNote") : t("noSteam")}{result.scan_incomplete ? ` ${t("incompleteScan")}` : ""}</div>
          <div className="game-list" aria-label={t("libraryTitle")}>{result.games.map((game) => {
            const achievementResult = achievementResults[game.app_id];
            return <article className="game-card" key={game.app_id}>
              <div className="game-row"><GameIcon game={game} /><div className="game-name"><strong>{game.name}</strong><span>AppID {game.app_id} · {t("installed")}</span></div>
                <button className="achievement-load" onClick={() => readAchievements(game.app_id)} disabled={achievementBusy !== null}>
                  {achievementBusy === game.app_id ? t("loading") : achievementResult ? t("reloadAchievements") : t("loadAchievements")}
                </button>
              </div>
              {achievementResult && <div className="achievement-panel">
                {achievementResult.status === "unavailable" ? <p className="achievement-unavailable">{localizeSteamError(achievementResult.message, language)}</p> : <>
                  <div className="achievement-summary"><strong>{new Intl.NumberFormat(localeFor[language]).format(achievementResult.achievements.filter((item) => item.achieved).length)} / {new Intl.NumberFormat(localeFor[language]).format(achievementResult.achievements.length)}</strong><span>{t("earned")}</span></div>
                  {achievementResult.achievements.length === 0 ? <p className="achievement-unavailable">{t("noAchievements")}</p> : <ul className="achievement-list">{achievementResult.achievements.map((achievement) => <li key={achievement.id}>
                    <span className={achievement.achieved ? "achievement-state earned" : "achievement-state"}>{achievement.achieved ? "✓" : "○"}</span>
                    <div><strong>{achievement.name}</strong>{achievement.description && <p>{achievement.description}</p>}</div>
                    {achievement.unlock_time !== null && <time>{new Intl.DateTimeFormat(localeFor[language]).format(new Date(achievement.unlock_time * 1000))}</time>}
                  </li>)}</ul>}
                </>}
                <small>{t("steamRequestNote")}</small>
              </div>}
            </article>;
          })}</div>
        </>}
        <footer><span>{t("windowsSteam")}</span><span>{t("prototype")}</span></footer>
      </section>
    </main>
  );
}

export default App;
