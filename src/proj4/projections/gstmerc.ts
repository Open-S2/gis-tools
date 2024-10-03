import { ProjectionBase } from '.';
import { cosh, invlatiso, latiso, sinh } from '../common';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

const { pow, sin, cos, sqrt, atan, asin } = Math;

/**
 * # Gauss-Schreiber Transverse Mercator (aka Gauss-Laborde Reunion)
 *
 * **Classification**: Conformal
 *
 * **Available forms**: Forward and inverse, spherical projection
 *
 * **Defined area**: Global
 *
 * **Alias**: gstmerc
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=gstmerc
 * ```
 *
 * ## Optional Parameters
 * - `+k_0=<value>`: Scale factor at the central meridian.
 * - `+lon_0=<value>`: Longitude of the central meridian.
 * - `+lat_0=<value>`: Latitude of origin.
 * - `+ellps=<value>`: Ellipsoid name (e.g., GRS80, WGS84).
 * - `+R=<value>`: Radius of the sphere (used in spherical projections).
 * - `+x_0=<value>`: False easting.
 * - `+y_0=<value>`: False northing.
 *
 * ## Usage Example
 * ```
 * echo 12 55 | proj +proj=gstmerc +ellps=WGS84
 * echo 12 55 | proj +proj=gstmerc +k_0=1 +lon_0=0 +x_0=500000 +y_0=0
 * ```
 *
 * ![Gauss-Schreiber Transverse Mercator](./images/gstmerc.png)
 */
export class GaussSchreiberTransverseMercator
  extends ProjectionBase
  implements ProjectionTransform
{
  name = 'GaussSchreiberTransverseMercator';
  static names = [
    'GaussSchreiberTransverseMercator',
    'Gauss_Schreiber_Transverse_Mercator',
    'gstmerg',
    'gstmerc',
  ];
  // GaussSchreiberTransverseMercator specific variables
  longc: number;
  cp: number;
  n1: number;
  n2: number;
  xs: number;
  ys: number;

  /**
   * TODO: This whole file is arguably wrong. https://github.com/OSGeo/PROJ/blob/e2174f8292a39cdd2f92ce8122601e9b264555c2/src/projections/gstmerc.cpp#L15
   * Preps an GaussSchreiberTransverseMercator projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);

    const temp = this.b / this.a;
    this.e = sqrt(1 - temp * temp);
    this.longc = this.long0;
    this.n1 = sqrt(1 + (this.es * pow(cos(this.lat0), 4)) / (1 - this.es));
    const sinz = sin(this.lat0);
    const pc = asin(sinz / this.n1);
    const sinzpc = sin(pc);
    this.cp = latiso(0, pc, sinzpc) - this.n1 * latiso(this.e, this.lat0, sinz);
    this.n2 = (this.k0 * this.a * sqrt(1 - this.es)) / (1 - this.es * sinz * sinz);
    this.xs = this.x0;
    this.ys = this.y0 - this.n2 * pc;

    // Q->lamc = P->lam0;
    // Q->n1 = sqrt(1 + P->es * pow(cos(P->phi0), 4.0) / (1 - P->es));
    // Q->phic = asin(sin(P->phi0) / Q->n1);
    // Q->c = log(pj_tsfn(-Q->phic, -sin(P->phi0) / Q->n1, 0.0)) -
    //        Q->n1 * log(pj_tsfn(-P->phi0, -sin(P->phi0), P->e));
    // Q->n2 = P->k0 * P->a * sqrt(1 - P->es) /
    //         (1 - P->es * sin(P->phi0) * sin(P->phi0));
    // Q->XS = 0;
    // Q->YS = -Q->n2 * Q->phic;
  }

  /**
   * GaussSchreiberTransverseMercator forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const { x: lon, y: lat } = p;
    const L = this.n1 * (lon - this.longc);
    const Ls = this.cp + this.n1 * latiso(this.e, lat, sin(lat));
    const lat1 = asin(sin(L) / cosh(Ls));
    const Ls1 = latiso(0, lat1, sin(lat1));
    p.x = this.xs + this.n2 * Ls1;
    p.y = this.ys + this.n2 * atan(sinh(Ls) / cos(L));
  }

  /**
   * GaussSchreiberTransverseMercator inverse equations--mapping x-y to lon-lat
   * @param p - GaussSchreiberTransverseMercator point
   */
  inverse(p: VectorPoint): void {
    const { x, y } = p;
    const L = atan(sinh((x - this.xs) / this.n2) / cos((y - this.ys) / this.n2));
    const lat1 = asin(sin((y - this.ys) / this.n2) / cosh((x - this.xs) / this.n2));
    const LC = latiso(0, lat1, sin(lat1));
    p.x = this.longc + L / this.n1;
    p.y = invlatiso(this.e, (LC - this.cp) / this.n1);
  }
}
