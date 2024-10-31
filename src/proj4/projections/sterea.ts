import { GaussKruger } from './gauss';
import { adjustLon, hypot } from '../common';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

const { sin, cos, atan2, asin } = Math;

/**
 * # Oblique Stereographic Alternative
 *
 * **Classification**: Azimuthal
 *
 * **Available forms**: Forward and inverse, spherical and ellipsoidal
 *
 * **Defined area**: Global
 *
 * **Alias**: sterea
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=sterea +lat_0=52.1561605555556 +lon_0=5.38763888888889 +k=0.9999079 +x_0=155000 +y_0=463000 +ellps=bessel
 * ```
 *
 * ## Note
 * This projection method, referenced by EPSG as "Oblique Stereographic", is
 * for example used for the Netherlands "Amersfoort / RD New" projected CRS.
 * It gives different results than the :ref:`stere` method in the non-polar cases
 * (i.e. the oblique and equatorial case).
 *
 * ## Required Parameters
 * - None
 *
 * ## Optional Parameters
 * - `+lat_0=<value>`: Latitude of origin.
 * - `+lon_0=<value>`: Central meridian.
 * - `+k=<value>`: Scale factor.
 * - `+x_0=<value>`: False easting.
 * - `+y_0=<value>`: False northing.
 * - `+ellps=<value>`: Ellipsoid used.
 * - `+R=<value>`: Radius of the projection sphere.
 *
 * ![Oblique Stereographic Alternative](https://github.com/Open-S2/s2-tools/blob/master/assets/proj4/projections/images/sterea.png?raw=true)
 */
export class StereographicNorthPole extends GaussKruger implements ProjectionTransform {
  name = 'StereographicNorthPole';
  static names = [
    'StereographicNorthPole',
    'Stereographic_North_Pole',
    'Oblique_Stereographic',
    'sterea',
    'Oblique Stereographic Alternative',
    'Double_Stereographic',
  ];
  // StereographicNorthPole specific variables
  R2: number;
  sinc0: number;
  cosc0: number;

  /**
   * Preps an StereographicNorthPole projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);

    if (this.rc === undefined) throw new Error('rc must be defined');
    this.sinc0 = sin(this.phic0);
    this.cosc0 = cos(this.phic0);
    this.R2 = 2 * this.rc;
  }

  /**
   * StereographicNorthPole forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    p.x = adjustLon(p.x - this.long0);
    super.forward(p);
    const sinc = sin(p.y);
    const cosc = cos(p.y);
    const cosl = cos(p.x);
    const k = (this.k0 * this.R2) / (1 + this.sinc0 * sinc + this.cosc0 * cosc * cosl);
    p.x = k * cosc * sin(p.x);
    p.y = k * (this.cosc0 * sinc - this.sinc0 * cosc * cosl);
    p.x = this.a * p.x + this.x0;
    p.y = this.a * p.y + this.y0;
  }

  /**
   * StereographicNorthPole inverse equations--mapping x-y to lon-lat
   * @param p - StereographicNorthPole point
   */
  inverse(p: VectorPoint): void {
    let sinc, cosc, lon, lat;
    p.x = (p.x - this.x0) / this.a;
    p.y = (p.y - this.y0) / this.a;
    p.x /= this.k0;
    p.y /= this.k0;
    const rho = hypot(p.x, p.y);
    if (rho !== 0) {
      const c = 2 * atan2(rho, this.R2);
      sinc = sin(c);
      cosc = cos(c);
      lat = asin(cosc * this.sinc0 + (p.y * sinc * this.cosc0) / rho);
      lon = atan2(p.x * sinc, rho * this.cosc0 * cosc - p.y * this.sinc0 * sinc);
    } else {
      lat = this.phic0;
      lon = 0;
    }
    p.x = lon;
    p.y = lat;
    super.inverse(p);
    p.x = adjustLon(p.x + this.long0);
  }
}
