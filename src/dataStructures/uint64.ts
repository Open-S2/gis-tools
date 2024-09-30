import { S2Point, fromS2Point as fromS2, fromST } from '../geometry';

import type { Face, Point3D, S2CellId } from '../geometry';

/** An Uint64Cell contains all the information needed to uniquely identify a 64-bit cell */
export interface Uint64Cell {
  low: number;
  high: number;
}

/** The 64-bit cell id in BigInt, Number, or Uint64Cell form */
export type Uint64 = number | S2CellId | Uint64Cell;

/**
 * Convert a BigInt to an Uint64Cell representation
 * @param id - the 64-bit cell id in BigInt or number form
 * @returns - an Uint64Cell with the appropriate id and functions
 */
export function toCell(id: Uint64): Uint64Cell {
  if (typeof id === 'object') return id;
  const bigint = BigInt(id);
  return {
    low: Number(bigint & 0xffffffffn),
    high: Number(bigint >> 32n) & 0xffffffff,
  };
}

/**
 * Convert a lon/lat to an Uint64Cell representation
 * @param lon - longitude
 * @param lat - latitude
 * @returns - an Uint64Cell with the appropriate id and functions
 */
export function fromLonLat(lon: number, lat: number): Uint64Cell {
  const { toST, fromLonLat } = S2Point;
  const [face, s, t] = toST(fromLonLat(lon, lat));
  return fromFaceST(face, s, t);
}

/**
 * Convert a face/s/t to an Uint64Cell representation
 * @param face - face on the sphere
 * @param s - x position
 * @param t - y position
 * @returns - an Uint64Cell with the appropriate id and functions
 */
export function fromFaceST(face: Face, s: number, t: number): Uint64Cell {
  const id = fromST(face, s, t);
  return toCell(id);
}

/**
 * @param point - a vector point on the sphere
 * @returns - an Uint64Cell with the appropriate id and functions
 */
export function fromS2Point(point: Point3D): Uint64Cell {
  const id = fromS2(point);
  return toCell(id);
}

/**
 * @param a - the first cell
 * @param b - the second cell
 * @returns -1 | 0 | 1
 */
export function compare(a: Uint64Cell, b: Uint64Cell): -1 | 0 | 1 {
  if (a.high < b.high) return -1;
  if (a.high > b.high) return 1;
  if (a.low < b.low) return -1;
  if (a.low > b.low) return 1;
  return 0;
}
