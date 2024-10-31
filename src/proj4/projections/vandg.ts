import { ProjectionBase } from '.';
import { EPSLN, HALF_PI } from '../constants';
import { adjustLon, asinz } from '../common';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

const { abs, PI, sin, cos, sqrt, tan, acos } = Math;

/**
 * # van der Grinten (I)
 *
 * **Classification**: Miscellaneous
 *
 * **Available forms**: Forward and inverse, spherical projection
 *
 * **Defined area**: Global
 *
 * **Alias**: vandg
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=vandg
 * ```
 *
 * ## Parameters
 *
 * All parameters are optional.
 *
 * - `+lon_0=<value>`: Central meridian.
 * - `+R=<value>`: Radius of the sphere.
 * - `+x_0=<value>`: False easting.
 * - `+y_0=<value>`: False northing.
 *
 * ![van der Grinten (I)](https://github.com/Open-S2/s2-tools/blob/master/assets/proj4/projections/images/vandg.png?raw=true)
 */
export class VanDerGrinten extends ProjectionBase implements ProjectionTransform {
  name = 'VanDerGrinten';
  static names = ['VanDerGrinten', 'Van_der_Grinten_I', 'vandg'];
  // VanDerGrinten specific variables
  R: number;

  /**
   * Preps an VanDerGrinten projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);

    // R = 6370997 -> Radius of earth
    this.R = this.a;
  }

  /**
   * VanDerGrinten forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const { x: lon, y: lat } = p;
    const dlon = adjustLon(lon - this.long0);
    let x, y;
    if (abs(lat) <= EPSLN) {
      x = this.x0 + this.R * dlon;
      y = this.y0;
    }
    const theta = asinz(2 * abs(lat / PI));
    if (abs(dlon) <= EPSLN || abs(abs(lat) - HALF_PI) <= EPSLN) {
      x = this.x0;
      if (lat >= 0) {
        y = this.y0 + PI * this.R * tan(0.5 * theta);
      } else {
        y = this.y0 + PI * this.R * -tan(0.5 * theta);
      }
      //  return(OK);
    }
    const al = 0.5 * abs(PI / dlon - dlon / PI);
    const asq = al * al;
    const sinth = sin(theta);
    const costh = cos(theta);
    const g = costh / (sinth + costh - 1);
    const gsq = g * g;
    const m = g * (2 / sinth - 1);
    const msq = m * m;
    let con =
      (PI *
        this.R *
        (al * (g - msq) + sqrt(asq * (g - msq) * (g - msq) - (msq + asq) * (gsq - msq)))) /
      (msq + asq);
    if (dlon < 0) {
      con = -con;
    }
    x = this.x0 + con;
    //con = abs(con / (PI * this.R));
    const q = asq + g;
    con = (PI * this.R * (m * q - al * sqrt((msq + asq) * (asq + 1) - q * q))) / (msq + asq);
    if (lat >= 0) {
      //y = this.y0 + PI * this.R * sqrt(1 - con * con - 2 * al * con);
      y = this.y0 + con;
    } else {
      //y = this.y0 - PI * this.R * sqrt(1 - con * con - 2 * al * con);
      y = this.y0 - con;
    }
    p.x = x;
    p.y = y;
  }

  /**
   * VanDerGrinten inverse equations--mapping x-y to lon-lat
   * @param p - VanDerGrinten point
   */
  inverse(p: VectorPoint): void {
    let lon, lat;
    p.x -= this.x0;
    p.y -= this.y0;
    let con = PI * this.R;
    const xx = p.x / con;
    const yy = p.y / con;
    const xys = xx * xx + yy * yy;
    const c1 = -abs(yy) * (1 + xys);
    const c2 = c1 - 2 * yy * yy + xx * xx;
    const c3 = -2 * c1 + 1 + 2 * yy * yy + xys * xys;
    const d = (yy * yy) / c3 + ((2 * c2 * c2 * c2) / c3 / c3 / c3 - (9 * c1 * c2) / c3 / c3) / 27;
    const a1 = (c1 - (c2 * c2) / 3 / c3) / c3;
    const m1 = 2 * sqrt(-a1 / 3);
    con = (3 * d) / a1 / m1;
    if (abs(con) > 1) {
      if (con >= 0) {
        con = 1;
      } else {
        con = -1;
      }
    }
    const th1 = acos(con) / 3;
    if (p.y >= 0) {
      lat = (-m1 * cos(th1 + PI / 3) - c2 / 3 / c3) * PI;
    } else {
      lat = -(-m1 * cos(th1 + PI / 3) - c2 / 3 / c3) * PI;
    }
    if (abs(xx) < EPSLN) {
      lon = this.long0;
    } else {
      lon = adjustLon(
        this.long0 + (PI * (xys - 1 + sqrt(1 + 2 * (xx * xx - yy * yy) + xys * xys))) / 2 / xx,
      );
    }
    p.x = lon;
    p.y = lat;
  }
}
