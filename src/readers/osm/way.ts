import { Info } from './info';
import { extendBBox } from '../../geometry';

import type { Metadata } from './primitive';
import type { PbfReader } from 'pbf-ts';
import type { PrimitiveBlock } from './primitive';
import type { InfoBlock, OSMProperties, OSMReader } from '.';

import type {
  BBOX,
  Properties,
  VectorFeature,
  VectorGeometry,
  VectorLineString,
} from '../../geometry';

/** Linebased node reference store */
export type WayNodes = number[];

/** An intermediate vector feature where the way nodes haven't been resolved yet. */
export interface IntermediateWay {
  id: number;
  properties: OSMProperties;
  metadata: InfoBlock;
  wayNodes: WayNodes;
  isArea: boolean;
}

/**
 * Convert an intermediate way to a vector feature
 * @param way - intermediate way
 * @param reader - OSM reader
 * @returns - way vector feature
 */
export async function intermediateWayToVectorFeature(
  way: IntermediateWay,
  reader: OSMReader,
): Promise<VectorFeature<Metadata, Properties, OSMProperties> | undefined> {
  const { addBBox } = reader;
  const { id, isArea, wayNodes, properties, metadata } = way;
  // build line
  const vectorLine: VectorLineString = [];
  for (const ref of wayNodes) {
    const node = await reader.nodeGeometry.get(ref);
    if (node === undefined) return;
    vectorLine.push({ ...node });
  }
  let bbox: BBOX | undefined;
  if (addBBox) {
    for (const node of vectorLine) bbox = extendBBox(bbox, node);
  }
  // build geometry
  const geometry: VectorGeometry = isArea
    ? { type: 'Polygon', is3D: false, coordinates: [vectorLine], bbox }
    : { type: 'LineString', is3D: false, coordinates: vectorLine, bbox };

  return {
    id,
    type: 'VectorFeature',
    properties,
    geometry,
    metadata: { type: 'way', info: metadata },
  };
}

/**
 * Way Class
 */
export class Way {
  id = -1;
  info?: Info;
  // Parallel arrays
  #keys: number[] = [];
  #vals: number[] = [];
  #refs: number[] = []; // DELTA coded
  // Optional infield lat-lon
  // NOTE: I'm not going to bother implementing this, I've never seen it used.
  //   #lats: number[] = []; // optional DELTA coded
  //   #lons: number[] = []; // optional DELTA coded

  /**
   * @param primitiveBlock - the primitive block to access keys and values
   * @param reader - the OSM reader
   * @param pbf - the Protobuf object to read from
   */
  constructor(
    public primitiveBlock: PrimitiveBlock,
    public reader: OSMReader,
    pbf: PbfReader,
  ) {
    pbf.readMessage(this.#readLayer, this);
  }

  /**
   * Checks if the way is filterable or not
   * @returns - true if the way is filterable
   */
  isFilterable(): boolean {
    const { tagFilter, skipWays } = this.reader;
    if (skipWays) return true;
    if (tagFilter !== undefined) {
      for (let i = 0; i < this.#keys.length; i++) {
        const keyStr = this.primitiveBlock.getString(this.#keys[i]);
        const valStr = this.primitiveBlock.getString(this.#vals[i]);
        if (tagFilter.matchFound('Way', keyStr, valStr)) return false;
      }
      // if we make it here, we didn't find any matching tags
      return true;
    }
    return false;
  }

  /**
   * Access the way's properties
   * @returns - the way's properties
   */
  properties(): OSMProperties {
    return this.primitiveBlock.tags(this.#keys, this.#vals);
  }

  /**
   * Access the way's node IDs associated with this way
   * @returns - the way's nodes
   */
  nodeRefs(): number[] {
    const res: number[] = [];
    let ref = 0;
    for (let i = 0; i < this.#refs.length; i++) {
      ref += this.#refs[i];
      res.push(ref);
    }
    return res;
  }

  /**
   * Checks if the way has a key-value pair (value optional)
   * @param key - the key
   * @param val - the value
   * @returns - true if the way has the key and value
   */
  hasKeyValue(key: string, val?: string): boolean {
    const { primitiveBlock: pb } = this;
    for (let i = 0; i < this.#keys.length; i++) {
      if (pb.getString(this.#keys[i]) === key) {
        if (val === undefined) return true;
        if (pb.getString(this.#vals[i]) === val) return true;
      }
    }
    return false;
  }

  /**
   * Checks if the way is an area based on it's key-value pairs
   * @returns - true if the way is an area
   */
  isArea(): boolean {
    const { upgradeWaysToAreas } = this.reader;
    if (
      (upgradeWaysToAreas &&
        this.#refs.length >= 4 &&
        this.#refs[0] === this.#refs[this.#refs.length - 1]) ||
      this.hasKeyValue('area', 'yes')
    ) {
      return true;
    }
    return false;
  }

  /**
   * Converts the way to an intermediate vector feature (way's nodes have not been parsed)
   * @returns - the way as an intermediate vector feature
   */
  toVectorFeature(): undefined | IntermediateWay {
    const isArea = this.isArea();
    const wayNodes = this.nodeRefs();
    if (wayNodes.length < 2) return;
    return {
      id: this.id,
      isArea,
      properties: this.properties(),
      wayNodes,
      metadata: this.info?.toBlock() ?? {},
    };
  }

  /**
   * @param tag - the tag of the message
   * @param way - the way to modify
   * @param pbf - the Protobuf object to read from
   */
  #readLayer(tag: number, way: Way, pbf: PbfReader): void {
    if (tag === 1) way.id = pbf.readVarint();
    else if (tag === 2) way.#keys = pbf.readPackedVarint();
    else if (tag === 3) way.#vals = pbf.readPackedVarint();
    else if (tag === 4) way.info = new Info(way.primitiveBlock, pbf);
    else if (tag === 8) way.#refs = pbf.readPackedSVarint();
    // skip, not used.
    else if (tag === 9 || tag === 10) return;
    // else if (tag === 9) way.#lats = pbf.readPackedSVarint();
    // else if (tag === 10) way.#lons = pbf.readPackedSVarint();
    else throw new Error(`Unknown tag: ${tag}`);
  }
}
