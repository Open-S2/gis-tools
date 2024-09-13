import { Info } from './info';

import type { PrimitiveBlock } from './primitive';
import type { Pbf as Protobuf } from 'open-vector-tile';
import type { InfoBlock, OSMReader } from '.';

import type { VectorFeature, VectorGeometry, VectorLineString } from 's2-tools/geometry';

/**
 *
 */
export class Way {
  id = -1;
  info?: Info;
  // Parallel arrays
  #keys: number[] = [];
  #vals: number[] = [];
  #refs: number[] = []; // DELTA coded
  // Optional infield lat-lon
  // NOTE: I'm not going to bother implementing this.
  //   #lats: number[] = []; // optional DELTA coded
  //   #lons: number[] = []; // optional DELTA coded
  #build?: VectorLineString;

  /**
   * @param primitiveBlock
   * @param reader
   * @param pbf
   */
  constructor(
    public primitiveBlock: PrimitiveBlock,
    public reader: OSMReader,
    pbf: Protobuf,
  ) {
    pbf.readMessage(this.#readLayer, this);
  }

  /**
   * @param options
   */
  isFilterable(): boolean {
    const { tagFilter } = this.reader;
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
   *
   */
  properties(): Record<string, string> {
    return this.primitiveBlock.tags(this.#keys, this.#vals);
  }

  /**
   *
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
   * @param key
   * @param val
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
   * @param maybeArea
   */
  isArea(): boolean {
    const { upgradeWaysToAreas } = this.reader;
    if (
      (upgradeWaysToAreas &&
        this.#refs.length >= 4 &&
        this.#refs[0] == this.#refs[this.#refs.length - 1]) ||
      this.hasKeyValue('area', 'yes')
    ) {
      return true;
    }
    return false;
  }

  /**
   *
   */
  toVectorGeometry(): undefined | VectorLineString {
    if (this.#build !== undefined) return this.#build;
    const { reader } = this;
    const res: VectorLineString = [];
    const nodeRefs = this.nodeRefs();
    for (const ref of nodeRefs) {
      const node = reader.nodes.get(ref);
      if (node === undefined) return;
      res.push({ ...node });
    }
    this.#build = res;
    return res;
  }

  /**
   *
   */
  toVectorFeature(): undefined | VectorFeature<InfoBlock | undefined> {
    const isArea = this.isArea();
    const coordinates = this.toVectorGeometry();
    if (coordinates === undefined) return;
    const geometry: VectorGeometry = isArea
      ? { type: 'Polygon', is3D: false, coordinates: [coordinates] }
      : { type: 'LineString', is3D: false, coordinates };
    return {
      id: this.id,
      type: 'VectorFeature',
      properties: this.properties(),
      geometry,
      metadata: this.info?.toBlock(),
    };
  }

  /**
   * @param tag
   * @param way
   * @param pbf
   */
  #readLayer(tag: number, way: Way, pbf: Protobuf): void {
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
