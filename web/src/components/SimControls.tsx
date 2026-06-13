import { useState } from 'react';
import type { RunConfig, Strategy } from '../types';
import styles from './SimControls.module.css';

interface Props {
  ready: boolean;
  running: boolean;
  onRun: (config: RunConfig) => void;
}

function randomSeed(): number {
  return crypto.getRandomValues(new Uint32Array(1))[0];
}

export function SimControls({ ready, running, onRun }: Props) {
  const [strategy, setStrategy] = useState<Strategy>('random');
  const [seqDepth, setSeqDepth] = useState(3);
  const [batchSize, setBatchSize] = useState(20);
  const [seed, setSeed] = useState(randomSeed);
  const [trace, setTrace] = useState(false);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onRun({ strategy, seq_depth: seqDepth, batch_size: batchSize, seed, trace });
  };

  return (
    <form className={styles.form} onSubmit={handleSubmit}>
      <div className={styles.groups}>
        <fieldset className={styles.group}>
          <legend>Strategy</legend>
          <label className={styles.radio}>
            <input
              type="radio" name="strategy" value="random"
              checked={strategy === 'random'}
              onChange={() => setStrategy('random')}
            />
            Random
          </label>
          <label className={styles.radio}>
            <input
              type="radio" name="strategy" value="optimistic"
              checked={strategy === 'optimistic'}
              onChange={() => setStrategy('optimistic')}
            />
            Optimistic
          </label>
          {strategy === 'optimistic' && (
            <label className={styles.field}>
              <span>Search depth</span>
              <input
                type="number" min={1} max={6} value={seqDepth}
                onChange={e => setSeqDepth(Number(e.target.value))}
                className={styles.numInput}
              />
              {seqDepth >= 4 && <span className={styles.warn}>⚠ slow</span>}
            </label>
          )}
        </fieldset>

        <fieldset className={styles.group}>
          <legend>Parameters</legend>
          {!trace && (
            <label className={styles.field}>
              <span>Batch size</span>
              <input
                type="number" min={1} max={1000} value={batchSize}
                onChange={e => setBatchSize(Number(e.target.value))}
                className={styles.numInput}
              />
            </label>
          )}
          <label className={styles.field}>
            <span>Seed</span>
            <input
              type="number" min={0} max={4294967295} value={seed}
              onChange={e => setSeed(Number(e.target.value))}
              className={styles.seedInput}
            />
            <button
              type="button"
              className={styles.iconBtn}
              title="Randomize seed"
              onClick={() => setSeed(randomSeed())}
            >↺</button>
          </label>
          <label className={styles.radio}>
            <input
              type="checkbox"
              checked={trace}
              onChange={e => setTrace(e.target.checked)}
            />
            Trace mode
          </label>
        </fieldset>
      </div>

      <button className={styles.runBtn} type="submit" disabled={!ready || running}>
        {!ready ? 'Loading…' : running ? 'Running…' : 'Run'}
      </button>
    </form>
  );
}
