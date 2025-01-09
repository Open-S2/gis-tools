import { EPSLN } from '../constants';
import { ProjectionBase } from '.';
import { adjustLon, asinz } from '../common';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

/**
 * # Gnomonic (gnom)
 *
 * For a sphere, the gnomonic projection is a projection from the center of
 * the sphere onto a plane tangent to the center point of the projection.
 * This projects great circles to straight lines.  For an ellipsoid, it is
 * the limit of a doubly azimuthal projection, a projection where the
 * azimuths from 2 points are preserved, as the two points merge into the
 * center point.  In this case, geodesics project to approximately straight
 * lines (these are exactly straight if the geodesic includes the center
 * point).  For details, see Section 8 of :cite:`Karney2013`.
 *
 * **Classification**: Azimuthal
 *
 * **Available forms**: Forward and inverse, spherical and ellipsoidal
 *
 * **Defined area**: Within a quarter circumference of the center point
 *
 * **Alias**: gnom
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=gnom +lat_0=90 +lon_0=-50 +R=6.4e6
 * ```
 *
 * ## Required Parameters
 * - None, all parameters are optional for this projection.
 *
 * ## Optional Parameters
 * - `+lon_0`: Longitude of origin (central meridian).
 * - `+lat_0`: Latitude of origin.
 * - `+x_0`: False easting.
 * - `+y_0`: False northing.
 * - `+ellps`: Ellipsoid.
 * - `+R`: Earth radius.
 *
 * Reference:
 * Wolfram Mathworld "Gnomonic Projection"
 * http://mathworld.wolfram.com/GnomonicProjection.html
 * Accessed: 12th November 2009
 *
 * ![Gnomonic](https://github.com/Open-S2/gis-tools/blob/master/assets/proj4/projections/images/gnom.png?raw=true)
 */
export class Gnomonic extends ProjectionBase implements ProjectionTransform {
  name = 'GnomonicProjection';
  static names = ['GnomonicProjection', 'Gnomonic Projection', 'gnom'];
  // Gnomonic specific variables
  rc: number;
  declare phic0: number;
  sinP14: number;
  cosP14: number;
  infinityDist: number;

  /**
   * Preps an Gnomonic projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    const { sin, cos } = Math;
    super(params);

    this.sinP14 = sin(this.lat0);
    this.cosP14 = cos(this.lat0);
    // Approximation for projecting points to the horizon (infinity)
    this.infinityDist = 1_000 * this.a;
    this.rc = 1;
  }

  /**
   * Gnomonic forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const { abs, sin, cos } = Math;
    const { x: lon, y: lat } = p;
    let x, y;
    /* Forward equations
          -----------------*/
    const dlon = adjustLon(lon - this.long0); /* delta longitude value      */
    const sinphi = sin(lat);
    const cosphi = cos(lat);
    const coslon = cos(dlon); /* cos of longitude        */
    const g = this.sinP14 * sinphi + this.cosP14 * cosphi * coslon;
    const ksp = 1; /* scale factor          */
    if (g > 0 || abs(g) <= EPSLN) {
      x = this.x0 + (this.a * ksp * cosphi * sin(dlon)) / g;
      y = this.y0 + (this.a * ksp * (this.cosP14 * sinphi - this.sinP14 * cosphi * coslon)) / g;
    } else {
      // Point is in the opposing hemisphere and is unprojectable
      // We still need to return a reasonable point, so we project
      // to infinity, on a bearing
      // equivalent to the northern hemisphere equivalent
      // This is a reasonable approximation for short shapes and lines that
      // straddle the horizon.
      x = this.x0 + this.infinityDist * cosphi * sin(dlon);
      y = this.y0 + this.infinityDist * (this.cosP14 * sinphi - this.sinP14 * cosphi * coslon);
    }
    p.x = x;
    p.y = y;
  }

  /**
   * Gnomonic inverse equations--mapping x-y to lon-lat
   * @param p - Gnomonic point
   */
  inverse(p: VectorPoint): void {
    const { sin, cos, sqrt, atan2 } = Math;
    let sinc, cosc;
    let c;
    let lon, lat;
    /* Inverse equations
          -----------------*/
    p.x = (p.x - this.x0) / this.a;
    p.y = (p.y - this.y0) / this.a;
    p.x /= this.k0;
    p.y /= this.k0;
    const rh = sqrt(p.x * p.x + p.y * p.y); /* Rho */
    if (rh !== 0) {
      c = atan2(rh, this.rc);
      sinc = sin(c);
      cosc = cos(c);
      lat = asinz(cosc * this.sinP14 + (p.y * sinc * this.cosP14) / rh);
      lon = atan2(p.x * sinc, rh * this.cosP14 * cosc - p.y * this.sinP14 * sinc);
      lon = adjustLon(this.long0 + lon);
    } else {
      lat = this.phic0;
      lon = 0;
    }
    p.x = lon;
    p.y = lat;
  }
}
