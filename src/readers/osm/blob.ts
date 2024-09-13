import { decompressStream } from 's2-tools/util';

import type { Pbf as Protobuf } from 'open-vector-tile';

export const MAX_HEADER_SIZE = 64 * 1024;

// blobs have a max size of 32MB
export const MAX_BLOB_SIZE = 32 * 1024 * 1024;

/**
 * A file contains an sequence of fileblock headers, each prefixed by
 * their length in network byte order, followed by a data block
 * containing the actual data. Types starting with a "_" are reserved.
 * example: { type: 'OSMHeader', indexdata: null, datasize: 173 }
 */
export class BlobHeader {
  type: string = '';
  indexdata: Uint8Array = new Uint8Array(0);
  datasize = 0;

  /**
   * @param pbf
   */
  constructor(pbf: Protobuf) {
    pbf.readFields(this.#readLayer, this, 0);
  }

  /**
   * @param tag
   * @param blobHeader
   * @param pbf
   */
  #readLayer(tag: number, blobHeader: BlobHeader, pbf: Protobuf): void {
    if (tag == 1) blobHeader.type = pbf.readString();
    else if (tag == 2) blobHeader.indexdata = pbf.readBytes();
    else if (tag == 3) blobHeader.datasize = pbf.readVarint();
    else throw new Error('unknown tag ' + tag);
  }
}

///  STORAGE LAYER: Storing primitives.
/**
 *
 */
export class Blob {
  raw_size = 0;
  data: Uint8Array | Promise<Uint8Array> = new Uint8Array(0);

  /**
   * @param pbf
   */
  constructor(pbf: Protobuf) {
    pbf.readFields(this.#readLayer, this, 0);
  }

  /**
   * @param tag
   * @param blob
   * @param pbf
   */
  #readLayer(tag: number, blob: Blob, pbf: Protobuf): void {
    // no compression
    if (tag == 1) blob.data = pbf.readBytes();
    else if (tag == 2) blob.raw_size = pbf.readVarint();
    // ZLIB:
    else if (tag == 3) blob.data = decompressStream(pbf.readBytes());
    // For LZMA compressed data (optional)
    else if (tag == 4) blob.data = pbf.readBytes();
    // Formerly used for bzip2 compressed data. Deprecated in 2010.
    else if (tag == 5) throw new Error('bzip2 not supported');
    // For LZ4 compressed data (optional)
    else if (tag == 6) blob.data = pbf.readBytes();
    // For ZSTD compressed data (optional)
    else if (tag == 7) blob.data = pbf.readBytes();
    else throw new Error('unknown tag ' + tag);
  }
}
