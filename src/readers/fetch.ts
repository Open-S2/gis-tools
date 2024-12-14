import type { Reader } from '.';

/** The browser reader that fetches data from a URL. */
export class FetchReader implements Reader {
  byteLength = 0;
  byteOffset = 0;
  /**
   * @param path - the location of the PMTiles data
   * @param rangeRequests - FetchReader specific; enable range requests or use urlParam "bytes"
   */
  constructor(
    public path: string,
    public rangeRequests: boolean,
  ) {}

  /**
   * Not applicable for FetchReader
   * @param _byteOffset - offset
   * @param _littleEndian - le or be
   * @returns - 0
   */
  getBigInt64(_byteOffset: number, _littleEndian?: boolean): bigint {
    return 0n;
  }
  /**
   * Not applicable for FetchReader
   * @param _byteOffset - offset
   * @param _littleEndian - le or be
   * @returns - 0
   */
  getBigUint64(_byteOffset: number, _littleEndian?: boolean): bigint {
    return 0n;
  }
  /**
   * Not applicable for FetchReader
   * @param _byteOffset - offset
   * @param _littleEndian - le or be
   * @returns - 0
   */
  getFloat32(_byteOffset: number, _littleEndian?: boolean): number {
    return 0;
  }
  /**
   * Not applicable for FetchReader
   * @param _byteOffset - offset
   * @param _littleEndian - le or be
   * @returns - 0
   */
  getFloat64(_byteOffset: number, _littleEndian?: boolean): number {
    return 0;
  }
  /**
   * Not applicable for FetchReader
   * @param _byteOffset - offset
   * @param _littleEndian - le or be
   * @returns - 0
   */
  getInt16(_byteOffset: number, _littleEndian?: boolean): number {
    return 0;
  }
  /**
   * Not applicable for FetchReader
   * @param _byteOffset - offset
   * @param _littleEndian - le or be
   * @returns - 0
   */
  getInt32(_byteOffset: number, _littleEndian?: boolean): number {
    return 0;
  }
  /**
   * Not applicable for FetchReader
   * @param _byteOffset - offset
   * @returns - 0
   */
  getInt8(_byteOffset: number): number {
    return 0;
  }
  /**
   * Not applicable for FetchReader
   * @param _byteOffset - offset
   * @param _littleEndian - le or be
   * @returns - 0
   */
  getUint16(_byteOffset: number, _littleEndian?: boolean): number {
    return 0;
  }
  /**
   * Not applicable for FetchReader
   * @param _byteOffset - offset
   * @param _littleEndian - le or be
   * @returns - 0
   */
  getUint32(_byteOffset: number, _littleEndian?: boolean): number {
    return 0;
  }
  /**
   * Not applicable for FetchReader
   * @param _byteOffset - offset
   * @returns - 0
   */
  getUint8(_byteOffset: number): number {
    return 0;
  }
  /**
   * Not applicable for FetchReader
   * @param _begin - beginning
   * @param _end - end
   * @returns - empty DataView
   */
  slice(_begin: number, _end: number): DataView<ArrayBuffer> {
    return new DataView(new Uint8Array([]).buffer);
  }
  /**
   * Not applicable for FetchReader
   * @param _encoding - does nothing
   */
  setStringEncoding(_encoding: string): void {}
  /**
   * Not applicable for FetchReader
   * @param _byteOffset - offset
   * @param _byteLength - length
   * @returns - empty string
   */
  parseString(_byteOffset: number, _byteLength: number): string {
    return '';
  }

  /**
   * @param offset - the offset of the range
   * @param length - the length of the range
   * @returns - the ranged buffer
   */
  async getRange(offset: number, length: number): Promise<Uint8Array<ArrayBuffer>> {
    const bytes = String(offset) + '-' + String(offset + length);
    const fetchReq = this.rangeRequests
      ? fetch(this.path, { headers: { Range: `bytes=${offset}-${offset + length - 1}` } })
      : fetch(`${this.path}&bytes=${bytes}`);
    const res = await fetchReq.then(async (res) => await res.arrayBuffer());
    return new Uint8Array(res, 0, res.byteLength);
  }
}
