import type { ReactNode } from 'react';
import styles from './MissionCard.module.css';

type Col = 'Red' | 'Green' | 'Yellow' | 'Blue';

const COL: Record<Col, string> = {
  Red:    '#e94560',
  Green:  '#34d399',
  Yellow: '#fbbf24',
  Blue:   '#60a5fa',
};

// ── Atom sub-components ─────────────────────────────────────────────────────

function Dot({ color }: { color: Col }) {
  return <span className={styles.dot} style={{ background: COL[color] }} />;
}

function Stub({ color, label, faded }: { color?: Col; label?: string; faded?: boolean }) {
  const accent = color ? COL[color] : '#666';
  return (
    <span
      className={`${styles.stub}${faded ? ` ${styles.faded}` : ''}`}
      style={{ borderTopColor: accent, color: accent }}
    >
      {label ?? (color ? '●' : '?')}
    </span>
  );
}

function Sig() { return <span className={styles.sigma}>Σ</span>; }
function Op({ s }: { s: string }) { return <span className={styles.op}>{s}</span>; }
function N({ v }: { v: number | string }) { return <span className={styles.num}>{v}</span>; }
function Braces({ children }: { children: ReactNode }) {
  return <span className={styles.braces}>{children}</span>;
}
function Row2({ top, bot }: { top: ReactNode; bot: ReactNode }) {
  return (
    <span className={styles.twoLine}>
      <span className={styles.twoLineRow}>{top}</span>
      <span className={styles.twoLineRow}>{bot}</span>
    </span>
  );
}
function Dots4() {
  return (
    <span className={styles.dots4}>
      <Dot color="Red" /><Dot color="Green" />
      <Dot color="Blue" /><Dot color="Yellow" />
    </span>
  );
}

// ── Mission visual registry ─────────────────────────────────────────────────

type Render = () => JSX.Element;

const VISUALS: Record<string, Render> = {
  // Total sum
  sum_10: () => <><Sig/><Op s="="/><N v={10}/></>,
  sum_15: () => <><Sig/><Op s="="/><N v={15}/></>,
  sum_18: () => <><Sig/><Op s="="/><N v={18}/></>,
  sum_20: () => <><Sig/><Op s="="/><N v={20}/></>,

  // Color sums
  red_sum_4:     () => <><Sig/><Dot color="Red"   /><Op s="="/><N v={4} /></>,
  red_sum_10:    () => <><Sig/><Dot color="Red"   /><Op s="="/><N v={10}/></>,
  yellow_sum_2:  () => <><Sig/><Dot color="Yellow"/><Op s="="/><N v={2} /></>,
  yellow_sum_11: () => <><Sig/><Dot color="Yellow"/><Op s="="/><N v={11}/></>,
  blue_sum_3:    () => <><Sig/><Dot color="Blue"  /><Op s="="/><N v={3} /></>,
  blue_sum_9:    () => <><Sig/><Dot color="Blue"  /><Op s="="/><N v={9} /></>,
  green_sum_6:   () => <><Sig/><Dot color="Green" /><Op s="="/><N v={6} /></>,
  green_sum_7:   () => <><Sig/><Dot color="Green" /><Op s="="/><N v={7} /></>,

  // All-color-in-set
  all_red_or_blue:     () => <><Op s="∀∈"/><Braces><Dot color="Red"   /><Dot color="Blue"  /></Braces></>,
  all_yellow_or_blue:  () => <><Op s="∀∈"/><Braces><Dot color="Yellow"/><Dot color="Blue"  /></Braces></>,
  all_red_or_green:    () => <><Op s="∀∈"/><Braces><Dot color="Red"   /><Dot color="Green" /></Braces></>,
  all_yellow_or_green: () => <><Op s="∀∈"/><Braces><Dot color="Yellow"/><Dot color="Green" /></Braces></>,

  // Parity
  all_odd:  () => <N v="1·3·5·7"/>,
  all_even: () => <N v="2·4·6·8"/>,

  // Value range
  all_greater_than_5: () => <><Op s="∀"/><Op s="≥"/><N v={5}/></>,
  all_lower_then_3:   () => <><Op s="∀"/><Op s="≤"/><N v={3}/></>,

  // Color count
  three_red:    () => <><Op s="#"/><Dot color="Red"   /><Op s="="/><N v={3}/></>,
  three_yellow: () => <><Op s="#"/><Dot color="Yellow"/><Op s="="/><N v={3}/></>,
  three_blue:   () => <><Op s="#"/><Dot color="Blue"  /><Op s="="/><N v={3}/></>,
  three_green:  () => <><Op s="#"/><Dot color="Green" /><Op s="="/><N v={3}/></>,

  // Adjacent (no gap)
  two_adjacent_red:    () => <><Stub color="Red"   /><Stub color="Red"   /></>,
  two_adjacent_yellow: () => <><Stub color="Yellow"/><Stub color="Yellow"/></>,
  two_adjacent_blue:   () => <><Stub color="Blue"  /><Stub color="Blue"  /></>,
  two_adjacent_green:  () => <><Stub color="Green" /><Stub color="Green" /></>,

  // Separate (≥ 2 cards apart)
  two_separate_red:    () => <><Stub color="Red"   /><Op s="···"/><Stub color="Red"   /></>,
  two_separate_yellow: () => <><Stub color="Yellow"/><Op s="···"/><Stub color="Yellow"/></>,
  two_separate_blue:   () => <><Stub color="Blue"  /><Op s="···"/><Stub color="Blue"  /></>,
  two_separate_green:  () => <><Stub color="Green" /><Op s="···"/><Stub color="Green" /></>,
  two_separate_odds:   () => <><Stub label="1·3"   /><Op s="···"/><Stub label="1·3"   /></>,

  // Barely split (exactly 1 card between)
  two_barely_split_red:    () => <><Stub color="Red"   /><Stub faded/><Stub color="Red"   /></>,
  two_barely_split_yellow: () => <><Stub color="Yellow"/><Stub faded/><Stub color="Yellow"/></>,
  two_barely_split_blue:   () => <><Stub color="Blue"  /><Stub faded/><Stub color="Blue"  /></>,
  two_barely_split_green:  () => <><Stub color="Green" /><Stub faded/><Stub color="Green" /></>,

  // Distinct
  all_distinct_values: () => <><N v={1}/><Op s="≠"/><N v={2}/><Op s="≠"/><N v={3}/></>,
  all_distinct_colors: () => <Dots4/>,
  all_distinct_colors_and_values: () => (
    <Row2
      top={<Dots4/>}
      bot={<><N v={1}/><Op s="≠"/><N v={2}/><Op s="≠"/><N v={3}/></>}
    />
  ),

  // Consecutive
  three_consecutive_ordered: () => <><Stub label="3"/><Op s="→"/><Stub label="4"/><Op s="→"/><Stub label="5"/></>,
  four_consecutive:          () => <><Stub label="3"/><Stub label="4"/><Stub label="5"/><Stub label="6"/></>,

  // Sum equality
  sum_yellow_equals_green: () => <><Sig/><Dot color="Yellow"/><Op s="="/><Sig/><Dot color="Green" /></>,
  sum_yellow_equals_red:   () => <><Sig/><Dot color="Yellow"/><Op s="="/><Sig/><Dot color="Red"   /></>,
  sum_blue_equals_green:   () => <><Sig/><Dot color="Blue"  /><Op s="="/><Sig/><Dot color="Green" /></>,
  sum_blue_equals_red:     () => <><Sig/><Dot color="Blue"  /><Op s="="/><Sig/><Dot color="Red"   /></>,

  // 2× sum equality
  twice_sum_yellow_equals_green: () => <><Op s="2×"/><Sig/><Dot color="Yellow"/><Op s="="/><Sig/><Dot color="Green" /></>,
  twice_sum_yellow_equals_red:   () => <><Op s="2×"/><Sig/><Dot color="Yellow"/><Op s="="/><Sig/><Dot color="Red"   /></>,
  twice_sum_blue_equals_green:   () => <><Op s="2×"/><Sig/><Dot color="Blue"  /><Op s="="/><Sig/><Dot color="Green" /></>,
  twice_sum_blue_equals_red:     () => <><Op s="2×"/><Sig/><Dot color="Blue"  /><Op s="="/><Sig/><Dot color="Red"   /></>,
};

// ── Mission families for grouped display ────────────────────────────────────

export interface MissionFamily {
  label: string;
  missions: string[];
}

export const MISSION_FAMILIES: MissionFamily[] = [
  { label: 'Sum',           missions: ['sum_10', 'sum_15', 'sum_18', 'sum_20'] },
  { label: 'Color Sum',     missions: ['red_sum_4', 'red_sum_10', 'yellow_sum_2', 'yellow_sum_11', 'blue_sum_3', 'blue_sum_9', 'green_sum_6', 'green_sum_7'] },
  { label: 'Color Set',     missions: ['all_red_or_blue', 'all_yellow_or_blue', 'all_red_or_green', 'all_yellow_or_green'] },
  { label: 'Parity',        missions: ['all_odd', 'all_even'] },
  { label: 'Value Range',   missions: ['all_greater_than_5', 'all_lower_then_3'] },
  { label: 'Color Count',   missions: ['three_red', 'three_yellow', 'three_blue', 'three_green'] },
  { label: 'Adjacent',      missions: ['two_adjacent_red', 'two_adjacent_yellow', 'two_adjacent_blue', 'two_adjacent_green'] },
  { label: 'Separate',      missions: ['two_separate_red', 'two_separate_yellow', 'two_separate_blue', 'two_separate_green', 'two_separate_odds'] },
  { label: 'Barely Split',  missions: ['two_barely_split_red', 'two_barely_split_yellow', 'two_barely_split_blue', 'two_barely_split_green'] },
  { label: 'Distinct',      missions: ['all_distinct_values', 'all_distinct_colors', 'all_distinct_colors_and_values'] },
  { label: 'Consecutive',   missions: ['three_consecutive_ordered', 'four_consecutive'] },
  { label: 'Sum Equal',     missions: ['sum_yellow_equals_green', 'sum_yellow_equals_red', 'sum_blue_equals_green', 'sum_blue_equals_red'] },
  { label: '2× Sum Equal',  missions: ['twice_sum_yellow_equals_green', 'twice_sum_yellow_equals_red', 'twice_sum_blue_equals_green', 'twice_sum_blue_equals_red'] },
];

// ── Component ────────────────────────────────────────────────────────────────

interface Props {
  name: string;
  showLabel?: boolean;
}

export function MissionCard({ name, showLabel }: Props) {
  const render = VISUALS[name];
  return (
    <div className={styles.wrap}>
      <div className={styles.card} title={name}>
        <div className={styles.body}>
          {render ? render() : <span className={styles.unknown}>?</span>}
        </div>
      </div>
      {showLabel && <span className={styles.label}>{name}</span>}
    </div>
  );
}
