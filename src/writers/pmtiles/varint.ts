import type { VarintBufPos } from 's2-tools/readers/pmtiles';

/**
 * Write a varint. Can be max 64-bits. Numbers are coerced to an unsigned
 * while number before using this function.
 * @param val - any whole unsigned number.
 * @param bufPos - the buffer with it's position to write at
 */
export function writeVarint(val: number, bufPos: VarintBufPos): void {
  if (val > 0xfffffff || val < 0) {
    writeBigVarint(val, bufPos);
    return;
  }

  realloc(bufPos, 4);

  bufPos.buf[bufPos.pos++] = (val & 0x7f) | (val > 0x7f ? 0x80 : 0);
  if (val <= 0x7f) return;
  bufPos.buf[bufPos.pos++] = ((val >>>= 7) & 0x7f) | (val > 0x7f ? 0x80 : 0);
  if (val <= 0x7f) return;
  bufPos.buf[bufPos.pos++] = ((val >>>= 7) & 0x7f) | (val > 0x7f ? 0x80 : 0);
  if (val <= 0x7f) return;
  bufPos.buf[bufPos.pos++] = (val >>> 7) & 0x7f;
}

/**
 * Write a varint larger then 54-bits.
 * @param val - the number
 * @param bufPos - the buffer with it's position to write at
 */
export function writeBigVarint(val: number, bufPos: VarintBufPos): void {
  let low = val % 0x100000000 | 0;
  let high = (val / 0x100000000) | 0;

  if (val < 0) {
    low = ~(-val % 0x100000000);
    high = ~(-val / 0x100000000);

    if ((low ^ 0xffffffff) !== 0) {
      low = (low + 1) | 0;
    } else {
      low = 0;
      high = (high + 1) | 0;
    }
  }

  if (val >= 0x10000000000000000n || val < -0x10000000000000000n) {
    throw new Error("Given varint doesn't fit into 10 bytes");
  }

  realloc(bufPos, 10);

  writeBigVarintLow(low, high, bufPos);
  writeBigVarintHigh(high, bufPos);
}

/**
 * Write a varint larger then 54-bits on the low end
 * @param low - lower 32 bits
 * @param _high - unused "high" bits
 * @param bufPos - the buffer with it's position to write at
 */
export function writeBigVarintLow(low: number, _high: number, bufPos: VarintBufPos): void {
  bufPos.buf[bufPos.pos++] = (low & 0x7f) | 0x80;
  low >>>= 7;
  bufPos.buf[bufPos.pos++] = (low & 0x7f) | 0x80;
  low >>>= 7;
  bufPos.buf[bufPos.pos++] = (low & 0x7f) | 0x80;
  low >>>= 7;
  bufPos.buf[bufPos.pos++] = (low & 0x7f) | 0x80;
  low >>>= 7;
  bufPos.buf[bufPos.pos] = low & 0x7f;
}

/**
 * Write a varint larger then 54-bits on the high end
 * @param high - the high 32 bits
 * @param bufPos - the buffer with it's position to write at
 */
export function writeBigVarintHigh(high: number, bufPos: VarintBufPos): void {
  const lsb = (high & 0x07) << 4;

  bufPos.buf[bufPos.pos++] |= lsb | ((high >>>= 3) !== 0 ? 0x80 : 0);
  if (high === 0) return;
  bufPos.buf[bufPos.pos++] = (high & 0x7f) | ((high >>>= 7) !== 0 ? 0x80 : 0);
  if (high === 0) return;
  bufPos.buf[bufPos.pos++] = (high & 0x7f) | ((high >>>= 7) !== 0 ? 0x80 : 0);
  if (high === 0) return;
  bufPos.buf[bufPos.pos++] = (high & 0x7f) | ((high >>>= 7) !== 0 ? 0x80 : 0);
  if (high === 0) return;
  bufPos.buf[bufPos.pos++] = (high & 0x7f) | ((high >>>= 7) !== 0 ? 0x80 : 0);
  if (high === 0) return;
  bufPos.buf[bufPos.pos++] = high & 0x7f;
}

/**
 * Allocate more space in the buffer
 * @param bufPos - the buffer with it's position
 * @param min - the minimum number of bytes to allocate
 */
function realloc(bufPos: VarintBufPos, min: number): void {
  let length = bufPos.buf.length > 0 ? bufPos.buf.length : 16;

  while (length < bufPos.pos + min) length *= 2;

  if (length !== bufPos.buf.length) {
    const buf = new Uint8Array(length);
    buf.set(bufPos.buf);
    bufPos.buf = buf;
  }
}
