import { estimate, predSum, predSumThree, resulterrbound, scale, splitter, vec } from './util';

const iccerrboundA = 1.1102230246251577e-15; // (10 + 96 * epsilon) * epsilon;
const iccerrboundB = 4.440892098500632e-16; // (4 + 48 * epsilon) * epsilon;
const iccerrboundC = 5.423418723394464e-31; // (44 + 576 * epsilon) * epsilon * epsilon;

/**
 * Constants for incircle
 */
export interface InCircleConstants {
  bc: Float64Array;
  ca: Float64Array;
  ab: Float64Array;
  aa: Float64Array;
  bb: Float64Array;
  cc: Float64Array;
  u: Float64Array;
  v: Float64Array;
  axtbc: Float64Array;
  aytbc: Float64Array;
  bxtca: Float64Array;
  bytca: Float64Array;
  cxtab: Float64Array;
  cytab: Float64Array;
  abt: Float64Array;
  bct: Float64Array;
  cat: Float64Array;
  abtt: Float64Array;
  bctt: Float64Array;
  catt: Float64Array;

  _8: Float64Array;
  _16: Float64Array;
  _16b: Float64Array;
  _16c: Float64Array;
  _32: Float64Array;
  _32b: Float64Array;
  _48: Float64Array;
  _64: Float64Array;

  fin: Float64Array;
  fin2: Float64Array;
}

let constants: undefined | InCircleConstants;

/**
 * build constants for future reuse
 * @returns - the constants
 */
function buildConstants(): InCircleConstants {
  return {
    bc: vec(4),
    ca: vec(4),
    ab: vec(4),
    aa: vec(4),
    bb: vec(4),
    cc: vec(4),
    u: vec(4),
    v: vec(4),
    axtbc: vec(8),
    aytbc: vec(8),
    bxtca: vec(8),
    bytca: vec(8),
    cxtab: vec(8),
    cytab: vec(8),
    abt: vec(8),
    bct: vec(8),
    cat: vec(8),
    abtt: vec(4),
    bctt: vec(4),
    catt: vec(4),

    _8: vec(8),
    _16: vec(16),
    _16b: vec(16),
    _16c: vec(16),
    _32: vec(32),
    _32b: vec(32),
    _48: vec(48),
    _64: vec(64),

    fin: vec(1152),
    fin2: vec(1152),
  };
}

/**
 * @param finlen - length of array
 * @param a - index
 * @param alen - array
 * @returns - updated finlen
 */
function finadd(finlen: number, a: number, alen: number[] | Float64Array): number {
  if (constants === undefined) constants = buildConstants();
  finlen = predSum(finlen, constants.fin, a, alen, constants.fin2);
  const tmp = constants.fin;
  constants.fin = constants.fin2;
  constants.fin2 = tmp;
  return finlen;
}

/**
 * @param ax - x coordinate of first point
 * @param ay - y coordinate of first point
 * @param bx - x coordinate of second point
 * @param by - y coordinate of second point
 * @param cx - x coordinate of third point
 * @param cy - y coordinate of third point
 * @param dx - x coordinate of comparison point
 * @param dy - y coordinate of comparison point
 * @param permanent - a value between 0 and 1
 * @returns - a positive value if the point d lies outside the circle passing through a, b, and c.
 * - a negative value if it lies inside.
 * - zero if the four points are cocircular.
 */
function incircleadapt(
  ax: number,
  ay: number,
  bx: number,
  by: number,
  cx: number,
  cy: number,
  dx: number,
  dy: number,
  permanent: number,
): number {
  if (constants === undefined) constants = buildConstants();
  const {
    bc,
    ca,
    ab,
    aa,
    bb,
    cc,
    u,
    v,
    axtbc,
    aytbc,
    bxtca,
    bytca,
    cxtab,
    cytab,
    abt,
    bct,
    cat,
    abtt,
    bctt,
    catt,

    _8,
    _16,
    _16b,
    _16c,
    _32,
    _32b,
    _48,
    _64,

    fin,
  } = constants;
  let finlen;
  let axtbclen = 0,
    aytbclen = 0,
    bxtcalen = 0,
    bytcalen = 0,
    cxtablen = 0,
    cytablen = 0;
  let abtlen, bctlen, catlen;
  let abttlen, bcttlen, cattlen;
  let n1, n0;

  let bvirt, c, ahi, alo, bhi, blo, _i, _j, _0, s1, s0, t1, t0, u3;

  const adx = ax - dx;
  const bdx = bx - dx;
  const cdx = cx - dx;
  const ady = ay - dy;
  const bdy = by - dy;
  const cdy = cy - dy;

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
    predSum(
      predSum(
        scale(scale(4, bc, adx, _8), _8, adx, _16),
        _16,
        scale(scale(4, bc, ady, _8), _8, ady, _16b),
        _16b,
        _32,
      ),
      _32,
      predSum(
        scale(scale(4, ca, bdx, _8), _8, bdx, _16),
        _16,
        scale(scale(4, ca, bdy, _8), _8, bdy, _16b),
        _16b,
        _32b,
      ),
      _32b,
      _64,
    ),
    _64,
    predSum(
      scale(scale(4, ab, cdx, _8), _8, cdx, _16),
      _16,
      scale(scale(4, ab, cdy, _8), _8, cdy, _16b),
      _16b,
      _32,
    ),
    _32,
    fin,
  );

  let det = estimate(finlen, fin);
  let errbound = iccerrboundB * permanent;
  if (det >= errbound || -det >= errbound) {
    return det;
  }

  bvirt = ax - adx;
  const adxtail = ax - (adx + bvirt) + (bvirt - dx);
  bvirt = ay - ady;
  const adytail = ay - (ady + bvirt) + (bvirt - dy);
  bvirt = bx - bdx;
  const bdxtail = bx - (bdx + bvirt) + (bvirt - dx);
  bvirt = by - bdy;
  const bdytail = by - (bdy + bvirt) + (bvirt - dy);
  bvirt = cx - cdx;
  const cdxtail = cx - (cdx + bvirt) + (bvirt - dx);
  bvirt = cy - cdy;
  const cdytail = cy - (cdy + bvirt) + (bvirt - dy);
  if (
    adxtail === 0 &&
    bdxtail === 0 &&
    cdxtail === 0 &&
    adytail === 0 &&
    bdytail === 0 &&
    cdytail === 0
  ) {
    return det;
  }

  errbound = iccerrboundC * permanent + resulterrbound * Math.abs(det);
  det +=
    (adx * adx + ady * ady) * (bdx * cdytail + cdy * bdxtail - (bdy * cdxtail + cdx * bdytail)) +
    2 * (adx * adxtail + ady * adytail) * (bdx * cdy - bdy * cdx) +
    ((bdx * bdx + bdy * bdy) * (cdx * adytail + ady * cdxtail - (cdy * adxtail + adx * cdytail)) +
      2 * (bdx * bdxtail + bdy * bdytail) * (cdx * ady - cdy * adx)) +
    ((cdx * cdx + cdy * cdy) * (adx * bdytail + bdy * adxtail - (ady * bdxtail + bdx * adytail)) +
      2 * (cdx * cdxtail + cdy * cdytail) * (adx * bdy - ady * bdx));

  if (det >= errbound || -det >= errbound) {
    return det;
  }

  if (bdxtail !== 0 || bdytail !== 0 || cdxtail !== 0 || cdytail !== 0) {
    s1 = adx * adx;
    c = splitter * adx;
    ahi = c - (c - adx);
    alo = adx - ahi;
    s0 = alo * alo - (s1 - ahi * ahi - (ahi + ahi) * alo);
    t1 = ady * ady;
    c = splitter * ady;
    ahi = c - (c - ady);
    alo = ady - ahi;
    t0 = alo * alo - (t1 - ahi * ahi - (ahi + ahi) * alo);
    _i = s0 + t0;
    bvirt = _i - s0;
    aa[0] = s0 - (_i - bvirt) + (t0 - bvirt);
    _j = s1 + _i;
    bvirt = _j - s1;
    _0 = s1 - (_j - bvirt) + (_i - bvirt);
    _i = _0 + t1;
    bvirt = _i - _0;
    aa[1] = _0 - (_i - bvirt) + (t1 - bvirt);
    u3 = _j + _i;
    bvirt = u3 - _j;
    aa[2] = _j - (u3 - bvirt) + (_i - bvirt);
    aa[3] = u3;
  }
  if (cdxtail !== 0 || cdytail !== 0 || adxtail !== 0 || adytail !== 0) {
    s1 = bdx * bdx;
    c = splitter * bdx;
    ahi = c - (c - bdx);
    alo = bdx - ahi;
    s0 = alo * alo - (s1 - ahi * ahi - (ahi + ahi) * alo);
    t1 = bdy * bdy;
    c = splitter * bdy;
    ahi = c - (c - bdy);
    alo = bdy - ahi;
    t0 = alo * alo - (t1 - ahi * ahi - (ahi + ahi) * alo);
    _i = s0 + t0;
    bvirt = _i - s0;
    bb[0] = s0 - (_i - bvirt) + (t0 - bvirt);
    _j = s1 + _i;
    bvirt = _j - s1;
    _0 = s1 - (_j - bvirt) + (_i - bvirt);
    _i = _0 + t1;
    bvirt = _i - _0;
    bb[1] = _0 - (_i - bvirt) + (t1 - bvirt);
    u3 = _j + _i;
    bvirt = u3 - _j;
    bb[2] = _j - (u3 - bvirt) + (_i - bvirt);
    bb[3] = u3;
  }
  if (adxtail !== 0 || adytail !== 0 || bdxtail !== 0 || bdytail !== 0) {
    s1 = cdx * cdx;
    c = splitter * cdx;
    ahi = c - (c - cdx);
    alo = cdx - ahi;
    s0 = alo * alo - (s1 - ahi * ahi - (ahi + ahi) * alo);
    t1 = cdy * cdy;
    c = splitter * cdy;
    ahi = c - (c - cdy);
    alo = cdy - ahi;
    t0 = alo * alo - (t1 - ahi * ahi - (ahi + ahi) * alo);
    _i = s0 + t0;
    bvirt = _i - s0;
    cc[0] = s0 - (_i - bvirt) + (t0 - bvirt);
    _j = s1 + _i;
    bvirt = _j - s1;
    _0 = s1 - (_j - bvirt) + (_i - bvirt);
    _i = _0 + t1;
    bvirt = _i - _0;
    cc[1] = _0 - (_i - bvirt) + (t1 - bvirt);
    u3 = _j + _i;
    bvirt = u3 - _j;
    cc[2] = _j - (u3 - bvirt) + (_i - bvirt);
    cc[3] = u3;
  }

  if (adxtail !== 0) {
    axtbclen = scale(4, bc, adxtail, axtbc);
    finlen = finadd(
      finlen,
      predSumThree(
        scale(axtbclen, axtbc, 2 * adx, _16),
        _16,
        scale(scale(4, cc, adxtail, _8), _8, bdy, _16b),
        _16b,
        scale(scale(4, bb, adxtail, _8), _8, -cdy, _16c),
        _16c,
        _32,
        _48,
      ),
      _48,
    );
  }
  if (adytail !== 0) {
    aytbclen = scale(4, bc, adytail, aytbc);
    finlen = finadd(
      finlen,
      predSumThree(
        scale(aytbclen, aytbc, 2 * ady, _16),
        _16,
        scale(scale(4, bb, adytail, _8), _8, cdx, _16b),
        _16b,
        scale(scale(4, cc, adytail, _8), _8, -bdx, _16c),
        _16c,
        _32,
        _48,
      ),
      _48,
    );
  }
  if (bdxtail !== 0) {
    bxtcalen = scale(4, ca, bdxtail, bxtca);
    finlen = finadd(
      finlen,
      predSumThree(
        scale(bxtcalen, bxtca, 2 * bdx, _16),
        _16,
        scale(scale(4, aa, bdxtail, _8), _8, cdy, _16b),
        _16b,
        scale(scale(4, cc, bdxtail, _8), _8, -ady, _16c),
        _16c,
        _32,
        _48,
      ),
      _48,
    );
  }
  if (bdytail !== 0) {
    bytcalen = scale(4, ca, bdytail, bytca);
    finlen = finadd(
      finlen,
      predSumThree(
        scale(bytcalen, bytca, 2 * bdy, _16),
        _16,
        scale(scale(4, cc, bdytail, _8), _8, adx, _16b),
        _16b,
        scale(scale(4, aa, bdytail, _8), _8, -cdx, _16c),
        _16c,
        _32,
        _48,
      ),
      _48,
    );
  }
  if (cdxtail !== 0) {
    cxtablen = scale(4, ab, cdxtail, cxtab);
    finlen = finadd(
      finlen,
      predSumThree(
        scale(cxtablen, cxtab, 2 * cdx, _16),
        _16,
        scale(scale(4, bb, cdxtail, _8), _8, ady, _16b),
        _16b,
        scale(scale(4, aa, cdxtail, _8), _8, -bdy, _16c),
        _16c,
        _32,
        _48,
      ),
      _48,
    );
  }
  if (cdytail !== 0) {
    cytablen = scale(4, ab, cdytail, cytab);
    finlen = finadd(
      finlen,
      predSumThree(
        scale(cytablen, cytab, 2 * cdy, _16),
        _16,
        scale(scale(4, aa, cdytail, _8), _8, bdx, _16b),
        _16b,
        scale(scale(4, bb, cdytail, _8), _8, -adx, _16c),
        _16c,
        _32,
        _48,
      ),
      _48,
    );
  }

  if (adxtail !== 0 || adytail !== 0) {
    if (bdxtail !== 0 || bdytail !== 0 || cdxtail !== 0 || cdytail !== 0) {
      s1 = bdxtail * cdy;
      c = splitter * bdxtail;
      ahi = c - (c - bdxtail);
      alo = bdxtail - ahi;
      c = splitter * cdy;
      bhi = c - (c - cdy);
      blo = cdy - bhi;
      s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
      t1 = bdx * cdytail;
      c = splitter * bdx;
      ahi = c - (c - bdx);
      alo = bdx - ahi;
      c = splitter * cdytail;
      bhi = c - (c - cdytail);
      blo = cdytail - bhi;
      t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
      _i = s0 + t0;
      bvirt = _i - s0;
      u[0] = s0 - (_i - bvirt) + (t0 - bvirt);
      _j = s1 + _i;
      bvirt = _j - s1;
      _0 = s1 - (_j - bvirt) + (_i - bvirt);
      _i = _0 + t1;
      bvirt = _i - _0;
      u[1] = _0 - (_i - bvirt) + (t1 - bvirt);
      u3 = _j + _i;
      bvirt = u3 - _j;
      u[2] = _j - (u3 - bvirt) + (_i - bvirt);
      u[3] = u3;
      s1 = cdxtail * -bdy;
      c = splitter * cdxtail;
      ahi = c - (c - cdxtail);
      alo = cdxtail - ahi;
      c = splitter * -bdy;
      bhi = c - (c - -bdy);
      blo = -bdy - bhi;
      s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
      t1 = cdx * -bdytail;
      c = splitter * cdx;
      ahi = c - (c - cdx);
      alo = cdx - ahi;
      c = splitter * -bdytail;
      bhi = c - (c - -bdytail);
      blo = -bdytail - bhi;
      t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
      _i = s0 + t0;
      bvirt = _i - s0;
      v[0] = s0 - (_i - bvirt) + (t0 - bvirt);
      _j = s1 + _i;
      bvirt = _j - s1;
      _0 = s1 - (_j - bvirt) + (_i - bvirt);
      _i = _0 + t1;
      bvirt = _i - _0;
      v[1] = _0 - (_i - bvirt) + (t1 - bvirt);
      u3 = _j + _i;
      bvirt = u3 - _j;
      v[2] = _j - (u3 - bvirt) + (_i - bvirt);
      v[3] = u3;
      bctlen = predSum(4, u, 4, v, bct);
      s1 = bdxtail * cdytail;
      c = splitter * bdxtail;
      ahi = c - (c - bdxtail);
      alo = bdxtail - ahi;
      c = splitter * cdytail;
      bhi = c - (c - cdytail);
      blo = cdytail - bhi;
      s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
      t1 = cdxtail * bdytail;
      c = splitter * cdxtail;
      ahi = c - (c - cdxtail);
      alo = cdxtail - ahi;
      c = splitter * bdytail;
      bhi = c - (c - bdytail);
      blo = bdytail - bhi;
      t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
      _i = s0 - t0;
      bvirt = s0 - _i;
      bctt[0] = s0 - (_i + bvirt) + (bvirt - t0);
      _j = s1 + _i;
      bvirt = _j - s1;
      _0 = s1 - (_j - bvirt) + (_i - bvirt);
      _i = _0 - t1;
      bvirt = _0 - _i;
      bctt[1] = _0 - (_i + bvirt) + (bvirt - t1);
      u3 = _j + _i;
      bvirt = u3 - _j;
      bctt[2] = _j - (u3 - bvirt) + (_i - bvirt);
      bctt[3] = u3;
      bcttlen = 4;
    } else {
      bct[0] = 0;
      bctlen = 1;
      bctt[0] = 0;
      bcttlen = 1;
    }
    if (adxtail !== 0) {
      const len = scale(bctlen, bct, adxtail, _16c);
      finlen = finadd(
        finlen,
        predSum(
          scale(axtbclen, axtbc, adxtail, _16),
          _16,
          scale(len, _16c, 2 * adx, _32),
          _32,
          _48,
        ),
        _48,
      );

      const len2 = scale(bcttlen, bctt, adxtail, _8);
      finlen = finadd(
        finlen,
        predSumThree(
          scale(len2, _8, 2 * adx, _16),
          _16,
          scale(len2, _8, adxtail, _16b),
          _16b,
          scale(len, _16c, adxtail, _32),
          _32,
          _32b,
          _64,
        ),
        _64,
      );

      if (bdytail !== 0) {
        finlen = finadd(finlen, scale(scale(4, cc, adxtail, _8), _8, bdytail, _16), _16);
      }
      if (cdytail !== 0) {
        finlen = finadd(finlen, scale(scale(4, bb, -adxtail, _8), _8, cdytail, _16), _16);
      }
    }
    if (adytail !== 0) {
      const len = scale(bctlen, bct, adytail, _16c);
      finlen = finadd(
        finlen,
        predSum(
          scale(aytbclen, aytbc, adytail, _16),
          _16,
          scale(len, _16c, 2 * ady, _32),
          _32,
          _48,
        ),
        _48,
      );

      const len2 = scale(bcttlen, bctt, adytail, _8);
      finlen = finadd(
        finlen,
        predSumThree(
          scale(len2, _8, 2 * ady, _16),
          _16,
          scale(len2, _8, adytail, _16b),
          _16b,
          scale(len, _16c, adytail, _32),
          _32,
          _32b,
          _64,
        ),
        _64,
      );
    }
  }
  if (bdxtail !== 0 || bdytail !== 0) {
    if (cdxtail !== 0 || cdytail !== 0 || adxtail !== 0 || adytail !== 0) {
      s1 = cdxtail * ady;
      c = splitter * cdxtail;
      ahi = c - (c - cdxtail);
      alo = cdxtail - ahi;
      c = splitter * ady;
      bhi = c - (c - ady);
      blo = ady - bhi;
      s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
      t1 = cdx * adytail;
      c = splitter * cdx;
      ahi = c - (c - cdx);
      alo = cdx - ahi;
      c = splitter * adytail;
      bhi = c - (c - adytail);
      blo = adytail - bhi;
      t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
      _i = s0 + t0;
      bvirt = _i - s0;
      u[0] = s0 - (_i - bvirt) + (t0 - bvirt);
      _j = s1 + _i;
      bvirt = _j - s1;
      _0 = s1 - (_j - bvirt) + (_i - bvirt);
      _i = _0 + t1;
      bvirt = _i - _0;
      u[1] = _0 - (_i - bvirt) + (t1 - bvirt);
      u3 = _j + _i;
      bvirt = u3 - _j;
      u[2] = _j - (u3 - bvirt) + (_i - bvirt);
      u[3] = u3;
      n1 = -cdy;
      n0 = -cdytail;
      s1 = adxtail * n1;
      c = splitter * adxtail;
      ahi = c - (c - adxtail);
      alo = adxtail - ahi;
      c = splitter * n1;
      bhi = c - (c - n1);
      blo = n1 - bhi;
      s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
      t1 = adx * n0;
      c = splitter * adx;
      ahi = c - (c - adx);
      alo = adx - ahi;
      c = splitter * n0;
      bhi = c - (c - n0);
      blo = n0 - bhi;
      t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
      _i = s0 + t0;
      bvirt = _i - s0;
      v[0] = s0 - (_i - bvirt) + (t0 - bvirt);
      _j = s1 + _i;
      bvirt = _j - s1;
      _0 = s1 - (_j - bvirt) + (_i - bvirt);
      _i = _0 + t1;
      bvirt = _i - _0;
      v[1] = _0 - (_i - bvirt) + (t1 - bvirt);
      u3 = _j + _i;
      bvirt = u3 - _j;
      v[2] = _j - (u3 - bvirt) + (_i - bvirt);
      v[3] = u3;
      catlen = predSum(4, u, 4, v, cat);
      s1 = cdxtail * adytail;
      c = splitter * cdxtail;
      ahi = c - (c - cdxtail);
      alo = cdxtail - ahi;
      c = splitter * adytail;
      bhi = c - (c - adytail);
      blo = adytail - bhi;
      s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
      t1 = adxtail * cdytail;
      c = splitter * adxtail;
      ahi = c - (c - adxtail);
      alo = adxtail - ahi;
      c = splitter * cdytail;
      bhi = c - (c - cdytail);
      blo = cdytail - bhi;
      t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
      _i = s0 - t0;
      bvirt = s0 - _i;
      catt[0] = s0 - (_i + bvirt) + (bvirt - t0);
      _j = s1 + _i;
      bvirt = _j - s1;
      _0 = s1 - (_j - bvirt) + (_i - bvirt);
      _i = _0 - t1;
      bvirt = _0 - _i;
      catt[1] = _0 - (_i + bvirt) + (bvirt - t1);
      u3 = _j + _i;
      bvirt = u3 - _j;
      catt[2] = _j - (u3 - bvirt) + (_i - bvirt);
      catt[3] = u3;
      cattlen = 4;
    } else {
      cat[0] = 0;
      catlen = 1;
      catt[0] = 0;
      cattlen = 1;
    }
    if (bdxtail !== 0) {
      const len = scale(catlen, cat, bdxtail, _16c);
      finlen = finadd(
        finlen,
        predSum(
          scale(bxtcalen, bxtca, bdxtail, _16),
          _16,
          scale(len, _16c, 2 * bdx, _32),
          _32,
          _48,
        ),
        _48,
      );

      const len2 = scale(cattlen, catt, bdxtail, _8);
      finlen = finadd(
        finlen,
        predSumThree(
          scale(len2, _8, 2 * bdx, _16),
          _16,
          scale(len2, _8, bdxtail, _16b),
          _16b,
          scale(len, _16c, bdxtail, _32),
          _32,
          _32b,
          _64,
        ),
        _64,
      );

      if (cdytail !== 0) {
        finlen = finadd(finlen, scale(scale(4, aa, bdxtail, _8), _8, cdytail, _16), _16);
      }
      if (adytail !== 0) {
        finlen = finadd(finlen, scale(scale(4, cc, -bdxtail, _8), _8, adytail, _16), _16);
      }
    }
    if (bdytail !== 0) {
      const len = scale(catlen, cat, bdytail, _16c);
      finlen = finadd(
        finlen,
        predSum(
          scale(bytcalen, bytca, bdytail, _16),
          _16,
          scale(len, _16c, 2 * bdy, _32),
          _32,
          _48,
        ),
        _48,
      );

      const len2 = scale(cattlen, catt, bdytail, _8);
      finlen = finadd(
        finlen,
        predSumThree(
          scale(len2, _8, 2 * bdy, _16),
          _16,
          scale(len2, _8, bdytail, _16b),
          _16b,
          scale(len, _16c, bdytail, _32),
          _32,
          _32b,
          _64,
        ),
        _64,
      );
    }
  }
  if (cdxtail !== 0 || cdytail !== 0) {
    if (adxtail !== 0 || adytail !== 0 || bdxtail !== 0 || bdytail !== 0) {
      s1 = adxtail * bdy;
      c = splitter * adxtail;
      ahi = c - (c - adxtail);
      alo = adxtail - ahi;
      c = splitter * bdy;
      bhi = c - (c - bdy);
      blo = bdy - bhi;
      s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
      t1 = adx * bdytail;
      c = splitter * adx;
      ahi = c - (c - adx);
      alo = adx - ahi;
      c = splitter * bdytail;
      bhi = c - (c - bdytail);
      blo = bdytail - bhi;
      t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
      _i = s0 + t0;
      bvirt = _i - s0;
      u[0] = s0 - (_i - bvirt) + (t0 - bvirt);
      _j = s1 + _i;
      bvirt = _j - s1;
      _0 = s1 - (_j - bvirt) + (_i - bvirt);
      _i = _0 + t1;
      bvirt = _i - _0;
      u[1] = _0 - (_i - bvirt) + (t1 - bvirt);
      u3 = _j + _i;
      bvirt = u3 - _j;
      u[2] = _j - (u3 - bvirt) + (_i - bvirt);
      u[3] = u3;
      n1 = -ady;
      n0 = -adytail;
      s1 = bdxtail * n1;
      c = splitter * bdxtail;
      ahi = c - (c - bdxtail);
      alo = bdxtail - ahi;
      c = splitter * n1;
      bhi = c - (c - n1);
      blo = n1 - bhi;
      s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
      t1 = bdx * n0;
      c = splitter * bdx;
      ahi = c - (c - bdx);
      alo = bdx - ahi;
      c = splitter * n0;
      bhi = c - (c - n0);
      blo = n0 - bhi;
      t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
      _i = s0 + t0;
      bvirt = _i - s0;
      v[0] = s0 - (_i - bvirt) + (t0 - bvirt);
      _j = s1 + _i;
      bvirt = _j - s1;
      _0 = s1 - (_j - bvirt) + (_i - bvirt);
      _i = _0 + t1;
      bvirt = _i - _0;
      v[1] = _0 - (_i - bvirt) + (t1 - bvirt);
      u3 = _j + _i;
      bvirt = u3 - _j;
      v[2] = _j - (u3 - bvirt) + (_i - bvirt);
      v[3] = u3;
      abtlen = predSum(4, u, 4, v, abt);
      s1 = adxtail * bdytail;
      c = splitter * adxtail;
      ahi = c - (c - adxtail);
      alo = adxtail - ahi;
      c = splitter * bdytail;
      bhi = c - (c - bdytail);
      blo = bdytail - bhi;
      s0 = alo * blo - (s1 - ahi * bhi - alo * bhi - ahi * blo);
      t1 = bdxtail * adytail;
      c = splitter * bdxtail;
      ahi = c - (c - bdxtail);
      alo = bdxtail - ahi;
      c = splitter * adytail;
      bhi = c - (c - adytail);
      blo = adytail - bhi;
      t0 = alo * blo - (t1 - ahi * bhi - alo * bhi - ahi * blo);
      _i = s0 - t0;
      bvirt = s0 - _i;
      abtt[0] = s0 - (_i + bvirt) + (bvirt - t0);
      _j = s1 + _i;
      bvirt = _j - s1;
      _0 = s1 - (_j - bvirt) + (_i - bvirt);
      _i = _0 - t1;
      bvirt = _0 - _i;
      abtt[1] = _0 - (_i + bvirt) + (bvirt - t1);
      u3 = _j + _i;
      bvirt = u3 - _j;
      abtt[2] = _j - (u3 - bvirt) + (_i - bvirt);
      abtt[3] = u3;
      abttlen = 4;
    } else {
      abt[0] = 0;
      abtlen = 1;
      abtt[0] = 0;
      abttlen = 1;
    }
    if (cdxtail !== 0) {
      const len = scale(abtlen, abt, cdxtail, _16c);
      finlen = finadd(
        finlen,
        predSum(
          scale(cxtablen, cxtab, cdxtail, _16),
          _16,
          scale(len, _16c, 2 * cdx, _32),
          _32,
          _48,
        ),
        _48,
      );

      const len2 = scale(abttlen, abtt, cdxtail, _8);
      finlen = finadd(
        finlen,
        predSumThree(
          scale(len2, _8, 2 * cdx, _16),
          _16,
          scale(len2, _8, cdxtail, _16b),
          _16b,
          scale(len, _16c, cdxtail, _32),
          _32,
          _32b,
          _64,
        ),
        _64,
      );

      if (adytail !== 0) {
        finlen = finadd(finlen, scale(scale(4, bb, cdxtail, _8), _8, adytail, _16), _16);
      }
      if (bdytail !== 0) {
        finlen = finadd(finlen, scale(scale(4, aa, -cdxtail, _8), _8, bdytail, _16), _16);
      }
    }
    if (cdytail !== 0) {
      const len = scale(abtlen, abt, cdytail, _16c);
      finlen = finadd(
        finlen,
        predSum(
          scale(cytablen, cytab, cdytail, _16),
          _16,
          scale(len, _16c, 2 * cdy, _32),
          _32,
          _48,
        ),
        _48,
      );

      const len2 = scale(abttlen, abtt, cdytail, _8);
      finlen = finadd(
        finlen,
        predSumThree(
          scale(len2, _8, 2 * cdy, _16),
          _16,
          scale(len2, _8, cdytail, _16b),
          _16b,
          scale(len, _16c, cdytail, _32),
          _32,
          _32b,
          _64,
        ),
        _64,
      );
    }
  }

  return fin[finlen - 1];
}

/**
 * The points a, b, and c must be in counterclockwise order, or the sign of the result will be reversed.
 * @param ax - x coordinate of first point
 * @param ay - y coordinate of first point
 * @param bx - x coordinate of second point
 * @param by - y coordinate of second point
 * @param cx - x coordinate of third point
 * @param cy - y coordinate of third point
 * @param dx - x coordinate of fourth point
 * @param dy - y coordinate of fourth point
 * @returns - a positive value if the point d lies outside the circle passing through a, b, and c.
 * - a negative value if it lies inside.
 * - zero if the four points are cocircular.
 */
export function incircle(
  ax: number,
  ay: number,
  bx: number,
  by: number,
  cx: number,
  cy: number,
  dx: number,
  dy: number,
): number {
  const adx = ax - dx;
  const bdx = bx - dx;
  const cdx = cx - dx;
  const ady = ay - dy;
  const bdy = by - dy;
  const cdy = cy - dy;

  const bdxcdy = bdx * cdy;
  const cdxbdy = cdx * bdy;
  const alift = adx * adx + ady * ady;

  const cdxady = cdx * ady;
  const adxcdy = adx * cdy;
  const blift = bdx * bdx + bdy * bdy;

  const adxbdy = adx * bdy;
  const bdxady = bdx * ady;
  const clift = cdx * cdx + cdy * cdy;

  const det = alift * (bdxcdy - cdxbdy) + blift * (cdxady - adxcdy) + clift * (adxbdy - bdxady);

  const permanent =
    (Math.abs(bdxcdy) + Math.abs(cdxbdy)) * alift +
    (Math.abs(cdxady) + Math.abs(adxcdy)) * blift +
    (Math.abs(adxbdy) + Math.abs(bdxady)) * clift;

  const errbound = iccerrboundA * permanent;

  if (det > errbound || -det > errbound) {
    return det;
  }
  return incircleadapt(ax, ay, bx, by, cx, cy, dx, dy, permanent);
}

/**
 * @param ax - x coordinate of first point
 * @param ay - y coordinate of first point
 * @param bx - x coordinate of second point
 * @param by - y coordinate of second point
 * @param cx - x coordinate of third point
 * @param cy - y coordinate of third point
 * @param dx - x coordinate of comparison point
 * @param dy - y coordinate of comparison point
 * @returns - point d is in circle if the return value is less than 0, otherwise not in circle
 */
export function incirclefast(
  ax: number,
  ay: number,
  bx: number,
  by: number,
  cx: number,
  cy: number,
  dx: number,
  dy: number,
): number {
  const adx = ax - dx;
  const ady = ay - dy;
  const bdx = bx - dx;
  const bdy = by - dy;
  const cdx = cx - dx;
  const cdy = cy - dy;

  const abdet = adx * bdy - bdx * ady;
  const bcdet = bdx * cdy - cdx * bdy;
  const cadet = cdx * ady - adx * cdy;
  const alift = adx * adx + ady * ady;
  const blift = bdx * bdx + bdy * bdy;
  const clift = cdx * cdx + cdy * cdy;

  return alift * bcdet + blift * cadet + clift * abdet;
}
