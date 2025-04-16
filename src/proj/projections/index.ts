import type { VectorPoint } from '../../index.js';

export * from './base.js';
export * from './coords.js';

/** All projections need these parameters */
export interface ProjectionTransform {
  shortName: string;
  projName?: string;
  forward: (p: VectorPoint) => void;
  inverse: (p: VectorPoint) => void;
}
