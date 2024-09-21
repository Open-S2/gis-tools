import { Info, InfoBlock } from './info';

import type { OSMReader } from '.';
import type { PrimitiveBlock } from './primitive';
import type { Pbf as Protobuf } from 'open-vector-tile';

import type {
  VectorFeature,
  VectorGeometry,
  VectorLineString,
  VectorMultiLineString,
  VectorMultiPolygon,
  VectorPoint,
  VectorPolygon,
} from 's2-tools/geometry';

/**
 *
 */
enum MemberType {
  Node = 0,
  Way = 1,
  Relation = 2,
}

/**
 *
 */
export enum RoleType {
  Area,
  Outer,
  Inner,
  From,
  To,
  Via,
  Label,
  AdminCentre,
  Stop,
  Empty,
  Other,
}
/**
 * @param name - string name of the role
 * @returns the RoleType
 */
export function fromStringToRoleType(name: string): RoleType {
  if (name === 'area') return RoleType.Area;
  if (name === 'outer') return RoleType.Outer;
  if (name === 'inner') return RoleType.Inner;
  if (name === 'from') return RoleType.From;
  if (name === 'to') return RoleType.To;
  if (name === 'via') return RoleType.Via;
  if (name === 'label') return RoleType.Label;
  if (name === 'admin_centre') return RoleType.AdminCentre;
  if (name === 'stop') return RoleType.Stop;
  return RoleType.Other;
}

/** Member Options. Relations is skipped as it is not supported / has no use. */
export type Member = NodeMember | WayMember;
/** Node Member */
export interface NodeMember {
  id: number;
  role: RoleType;
  node: VectorPoint;
}
/** Way Member */
export interface WayMember {
  id: number;
  role: RoleType;
  way: VectorLineString;
}

/** Relation coordinates from ways with information about node relations. */
export type RelationGeometry = RelationGeometryLines | RelationGeometryArea;
/** Lines Geometry */
export interface RelationGeometryLines {
  type: 0;
  coordinates: VectorMultiLineString;
  nodes: NodeMember[];
}
/** Area Geometry */
export interface RelationGeometryArea {
  type: 1;
  coordinates: VectorMultiPolygon;
  nodes: NodeMember[];
}

/**
 *
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
  #build?: RelationGeometry;

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
   * @param options
   */
  isFilterable(): boolean {
    const { primitiveBlock: pb, reader } = this;
    const { tagFilter } = reader;
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

  /**
   *
   */
  properties(): Record<string, string> {
    return this.primitiveBlock.tags(this.#keys, this.#vals);
  }

  /**
   * Each member can be node, way or relation.
   * @returns an array of members associated with this relation
   */
  members(): Member[] {
    const { primitiveBlock: pb, reader } = this;
    const res: Member[] = [];
    let memid: number = 0;
    for (let i = 0; i < this.#memids.length; i++) {
      memid += this.#memids[i];
      const role = fromStringToRoleType(pb.getString(this.#rolesSid[i]));
      const curType = this.#types[i];
      if (curType === MemberType.Node) {
        const node = reader.nodes.get(memid);
        if (node !== undefined) res.push({ id: memid, role, node });
      } else if (curType === MemberType.Way) {
        const way = reader.ways.get(memid);
        if (way !== undefined) res.push({ id: memid, role, way });
      } else {
        // Relation -> no-op
      }
    }
    return res;
  }

  /**
   *
   */
  toVectorGeometry(): undefined | RelationGeometry {
    if (this.#build !== undefined) return this.#build;
    const members = this.members();
    const nodes: NodeMember[] = members.filter((m) => 'node' in m);
    this.#build = buildGeometry(
      members.filter((m) => 'way' in m),
      nodes,
    );
    return this.#build;
  }

  /**
   * Convert members to vector geometry. members are grouped by type if applicable.
   * (outer + inner are grouped together to be a polygon/multipolygon for instance)
   */
  toVectorFeature(): undefined | VectorFeature<InfoBlock | undefined> {
    const vg = this.toVectorGeometry();
    if (vg === undefined) return;
    const { type, coordinates } = vg;
    const is3D = false;
    const geometry: VectorGeometry =
      type === 0
        ? coordinates.length === 1
          ? { type: 'LineString', is3D, coordinates: coordinates[0] }
          : { type: 'MultiLineString', is3D, coordinates }
        : coordinates.length === 1
          ? { type: 'Polygon', is3D, coordinates: coordinates[0] }
          : { type: 'MultiPolygon', is3D, coordinates };
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
   * @param relation
   * @param pbf
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
 * Given a group of Members whose type is "way", build a multilinestring or multipolygon Feature.
 * If the ways include an 'outer' or 'inner', then we know its an area, otherwise its a line.
 * @param members - an array of way members
 * @param nodes
 * @returns - a multipolygon
 */
function buildGeometry(members: WayMember[], nodes: NodeMember[]): undefined | RelationGeometry {
  // prep variables
  const polygons: VectorMultiPolygon = [];
  const currentPolygon: VectorPolygon = [];
  const currentRing: VectorLineString = [];

  const isArea =
    members.some((m) => m.role === RoleType.Outer) ||
    members.some((m) => m.role === RoleType.Inner);

  // prepare step: members are stored out of order
  sortMembers(members);

  for (const member of members) {
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
      if (member.role === RoleType.Outer && currentPolygon.length > 0) {
        polygons.push(currentPolygon);
      }
      currentPolygon.push(currentRing);
    }
  }

  // Last step is to build:
  // flush ring if it exists
  if (currentRing.length > 0) currentPolygon.push(currentRing);
  if (!isArea) return { type: 0, coordinates: currentPolygon, nodes };
  // flush the current polygon if it exists
  if (currentPolygon.length > 0) polygons.push(currentPolygon);
  // grab the polys and return a feature
  return { type: 1, coordinates: polygons, nodes };
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
