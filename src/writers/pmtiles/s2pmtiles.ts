import { S2_HEADER_SIZE_BYTES } from '../../readers/pmtiles';
import { headerToBytes, setUint64 } from './pmtiles';

import type { S2Header } from '../../readers/pmtiles';

/**
 * Create raw header bytes from an S2Header object.
 * @param header - the header object
 * @returns the raw header bytes
 */
export function s2HeaderToBytes(header: S2Header): Uint8Array {
  const defaultHeader = headerToBytes(header);
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
