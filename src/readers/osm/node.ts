import { intermediateRelationToVectorFeature } from './relation';
import { mergeBBoxes } from '../../geometry';
import { DenseInfo, Info } from './info';

import type { PbfReader } from 'pbf-ts';
import type { PrimitiveBlock } from './primitive';
import type { OSMProperties, OSMReader } from '.';

import type { Metadata } from './primitive';
import type {
  BBox,
  Properties,
  VectorFeature,
  VectorPoint,
  VectorPointGeometry,
} from '../../geometry';

/**
 * Merge an associated relation if it exists
 * @param feature - the node vector feature
 * @param reader - the OSM reader
 */
export async function mergeRelationIfExists(
  feature: VectorFeature<Metadata>,
  reader: OSMReader,
): Promise<void> {
  const { nodeRelationPairs, relations, addBBox } = reader;
  const { id, metadata } = feature;
  const pair = await nodeRelationPairs.get(id ?? -1);
  if (pair === undefined) return;
  const { relationID, role } = pair;
  const relation = await relations.get(relationID);
  if (relation === undefined) return;
  const { properties } = relation;
  if (metadata !== undefined) metadata.relation = { role, properties };
  if (addBBox) {
    const relationVectorFeature = await intermediateRelationToVectorFeature(relation, reader);
    if (relationVectorFeature === undefined) return;
    feature.geometry.bbox = mergeBBoxes(
      feature.geometry.bbox,
      relationVectorFeature.geometry.bbox as BBox,
    );
  }
}

/**
 * Node class
 * contains a single node.
 */
export class Node {
  id = 1;
  info?: Info;
  lat = 0.0;
  lon = 0.0;
  #keys: number[] = [];
  #vals: number[] = [];
  #properties?: OSMProperties;

  /**
   * @param primitiveBlock - the primitive block to access keys and values
   * @param reader - the OSM reader
   * @param pbf - the Protobuf object to read from
   */
  constructor(
    public primitiveBlock: PrimitiveBlock,
    public reader: OSMReader,
    pbf?: PbfReader,
  ) {
    this.primitiveBlock = primitiveBlock;
    if (pbf !== undefined) pbf.readMessage(this.#readLayer, this);
  }

  /**
   * Create a node from a dense representation
   * @param id - the node id
   * @param info - the node info
   * @param keys - list of keys
   * @param vals - list of values
   * @param lat - the latitude
   * @param lon - the longitude
   * @param pb - the primitive block to access keys and values
   * @param reader - the OSM reader
   * @returns - the node
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
   * Check if the node is filterable
   * @returns - true if the node is filterable
   */
  isFilterable(): boolean {
    const { primitiveBlock: pb, reader } = this;
    const { tagFilter, removeEmptyNodes, skipNodes } = reader;
    if (skipNodes) return true;
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
   * Get the properties of the node
   * @returns - the properties
   */
  properties(): OSMProperties {
    if (this.#properties !== undefined) return this.#properties;
    this.#properties = this.primitiveBlock.tags(this.#keys, this.#vals);
    return this.#properties;
  }

  /**
   * @param tag - the tag of the message
   * @param node - the node
   * @param pbf - the Protobuf object to read from
   */
  #readLayer(tag: number, node: Node, pbf: PbfReader): void {
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
   * Gain access to the nodes geometry
   * @returns - the vector feature
   */
  toVectorGeometry(): VectorPoint {
    // if feature has altitude or something defining its z position, make feature 3D
    const { altitude, ele, elevation, height, depth } = this.properties();
    const z = parseAltitude(
      ele ?? height ?? altitude ?? elevation ?? (depth !== undefined ? `-${depth}` : ''),
    );
    return { x: this.lon, y: this.lat, z };
  }

  /**
   * Convert the node to a vector feature
   * @returns - the vector feature
   */
  toVectorFeature(): VectorFeature<Metadata, Properties, OSMProperties, VectorPointGeometry> {
    const { addBBox } = this.reader;
    const bbox = addBBox ? this.buildBBox() : undefined;
    const coordinates = this.toVectorGeometry();
    return {
      id: this.id,
      type: 'VectorFeature',
      properties: this.properties(),
      geometry: { type: 'Point', is3D: coordinates.z !== undefined, coordinates, bbox },
      metadata: { type: 'node', info: this.info?.toBlock() ?? {} },
    };
  }

  /** @returns - the bounding box for this node */
  buildBBox(): BBox {
    return [this.lon, this.lat, this.lon, this.lat];
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
   * @param primitiveBlock - the primitive block to access keys and values
   * @param reader - the OSM reader
   * @param pbf - the Protobuf object to read from
   */
  constructor(
    public primitiveBlock: PrimitiveBlock,
    public reader: OSMReader,
    pbf: PbfReader,
  ) {
    this.primitiveBlock = primitiveBlock;
    pbf.readMessage(this.#readLayer, this);
  }

  /**
   * Access the nodes in this block
   * @returns - the nodes in this block
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
   * @param tag - the tag of the message
   * @param denseNodes - the DenseNodes object
   * @param pbf - the Protobuf object to read from
   */
  #readLayer(tag: number, denseNodes: DenseNodes, pbf: PbfReader): void {
    const { primitiveBlock: pb } = denseNodes;

    if (tag === 1) denseNodes.ids = pbf.readPackedSVarint();
    else if (tag === 5) denseNodes.denseinfo = new DenseInfo(pb, pbf);
    else if (tag === 8) denseNodes.lats = pbf.readPackedSVarint();
    else if (tag === 9) denseNodes.lons = pbf.readPackedSVarint();
    else if (tag === 10) denseNodes.keysVals = pbf.readPackedVarint();
    else throw new Error('unknown tag ' + tag);
  }
}

/**
 * @param alt - the altitude to parse
 * @returns the altitude assuming it is in meters
 */
function parseAltitude(alt: string): number | undefined {
  alt = alt.replace(/\D/g, '');
  if (alt === '') return undefined;
  // common inputs: 246,62␣m - remove all non-digits
  return Number(alt);
}
