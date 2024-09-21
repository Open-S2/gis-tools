import { HeaderBlock } from './headerBlock';
import { PrimitiveBlock } from './primitive';
import { Pbf as Protobuf } from 'open-vector-tile';
import { Blob, BlobHeader } from './blob';

import type { InfoBlock } from './info';
import type { KVStore } from 's2-tools/dataStore';
import type { OSMHeader } from './headerBlock';
import type { FeatureIterator, Reader } from '..';
import type { VectorFeature, VectorLineString, VectorPoint } from 's2-tools/geometry';

export type * from './blob';
export type * from './headerBlock';
export type * from './info';
export type * from './node';
export type * from './relation';
export type * from './way';

// TODO: Add bbox for each node, way, relation toVectorFeature

/**
 *
 */
export type FilterType = 'All' | 'Node' | 'Way' | 'Relation';

/**
 *
 */
export type FilterMap = Map<string, string | undefined>;

/**
 *
 */
export class TagFilter {
  #filters: FilterMap = new Map<string, string | undefined>();
  #nodeFilters: FilterMap = new Map<string, string | undefined>();
  #wayFilters: FilterMap = new Map<string, string | undefined>();
  #relationFilters: FilterMap = new Map<string, string | undefined>();

  /**
   * @param filterType
   */
  #getFilter(filterType: FilterType): FilterMap {
    if (filterType === 'All') return this.#filters;
    else if (filterType === 'Node') return this.#nodeFilters;
    else if (filterType === 'Way') return this.#wayFilters;
    else return this.#relationFilters;
  }

  /**
   * @param filterType
   * @param key
   * @param value
   */
  addFilter(filterType: FilterType, key: string, value?: string): void {
    const filter: FilterMap = this.#getFilter(filterType);
    filter.set(key, value);
  }

  /**
   * @param filterType
   * @param key
   * @param value
   */
  matchFound(filterType: FilterType, key: string, value?: string): boolean {
    const filter: FilterMap = this.#getFilter(filterType);
    // check all filters first
    if (this.#filters.has(key)) {
      const filterValue = this.#filters.get(key);
      if (filterValue === value) return true;
    }
    // check type-specific filters
    if (filterType !== 'All' && filter.has(key)) {
      const filterValue = filter.get(key);
      if (filterValue === value) return true;
    }
    return false;
  }
}

/** OSM Reader options */
export interface OsmReaderOptions {
  /** if true, remove nodes that have no tags [Default = true] */
  removeEmptyNodes?: boolean;
  /** If provided, filters of the  */
  tagFilter?: TagFilter;
  /** If set to true, nodes will be skipped. [Default = false] */
  skipNodes?: boolean;
  /** If set to true, ways will be skipped. [Default = false] */
  skipWays?: boolean;
  /** If set to true, relations will be skipped. [Default = false] */
  skipRelations?: boolean;
  /**
   * If set to true, ways will be converted to areas if they are closed.
   * NOTE: They are upgraded anyways if the tag "area" is set to "yes".
   * [Default = false]
   */
  upgradeWaysToAreas?: boolean;
}

/**
 *
 */
export class OSMReader implements FeatureIterator<InfoBlock | undefined> {
  /** if true, remove nodes that have no tags [Default = true] */
  removeEmptyNodes: boolean;
  /** If provided, filters of the  */
  tagFilter?: TagFilter;
  /** If set to true, nodes will be skipped */
  skipNodes: boolean;
  /** If set to true, ways will be skipped */
  skipWays: boolean;
  /** If set to true, relations will be skipped */
  skipRelations: boolean;
  /**
   * If set to true, ways will be converted to areas if they are closed.
   * NOTE: They are upgraded anyways if the tag "area" is set to "yes".
   * [Default = false]
   */
  upgradeWaysToAreas: boolean;

  nodes: KVStore<number, VectorPoint> = new Map<number, VectorPoint>();
  ways: KVStore<number, VectorLineString> = new Map<number, VectorLineString>();
  #offset = 0;

  /**
   * @param reader
   * @param options
   */
  constructor(
    public reader: Reader,
    public options?: OsmReaderOptions,
  ) {
    this.removeEmptyNodes = options?.removeEmptyNodes ?? true;
    this.tagFilter = options?.tagFilter;
    this.skipNodes = options?.skipNodes ?? false;
    this.skipWays = options?.skipWays ?? false;
    this.skipRelations = options?.skipRelations ?? false;
    this.upgradeWaysToAreas = options?.upgradeWaysToAreas ?? false;
  }

  /**
   *
   */
  // async *iterate(): AsyncGenerator<VectorFeature<InfoBlock | undefined>> {
  async *[Symbol.asyncIterator](): AsyncGenerator<VectorFeature<InfoBlock | undefined>> {
    this.#offset = 0;
    // skip the header
    await this.#next();

    while (true) {
      const blob = await this.#next();
      if (blob === undefined) break;
      const pb = await this.#readBlob(blob);
      for (const pg of pb.primitiveGroups) {
        for (const node of pg.nodes) yield node.toVectorFeature();
        for (const way of pg.ways) {
          const feature = way.toVectorFeature();
          if (feature !== undefined) yield feature;
        }
        for (const relation of pg.relations) {
          const feature = relation.toVectorFeature();
          if (feature !== undefined) yield feature;
        }
      }
    }
  }

  /**
   *
   */
  async getHeader(): Promise<OSMHeader> {
    this.#offset = 0;
    const blobHeader = await this.#next();
    if (blobHeader === undefined) throw new Error('Header not found');
    const headerBlock = new HeaderBlock(new Protobuf(new Uint8Array(blobHeader.buffer)));

    return headerBlock.toHeader();
  }

  /**
   * @param isHeader
   * @param offset
   */
  async #next(): Promise<undefined | DataView> {
    const { reader } = this;
    // if we've already read all the data, return null
    if (this.#offset >= reader.byteLength) return;
    // STEP 1: Get blob size
    // read length of current blob
    const length = reader.getInt32(this.#offset);
    this.#offset += 4;
    const blobHeaderData = reader.slice(this.#offset, this.#offset + length);
    this.#offset += length;
    // build a blob header
    const pbf = new Protobuf(new Uint8Array(blobHeaderData.buffer));
    const blobHeader = new BlobHeader(pbf);
    // STEP 2: Get blob data
    const compressedBlobData = reader.slice(this.#offset, this.#offset + blobHeader.datasize);
    this.#offset += blobHeader.datasize;

    return compressedBlobData;
  }

  /**
   * @param blob
   * @param data
   */
  async #readBlob(data: DataView): Promise<PrimitiveBlock> {
    // Blob data is PBF encoded and ?compressed, so we need to parse & decompress it first
    let pbf = new Protobuf(new Uint8Array(data.buffer));
    const blob = new Blob(pbf);
    pbf = new Protobuf(await blob.data);
    // Parse the PrimitiveBlock and read its contents.
    // all nodes/ways/relations that can be filtered already are on invocation.
    return new PrimitiveBlock(pbf, this);
  }
}
