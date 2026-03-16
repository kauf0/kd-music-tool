import { useState, useRef, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { open, confirm } from "@tauri-apps/plugin-dialog";
import "./App.css";

interface TrackMeta {
  id: string;
  file_path: string;
  artist: string;
  album: string;
  title: string;
  track_number: number;
  duration: number | null;
  status: "pending" | "converting" | "done" | "error";
  error?: string;
}

interface InstalledTrack {
  dev_name: string;
  title: string;
  artist: string;
  track: string;
  start: number;
}

function fmt_dur(s: number | null): string {
  if (s === null) return "--:--";
  const m = Math.floor(s / 60);
  const sec = Math.floor(s % 60);
  return `${m}:${sec.toString().padStart(2, "0")}`;
}

function transliterate(s: string): string {
  const map: Record<string, string> = {
    а: "a", б: "b", в: "v", г: "g", д: "d", е: "e", ё: "yo", ж: "zh", з: "z", и: "i", й: "y",
    к: "k", л: "l", м: "m", н: "n", о: "o", п: "p", р: "r", с: "s", т: "t", у: "u", ф: "f",
    х: "kh", ц: "ts", ч: "ch", ш: "sh", щ: "sch", ъ: "", ы: "y", ь: "", э: "e", ю: "yu", я: "ya",
    А: "A", Б: "B", В: "V", Г: "G", Д: "D", Е: "E", Ё: "Yo", Ж: "Zh", З: "Z", И: "I", Й: "Y",
    К: "K", Л: "L", М: "M", Н: "N", О: "O", П: "P", Р: "R", С: "S", Т: "T", У: "U", Ф: "F",
    Х: "Kh", Ц: "Ts", Ч: "Ch", Ш: "Sh", Щ: "Sch", Ъ: "", Ы: "Y", Ь: "", Э: "E", Ю: "Yu", Я: "Ya",
  };
  return s.split("").map(c => map[c] ?? c).join("");
}

function make_track_id(artist: string, album: string, num: number, title: string): string {
  const up = (s: string) => transliterate(s).toUpperCase().replace(/[^A-Z0-9]+/g, "_").replace(/^_|_$/g, "");
  return `music_${up(artist)}___${up(album)}___${num.toString().padStart(2, "0")}_${up(title)}`;
}

function make_dev_name(artist: string, num: number, title: string): string {
  const lo = (s: string) => transliterate(s).toLowerCase().replace(/[^a-z0-9]+/g, "_").replace(/^_|_$/g, "");
  return `${lo(artist)}_${num.toString().padStart(2, "0")}_${lo(title)}`;
}

export default function App() {
  const [gamePath, setGamePath] = useState("");
  const [queue, setQueue] = useState<TrackMeta[]>([]);
  const [installed, setInstalled] = useState<InstalledTrack[]>([]);
  const [selQueue, setSelQueue] = useState<string | null>(null);
  const [selInstalled, setSelInstalled] = useState<string | null>(null);
  const [log, setLog] = useState<string[]>([]);
  const [busy, setBusy] = useState(false);
  const [dragOver, setDragOver] = useState(false);
  const logRef = useRef<HTMLDivElement>(null);

  const addLog = useCallback((msg: string) => {
    setLog(prev => [...prev.slice(-99), msg]);
  }, []);

  useEffect(() => {
    if (logRef.current) logRef.current.scrollTop = logRef.current.scrollHeight;
  }, [log]);

  useEffect(() => {
    const unlisten = listen<{ paths: string[] }>("tauri://drag-drop", event => {
      const audio = event.payload.paths.filter(p =>
        /\.(mp3|ogg|wav|flac|aac|m4a)$/i.test(p)
      );
      for (const p of audio) addFile(p);
    });
    return () => { unlisten.then(f => f()); };
  }, []);

  useEffect(() => {
    invoke("check_deps").catch(e => addLog(`warning: ${e}`));
  }, []);

  async function pickGameFolder() {
    const sel = await open({ directory: true, title: "Select Keep Driving folder" });
    if (!sel) return;
    const p = sel as string;
    setGamePath(p);
    addLog(`game folder: ${p}`);
    loadInstalled(p);
  }

  async function loadInstalled(path: string) {
    try {
      await invoke("backup_game_files", { gamePath: path });
      const tracks = await invoke<InstalledTrack[]>("read_music_kdr", { gamePath: path });
      setInstalled(tracks);
      addLog(`loaded ${tracks.length} tracks from music.kdr`);
    } catch (e) {
      addLog(`error: ${e}`);
    }
  }

  async function addFile(filePath: string) {
    try {
      const meta = await invoke<{ artist: string; album: string; title: string; track_number: number; duration: number | null }>(
        "read_audio_meta", { filePath }
      );
      const t: TrackMeta = {
        id: crypto.randomUUID(),
        file_path: filePath,
        artist: meta.artist || "Unknown Artist",
        album: meta.album || "Unknown Album",
        title: meta.title || filePath.split(/[\\/]/).pop()?.replace(/\.[^.]+$/, "") || "Unknown",
        track_number: meta.track_number || 1,
        duration: meta.duration,
        status: "pending",
      };
      setQueue(prev => [...prev, t]);
      addLog(`added: ${t.artist} — ${t.title}`);
    } catch (e) {
      addLog(`failed to read file: ${e}`);
    }
  }

  async function browseFiles() {
    const sel = await open({
      multiple: true,
      filters: [{ name: "Audio", extensions: ["mp3", "ogg", "wav", "flac", "aac", "m4a"] }],
    });
    if (!sel) return;
    const paths = Array.isArray(sel) ? sel : [sel];
    for (const p of paths) await addFile(p);
  }

  function onDrop(e: React.DragEvent) {
    e.preventDefault();
    setDragOver(false);
    // handled by tauri event listener
  }

  async function installAll() {
    if (!gamePath) { addLog("select game folder first"); return; }
    const pending = queue.filter(t => t.status === "pending");
    if (!pending.length) { addLog("nothing to install"); return; }
    setBusy(true);
    await new Promise(r => setTimeout(r, 50));

    for (const t of pending) {
      setQueue(prev => prev.map(x => x.id === t.id ? { ...x, status: "converting" } : x));
      addLog(`installing: ${t.artist} — ${t.title}`);
      try {
        await invoke("install_track", {
          gamePath,
          filePath: t.file_path,
          trackId: make_track_id(t.artist, t.album, t.track_number, t.title),
          devName: make_dev_name(t.artist, t.track_number, t.title),
          title: t.title,
          artist: t.artist,
        });
        setQueue(prev => prev.map(x => x.id === t.id ? { ...x, status: "done" } : x));
        addLog(`done: ${t.title}`);
      } catch (e) {
        setQueue(prev => prev.map(x => x.id === t.id ? { ...x, status: "error", error: String(e) } : x));
        addLog(`failed: ${t.title} — ${e}`);
      }
    }

    loadInstalled(gamePath);
    setBusy(false);
  }

  async function removeInstalled(devName: string) {
    if (!gamePath) return;
    try {
      await invoke("remove_track", { gamePath, devName });
      addLog(`removed: ${devName}`);
      loadInstalled(gamePath);
    } catch (e) {
      addLog(`remove failed: ${e}`);
    }
  }

  async function resetMusic() {
    if (!gamePath) return;
    const ok = await confirm("This will restore original data.win and audiogroup3.dat from backup and remove all custom tracks.", { title: "Reset custom music", kind: "warning" });
    if (!ok) return;
    try {
      await invoke("reset_custom_music", { gamePath });
      addLog("reset: custom music removed, original files restored");
      loadInstalled(gamePath);
    } catch (e) {
      addLog(`reset failed: ${e}`);
    }
  }

  const selQueueTrack = queue.find(t => t.id === selQueue);
  const selInstalledTrack = installed.find(t => t.dev_name === selInstalled);
  const [searchQuery, setSearchQuery] = useState("");
  const [queueQuery, setQueueQuery] = useState("");

  const filteredInstalled = installed.filter(t => {
    if (!searchQuery) return true;
    const q = searchQuery.toLowerCase();
    return t.title.toLowerCase().includes(q) || t.artist.toLowerCase().includes(q);
  });
  const filteredQueue = queue.filter(t => {
    if (!queueQuery) return true;
    const q = queueQuery.toLowerCase();
    return t.title.toLowerCase().includes(q) || t.artist.toLowerCase().includes(q);
  });

  return (
    <div className="app">
      <header className="header">
        <div className="logo">
          <span className="logo-name">KD<span>MUSIC</span>TOOL</span>
          <span className="logo-tag">// be on your wave</span>
        </div>
        <div className="path-row">
          <span className="path-label">GAME</span>
          <div className={`path-val ${!gamePath ? "empty" : ""}`} onClick={pickGameFolder}>
            {gamePath || "click to select Keep Driving folder"}
          </div>
          <button className="btn btn-sm" onClick={pickGameFolder}>BROWSE</button>
        </div>
      </header>

      <main className="panels">
        {/* queue */}
        <section className="panel">
          <div className="panel-head">
            <span className="panel-title">QUEUE</span>
            <span className="panel-count">{queue.length} tracks</span>
            <div className="search-wrap">
              <input className="search-input" placeholder="search..."
                value={queueQuery} onChange={e => setQueueQuery(e.target.value)} />
              {queueQuery && <button className="search-clear" onClick={() => setQueueQuery("")}>×</button>}
            </div>
            <button className="btn btn-sm" onClick={browseFiles}>+ ADD</button>
            {queue.some(t => t.status === "done") && (
              <button className="btn btn-sm btn-danger"
                onClick={() => setQueue(q => q.filter(t => t.status !== "done"))}>
                CLEAR DONE
              </button>
            )}
          </div>

          <div
            className={`dropzone ${dragOver ? "over" : ""}`}
            onDrop={onDrop}
            onDragOver={e => { e.preventDefault(); setDragOver(true); }}
            onDragLeave={() => setDragOver(false)}
          >
            {queue.length === 0 ? (
              <div className="drop-hint">
                <span className="drop-icon">⊕</span>
                <span>drop audio files here</span>
                <span className="drop-sub">mp3 · ogg · wav · flac · aac</span>
              </div>
            ) : (
              <div className="track-list">
                {filteredQueue.map(t => (
                  <div key={t.id}
                    className={`track-row st-${t.status} ${selQueue === t.id ? "active" : ""}`}
                    onClick={() => setSelQueue(t.id === selQueue ? null : t.id)}
                  >
                    <div className="tr-num">{t.track_number.toString().padStart(2, "0")}</div>
                    <div className="tr-info">
                      <div className="tr-title">{t.title}</div>
                      <div className="tr-sub">{t.artist} · {t.album}</div>
                    </div>
                    <div className="tr-dur">{fmt_dur(t.duration)}</div>
                    <div className="status-dot">
                      {t.status === "pending" && <span className="sd-pending">●</span>}
                      {t.status === "converting" && <span className="sd-converting">◌</span>}
                      {t.status === "done" && <span className="sd-done">✓</span>}
                      {t.status === "error" && <span className="sd-error" title={t.error}>✗</span>}
                    </div>
                    <button className="btn-x" onClick={e => { e.stopPropagation(); setQueue(q => q.filter(x => x.id !== t.id)); }}>×</button>
                  </div>
                ))}
              </div>
            )}
          </div>

          {selQueueTrack && (
            <div className="detail">
              <div className="detail-row">
                <span className="dk">ARTIST</span>
                <input className="detail-input" value={selQueueTrack.artist}
                  onChange={e => setQueue(q => q.map(t => t.id === selQueueTrack.id ? { ...t, artist: e.target.value } : t))} />
              </div>
              <div className="detail-row">
                <span className="dk">ALBUM</span>
                <input className="detail-input" value={selQueueTrack.album}
                  onChange={e => setQueue(q => q.map(t => t.id === selQueueTrack.id ? { ...t, album: e.target.value } : t))} />
              </div>
              <div className="detail-row">
                <span className="dk">TITLE</span>
                <input className="detail-input" value={selQueueTrack.title}
                  onChange={e => setQueue(q => q.map(t => t.id === selQueueTrack.id ? { ...t, title: e.target.value } : t))} />
              </div>
              <div className="detail-row">
                <span className="dk">TRACK#</span>
                <input className="detail-input" style={{ width: 40 }} type="number" min={1} max={99}
                  value={selQueueTrack.track_number}
                  onChange={e => setQueue(q => q.map(t => t.id === selQueueTrack.id ? { ...t, track_number: parseInt(e.target.value) || 1 } : t))} />
              </div>
              <div className="detail-row">
                <span className="dk">ID</span>
                <span className="dv mono">{make_track_id(selQueueTrack.artist, selQueueTrack.album, selQueueTrack.track_number, selQueueTrack.title)}</span>
              </div>
            </div>
          )}
        </section>

        {/* install button */}
        <div className="mid-col">
          <div className="mid-line" />
          <button
            className={`btn-install ${busy ? "busy" : ""}`}
            onClick={installAll}
            disabled={busy || !gamePath || !queue.some(t => t.status === "pending")}
          >
            {busy ? <span className="spin">◌</span> : <>▶ GO</>}
          </button>
          <div className="mid-line" />
        </div>

        {/* installed */}
        <section className="panel">
          <div className="panel-head">
            <span className="panel-title">IN GAME</span>
            <span className="panel-count">{installed.length} tracks</span>
            <div className="search-wrap">
              <input className="search-input" placeholder="search..."
                value={searchQuery} onChange={e => setSearchQuery(e.target.value)} />
              {searchQuery && <button className="search-clear" onClick={() => setSearchQuery("")}>×</button>}
            </div>
            <button className="btn btn-sm btn-danger" onClick={resetMusic}>RESET</button>
          </div>

          <div className="track-list" style={{ flex: 1, overflowY: "auto" }}>
            {installed.length === 0 ? (
              <div className="empty">
                <span>no tracks loaded</span>
                <span className="sub">select game folder first</span>
              </div>
            ) : filteredInstalled.map(t => (
              <div key={t.dev_name}
                className={`track-row ${selInstalled === t.dev_name ? "active" : ""}`}
                onClick={() => setSelInstalled(t.dev_name === selInstalled ? null : t.dev_name)}
              >
                <div className="tr-info">
                  <div className="tr-title">{t.title}</div>
                  <div className="tr-sub">{t.artist}</div>
                </div>
                <span className={`badge ${t.start === 1 ? "badge-radio" : "badge-cd"}`}>
                  {t.start === 1 ? "RADIO" : "CD"}
                </span>
                <button className="btn-x" onClick={e => { e.stopPropagation(); removeInstalled(t.dev_name); }}>×</button>
              </div>
            ))}
          </div>

          {selInstalledTrack && (
            <div className="detail">
              <div className="detail-row"><span className="dk">TITLE</span><span className="dv">{selInstalledTrack.title}</span></div>
              <div className="detail-row"><span className="dk">ARTIST</span><span className="dv">{selInstalledTrack.artist}</span></div>
              <div className="detail-row"><span className="dk">TRACK</span><span className="dv mono">{selInstalledTrack.track}</span></div>
            </div>
          )}
        </section>
      </main>

      <div className="log" ref={logRef}>
        {log.length === 0
          ? <div className="log-line muted">// ready</div>
          : log.map((l, i) => <div key={i} className="log-line">{l}</div>)
        }
      </div>
    </div>
  );
}
