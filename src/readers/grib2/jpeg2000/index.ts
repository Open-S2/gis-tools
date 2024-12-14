// https://github.com/runk/jpeg2000/tree/main

// Table C-2
const QeTable = [
  { qe: 0x5601, nmps: 1, nlps: 1, switchFlag: 1 },
  { qe: 0x3401, nmps: 2, nlps: 6, switchFlag: 0 },
  { qe: 0x1801, nmps: 3, nlps: 9, switchFlag: 0 },
  { qe: 0x0ac1, nmps: 4, nlps: 12, switchFlag: 0 },
  { qe: 0x0521, nmps: 5, nlps: 29, switchFlag: 0 },
  { qe: 0x0221, nmps: 38, nlps: 33, switchFlag: 0 },
  { qe: 0x5601, nmps: 7, nlps: 6, switchFlag: 1 },
  { qe: 0x5401, nmps: 8, nlps: 14, switchFlag: 0 },
  { qe: 0x4801, nmps: 9, nlps: 14, switchFlag: 0 },
  { qe: 0x3801, nmps: 10, nlps: 14, switchFlag: 0 },
  { qe: 0x3001, nmps: 11, nlps: 17, switchFlag: 0 },
  { qe: 0x2401, nmps: 12, nlps: 18, switchFlag: 0 },
  { qe: 0x1c01, nmps: 13, nlps: 20, switchFlag: 0 },
  { qe: 0x1601, nmps: 29, nlps: 21, switchFlag: 0 },
  { qe: 0x5601, nmps: 15, nlps: 14, switchFlag: 1 },
  { qe: 0x5401, nmps: 16, nlps: 14, switchFlag: 0 },
  { qe: 0x5101, nmps: 17, nlps: 15, switchFlag: 0 },
  { qe: 0x4801, nmps: 18, nlps: 16, switchFlag: 0 },
  { qe: 0x3801, nmps: 19, nlps: 17, switchFlag: 0 },
  { qe: 0x3401, nmps: 20, nlps: 18, switchFlag: 0 },
  { qe: 0x3001, nmps: 21, nlps: 19, switchFlag: 0 },
  { qe: 0x2801, nmps: 22, nlps: 19, switchFlag: 0 },
  { qe: 0x2401, nmps: 23, nlps: 20, switchFlag: 0 },
  { qe: 0x2201, nmps: 24, nlps: 21, switchFlag: 0 },
  { qe: 0x1c01, nmps: 25, nlps: 22, switchFlag: 0 },
  { qe: 0x1801, nmps: 26, nlps: 23, switchFlag: 0 },
  { qe: 0x1601, nmps: 27, nlps: 24, switchFlag: 0 },
  { qe: 0x1401, nmps: 28, nlps: 25, switchFlag: 0 },
  { qe: 0x1201, nmps: 29, nlps: 26, switchFlag: 0 },
  { qe: 0x1101, nmps: 30, nlps: 27, switchFlag: 0 },
  { qe: 0x0ac1, nmps: 31, nlps: 28, switchFlag: 0 },
  { qe: 0x09c1, nmps: 32, nlps: 29, switchFlag: 0 },
  { qe: 0x08a1, nmps: 33, nlps: 30, switchFlag: 0 },
  { qe: 0x0521, nmps: 34, nlps: 31, switchFlag: 0 },
  { qe: 0x0441, nmps: 35, nlps: 32, switchFlag: 0 },
  { qe: 0x02a1, nmps: 36, nlps: 33, switchFlag: 0 },
  { qe: 0x0221, nmps: 37, nlps: 34, switchFlag: 0 },
  { qe: 0x0141, nmps: 38, nlps: 35, switchFlag: 0 },
  { qe: 0x0111, nmps: 39, nlps: 36, switchFlag: 0 },
  { qe: 0x0085, nmps: 40, nlps: 37, switchFlag: 0 },
  { qe: 0x0049, nmps: 41, nlps: 38, switchFlag: 0 },
  { qe: 0x0025, nmps: 42, nlps: 39, switchFlag: 0 },
  { qe: 0x0015, nmps: 43, nlps: 40, switchFlag: 0 },
  { qe: 0x0009, nmps: 44, nlps: 41, switchFlag: 0 },
  { qe: 0x0005, nmps: 45, nlps: 42, switchFlag: 0 },
  { qe: 0x0001, nmps: 45, nlps: 43, switchFlag: 0 },
  { qe: 0x5601, nmps: 46, nlps: 46, switchFlag: 0 },
];

/**
 * Calculate the base 2 logarithm of the number `x`. This differs from the
 * native function in the sense that it returns the ceiling value and that it
 * returns 0 instead of `Infinity`/`NaN` for `x` values smaller than/equal to 0.
 * @param x - the number to calculate the logarithm of
 * @returns the base 2 logarithm
 */
function log2(x: number): number {
  if (x <= 0) return 0;
  return Math.ceil(Math.log2(x));
}

/**
 * This class implements the QM Coder decoding as defined in
 *   JPEG 2000 Part I Final Committee Draft Version 1.0
 *   Annex C.3 Arithmetic decoding procedure
 * available at http://www.jpeg.org/public/fcd15444-1.pdf
 *
 * The arithmetic decoder is used in conjunction with context models to decode
 * JPEG2000 and JBIG2 streams.
 */
export class ArithmeticDecoder {
  a: number;
  chigh: number;
  ct = 0;
  clow = 0;
  // C.3.5 Initialisation of the decoder (INITDEC)
  /**
   * @param data - compressed data
   * @param start - start index
   * @param end - end index
   */
  constructor(
    public data: Uint8Array,
    public start: number,
    public end: number,
  ) {
    this.chigh = data[start];
    this.byteIn();

    this.chigh = ((this.chigh << 7) & 0xffff) | ((this.clow >> 9) & 0x7f);
    this.clow = (this.clow << 7) & 0xffff;
    this.ct -= 7;
    this.a = 0x8000;
  }

  /** C.3.4 Compressed data input (BYTEIN) */
  byteIn(): void {
    const data = this.data;
    let bp = this.start;

    if (data[bp] === 0xff) {
      if (data[bp + 1] > 0x8f) {
        this.clow += 0xff00;
        this.ct = 8;
      } else {
        bp++;
        this.clow += data[bp] << 9;
        this.ct = 7;
        this.start = bp;
      }
    } else {
      bp++;
      this.clow += bp < this.end ? data[bp] << 8 : 0xff00;
      this.ct = 8;
      this.start = bp;
    }
    if (this.clow > 0xffff) {
      this.chigh += this.clow >> 16;
      this.clow &= 0xffff;
    }
  }

  /**
   * C.3.2 Decoding a decision (DECODE)
   * @param contexts - context models
   * @param pos - context model index
   * @returns 0 or 1
   */
  readBit(contexts: number[], pos: number): number {
    // Contexts are packed into 1 byte:
    // highest 7 bits carry cx.index, lowest bit carries cx.mps
    let cx_index = contexts[pos] >> 1,
      cx_mps = contexts[pos] & 1;
    const qeTableIcx = QeTable[cx_index];
    const qeIcx = qeTableIcx.qe;
    let d;
    let a = this.a - qeIcx;

    if (this.chigh < qeIcx) {
      // exchangeLps
      if (a < qeIcx) {
        a = qeIcx;
        d = cx_mps;
        cx_index = qeTableIcx.nmps;
      } else {
        a = qeIcx;
        d = 1 ^ cx_mps;
        if (qeTableIcx.switchFlag === 1) {
          cx_mps = d;
        }
        cx_index = qeTableIcx.nlps;
      }
    } else {
      this.chigh -= qeIcx;
      if ((a & 0x8000) !== 0) {
        this.a = a;
        return cx_mps;
      }
      // exchangeMps
      if (a < qeIcx) {
        d = 1 ^ cx_mps;
        if (qeTableIcx.switchFlag === 1) {
          cx_mps = d;
        }
        cx_index = qeTableIcx.nlps;
      } else {
        d = cx_mps;
        cx_index = qeTableIcx.nmps;
      }
    }
    // C.3.3 renormD;
    do {
      if (this.ct === 0) {
        this.byteIn();
      }

      a <<= 1;
      this.chigh = ((this.chigh << 1) & 0xffff) | ((this.clow >> 15) & 1);
      this.clow = (this.clow << 1) & 0xffff;
      this.ct--;
    } while ((a & 0x8000) === 0);
    this.a = a;

    contexts[pos] = (cx_index << 1) | cx_mps;
    return d;
  }
}
