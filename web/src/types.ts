export interface SimResult {
  seed: number;
  completed_missions: number;
  turns: number;
  victory: boolean;
}

export type Strategy = 'random' | 'optimistic';

export interface RunConfig {
  strategy: Strategy;
  seq_depth: number;
  batch_size: number;
  seed: number;
  trace: boolean;
}

export interface GameCard {
  color: 'Red' | 'Green' | 'Yellow' | 'Blue';
  value: number;
}

export interface SimMove {
  idx_hand: number;
  idx_table: number;
}

export interface GameState {
  current_player: 'Player0' | 'Player1';
  player_hands: GameCard[][];
  table_cards: GameCard[];
  table_missions: string[];
  deck_cards: GameCard[];
  deck_missions: string[];
  turn: number;
  completed_missions: number;
  final_sprint: boolean;
}

export interface SimTrace {
  seed: bigint;
  state_list: GameState[];
  move_list: SimMove[];
}
