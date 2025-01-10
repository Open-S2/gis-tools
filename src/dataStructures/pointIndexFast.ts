import { EARTH_RADIUS } from '..';
import { KDSpatialIndex } from '../dataStore';
import { PriorityQueue } from './priorityQueue';
import { fromST } from '../geometry/s2/point';
import { toWM } from '../geometry';
import { xyzToLonLat } from '../geometry/s2/coords';

import type { Face, FeatureIterator, MValue, Properties, VectorPoint } from '..';
import type { KDStore, KDStoreConstructor } from '../dataStore';

/** A query node in the kd-tree used by a spherical search */
interface NodeQuery<M extends MValue = Properties> {
  // index?: number;
  point?: VectorPoint<M>;
  left?: number; // left index in the kd-tree array
  right?: number; // right index
  axis?: number; // 0 for longitude axis and 1 for latitude axis
  dist: number; // will hold the lower bound of children's distances to the query point
  minLon?: number; // bounding box of the node
  minLat?: number;
  maxLon?: number;
  maxLat?: number;
}

const RAD = 0.017453292519943295; // Math.PI / 180;

/**
 * # Point Index Fast
 *
 * ## Description
 * An index of cells with radius queries
 * Assumes the data is compatible with {@link Properties}
 * Because of the nature of low level language like Javascript, using u64 is slow. This index
 * uses f64 which Number supports. So it is fast and efficient.
 *
 * ## Usage
 * ```ts
 * import { PointIndexFast } from 'gis-tools';
 * import { KDMMapSpatialIndex } from 'gis-tools/mmap';
 *
 * const pointIndex = new PointIndexFast();
 * // or used a mmap based store
 * const pointIndex = new PointIndex(KDMMapSpatialIndex);
 *
 * // insert a lon-lat
 * pointIndex.insertLonLat(lon, lat, data);
 * // insert an STPoint
 * pointIndex.insertFaceST(face, s, t, data);
 *
 * // after adding data build the index. NOTE: You don't have to call this, it will be called
 * // automatically when making a query
 * await pointIndex.sort();
 *
 * // you can search a range
 * const points = await pointIndex.searchRange(minX, minY, maxX, maxY);
 * // or a standard radius search
 * const points = await pointIndex.searchRadius(qx, qy, r);
 * // or a spherical radius search that wraps around the -180/180 boundary
 * const points = await pointIndex.searchRadiusSphere(lon, lat, dist);
 * ```
 */
export class PointIndexFast<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
> {
  #store: KDStore<D | P>;
  #sorted: boolean = false;

  /**
   * @param store - the store to index. May be an in memory or disk
   * @param nodeSize - the size of each kd-tree node
   */
  constructor(
    store: KDStoreConstructor<D | P> = KDSpatialIndex,
    private readonly nodeSize = 64,
  ) {
    this.#store = new store(nodeSize);
  }

  /**
   * Add a properly shaped point with it's x, y, and data values
   * @param point - the point to be indexed
   */
  insert(point: VectorPoint<D | P>): void {
    this.#store.push(point);
    this.#sorted = false;
  }

  /**
   * Add all points from a reader. It will try to use the M-value first, but if it doesn't exist
   * it will use the feature properties data
   * @param reader - a reader containing the input data
   */
  async insertReader(reader: FeatureIterator<M, D, P>): Promise<void> {
    for await (const feature of reader) {
      if (feature.geometry.type !== 'Point' && feature.geometry.type !== 'MultiPoint') continue;
      const {
        geometry: { coordinates, type },
      } = feature.type === 'S2Feature' ? toWM(feature) : feature;
      if (type === 'Point') {
        this.insertLonLat(coordinates.x, coordinates.y, coordinates.m ?? feature.properties);
      } else if (type === 'MultiPoint') {
        for (const { x, y, m } of coordinates) this.insertLonLat(x, y, m ?? feature.properties);
      }
    }
  }

  /**
   * Add a lon-lat pair to the cluster
   * @param lon - longitude in degrees
   * @param lat - latitude in degrees
   * @param data - the data associated with the point
   */
  insertLonLat(lon: number, lat: number, data: D | P): void {
    this.insert({ x: lon, y: lat, m: data });
  }

  /**
   * Insert an STPoint to the index
   * @param face - the face of the cell
   * @param s - the s coordinate
   * @param t - the t coordinate
   * @param data - the data associated with the point
   */
  insertFaceST(face: Face, s: number, t: number, data: D | P): void {
    const [lon, lat] = xyzToLonLat(fromST(face, s, t));
    this.insert({ x: lon, y: lat, m: data });
  }

  /**
   * iterate through the points
   * @yields a PointShapeFast<T>
   */
  *[Symbol.iterator](): Generator<VectorPoint<D | P>> {
    this.sort();
    for (const value of this.#store) yield value;
  }

  /** Perform indexing of the added points. */
  sort(): void {
    if (this.#sorted) return;
    // kd-sort both arrays for efficient search
    this.#store.sort();
    this.#sorted = true;
  }

  /**
   * Search the index for items within a given bounding box.
   * @param minX - the min x coordinate
   * @param minY - the min y coordinate
   * @param maxX - the max x coordinate
   * @param maxY - the max y coordinate
   * @param maxResults - the maximum number of results
   * @returns - the items that are in range
   */
  searchRange(
    minX: number,
    minY: number,
    maxX: number,
    maxY: number,
    maxResults = Infinity,
  ): Array<VectorPoint<D | P>> {
    this.sort();

    const { nodeSize } = this;
    const stack: Array<[left: number, right: number, axis: number]> = [
      [0, this.#store.length - 1, 0],
    ];
    const result: Array<VectorPoint<D | P>> = []; // ids of items that are in range

    // recursively search for items in range in the kd-sorted arrays
    while (stack.length > 0) {
      const [left, right, axis] = stack.pop() as [left: number, right: number, axis: number];

      // if we reached "tree node", search linearly
      if (right - left <= nodeSize) {
        for (const point of this.#store.getRange(left, right + 1)) {
          const { x, y } = point;
          if (x >= minX && x <= maxX && y >= minY && y <= maxY) {
            result.push(point);
            if (result.length >= maxResults) return result;
          }
        }
        continue;
      }

      // otherwise find the middle index
      const m = (left + right) >> 1;
      // include the middle item if it's in range
      const mPoint = this.#store.get(m);
      const { x, y } = mPoint;
      if (x >= minX && x <= maxX && y >= minY && y <= maxY) {
        result.push(mPoint);
        if (result.length >= maxResults) return result;
      }
      // queue search in halves that intersect the query
      if (axis === 0 ? minX <= x : minY <= y) stack.push([left, m - 1, 1 - axis]);
      if (axis === 0 ? maxX >= x : maxY >= y) stack.push([m + 1, right, 1 - axis]);
    }

    return result;
  }

  /**
   * Search the index for items within a given radius.
   * @param qx - the query x coordinate
   * @param qy - the query y coordinate
   * @param r - the radius
   * @param maxResults - the maximum number of results
   * @returns - the items that are in range
   */
  searchRadius(
    qx: number,
    qy: number,
    r: number,
    maxResults = Infinity,
  ): Array<VectorPoint<D | P>> {
    this.sort();

    const { nodeSize } = this;
    const stack: Array<[left: number, right: number, axis: number]> = [
      [0, this.#store.length - 1, 0],
    ]; // left, right, axis
    const result: Array<VectorPoint<D | P>> = []; // ids of items that are in range
    const r2 = r * r;

    // recursively search for items within radius in the kd-sorted arrays
    while (stack.length > 0) {
      const [left, right, axis] = stack.pop() as [left: number, right: number, axis: number];

      // if we reached "tree node", search linearly
      if (right - left <= nodeSize) {
        for (const point of this.#store.getRange(left, right + 1)) {
          if (this.#sqDist(point.x, point.y, qx, qy) <= r2) {
            result.push(point);
            if (result.length >= maxResults) return result;
          }
        }
        continue;
      }

      // otherwise find the middle index
      const m = (left + right) >> 1;

      // include the middle item if it's in range
      const pointM = this.#store.get(m);
      const { x, y } = pointM;
      if (this.#sqDist(x, y, qx, qy) <= r2) {
        result.push(pointM);
        if (result.length >= maxResults) return result;
      }

      // queue search in halves that intersect the query
      if (axis === 0 ? qx - r <= x : qy - r <= y) stack.push([left, m - 1, 1 - axis]);
      if (axis === 0 ? qx + r >= x : qy + r >= y) stack.push([m + 1, right, 1 - axis]);
    }

    return result;
  }

  /**
   * Search the index for items within a given radius using a spherical query.
   * NOTE: Assumes the input points are lon-lat pairs in degrees.
   * @param lon - longitude
   * @param lat - latitude
   * @param dist - max distance in meters
   * @param maxResults - max number of results
   * @param planetRadius - the radius of the planet (Earth by default)
   * @returns - the items that are in range
   */
  searchRadiusSphere(
    lon: number,
    lat: number,
    dist: number,
    maxResults = Infinity,
    planetRadius = EARTH_RADIUS,
  ): Array<VectorPoint<D | P>> {
    this.sort();

    const { nodeSize } = this;
    const result: Array<VectorPoint<D | P>> = []; // ids of items that are in range
    const maxHaverSinDist = haverSin(dist / planetRadius);
    const cosLat = Math.cos(lat * RAD);
    // a distance-sorted priority queue that will contain both points and kd-tree nodes
    const pq = new PriorityQueue<NodeQuery<D | P>>([], (a, b) => a.dist - b.dist);
    // an object that represents the top kd-tree node (the whole Earth)
    let node: NodeQuery<D | P> | undefined = {
      axis: 0, // 0 for longitude axis and 1 for latitude axis
      dist: 0, // will hold the lower bound of children's distances to the query point
    };

    while (node !== undefined) {
      const right = node.right ?? this.#store.length - 1;
      const left = node.left ?? 0;

      if (right - left <= nodeSize) {
        // leaf node case
        // add all points of the leaf node to the queue
        for (const point of this.#store.getRange(left, right + 1)) {
          const dist = haverSinDist(lon, lat, point.x, point.y, cosLat);
          pq.push({ point, dist });
        }
      } else {
        // not a leaf node (has child nodes)
        // middle index point
        const m = (left + right) >> 1;
        const midPoint = this.#store.get(m);

        // add middle point to the queue
        const dist = haverSinDist(lon, lat, midPoint.x, midPoint.y, cosLat);
        pq.push({ point: midPoint, dist });

        const nextAxis = (node.axis ?? 0 + 1) % 2;

        // first half of the node
        const leftNode = {
          left,
          right: m - 1,
          axis: nextAxis,
          minLon: node.minLon,
          minLat: node.minLat,
          maxLon: node.axis === 0 ? midPoint.x : node.maxLon,
          maxLat: node.axis === 1 ? midPoint.y : node.maxLat,
          dist: 0,
        };
        // second half of the node
        const rightNode = {
          left: m + 1,
          right,
          axis: nextAxis,
          minLon: node.axis === 0 ? midPoint.x : node.minLon,
          minLat: node.axis === 1 ? midPoint.y : node.minLat,
          maxLon: node.maxLon,
          maxLat: node.maxLat,
          dist: 0,
        };

        leftNode.dist = boxDist(lon, lat, cosLat, leftNode);
        rightNode.dist = boxDist(lon, lat, cosLat, rightNode);

        // add child nodes to the queue
        pq.push(leftNode);
        pq.push(rightNode);
      }

      // fetch closest points from the queue; they're guaranteed to be closer
      // than all remaining points (both individual and those in kd-tree nodes),
      // since each node's distance is a lower bound of distances to its children
      while (pq.length !== 0 && pq.peek()?.point !== undefined) {
        const candidate = pq.pop();
        if (candidate?.point === undefined) break;
        if (candidate.dist > maxHaverSinDist) return result;
        result.push(candidate.point);
        if (result.length === maxResults) return result;
      }

      // the next closest kd-tree node
      node = pq.pop();
    }

    return result;
  }

  /**
   * Compute the squared distance between two points
   * @param ax - the first x coordinate
   * @param ay - the first y coordinate
   * @param bx - the second x coordinate
   * @param by - the second y coordinate
   * @returns - the squared distance
   */
  #sqDist(ax: number, ay: number, bx: number, by: number): number {
    const dx = ax - bx;
    const dy = ay - by;
    return dx * dx + dy * dy;
  }
}

/**
 * lower bound for distance from a location to points inside a bounding box
 * @param lon - the longitude
 * @param lat - the latitude
 * @param cosLat - the cosine of the latitude
 * @param node - the query node to test against
 * @returns - the box distance
 */
function boxDist<T extends Properties = Properties>(
  lon: number,
  lat: number,
  cosLat: number,
  node: NodeQuery<T>,
): number {
  const minLon = node.minLon ?? -180;
  const maxLon = node.maxLon ?? 180;
  const minLat = node.minLat ?? -90;
  const maxLat = node.maxLat ?? 90;

  // query point is between minimum and maximum longitudes
  if (lon >= minLon && lon <= maxLon) {
    if (lat < minLat) return haverSin((lat - minLat) * RAD);
    if (lat > maxLat) return haverSin((lat - maxLat) * RAD);
    return 0;
  }

  // query point is west or east of the bounding box;
  // calculate the extremum for great circle distance from query point to the closest longitude;
  const haverSinDLon = Math.min(haverSin((lon - minLon) * RAD), haverSin((lon - maxLon) * RAD));
  const extremumLat = vertexLat(lat, haverSinDLon);

  // if extremum is inside the box, return the distance to it
  if (extremumLat > minLat && extremumLat < maxLat) {
    return haverSinDistPartial(haverSinDLon, cosLat, lat, extremumLat);
  }
  // otherwise return the distan e to one of the bbox corners (whichever is closest)
  return Math.min(
    haverSinDistPartial(haverSinDLon, cosLat, lat, minLat),
    haverSinDistPartial(haverSinDLon, cosLat, lat, maxLat),
  );
}

/**
 * Returns the square of the haversine of the angle
 * @param theta - the angle
 * @returns - the square of the haversine
 */
function haverSin(theta: number): number {
  const s = Math.sin(theta / 2);
  return s * s;
}

/**
 * Returns the haversine of the angle
 * @param haverSinDLon - the haversine of the longitude difference
 * @param cosLat1 - the cosine of the first latitude
 * @param lat1 - the first latitude
 * @param lat2 - the second latitude
 * @returns - the haversine of the angle
 */
function haverSinDistPartial(
  haverSinDLon: number,
  cosLat1: number,
  lat1: number,
  lat2: number,
): number {
  return cosLat1 * Math.cos(lat2 * RAD) * haverSinDLon + haverSin((lat1 - lat2) * RAD);
}

/**
 * Returns the square of the haversine of the distance
 * @param lon1 - the first longitude
 * @param lat1 - the first latitude
 * @param lon2 - the second longitude
 * @param lat2 - the second latitude
 * @param cosLat1 - the cosine of the first latitude
 * @returns - the square of the haversine
 */
function haverSinDist(
  lon1: number,
  lat1: number,
  lon2: number,
  lat2: number,
  cosLat1: number,
): number {
  const haverSinDLon = haverSin((lon1 - lon2) * RAD);
  return haverSinDistPartial(haverSinDLon, cosLat1, lat1, lat2);
}

/**
 * Returns the distance between two points given the spherical radius in meters (defaults to earth's radius)
 * @param lon1 - the first longitude
 * @param lat1 - the first latitude
 * @param lon2 - the second longitude
 * @param lat2 - the second latitude
 * @param planetRadius - the radius of the planet (Earth by default)
 * @returns - the distance
 */
export function sphericalDistance(
  lon1: number,
  lat1: number,
  lon2: number,
  lat2: number,
  planetRadius: number = EARTH_RADIUS,
): number {
  const h = haverSinDist(lon1, lat1, lon2, lat2, Math.cos(lat1 * RAD));
  return 2 * planetRadius * Math.asin(Math.sqrt(h));
}

/**
 * Returns the latitude of a vertex given the latitude and the haversine of the longitude difference
 * @param lat - the latitude
 * @param haverSinDLon - the haversine of the longitude difference
 * @returns - the latitude
 */
function vertexLat(lat: number, haverSinDLon: number): number {
  const cosDLon = 1 - 2 * haverSinDLon;
  if (cosDLon <= 0) return lat > 0 ? 90 : -90;
  return Math.atan(Math.tan(lat * RAD) / cosDLon) / RAD;
}
