import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
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

function GameIcon({ game }: { game: GameCandidate }) {
  const [imageFailed, setImageFailed] = useState(false);

  if (!game.icon_jpeg || imageFailed) {
    return <div className="game-icon" aria-hidden="true">{game.name.slice(0, 1).toUpperCase()}</div>;
  }

  let binary = "";
  for (const byte of game.icon_jpeg) binary += String.fromCharCode(byte);
  const imageUrl = `data:image/jpeg;base64,${btoa(binary)}`;

  return (
    <div className="game-icon" aria-hidden="true">
      <img src={imageUrl} alt="" onError={() => setImageFailed(true)} />
    </div>
  );
}

function App() {
  const [result, setResult] = useState<Inspection | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");
  const [achievementResults, setAchievementResults] = useState<Record<string, AchievementReadResult>>({});
  const [achievementBusy, setAchievementBusy] = useState<string | null>(null);

  async function inspect() {
    setBusy(true);
    setError("");
    try {
      setResult(await invoke<Inspection>("inspect_local_steam"));
    } catch {
      setError("Steam-Verzeichnisse konnten nicht geprüft werden.");
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
        [appId]: { status: "unavailable", achievements: [], message: "Errungenschaften konnten nicht gelesen werden." },
      }));
    } finally {
      setAchievementBusy(null);
    }
  }

  return (
    <main className="shell">
      <aside className="rail"><div className="brand-mark">G</div><span className="rail-active">▦</span><span>◷</span><span>⚙</span><div className="rail-bottom">GK</div></aside>
      <section className="content">
        <header><div><p className="eyebrow">YOUR GAMING PROGRESS. ONE PLACE.</p><h1>GameVault</h1></div><span className="local-pill"><i /> LOCAL ONLY</span></header>
        <section className="welcome">
          <div><p className="eyebrow">STEAM LIBRARY</p><h2>Deine Spiele, auf einen Blick.</h2><p>GameVault prüft lokale Steam-Installationen und zeigt, welche Daten auf diesem PC verfügbar sind.</p></div>
          <button onClick={inspect} disabled={busy}>{busy ? "Wird geprüft …" : "Steam lokal prüfen"}<span>→</span></button>
        </section>
        <div className="section-heading"><div><h2>Übersicht</h2><p>Lokale Erkennung · Nur lesend</p></div><span className="muted">{result ? `${result.games.length} lokale Einträge` : "Noch nicht geprüft"}</span></div>
        {error && <div className="error">{error}</div>}
        {!result ? <div className="empty"><div className="empty-icon">▤</div><h3>Deine Bibliothek wartet</h3><p>Starte die lokale Prüfung. GameVault verändert keine Steam-Dateien und benötigt keine Zugangsdaten.</p></div> : <>
          <div className="status-grid">
            <article className="status-card"><span>STEAM-INSTALLATION</span><strong>{result.installation_found ? "Gefunden" : "Nicht gefunden"}</strong><small>Übliche Installationsorte wurden geprüft</small></article>
            <article className="status-card"><span>STEAM-SITZUNG</span><strong>{result.client_session}</strong><small>Verbindung nur während einer angeforderten Leseaktion</small></article>
            <article className="status-card"><span>ACHIEVEMENTS</span><strong>{Object.values(achievementResults).some((entry) => entry.status === "available") ? "Daten gelesen" : Object.keys(achievementResults).length > 0 ? "Nicht verfügbar" : result.achievements}</strong><small>Für einzelne installierte Spiele auf Anfrage lesbar</small></article>
          </div>
          <div className="notice">{result.note}{result.scan_incomplete ? " Einige lokale Einträge konnten wegen Zugriffsbeschränkungen oder Sicherheitslimits nicht geprüft werden." : ""}</div>
          <div className="game-list">{result.games.map((game) => {
            const achievementResult = achievementResults[game.app_id];
            return <article className="game-card" key={game.app_id}>
              <div className="game-row"><GameIcon game={game} /><div className="game-name"><strong>{game.name}</strong><span>AppID {game.app_id} · lokal installiert</span></div><button className="achievement-load" onClick={() => readAchievements(game.app_id)} disabled={achievementBusy !== null}>{achievementBusy === game.app_id ? "Lese …" : achievementResult ? "Erneut lesen" : "Errungenschaften anzeigen"}</button></div>
              {achievementResult && <div className="achievement-panel">
                {achievementResult.status === "unavailable" ? <p className="achievement-unavailable">{achievementResult.message ?? "Errungenschaftsdaten nicht verfügbar."}</p> : <>
                  <div className="achievement-summary"><strong>{achievementResult.achievements.filter((item) => item.achieved).length} / {achievementResult.achievements.length}</strong><span>freigeschaltet · direkt vom Steam-Client gelesen</span></div>
                  {achievementResult.achievements.length === 0 ? <p className="achievement-unavailable">Steam meldet für dieses Spiel keine Errungenschaften.</p> : <ul className="achievement-list">{achievementResult.achievements.map((achievement) => <li key={achievement.id}>
                    <span className={achievement.achieved ? "achievement-state earned" : "achievement-state"}>{achievement.achieved ? "✓" : "○"}</span>
                    <div><strong>{achievement.name}</strong>{achievement.description && <p>{achievement.description}</p>}</div>
                    {achievement.unlock_time !== null && <time>{new Date(achievement.unlock_time * 1000).toLocaleDateString()}</time>}
                  </li>)}</ul>}
                </>}
                <small>Steam verarbeitet die Anfrage für dein angemeldetes Konto. Experimentelle Client-Schnittstelle · Nur lesend</small>
              </div>}
            </article>;
          })}</div>
        </>}
        <footer><span>GameVault · Windows · Steam</span><span>Experimenteller lokaler Prototyp</span></footer>
      </section>
    </main>
  );
}

export default App;
