import type { VectorPoint } from '../..';

export * from './base';
export * from './coords';

/** All projections need these parameters */
export interface ProjectionTransform {
  shortName: string;
  projName?: string;
  forward: (p: VectorPoint) => void;
  inverse: (p: VectorPoint) => void;
}
