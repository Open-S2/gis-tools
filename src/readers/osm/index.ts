import { HeaderBlock } from './headerBlock';
import { KV } from '../../dataStore';
import { PrimitiveBlock } from './primitive';
import { Pbf as Protobuf } from '../../readers/protobuf';
import { intermediateRelationToVectorFeature } from './relation';
import { intermediateWayToVectorFeature } from './way';
import { mergeRelationIfExists } from './node';
import { tmpdir } from 'os';
import { Blob, BlobHeader } from './blob';

import type { Metadata } from './primitive';
import type { OSMHeader } from './headerBlock';
import type { FeatureIterator, Reader } from '..';
import type { IntermediateNodeMember, IntermediateRelation } from './relation';
import type { IntermediateWay, WayNodes } from './way';
import type { KVStore, KVStoreConstructor } from '../../dataStore';
import type { VectorFeature, VectorPoint } from '../../geometry';

export type * from './blob';
export type * from './headerBlock';
export type * from './info';
export type * from './node';
export type * from './primitive';
export type * from './relation';
export type * from './way';

// https://wiki.openstreetmap.org/wiki/PBF_Format#File_format
// https://github.com/openstreetmap/pbf/blob/master/OSM-binary.md

// TODO: Add threads for reading the blocks

/**
 * Filter types
 * Options are:
 * - All
 * - Node
 * - Way
 * - Relation
 */
export type FilterType = 'All' | 'Node' | 'Way' | 'Relation';

/** Filter map. Used internally by TagFilter */
export type FilterMap = Map<string, string | undefined>;

/**
 * TagFilter Class
 * Builds a filter for the tags when parsing data.
 * Can parse tags from nodes, ways and relations.
 * Also allows the ability to add tags that apply to all object types.
 * Can filter by key, but also both key and value.
 */
export class TagFilter {
  #filters: FilterMap = new Map<string, string | undefined>();
  #nodeFilters: FilterMap = new Map<string, string | undefined>();
  #wayFilters: FilterMap = new Map<string, string | undefined>();
  #relationFilters: FilterMap = new Map<string, string | undefined>();

  /**
   * Internal method to get the correct filter map
   * @param filterType - The filter type
   * @returns - The correct filter map to check against
   */
  #getFilter(filterType: FilterType): FilterMap {
    if (filterType === 'All') return this.#filters;
    else if (filterType === 'Node') return this.#nodeFilters;
    else if (filterType === 'Way') return this.#wayFilters;
    else return this.#relationFilters;
  }

  /**
   * Add a filter
   * @param filterType - The filter type to apply the filter
   * @param key - The key to apply the filter
   * @param value - The value to apply the filter (optional)
   */
  addFilter(filterType: FilterType, key: string, value?: string): void {
    const filter: FilterMap = this.#getFilter(filterType);
    filter.set(key, value);
  }

  /**
   * Check if a filter has been found
   * @param filterType - The filter type
   * @param key - The key
   * @param value - The value (optional)
   * @returns - True if the filter has been found
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
  /** If set to true, add a bbox property to each feature */
  addBBox?: boolean;
  /** TODO: If defined, replace the stores with the provided class */
  store?: KVStoreConstructor;
}

/**
 * OSM Reader
 * Parses OSM PBF files
 * https://wiki.openstreetmap.org/wiki/PBF_Format
 */
export class OSMReader implements FeatureIterator<Metadata> {
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
  /** If set to true, add a bbox property to each feature */
  addBBox: boolean;

  nodeGeometry: KVStore<VectorPoint> = new KV<VectorPoint>();
  nodes: KVStore<VectorFeature<Metadata>> = new KV<VectorFeature<Metadata>>();
  wayGeometry: KVStore<WayNodes> = new KV<WayNodes>();
  ways: KVStore<IntermediateWay> = new KV<IntermediateWay>();
  relations: KVStore<IntermediateRelation> = new KV<IntermediateRelation>();
  nodeRelationPairs: KVStore<IntermediateNodeMember> = new KV<IntermediateNodeMember>();
  #offset = 0;

  /**
   * @param reader - The reader input (may be a local memory filter or file reader)
   * @param options - User defined options to apply when reading the OSM file
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
    this.addBBox = options?.addBBox ?? false;
    const store = options?.store;
    if (store !== undefined) {
      this.nodeGeometry = new store(buildTmpFileName('nodeGeometry')) as KVStore<VectorPoint>;
      this.nodes = new store(buildTmpFileName('nodes')) as KVStore<VectorFeature<Metadata>>;
      this.wayGeometry = new store(buildTmpFileName('wayGeometry')) as KVStore<WayNodes>;
      this.ways = new store(buildTmpFileName('ways')) as KVStore<IntermediateWay>;
      this.relations = new store(buildTmpFileName('relations')) as KVStore<IntermediateRelation>;
      this.nodeRelationPairs = new store(
        buildTmpFileName('nodeRelationPairs'),
      ) as KVStore<IntermediateNodeMember>;
    }
  }

  /**
   * An async iterator to read in each feature
   * @yields {VectorFeature}
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<VectorFeature<Metadata>> {
    this.#offset = 0;
    // skip the header
    this.#next();

    // PARSE
    while (true) {
      const blob = this.#next();
      if (blob === undefined) break;
      await this.#readBlob(blob);
    }
    // NODES
    if (!this.skipNodes) {
      for await (const node of this.nodes) {
        await mergeRelationIfExists(node, this);
        yield node;
      }
    }
    // WAYS
    if (!this.skipWays) {
      for await (const interWay of this.ways) {
        const way = await intermediateWayToVectorFeature(interWay, this);
        if (way !== undefined) yield way;
      }
    }
    // RELATIONS
    if (!this.skipRelations) {
      for await (const interRelation of this.relations) {
        const relation = await intermediateRelationToVectorFeature(interRelation, this);
        if (relation !== undefined) yield relation;
      }
    }
  }

  /**
   * @returns - The header of the OSM file
   */
  getHeader(): OSMHeader {
    this.#offset = 0;
    const blobHeader = this.#next();
    if (blobHeader === undefined) throw new Error('Header not found');
    const headerBlock = new HeaderBlock(new Protobuf(new Uint8Array(blobHeader.buffer)));

    return headerBlock.toHeader();
  }

  /**
   * Read the next blob
   * @returns - the next blob if it exists
   */
  #next(): undefined | DataView<ArrayBuffer> {
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
   * Read the input blob and parse the block of data
   * @param data - the data to parse
   * @returns - the parsed primitive block
   */
  async #readBlob(data: DataView<ArrayBuffer>): Promise<PrimitiveBlock> {
    // Blob data is PBF encoded and ?compressed, so we need to parse & decompress it first
    let pbf = new Protobuf(new Uint8Array(data.buffer));
    const blob = new Blob(pbf);
    pbf = new Protobuf(await blob.data);
    // Parse the PrimitiveBlock and read its contents.
    // all nodes/ways/relations that can be filtered already are on invocation.
    return new PrimitiveBlock(pbf, this);
  }

  /** Close out the data which will cleanup any temporary files if they exist */
  close(): void {
    this.nodeGeometry.close();
    this.nodes.close();
    this.wayGeometry.close();
    this.ways.close();
    this.relations.close();
    this.nodeRelationPairs.close();
  }
}

/**
 * Build a temporary file name
 * @param name - the name of the temporary file
 * @returns - a temporary file name based on a random number.
 */
function buildTmpFileName(name: string): string {
  const tmpd = tmpdir();
  const randomName = Math.random().toString(36).slice(2);
  return `${tmpd}/${name ?? ''}_${randomName}`;
}
