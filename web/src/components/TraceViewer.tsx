import { useState, useEffect } from 'react';
import type { SimTrace } from '../types';
import { Card } from '../game_assets';
import type { CardColor } from '../game_assets';
import styles from './TraceViewer.module.css';

interface Props {
  trace: SimTrace;
  onClose: () => void;
}

export function TraceViewer({ trace, onClose }: Props) {
  const [step, setStep] = useState(0);
  const total = trace.state_list.length;
  const state = trace.state_list[step];
  const prevMove = step > 0 ? trace.move_list[step - 1] : null;
  const currentPlayerIdx = state.current_player === 'Player0' ? 0 : 1;

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === 'ArrowLeft')  setStep(s => Math.max(0, s - 1));
      if (e.key === 'ArrowRight') setStep(s => Math.min(total - 1, s + 1));
      if (e.key === 'Escape')     onClose();
    };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, [total, onClose]);

  return (
    <div className={styles.overlay} onClick={onClose}>
      <div className={styles.modal} onClick={e => e.stopPropagation()}>

        <div className={styles.header}>
          <div className={styles.stepInfo}>
            <span className={styles.turnLabel}>Turn {state.turn}</span>
            <span className={styles.stepCounter}>{step + 1} / {total}</span>
            {state.final_sprint && <span className={styles.sprint}>Final Sprint</span>}
          </div>
          <div className={styles.nav}>
            <button className={styles.navBtn} disabled={step === 0}
              onClick={() => setStep(s => s - 1)}>←</button>
            <button className={styles.navBtn} disabled={step === total - 1}
              onClick={() => setStep(s => s + 1)}>→</button>
          </div>
          <button className={styles.closeBtn} onClick={onClose}>✕</button>
        </div>

        <div className={styles.board}>

          <div className={styles.hands}>
            {[0, 1].map(p => (
              <div key={p} className={`${styles.hand} ${p === currentPlayerIdx ? styles.handActive : ''}`}>
                <div className={styles.sectionLabel}>
                  Player {p}{p === currentPlayerIdx ? ' — playing' : ''}
                </div>
                <div className={styles.cardRow}>
                  {state.player_hands[p].map((card, i) => (
                    <Card key={i} color={card.color as CardColor} value={card.value} size="sm" />
                  ))}
                </div>
              </div>
            ))}
          </div>

          <div className={styles.tableSection}>
            <div className={styles.sectionLabel}>
              Table
              {prevMove && (
                <span className={styles.moveHint}>
                  — played to position {prevMove.idx_table + 1}
                </span>
              )}
            </div>
            <div className={styles.cardRow}>
              {state.table_cards.map((card, i) => (
                <div key={i} className={prevMove?.idx_table === i ? styles.highlight : styles.cardSlot}>
                  <Card color={card.color as CardColor} value={card.value} size="md" />
                </div>
              ))}
            </div>
          </div>

          <div className={styles.missionsSection}>
            <div className={styles.sectionLabel}>Active missions ({state.table_missions.length})</div>
            <div className={styles.missionNames}>
              {state.table_missions.map((name, i) => (
                <span key={i} className={styles.missionName}>{name}</span>
              ))}
            </div>
          </div>

          <div className={styles.footer}>
            <div className={styles.progress}>
              <span className={styles.progressLabel}>Completed</span>
              <span className={styles.progressValue}>{state.completed_missions} / 50</span>
              <div className={styles.bar}>
                <div className={styles.barFill} style={{ width: `${(state.completed_missions / 50) * 100}%` }} />
              </div>
            </div>
            <div className={styles.deckInfo}>
              <span>Cards left: {state.deck_cards.length}</span>
              <span>Missions left: {state.deck_missions.length}</span>
            </div>
          </div>

        </div>
      </div>
    </div>
  );
}
