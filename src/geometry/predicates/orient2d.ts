import { epsilon, estimate, predSum, resulterrbound, splitter, vec } from './util.js';

const ccwerrboundA = (3 + 16 * epsilon) * epsilon;
const ccwerrboundB = (2 + 12 * epsilon) * epsilon;
const ccwerrboundC = (9 + 64 * epsilon) * epsilon * epsilon;

const B = vec(4);
const C1 = vec(8);
const C2 = vec(12);
const D = vec(16);
const u = vec(4);

/**
 * @param ax - x coordinate of first point
 * @param ay - y coordinate of first point
 * @param bx - x coordinate of second point
 * @param by - y coordinate of second point
 * @param cx - x coordinate of comparison point
 * @param cy - y coordinate of comparison point
 * @param detsum - sum of the determinants of all intermediate cross products
 * @returns - a negative value if the points a, b, and c occur in counterclockwise order
 * (c lies to the left of the directed line defined by points a and b).
 * - Returns a positive value if they occur in clockwise order (c lies to the right of the directed line ab).
 * - Returns zero if they are collinear.
 */
function orient2dadapt(
  ax: number,
  ay: number,
  bx: number,
  by: number,
  cx: number,
  cy: number,
  detsum: number,
): number {
  let bvirt, c, ahi, alo, bhi, blo, _i, _j, _0, s1, s0, t1, t0, u3;

  const acx = ax - cx;
  const bcx = bx - cx;
  const acy = ay - cy;
  const bcy = by - cy;

  s1 = acx * bcy;
  c = splitter * acx;
  ahi = c - (c - acx);
  alo = acx - ahi;
  c = splitter * bcy;
  bhi = c - (c - bcy);
  blo = bcy - bhi;
  s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
  t1 = acy * bcx;
  c = splitter * acy;
  ahi = c - (c - acy);
  alo = acy - ahi;
  c = splitter * bcx;
  bhi = c - (c - bcx);
  blo = bcx - bhi;
  t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
  _i = s0 - t0;
  bvirt = s0 - _i;
  B[0] = s0 - (_i + bvirt) + (bvirt - t0);
  _j = s1 + _i;
  bvirt = _j - s1;
  _0 = s1 - (_j - bvirt) + (_i - bvirt);
  _i = _0 - t1;
  bvirt = _0 - _i;
  B[1] = _0 - (_i + bvirt) + (bvirt - t1);
  u3 = _j + _i;
  bvirt = u3 - _j;
  B[2] = _j - (u3 - bvirt) + (_i - bvirt);
  B[3] = u3;

  let det = estimate(4, B);
  let errbound = ccwerrboundB * detsum;
  if (det >= errbound || -det >= errbound) {
    return det;
  }

  bvirt = ax - acx;
  const acxtail = ax - (acx + bvirt) + (bvirt - cx);
  bvirt = bx - bcx;
  const bcxtail = bx - (bcx + bvirt) + (bvirt - cx);
  bvirt = ay - acy;
  const acytail = ay - (acy + bvirt) + (bvirt - cy);
  bvirt = by - bcy;
  const bcytail = by - (bcy + bvirt) + (bvirt - cy);

  if (acxtail === 0 && acytail === 0 && bcxtail === 0 && bcytail === 0) {
    return det;
  }

  errbound = ccwerrboundC * detsum + resulterrbound * Math.abs(det);
  det += acx * bcytail + bcy * acxtail - (acy * bcxtail + bcx * acytail);
  if (det >= errbound || -det >= errbound) return det;

  s1 = acxtail * bcy;
  c = splitter * acxtail;
  ahi = c - (c - acxtail);
  alo = acxtail - ahi;
  c = splitter * bcy;
  bhi = c - (c - bcy);
  blo = bcy - bhi;
  s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
  t1 = acytail * bcx;
  c = splitter * acytail;
  ahi = c - (c - acytail);
  alo = acytail - ahi;
  c = splitter * bcx;
  bhi = c - (c - bcx);
  blo = bcx - bhi;
  t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
  _i = s0 - t0;
  bvirt = s0 - _i;
  u[0] = s0 - (_i + bvirt) + (bvirt - t0);
  _j = s1 + _i;
  bvirt = _j - s1;
  _0 = s1 - (_j - bvirt) + (_i - bvirt);
  _i = _0 - t1;
  bvirt = _0 - _i;
  u[1] = _0 - (_i + bvirt) + (bvirt - t1);
  u3 = _j + _i;
  bvirt = u3 - _j;
  u[2] = _j - (u3 - bvirt) + (_i - bvirt);
  u[3] = u3;
  const C1len = predSum(4, B, 4, u, C1);

  s1 = acx * bcytail;
  c = splitter * acx;
  ahi = c - (c - acx);
  alo = acx - ahi;
  c = splitter * bcytail;
  bhi = c - (c - bcytail);
  blo = bcytail - bhi;
  s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
  t1 = acy * bcxtail;
  c = splitter * acy;
  ahi = c - (c - acy);
  alo = acy - ahi;
  c = splitter * bcxtail;
  bhi = c - (c - bcxtail);
  blo = bcxtail - bhi;
  t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
  _i = s0 - t0;
  bvirt = s0 - _i;
  u[0] = s0 - (_i + bvirt) + (bvirt - t0);
  _j = s1 + _i;
  bvirt = _j - s1;
  _0 = s1 - (_j - bvirt) + (_i - bvirt);
  _i = _0 - t1;
  bvirt = _0 - _i;
  u[1] = _0 - (_i + bvirt) + (bvirt - t1);
  u3 = _j + _i;
  bvirt = u3 - _j;
  u[2] = _j - (u3 - bvirt) + (_i - bvirt);
  u[3] = u3;
  const C2len = predSum(C1len, C1, 4, u, C2);

  s1 = acxtail * bcytail;
  c = splitter * acxtail;
  ahi = c - (c - acxtail);
  alo = acxtail - ahi;
  c = splitter * bcytail;
  bhi = c - (c - bcytail);
  blo = bcytail - bhi;
  s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
  t1 = acytail * bcxtail;
  c = splitter * acytail;
  ahi = c - (c - acytail);
  alo = acytail - ahi;
  c = splitter * bcxtail;
  bhi = c - (c - bcxtail);
  blo = bcxtail - bhi;
  t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
  _i = s0 - t0;
  bvirt = s0 - _i;
  u[0] = s0 - (_i + bvirt) + (bvirt - t0);
  _j = s1 + _i;
  bvirt = _j - s1;
  _0 = s1 - (_j - bvirt) + (_i - bvirt);
  _i = _0 - t1;
  bvirt = _0 - _i;
  u[1] = _0 - (_i + bvirt) + (bvirt - t1);
  u3 = _j + _i;
  bvirt = u3 - _j;
  u[2] = _j - (u3 - bvirt) + (_i - bvirt);
  u[3] = u3;
  const Dlen = predSum(C2len, C2, 4, u, D);

  return D[Dlen - 1];
}

/**
 * @param ax - x coordinate of first point
 * @param ay - y coordinate of first point
 * @param bx - x coordinate of second point
 * @param by - y coordinate of second point
 * @param cx - x coordinate of comparison point
 * @param cy - y coordinate of comparison point
 * @returns - a positive value if the points a, b, and c occur in counterclockwise order
 * (c lies to the left of the directed line defined by points a and b).
 * - Returns a negative value if they occur in clockwise order (c lies to the right of the directed line ab).
 * - Returns zero if they are collinear.
 */
export function orient2d(
  ax: number,
  ay: number,
  bx: number,
  by: number,
  cx: number,
  cy: number,
): number {
  const { abs } = Math;
  const detleft = (ay - cy) * (bx - cx);
  const detright = (ax - cx) * (by - cy);
  const det = detleft - detright;

  const detsum = abs(detleft + detright);
  if (abs(det) >= ccwerrboundA * detsum) return det;

  return -orient2dadapt(ax, ay, bx, by, cx, cy, detsum);
}

/**
 * @param ax - x coordinate of first point
 * @param ay - y coordinate of first point
 * @param bx - x coordinate of second point
 * @param by - y coordinate of second point
 * @param cx - x coordinate of comparison point
 * @param cy - y coordinate of comparison point
 * @returns - a positive value if the points a, b, and c occur in counterclockwise order
 * (c lies to the left of the directed line defined by points a and b).
 * - Returns a negative value if they occur in clockwise order (c lies to the right of the directed line ab).
 * - Returns zero if they are collinear.
 */
export function orient2dfast(
  ax: number,
  ay: number,
  bx: number,
  by: number,
  cx: number,
  cy: number,
): number {
  return (ay - cy) * (bx - cx) - (ax - cx) * (by - cy);
}
