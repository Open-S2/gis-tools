import { Info, InfoBlock } from './info';
import { fromMultiLineString, fromMultiPolygon } from 's2-tools/geometry';

import type { OSMReader } from '.';
import type { Pbf as Protobuf } from 's2-tools/readers/protobuf';
import type { Metadata, PrimitiveBlock } from './primitive';

import type {
  BBOX,
  VectorFeature,
  VectorGeometry,
  VectorLineString,
  VectorMultiLineString,
  VectorMultiPolygon,
  VectorPoint,
  VectorPolygon,
} from 's2-tools/geometry';

/** An intermediate vector feature where the ways and nodes haven't been resolved yet. */
export interface IntermediateRelation {
  id: number;
  properties: Record<string, string>;
  members: IntermediateMember[];
  metadata: InfoBlock;
}

/** An intermediate vector feature where the way nodes haven't been resolved yet. */
export type IntermediateMember = IntermediateNodeMember | IntermediateWayMember;
/** An intermediate vector feature where the way nodes haven't been resolved yet. */
export interface IntermediateNodeMember {
  relationID: number;
  role: string;
  node: number;
}
/** An intermediate vector feature where the way nodes haven't been resolved yet. */
export interface IntermediateWayMember {
  role: string;
  way: number;
}

/**
 * @param relation - the intermediate relation
 * @param reader - the OSM reader
 * @returns - the feature in vector format
 */
export async function intermediateRelationToVectorFeature(
  relation: IntermediateRelation,
  reader: OSMReader,
): Promise<VectorFeature<Metadata> | undefined> {
  const { addBBox, nodeGeometry, wayGeometry } = reader;
  const { id, members, properties, metadata } = relation;
  const iNodes: IntermediateNodeMember[] = members.filter((m) => 'node' in m);
  const nodes: NodeMember[] = [];
  for (const { role, node } of iNodes) {
    const n = await nodeGeometry.get(node);
    if (n === undefined) return;
    nodes.push({ id: node, role, node: n });
  }
  const iWays: IntermediateWayMember[] = members.filter((m) => 'way' in m);
  const ways: WayMember[] = [];
  for (const { role, way } of iWays) {
    const w = await wayGeometry.get(way);
    if (w === undefined) return;
    const mappedW: VectorLineString = [];
    for (const nodeID of w) {
      const n = await nodeGeometry.get(nodeID);
      if (n === undefined) return;
      mappedW.push(n);
    }
    ways.push({ id: way, role, way: mappedW });
  }
  const geo = buildGeometry(ways);
  if (geo === undefined) return;

  const { type, coordinates } = geo;
  let bbox: BBOX | undefined;
  if (addBBox) {
    if (type === 0) bbox = fromMultiLineString(coordinates);
    else bbox = fromMultiPolygon(coordinates);
  }
  const is3D = false;
  const geometry: VectorGeometry =
    type === 0
      ? coordinates.length === 1
        ? { type: 'LineString', is3D, coordinates: coordinates[0], bbox }
        : { type: 'MultiLineString', is3D, coordinates, bbox }
      : coordinates.length === 1
        ? { type: 'Polygon', is3D, coordinates: coordinates[0], bbox }
        : { type: 'MultiPolygon', is3D, coordinates, bbox };
  return {
    id,
    type: 'VectorFeature',
    properties,
    geometry,
    metadata: {
      info: metadata,
      nodes: iNodes,
    },
  };
}

/** Member Type can be Node, Way or Relation. */
export enum MemberType {
  Node = 0,
  Way = 1,
  Relation = 2,
}

/** Member Options. Relations is skipped as it is not supported / has no use. */
export type Member = NodeMember | WayMember;
/** Node Member */
export interface NodeMember {
  id: number;
  role: string;
  node: VectorPoint;
}
/** Way Member */
export interface WayMember {
  id: number;
  role: string;
  way: VectorLineString;
}

/** Relation coordinates from ways with information about node relations. */
export type RelationGeometry = RelationGeometryLines | RelationGeometryArea;
/** Lines Geometry */
export interface RelationGeometryLines {
  type: 0;
  coordinates: VectorMultiLineString;
}
/** Area Geometry */
export interface RelationGeometryArea {
  type: 1;
  coordinates: VectorMultiPolygon;
}

/**
 * Relation class contains a collection of nodes, ways and relations as members.
 */
export class Relation {
  id = -1;
  info?: Info;
  // Parallel arrays
  #keys: number[] = [];
  #vals: number[] = [];
  #rolesSid: number[] = []; // This should have been defined as uint32 for consistency, but it is now too late to change it
  #memids: number[] = []; // DELTA encoded
  #types: MemberType[] = [];

  /**
   * @param primitiveBlock - the primitive block
   * @param reader - the OSM reader
   * @param pbf - the Protobuf if provided
   */
  constructor(
    public primitiveBlock: PrimitiveBlock,
    public reader: OSMReader,
    pbf?: Protobuf,
  ) {
    this.primitiveBlock = primitiveBlock;
    if (pbf !== undefined) pbf.readMessage(this.#readLayer, this);
  }

  /** @returns - true if the relation is filterable */
  isFilterable(): boolean {
    const { primitiveBlock: pb, reader } = this;
    const { tagFilter, skipRelations } = reader;
    if (skipRelations) return true;
    if (tagFilter !== undefined) {
      for (let i = 0; i < this.#keys.length; i++) {
        const keyStr = pb.getString(this.#keys[i]);
        const valStr = pb.getString(this.#vals[i]);
        if (tagFilter.matchFound('Relation', keyStr, valStr)) return false;
      }
      // if we make it here, we didn't find any matching tags
      return true;
    }
    return false;
  }

  /** @returns - the properties of the relation */
  properties(): Record<string, string> {
    return this.primitiveBlock.tags(this.#keys, this.#vals);
  }

  /**
   * Each member can be node, way or relation.
   * @returns an array of members associated with this relation
   */
  members(): IntermediateMember[] {
    const { primitiveBlock: pb } = this;
    const res: IntermediateMember[] = [];
    let memid: number = 0;
    for (let i = 0; i < this.#memids.length; i++) {
      memid += this.#memids[i];
      const role = pb.getString(this.#rolesSid[i]);
      const curType = this.#types[i];
      if (curType === MemberType.Node) {
        res.push({ role, node: memid, relationID: this.id });
      } else if (curType === MemberType.Way) {
        res.push({ role, way: memid });
      } else {
        // Relation -> no-op
      }
    }
    return res;
  }

  /** @returns - the feature in intermediate format to build later */
  toIntermediateFeature(): undefined | IntermediateRelation {
    const members = this.members();
    if (members.length === 0) return;
    return {
      id: this.id,
      properties: this.properties(),
      members,
      metadata: this.info?.toBlock() ?? {},
    };
  }

  /**
   * @param tag - the tag
   * @param relation - the relation to update
   * @param pbf - the protobuf to parse from
   */
  #readLayer(tag: number, relation: Relation, pbf: Protobuf): void {
    if (tag === 1) relation.id = pbf.readVarint();
    else if (tag === 2) relation.#keys = pbf.readPackedVarint();
    else if (tag === 3) relation.#vals = pbf.readPackedVarint();
    else if (tag === 4) relation.info = new Info(relation.primitiveBlock, pbf);
    else if (tag === 8) relation.#rolesSid = pbf.readPackedVarint();
    else if (tag === 9) relation.#memids = pbf.readPackedSVarint();
    else if (tag === 10) relation.#types = pbf.readPackedVarint();
    else throw new Error(`unexpected tag ${tag}`);
  }
}

/**
 * @param members - an array of members
 * @returns - an array of node members that have a 'label' or 'admin_centre' role
 */
export function getNodeRelationPairs(members: IntermediateMember[]): IntermediateNodeMember[] {
  const res: IntermediateNodeMember[] = [];
  for (const member of members) {
    if ('node' in member && (member.role === 'label' || member.role === 'admin_centre'))
      res.push(member);
  }
  return res;
}

/**
 * Given a group of Members whose type is "way", build a multilinestring or multipolygon Feature.
 * If the ways include an 'outer' or 'inner', then we know its an area, otherwise its a line.
 * @param ways - an array of way members
 * @returns - a multipolygon
 */
function buildGeometry(ways: WayMember[]): undefined | RelationGeometry {
  // prep variables
  const polygons: VectorMultiPolygon = [];
  const currentPolygon: VectorPolygon = [];
  const currentRing: VectorLineString = [];

  const isArea = ways.some((m) => m.role === 'outer') || ways.some((m) => m.role === 'inner');

  // prepare step: members are stored out of order
  sortMembers(ways);

  for (const member of ways) {
    // Using "isClockwise", depending on whether the ring is outer or inner,
    // we may need to reverse the order of the points. Every time we find the
    // first and last point are the same, close out the ring, add it to the current
    // polygon, and start a new ring. if the current polygon is NOT empty, we store
    // it in the polygons list and start a new one before adding the completed ring.
    // NOTE: Due to the nature of OSM data, it is possible that resulting ring is reversed.
    // Check against the current ring to see if the way needs to be edited.
    //
    // grab the geometry from the member
    const geometry = member.way;
    if (geometry === undefined) return;
    // store in current ring, checking current rings order

    if (currentRing.length === 0) {
      currentRing.push(...geometry);
    } else {
      currentRing.push(...geometry.slice(1));
    }
    // if current rings first and last point are the same, close out the ring
    if (equalPoints(currentRing[0], currentRing[currentRing.length - 1])) {
      // add the ring to the current polygon. If member role is outer and
      // currentPolygon already has data, we need to store the current poly and
      // start a new polygon.
      // If the member role is inner, we can add the ring to the
      // current polygon.
      if (member.role === 'outer' && currentPolygon.length > 0) {
        polygons.push(currentPolygon);
      }
      currentPolygon.push(currentRing);
    }
  }

  // Last step is to build:
  // flush ring if it exists
  if (currentRing.length > 0) currentPolygon.push(currentRing);
  if (!isArea) return { type: 0, coordinates: currentPolygon };
  // flush the current polygon if it exists
  if (currentPolygon.length > 0) polygons.push(currentPolygon);
  // grab the polys and return a feature
  return { type: 1, coordinates: polygons };
}

/**
 * @param a - the first point
 * @param b - the second point
 * @returns true if the points are equal
 */
function equalPoints(a: VectorPoint, b: VectorPoint): boolean {
  return a.x === b.x && a.y === b.y;
}

/**
 * osm throws relation members out of order, so we need to not only sort them
 * but also check if the first and last points of each way follow the same direction.
 * @param members - the ways to be sorted
 */
function sortMembers(members: WayMember[]): void {
  if (members.length < 3) return;
  for (let i = 0; i < members.length - 1; i++) {
    const curWay = members[i].way;
    const curFirstPoint = curWay[0];
    const curLastPoint = curWay[curWay.length - 1];
    // if current way is already self closing break
    if (curFirstPoint === curLastPoint) break;
    for (let j = i + 1; j < members.length; j++) {
      const nextWay = members[j].way;
      const nextFirstPoint = nextWay[0];
      const nextLastPoint = nextWay[nextWay.length - 1];
      // if we find a match between any of the points, swap the member positions
      // if curFirstPoint == nextFirstPoint or curLastPoint == nextLastPoint
      // swap the order
      const equalFirst = equalPoints(curFirstPoint, nextFirstPoint);
      const equalLast = equalPoints(curLastPoint, nextLastPoint);
      const equalFirstLast = equalPoints(curFirstPoint, nextLastPoint);
      const equalLastFirst = equalPoints(curLastPoint, nextFirstPoint);
      if (equalFirst || equalLast || equalFirstLast || equalLastFirst) {
        if (equalFirst) {
          curWay.reverse();
        } else if (equalLast) {
          nextWay.reverse();
        } else if (equalFirstLast) {
          curWay.reverse();
          nextWay.reverse();
        }
        // we want to move the found member to be next to the current member
        if (i + 1 !== j) {
          const temp = members[i + 1];
          members[i + 1] = members[j];
          members[j] = temp;
        }
        break;
      }
    }
  }
}
