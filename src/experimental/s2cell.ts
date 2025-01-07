import wasmBase64 from './s2cell.wasm';
import { fromLonLat, toST } from '../geometry/s2/point';

import type { Face, Point3D } from '../geometry';

/**
 * An S2CellId is a 64-bit unsigned integer that uniquely identifies a cell in the S2
 * cell decomposition. Built to interface with internal wasm functions.
 * The Id in this case is just the index pointing to the structure in wasm
 */
export type S2CellId = number;

/**
 * An S2Cell contains all the information needed to uniquely identify a cell in the S2
 */
export interface S2Cell {
  id: S2CellId;
  compare: (other: S2Cell) => -1 | 0 | 1;
}

/**
 * An S2CellIdComparitor compares two S2CellIds and finds which is smaller
 */
export type WasmS2CellIdComparitor = (id1: S2CellId, id2: S2CellId) => -1 | 0 | 1;
/**
 * @param face - face on the sphere
 * @param s - x position
 * @param t - y position
 * @returns S2CellId pointer in wasm memory
 */
type WasmFromFaceST = (face: Face, s: number, t: number) => S2CellId;

/**
 * Frees the S2CellId
 * @param ptr - pointer in wasm memory
 */
type WasmFreeS2CellId = (ptr: S2CellId) => void;

/**
 * Generator that builds S2Cells from either lon/lat, S2Point, or face/s/t
 */
export class S2CellGenerator {
  instance!: WebAssembly.Instance;
  wasmMemory?: Uint8Array;
  tmpString = '';
  #finalizationRegistry: FinalizationRegistry<number>;
  /**
   * Creates an instance of S2CellGenerator that manages the wasm memory
   */
  constructor() {
    const mod = new WebAssembly.Module(base64ToArrayBuffer(wasmBase64));
    this.instance = new WebAssembly.Instance(mod, {
      env: {},
    });

    this.#finalizationRegistry = new FinalizationRegistry<number>((id: S2CellId): void => {
      const freeS2CellId = this.instance.exports.free_s2_cell_id as WasmFreeS2CellId;
      freeS2CellId(id);
    });
  }

  /**
   * @param lon - longitude
   * @param lat - latitude
   * @returns - an S2Cell with the appropriate id and functions
   */
  fromLonLat(lon: number, lat: number): S2Cell {
    return this.fromS2Point(fromLonLat(lon, lat));
  }

  /**
   * @param point - a vector point on the sphere
   * @returns - an S2Cell with the appropriate id and functions
   */
  fromS2Point(point: Point3D): S2Cell {
    const [face, s, t] = toST(point);
    return this.fromFaceST(face, s, t);
  }

  /**
   * @param face - face on the sphere
   * @param s - x position
   * @param t - y position
   * @returns - an S2Cell with the appropriate id and functions
   */
  fromFaceST(face: Face, s: number, t: number): S2Cell {
    const fromFaceST = this.instance.exports.from_face_st as WasmFromFaceST;
    const comparitor = this.instance.exports.compare_s2_cell_id as WasmS2CellIdComparitor;
    const id = fromFaceST(face, s, t);
    const cell: S2Cell = {
      id,
      /**
       * @param other - other S2Cell to compare to
       * @returns -1 | 0 | 1; -1 if this < other, 0 if this == other, 1 if this > other
       */
      compare: (other: S2Cell): -1 | 0 | 1 => {
        return comparitor(id, other.id);
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
function base64ToArrayBuffer(base64: string): ArrayBufferLike {
  const binaryString = atob(base64);
  const len = binaryString.length;
  const bytes = new Uint8Array(len);
  for (let i = 0; i < len; i++) bytes[i] = binaryString.charCodeAt(i);

  return bytes.buffer;
}
