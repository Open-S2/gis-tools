import { ProjectionBase } from '.';
import { adjustLon } from '../common';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

const { abs, pow, sin, cos, sqrt, atan2, asin, tan, atan } = Math;

/**
 * # Krovak
 *
 * **Classification**: Conformal Conical
 *
 * **Available forms**: Forward and inverse, spherical and ellipsoidal
 *
 * **Defined area**: Global, but more accurate around Czech Republic and Slovakia
 *
 * **Alias**: krovak
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=krovak
 * ```
 *
 * By default, coordinates in the forward direction are output in easting, northing,
 * and negative in the Czech Republic and Slovakia, with absolute value of
 * easting/westing being smaller than absolute value of northing/southing.
 *
 * See also `mod_krovak` for a variation of Krovak used with the S-JTSK/05 datum
 * in the Czech Republic.
 *
 * ## Required Parameters
 * - None, all parameters are optional for this projection.
 *
 * ## Optional Parameters
 * - `+czech`: Reverses the sign of the output coordinates for use in the Czech Republic and Slovakia (positive values become westing and southing).
 * - `+lon_0`: Longitude of projection center. Defaults to `24°50'` (24.8333).
 * - `+lat_0`: Latitude of projection center. Defaults to `49.5`.
 * - `+k_0`: Scale factor. Defaults to `0.9999`.
 * - `+x_0`: False easting. Defaults to `0`.
 * - `+y_0`: False northing. Defaults to `0`.
 *
 * ## Notes
 * - The latitude of the pseudo standard parallel is hardcoded to `78.5°`.
 * - The ellipsoid used is Bessel by default.
 * - Before PROJ 9.4, using custom `x_0` or `y_0` without the `+czech` switch resulted in incorrect values.
 *
 * ![Krovak](./images/krovak.png)
 */
export class Krovak extends ProjectionBase implements ProjectionTransform {
  name = 'Krovak';
  static names = ['krovak'];
  // Krovak specific variables
  s45: number;
  s90: number;
  fi0: number;
  e2: number;
  e: number;
  alfa: number;
  uq: number;
  u0: number;
  g: number;
  a: number;
  es: number;
  k1: number;
  n0: number;
  n: number;
  s0: number;
  ro0: number;
  ad: number;
  k: number;
  czech?: number;

  /**
   * Preps an Krovak projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);

    this.a = 6377397.155;
    this.es = 0.006674372230614;
    this.e = sqrt(this.es);
    if (this.lat0 === 0) {
      this.lat0 = 0.863937979737193;
    }
    if (this.long0 === 0) {
      this.long0 = 0.7417649320975901 - 0.308341501185665;
    }
    /* if scale not set default to 0.9999 */
    if (this.k0 === undefined) {
      this.k0 = 0.9999;
    }
    this.s45 = 0.785398163397448; /* 45 */
    this.s90 = 2 * this.s45;
    this.fi0 = this.lat0;
    this.e2 = this.es;
    this.e = sqrt(this.e2);
    this.alfa = sqrt(1 + (this.e2 * pow(cos(this.fi0), 4)) / (1 - this.e2));
    this.uq = 1.04216856380474;
    this.u0 = asin(sin(this.fi0) / this.alfa);
    this.g = pow(
      (1 + this.e * sin(this.fi0)) / (1 - this.e * sin(this.fi0)),
      (this.alfa * this.e) / 2,
    );
    this.k = (tan(this.u0 / 2 + this.s45) / pow(tan(this.fi0 / 2 + this.s45), this.alfa)) * this.g;
    this.k1 = this.k0;
    this.n0 = (this.a * sqrt(1 - this.e2)) / (1 - this.e2 * pow(sin(this.fi0), 2));
    this.s0 = 1.37008346281555;
    this.n = sin(this.s0);
    this.ro0 = (this.k1 * this.n0) / tan(this.s0);
    this.ad = this.s90 - this.uq;
  }

  /**
   * Krovak forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const { x: lon, y: lat } = p;
    const delta_lon = adjustLon(lon - this.long0);
    /* Transformation */
    const gfi = pow((1 + this.e * sin(lat)) / (1 - this.e * sin(lat)), (this.alfa * this.e) / 2);
    const u = 2 * (atan((this.k * pow(tan(lat / 2 + this.s45), this.alfa)) / gfi) - this.s45);
    const deltav = -delta_lon * this.alfa;
    const s = asin(cos(this.ad) * sin(u) + sin(this.ad) * cos(u) * cos(deltav));
    const d = asin((cos(u) * sin(deltav)) / cos(s));
    const eps = this.n * d;
    const ro =
      (this.ro0 * pow(tan(this.s0 / 2 + this.s45), this.n)) / pow(tan(s / 2 + this.s45), this.n);
    p.y = (ro * cos(eps)) / 1;
    p.x = (ro * sin(eps)) / 1;
    if (this.czech === undefined) {
      p.y *= -1;
      p.x *= -1;
    }
  }

  /**
   * Krovak inverse equations--mapping x-y to lon-lat
   * @param p - Krovak point
   */
  inverse(p: VectorPoint): void {
    let ok;
    /* Transformation */
    /* revert y, x*/
    const tmp = p.x;
    p.x = p.y;
    p.y = tmp;
    if (this.czech === undefined) {
      p.y *= -1;
      p.x *= -1;
    }
    const ro = sqrt(p.x * p.x + p.y * p.y);
    const eps = atan2(p.y, p.x);
    const d = eps / sin(this.s0);
    const s = 2 * (atan(pow(this.ro0 / ro, 1 / this.n) * tan(this.s0 / 2 + this.s45)) - this.s45);
    const u = asin(cos(this.ad) * sin(s) - sin(this.ad) * cos(s) * cos(d));
    const deltav = asin((cos(s) * sin(d)) / cos(u));
    p.x = this.long0 - deltav / this.alfa;
    let fi1 = u;
    ok = 0;
    let iter = 0;
    do {
      p.y =
        2 *
        (atan(
          pow(this.k, -1 / this.alfa) *
            pow(tan(u / 2 + this.s45), 1 / this.alfa) *
            pow((1 + this.e * sin(fi1)) / (1 - this.e * sin(fi1)), this.e / 2),
        ) -
          this.s45);
      if (abs(fi1 - p.y) < 0.0000000001) {
        ok = 1;
      }
      fi1 = p.y;
      iter += 1;
    } while (ok === 0 && iter < 15);
    if (iter >= 15) {
      throw new Error('Failed to converge');
    }
  }
}
