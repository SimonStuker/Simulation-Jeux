import { useState, useMemo } from 'react';
import type { SimResult } from '../types';
import styles from './ResultsPanel.module.css';

const MAX_MISSIONS = 50;
const PAGE_SIZE = 100;

type SortKey = 'completed_missions' | 'turns';
type SortDir = 'asc' | 'desc';
type OutcomeFilter = 'all' | 'victory' | 'defeat';

interface Props {
  results: SimResult[] | null;
  running: boolean;
  onVisualize: (result: SimResult) => void;
}

function SortIcon({ col, sortBy, dir }: { col: SortKey; sortBy: SortKey; dir: SortDir }) {
  if (col !== sortBy) return <span className={styles.sortIdle}>↕</span>;
  return <span className={styles.sortActive}>{dir === 'asc' ? '↑' : '↓'}</span>;
}

export function ResultsPanel({ results, running, onVisualize }: Props) {
  const [sortBy, setSortBy]                   = useState<SortKey>('completed_missions');
  const [sortDir, setSortDir]                 = useState<SortDir>('asc');
  const [filterOutcome, setFilterOutcome]     = useState<OutcomeFilter>('all');
  const [filterMinMissions, setFilterMin]     = useState('');
  const [page, setPage]                       = useState(0);

  const handleSort = (key: SortKey) => {
    if (sortBy === key) setSortDir(d => d === 'asc' ? 'desc' : 'asc');
    else { setSortBy(key); setSortDir('asc'); }
    setPage(0);
  };

  const filtered = useMemo(() => {
    if (!results) return [];
    let data: SimResult[] = results;
    if (filterOutcome !== 'all')
      data = data.filter(r => filterOutcome === 'victory' ? r.victory : !r.victory);
    const minM = Number(filterMinMissions);
    if (filterMinMissions !== '' && !isNaN(minM))
      data = data.filter(r => r.completed_missions >= minM);
    return data;
  }, [results, filterOutcome, filterMinMissions]);

  const sorted = useMemo(() => {
    const data = [...filtered];
    data.sort((a, b) => {
      const diff = a[sortBy] - b[sortBy];
      return sortDir === 'asc' ? diff : -diff;
    });
    return data;
  }, [filtered, sortBy, sortDir]);

  const totalPages = Math.max(1, Math.ceil(sorted.length / PAGE_SIZE));
  const safePage   = Math.min(page, totalPages - 1);
  const pageStart  = safePage * PAGE_SIZE;
  const visible    = sorted.slice(pageStart, pageStart + PAGE_SIZE);

  // quantile markers keyed by absolute index in the sorted (filtered) array
  const markers = useMemo(() => {
    const n = sorted.length;
    const m: Record<number, string> = {};
    if (n >= 2) {
      m[Math.round(n * 0.10)] = 'p10';
      m[Math.round(n * 0.50)] = 'p50';
      m[Math.round(n * 0.90)] = 'p90';
    }
    return m;
  }, [sorted.length]);

  // stats always by missions, regardless of active sort
  const stats = useMemo(() => {
    const n = sorted.length;
    if (n === 0) return null;
    const victories = sorted.filter(r => r.victory).length;
    const byM = [...sorted].sort((a, b) => a.completed_missions - b.completed_missions);
    return {
      n,
      total: results?.length ?? n,
      victories,
      winPct: Math.round((victories / n) * 100),
      median: byM[Math.floor(n * 0.5)].completed_missions,
      p90:    byM[Math.floor(n * 0.9)].completed_missions,
      best:   byM[n - 1].completed_missions,
      worst:  byM[0].completed_missions,
    };
  }, [sorted, results]);

  if (!results) return <div className={styles.loading}>Running…</div>;

  return (
    <section className={styles.panel}>
      {stats && (
        <div className={styles.stats}>
          <Stat
            label={running ? 'Games…' : 'Games'}
            value={stats.n < stats.total ? `${stats.n} / ${stats.total}` : stats.n}
          />
          <Stat label="Victories"    value={`${stats.victories} (${stats.winPct}%)`} highlight={stats.winPct > 50} />
          <Stat label="Median"       value={`${stats.median} / ${MAX_MISSIONS}`} />
          <Stat label="p90"          value={`${stats.p90} / ${MAX_MISSIONS}`} />
          <Stat label="Best"         value={stats.best} />
          <Stat label="Worst"        value={stats.worst} />
        </div>
      )}

      <div className={styles.tableWrap}>
        <table className={styles.table}>
          <thead>
            <tr>
              <th className={styles.colRank}>#</th>
              <th className={styles.colSeed}>Seed</th>

              <th className={styles.colOutcome}>
                <div className={styles.thStack}>
                  <span>Outcome</span>
                  <select
                    className={styles.filterSelect}
                    value={filterOutcome}
                    onChange={e => { setFilterOutcome(e.target.value as OutcomeFilter); setPage(0); }}
                  >
                    <option value="all">All</option>
                    <option value="victory">Victory</option>
                    <option value="defeat">Defeat</option>
                  </select>
                </div>
              </th>

              <th className={styles.colMissions}>
                <div className={styles.thStack}>
                  <button className={styles.sortBtn} onClick={() => handleSort('completed_missions')}>
                    Missions <SortIcon col="completed_missions" sortBy={sortBy} dir={sortDir} />
                  </button>
                  <input
                    className={styles.filterInput}
                    type="number"
                    placeholder="≥ min"
                    min={0}
                    max={MAX_MISSIONS}
                    value={filterMinMissions}
                    onChange={e => { setFilterMin(e.target.value); setPage(0); }}
                  />
                </div>
              </th>

              <th className={styles.colTurns}>
                <button className={styles.sortBtn} onClick={() => handleSort('turns')}>
                  Turns <SortIcon col="turns" sortBy={sortBy} dir={sortDir} />
                </button>
              </th>

              <th className={styles.colAction} />
            </tr>
          </thead>

          <tbody>
            {visible.map((r, i) => {
              const absIdx = pageStart + i;
              return (
                <tr key={r.seed} className={r.victory ? styles.rowWin : styles.rowLose}>
                  <td className={styles.colRank}>
                    <span className={styles.rank}>{absIdx + 1}</span>
                    {markers[absIdx] && <span className={styles.pctile}>{markers[absIdx]}</span>}
                  </td>
                  <td className={styles.colSeed}>
                    <span className={styles.seedVal}>{r.seed}</span>
                  </td>
                  <td className={styles.colOutcome}>
                    <span className={r.victory ? styles.badgeWin : styles.badgeLose}>
                      {r.victory ? 'Victory' : 'Defeat'}
                    </span>
                  </td>
                  <td className={styles.colMissions}>
                    <div className={styles.missionCell}>
                      <span className={styles.missionCount}>
                        {r.completed_missions}
                        <span className={styles.missionMax}> / {MAX_MISSIONS}</span>
                      </span>
                      <div className={styles.bar}>
                        <div
                          className={styles.barFill}
                          style={{ width: `${Math.min((r.completed_missions / MAX_MISSIONS) * 100, 100)}%` }}
                        />
                      </div>
                    </div>
                  </td>
                  <td className={styles.colTurns}>{r.turns}</td>
                  <td className={styles.colAction}>
                    <button className={styles.vizBtn} onClick={() => onVisualize(r)}>
                      Visualize
                    </button>
                  </td>
                </tr>
              );
            })}

            {visible.length === 0 && (
              <tr>
                <td colSpan={6} className={styles.emptyRow}>No results match the current filters.</td>
              </tr>
            )}
          </tbody>
        </table>
      </div>

      {totalPages > 1 && (
        <div className={styles.pagination}>
          <button className={styles.pageBtn} disabled={safePage === 0} onClick={() => setPage(p => p - 1)}>
            ← Prev
          </button>
          <span className={styles.pageInfo}>
            {pageStart + 1}–{Math.min(pageStart + PAGE_SIZE, sorted.length)} of {sorted.length}
          </span>
          <button className={styles.pageBtn} disabled={safePage >= totalPages - 1} onClick={() => setPage(p => p + 1)}>
            Next →
          </button>
        </div>
      )}
    </section>
  );
}

function Stat({ label, value, highlight }: { label: string; value: string | number; highlight?: boolean }) {
  return (
    <div className={styles.stat}>
      <span className={styles.statLabel}>{label}</span>
      <span className={`${styles.statValue} ${highlight ? styles.statHighlight : ''}`}>{value}</span>
    </div>
  );
}
