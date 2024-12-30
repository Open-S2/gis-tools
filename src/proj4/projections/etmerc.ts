// Heavily based on this etmerc projection implementation
// https://github.com/mbloch/mapshaper-proj/blob/master/src/projections/etmerc.js

import { TransverseMercator } from './tmerc';
import { adjustLon, asinhy, clens, clensCmplx, gatg, hypot, sinh } from '../common';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

/**
 * # Extended Transverse Mercator
 *
 * **Classification**: Transverse and oblique cylindrical
 *
 * **Available forms**: Forward and inverse, spherical and ellipsoidal
 *
 * **Defined area**: Global, with full accuracy within 3900 km of the central meridian
 *
 * **Alias**: etmerc
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=etmerc
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
 * ![ExtendedTransverseMercator](https://github.com/Open-S2/s2-tools/blob/master/assets/proj4/projections/images/tmerc.png?raw=true)
 */
export class ExtendedTransverseMercator extends TransverseMercator implements ProjectionTransform {
  name = 'ExtendedTransverseMercator';
  static names = [
    'ExtendedTransverseMercator',
    'Extended_Transverse_Mercator',
    'Extended Transverse Mercator',
    'etmerc',
  ];
  // ExtendedTransverseMercator specific variables
  Qn: number;
  Zb: number;
  cgb: [number, number, number, number, number, number];
  cbg: [number, number, number, number, number, number];
  utg: [number, number, number, number, number, number];
  gtu: [number, number, number, number, number, number];

  /**
   * Preps an ExtendedTransverseMercator projection
   * @param params - projection specific parameters
   * @param precompute - optional precompute function (used by UTM)
   */
  constructor(
    params?: ProjectionParams,
    precompute?: (etmerc: ExtendedTransverseMercator) => void,
  ) {
    const { pow, sqrt } = Math;
    super(params);
    if (precompute !== undefined) precompute(this);

    if (!this.approx && (this.ep2 === Infinity || this.es <= 0)) {
      throw new Error(
        'Incorrect elliptical usage. Try using the +approx option in the proj string, or PROJECTION["Fast_Transverse_Mercator"] in the WKT.',
      );
    }
    if (this.approx) {
      // When '+approx' is set, use tmerc instead
      this.forward = super.forward;
      this.inverse = super.inverse;
    }

    this.x0 = this.x0 !== undefined ? this.x0 : 0;
    this.y0 = this.y0 !== undefined ? this.y0 : 0;
    this.long0 = this.long0 !== undefined ? this.long0 : 0;
    this.lat0 = this.lat0 !== undefined ? this.lat0 : 0;

    this.cgb = [0, 0, 0, 0, 0, 0];
    this.cbg = [0, 0, 0, 0, 0, 0];
    this.utg = [0, 0, 0, 0, 0, 0];
    this.gtu = [0, 0, 0, 0, 0, 0];

    const f = this.es / (1 + sqrt(1 - this.es));
    const n = f / (2 - f);
    let np = n;

    this.cgb[0] =
      n * (2 + n * (-2 / 3 + n * (-2 + n * (116 / 45 + n * (26 / 45 + n * (-2854 / 675))))));
    this.cbg[0] =
      n * (-2 + n * (2 / 3 + n * (4 / 3 + n * (-82 / 45 + n * (32 / 45 + n * (4642 / 4725))))));

    np = np * n;
    this.cgb[1] =
      np * (7 / 3 + n * (-8 / 5 + n * (-227 / 45 + n * (2704 / 315 + n * (2323 / 945)))));
    this.cbg[1] =
      np * (5 / 3 + n * (-16 / 15 + n * (-13 / 9 + n * (904 / 315 + n * (-1522 / 945)))));

    np = np * n;
    this.cgb[2] = np * (56 / 15 + n * (-136 / 35 + n * (-1262 / 105 + n * (73814 / 2835))));
    this.cbg[2] = np * (-26 / 15 + n * (34 / 21 + n * (8 / 5 + n * (-12686 / 2835))));

    np = np * n;
    this.cgb[3] = np * (4279 / 630 + n * (-332 / 35 + n * (-399572 / 14175)));
    this.cbg[3] = np * (1237 / 630 + n * (-12 / 5 + n * (-24832 / 14175)));

    np = np * n;
    this.cgb[4] = np * (4174 / 315 + n * (-144838 / 6237));
    this.cbg[4] = np * (-734 / 315 + n * (109598 / 31185));

    np = np * n;
    this.cgb[5] = np * (601676 / 22275);
    this.cbg[5] = np * (444337 / 155925);

    np = pow(n, 2);
    this.Qn = (this.k0 / (1 + n)) * (1 + np * (1 / 4 + np * (1 / 64 + np / 256)));

    this.utg[0] =
      n *
      (-0.5 +
        n * (2 / 3 + n * (-37 / 96 + n * (1 / 360 + n * (81 / 512 + n * (-96199 / 604800))))));
    this.gtu[0] =
      n *
      (0.5 + n * (-2 / 3 + n * (5 / 16 + n * (41 / 180 + n * (-127 / 288 + n * (7891 / 37800))))));

    this.utg[1] =
      np * (-1 / 48 + n * (-1 / 15 + n * (437 / 1440 + n * (-46 / 105 + n * (1118711 / 3870720)))));
    this.gtu[1] =
      np * (13 / 48 + n * (-3 / 5 + n * (557 / 1440 + n * (281 / 630 + n * (-1983433 / 1935360)))));

    np = np * n;
    this.utg[2] = np * (-17 / 480 + n * (37 / 840 + n * (209 / 4480 + n * (-5569 / 90720))));
    this.gtu[2] = np * (61 / 240 + n * (-103 / 140 + n * (15061 / 26880 + n * (167603 / 181440))));

    np = np * n;
    this.utg[3] = np * (-4397 / 161280 + n * (11 / 504 + n * (830251 / 7257600)));
    this.gtu[3] = np * (49561 / 161280 + n * (-179 / 168 + n * (6601661 / 7257600)));

    np = np * n;
    this.utg[4] = np * (-4583 / 161280 + n * (108847 / 3991680));
    this.gtu[4] = np * (34729 / 80640 + n * (-3418889 / 1995840));

    np = np * n;
    this.utg[5] = np * (-20648693 / 638668800);
    this.gtu[5] = np * (212378941 / 319334400);

    const Z = gatg(this.cbg, this.lat0);
    this.Zb = -this.Qn * (Z + clens(this.gtu, 2 * Z));
  }

  /**
   * ExtendedTransverseMercator forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const { abs, sin, cos, atan2, tan } = Math;
    let Ce = adjustLon(p.x - this.long0);
    let Cn = p.y;
    Cn = gatg(this.cbg, Cn);
    const sin_Cn = sin(Cn);
    const cos_Cn = cos(Cn);
    const sin_Ce = sin(Ce);
    const cos_Ce = cos(Ce);
    Cn = atan2(sin_Cn, cos_Ce * cos_Cn);
    Ce = atan2(sin_Ce * cos_Cn, hypot(sin_Cn, cos_Cn * cos_Ce));
    Ce = asinhy(tan(Ce));
    const tmp = clensCmplx(this.gtu, 2 * Cn, 2 * Ce);
    Cn = Cn + tmp[0];
    Ce = Ce + tmp[1];
    let x;
    let y;
    if (abs(Ce) <= 2.623395162778) {
      x = this.a * (this.Qn * Ce) + this.x0;
      y = this.a * (this.Qn * Cn + this.Zb) + this.y0;
    } else {
      x = Infinity;
      y = Infinity;
    }
    p.x = x;
    p.y = y;
  }

  /**
   * ExtendedTransverseMercator inverse equations--mapping x-y to lon-lat
   * @param p - ExtendedTransverseMercator point
   */
  inverse(p: VectorPoint): void {
    const { abs, sin, cos, atan2, atan } = Math;
    let Ce = (p.x - this.x0) * (1 / this.a);
    let Cn = (p.y - this.y0) * (1 / this.a);
    Cn = (Cn - this.Zb) / this.Qn;
    Ce = Ce / this.Qn;
    let lon;
    let lat;
    if (abs(Ce) <= 2.623395162778) {
      const tmp = clensCmplx(this.utg, 2 * Cn, 2 * Ce);
      Cn = Cn + tmp[0];
      Ce = Ce + tmp[1];
      Ce = atan(sinh(Ce));
      const sin_Cn = sin(Cn);
      const cos_Cn = cos(Cn);
      const sin_Ce = sin(Ce);
      const cos_Ce = cos(Ce);
      Cn = atan2(sin_Cn * cos_Ce, hypot(sin_Ce, cos_Ce * cos_Cn));
      Ce = atan2(sin_Ce, cos_Ce * cos_Cn);
      lon = adjustLon(Ce + this.long0);
      lat = gatg(this.cgb, Cn);
    } else {
      lon = Infinity;
      lat = Infinity;
    }
    p.x = lon;
    p.y = lat;
  }
}
