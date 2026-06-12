import type { SimResult } from '../types';
import { Card } from '../game_assets';
import type { CardColor } from '../game_assets';
import styles from './Visualization.module.css';

const COLORS: CardColor[] = ['Blue', 'Green', 'Red', 'Yellow'];
const VALUES = [1, 2, 3, 4, 5, 6, 7];

interface Props {
  result: SimResult;
  onClose: () => void;
}

export function Visualization({ onClose }: Props) {
  return (
    <div className={styles.overlay} onClick={onClose}>
      <div className={styles.modal} onClick={e => e.stopPropagation()}>
        <div className={styles.header}>
          <h2 className={styles.title}>Cards</h2>
          <button className={styles.closeBtn} onClick={onClose} aria-label="Close">✕</button>
        </div>

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
      </div>
    </div>
  );
}
