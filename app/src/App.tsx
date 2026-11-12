import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type GameCandidate = { app_id: string; name: string; manifest_path: string; schema_available: boolean };
type Inspection = {
  installation_found: boolean;
  steam_roots: string[];
  games: GameCandidate[];
  client_session: string;
  achievements: string;
  note: string;
};

function App() {
  const [result, setResult] = useState<Inspection | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");

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
            <article className="status-card"><span>STEAM-INSTALLATION</span><strong>{result.installation_found ? "Gefunden" : "Nicht gefunden"}</strong><small>{result.steam_roots.length ? result.steam_roots.join(" · ") : "Übliche Installationsorte wurden geprüft"}</small></article>
            <article className="status-card"><span>STEAM-SITZUNG</span><strong>{result.client_session}</strong><small>Interne Client-Verbindung noch nicht implementiert</small></article>
            <article className="status-card"><span>ACHIEVEMENTS</span><strong>{result.achievements}</strong><small>Schema-Inhalte werden noch nicht gelesen</small></article>
          </div>
          <div className="notice">{result.note}</div>
          <div className="game-list">{result.games.map((game) => <article className="game-row" key={game.app_id}><div className="game-icon">{game.name.slice(0, 1).toUpperCase()}</div><div className="game-name"><strong>{game.name}</strong><span>AppID {game.app_id} · lokal installiert</span></div><span className={game.schema_available ? "tag available" : "tag"}>{game.schema_available ? "Schema-Datei vorhanden" : "Kein Schema"}</span></article>)}</div>
        </>}
        <footer><span>GameVault · Windows · Steam</span><span>Experimenteller lokaler Prototyp</span></footer>
      </section>
    </main>
  );
}

export default App;
