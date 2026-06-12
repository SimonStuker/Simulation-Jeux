import { useState } from 'react';
import type { SimResult } from '../types';
import { Card, MissionCard, MISSION_FAMILIES } from '../game_assets';
import type { CardColor } from '../game_assets';
import styles from './Visualization.module.css';

const COLORS: CardColor[] = ['Blue', 'Green', 'Red', 'Yellow'];
const VALUES = [1, 2, 3, 4, 5, 6, 7];

type Tab = 'cards' | 'missions';

interface Props {
  result: SimResult;
  onClose: () => void;
}

export function Visualization({ onClose }: Props) {
  const [tab, setTab] = useState<Tab>('cards');

  return (
    <div className={styles.overlay} onClick={onClose}>
      <div className={styles.modal} onClick={e => e.stopPropagation()}>
        <div className={styles.header}>
          <div className={styles.tabs}>
            <button
              className={`${styles.tab} ${tab === 'cards' ? styles.tabActive : ''}`}
              onClick={() => setTab('cards')}
            >Cards</button>
            <button
              className={`${styles.tab} ${tab === 'missions' ? styles.tabActive : ''}`}
              onClick={() => setTab('missions')}
            >Missions</button>
          </div>
          <button className={styles.closeBtn} onClick={onClose} aria-label="Close">✕</button>
        </div>

        {tab === 'cards' && (
          <div className={styles.grid}>
            <div className={styles.colLabels}>
              <div className={styles.rowLabelSpacer} />
              {VALUES.map(v => (
                <span key={v} className={styles.colLabel}>{v}</span>
              ))}
            </div>
            {COLORS.map(color => (
              <div key={color} className={styles.row}>
                <span className={styles.rowLabel} data-color={color}>{color}</span>
                {VALUES.map(value => (
                  <Card key={value} color={color} value={value} size="md" />
                ))}
              </div>
            ))}
          </div>
        )}

        {tab === 'missions' && (
          <div className={styles.missionsScroll}>
            {MISSION_FAMILIES.map(family => (
              <div key={family.label} className={styles.family}>
                <span className={styles.familyLabel}>{family.label}</span>
                <div className={styles.familyRow}>
                  {family.missions.map(name => (
                    <MissionCard key={name} name={name} showLabel />
                  ))}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
