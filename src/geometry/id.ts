/** COMPONENTS */
import {
  IJtoST,
  STtoIJ,
  quadraticSTtoUV as STtoUV,
  SiTiToST,
  quadraticUVtoST as UVtoST,
  XYZtoFaceUV,
  faceUVtoXYZ,
  getUNorm,
  getVNorm,
  lonLatToXYZ,
  xyzToLonLat,
} from './s2/coords';
import { toIJ as S2PointToIJ, fromS2CellID, invert, normalize } from './s2/point';

import type { BBox, Face, Point3D } from './';

/**
 * An S2CellId is a 64-bit unsigned integer that uniquely identifies a
 * cell in the S2 cell decomposition.  It has the following format:
 *
 *   id = [face][face_pos]
 *
 *   face:     a 3-bit number (range 0..5) encoding the cube face.
 *
 *   face_pos: a 61-bit number encoding the position of the center of this
 *             cell along the Hilbert curve over this face (see the Wiki
 *             pages for details).
 *
 * Sequentially increasing cell ids follow a continuous space-filling curve
 * over the entire sphere.  They have the following properties:
 *
 *  - The id of a cell at level k consists of a 3-bit face number followed
 *    by k bit pairs that recursively select one of the four children of
 *    each cell.  The next bit is always 1, and all other bits are 0.
 *    Therefore, the level of a cell is determined by the position of its
 *    lowest-numbered bit that is turned on (for a cell at level k, this
 *    position is 2 * (kMaxLevel - k).)
 *
 *  - The id of a parent cell is at the midpoint of the range of ids spanned
 *    by its children (or by its descendants at any level).
 *
 * Leaf cells are often used to represent points on the unit sphere, and
 * this class provides methods for converting directly between these two
 * representations.  For cells that represent 2D regions rather than
 * discrete point, it is better to use the S2Cell class.
 *
 * This class is intended to be copied by value as desired.  It uses
 * the default copy constructor and assignment operator.
 */
export type S2CellId = bigint;

/** CONSTANTS */
const LOOKUP_POS: S2CellId[] = [];
const LOOKUP_IJ: S2CellId[] = [];
export const FACE_BITS = 3n;
export const NUM_FACES = 6n;
export const K_MAX_LEVEL = 30;
export const MAX_LEVEL = 30n;
export const POS_BITS = 61n;
export const K_WRAP_OFFSET = 13835058055282163712n;
export const K_MAX_SIZE = 1073741824;

/** INITIALIZATION */
for (let i = 0; i < 4; i++) initLookupCell(0, 0, 0, i, 0, i);
/**
 * @param level - zoom level of the cell
 * @param i - x coord
 * @param j - y coord
 * @param origOrientation - original orientation
 * @param pos - position
 * @param orientation - orientation
 */
function initLookupCell(
  level: number,
  i: number,
  j: number,
  origOrientation: number,
  pos: number,
  orientation: number,
): void {
  const kPosToOriengation = [1, 0, 0, 3];
  const kPosToIJ = [
    [0, 1, 3, 2],
    [0, 2, 3, 1],
    [3, 2, 0, 1],
    [3, 1, 0, 2],
  ];
  if (level === 4) {
    const ij = (i << 4) + j;
    LOOKUP_POS[(ij << 2) + origOrientation] = BigInt((pos << 2) + orientation);
    LOOKUP_IJ[(pos << 2) + origOrientation] = BigInt((ij << 2) + orientation);
  } else {
    level++;
    i <<= 1;
    j <<= 1;
    pos <<= 2;
    const r = kPosToIJ[orientation];
    initLookupCell(
      level,
      i + (r[0] >> 1),
      j + (r[0] & 1),
      origOrientation,
      pos,
      orientation ^ kPosToOriengation[0],
    );
    initLookupCell(
      level,
      i + (r[1] >> 1),
      j + (r[1] & 1),
      origOrientation,
      pos + 1,
      orientation ^ kPosToOriengation[1],
    );
    initLookupCell(
      level,
      i + (r[2] >> 1),
      j + (r[2] & 1),
      origOrientation,
      pos + 2,
      orientation ^ kPosToOriengation[2],
    );
    initLookupCell(
      level,
      i + (r[3] >> 1),
      j + (r[3] & 1),
      origOrientation,
      pos + 3,
      orientation ^ kPosToOriengation[3],
    );
  }
}

/**
 * Create a default S2CellID given a face on the sphere [0-6)
 * @param face - the face
 * @returns the S2CellID
 */
export function fromFace(face: Face): S2CellId {
  return (BigInt(face) << POS_BITS) + (1n << 60n);
}

/**
 * Return a cell given its face (range 0..5), Hilbert curve position within
 * that face (an unsigned integer with S2CellId::kPosBits bits), and level
 * (range 0..kMaxLevel).  The given position will be modified to correspond
 * to the Hilbert curve position at the center of the returned cell. This
 * is a static function rather than a constructor in order to indicate what
 * the arguments represent.
 * @param face - the face
 * @param pos - the Hilbert curve position
 * @param level - the level
 * @returns the S2CellID
 */
export function fromFacePosLevel(face: Face, pos: bigint, level: number): S2CellId {
  const cell = (BigInt(face) << POS_BITS) + (pos | 1n);
  return parentLevel(cell, BigInt(level));
}

/**
 * Create an S2CellID from a lon-lat coordinate
 * @param lon - longitude
 * @param lat - latitude
 * @returns the S2CellID
 */
export function fromLonLat(lon: number, lat: number): S2CellId {
  const xyz = lonLatToXYZ(lon, lat);
  return fromS2Point(xyz);
}

/**
 * Create an S2CellID from an XYZ Point
 * @param xyz - 3D input vector
 * @returns the S2CellID
 */
export function fromS2Point(xyz: Point3D): S2CellId {
  // convert to face-i-j
  const [face, i, j] = S2PointToIJ(xyz);
  // now convert from ij
  return fromIJ(face, i, j);
}

/**
 * Create an S2CellID from an Face-U-V coordinate
 * @param face - the face
 * @param u - u coordinate
 * @param v - v coordinate
 * @returns the S2CellID
 */
export function fromUV(face: Face, u: number, v: number): S2CellId {
  // now convert from st
  return fromST(face, UVtoST(u), UVtoST(v));
}

/**
 * Create an S2CellID from an Face-S-T coordinate
 * @param face - the face
 * @param s - s coordinate
 * @param t - t coordinate
 * @returns the S2CellID
 */
export function fromST(face: Face, s: number, t: number): S2CellId {
  // now convert from ij
  return fromIJ(face, STtoIJ(s), STtoIJ(t));
}

/**
 * Create an S2CellID given a distance and level (zoom). Default level is 30n
 * @param distance - distance
 * @param level - level
 * @returns the S2CellID
 */
export function fromDistance(distance: bigint, level = MAX_LEVEL): S2CellId {
  level = 2n * (MAX_LEVEL - level);
  return (distance << (level + 1n)) + (1n << level);
}

/**
 * @param id - the S2CellID
 * @returns [face, zoom, i, j]
 */
export function toFaceIJ(id: S2CellId): [face: Face, zoom: number, i: number, j: number] {
  const zoom = level(id);
  const [face, i, j] = toIJ(id, zoom);
  return [face, zoom, i, j];
}

/**
 * Create an S2CellID from an Face-I-J coordinate and map it to a zoom if desired.
 * @param face - the face
 * @param i - i coordinate
 * @param j - j coordinate
 * @param level - zoom level
 * @returns the S2CellID
 */
export function fromIJ(face: Face, i: number, j: number, level?: number): S2CellId {
  const bigFace = BigInt(face);
  let bigI = BigInt(i);
  let bigJ = BigInt(j);
  if (level !== undefined) {
    const levelB = BigInt(level);
    bigI = bigI << (MAX_LEVEL - levelB);
    bigJ = bigJ << (MAX_LEVEL - levelB);
  }
  let n = bigFace << 60n;
  // Alternating faces have opposite Hilbert curve orientations; this
  // is necessary in order for all faces to have a right-handed
  // coordinate system.
  let bits = bigFace & 1n;
  // Each iteration maps 4 bits of "i" and "j" into 8 bits of the Hilbert
  // curve position.  The lookup table transforms a 10-bit key of the form
  // "iiiijjjjoo" to a 10-bit value of the form "ppppppppoo", where the
  // letters [ijpo] denote bits of "i", "j", Hilbert curve position, and
  // Hilbert curve orientation respectively.
  for (let k = 7n; k >= 0n; k--) {
    const kk = k * 4n;
    bits += ((bigI >> kk) & 15n) << NUM_FACES;
    bits += ((bigJ >> kk) & 15n) << 2n;
    bits = LOOKUP_POS[Number(bits)];
    n |= (bits >> 2n) << (k * 8n);
    bits &= FACE_BITS;
  }

  const id = n * 2n + 1n;

  if (level !== undefined) return parent(id, level);
  return id;
}

/**
 * Convert an S2CellID to a Face-I-J coordinate and provide its orientation.
 * If a level is provided, the I-J coordinates will be shifted to that level.
 * @param id - the S2CellID
 * @param level - zoom level
 * @returns face-i-j with orientation
 */
export function toIJ(
  id: S2CellId,
  level?: number | bigint,
): [face: Face, i: number, j: number, orientation: number] {
  let i = 0n;
  let j = 0n;
  const face = Number(id >> POS_BITS);
  let bits = BigInt(face) & 1n;

  // Each iteration maps 8 bits of the Hilbert curve position into
  // 4 bits of "i" and "j".  The lookup table transforms a key of the
  // form "ppppppppoo" to a value of the form "iiiijjjjoo", where the
  // letters [ijpo] represents bits of "i", "j", the Hilbert curve
  // position, and the Hilbert curve orientation respectively.
  //
  // On the first iteration we need to be careful to clear out the bits
  // representing the cube face.
  for (let k = 7n; k >= 0n; k--) {
    const nbits = k === 7n ? 2n : 4n;
    bits += ((id >> (k * 8n + 1n)) & ((1n << (2n * nbits)) - 1n)) << 2n;
    bits = LOOKUP_IJ[Number(bits)];
    i += (bits >> NUM_FACES) << (k * 4n);
    j += ((bits >> 2n) & 15n) << (k * 4n);
    bits &= FACE_BITS;
  }

  // adjust bits to the orientation
  const lsb = id & (~id + 1n);
  if ((lsb & 1229782938247303424n) !== 0n) bits ^= 1n;

  if (level !== undefined) {
    level = BigInt(level);
    i = i >> (MAX_LEVEL - level);
    j = j >> (MAX_LEVEL - level);
  }
  return [face as Face, Number(i), Number(j), Number(bits)];
}

/**
 * Convert an S2CellID to an Face-S-T coordinate
 * @param id - the S2CellID
 * @returns face-s-t coordinate associated with the S2CellID
 */
export function toST(id: S2CellId): [face: Face, s: number, t: number] {
  const [face, i, j] = toIJ(id);
  const s = IJtoST(i);
  const t = IJtoST(j);

  return [face, s, t];
}

/**
 * Convert an S2CellID to an Face-U-V coordinate
 * @param id - the S2CellID
 * @returns face-u-v coordinate associated with the S2CellID
 */
export function toUV(id: S2CellId): [face: Face, u: number, v: number] {
  const [face, s, t] = toST(id);
  const u = STtoUV(s);
  const v = STtoUV(t);

  return [face, u, v];
}

/**
 * Convert an S2CellID to an lon-lat coordinate
 * @param id - the S2CellID
 * @returns lon-lat coordinates
 */
export function toLonLat(id: S2CellId): [lon: number, lat: number] {
  const xyz = toS2Point(id);

  return xyzToLonLat(xyz);
}

/**
 * Convert an S2CellID to an XYZ Point
 * @param id - the S2CellID
 * @returns a 3D vector
 */
export function toS2Point(id: S2CellId): Point3D {
  return fromS2CellID(id);
}

/**
 * Given an S2CellID, get the face it's located in
 * @param id - the S2CellID
 * @returns face of the cell
 */
export function face(id: S2CellId): Face {
  const face = Number(id >> POS_BITS);
  return face as Face;
}

/**
 * Given an S2CellID, check if it is a Face Cell.
 * @param id - the S2CellID
 * @returns true if the cell is a face (lowest zoom level)
 */
export function isFace(id: S2CellId): boolean {
  return (id & ((1n << 60n) - 1n)) === 0n;
}

/**
 * Given an S2CellID, find the quad tree position [0-4) it's located in
 * @param id - the S2CellID
 * @returns quad tree position
 */
export function pos(id: S2CellId): S2CellId {
  return id & 2305843009213693951n;
}

/**
 * Given an S2CellID, find the level (zoom) its located in
 * @param id - the S2CellID
 * @returns zoom level
 */
export function level(id: S2CellId): number {
  let count = 0;

  let i = 0n;
  while ((id & (1n << i)) === 0n && i < 60n) {
    i += 2n;
    count++;
  }

  return 30 - count;
}

/**
 * Given an S2CellID, get the distance it spans (or length it covers)
 * @param id - the S2CellID
 * @param lev - optional zoom level
 * @returns distance
 */
export function distance(id: S2CellId, lev?: number): bigint {
  if (lev === undefined) lev = level(id);
  return id >> BigInt(2 * (30 - lev) + 1);
}

/**
 * Given an S2CellID, get the quad child tile of your choice [0, 4)
 * @param id - the S2CellID
 * @param pos - quad position 0, 1, 2, or 3
 * @returns the child tile at that position
 */
export function child(id: S2CellId, pos: 0n | 1n | 2n | 3n): S2CellId {
  const newLSB = (id & (~id + 1n)) >> 2n;
  return id + (2n * pos - FACE_BITS) * newLSB;
}

/**
 * Given an S2CellID, get all the quad children tiles
 * @param id - the S2CellID
 * @param orientation - orientation of the child (0 or 1)
 * @returns the child tile at that position
 */
export function children(id: S2CellId, orientation = 0): [S2CellId, S2CellId, S2CellId, S2CellId] {
  const childs: [S2CellId, S2CellId, S2CellId, S2CellId] = [
    child(id, 0n),
    child(id, 3n),
    child(id, 2n),
    child(id, 1n),
  ];
  if (orientation === 0) {
    const tmp = childs[1];
    childs[1] = childs[3];
    childs[3] = tmp;
  }

  return childs;
}

/**
 * Given a Face-level-i-j coordinate, get all its quad children tiles
 * @param face - the Face
 * @param level - zoom level
 * @param i - i coordinate
 * @param j - j coordinate
 * @returns the child tile at that position
 */
export function childrenIJ(
  face: Face,
  level: number,
  i: number,
  j: number,
): [blID: S2CellId, brID: S2CellId, tlID: S2CellId, trID: S2CellId] {
  i = i << 1;
  j = j << 1;

  return [
    fromIJ(face, i, j, level + 1),
    fromIJ(face, i + 1, j, level + 1),
    fromIJ(face, i, j + 1, level + 1),
    fromIJ(face, i + 1, j + 1, level + 1),
  ];
}

/**
 * Given an S2CellID, get the quad position relative to its parent
 * @param id - the S2CellID
 * @param level - zoom level
 * @returns the child tile at that position
 */
export function childPosition(id: S2CellId, level: number): number {
  return Number((id >> (2n * (MAX_LEVEL - BigInt(level)) + 1n)) & FACE_BITS);
}

/**
 * Given an S2CellID, get the parent quad tile
 * @param id - the S2CellID
 * @param level - zoom level
 * @returns the parent of the input S2CellID
 */
export function parent(id: S2CellId, level?: number): S2CellId {
  const newLSB =
    level !== undefined ? 1n << (2n * (MAX_LEVEL - BigInt(level))) : (id & (~id + 1n)) << 2n;
  return (id & (~newLSB + 1n)) | newLSB;
}

/**
 * given an id and level, return the id of the parent level
 * @param id - the S2CellID
 * @param level - zoom level
 * @returns - the parent of the input S2CellID
 */
export function parentLevel(id: S2CellId, level: bigint): S2CellId {
  const newLsb = 1n << (2n * (MAX_LEVEL - level));
  return (id & (~newLsb + 1n)) | newLsb;
}

/**
 * Given an S2CellID, get the hilbert range it spans
 * @param id - the S2CellID
 * @returns [min, max]
 */
export function range(id: S2CellId): [min: S2CellId, max: S2CellId] {
  const lsb = id & (~id + 1n);

  return [id - (lsb - 1n), id + (lsb - 1n)];
}

/**
 * Check if the first S2CellID contains the second.
 * @param a - the first S2CellID
 * @param b - the second S2CellID
 * @returns true if a contains b
 */
export function contains(a: S2CellId, b: S2CellId): boolean {
  const [min, max] = range(a);
  return b >= min && b <= max;
}

/**
 * @param a - the first S2CellID
 * @param p - the second Point3D
 * @returns true if a contains p
 */
export function containsS2Point(a: S2CellId, p: Point3D): boolean {
  const b = fromS2Point(p);
  return contains(a, b);
}

/**
 * Check if an S2CellID intersects another. This includes edges touching.
 * @param a - the first S2CellID
 * @param b - the second S2CellID
 * @returns true if a intersects b
 */
export function intersects(a: S2CellId, b: S2CellId): boolean {
  const [aMin, aMax] = range(a);
  const [bMin, bMax] = range(b);
  return bMin <= aMax && bMax >= aMin;
}

/**
 * Get the next S2CellID in the hilbert space
 * @param id - input S2CellID
 * @returns the next S2CellID in the hilbert space
 */
export function next(id: S2CellId): S2CellId {
  const n = id + ((id & (~id + 1n)) << 1n);
  if (n < K_WRAP_OFFSET) return n;
  return n - K_WRAP_OFFSET;
}

/**
 * Get the previous S2CellID in the hilbert space
 * @param id - input S2CellID
 * @returns the previous S2CellID in the hilbert space
 */
export function prev(id: S2CellId): S2CellId {
  const p = id - ((id & (~id + 1n)) << 1n);
  if (p < K_WRAP_OFFSET) return p;
  return p + K_WRAP_OFFSET;
}

/**
 * Check if the S2CellID is a leaf value. This means it's the smallest possible cell
 * @param id - input S2CellID
 * @returns true if the S2CellID is a leaf
 */
export function isLeaf(id: S2CellId): boolean {
  return (id & 1n) === 1n;
}

/**
 * Given an S2CellID and level (zoom), get the center point of that cell in S-T space
 * @param id - the S2CellID
 * @returns [face, s, t]
 */
export function centerST(id: S2CellId): [face: Face, s: number, t: number] {
  const [face, i, j] = toIJ(id);
  const delta = (id & 1n) !== 0n ? 1 : ((BigInt(i) ^ (id >> 2n)) & 1n) !== 0n ? 2 : 0;
  // Note that (2 * {i,j} + delta) will never overflow a 32-bit integer.
  const si = 2 * i + delta;
  const ti = 2 * j + delta;

  return [face, SiTiToST(Number(si)), SiTiToST(Number(ti))];
}

/**
 * Given an S2CellID and level (zoom), get the S-T bounding range of that cell
 * @param id - the S2CellID
 * @param lev - zoom level
 * @returns [sMin, tMin, sMax, tMax]
 */
export function boundsST(id: S2CellId, lev: number): BBox {
  if (lev === undefined) lev = level(id);

  const [, s, t] = centerST(id);
  const halfSize = sizeST(lev) * 0.5;

  return [s - halfSize, t - halfSize, s + halfSize, t + halfSize];
}

/**
 * Return the range maximum of a level (zoom) in S-T space
 * @param level - zoom level
 * @returns sMax or tMax
 */
export function sizeST(level: number): number {
  return IJtoST(sizeIJ(level));
}

/**
 * Return the range maximum of a level (zoom) in I-J space
 * @param level - zoom level
 * @returns iMax or jMax
 */
export function sizeIJ(level: number): number {
  return 1 << (30 - level);
}

/**
 * Given an S2CellID, find the neighboring S2CellIDs
 * @param id - the S2CellID
 * @returns [up, right, down, left]
 */
export function neighbors(id: S2CellId): [S2CellId, S2CellId, S2CellId, S2CellId] {
  const lev = level(id);
  const size = sizeIJ(lev);
  const [face, i, j] = toIJ(id);

  return [
    parent(fromIJSame(face, i, j - size, j - size >= 0), lev),
    parent(fromIJSame(face, i + size, j, i + size < K_MAX_SIZE), lev),
    parent(fromIJSame(face, i, j + size, j + size < K_MAX_SIZE), lev),
    parent(fromIJSame(face, i - size, j, i - size >= 0), lev),
  ];
}

/**
 * Given a Face-I-J and a desired level (zoom), find the neighboring S2CellIDs
 * @param face - the Face
 * @param i - the I coordinate
 * @param j - the J coordinate
 * @param level - the zoom level (desired)
 * @returns neighbors: [down, right, up, left]
 */
export function neighborsIJ(
  face: Face,
  i: number,
  j: number,
  level: number,
): [S2CellId, S2CellId, S2CellId, S2CellId] {
  const size = sizeIJ(level);

  return [
    parent(fromIJSame(face, i, j - size, j - size >= 0), level),
    parent(fromIJSame(face, i + size, j, i + size < K_MAX_SIZE), level),
    parent(fromIJSame(face, i, j + size, j + size < K_MAX_SIZE), level),
    parent(fromIJSame(face, i - size, j, i - size >= 0), level),
  ];
}

/**
 * Build an S2CellID given a Face-I-J, but ensure the face is the same if desired
 * @param face - the Face
 * @param i - the I coordinate
 * @param j - the J coordinate
 * @param sameFace - if the face should be the same
 * @returns the S2CellID
 */
export function fromIJSame(face: Face, i: number, j: number, sameFace: boolean): S2CellId {
  if (sameFace) return fromIJ(face, i, j);
  else return fromIJWrap(face, i, j);
}

/**
 * Build an S2CellID given a Face-I-J, but ensure it's a legal value, otherwise wrap before creation
 * @param face - the Face
 * @param i - the I coordinate
 * @param j - the J coordinate
 * @returns the S2CellID
 */
export function fromIJWrap(face: Face, i: number, j: number): S2CellId {
  const { max, min } = Math;

  // Convert i and j to the coordinates of a leaf cell just beyond the
  // boundary of this face.  This prevents 32-bit overflow in the case
  // of finding the neighbors of a face cell.
  i = max(-1, min(K_MAX_SIZE, i));
  j = max(-1, min(K_MAX_SIZE, j));

  // We want to wrap these coordinates onto the appropriate adjacent face.
  // The easiest way to do this is to convert the (i,j) coordinates to (x,y,z)
  // (which yields a point outside the normal face boundary), and then call
  // S2::XYZtoFaceUV() to project back onto the correct face.
  //
  // The code below converts (i,j) to (si,ti), and then (si,ti) to (u,v) using
  // the linear projection (u=2*s-1 and v=2*t-1).  (The code further below
  // converts back using the inverse projection, s=0.5*(u+1) and t=0.5*(v+1).
  // Any projection would work here, so we use the simplest.)  We also clamp
  // the (u,v) coordinates so that the point is barely outside the
  // [-1,1]x[-1,1] face rectangle, since otherwise the reprojection step
  // (which divides by the new z coordinate) might change the other
  // coordinates enough so that we end up in the wrong leaf cell.
  const kScale = 1 / K_MAX_SIZE;
  const kLimit = 1 + 2.2204460492503131e-16;
  const u = max(-kLimit, min(kLimit, kScale * (2 * (i - K_MAX_SIZE / 2) + 1)));
  const v = max(-kLimit, min(kLimit, kScale * (2 * (j - K_MAX_SIZE / 2) + 1)));

  // Find the leaf cell coordinates on the adjacent face, and convert
  // them to a cell id at the appropriate level.
  const [nFace, nU, nV] = XYZtoFaceUV(faceUVtoXYZ(face, u, v));
  return fromIJ(nFace, STtoIJ(0.5 * (nU + 1)), STtoIJ(0.5 * (nV + 1)));
}

/**
 * Given an S2CellID, find it's nearest neighbors associated with it
 * @param id - the S2CellID
 * @param lev - the zoom level (if not provided, defaults to current level of id)
 * @returns neighbors
 */
export function vertexNeighbors(id: S2CellId, lev?: number): S2CellId[] {
  if (lev === undefined) lev = level(id);
  const res: S2CellId[] = [];

  const [face, i, j] = toIJ(id);

  // Determine the i- and j-offsets to the closest neighboring cell in each
  // direction.  This involves looking at the next bit of "i" and "j" to
  // determine which quadrant of this->parent(level) this cell lies in.
  const halfsize = sizeIJ(lev + 1);
  const size = halfsize << 1;
  let isame: boolean, jsame: boolean, ioffset: number, joffset: number;

  if ((i & halfsize) !== 0) {
    ioffset = size;
    isame = i + size < K_MAX_SIZE;
  } else {
    ioffset = -size;
    isame = i - size >= 0;
  }
  if ((j & halfsize) !== 0) {
    joffset = size;
    jsame = j + size < K_MAX_SIZE;
  } else {
    joffset = -size;
    jsame = j - size >= 0;
  }

  res.push(parent(id, lev));
  res.push(parent(fromIJSame(face, i + ioffset, j, isame), lev));
  res.push(parent(fromIJSame(face, i, j + joffset, jsame), lev));
  if (isame || jsame)
    res.push(parent(fromIJSame(face, i + ioffset, j + joffset, isame && jsame), lev));

  return res;
}

/** The four vertices of the cell. */
export type Vertices = [Point3D, Point3D, Point3D, Point3D];

/**
 * Returns the four vertices of the cell.  Vertices are returned
 * in CCW order (lower left, lower right, upper right, upper left in the UV
 * plane).  The points returned by getVertices are normalized.
 * @param id - the S2CellID
 * @returns the k-th vertex of the cell
 */
export function getVertices(id: S2CellId): Vertices {
  return getVerticesRaw(id).map(normalize) as Vertices;
}

/**
 * Returns the k-th vertex of the cell (k = 0,1,2,3).  Vertices are returned
 * in CCW order (lower left, lower right, upper right, upper left in the UV
 * plane).  The points returned by getVerticesRaw are not normalized.
 * @param id - the S2CellID
 * @returns the k-th vertex of the cell
 */
export function getVerticesRaw(id: S2CellId): Vertices {
  const f = face(id);
  const [uLow, uHigh, vLow, vHigh] = getBoundUV(id);
  return [
    faceUVtoXYZ(f, uLow, vLow),
    faceUVtoXYZ(f, uHigh, vLow),
    faceUVtoXYZ(f, uHigh, vHigh),
    faceUVtoXYZ(f, uLow, vHigh),
  ];
}

/**
 * Returns the inward-facing normal of the great circle passing through the
 * edge from vertex k to vertex k+1 (mod 4). The normals returned by
 * getEdges will be unit length.
 * @param id - the S2CellID
 * @returns the 4 edges of the cell normalized
 */
export function getEdges(id: S2CellId): Vertices {
  return getEdgesRaw(id).map(normalize) as Vertices;
}

/**
 * Returns the inward-facing normal of the great circle passing through the
 * edge from vertex k to vertex k+1 (mod 4). The normals returned by
 * getEdgesRaw are not necessarily unit length.
 * @param id - the S2CellID
 * @returns the 4 edges of the cell
 */
export function getEdgesRaw(id: S2CellId): Vertices {
  const f = face(id);
  const [uLow, uHigh, vLow, vHigh] = getBoundUV(id);
  return [
    getVNorm(f, vLow),
    getUNorm(f, uHigh),
    invert(getVNorm(f, vHigh)),
    invert(getUNorm(f, uLow)),
  ];
}

/**
 * Return the bounds of this cell in (u,v)-space.
 * @param id - the S2CellID
 * @returns the bounds [uLow, uHigh, vLow, vHigh]
 */
export function getBoundUV(id: S2CellId): BBox {
  const [, i, j] = toIJ(id);
  const cellSize = getSizeIJ(id);
  const iLow = i & -cellSize;
  const jLow = j & -cellSize;
  const ijBounds: BBox = [iLow, iLow + cellSize, jLow, jLow + cellSize];
  return ijBounds.map((n) => STtoUV(IJtoST(n))) as BBox;
}

/**
 * Return the edge length of this cell in (i,j)-space.
 * @param id - the S2CellID
 * @returns the edge length
 */
export function getSizeIJ(id: S2CellId): number {
  return 1 << (K_MAX_LEVEL - level(id));
}
