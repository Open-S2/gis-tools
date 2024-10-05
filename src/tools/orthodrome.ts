import { degToRad, radToDeg } from '../geometry';

import type { VectorPoint } from '../geometry';

/**
 * Represents an orthodrome, which is the shortest path between two points on a sphere.
 * [Learn more here](http://www.movable-type.co.uk/scripts/latlong.html)
 */
export class Orthodrome {
  /** start longitude */
  readonly lon1: number;
  /** start latitude */
  readonly lat1: number;
  /** end longitude */
  readonly lon2: number;
  /** end latitude */
  readonly lat2: number;
  /** distance property */
  readonly a: number;
  /** distance property */
  readonly dist: number;
  /**
   * @param startLon - start longitude
   * @param startLat - start latitude
   * @param endLon - end longitude
   * @param endLat - end latitude
   */
  constructor(startLon: number, startLat: number, endLon: number, endLat: number) {
    const { sin, cos, atan2, sqrt } = Math;
    const lon1 = (this.lon1 = degToRad(startLon));
    const lat1 = (this.lat1 = degToRad(startLat));
    const lon2 = (this.lon2 = degToRad(endLon));
    const lat2 = (this.lat2 = degToRad(endLat));
    const dLat = lat2 - lat1;
    const dLon = lon2 - lon1;
    const a = (this.a =
      sin(dLat / 2) * sin(dLat / 2) + cos(lat1) * cos(lat2) * sin(dLon / 2) * sin(dLon / 2));
    this.dist = 2 * atan2(sqrt(a), sqrt(1 - a));
  }

  /**
   * input t 0->1. Find a point along the orthodrome.
   * @param t - distance along the orthodrome to find
   * @returns [lon, lat]
   */
  intermediatePoint(t: number): VectorPoint {
    const { lon1, lon2, lat1, lat2, dist } = this;
    const { sin, cos, atan2, sqrt } = Math;

    // check corner cases first
    if (t === 0) return { x: radToDeg(lon1), y: radToDeg(lat1) };
    else if (t === 1) return { x: radToDeg(lon2), y: radToDeg(lat2) };
    // check if points are equal
    else if (lon1 === lon2 && lat1 === lat2) return { x: radToDeg(lon1), y: radToDeg(lat1) };

    const A = sin((1 - t) * dist) / sin(dist);
    const B = sin(t * dist) / sin(dist);

    const x = A * cos(lat1) * cos(lon1) + B * cos(lat2) * cos(lon2);
    const y = A * cos(lat1) * sin(lon1) + B * cos(lat2) * sin(lon2);
    const z = A * sin(lat1) + B * sin(lat2);

    const lat = atan2(z, sqrt(x * x + y * y));
    const lon = atan2(y, x);

    return { x: radToDeg(lon), y: radToDeg(lat) };
  }

  /**
   * projected normalized (0->1)
   * @returns - total distance between the two points
   */
  distanceTo(): number {
    const { a } = this;
    const { atan2, sqrt } = Math;

    return 2 * atan2(sqrt(a), sqrt(1 - a));
  }
}
