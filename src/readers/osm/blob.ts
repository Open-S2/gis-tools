import { decompressStream } from '../../util';

import type { PbfReader } from 'pbf-ts';

// headers have a max size of 64KB
export const OSM_MAX_HEADER_SIZE = 65_536; // 64 * 1024;
// blobs have a max size of 32MB
export const OSM_MAX_BLOB_SIZE = 33_554_432; // 32 * 1024 * 1024;

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

  /** @param pbf - the Protobuf object to read from */
  constructor(pbf: PbfReader) {
    pbf.readFields(this.#readLayer, this, 0);
  }

  /**
   * @param tag - the tag of the layer
   * @param blobHeader - the blobHeader to modify
   * @param pbf - the Protobuf object to read from
   */
  #readLayer(tag: number, blobHeader: BlobHeader, pbf: PbfReader): void {
    if (tag === 1) blobHeader.type = pbf.readString();
    else if (tag === 2) blobHeader.indexdata = pbf.readBytes();
    else if (tag === 3) blobHeader.datasize = pbf.readVarint();
    else throw new Error('unknown tag ' + tag);
  }
}

///  STORAGE LAYER: Storing primitives.
/** A Blob is a data block containing the actual data. */
export class Blob {
  raw_size = 0;
  data: Uint8Array | Promise<Uint8Array> = new Uint8Array(0);

  /** @param pbf - the Protobuf object to read from */
  constructor(pbf: PbfReader) {
    pbf.readFields(this.#readLayer, this, 0);
  }

  /**
   * @param tag - the tag of the layer
   * @param blob - the blob to modify
   * @param pbf - the Protobuf object to read from
   */
  #readLayer(tag: number, blob: Blob, pbf: PbfReader): void {
    // no compression
    if (tag === 1) blob.data = pbf.readBytes();
    else if (tag === 2) blob.raw_size = pbf.readVarint();
    // ZLIB:
    else if (tag === 3) blob.data = decompressStream(pbf.readBytes());
    // For LZMA compressed data (optional)
    else if (tag === 4) blob.data = pbf.readBytes();
    // Formerly used for bzip2 compressed data. Deprecated in 2010.
    else if (tag === 5) throw new Error('bzip2 not supported');
    // For LZ4 compressed data (optional)
    else if (tag === 6) blob.data = pbf.readBytes();
    // For ZSTD compressed data (optional)
    else if (tag === 7) blob.data = pbf.readBytes();
    else throw new Error('unknown tag ' + tag);
  }
}
