import { ProjectionBase } from './base';
import { EPSLN, HALF_PI } from '../constants';
import { adjustLat, adjustLon, e0fn, e1fn, e2fn, e3fn, gN, imlfn, mlfn } from '../common';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

/**
 * # Cassini (Cassini-Soldner) Projection
 *
 * Although the Cassini projection has been largely replaced by the Transverse Mercator, it is
 * still in limited use outside the United States and was one of the major topographic mapping
 * projections until the early 20th century.
 *
 * **Classification**: Transverse and oblique cylindrical
 *
 * **Available forms**: Forward and Inverse, Spherical and ellipsoidal
 *
 * **Defined area**: Global, but best used near the central meridian with long, narrow areas
 *
 * **Alias**: `cass`
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=cass
 * ```
 *
 * ## Required Parameters
 * - `lat_0`
 *
 * ## Optional Parameters
 * - `lon_0`
 * - `x_0`
 * - `y_0`
 * - `ellps`
 * - `R`
 * - `+hyperbolic`: Use modified form of the standard Cassini-Soldner projection known as the Hyperbolic Cassini-Soldner (used in EPSG:3139).
 *
 * ![Cassini (Cassini-Soldner)](https://github.com/Open-S2/s2-tools/blob/master/assets/proj4/projections/images/cass.png?raw=true)
 */
export class CassiniSoldner extends ProjectionBase implements ProjectionTransform {
  name = 'Cassini_Soldner';
  static names = ['Cassini_Soldner', 'Cassini', 'cass'];
  e0: number = 0;
  e1: number = 0;
  e2: number = 0;
  e3: number = 0;
  ml0: number = 0;

  /**
   * Preps an Cassini Soldner projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);
    if (!this.sphere) {
      this.e0 = e0fn(this.es);
      this.e1 = e1fn(this.es);
      this.e2 = e2fn(this.es);
      this.e3 = e3fn(this.es);
      this.ml0 = this.a * mlfn(this.e0, this.e1, this.e2, this.e3, this.lat0);
    }
  }

  /**
   * Cassini Soldner forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const { sin, cos, asin, atan2, tan } = Math;
    let x, y;
    let lam = p.x;
    const phi = p.y;
    lam = adjustLon(lam - this.long0);

    if (this.sphere) {
      x = this.a * asin(cos(phi) * sin(lam));
      y = this.a * (atan2(tan(phi), cos(lam)) - this.lat0);
    } else {
      //ellipsoid
      const sinphi = sin(phi);
      const cosphi = cos(phi);
      const nl = gN(this.a, this.e, sinphi);
      const tl = tan(phi) * tan(phi);
      const al = lam * cos(phi);
      const asq = al * al;
      const cl = (this.es * cosphi * cosphi) / (1 - this.es);
      const ml = this.a * mlfn(this.e0, this.e1, this.e2, this.e3, phi);

      x = nl * al * (1 - asq * tl * (1 / 6 - ((8 - tl + 8 * cl) * asq) / 120));
      y = ml - this.ml0 + ((nl * sinphi) / cosphi) * asq * (0.5 + ((5 - tl + 6 * cl) * asq) / 24);
    }

    p.x = x + this.x0;
    p.y = y + this.y0;
  }

  /**
   * Cassini Soldner inverse equations--mapping x-y to lon-lat
   * @param p - A Cassini Soldner point
   */
  inverse(p: VectorPoint): void {
    const { abs, sin, cos, asin, atan2, tan, pow } = Math;
    p.x -= this.x0;
    p.y -= this.y0;
    const x = p.x / this.a;
    const y = p.y / this.a;
    let phi, lam;

    if (this.sphere) {
      const dd = y + this.lat0;
      phi = asin(sin(dd) * cos(x));
      lam = atan2(tan(x), cos(dd));
    } else {
      /* ellipsoid */
      const ml1 = this.ml0 / this.a + y;
      const phi1 = imlfn(ml1, this.e0, this.e1, this.e2, this.e3);
      if (abs(abs(phi1) - HALF_PI) <= EPSLN) {
        p.x = this.long0;
        p.y = HALF_PI;
        if (y < 0) {
          p.y *= -1;
        }
        return;
      }
      const nl1 = gN(this.a, this.e, sin(phi1));

      const rl1 = ((nl1 * nl1 * nl1) / this.a / this.a) * (1 - this.es);
      const tl1 = pow(tan(phi1), 2);
      const dl = (x * this.a) / nl1;
      const dsq = dl * dl;
      phi = phi1 - ((nl1 * tan(phi1)) / rl1) * dl * dl * (0.5 - ((1 + 3 * tl1) * dl * dl) / 24);
      lam = (dl * (1 - dsq * (tl1 / 3 + ((1 + 3 * tl1) * tl1 * dsq) / 15))) / cos(phi1);
    }

    p.x = adjustLon(lam + this.long0);
    p.y = adjustLat(phi);
  }
}
