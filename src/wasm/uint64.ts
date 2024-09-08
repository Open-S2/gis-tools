import wasmBase64 from './uint64.wasm';

import { S2Point, fromST } from '../geometry';

import type { Face, Point3D, S2CellId } from '../geometry';

/**
 * An Uint64 is a 64-bit unsigned integer that uniquely identifies a cell in the S2
 * cell decomposition. Built to interface with internal wasm functions.
 * The Id in this case is just the index pointing to the structure in wasm
 */
export type Uint64 = number;

/** An Uint64 contains all the information needed to uniquely identify a 64-bit cell */
export interface Uint64Cell {
  id: Uint64;
  compare: (other: Uint64Cell) => -1 | 0 | 1;
}

/** An Uint64Comparitor compares two Uint64s and finds which is smaller */
export type WasmUint64Comparitor = (id1: Uint64, id2: Uint64) => -1 | 0 | 1;
/**
 * @param low - lower 32 bits of the Uint64
 * @param high - upper 32 bits of the Uint64
 * @returns Uint64 pointer in wasm memory
 */
type WasmFromLowHigh = (low: number, high: number) => Uint64;

/**
 * Frees the Uint64
 * @param ptr - pointer in wasm memory
 */
type WasmFreeUint64 = (ptr: Uint64) => void;

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

    this.#finalizationRegistry = new FinalizationRegistry<number>((id: Uint64): void => {
      const freeUint64 = this.instance.exports.free_s2_cell_id as WasmFreeUint64;
      freeUint64(id);
    });
  }

  /**
   * Convert a lon/lat to an Uint64Cell representation
   * @param lon - longitude
   * @param lat - latitude
   * @returns - an Uint64Cell with the appropriate id and functions
   */
  fromLonLat(lon: number, lat: number): Uint64Cell {
    const { toST, fromLonLat } = S2Point;
    const [face, s, t] = toST(fromLonLat(lon, lat));
    return this.fromFaceST(face, s, t);
  }

  /**
   * Convert a face/s/t to an Uint64Cell representation
   * @param face - face on the sphere
   * @param s - x position
   * @param t - y position
   * @returns - an Uint64Cell with the appropriate id and functions
   */
  fromFaceST(face: Face, s: number, t: number): Uint64Cell {
    const id = fromST(face, s, t);
    return this.fromBigInt(id);
  }

  /**
   * @param point - a vector point on the sphere
   * @returns - an Uint64Cell with the appropriate id and functions
   */
  fromS2Point(point: Point3D): Uint64Cell {
    // TODO:
  }

  /**
   * Convert a BigInt to an Uint64Cell representation
   * @param id - the 64-bit cell id in BigInt form
   * @returns - an Uint64Cell with the appropriate id and functions
   */
  fromBigInt(id: S2CellId): Uint64Cell {
    const low = Number(id & 0xffffffffn);
    const high = Number(id >> 32n);
    return this.#fromLowHigh(low, high);
  }

  /**
   * Convert a low/high pair to an Uint64Cell representation
   * @param low - low 32 bits
   * @param high - high 32 bits
   * @returns - an Uint64Cell with the appropriate id and functions
   */
  #fromLowHigh(low: number, high: number): Uint64Cell {
    const _fromLowHigh = this.instance.exports.from_low_high as WasmFromLowHigh;
    const _comparitor = this.instance.exports.compare_uint64 as WasmUint64Comparitor;
    const id = _fromLowHigh(low, high);
    const cell: Uint64Cell = {
      id,
      /**
       * @param other - other Uint64Cell to compare to
       * @returns -1 | 0 | 1; -1 if this < other, 0 if this == other, 1 if this > other
       */
      compare: (other: Uint64Cell): -1 | 0 | 1 => {
        return _comparitor(id, other.id);
      },
    };

    // Register the id with the finalization registry to ensure it gets freed
    this.#finalizationRegistry.register(cell, id);

    return cell;
  }
}

/**
 * pollyfill for string to array buffer
 * @param base64 - base64 encoded string
 * @returns converted ArrayBuffer of the string data
 */
function base64ToArrayBuffer(base64: string): ArrayBuffer {
  const binaryString = atob(base64);
  const len = binaryString.length;
  const bytes = new Uint8Array(len);
  for (let i = 0; i < len; i++) bytes[i] = binaryString.charCodeAt(i);

  return bytes.buffer as ArrayBuffer;
}
