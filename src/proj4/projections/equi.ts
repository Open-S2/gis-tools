import { ProjectionBase } from '.';
import { adjustLon } from '../common';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

/**
 * EquiRectangular Projection
 */
export class EquiRectangular extends ProjectionBase implements ProjectionTransform {
  name = 'EquiRectangular';
  static names = ['EquiRectangular', 'equi'];
  // EquiRectangular specific variables

  /**
   * Preps an EquiRectangular projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);

    this.x0 = this.x0 ?? 0;
    this.y0 = this.y0 ?? 0;
    this.lat0 = this.lat0 ?? 0;
    this.long0 = this.long0 ?? 0;
  }

  /**
   * EquiRectangular forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const { a, long0, lat0, x0, y0 } = this;
    const { x: lon, y: lat } = p;
    const dlon = adjustLon(lon - long0);
    const x = x0 + a * dlon * Math.cos(lat0);
    const y = y0 + a * lat;
    // this.t1 = x;
    // this.t2 = Math.cos(this.lat0);
    p.x = x;
    p.y = y;
  }

  /**
   * EquiRectangular inverse equations--mapping x-y to lon-lat
   * @param p - EquiRectangular point
   */
  inverse(p: VectorPoint): void {
    p.x -= this.x0;
    p.y -= this.y0;
    const lat = p.y / this.a;
    const lon = adjustLon(this.long0 + p.x / (this.a * Math.cos(this.lat0)));
    p.x = lon;
    p.y = lat;
  }
}
