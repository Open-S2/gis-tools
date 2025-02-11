import { U32I32F32, U64I64F64 } from '../util';

import type { Reader } from '../..';

/* this header byte needs to change in case incompatible change happen */
export const AC_HEADER_BYTE = 2;
export const AC_BUFFER_SIZE = 4096;

export const AC__MinLength = 0x01000000; // threshold for renormalization
export const AC__MaxLength = 0xffffffff; // maximum AC interval length

// Maximum values for binary models
export const BM__LengthShift = 13; // length bits discarded before mult.
export const BM__MaxCount = 8_192; // 1 << BM__LengthShift; // for adaptive models

// Maximum values for general models
export const DM__LengthShift = 15; // length bits discarded before mult.
export const DM__MaxCount = 32_768; // 1 << DM__LengthShift; // for adaptive models

/**
 * https://github.com/LASzip/LASzip/blob/master/src/arithmeticdecoder.cpp
 */
export class ArithmeticDecoder {
  // U32 value, length;
  value = 0; // U32
  length = AC__MaxLength; // U32
  /** @param reader - The input reader */
  constructor(readonly reader: Reader) {}

  /** @param reallyInit - if set to true, initializes the value */
  init(reallyInit = true): void {
    this.length = AC__MaxLength;
    if (reallyInit) this.value = this.reader.getUint32();
  }

  /**
   * @param m - The arithmetic bit model
   * @returns - The decoded bit
   */
  decodeBit(m: ArithmeticBitModel): number {
    const x = m.bit0Prob * Math.trunc(this.length >>> BM__LengthShift); // product l x p0
    const sym = this.value >= x ? 1 : 0; // decision
    // update & shift interval
    if (sym === 0) {
      this.length = x;
      ++m.bit0Count;
    } else {
      this.value -= x; // shifted interval base = 0
      this.length -= x;
    }

    if (this.length < AC__MinLength) this.renormDecInterval(); // renormalization
    if (--m.bitsUntilUpdate === 0) m.update(); // periodic model update

    return sym; // return data bit value
  }

  /**
   * @param m - The arithmetic model
   * @returns - The decoded symbol
   */
  decodeSymbol(m: ArithmeticModel): number {
    let n: number,
      sym: number,
      x: number,
      y = this.length;

    if (m.decoderTableIndex !== -1) {
      this.length = Math.trunc(this.length >>> DM__LengthShift);
      const dv = Math.trunc(this.value / this.length);
      const t = Math.trunc(dv >>> m.tableShift);

      sym = m.distribution[m.decoderTableIndex + t]; // initial decision based on table look-up
      n = m.distribution[m.decoderTableIndex + t + 1] + 1;

      while (n > sym + 1) {
        // finish with bisection search
        const k = Math.trunc((sym + n) >>> 1);
        if (m.distribution[k] > dv) n = k;
        else sym = k;
      }
      // compute products
      x = m.distribution[sym] * this.length;
      if (sym !== m.lastSymbol) y = m.distribution[sym + 1] * this.length;
    } else {
      // decode using only multiplications

      x = sym = 0;
      this.length = Math.trunc(this.length >>> DM__LengthShift);
      let k = Math.trunc((n = m.symbols) >>> 1);
      // decode via bisection search
      do {
        const z = this.length * m.distribution[k];
        if (z > this.value) {
          n = k;
          y = z; // value is smaller
        } else {
          sym = k;
          x = z; // value is larger or equal
        }
      } while ((k = Math.trunc(sym + n) >> 1) !== sym);
    }

    this.value -= x; // update interval
    this.length = y - x;

    if (this.length < AC__MinLength) this.renormDecInterval(); // renormalization

    ++m.distribution[m.symbolCountIndex + sym];
    if (--m.symbolsUntilUpdate === 0) m.update(); // periodic model update
    // assert(sym < m.symbols);

    return sym;
  }

  /** @returns - The decoded bit */
  readBit(): number {
    this.length = Math.trunc(this.length >>> 1);
    const sym = Math.trunc(this.value / this.length); // decode symbol, change length
    this.value -= this.length * sym; // update interval

    if (this.length < AC__MinLength) this.renormDecInterval(); // renormalization
    if (sym >= 2) throw new Error('4711');

    return sym;
  }

  /**
   * @param bits - The number of bits
   * @returns - The decoded bits
   */
  readBits(bits: number): number {
    // assert(bits && (bits <= 32));

    if (bits > 19) {
      const tmp = this.readShort();
      bits = bits - 16;
      const tmp1 = this.readBits(bits) * 65_536;
      return tmp1 | tmp;
    }

    this.length = Math.trunc(this.length >>> bits);
    const sym = Math.trunc(this.value / this.length); // decode symbol, change length
    this.value -= this.length * sym; // update interval

    if (this.length < AC__MinLength) this.renormDecInterval(); // renormalization
    if (sym >= 1 << bits) throw new Error('4711');

    return sym;
  }

  /** @returns - The decoded byte */
  readByte(): number {
    this.length = Math.trunc(this.length >>> 8);
    const sym = Math.trunc(this.value / this.length); // decode symbol, change length
    this.value -= this.length * sym; // update interval

    if (this.length < AC__MinLength) this.renormDecInterval(); // renormalization
    if (sym >= 1 << 8) throw new Error('4711');

    return sym;
  }

  /** @returns - The decoded short */
  readShort(): number {
    this.length = Math.trunc(this.length >>> 16);
    const sym = Math.trunc(this.value / this.length); // decode symbol, change length
    this.value -= this.length * sym; // update interval

    if (this.length < AC__MinLength) this.renormDecInterval(); // renormalization
    if (sym >= 65_536) throw new Error('4711');

    return sym;
  }

  /** @returns - The decoded int */
  readInt(): number {
    const lowerInt = this.readShort();
    const upperInt = this.readShort();
    return ((upperInt * 65_536) | lowerInt) >>> 0;
  }

  /** @returns - The decoded float */
  readFloat(): number {
    /* danger in float reinterpretation */
    const u32i32f32 = new U32I32F32(this.readInt(), 'u32');
    return u32i32f32.f32;
  }

  /** @returns - The decoded int64 */
  readInt64(): bigint {
    const lowerInt = this.readInt();
    const upperInt = this.readInt();
    return (BigInt(upperInt) << 32n) | BigInt(lowerInt);
  }

  /** @returns - The decoded double */
  readDouble(): number {
    /* danger in float reinterpretation */
    const lowerInt = this.readInt();
    const upperInt = this.readInt();
    const u64 = (BigInt(upperInt) << 32n) | BigInt(lowerInt);
    const u64i64f64 = new U64I64F64(u64, 'u64');
    return u64i64f64.f64;
  }

  /** Renormalize the decoder interval */
  renormDecInterval(): void {
    const value = new U32I32F32(this.value, 'u32');
    const length = new U32I32F32(this.length, 'u32');
    do {
      const nextByte = this.reader.getUint8();
      // read least-significant byte
      value.u32 = (value.u32 * 256) | nextByte;
    } while ((length.u32 *= 256) < AC__MinLength); // length multiplied by 256

    this.value = value.u32;
    this.length = length.u32;
  }
}

/** Arithmetic Model */
export class ArithmeticModel {
  distribution: number[] = [];
  symbolCountIndex = 0;
  decoderTableIndex = -1;
  totalCount = 0;
  updateCycle = 0;
  symbolsUntilUpdate = 0;
  lastSymbol = 0;
  tableSize = 0;
  tableShift = 0;
  /**
   * @param symbols - The number of symbols
   * @param compress - If the model has been compressed
   */
  constructor(
    readonly symbols: number,
    readonly compress = false,
  ) {}

  /** @param table - The table */
  init(table?: number[]): void {
    if (this.distribution.length === 0) {
      if (this.symbols < 2 || this.symbols > 1 << 11) throw Error('invalid number of symbols');
      this.lastSymbol = this.symbols - 1;
      if (!this.compress && this.symbols > 16) {
        let table_bits = 3;
        while (this.symbols > 1 << (table_bits + 2)) ++table_bits;
        this.tableSize = 1 << table_bits;
        this.tableShift = DM__LengthShift - table_bits;
        this.distribution = new Array(2 * this.symbols + this.tableSize + 2);
        this.decoderTableIndex = 2 * this.symbols;
      } else {
        // small alphabet: no table needed
        this.decoderTableIndex = -1;
        this.tableSize = this.tableShift = 0;
        this.distribution = new Array(2 * this.symbols).fill(0);
      }
      this.symbolCountIndex = this.symbols;
    }

    this.totalCount = 0;
    this.updateCycle = this.symbols;
    if (table !== undefined)
      for (let k = 0; k < this.symbols; k++)
        this.distribution[this.symbolCountIndex + k] = table[k];
    else for (let k = 0; k < this.symbols; k++) this.distribution[this.symbolCountIndex + k] = 1;

    this.update();
    this.symbolsUntilUpdate = this.updateCycle = (this.symbols + 6) >> 1;
  }

  /** Update the model */
  update(): void {
    // halve counts when a threshold is reached
    if ((this.totalCount += this.updateCycle) > DM__MaxCount) {
      this.totalCount = 0;
      for (let n = 0; n < this.symbols; n++) {
        this.totalCount += this.distribution[this.symbolCountIndex + n] = Math.trunc(
          (this.distribution[this.symbolCountIndex + n] + 1) >>> 1,
        );
      }
    }

    // compute cumulative distribution, decoder table
    let k: number,
      sum = 0,
      s = 0;
    const scale = Math.trunc(0x80000000 / this.totalCount);

    if (this.compress || this.tableSize === 0) {
      for (k = 0; k < this.symbols; k++) {
        this.distribution[k] = Math.trunc((scale * sum) >>> (31 - DM__LengthShift));
        sum += this.distribution[this.symbolCountIndex + k];
      }
    } else {
      for (k = 0; k < this.symbols; k++) {
        this.distribution[k] = Math.trunc((scale * sum) >>> (31 - DM__LengthShift));
        sum += this.distribution[this.symbolCountIndex + k];
        const w = Math.trunc(this.distribution[k] >>> this.tableShift);
        while (s < w) this.distribution[this.decoderTableIndex + ++s] = k - 1;
      }
      this.distribution[this.decoderTableIndex] = 0;
      while (s <= this.tableSize)
        this.distribution[this.decoderTableIndex + ++s] = this.symbols - 1;
    }

    // set frequency of model updates
    this.updateCycle = Math.trunc((5 * this.updateCycle) >>> 2);
    const max_cycle = (this.symbols + 6) << 3;
    if (this.updateCycle > max_cycle) this.updateCycle = max_cycle;
    this.symbolsUntilUpdate = this.updateCycle;
  }
}

/** Arithmetic Bit Model */
export class ArithmeticBitModel {
  // start with frequent updates
  updateCycle = 4;
  bitsUntilUpdate = 4;
  // initialization to equiprobable model
  bit0Prob = 1 << (BM__LengthShift - 1);
  bit0Count = 1;
  bitCount = 2;

  /** @returns - 0 */
  init(): number {
    return 0;
  }

  /** Update the model */
  update(): void {
    // halve counts when a threshold is reached
    if ((this.bitCount += this.updateCycle) > BM__MaxCount) {
      this.bitCount = Math.trunc((this.bitCount + 1) >>> 1);
      this.bit0Count = Math.trunc((this.bit0Count + 1) >>> 1);
      if (this.bit0Count === this.bitCount) ++this.bitCount;
    }

    // compute scaled bit 0 probability
    const scale = Math.trunc(0x80000000 / this.bitCount);
    this.bit0Prob = Math.trunc((this.bit0Count * scale) >>> (31 - BM__LengthShift));

    // set frequency of model updates
    this.updateCycle = Math.trunc((5 * this.updateCycle) >>> 2);
    if (this.updateCycle > 64) this.updateCycle = 64;
    this.bitsUntilUpdate = this.updateCycle;
  }
}
