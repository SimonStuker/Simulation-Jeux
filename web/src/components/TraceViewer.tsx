import { useState, useEffect, useCallback } from 'react';
import type { SimTrace } from '../types';
import { Card, MissionCard } from '../game_assets';
import type { CardColor } from '../game_assets';
import styles from './TraceViewer.module.css';

function PersonIcon() {
  return (
    <svg width="11" height="11" viewBox="0 0 16 16" fill="currentColor" style={{ flexShrink: 0 }}>
      <circle cx="8" cy="4.5" r="3" />
      <path d="M2 15c0-3.314 2.686-6 6-6s6 2.686 6 6" />
    </svg>
  );
}

function TableIcon() {
  return (
    <svg width="11" height="11" viewBox="0 0 16 16" fill="currentColor" style={{ flexShrink: 0 }}>
      <rect x="1" y="4.5" width="14" height="2.5" rx="1" />
      <rect x="3" y="7" width="2" height="6" rx="0.5" />
      <rect x="11" y="7" width="2" height="6" rx="0.5" />
    </svg>
  );
}

function StarIcon() {
  return (
    <svg width="11" height="11" viewBox="0 0 16 16" fill="currentColor" style={{ flexShrink: 0 }}>
      <path d="M8 1l1.76 3.9 4.24.62-3.06 3 .72 4.23L8 10.5l-3.66 2.25.72-4.23L2 5.52l4.24-.62L8 1z" />
    </svg>
  );
}

interface Props {
  trace: SimTrace;
  onClose: () => void;
}

export function TraceViewer({ trace, onClose }: Props) {
  const [step, setStep] = useState(0);

  const totalSteps = trace.state_list.length;
  const state = trace.state_list[step];
  const move = trace.move_list[step];
  const playerIdx = state.current_player === 'Player0' ? 0 : 1;

  const goBack = useCallback(() => setStep(s => Math.max(0, s - 1)), []);
  const goForward = useCallback(() => setStep(s => Math.min(totalSteps - 1, s + 1)), [totalSteps]);

  useEffect(() => { setStep(0); }, [trace]);

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onClose();
      else if (e.key === 'ArrowLeft') goBack();
      else if (e.key === 'ArrowRight') goForward();
    };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, [onClose, goBack, goForward]);

  return (
    <div className={styles.overlay} onClick={onClose}>
      <div className={styles.modal} onClick={e => e.stopPropagation()}>

        <div className={styles.header}>
          <div className={styles.stepInfo}>
            <span className={styles.turnLabel}>Seed {trace.seed}</span>
            <span className={styles.stepCounter}>turn {state.turn} · {step + 1}/{totalSteps}</span>
            {state.final_sprint && <span className={styles.sprint}>Final Sprint</span>}
          </div>
          <div className={styles.nav}>
            <button className={styles.navBtn} disabled={step === 0} onClick={goBack}>←</button>
            <button className={styles.navBtn} disabled={step === totalSteps - 1} onClick={goForward}>→</button>
          </div>
          <button className={styles.closeBtn} onClick={onClose}>✕</button>
        </div>

        <div className={styles.board}>
          <div className={styles.missionsSection}>
            <div className={styles.sectionLabel}>
              <StarIcon /> Missions on table
              <span className={styles.moveHint}>
                {state.completed_missions} completed · {state.deck_missions.length} in deck
              </span>
            </div>
            <div className={styles.missionCards}>
              {state.table_missions.map((name, i) => (
                <MissionCard key={i} name={name} />
              ))}
            </div>
          </div>

          <div className={styles.tableSection}>
            <div className={styles.sectionLabel}>
              <TableIcon /> Table
              {move && <span className={styles.moveHint}>→ slot {move.idx_table + 1}</span>}
            </div>
            <div className={styles.cardRowCentered}>
              {state.table_cards.map((card, i) => (
                <div key={i} className={(move && move.idx_table === i) ? styles.highlight : styles.cardSlot}>
                  <Card color={card.color as CardColor} value={card.value} size="sm" />
                </div>
              ))}
            </div>
          </div>

          <div className={styles.hands}>
            {[0, 1].map(p => {
              const isActive = p === playerIdx;
              const playedIdx = (move && isActive) ? move.idx_hand : null;
              return (
                <div key={p} className={`${styles.hand} ${isActive ? styles.handActive : ''}`}>
                  <div className={styles.sectionLabel}>
                    <PersonIcon /> Player {p}
                    {isActive && move && (
                      <span className={styles.moveHint}>→ card {playedIdx! + 1}</span>
                    )}
                  </div>
                  <div className={styles.cardRow}>
                    {state.player_hands[p].map((card, i) => (
                      <div key={i} className={playedIdx === i ? styles.highlight : styles.cardSlot}>
                        <Card color={card.color as CardColor} value={card.value} size="sm" />
                      </div>
                    ))}
                  </div>
                </div>
              );
            })}
          </div>

        </div>

      </div>
    </div>
  );
}
