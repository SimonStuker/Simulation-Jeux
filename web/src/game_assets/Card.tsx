import styles from './Card.module.css';

export type CardColor = 'Red' | 'Green' | 'Yellow' | 'Blue';

interface Props {
  value: number;
  color: CardColor;
  size?: 'sm' | 'md' | 'lg';
}

const COLOR_HEX: Record<CardColor, string> = {
  Red:    '#e94560',
  Green:  '#34d399',
  Yellow: '#fbbf24',
  Blue:   '#60a5fa',
};

export function Card({ value, color, size = 'md' }: Props) {
  const hex = COLOR_HEX[color];
  return (
    <div
      className={`${styles.card} ${styles[size]}`}
      style={{ '--c': hex } as React.CSSProperties}
      title={`${color} ${value}`}
    >
      <span className={`${styles.corner} ${styles.topLeft}`}>{value}</span>
      <span className={styles.center}>{value}</span>
      <span className={`${styles.corner} ${styles.bottomRight}`}>{value}</span>
    </div>
  );
}
