// NOTE I think this is going to become a polyfill one day... keeping it here for now

/**
 * Increment x by the next representable float
 * @param x - the float
 * @param incSize - the increment size
 * @returns - the incremented float
 */
export function nextUp(x: number, incSize = 1): number {
  if (Number.isNaN(x) || x === Infinity || x === -Infinity) return x;
  if (x === 0) return Number.MIN_VALUE;

  const buffer = new ArrayBuffer(8);
  const view = new DataView(buffer);
  view.setFloat64(0, x, false); // big-endian for consistent byte order
  // Read bits as unsigned 64-bit split into hi/lo parts (since JS lacks 64-bit ints)
  let hi = view.getUint32(0);
  let lo = view.getUint32(4);

  while (incSize-- > 0) {
    if (x > 0) {
      if (++lo === 0) hi++;
    } else {
      if (lo-- === 0) hi--;
    }
  }

  view.setUint32(0, hi);
  view.setUint32(4, lo);
  return view.getFloat64(0, false);
}

/**
 * Decrement x by the next representable float
 * @param x - the float
 * @param decSize - the dcrement size
 * @returns - the decremented float
 */
export function nextDown(x: number, decSize = 1): number {
  if (Number.isNaN(x) || x === -Infinity || x === Infinity) return x;
  if (x === 0) return -Number.MIN_VALUE;
  return -nextUp(-x, decSize);
}
