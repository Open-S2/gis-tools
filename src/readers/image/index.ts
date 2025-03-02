import type { ValueArrayObject } from '../..';

export * from './decoder';
export * from './jpeg';
export * from './jpeg2000';
export * from './lanczos';
export * from './util';

/** An RGBA color */
export interface RGBA extends ValueArrayObject {
  r: number;
  g: number;
  b: number;
  a: number;
}

/** Gamma value; How displays store RGB values is gamma-corrected for human perception purposes */
export const GAMMA = 2.2;

/**
 * Convert from sRGB (gamma-encoded) to linear space
 * @param n - singular R, G, or B value
 * @returns - linear value
 */
export function sRGBToLinear(n: number): number {
  return Math.pow(n / 255, 1 / GAMMA);
}

/**
 * Convert from linear space to sRGB (gamma-encoded)
 * @param n - singular R, G, or B value
 * @returns - gamma-encoded value
 */
export function lRGBToGamma(n: number): number {
  return Math.pow(n, GAMMA) * 255;
}
