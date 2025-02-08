import { U32I32F32 } from './util';
import { ArithmeticBitModel, ArithmeticModel } from './arithmeticDecoder';

import type { ArithmeticDecoder } from './arithmeticDecoder';
import type { LAZContext } from './getPointCompressed';

const I32_MIN = -2147483648; // ((I32)0x80000000)
const I32_MAX = 2147483647; // ((I32)0x7FFFFFFF)

/**
 * https://github.com/LASzip/LASzip/blob/master/src/integercompressor.cpp
 */
export class IntegerCompressor {
  #k = 0;
  #corrBits: number;
  #corrRange: number;
  corrMin: number;
  corrMax: number;
  #mBits: ArithmeticModel[] = [];
  #mCorrector: (ArithmeticModel | ArithmeticBitModel)[] = [];
  /**
   * @param dec - the arithmetic decoder
   * @param bits - the number of bits
   * @param contexts - the number of contexts
   * @param bitsHigh - the number of bits for the high bits
   * @param range - the range
   */
  constructor(
    private dec: ArithmeticDecoder,
    private bits = 16, // U32
    private contexts = 1, // U32
    private bitsHigh = 8, // U32
    private range = 0, // U32
  ) {
    if (this.range !== 0) {
      // the corrector's significant bits and range
      this.#corrBits = 0;
      this.#corrRange = this.range;
      while (this.range !== 0) {
        this.range = this.range >> 1;
        this.#corrBits++;
      }
      if (this.#corrRange === 1 << (this.#corrBits - 1)) {
        this.#corrBits--;
      }
      // the corrector must fall into this interval
      this.corrMin = -Math.trunc(this.#corrRange / 2);
      this.corrMax = this.corrMin + this.#corrRange - 1;
    } else if (this.bits !== 0 && this.bits < 32) {
      this.#corrBits = this.bits;
      this.#corrRange = 1 << this.bits;
      // the corrector must fall into this interval
      this.corrMin = -Math.trunc(this.#corrRange / 2);
      this.corrMax = this.corrMin + this.#corrRange - 1;
    } else {
      this.#corrBits = 32;
      this.#corrRange = 0;
      // the corrector must fall into this interval
      this.corrMin = I32_MIN;
      this.corrMax = I32_MAX;
    }
  }

  /**
   * Get the K value in the Compressor
   * @returns - the current k value
   */
  getK(): number {
    return this.#k;
  }

  /** Initialize the decompressor */
  initDecompressor(): void {
    let i: number;

    // maybe create the models
    if (this.#mBits.length === 0) {
      this.#mBits = new Array(this.contexts);
      for (i = 0; i < this.contexts; i++) {
        // this.#mBits[i] = this.dec.createSymbolModel(this.#corrBits + 1);
        this.#mBits[i] = new ArithmeticModel(this.#corrBits + 1, false);
      }
      this.#mCorrector = new Array(this.#corrBits + 1);
      this.#mCorrector[0] = new ArithmeticBitModel();
      for (i = 1; i <= this.#corrBits; i++) {
        if (i <= this.bitsHigh) {
          this.#mCorrector[i] = new ArithmeticModel(1 << i, false);
        } else {
          this.#mCorrector[i] = new ArithmeticModel(1 << this.bitsHigh, false);
        }
      }
    }
    // certainly init the models
    for (i = 0; i < this.contexts; i++) this.#mBits[i].init();
    for (i = 0; i <= this.#corrBits; i++) this.#mCorrector[i].init();
  }

  /**
   * @param pred - the predicted value
   * @param context - the context
   * @returns - the decompressed value
   */
  decompress(pred: number, context: LAZContext = { value: 0 }): number {
    const real = new U32I32F32(pred + this.readCorrector(this.#mBits[context.value]), 'i32');
    if (real.i32 < 0) real.i32 += this.#corrRange;
    else if (real.u32 >= this.#corrRange) real.i32 -= this.#corrRange;
    return real.i32;
  }

  /**
   * @param mBits - the arithmetic model
   * @returns - the corrector
   */
  readCorrector(mBits: ArithmeticModel): number {
    let c: number; // I32

    // decode within which interval the corrector is falling

    this.#k = this.dec.decodeSymbol(mBits);

    // decode the exact location of the corrector within the interval

    if (this.#k !== 0) {
      // then c is either smaller than 0 or bigger than 1
      if (this.#k < 32) {
        if (this.#k <= this.bitsHigh) {
          // for small k we can do this in one step
          // decompress c with the range coder
          c = this.dec.decodeSymbol(this.#mCorrector[this.#k] as ArithmeticModel);
        } else {
          // for larger k we need to do this in two steps
          const k1 = this.#k - this.bitsHigh;
          // decompress higher bits with table
          c = this.dec.decodeSymbol(this.#mCorrector[this.#k] as ArithmeticModel);
          // read lower bits raw
          const c1 = this.dec.readBits(k1);
          // put the corrector back together
          c = (c << k1) | c1;
        }
        // translate c back into its correct interval
        if (c >= 1 << (this.#k - 1)) {
          // if c is in the interval [ 2^(k-1)  ...  + 2^k - 1 ]
          // so we translate c back into the interval [ 2^(k-1) + 1  ...  2^k ] by adding 1
          c += 1;
        } else {
          // otherwise c is in the interval [ 0 ...  + 2^(k-1) - 1 ]
          // so we translate c back into the interval [ - (2^k - 1)  ...  - (2^(k-1)) ] by subtracting (2^k - 1)
          c -= (1 << this.#k) - 1;
        }
      } else {
        c = this.corrMin;
      }
    } else {
      // then c is either 0 or 1
      c = this.dec.decodeBit(this.#mCorrector[0] as ArithmeticBitModel);
    }

    return c;
  }
}
