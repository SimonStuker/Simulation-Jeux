import { useState, useEffect, useCallback } from 'react';
import init, { run_batch_wasm } from '../pkg/jeu_50missions.js';
import wasmUrl from '../pkg/jeu_50missions_bg.wasm?url';
import type { SimResult, RunConfig } from './types';
import { SimControls } from './components/SimControls';
import { ResultsPanel } from './components/ResultsPanel';
import styles from './App.module.css';

export default function App() {
  const [ready, setReady] = useState(false);
  const [running, setRunning] = useState(false);
  const [results, setResults] = useState<SimResult[] | null>(null);
  const [toast, setToast] = useState<string | null>(null);

  useEffect(() => {
    init(wasmUrl).then(() => setReady(true));
  }, []);

  const handleRun = useCallback((config: RunConfig) => {
    setRunning(true);
    setResults(null);
    // yield to React so the "Running…" state renders before we block the thread
    setTimeout(() => {
      const raw = run_batch_wasm(config.batch_size, config.seed, config.strategy === 'random', config.seq_depth) as SimResult[];
      setResults(raw);
      setRunning(false);
    }, 0);
  }, []);

  const showToast = useCallback((msg: string) => {
    setToast(msg);
    setTimeout(() => setToast(null), 2500);
  }, []);

  return (
    <div className={styles.layout}>
      <header className={styles.header}>
        <h1>50 Missions</h1>
        <p className={styles.subtitle}>Game simulation</p>
      </header>

      <SimControls ready={ready} running={running} onRun={handleRun} />

      {(running || results) && (
        <ResultsPanel
          results={results}
          running={running}
          onVisualize={() => showToast('Under construction 🚧')}
        />
      )}

      {toast && <div className={styles.toast}>{toast}</div>}
    </div>
  );
}
