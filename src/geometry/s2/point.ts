import { EARTH_RADIUS_EQUATORIAL, EARTH_RADIUS_POLAR } from '../../space/planets/earth';
import {
  IJtoST,
  STtoIJ,
  quadraticSTtoUV as STtoUV,
  quadraticUVtoST as UVtoST,
  XYZtoFace,
  XYZtoFaceUV,
  faceUVtoXYZ,
  faceUVtoXYZGL,
  lonLatToXYZ,
  lonLatToXYZGL,
  xyzToLonLat,
} from './coords';
import { fromS2Point as idFromS2Point, toUV as idToUV } from '../id';

import type { S1Angle } from '../s1/angle';
import type { Face, MValue, Properties, S2CellId, VectorPoint } from '../';

/**
 * Convert a lon-lat coord to an XYZ Point using the left-hand-rule
 * @param ll - LonLat vector point in degrees
 * @returns The XYZ Point
 */
export function fromLonLat<M extends MValue = Properties>(ll: VectorPoint<M>): VectorPoint<M> {
  return lonLatToXYZ(ll);
}

/**
 * Convert a lon-lat coord to an XYZ Point using the right-hand-rule.
 * This function takes longitude and latitude as input and returns the corresponding XYZ coordinates.
 * @param ll - LonLat vector point in degrees
 * @returns The XYZ Point representing the provided longitude and latitude.
 */
export function fromLonLatGL<M extends MValue = Properties>(ll: VectorPoint<M>): VectorPoint<M> {
  // Convert longitude and latitude to XYZ coordinates using the right-hand rule.
  return lonLatToXYZGL(ll);
}

/**
 * Convert a u-v coordinate to an XYZ Point.
 * @param face - The face of the S2 cell.
 * @param u - The u-coordinate on the face.
 * @param v - The v-coordinate on the face.
 * @param m - M-Value data
 * @returns The XYZ Point representing the given u-v coordinates.
 */
export function fromUV<M extends MValue = Properties>(
  face: Face,
  u: number,
  v: number,
  m?: M,
): VectorPoint<M> {
  // Convert the given u-v coordinates to an XYZ Point using the left-hand rule.
  return normalize(faceUVtoXYZ(face, u, v, m));
}

/**
 * Convert an s-t coordinate to an XYZ Point.
 * @param face - The face of the S2 cell.
 * @param s - The s-coordinate on the face.
 * @param t - The t-coordinate on the face.
 * @param m - M-Value data
 * @returns The XYZ Point representing the given s-t coordinates.
 */
export function fromST<M extends MValue = Properties>(
  face: Face,
  s: number,
  t: number,
  m?: M,
): VectorPoint<M> {
  // Convert the given s-t coordinates to u-v coordinates.
  const u = STtoUV(s);
  const v = STtoUV(t);

  // Convert the u-v coordinates to an XYZ Point.
  return fromUV(face, u, v, m);
}

/**
 * Convert an i-j coordinate to an XYZ Point.
 * @param face - The face of the S2 cell.
 * @param i - The i-coordinate on the face.
 * @param j - The j-coordinate on the face.
 * @returns The XYZ Point representing the given i-j coordinates.
 */
export function fromIJ(face: Face, i: number, j: number): VectorPoint {
  // Convert the given i-j coordinates to s-t coordinates.
  const s = IJtoST(i);
  const t = IJtoST(j);

  // Convert the s-t coordinates to an XYZ Point.
  return fromST(face, s, t);
}

/**
 * Convert an S2CellID to an XYZ Point.
 * @param id - The S2CellID to convert.
 * @returns The XYZ Point representing the given S2CellID.
 */
export function fromS2CellID(id: S2CellId): VectorPoint {
  // Decompose the S2CellID into its constituent parts: face, u, and v.
  const [face, u, v] = idToUV(id);

  // Use the decomposed parts to construct an XYZ Point.
  return normalize(fromUV(face, u, v));
}

/**
 * Convert an Face-U-V coord to an XYZ Point using the right-hand-rule
 * @param face - The face of the S2 cell.
 * @param u - The u-coordinate on the face.
 * @param v - The v-coordinate on the face.
 * @returns The XYZ Point representing the given Face-U-V coordinates.
 */
export function fromUVGL(face: Face, u: number, v: number): VectorPoint {
  // Convert the given Face-U-V coordinates to an XYZ Point using the right-hand rule.
  return normalize(faceUVtoXYZGL(face, u, v));
}

/**
 * Convert an Face-S-T coord to an XYZ Point using the right-hand-rule
 * @param face - The face of the S2 cell.
 * @param s - The s-coordinate on the face.
 * @param t - The t-coordinate on the face.
 * @returns The XYZ Point representing the given Face-S-T coordinates.
 */
export function fromSTGL(face: Face, s: number, t: number): VectorPoint {
  // Convert the given Face-S-T coordinates to an XYZ Point using the right-hand rule.
  // First, convert the s-t coordinates to u-v coordinates.
  const [u, v] = [STtoUV(s), STtoUV(t)];

  // Then, convert the u-v coordinates to an XYZ Point.
  return fromUVGL(face, u, v);
}

/**
 * Convert an XYZ Point to a Face-U-V coord
 * @param xyz - The XYZ Point to convert.
 * @returns - The Face-U-V coordinates representing the given XYZ Point.
 */
export function toUV(xyz: VectorPoint): [face: Face, u: number, v: number] {
  // Convert the given XYZ Point to Face-U-V coordinates using the right-hand rule.
  return XYZtoFaceUV(xyz);
}

/**
 * Convert an XYZ Point to a Face-S-T coord
 * @param xyz - The XYZ Point to convert.
 * @returns - The Face-S-T coordinates representing the given XYZ Point.
 */
export function toST(xyz: VectorPoint): [face: Face, s: number, t: number] {
  // Convert the given XYZ Point to Face-U-V coordinates.
  const [face, u, v] = toUV(xyz);

  // Convert the U-V coordinates to S-T coordinates using the inverse of the
  // UVtoST function.
  return [face, UVtoST(u), UVtoST(v)];
}

/**
 * Convert an XYZ Point to a Face-I-J coord
 * @param xyz - The XYZ Point to convert.
 * @param level - The zoom level of the result. If not provided, the result will have 30 bits of precision.
 * @returns The Face-I-J coordinates representing the given XYZ Point.
 */
export function toIJ(xyz: VectorPoint, level?: number): [face: Face, i: number, j: number] {
  // Convert the given XYZ Point to Face-S-T coordinates.
  const [face, s, t] = toST(xyz);

  // Convert the S-T coordinates to I-J coordinates using the STtoIJ function.
  let i = STtoIJ(s);
  let j = STtoIJ(t);

  // If a level is provided, shift the I-J coordinates to the right by (30 - level) bits.
  if (level !== undefined) {
    i = i >> (30 - level);
    j = j >> (30 - level);
  }

  // Return the Face-I-J coordinates.
  return [face, i, j];
}

/**
 * Convert an XYZ Point to a lon-lat coord
 * @param xyz - The XYZ Point to convert.
 * @returns The lon-lat coordinates representing the given XYZ Point.
 */
export function toLonLat<M extends MValue = Properties>(xyz: VectorPoint<M>): VectorPoint<M> {
  return xyzToLonLat(xyz);
}

/**
 * Convert an XYZ Point to an S2CellID
 * @param xyz - The XYZ Point to convert.
 * @returns The S2CellID representing the given XYZ Point.
 */
export function toS2CellID(xyz: VectorPoint): S2CellId {
  return idFromS2Point(xyz);
}

/**
 * Take an XYZ Point and add another XYZ Point to it
 * @param a - The XYZ Point to add to.
 * @param b - The XYZ Point to add.
 * @returns - The XYZ Point with the added XYZ Point.
 */
export function add(a: VectorPoint, b: VectorPoint): VectorPoint {
  return { x: a.x + b.x, y: a.y + b.y, z: (a.z ?? 1) + (b.z ?? 1) };
}

/**
 * Take an XYZ Point and add another XYZ Point to it
 * @param a - The XYZ Point to add to.
 * @param b - The XYZ Point to add.
 */
export function addMut(a: VectorPoint, b: VectorPoint): void {
  a.x += b.x;
  a.y += b.y;
  if (a.z === undefined || b.z === undefined) return;
  a.z += b.z;
}

/**
 * Take an XYZ Point and add an n to each component
 * @param xyz - The XYZ Point to add to.
 * @param n - The amount to add.
 * @returns - The XYZ Point with the added amount.
 */
export function addScalar(xyz: VectorPoint, n: number): VectorPoint {
  return { x: xyz.x + n, y: xyz.y + n, z: (xyz.z ?? 1) + n };
}

/**
 * Take an XYZ Point and subtract another XYZ Point from it
 * @param a - The XYZ Point to subtract from.
 * @param b - The XYZ Point to subtract.
 * @returns - The XYZ Point with the subtracted XYZ Point.
 */
export function sub(a: VectorPoint, b: VectorPoint): VectorPoint {
  return { x: a.x - b.x, y: a.y - b.y, z: (a.z ?? 1) - (b.z ?? 1) };
}

/**
 * Take an XYZ Point and subtract an n from each component
 * @param xyz - The XYZ Point to subtract from.
 * @param n - The amount to subtract.
 * @returns - The XYZ Point with the subtracted amount.
 */
export function subScalar(xyz: VectorPoint, n: number): VectorPoint {
  return { x: xyz.x - n, y: xyz.y - n, z: (xyz.z ?? 1) - n };
}

/**
 * Take an XYZ Point and multiply it by another XYZ Point
 * @param a - The XYZ Point to multiply.
 * @param b - The XYZ Point to multiply.
 * @returns - The XYZ Point with the multiplied XYZ Point.
 */
export function mul(a: VectorPoint, b: VectorPoint): VectorPoint {
  return { x: a.x * b.x, y: a.y * b.y, z: (a.z ?? 1) * (b.z ?? 1) };
}

/**
 * Take an XYZ Point and multiply each component by n
 * @param xyz - The XYZ Point to multiply.
 * @param n - The amount to multiply.
 * @returns - The XYZ Point with the multiplied amount.
 */
export function mulScalar(xyz: VectorPoint, n: number): VectorPoint {
  return { x: xyz.x * n, y: xyz.y * n, z: (xyz.z ?? 1) * n };
}

/**
 * Take an XYZ Point and divide it by another XYZ Point
 * @param a - The XYZ Point to divide.
 * @param b - The XYZ Point to divide by.
 * @returns - The XYZ Point with the multiplied XYZ Point.
 */
export function div(a: VectorPoint, b: VectorPoint): VectorPoint {
  return { x: a.x / b.x, y: a.y / b.y, z: (a.z ?? 1) / (b.z ?? 1) };
}

/**
 * Take an XYZ Point and divide each component by n
 * @param xyz - The XYZ Point to divide.
 * @param n - The amount to divide by.
 * @returns - The XYZ Point with the multiplied amount.
 */
export function divScalar(xyz: VectorPoint, n: number): VectorPoint {
  return { x: xyz.x / n, y: xyz.y / n, z: (xyz.z ?? 1) / n };
}

/**
 * Take an XYZ Point and divide each component by n
 * @param xyz - The XYZ Point to divide.
 * @param n - The amount to divide by.
 */
export function divMutScalar(xyz: VectorPoint, n: number) {
  xyz.x /= n;
  xyz.y /= n;
  if (xyz.z !== undefined) xyz.z /= n;
}

/**
 * Take an XYZ Point and divide each component by its length
 * @param xyz - The XYZ Point to divide.
 * @returns - The XYZ Point with the divided amount.
 */
export function normalize<M extends MValue = Properties>(xyz: VectorPoint<M>): VectorPoint<M> {
  const len = length(xyz);
  xyz.x /= len;
  xyz.y /= len;
  if (xyz.z !== undefined) xyz.z /= len;

  return xyz;
}

/**
 * Get the length of the XYZ Point
 * @param xyz - The XYZ Point
 * @returns - The length of the XYZ Point
 */
export function length(xyz: VectorPoint): number {
  return Math.sqrt(norm2(xyz));
}

/**
 * Get the squared length of the XYZ Point with itself
 * @param xyz - The XYZ Point
 * @returns - The squared length of the XYZ Point
 */
export function norm2(xyz: VectorPoint): number {
  return dot(xyz, xyz);
}

/**
 * Invert the XYZ Point
 * @param xyz - The XYZ Point
 * @returns - The inverted XYZ Point
 */
export function invert<M extends MValue = Properties>(xyz: VectorPoint<M>): VectorPoint<M> {
  const { x, y, z, m } = xyz;
  return { x: -x, y: -y, z: -(z ?? 1), m };
}

/**
 * dot returns the standard dot product of a and b.
 * @param a - The first XYZ Point
 * @param b - The second XYZ Point
 * @returns - The dot product of the two XYZ Points
 */
export function dot(a: VectorPoint, b: VectorPoint): number {
  return a.x * b.x + a.y * b.y + (a.z ?? 1) * (b.z ?? 1);
}

/**
 * Get the corss product of two XYZ Points
 * @param a - The first XYZ Point
 * @param b - The second XYZ Point
 * @returns - The cross product of the two XYZ Points
 */
export function cross(a: VectorPoint, b: VectorPoint): VectorPoint {
  const az = a.z ?? 1;
  const bz = b.z ?? 1;
  return { x: a.y * bz - az * b.y, y: az * b.x - a.x * bz, z: a.x * b.y - a.y * b.x };
}

/**
 * Get the distance between two XYZ Points using Earth's size
 * @param a - The first XYZ Point
 * @param b - The second XYZ Point
 * @returns - The distance between the two XYZ Points
 */
export function distanceEarth(a: VectorPoint, b: VectorPoint): number {
  a.x *= EARTH_RADIUS_EQUATORIAL;
  b.x *= EARTH_RADIUS_EQUATORIAL;
  a.y *= EARTH_RADIUS_EQUATORIAL;
  b.y *= EARTH_RADIUS_EQUATORIAL;
  if (a.z !== undefined) a.z *= EARTH_RADIUS_POLAR;
  if (b.z !== undefined) b.z *= EARTH_RADIUS_POLAR;

  return distance(a, b);
}

/**
 * Get the distance between two XYZ Points
 * @param a - The first XYZ Point
 * @param b - The second XYZ Point
 * @returns - The raw distance between the two XYZ Points. Highly inaccurate for large distances
 */
export function distance(a: VectorPoint, b: VectorPoint): number {
  const { sqrt, pow } = Math;

  return sqrt(pow(b.x - a.x, 2) + pow(b.y - a.y, 2) + pow((b.z ?? 0) - (a.z ?? 0), 2));
}

/**
 * @param a - The first XYZ Point
 * @param b - The second XYZ Point
 * @returns - The angle between the two XYZ Points
 */
export function angle(a: VectorPoint, b: VectorPoint): S1Angle {
  return Math.atan2(length(cross(a, b)), dot(a, b));
}

/**
 * Find the S2 Hilbert Face of the XYZ Point [0, 6)
 * @param xyz - The XYZ Point
 * @returns - The S2 Hilbert Face
 */
export function getFace(xyz: VectorPoint): number {
  return XYZtoFace(xyz);
}
