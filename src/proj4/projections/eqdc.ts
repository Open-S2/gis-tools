import { EPSLN } from '../constants';
import { ProjectionBase } from '.';
import { adjustLat, adjustLon, e0fn, e1fn, e2fn, e3fn, imlfn, mlfn, msfnz } from '../common';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

/**
 * # Equidistant Conic
 *
 * **Classification**: Conic
 *
 * **Available forms**: Forward and inverse, ellipsoidal
 *
 * **Defined area**: Global
 *
 * **Alias**: eqdc
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=eqdc +lat_1=55 +lat_2=60
 * ```
 *
 * ## Parameters
 *
 * ### Required
 * - `+lat_1` (Latitude of the first standard parallel)
 * - `+lat_2` (Latitude of the second standard parallel)
 *
 * ### Optional
 * - `+lon_0` (Central meridian)
 * - `+ellps` (Ellipsoid name)
 * - `+R` (Radius of the sphere)
 * - `+x_0` (False easting)
 * - `+y_0` (False northing)
 *
 * ![Equidistant Conic](https://github.com/Open-S2/s2-tools/blob/master/assets/proj4/projections/images/eqdc.png?raw=true)
 */
export class EquidistantConic extends ProjectionBase implements ProjectionTransform {
  name = 'Equidistant_Conic';
  static names = ['Equidistant_Conic', 'eqdc'];
  // EquidistantConic specific variables
  rh: number;
  temp: number;
  g: number;
  e0: number;
  e1: number;
  e2: number;
  e3: number;
  sinphi: number;
  cosphi: number;
  ms1: number;
  ms2 = 0;
  ml0: number;
  ml1: number;
  ml2 = 0;
  ns: number;

  /**
   * Preps an EquidistantConic projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    const { abs, sin, cos } = Math;
    super(params);

    if (abs(this.lat1 + this.lat2) < EPSLN)
      throw new Error('Standard parallels cannot be equal and on opposite sides of the equator');
    this.lat2 = this.lat2 ?? this.lat1;
    this.temp = this.b / this.a;
    this.es = 1 - Math.pow(this.temp, 2);
    this.e = Math.sqrt(this.es);
    this.e0 = e0fn(this.es);
    this.e1 = e1fn(this.es);
    this.e2 = e2fn(this.es);
    this.e3 = e3fn(this.es);

    this.sinphi = Math.sin(this.lat1);
    this.cosphi = Math.cos(this.lat1);

    this.ms1 = msfnz(this.e, this.sinphi, this.cosphi);
    this.ml1 = mlfn(this.e0, this.e1, this.e2, this.e3, this.lat1);

    if (abs(this.lat1 - this.lat2) < EPSLN) {
      this.ns = this.sinphi;
    } else {
      this.sinphi = sin(this.lat2);
      this.cosphi = cos(this.lat2);
      this.ms2 = msfnz(this.e, this.sinphi, this.cosphi);
      this.ml2 = mlfn(this.e0, this.e1, this.e2, this.e3, this.lat2);
      this.ns = (this.ms1 - this.ms2) / (this.ml2 - this.ml1);
    }
    this.g = this.ml1 + this.ms1 / this.ns;
    this.ml0 = mlfn(this.e0, this.e1, this.e2, this.e3, this.lat0);
    this.rh = this.a * (this.g - this.ml0);
  }

  /**
   * EquidistantConic forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const { sin, cos } = Math;
    const { x: lon, y: lat } = p;
    let rh1;
    if (this.sphere) {
      rh1 = this.a * (this.g - lat);
    } else {
      const ml = mlfn(this.e0, this.e1, this.e2, this.e3, lat);
      rh1 = this.a * (this.g - ml);
    }
    const theta = this.ns * adjustLon(lon - this.long0);
    const x = this.x0 + rh1 * sin(theta);
    const y = this.y0 + this.rh - rh1 * cos(theta);
    p.x = x;
    p.y = y;
  }

  /**
   * EquidistantConic inverse equations--mapping x-y to lon-lat
   * @param p - EquidistantConic point
   */
  inverse(p: VectorPoint): void {
    const { sqrt, atan2 } = Math;
    p.x -= this.x0;
    p.y = this.rh - p.y + this.y0;
    let con, rh1, lat, lon;
    if (this.ns >= 0) {
      rh1 = sqrt(p.x * p.x + p.y * p.y);
      con = 1;
    } else {
      rh1 = -sqrt(p.x * p.x + p.y * p.y);
      con = -1;
    }
    let theta = 0;
    if (rh1 !== 0) {
      theta = atan2(con * p.x, con * p.y);
    }
    if (this.sphere) {
      lon = adjustLon(this.long0 + theta / this.ns);
      lat = adjustLat(this.g - rh1 / this.a);
      p.x = lon;
      p.y = lat;
    } else {
      const ml = this.g - rh1 / this.a;
      lat = imlfn(ml, this.e0, this.e1, this.e2, this.e3);
      lon = adjustLon(this.long0 + theta / this.ns);
      p.x = lon;
      p.y = lat;
    }
  }
}
