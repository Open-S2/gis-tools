import { headerToBytes, setUint64 } from './pmtiles';

import type { Entry, Header } from './pmtiles';

/** Store entries for each Face */
export interface S2Entries {
  0: Entry[];
  1: Entry[];
  2: Entry[];
  3: Entry[];
  4: Entry[];
  5: Entry[];
}

/** S2PMTiles v3 header storing basic archive-level information. */
export interface S2Header extends Header {
  rootDirectoryOffset1: number;
  rootDirectoryLength1: number;
  rootDirectoryOffset2: number;
  rootDirectoryLength2: number;
  rootDirectoryOffset3: number;
  rootDirectoryLength3: number;
  rootDirectoryOffset4: number;
  rootDirectoryLength4: number;
  rootDirectoryOffset5: number;
  rootDirectoryLength5: number;
  leafDirectoryOffset1: number;
  leafDirectoryLength1: number;
  leafDirectoryOffset2: number;
  leafDirectoryLength2: number;
  leafDirectoryOffset3: number;
  leafDirectoryLength3: number;
  leafDirectoryOffset4: number;
  leafDirectoryLength4: number;
  leafDirectoryOffset5: number;
  leafDirectoryLength5: number;
}

export const S2_HEADER_SIZE_BYTES = 262;

export const S2_ROOT_SIZE = 98_304;

/**
 * @param header - the header object
 * @returns the raw header bytes
 */
export function s2HeaderToBytes(header: S2Header): Uint8Array {
  const defaultHeader: Uint8Array = headerToBytes(header);
  const base = new Uint8Array(S2_HEADER_SIZE_BYTES);
  base.set(defaultHeader, 0);
  const dv = new DataView(base.buffer);
  // re-write the magic number and spec version
  dv.setUint8(0, 'S'.charCodeAt(0));
  dv.setUint8(1, '2'.charCodeAt(0));
  dv.setUint8(7, 1);
  // now add the rest of the header
  setUint64(dv, 102, header.rootDirectoryOffset1);
  setUint64(dv, 110, header.rootDirectoryLength1);
  setUint64(dv, 118, header.rootDirectoryOffset2);
  setUint64(dv, 126, header.rootDirectoryLength2);
  setUint64(dv, 134, header.rootDirectoryOffset3);
  setUint64(dv, 142, header.rootDirectoryLength3);
  setUint64(dv, 150, header.rootDirectoryOffset4);
  setUint64(dv, 158, header.rootDirectoryLength4);
  setUint64(dv, 166, header.rootDirectoryOffset5);
  setUint64(dv, 174, header.rootDirectoryLength5);
  setUint64(dv, 182, header.leafDirectoryOffset1);
  setUint64(dv, 190, header.leafDirectoryLength1);
  setUint64(dv, 198, header.leafDirectoryOffset2);
  setUint64(dv, 206, header.leafDirectoryLength2);
  setUint64(dv, 214, header.leafDirectoryOffset3);
  setUint64(dv, 222, header.leafDirectoryLength3);
  setUint64(dv, 230, header.leafDirectoryOffset4);
  setUint64(dv, 238, header.leafDirectoryLength4);
  setUint64(dv, 246, header.leafDirectoryOffset5);
  setUint64(dv, 254, header.leafDirectoryLength5);

  return base;
}
