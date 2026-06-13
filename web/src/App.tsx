import { useState, useEffect, useCallback } from 'react';
import init, { launch_single, launch_single_trace } from '../pkg/jeu_50missions.js';
import wasmUrl from '../pkg/jeu_50missions_bg.wasm?url';
import type { SimResult, SimTrace, RunConfig } from './types';
import { SimControls } from './components/SimControls';
import { ResultsPanel } from './components/ResultsPanel';
import { TraceViewer } from './components/TraceViewer';
import { Visualization } from './components/Visualization';
import styles from './App.module.css';

const BATCH_CHUNK = 10;

export default function App() {
  const [ready, setReady]         = useState(false);
  const [running, setRunning]     = useState(false);
  const [results, setResults]     = useState<SimResult[] | null>(null);
  const [trace, setTrace]         = useState<SimTrace | null>(null);
  const [lastConfig, setLastConfig] = useState<RunConfig | null>(null);
  const [showViz, setShowViz]     = useState(false);
  const [toast, setToast]         = useState<string | null>(null);

  useEffect(() => {
    init(wasmUrl).then(() => setReady(true));
  }, []);

  const showToast = useCallback((msg: string) => {
    setToast(msg);
    setTimeout(() => setToast(null), 2500);
  }, []);

  const handleRun = useCallback((config: RunConfig) => {
    setRunning(true);
    setLastConfig(config);
    setTrace(null);

    if (config.trace) {
      setResults(null);
      setTimeout(() => {
        const t = launch_single_trace(
          config.strategy === 'random',
          BigInt(config.seed),
          config.seq_depth,
        ) as SimTrace;
        setTrace(t);
        setRunning(false);
      }, 0);
      return;
    }

    setResults([]);
    let done = 0;

    const runChunk = () => {
      const chunk: SimResult[] = [];
      const end = Math.min(done + BATCH_CHUNK, config.batch_size);
      for (let i = done; i < end; i++) {
        chunk.push(
          launch_single(
            config.strategy === 'random',
            BigInt(config.seed) + BigInt(i),
            config.seq_depth,
          ) as SimResult,
        );
      }
      done = end;
      setResults(prev => [...(prev ?? []), ...chunk]);
      if (done < config.batch_size) {
        setTimeout(runChunk, 0);
      } else {
        setRunning(false);
      }
    };

    setTimeout(runChunk, 0);
  }, []);

  const handleVisualize = useCallback((result: SimResult) => {
    if (!lastConfig) { showToast('Run a simulation first'); return; }
    setRunning(true);
    setTimeout(() => {
      const t = launch_single_trace(
        lastConfig.strategy === 'random',
        BigInt(result.seed),
        lastConfig.seq_depth,
      ) as SimTrace;
      setTrace(t);
      setRunning(false);
    }, 0);
  }, [lastConfig, showToast]);

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

      {(running || results !== null) && (
        <ResultsPanel
          results={results}
          running={running}
          onVisualize={handleVisualize}
        />
      )}

      {trace && <TraceViewer trace={trace} onClose={() => setTrace(null)} />}
      {showViz && <Visualization onClose={() => setShowViz(false)} />}

      {toast && <div className={styles.toast}>{toast}</div>}
    </div>
  );
}
