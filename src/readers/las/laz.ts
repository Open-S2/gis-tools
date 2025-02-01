/** A LAZ Header Item */
export interface LAZHeaderItem {
  // U16 type: 2 bytes * num_items
  type: number;
  // U16 size: 2 bytes * num_items
  size: number;
  // U16 version: 2 bytes * num_items
  version: number;
}

/** A LAZ Header */
export interface LAZHeader {
  // Compressor unsigned short 2 bytes *
  compressor: number;
  // Coder unsigned short 2 bytes *
  coder: number;
  // Version Major unsigned char 1 byte *
  versionMajor: number;
  // Version Minor unsigned char 1 byte *
  versionMinor: number;
  // Version Revision unsigned short 2 bytes *
  versionRevision: number;
  // Options unsigned long 4 bytes *
  options: number;
  // Chunk Size unsigned long 4 bytes *
  chunkSize: number;
  // Number of special EVLRs signed long long 8 bytes *
  numSpecialEvlrs: number;
  // Offset of special EVLRs signed long long 8 bytes *
  offsetSpecialEvlrs: number;
  // Number of Items unsigned short 2 bytes *
  numItems: number;
  // Item records Array of “Item record” 6 bytes * Number of Items *
  items: LAZHeaderItem[];
}

/**
 * # LAZ Parser
 *
 * ## Description
 * A parser for LAZ point data
 *
 * ## Links
 * https://downloads.rapidlasso.de/doc/LAZ_Specification_1.4_R1.pdf
 * https://github.com/LASzip/LASzip/blob/master/src/laszip.cpp
 */
export class LASzip {
  header: LAZHeader;
  /**
   * @param rawHeader - The raw header data
   */
  constructor(rawHeader: DataView) {
    this.header = this.#parseHeader(rawHeader);
  }

  /**
   * Parses the LAZ header
   * @param rawHeader - The raw header data
   * @returns - The parsed header
   */
  #parseHeader(rawHeader: DataView) {
    const header: LAZHeader = {
      compressor: rawHeader.getUint16(0, true),
      coder: rawHeader.getUint16(2, true),
      versionMajor: rawHeader.getUint8(4),
      versionMinor: rawHeader.getUint8(5),
      versionRevision: rawHeader.getUint16(6, true),
      options: rawHeader.getUint32(8, true),
      chunkSize: rawHeader.getUint32(12, true),
      numSpecialEvlrs: Number(rawHeader.getBigInt64(16, true)),
      offsetSpecialEvlrs: Number(rawHeader.getBigInt64(24, true)),
      numItems: rawHeader.getUint16(32, true),
      items: [],
    };
    // Parse items
    for (let i = 0; i < header.numItems; i++) {
      header.items.push({
        type: rawHeader.getUint16(34 + i * 6, true),
        size: rawHeader.getUint16(36 + i * 6, true),
        version: rawHeader.getUint16(38 + i * 6, true),
      });
    }

    return header;
  }
}
