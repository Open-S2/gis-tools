// Heavily based on this tmerc projection implementation
// https://github.com/mbloch/mapshaper-proj/blob/master/src/projections/tmerc.js

import { ProjectionBase } from '.';
import { EPSLN, HALF_PI } from '../constants';
import { adjustLon, pjEnfn, pjInvMlfn, pjMlfn, sign } from '../common';

import type { En } from '../common';
import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

const { abs, pow, sin, cos, sqrt, atan2, asin, log, tan, acos, exp } = Math;

/**
 * # Transverse Mercator
 *
 * **Classification**: Transverse and oblique cylindrical
 *
 * **Available forms**: Forward and inverse, spherical and ellipsoidal
 *
 * **Defined area**: Global, with full accuracy within 3900 km of the central meridian
 *
 * **Alias**: tmerc
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=tmerc
 * ```
 *
 * ## Required Parameters
 * - `+lon_0`: Longitude of the central meridian.
 *
 * ## Optional Parameters
 * - `+approx`: Use the faster Evenden-Snyder algorithm, less accurate beyond 3°.
 * - `+algo`: Select algorithm from "auto", "evenden_snyder", or "poder_engsager".
 * - `+lat_0`: Latitude of origin.
 * - `+k_0`: Scale factor on the central meridian.
 * - `+x_0`: False easting.
 * - `+y_0`: False northing.
 *
 * ![Transverse Mercator](https://github.com/Open-S2/s2-tools/blob/master/assets/proj4/projections/images/tmerc.png?raw=true)
 */
export class TransverseMercator extends ProjectionBase implements ProjectionTransform {
  name = 'Transverse_Mercator';
  static names = ['Transverse_Mercator', 'tmerc', 'Transverse_Mercator', 'Transverse Mercator'];
  // TransverseMercator specific variables
  en: En = [0, 0, 0, 0, 0];
  ml0 = 0;

  /**
   * Preps an TransverseMercator projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);

    this.x0 = this.x0 !== undefined ? this.x0 : 0;
    this.y0 = this.y0 !== undefined ? this.y0 : 0;
    this.long0 = this.long0 !== undefined ? this.long0 : 0;
    this.lat0 = this.lat0 !== undefined ? this.lat0 : 0;

    if (this.es !== 0) {
      this.en = pjEnfn(this.es);
      this.ml0 = pjMlfn(this.lat0, sin(this.lat0), cos(this.lat0), this.en);
    }
  }

  /**
   * TransverseMercator forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const { x: lon, y: lat } = p;
    const delta_lon = adjustLon(lon - this.long0);
    let con;
    let x, y;
    const sin_phi = sin(lat);
    const cos_phi = cos(lat);
    if (this.ep2 === Infinity) {
      let b = cos_phi * sin(delta_lon);
      if (abs(abs(b) - 1) < EPSLN) {
        throw new Error(
          'Incorrect elliptical usage. Try using the +approx option in the proj string, or PROJECTION["Transverse_Mercator"] in the WKT.',
        );
      } else {
        x = 0.5 * this.a * this.k0 * log((1 + b) / (1 - b)) + this.x0;
        y = (cos_phi * cos(delta_lon)) / sqrt(1 - pow(b, 2));
        b = abs(y);
        if (b >= 1) {
          if (b - 1 > EPSLN) {
            throw new Error(
              'Incorrect elliptical usage. Try using the +approx option in the proj string, or PROJECTION["Transverse_Mercator"] in the WKT.',
            );
          } else {
            y = 0;
          }
        } else {
          y = acos(y);
        }
        if (lat < 0) {
          y = -y;
        }
        y = this.a * this.k0 * (y - this.lat0) + this.y0;
      }
    } else {
      let al = cos_phi * delta_lon;
      const als = pow(al, 2);
      const c = this.ep2 * pow(cos_phi, 2);
      const cs = pow(c, 2);
      const tq = abs(cos_phi) > EPSLN ? tan(lat) : 0;
      const t = pow(tq, 2);
      const ts = pow(t, 2);
      con = 1 - this.es * pow(sin_phi, 2);
      al = al / sqrt(con);
      const ml = pjMlfn(lat, sin_phi, cos_phi, this.en);
      x =
        this.a *
          (this.k0 *
            al *
            (1 +
              (als / 6) *
                (1 -
                  t +
                  c +
                  (als / 20) *
                    (5 -
                      18 * t +
                      ts +
                      14 * c -
                      58 * t * c +
                      (als / 42) * (61 + 179 * ts - ts * t - 479 * t))))) +
        this.x0;
      y =
        this.a *
          (this.k0 *
            (ml -
              this.ml0 +
              ((sin_phi * delta_lon * al) / 2) *
                (1 +
                  (als / 12) *
                    (5 -
                      t +
                      9 * c +
                      4 * cs +
                      (als / 30) *
                        (61 +
                          ts -
                          58 * t +
                          270 * c -
                          330 * t * c +
                          (als / 56) * (1385 + 543 * ts - ts * t - 3111 * t)))))) +
        this.y0;
    }
    p.x = x;
    p.y = y;
  }

  /**
   * TransverseMercator inverse equations--mapping x-y to lon-lat
   * @param p - TransverseMercator point
   */
  inverse(p: VectorPoint): void {
    let con, phi;
    let lat, lon;
    const x = (p.x - this.x0) * (1 / this.a);
    const y = (p.y - this.y0) * (1 / this.a);
    if (this.ep2 === Infinity) {
      const f = exp(x / this.k0);
      const g = 0.5 * (f - 1 / f);
      const temp = this.lat0 + y / this.k0;
      const h = cos(temp);
      con = sqrt((1 - pow(h, 2)) / (1 + pow(g, 2)));
      lat = asin(con);
      if (y < 0) {
        lat = -lat;
      }
      if (g === 0 && h === 0) {
        lon = 0;
      } else {
        lon = adjustLon(atan2(g, h) + this.long0);
      }
    } else {
      // ellipsoidal form
      con = this.ml0 + y / this.k0;
      phi = pjInvMlfn(con, this.es, this.en);
      if (abs(phi) < HALF_PI) {
        const sin_phi = sin(phi);
        const cos_phi = cos(phi);
        const tan_phi = abs(cos_phi) > EPSLN ? tan(phi) : 0;
        const c = this.ep2 * pow(cos_phi, 2);
        const cs = pow(c, 2);
        const t = pow(tan_phi, 2);
        const ts = pow(t, 2);
        con = 1 - this.es * pow(sin_phi, 2);
        const d = (x * sqrt(con)) / this.k0;
        const ds = pow(d, 2);
        con = con * tan_phi;
        lat =
          phi -
          ((con * ds) / (1 - this.es)) *
            0.5 *
            (1 -
              (ds / 12) *
                (5 +
                  3 * t -
                  9 * c * t +
                  c -
                  4 * cs -
                  (ds / 30) *
                    (61 +
                      90 * t -
                      252 * c * t +
                      45 * ts +
                      46 * c -
                      (ds / 56) * (1385 + 3633 * t + 4095 * ts + 1574 * ts * t))));
        lon = adjustLon(
          this.long0 +
            (d *
              (1 -
                (ds / 6) *
                  (1 +
                    2 * t +
                    c -
                    (ds / 20) *
                      (5 +
                        28 * t +
                        24 * ts +
                        8 * c * t +
                        6 * c -
                        (ds / 42) * (61 + 662 * t + 1320 * ts + 720 * ts * t))))) /
              cos_phi,
        );
      } else {
        lat = HALF_PI * sign(y);
        lon = 0;
      }
    }
    p.x = lon;
    p.y = lat;
  }
}
