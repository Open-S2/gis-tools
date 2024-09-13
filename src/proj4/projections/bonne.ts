import { HALF_PI } from '../constants';
import { WGS84Projection } from '.';
import { adjustLat, adjustLon, hypot, pjEnfn, pjInvMlfn, pjMlfn } from '../common';

import type { En } from '../common';
import type { ProjectionTransformDefinition } from '.';
import type { VectorPoint } from 's2-tools/geometry';

const EPS10 = 1e-10;
const { abs, sin, cos, sqrt, atan2, tan } = Math;

/**
 * # Bonne (Werner lat_1=90) Projection
 *
 * **Classification**: Miscellaneous
 *
 * **Available forms**: Forward and inverse, spherical and ellipsoidal
 *
 * **Defined area**: Global
 *
 * **Alias**: `bonne`
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=bonne +lat_1=10
 * ```
 *
 * ## Required Parameters
 * - `lat1`: Latitude of first standard parallel
 *
 * ## Optional Parameters
 * - `lon0`: Longitude of origin
 * - `ellps`: Ellipsoid name
 * - `R`: Radius of sphere
 * - `x0`: False easting
 * - `y0`: False northing
 *
 * ![Bonne (Werner lat_1=90) Projection](https://github.com/OSGeo/PROJ/blob/38dd7c2446f3500a43f0257f5a4833d6aa5aab0b/docs/source/operations/projections/images/bonne.png?raw=true)
 */
export class BonneWernerProjection
  extends WGS84Projection
  implements ProjectionTransformDefinition
{
  name = 'Bonne (Werner lat_1=90)';
  names = [this.name, 'bonne'];
  // BonneWernerProjection specific variables
  phi1 = 0;
  en: En = [0, 0, 0, 0, 0];
  m1: number = 0;
  am1: number = 0;
  cphi1: number = 0;

  /** Preps an Albers Conic Equal Area projection */
  constructor() {
    super();
    let c;

    this.phi1 = this.lat1;
    if (abs(this.phi1) < EPS10) {
      throw new Error('Invalid latitude');
    }
    if (this.es) {
      this.en = pjEnfn(this.es);
      this.m1 = pjMlfn(this.phi1, (this.am1 = sin(this.phi1)), (c = cos(this.phi1)), this.en);
      this.am1 = c / (sqrt(1 - this.es * this.am1 * this.am1) * this.am1);
      this.inverse = this.eInv;
      this.forward = this.eFwd;
    } else {
      if (abs(this.phi1) + EPS10 >= HALF_PI) {
        this.cphi1 = 0;
      } else {
        this.cphi1 = 1 / tan(this.phi1);
      }
      this.inverse = this.sInv;
      this.forward = this.sFwd;
    }
  }

  /**
   * Bonne Werner Easting forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   * @returns - an Bonne Werner point
   */
  eFwd(p: VectorPoint): VectorPoint {
    const lam = adjustLon(p.x - (this.long0 || 0));
    const phi = p.y;
    let E, c;
    const rh = this.am1 + this.m1 - pjMlfn(phi, (E = sin(phi)), (c = cos(phi)), this.en);
    E = (c * lam) / (rh * sqrt(1 - this.es * E * E));
    p.x = rh * sin(E);
    p.y = this.am1 - rh * cos(E);

    p.x = this.a * p.x + (this.x0 || 0);
    p.y = this.a * p.y + (this.y0 || 0);
    return p;
  }

  /**
   * Bonne Werner Easting inverse equations--mapping x-y to lon-lat
   * @param p - Bonne Werner point
   * @returns - lon-lat WGS84 point
   */
  eInv(p: VectorPoint): VectorPoint {
    p.x = (p.x - (this.x0 || 0)) / this.a;
    p.y = (p.y - (this.y0 || 0)) / this.a;

    let s: number, lam: number;
    const rh = hypot(p.x, (p.y = this.am1 - p.y));
    const phi = pjInvMlfn(this.am1 + this.m1 - rh, this.es, this.en);
    if ((s = abs(phi)) < HALF_PI) {
      s = sin(phi);
      lam = (rh * atan2(p.x, p.y) * sqrt(1 - this.es * s * s)) / cos(phi);
    } else if (abs(s - HALF_PI) <= EPS10) {
      lam = 0;
    } else {
      throw new Error();
    }
    p.x = adjustLon(lam + (this.long0 || 0));
    p.y = adjustLat(phi);
    return p;
  }

  /**
   * Bonne Werner Southing forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   * @returns - an Bonne Werner point
   */
  sFwd(p: VectorPoint): VectorPoint {
    const lam = adjustLon(p.x - (this.long0 || 0));
    const phi = p.y;
    let E: number;
    const rh = this.cphi1 + this.phi1 - phi;
    if (abs(rh) > EPS10) {
      p.x = rh * sin((E = (lam * cos(phi)) / rh));
      p.y = this.cphi1 - rh * cos(E);
    } else {
      p.x = p.y = 0;
    }

    p.x = this.a * p.x + (this.x0 || 0);
    p.y = this.a * p.y + (this.y0 || 0);
    return p;
  }

  /**
   * Bonne Werner Southing inverse equations--mapping x-y to lon-lat
   * @param p - Bonne Werner point
   * @returns - lon-lat WGS84 point
   */
  sInv(p: VectorPoint): VectorPoint {
    p.x = (p.x - (this.x0 || 0)) / this.a;
    p.y = (p.y - (this.y0 || 0)) / this.a;

    let lam: number;
    const rh = hypot(p.x, (p.y = this.cphi1 - p.y));
    const phi = this.cphi1 + this.phi1 - rh;
    if (abs(phi) > HALF_PI) {
      throw new Error();
    }
    if (abs(abs(phi) - HALF_PI) <= EPS10) {
      lam = 0;
    } else {
      lam = (rh * atan2(p.x, p.y)) / cos(phi);
    }
    p.x = adjustLon(lam + (this.long0 || 0));
    p.y = adjustLat(phi);
    return p;
  }
}
