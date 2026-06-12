import { useState, useEffect } from 'react';
import init, { run_simulation_wasm } from '../pkg/jeu_50missions.js';
import wasmUrl from '../pkg/jeu_50missions_bg.wasm?url';
import styles from './App.module.css';

function App() {
  const [ready, setReady] = useState(false);
  const [result, setResult] = useState<number | null>(null);

  useEffect(() => {
    init(wasmUrl).then(() => setReady(true));
  }, []);

  const handleRun = () => {
    const seed = Date.now() >>> 0; // truncate to u32
    setResult(run_simulation_wasm(seed));
  };

  return (
    <main className={styles.main}>
      <h1>50 Missions</h1>
      <button className={styles.btn} onClick={handleRun} disabled={!ready}>
        {ready ? 'Run Simulation' : 'Loading…'}
      </button>
      {result !== null && (
        <p className={styles.result}>
          Completed missions: <strong>{result}</strong>
        </p>
      )}
    </main>
  );
}

export default App;
