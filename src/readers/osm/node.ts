import { DenseInfo, Info } from './info';

import type { PrimitiveBlock } from './primitive';
import type { Pbf as Protobuf } from 'open-vector-tile';
import type { InfoBlock, OSMReader } from '.';

import type { VectorFeature, VectorPoint } from 's2-tools/geometry';

/**
 *
 */
export class Node {
  id = 1;
  info?: Info;
  lat = 0.0;
  lon = 0.0;
  #keys: number[] = [];
  #vals: number[] = [];

  /**
   * @param primitiveBlock
   * @param reader
   * @param pbf
   */
  constructor(
    public primitiveBlock: PrimitiveBlock,
    public reader: OSMReader,
    pbf?: Protobuf,
  ) {
    this.primitiveBlock = primitiveBlock;
    if (pbf !== undefined) pbf.readMessage(this.#readLayer, this);
  }

  /**
   * @param id
   * @param info
   * @param keys
   * @param vals
   * @param lat
   * @param lon
   * @param pb
   * @param reader
   */
  static fromDense(
    id: number,
    info: Info | undefined,
    keys: number[],
    vals: number[],
    lat: number,
    lon: number,
    pb: PrimitiveBlock,
    reader: OSMReader,
  ): Node {
    const node = new Node(pb, reader);
    node.id = id;
    node.info = info;
    node.lat = 0.000000001 * (pb.latOffset + pb.granularity * lat);
    node.lon = 0.000000001 * (pb.latOffset + pb.granularity * lon);
    node.#keys = keys;
    node.#vals = vals;

    return node;
  }

  /**
   * @param options
   */
  isFilterable(): boolean {
    const { primitiveBlock: pb, reader } = this;
    const { tagFilter, removeEmptyNodes } = reader;
    if (removeEmptyNodes && this.#keys.length === 0) return true;
    if (tagFilter !== undefined) {
      for (let i = 0; i < this.#keys.length; i++) {
        const keyStr = pb.getString(this.#keys[i]);
        const valStr = pb.getString(this.#vals[i]);
        if (tagFilter.matchFound('Node', keyStr, valStr)) return false;
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
   * @param tag
   * @param node
   * @param pbf
   */
  #readLayer(tag: number, node: Node, pbf: Protobuf): void {
    const { primitiveBlock: pb } = node;

    if (tag === 1) this.id = pbf.readVarint();
    else if (tag === 2) this.#keys = pbf.readPackedVarint();
    else if (tag === 3) this.#vals = pbf.readPackedVarint();
    else if (tag === 4) this.info = new Info(pb, pbf);
    else if (tag === 8) this.lat = 0.000000001 * pb.latOffset + pb.granularity * pbf.readSVarint();
    else if (tag === 9) this.lon = 0.000000001 * pb.latOffset + pb.granularity * pbf.readSVarint();
    else throw new Error(`Unknown tag: ${tag}`);
  }

  /**
   *
   */
  toVectorGeometry(): VectorPoint {
    // TODO: if feature has altitude or something defining its z position, make feature 3D
    return { x: this.lon, y: this.lat };
  }

  /**
   *
   */
  toVectorFeature(): VectorFeature<InfoBlock | undefined> {
    return {
      id: this.id,
      type: 'VectorFeature',
      properties: this.properties(),
      geometry: { type: 'Point', is3D: false, coordinates: this.toVectorGeometry() },
      metadata: this.info?.toBlock(),
    };
  }
}

/**
 * Used to densly represent a sequence of nodes that do not have any tags.
 * We represent these nodes columnwise as five columns: ID's, lats, and
 * lons, all delta coded. When metadata is not omitted,
 * We encode keys & vals for all nodes as a single array of integers
 * containing key-stringid and val-stringid, using a stringid of 0 as a
 * delimiter between nodes.
 *    ( (<keyid> <valid>)* '0' )*
 */
export class DenseNodes {
  ids: number[] = []; // DELTA coded
  denseinfo?: DenseInfo;
  lats: number[] = []; // DELTA coded
  lons: number[] = []; // DELTA coded
  // Special packing of keys and vals into one array. May be empty if all nodes in this block are tagless.
  keysVals: number[] = [];

  /**
   * @param reader
   * @param pbf
   * @param primitiveBlock
   */
  constructor(
    public primitiveBlock: PrimitiveBlock,
    public reader: OSMReader,
    pbf: Protobuf,
  ) {
    this.primitiveBlock = primitiveBlock;
    pbf.readMessage(this.#readLayer, this);
  }

  /**
   *
   */
  nodes(): Node[] {
    const { primitiveBlock: pb, reader } = this;
    const res: Node[] = [];
    const infoMap = this.denseinfo?.infos();
    let j = 0;
    let curId = 0;
    let curLat = 0;
    let curLon = 0;
    for (let i = 0; i < this.ids.length; i++) {
      const curInfo = infoMap?.[i];
      curId += this.ids[i];
      curLat += this.lats[i];
      curLon += this.lons[i];
      const keys: number[] = [];
      const vals: number[] = [];
      if (this.keysVals.length > 0) {
        while (this.keysVals[j] !== 0) {
          keys.push(this.keysVals[j]);
          vals.push(this.keysVals[j + 1]);
          j += 2;
        }
        j += 1;
      }
      const node = Node.fromDense(curId, curInfo, keys, vals, curLat, curLon, pb, reader);
      res.push(node);
    }

    return res;
  }

  /**
   * @param tag
   * @param denseNodes
   * @param pbf
   */
  #readLayer(tag: number, denseNodes: DenseNodes, pbf: Protobuf): void {
    const { primitiveBlock: pb } = denseNodes;

    if (tag === 1) denseNodes.ids = pbf.readPackedSVarint();
    else if (tag === 5) denseNodes.denseinfo = new DenseInfo(pb, pbf);
    else if (tag === 8) denseNodes.lats = pbf.readPackedSVarint();
    else if (tag === 9) denseNodes.lons = pbf.readPackedSVarint();
    else if (tag === 10) denseNodes.keysVals = pbf.readPackedVarint();
    else throw new Error('unknown tag ' + tag);
  }
}
