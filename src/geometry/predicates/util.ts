export const epsilon = 1.1102230246251565e-16;
export const splitter = 134217729;
export const resulterrbound = 3.3306690738754706e-16; // (3 + 8 * epsilon) * epsilon;

/**
 * fast_expansion_sum_zeroelim routine from oritinal code
 * @param elen - number of elements in expansion
 * @param e - expansion
 * @param flen - number of elements in expansion
 * @param f - expansion
 * @param h - expansion
 * @returns - the sum
 */
export function predSum(
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
    enow = e[++eindex] ?? 0;
  } else {
    Q = fnow;
    fnow = f[++findex] ?? 0;
  }
  let hindex = 0;
  if (eindex < elen && findex < flen) {
    if (fnow > enow === fnow > -enow) {
      Qnew = enow + Q;
      hh = Q - (Qnew - enow);
      enow = e[++eindex] ?? 0;
    } else {
      Qnew = fnow + Q;
      hh = Q - (Qnew - fnow);
      fnow = f[++findex] ?? 0;
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
        enow = e[++eindex] ?? 0;
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
    enow = e[++eindex] ?? 0;
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
 * @param alen - length of array a
 * @param a - array A
 * @param blen - length of array b
 * @param b - array B
 * @param clen - length of array c
 * @param c - array C
 * @param tmp - temporary array
 * @param out - output
 * @returns the sum
 */
export function predSumThree(
  alen: number,
  a: number[] | Float64Array,
  blen: number,
  b: number[] | Float64Array,
  clen: number,
  c: number[] | Float64Array,
  tmp: number[] | Float64Array,
  out: number[] | Float64Array,
): number {
  return predSum(predSum(alen, a, blen, b, tmp), tmp, clen, c, out);
}

/**
 * scale_expansion_zeroelim routine from oritinal code
 * @param elen - number of elements in expansion
 * @param e - expansion
 * @param b - scalar
 * @param h - expansion
 * @returns - the size of the h expansion
 */
export function scale(
  elen: number,
  e: number[] | Float64Array,
  b: number,
  h: number[] | Float64Array,
): number {
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
 * estimation of expansion sum
 * @param elen - number of elements in expansion
 * @param e - expansion
 * @returns - the sum
 */
export function estimate(elen: number, e: number[] | Float64Array): number {
  let Q = e[0];
  for (let i = 1; i < elen; i++) Q += e[i];
  return Q;
}

/**
 * Creates a new Float64Array of n size
 * @param n - number of elements
 * @returns - new Float64Array of n size
 */
export function vec(n: number): Float64Array {
  return new Float64Array(n);
}
