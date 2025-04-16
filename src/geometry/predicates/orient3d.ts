import { estimate, predSum, resulterrbound, scale, splitter, vec } from './util.js';

import type { VectorPoint } from 's2json-spec';

const o3derrboundA = 7.771561172376103e-16; // (7 + 56 * epsilon) * epsilon;
const o3derrboundB = 3.330669073875473e-16; // (3 + 28 * epsilon) * epsilon;
const o3derrboundC = 3.2047474274603644e-31; // (26 + 288 * epsilon) * epsilon * epsilon;

/** Constants for orient3d */
export interface Orient3dConstants {
  bc: Float64Array;
  ca: Float64Array;
  ab: Float64Array;
  at_b: Float64Array;
  at_c: Float64Array;
  bt_c: Float64Array;
  bt_a: Float64Array;
  ct_a: Float64Array;
  ct_b: Float64Array;
  bct: Float64Array;
  cat: Float64Array;
  abt: Float64Array;
  u: Float64Array;

  _8: Float64Array;
  _8b: Float64Array;
  _16: Float64Array;
  _12: Float64Array;

  fin: Float64Array;
  fin2: Float64Array;
}

let constants: Orient3dConstants | undefined;

/**
 * build constants for future reuse
 * @returns - the constants
 */
function buildConstants(): Orient3dConstants {
  return {
    bc: vec(4),
    ca: vec(4),
    ab: vec(4),
    at_b: vec(4),
    at_c: vec(4),
    bt_c: vec(4),
    bt_a: vec(4),
    ct_a: vec(4),
    ct_b: vec(4),
    bct: vec(8),
    cat: vec(8),
    abt: vec(8),
    u: vec(4),

    _8: vec(8),
    _8b: vec(8),
    _16: vec(8),
    _12: vec(12),

    fin: vec(192),
    fin2: vec(192),
  };
}

/**
 * add to fin
 * @param finlen - length of array
 * @param alen - length of array
 * @param a - array
 * @param constants - constants
 * @returns - updated finlen
 */
function finadd(
  finlen: number,
  alen: number,
  a: number[] | Float64Array,
  constants: Orient3dConstants,
): number {
  let { fin, fin2 } = constants;
  finlen = predSum(finlen, fin, alen, a, fin2);
  const tmp = fin;
  fin = fin2;
  fin2 = tmp;
  return finlen;
}

/**
 * initialize the tail
 * @param xtail - xtail
 * @param ytail - ytail
 * @param ax - point A.x
 * @param ay - point A.y
 * @param bx - point B.x
 * @param by - point B.y
 * @param a - a values
 * @param b - b values
 * @returns - 4, 1, or 2 based on the length of the tail
 */
function tailinit(
  xtail: number,
  ytail: number,
  ax: number,
  ay: number,
  bx: number,
  by: number,
  a: number[] | Float64Array,
  b: number[] | Float64Array,
) {
  let bvirt, c, ahi, alo, bhi, blo, _i, _j, _k, _0, s1, s0, t1, t0, u3, negate;
  if (xtail === 0) {
    if (ytail === 0) {
      a[0] = 0;
      b[0] = 0;
      return 1;
    } else {
      negate = -ytail;
      s1 = negate * ax;
      c = splitter * negate;
      ahi = c - (c - negate);
      alo = negate - ahi;
      c = splitter * ax;
      bhi = c - (c - ax);
      blo = ax - bhi;
      a[0] = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
      a[1] = s1;
      s1 = ytail * bx;
      c = splitter * ytail;
      ahi = c - (c - ytail);
      alo = ytail - ahi;
      c = splitter * bx;
      bhi = c - (c - bx);
      blo = bx - bhi;
      b[0] = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
      b[1] = s1;
      return 2;
    }
  } else {
    if (ytail === 0) {
      s1 = xtail * ay;
      c = splitter * xtail;
      ahi = c - (c - xtail);
      alo = xtail - ahi;
      c = splitter * ay;
      bhi = c - (c - ay);
      blo = ay - bhi;
      a[0] = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
      a[1] = s1;
      negate = -xtail;
      s1 = negate * by;
      c = splitter * negate;
      ahi = c - (c - negate);
      alo = negate - ahi;
      c = splitter * by;
      bhi = c - (c - by);
      blo = by - bhi;
      b[0] = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
      b[1] = s1;
      return 2;
    } else {
      s1 = xtail * ay;
      c = splitter * xtail;
      ahi = c - (c - xtail);
      alo = xtail - ahi;
      c = splitter * ay;
      bhi = c - (c - ay);
      blo = ay - bhi;
      s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
      t1 = ytail * ax;
      c = splitter * ytail;
      ahi = c - (c - ytail);
      alo = ytail - ahi;
      c = splitter * ax;
      bhi = c - (c - ax);
      blo = ax - bhi;
      t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
      _i = s0 - t0;
      bvirt = s0 - _i;
      a[0] = s0 - (_i + bvirt) + (bvirt - t0);
      _j = s1 + _i;
      bvirt = _j - s1;
      _0 = s1 - (_j - bvirt) + (_i - bvirt);
      _i = _0 - t1;
      bvirt = _0 - _i;
      a[1] = _0 - (_i + bvirt) + (bvirt - t1);
      u3 = _j + _i;
      bvirt = u3 - _j;
      a[2] = _j - (u3 - bvirt) + (_i - bvirt);
      a[3] = u3;
      s1 = ytail * bx;
      c = splitter * ytail;
      ahi = c - (c - ytail);
      alo = ytail - ahi;
      c = splitter * bx;
      bhi = c - (c - bx);
      blo = bx - bhi;
      s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
      t1 = xtail * by;
      c = splitter * xtail;
      ahi = c - (c - xtail);
      alo = xtail - ahi;
      c = splitter * by;
      bhi = c - (c - by);
      blo = by - bhi;
      t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
      _i = s0 - t0;
      bvirt = s0 - _i;
      b[0] = s0 - (_i + bvirt) + (bvirt - t0);
      _j = s1 + _i;
      bvirt = _j - s1;
      _0 = s1 - (_j - bvirt) + (_i - bvirt);
      _i = _0 - t1;
      bvirt = _0 - _i;
      b[1] = _0 - (_i + bvirt) + (bvirt - t1);
      u3 = _j + _i;
      bvirt = u3 - _j;
      b[2] = _j - (u3 - bvirt) + (_i - bvirt);
      b[3] = u3;
      return 4;
    }
  }
}

/**
 * Add to the tail of the sum
 * @param finlen - length of array
 * @param a - a
 * @param b - b
 * @param k - k
 * @param z - z
 * @returns - updated finlen
 */
function tailadd(finlen: number, a: number, b: number, k: number, z: number): number {
  if (constants === undefined) constants = buildConstants();
  const { u } = constants;
  let bvirt, c, ahi, alo, bhi, blo, _i, _j, _k, _0, u3;
  const s1 = a * b;
  c = splitter * a;
  ahi = c - (c - a);
  alo = a - ahi;
  c = splitter * b;
  bhi = c - (c - b);
  blo = b - bhi;
  const s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
  c = splitter * k;
  bhi = c - (c - k);
  blo = k - bhi;
  _i = s0 * k;
  c = splitter * s0;
  ahi = c - (c - s0);
  alo = s0 - ahi;
  u[0] = alo * blo - (_i - ahi * bhi - alo * bhi - ahi * blo);
  _j = s1 * k;
  c = splitter * s1;
  ahi = c - (c - s1);
  alo = s1 - ahi;
  _0 = alo * blo - (_j - ahi * bhi - alo * bhi - ahi * blo);
  _k = _i + _0;
  bvirt = _k - _i;
  u[1] = _i - (_k - bvirt) + (_0 - bvirt);
  u3 = _j + _k;
  u[2] = _k - (u3 - _j);
  u[3] = u3;
  finlen = finadd(finlen, 4, u, constants);
  if (z !== 0) {
    c = splitter * z;
    bhi = c - (c - z);
    blo = z - bhi;
    _i = s0 * z;
    c = splitter * s0;
    ahi = c - (c - s0);
    alo = s0 - ahi;
    u[0] = alo * blo - (_i - ahi * bhi - alo * bhi - ahi * blo);
    _j = s1 * z;
    c = splitter * s1;
    ahi = c - (c - s1);
    alo = s1 - ahi;
    _0 = alo * blo - (_j - ahi * bhi - alo * bhi - ahi * blo);
    _k = _i + _0;
    bvirt = _k - _i;
    u[1] = _i - (_k - bvirt) + (_0 - bvirt);
    u3 = _j + _k;
    u[2] = _k - (u3 - _j);
    u[3] = u3;
    finlen = finadd(finlen, 4, u, constants);
  }

  return finlen;
}

/**
 * Get the 3D orientation of the point-plane of a-b-c via d
 * @param ax - x coordinate of first point
 * @param ay - y coordinate of first point
 * @param az - z coordinate of first point
 * @param bx - x coordinate of second point
 * @param by - y coordinate of second point
 * @param bz - z coordinate of second point
 * @param cx - x coordinate of origin point to create the abc plane
 * @param cy - y coordinate of origin point to create the abc plane
 * @param cz - z coordinate of origin point to create the abc plane
 * @param dx - x coordinate of compare point
 * @param dy - y coordinate of compare point
 * @param dz - z coordinate of compare point
 * @param permanent - if the point-plane of a-b-c via d is permanent
 * @returns - a positive value if the point-plane of a-b-c via d occur in counterclockwise order
 * (c lies to the left of the directed line defined by points a and b).
 * - Returns a negative value if they occur in clockwise order (c lies to the right of the directed line ab).
 * - Returns zero if they are collinear.
 */
function orient3dAdapt(
  ax: number,
  ay: number,
  az: number,
  bx: number,
  by: number,
  bz: number,
  cx: number,
  cy: number,
  cz: number,
  dx: number,
  dy: number,
  dz: number,
  permanent: number,
): number {
  if (constants === undefined) constants = buildConstants();
  const { bc, ca, ab, _8, _8b, _16, _12, fin, at_b, at_c, bt_c, bt_a, ct_a, ct_b, bct, cat, abt } =
    constants;
  let finlen;
  let bvirt, c, ahi, alo, bhi, blo, _i, _j, _k, _0, s1, s0, t1, t0, u3;

  const adx = ax - dx;
  const bdx = bx - dx;
  const cdx = cx - dx;
  const ady = ay - dy;
  const bdy = by - dy;
  const cdy = cy - dy;
  const adz = az - dz;
  const bdz = bz - dz;
  const cdz = cz - dz;

  s1 = bdx * cdy;
  c = splitter * bdx;
  ahi = c - (c - bdx);
  alo = bdx - ahi;
  c = splitter * cdy;
  bhi = c - (c - cdy);
  blo = cdy - bhi;
  s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
  t1 = cdx * bdy;
  c = splitter * cdx;
  ahi = c - (c - cdx);
  alo = cdx - ahi;
  c = splitter * bdy;
  bhi = c - (c - bdy);
  blo = bdy - bhi;
  t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
  _i = s0 - t0;
  bvirt = s0 - _i;
  bc[0] = s0 - (_i + bvirt) + (bvirt - t0);
  _j = s1 + _i;
  bvirt = _j - s1;
  _0 = s1 - (_j - bvirt) + (_i - bvirt);
  _i = _0 - t1;
  bvirt = _0 - _i;
  bc[1] = _0 - (_i + bvirt) + (bvirt - t1);
  u3 = _j + _i;
  bvirt = u3 - _j;
  bc[2] = _j - (u3 - bvirt) + (_i - bvirt);
  bc[3] = u3;
  s1 = cdx * ady;
  c = splitter * cdx;
  ahi = c - (c - cdx);
  alo = cdx - ahi;
  c = splitter * ady;
  bhi = c - (c - ady);
  blo = ady - bhi;
  s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
  t1 = adx * cdy;
  c = splitter * adx;
  ahi = c - (c - adx);
  alo = adx - ahi;
  c = splitter * cdy;
  bhi = c - (c - cdy);
  blo = cdy - bhi;
  t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
  _i = s0 - t0;
  bvirt = s0 - _i;
  ca[0] = s0 - (_i + bvirt) + (bvirt - t0);
  _j = s1 + _i;
  bvirt = _j - s1;
  _0 = s1 - (_j - bvirt) + (_i - bvirt);
  _i = _0 - t1;
  bvirt = _0 - _i;
  ca[1] = _0 - (_i + bvirt) + (bvirt - t1);
  u3 = _j + _i;
  bvirt = u3 - _j;
  ca[2] = _j - (u3 - bvirt) + (_i - bvirt);
  ca[3] = u3;
  s1 = adx * bdy;
  c = splitter * adx;
  ahi = c - (c - adx);
  alo = adx - ahi;
  c = splitter * bdy;
  bhi = c - (c - bdy);
  blo = bdy - bhi;
  s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
  t1 = bdx * ady;
  c = splitter * bdx;
  ahi = c - (c - bdx);
  alo = bdx - ahi;
  c = splitter * ady;
  bhi = c - (c - ady);
  blo = ady - bhi;
  t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
  _i = s0 - t0;
  bvirt = s0 - _i;
  ab[0] = s0 - (_i + bvirt) + (bvirt - t0);
  _j = s1 + _i;
  bvirt = _j - s1;
  _0 = s1 - (_j - bvirt) + (_i - bvirt);
  _i = _0 - t1;
  bvirt = _0 - _i;
  ab[1] = _0 - (_i + bvirt) + (bvirt - t1);
  u3 = _j + _i;
  bvirt = u3 - _j;
  ab[2] = _j - (u3 - bvirt) + (_i - bvirt);
  ab[3] = u3;

  finlen = predSum(
    predSum(scale(4, bc, adz, _8), _8, scale(4, ca, bdz, _8b), _8b, _16),
    _16,
    scale(4, ab, cdz, _8),
    _8,
    fin,
  );

  let det = estimate(finlen, fin);
  let errbound = o3derrboundB * permanent;
  if (det >= errbound || -det >= errbound) {
    return det;
  }

  bvirt = ax - adx;
  const adxtail = ax - (adx + bvirt) + (bvirt - dx);
  bvirt = bx - bdx;
  const bdxtail = bx - (bdx + bvirt) + (bvirt - dx);
  bvirt = cx - cdx;
  const cdxtail = cx - (cdx + bvirt) + (bvirt - dx);
  bvirt = ay - ady;
  const adytail = ay - (ady + bvirt) + (bvirt - dy);
  bvirt = by - bdy;
  const bdytail = by - (bdy + bvirt) + (bvirt - dy);
  bvirt = cy - cdy;
  const cdytail = cy - (cdy + bvirt) + (bvirt - dy);
  bvirt = az - adz;
  const adztail = az - (adz + bvirt) + (bvirt - dz);
  bvirt = bz - bdz;
  const bdztail = bz - (bdz + bvirt) + (bvirt - dz);
  bvirt = cz - cdz;
  const cdztail = cz - (cdz + bvirt) + (bvirt - dz);

  if (
    adxtail === 0 &&
    bdxtail === 0 &&
    cdxtail === 0 &&
    adytail === 0 &&
    bdytail === 0 &&
    cdytail === 0 &&
    adztail === 0 &&
    bdztail === 0 &&
    cdztail === 0
  ) {
    return det;
  }

  errbound = o3derrboundC * permanent + resulterrbound * Math.abs(det);
  det +=
    adz * (bdx * cdytail + cdy * bdxtail - (bdy * cdxtail + cdx * bdytail)) +
    adztail * (bdx * cdy - bdy * cdx) +
    bdz * (cdx * adytail + ady * cdxtail - (cdy * adxtail + adx * cdytail)) +
    bdztail * (cdx * ady - cdy * adx) +
    cdz * (adx * bdytail + bdy * adxtail - (ady * bdxtail + bdx * adytail)) +
    cdztail * (adx * bdy - ady * bdx);
  if (det >= errbound || -det >= errbound) {
    return det;
  }

  const at_len = tailinit(adxtail, adytail, bdx, bdy, cdx, cdy, at_b, at_c);
  const bt_len = tailinit(bdxtail, bdytail, cdx, cdy, adx, ady, bt_c, bt_a);
  const ct_len = tailinit(cdxtail, cdytail, adx, ady, bdx, bdy, ct_a, ct_b);

  const bctlen = predSum(bt_len, bt_c, ct_len, ct_b, bct);
  finlen = finadd(finlen, scale(bctlen, bct, adz, _16), _16, constants);

  const catlen = predSum(ct_len, ct_a, at_len, at_c, cat);
  finlen = finadd(finlen, scale(catlen, cat, bdz, _16), _16, constants);

  const abtlen = predSum(at_len, at_b, bt_len, bt_a, abt);
  finlen = finadd(finlen, scale(abtlen, abt, cdz, _16), _16, constants);

  if (adztail !== 0) {
    finlen = finadd(finlen, scale(4, bc, adztail, _12), _12, constants);
    finlen = finadd(finlen, scale(bctlen, bct, adztail, _16), _16, constants);
  }
  if (bdztail !== 0) {
    finlen = finadd(finlen, scale(4, ca, bdztail, _12), _12, constants);
    finlen = finadd(finlen, scale(catlen, cat, bdztail, _16), _16, constants);
  }
  if (cdztail !== 0) {
    finlen = finadd(finlen, scale(4, ab, cdztail, _12), _12, constants);
    finlen = finadd(finlen, scale(abtlen, abt, cdztail, _16), _16, constants);
  }

  if (adxtail !== 0) {
    if (bdytail !== 0) {
      finlen = tailadd(finlen, adxtail, bdytail, cdz, cdztail);
    }
    if (cdytail !== 0) {
      finlen = tailadd(finlen, -adxtail, cdytail, bdz, bdztail);
    }
  }
  if (bdxtail !== 0) {
    if (cdytail !== 0) {
      finlen = tailadd(finlen, bdxtail, cdytail, adz, adztail);
    }
    if (adytail !== 0) {
      finlen = tailadd(finlen, -bdxtail, adytail, cdz, cdztail);
    }
  }
  if (cdxtail !== 0) {
    if (adytail !== 0) {
      finlen = tailadd(finlen, cdxtail, adytail, bdz, bdztail);
    }
    if (bdytail !== 0) {
      finlen = tailadd(finlen, -cdxtail, bdytail, adz, adztail);
    }
  }

  return fin[finlen - 1];
}

/**
 * Get the orientation of a tetrahedron
 * @param ax - x coordinate of first point
 * @param ay - y coordinate of first point
 * @param az - z coordinate of first point
 * @param bx - x coordinate of second point
 * @param by - y coordinate of second point
 * @param bz - z coordinate of second point
 * @param cx - x coordinate of origin point to create the abc plane
 * @param cy - y coordinate of origin point to create the abc plane
 * @param cz - z coordinate of origin point to create the abc plane
 * @param dx - x coordinate of compare point
 * @param dy - y coordinate of compare point
 * @param dz - z coordinate of compare point
 * @returns - a positive value if the point-plane of a-b-c via d occur in counterclockwise order
 * (c lies to the left of the directed line defined by points a and b).
 * - Returns a negative value if they occur in clockwise order (c lies to the right of the directed line ab).
 * - Returns zero if they are collinear.
 */
export function orient3d(
  ax: number,
  ay: number,
  az: number,
  bx: number,
  by: number,
  bz: number,
  cx: number,
  cy: number,
  cz: number,
  dx: number,
  dy: number,
  dz: number,
): number {
  const adx = ax - dx;
  const bdx = bx - dx;
  const cdx = cx - dx;
  const ady = ay - dy;
  const bdy = by - dy;
  const cdy = cy - dy;
  const adz = az - dz;
  const bdz = bz - dz;
  const cdz = cz - dz;

  const bdxcdy = bdx * cdy;
  const cdxbdy = cdx * bdy;

  const cdxady = cdx * ady;
  const adxcdy = adx * cdy;

  const adxbdy = adx * bdy;
  const bdxady = bdx * ady;

  const det = adz * (bdxcdy - cdxbdy) + bdz * (cdxady - adxcdy) + cdz * (adxbdy - bdxady);

  const permanent =
    (Math.abs(bdxcdy) + Math.abs(cdxbdy)) * Math.abs(adz) +
    (Math.abs(cdxady) + Math.abs(adxcdy)) * Math.abs(bdz) +
    (Math.abs(adxbdy) + Math.abs(bdxady)) * Math.abs(cdz);

  const errbound = o3derrboundA * permanent;
  if (det > errbound || -det > errbound) {
    return det;
  }

  return orient3dAdapt(ax, ay, az, bx, by, bz, cx, cy, cz, dx, dy, dz, permanent);
}

/**
 * Find the orientation of a point relative to a vector a-b plane relative to the origin
 * @param a - first point
 * @param b - second point
 * @param c - comparison point
 * @returns - a positive value if the point-plane of a-b via c occur in counterclockwise order
 * (c lies to the left of the directed line defined by points a and b).
 * - Returns a negative value if they occur in clockwise order (c lies to the right of the directed line ab).
 * - Returns zero if they are collinear.
 */
export function orient3dfastVector(a: VectorPoint, b: VectorPoint, c: VectorPoint): number {
  return orient3dfast(a.x, a.y, a.z ?? 0, b.x, b.y, b.z ?? 0, 0, 0, 0, c.x, c.y, c.z ?? 0);
}

/**
 * @param ax - x coordinate of first point
 * @param ay - y coordinate of first point
 * @param az - z coordinate of first point
 * @param bx - x coordinate of second point
 * @param by - y coordinate of second point
 * @param bz - z coordinate of second point
 * @param cx - x coordinate of third point
 * @param cy - y coordinate of third point
 * @param cz - z coordinate of third point
 * @param dx - x coordinate of compare point
 * @param dy - y coordinate of compare point
 * @param dz - z coordinate of compare point
 * @returns - a positive value if the point-plane of a-b-c via d occur in counterclockwise order
 * (c lies to the left of the directed line defined by points a and b).
 * - Returns a negative value if they occur in clockwise order (c lies to the right of the directed line ab).
 * - Returns zero if they are collinear.
 */
export function orient3dfast(
  ax: number,
  ay: number,
  az: number,
  bx: number,
  by: number,
  bz: number,
  cx: number,
  cy: number,
  cz: number,
  dx: number,
  dy: number,
  dz: number,
): number {
  const adx = ax - dx;
  const bdx = bx - dx;
  const cdx = cx - dx;
  const ady = ay - dy;
  const bdy = by - dy;
  const cdy = cy - dy;
  const adz = az - dz;
  const bdz = bz - dz;
  const cdz = cz - dz;

  return (
    adx * (bdy * cdz - bdz * cdy) + bdx * (cdy * adz - cdz * ady) + cdx * (ady * bdz - adz * bdy)
  );
}
