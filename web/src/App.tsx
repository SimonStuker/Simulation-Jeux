import { useState, useEffect, useCallback } from 'react';
import init, { run_batch_wasm } from '../pkg/jeu_50missions.js';
import wasmUrl from '../pkg/jeu_50missions_bg.wasm?url';
import type { SimResult, RunConfig } from './types';
import { SimControls } from './components/SimControls';
import { ResultsPanel } from './components/ResultsPanel';
import { Visualization } from './components/Visualization';
import styles from './App.module.css';

export default function App() {
  const [ready, setReady]     = useState(false);
  const [running, setRunning] = useState(false);
  const [results, setResults] = useState<SimResult[] | null>(null);
  const [showViz, setShowViz] = useState(false);
  const [toast, setToast]     = useState<string | null>(null);

  useEffect(() => {
    init(wasmUrl).then(() => setReady(true));
  }, []);

  const handleRun = useCallback((config: RunConfig) => {
    setRunning(true);
    setResults(null);
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
        <button className={styles.vizMenuBtn} onClick={() => setShowViz(true)}>
          Cards &amp; Missions
        </button>
      </header>

      <SimControls ready={ready} running={running} onRun={handleRun} />

      {(running || results) && (
        <ResultsPanel
          results={results}
          running={running}
          onVisualize={() => showToast('Under construction 🚧')}
        />
      )}

      {showViz && <Visualization onClose={() => setShowViz(false)} />}

      {toast && <div className={styles.toast}>{toast}</div>}
    </div>
  );
}
