import { base64ToArrayBuffer } from '../util';
import wasmBase64 from './uint64.wasm';

/**
 * An Uint64 is a 64-bit unsigned integer that uniquely identifies a cell in the S2
 * cell decomposition. Built to interface with internal wasm functions.
 * The Id in this case is just the index pointing to the structure in wasm
 */
export type Uint64Ref = number;

/** An Uint64LoHi contains the low and high 32 bits of the Uint64 in two components */
export interface Uint64LoHi {
  lo: number;
  hi: number;
}

/** Mathematical operations that can be performed on two Uint64s */
type MathTypes = 'ADD' | 'SUB' | 'DIV' | 'MUL' | 'BIT_AND' | 'BIT_OR' | 'BIT_XOR';

/** An Uint64Comparitor compares two Uint64s and finds which is smaller */
export type WasmUint64Comparitor = (id1: Uint64Ref, id2: Uint64Ref) => -1 | 0 | 1;
/** An Uint64Arithmetic adds two Uint64s and returns the result */
export type WasmArithmetic = (id1: Uint64Ref, id2: Uint64Ref) => Uint64Ref;
/** An Uint64Arithmetic NOT inverses the bits of the Uint64 */
export type WasmNot = (id1: Uint64Ref) => Uint64Ref;
/** Shifts the bits of the Uint64 */
export type WasmShift = (id1: Uint64Ref, shift: number) => Uint64Ref;
/**
 * @param low - lower 32 bits of the Uint64
 * @param high - upper 32 bits of the Uint64
 * @returns Uint64 pointer in wasm memory
 */
type WasmFromLowHigh = (low: number, high: number) => Uint64Ref;
/** Get the lower 32 bits of the Uint64 */
export type WasmLoBits = (id: Uint64Ref) => number;
/** Get the upper 32 bits of the Uint64 */
export type WasmHiBits = (id: Uint64Ref) => number;
/**
 * Frees the Uint64
 * @param ptr - pointer in wasm memory
 */
type WasmFreeUint64 = (ptr: Uint64Ref) => void;

/** Generator that builds Uint64Cells from either lon/lat or face/s/t */
export default class Uint64CellGenerator {
  instance!: WebAssembly.Instance;
  wasmMemory?: Uint8Array;
  tmpString = '';
  #finalizationRegistry: FinalizationRegistry<number>;
  /** Creates an instance of Uint64CellGenerator that manages the wasm memory */
  constructor() {
    const mod = new WebAssembly.Module(base64ToArrayBuffer(wasmBase64));
    this.instance = new WebAssembly.Instance(mod, {
      env: {},
    });

    this.#finalizationRegistry = new FinalizationRegistry<number>((id: number): void => {
      const freeUint64 = this.instance.exports.free_s2_cell_id as WasmFreeUint64;
      freeUint64(id);
    });
  }

  /**
   * @param type
   * @param bits
   * @param input
   */
  shift(type: 'LEFT' | 'RIGHT', bits: number, input: Uint64Cell): Uint64Cell {
    const shift =
      type === 'LEFT'
        ? (this.instance.exports.shift_left as WasmShift)
        : (this.instance.exports.shift_right as WasmShift);
    const id = shift(input.id, bits);
    const cell = new Uint64Cell(id);
    // Register the id with the finalization registry to ensure it gets freed
    this.#finalizationRegistry.register(cell, id);
    return cell;
  }

  /**
   * @param type
   * @param a
   * @param b
   */
  math(type: MathTypes, a: Uint64Cell, b: Uint64Cell): Uint64Cell {
    const math = this.#getMath(type);
    const id = math(a.id, b.id);
    const cell = new Uint64Cell(id);
    // Register the id with the finalization registry to ensure it gets freed
    this.#finalizationRegistry.register(cell, id);
    return cell;
  }

  /**
   * @param type
   */
  #getMath(type: MathTypes): WasmArithmetic {
    if (type === 'ADD') return this.instance.exports.add as WasmArithmetic;
    if (type === 'SUB') return this.instance.exports.sub as WasmArithmetic;
    if (type === 'DIV') return this.instance.exports.div as WasmArithmetic;
    if (type === 'MUL') return this.instance.exports.mul as WasmArithmetic;
    if (type === 'BIT_AND') return this.instance.exports.bit_and as WasmArithmetic;
    if (type === 'BIT_OR') return this.instance.exports.bit_or as WasmArithmetic;
    if (type === 'BIT_XOR') return this.instance.exports.bit_xor as WasmArithmetic;
    throw new Error(`Unknown math type ${type}`);
  }

  /**
   * @param input
   */
  not(input: Uint64Cell): Uint64Cell {
    const bit_not = this.instance.exports.bit_not as WasmNot;
    const id = bit_not(input.id);
    const cell = new Uint64Cell(id);
    // Register the id with the finalization registry to ensure it gets freed
    this.#finalizationRegistry.register(cell, id);
    return cell;
  }

  /**
   * Convert a low/high pair to an Uint64Cell representation
   * @param low - low 32 bits
   * @param high - high 32 bits
   * @returns - an Uint64Cell with the appropriate id and functions
   */
  fromLowHigh(low: number, high: number): Uint64Cell {
    const _fromLowHigh = this.instance.exports.from_low_high as WasmFromLowHigh;
    const id = _fromLowHigh(low, high);
    const cell = new Uint64Cell(id);
    // Register the id with the finalization registry to ensure it gets freed
    this.#finalizationRegistry.register(cell, id);

    return cell;
  }
}

const cellGen = new Uint64CellGenerator();

/**
 *
 */
export class Uint64Cell {
  /**
   * @param id
   */
  constructor(readonly id: Uint64Ref) {}

  /**
   * @param id
   */
  static fromNumber(id: number): Uint64Cell {
    return cellGen.fromLowHigh(id >>> 0, Math.floor(id / 0x100000000) >>> 0);
  }

  /**
   *  NOTE: The whole point of this package is to avoid bigint, but for testing we need to be able
   *  to verify large numbers in a convenient way.
   * @param id - a bigint
   * @returns the Uint64Cell
   */
  static fromBigint(id: bigint): Uint64Cell {
    return cellGen.fromLowHigh(Number(id & 0xffffffffn), Number((id >> 32n) & 0xffffffffn));
  }

  /**
   * @param low
   * @param high
   */
  static fromLowHigh(low: number, high: number): Uint64Cell {
    return cellGen.fromLowHigh(low, high);
  }

  /**
   * @param id
   * @param bits
   */
  shiftLeft(bits: number): Uint64Cell {
    return cellGen.shift('LEFT', bits, this);
  }

  /**
   * @param id
   * @param bits
   */
  shiftRight(bits: number): Uint64Cell {
    return cellGen.shift('RIGHT', bits, this);
  }

  /**
   *
   */
  not(): Uint64Cell {
    return cellGen.not(this);
  }

  /**
   * @param a
   * @param b
   */
  add(b: number | Uint64Cell): Uint64Cell {
    b = typeof b === 'number' ? Uint64(b) : b;
    return cellGen.math('ADD', this, b);
  }

  /**
   * @param a
   * @param b
   */
  sub(b: number | Uint64Cell): Uint64Cell {
    b = typeof b === 'number' ? Uint64(b) : b;
    return cellGen.math('SUB', this, b);
  }

  /**
   * @param a
   * @param b
   */
  mul(b: number | Uint64Cell): Uint64Cell {
    b = typeof b === 'number' ? Uint64(b) : b;
    return cellGen.math('MUL', this, b);
  }

  /**
   * @param a
   * @param b
   */
  div(b: number | Uint64Cell): Uint64Cell {
    b = typeof b === 'number' ? Uint64(b) : b;
    return cellGen.math('DIV', this, b);
  }

  /**
   * @param a
   * @param b
   */
  bitAnd(b: number | Uint64Cell): Uint64Cell {
    b = typeof b === 'number' ? Uint64(b) : b;
    return cellGen.math('BIT_AND', this, b);
  }

  /**
   * @param a
   * @param b
   */
  bitOr(b: number | Uint64Cell): Uint64Cell {
    b = typeof b === 'number' ? Uint64(b) : b;
    return cellGen.math('BIT_OR', this, b);
  }

  /**
   * @param a
   * @param b
   */
  bitXor(b: number | Uint64Cell): Uint64Cell {
    b = typeof b === 'number' ? Uint64(b) : b;
    return cellGen.math('BIT_XOR', this, b);
  }

  /**
   * @param id
   */
  toLoHi(): Uint64LoHi {
    const low_bits = cellGen.instance.exports.low_bits as WasmLoBits;
    const high_bits = cellGen.instance.exports.high_bits as WasmHiBits;
    return {
      lo: low_bits(this.id) >>> 0,
      hi: high_bits(this.id) >>> 0,
    };
  }

  /**
   * Convert the Uint64Cell to a bigint.
   * NOTE: The whole point of this package is to avoid bigint, but for testing we need to be able
   * to verify large numbers in a convenient way.
   * @returns the bigint
   */
  toBigInt(): bigint {
    const loHi = this.toLoHi();
    return BigInt(loHi.lo) + (BigInt(loHi.hi) << 32n);
  }

  /** @returns the number representation of the Uint64Cell. May lose precision. */
  toNumber(): number {
    const loHi = this.toLoHi();
    return loHi.lo + loHi.hi * 2 ** 32;
  }

  /**
   * @param a
   * @param b
   */
  compare(b: number | Uint64Cell): -1 | 0 | 1 {
    const comparitor = cellGen.instance.exports.compare_uint64 as WasmUint64Comparitor;
    b = typeof b === 'number' ? Uint64(b) : b;
    return comparitor(this.id, b.id);
  }

  /**
   * @param b
   */
  eq(b: number | Uint64Cell): boolean {
    return this.compare(b) === 0;
  }

  /**
   * @param b
   */
  neq(b: number | Uint64Cell): boolean {
    return this.compare(b) !== 0;
  }

  /**
   * @param b
   */
  lt(b: number | Uint64Cell): boolean {
    return this.compare(b) === -1;
  }

  /**
   * @param b
   */
  lte(b: number | Uint64Cell): boolean {
    return this.compare(b) <= 0;
  }

  /**
   * @param b
   */
  gt(b: number | Uint64Cell): boolean {
    return this.compare(b) === 1;
  }

  /**
   * @param b
   */
  gte(b: number | Uint64Cell): boolean {
    return this.compare(b) >= 0;
  }
}

/**
 * Convenience function to convert a number to an Uint64Cell
 * @param input - the number to convert
 * @returns - an Uint64Cell with the appropriate id and functions
 */
export function Uint64(input: number | Uint64Cell): Uint64Cell {
  if (typeof input !== 'number') return input;
  return Uint64Cell.fromNumber(input);
}

/**
 * @param lo
 * @param hi
 */
export function Uint64LoHi(lo: number, hi: number): Uint64Cell {
  return cellGen.fromLowHigh(lo, hi);
}
