import { ProjectionBase } from '.';
import { EPSLN, HALF_PI } from '../constants';
import { adjustLon, asinz } from '../common';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

const { abs, sin, cos, sqrt, atan2 } = Math;

/**
 * # Orthographic
 *
 * The orthographic projection is a perspective azimuthal projection centered
 * around a given latitude and longitude.
 *
 * **Classification**: Azimuthal
 *
 * **Available forms**: Forward and inverse, spherical and ellipsoidal
 *
 * **Defined area**: Global, although only one hemisphere can be seen at a time
 *
 * **Alias**: ortho
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=ortho
 * ```
 *
 * ## Required Parameters
 * - None
 *
 * ## Optional Parameters
 * - `+alpha=<value>`: Azimuth clockwise from north at the center of projection. *Defaults to 0.0.* (added in PROJ 9.5.0)
 * - `+k_0=<value>`: Scale factor. Determines scale factor used in the projection. *Defaults to 1.0.* (added in PROJ 9.5.0)
 * - `+lon_0=<value>`
 * - `+lat_0=<value>`
 * - `+ellps=<value>`
 * - `+R=<value>`
 * - `+x_0=<value>`
 * - `+y_0=<value>`
 *
 * ## Notes
 * - Before PROJ 7.2, only the spherical formulation was implemented. To replicate PROJ < 7.2 results with newer versions, force the ellipsoid to a sphere using `+f=0`.
 * - This projection method corresponds to `EPSG:9840` (or `EPSG:1130` with `k_0` or `alpha`).
 *
 * ![Orthographic](./images/ortho.png)
 */
export class Orthographic extends ProjectionBase implements ProjectionTransform {
  name = 'Orthographic';
  static names = ['Orthographic', 'ortho'];
  // Orthographic specific variables
  sinP14: number;
  cosP14: number;

  /**
   * Preps an Orthographic projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);

    this.sinP14 = sin(this.lat0);
    this.cosP14 = cos(this.lat0);
  }

  /**
   * Orthographic forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const { x: lon, y: lat } = p;
    let x, y;
    const dlon = adjustLon(lon - this.long0); /* delta longitude value      */
    const sinphi = sin(lat); /* sin value        */
    const cosphi = cos(lat); /* cos value        */
    const coslon = cos(dlon); /* cos of longitude        */
    const g = this.sinP14 * sinphi + this.cosP14 * cosphi * coslon;
    const ksp = 1; /* scale factor          */
    if (g > 0 || abs(g) <= EPSLN) {
      x = this.a * ksp * cosphi * sin(dlon);
      y = this.y0 + this.a * ksp * (this.cosP14 * sinphi - this.sinP14 * cosphi * coslon);
    } else {
      throw new Error('latitude out of range');
    }
    p.x = x;
    p.y = y;
  }

  /**
   * Orthographic inverse equations--mapping x-y to lon-lat
   * @param p - Orthographic point
   */
  inverse(p: VectorPoint): void {
    let lon, lat;
    p.x -= this.x0;
    p.y -= this.y0;
    const rh = sqrt(p.x * p.x + p.y * p.y); /* height above ellipsoid      */
    const z = asinz(rh / this.a); /* angle          */
    const sinz = sin(z); /* sin of z      */
    const cosz = cos(z); /*  cos of z      */
    lon = this.long0;
    if (abs(rh) <= EPSLN) {
      lat = this.lat0;
      p.x = lon;
      p.y = lat;
      return;
    }
    lat = asinz(cosz * this.sinP14 + (p.y * sinz * this.cosP14) / rh);
    const con = abs(this.lat0) - HALF_PI;
    if (abs(con) <= EPSLN) {
      if (this.lat0 >= 0) {
        lon = adjustLon(this.long0 + atan2(p.x, -p.y));
      } else {
        lon = adjustLon(this.long0 - atan2(-p.x, p.y));
      }
      p.x = lon;
      p.y = lat;
      return;
    }
    lon = adjustLon(
      this.long0 + atan2(p.x * sinz, rh * this.cosP14 * cosz - p.y * this.sinP14 * sinz),
    );
    p.x = lon;
    p.y = lat;
  }
}
