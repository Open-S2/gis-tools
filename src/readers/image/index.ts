import type { Properties } from '../..';

export * from './decoder';
export * from './jpeg';
export * from './jpeg2000';
export * from './lanczos';
export * from './util';

/** An RGBA color */
export interface RGBA extends Properties {
  r: number;
  g: number;
  b: number;
  a: number;
}
