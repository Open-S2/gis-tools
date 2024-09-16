export const epsilon = 1.1102230246251565e-16;
export const splitter = 134217729;
export const resulterrbound = (3 + 8 * epsilon) * epsilon;

// fast_expansion_sum_zeroelim routine from oritinal code
/**
 * @param elen
 * @param e
 * @param flen
 * @param f
 * @param h
 */
export function sum(
  elen: number,
  e: number[] | Float64Array,
  flen: number,
  f: number[] | Float64Array,
  h: number[] | Float64Array,
): number {
  let Q, Qnew, hh, bvirt;
  let enow = e[0];
  let fnow = f[0];
  let eindex = 0;
  let findex = 0;
  if (fnow > enow === fnow > -enow) {
    Q = enow;
    enow = e[++eindex];
  } else {
    Q = fnow;
    fnow = f[++findex];
  }
  let hindex = 0;
  if (eindex < elen && findex < flen) {
    if (fnow > enow === fnow > -enow) {
      Qnew = enow + Q;
      hh = Q - (Qnew - enow);
      enow = e[++eindex];
    } else {
      Qnew = fnow + Q;
      hh = Q - (Qnew - fnow);
      fnow = f[++findex];
    }
    Q = Qnew;
    if (hh !== 0) {
      h[hindex++] = hh;
    }
    while (eindex < elen && findex < flen) {
      if (fnow > enow === fnow > -enow) {
        Qnew = Q + enow;
        bvirt = Qnew - Q;
        hh = Q - (Qnew - bvirt) + (enow - bvirt);
        enow = e[++eindex];
      } else {
        Qnew = Q + fnow;
        bvirt = Qnew - Q;
        hh = Q - (Qnew - bvirt) + (fnow - bvirt);
        fnow = f[++findex];
      }
      Q = Qnew;
      if (hh !== 0) {
        h[hindex++] = hh;
      }
    }
  }
  while (eindex < elen) {
    Qnew = Q + enow;
    bvirt = Qnew - Q;
    hh = Q - (Qnew - bvirt) + (enow - bvirt);
    enow = e[++eindex];
    Q = Qnew;
    if (hh !== 0) {
      h[hindex++] = hh;
    }
  }
  while (findex < flen) {
    Qnew = Q + fnow;
    bvirt = Qnew - Q;
    hh = Q - (Qnew - bvirt) + (fnow - bvirt);
    fnow = f[++findex];
    Q = Qnew;
    if (hh !== 0) {
      h[hindex++] = hh;
    }
  }
  if (Q !== 0 || hindex === 0) {
    h[hindex++] = Q;
  }
  return hindex;
}

/**
 * @param alen
 * @param a
 * @param blen
 * @param b
 * @param clen
 * @param c
 * @param tmp
 * @param out
 */
export function sum_three(
  alen: number,
  a: number[] | Float64Array,
  blen: number,
  b: number[] | Float64Array,
  clen: number,
  c: number[] | Float64Array,
  tmp: number[] | Float64Array,
  out: number[] | Float64Array,
): number {
  return sum(sum(alen, a, blen, b, tmp), tmp, clen, c, out);
}

/**
 * scale_expansion_zeroelim routine from oritinal code
 * @param elen
 * @param e
 * @param b
 * @param h
 */
export function scale(
  elen: number,
  e: number[] | Float64Array,
  b: number,
  h: number[] | Float64Array,
) {
  let Q, sum, hh, product1, product0;
  let bvirt, c, ahi, alo;

  c = splitter * b;
  const bhi = c - (c - b);
  const blo = b - bhi;
  let enow = e[0];
  Q = enow * b;
  c = splitter * enow;
  ahi = c - (c - enow);
  alo = enow - ahi;
  hh = alo * blo - (Q - ahi * bhi - alo * bhi - ahi * blo);
  let hindex = 0;
  if (hh !== 0) {
    h[hindex++] = hh;
  }
  for (let i = 1; i < elen; i++) {
    enow = e[i];
    product1 = enow * b;
    c = splitter * enow;
    ahi = c - (c - enow);
    alo = enow - ahi;
    product0 = alo * blo - (product1 - ahi * bhi - alo * bhi - ahi * blo);
    sum = Q + product0;
    bvirt = sum - Q;
    hh = Q - (sum - bvirt) + (product0 - bvirt);
    if (hh !== 0) {
      h[hindex++] = hh;
    }
    Q = product1 + sum;
    hh = sum - (Q - product1);
    if (hh !== 0) {
      h[hindex++] = hh;
    }
  }
  if (Q !== 0 || hindex === 0) {
    h[hindex++] = Q;
  }
  return hindex;
}

// /**
//  * @param elen
//  * @param e
//  */
// export function negate(elen: number, e: number[] | Float64Array): number {
//   for (let i = 0; i < elen; i++) e[i] = -e[i];
//   return elen;
// }

/**
 * @param elen
 * @param e
 */
export function estimate(elen: number, e: number[] | Float64Array): number {
  let Q = e[0];
  for (let i = 1; i < elen; i++) Q += e[i];
  return Q;
}

/**
 * @param n
 */
export function vec(n: number): Float64Array {
  return new Float64Array(n);
}
