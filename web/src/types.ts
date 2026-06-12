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
}
