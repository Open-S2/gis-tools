import { ProjectionBase } from './base.js';
import { EPSLN, HALF_PI, QUART_PI } from '../constants/index.js';
import { adjustLon, msfnz, phi2z, tsfnz } from '../common.js';

import type { VectorPoint } from '../../geometry/index.js';
import type { ProjectionParams, ProjectionTransform } from './index.js';

/**
 * # Mercator Projection
 *
 * The Mercator projection is a cylindrical map projection originating from the 16th century.
 * It is widely recognized as the first regularly used map projection. It is a conformal projection
 * where the equator projects to a straight line at constant scale. A rhumb line, or course of
 * constant heading, projects to a straight line, making it suitable for navigational purposes.
 *
 * **Classification**: Conformal cylindrical
 *
 * **Available forms**: Forward and Inverse, spherical and ellipsoidal
 *
 * **Defined area**: Global, but best used near the equator
 *
 * **Alias**: `merc`
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```ini
 * +proj=merc
 * ```
 *
 * ## Usage
 * The Mercator projection is often used for equatorial regions and navigational charts. It is not
 * suitable for world maps due to significant area distortions. For example, Greenland appears
 * larger than South America in the projection, despite Greenland's actual area being approximately
 * one-eighth of South America's.
 *
 * **Examples:**
 *
 * - Using latitude of true scale:
 *   ```bash
 *   $ echo 56.35 12.32 | proj +proj=merc +lat_ts=56.5
 *   3470306.37    759599.90
 *   ```
 * - Using scaling factor:
 *   ```bash
 *   $ echo 56.35 12.32 | proj +proj=merc +k_0=2
 *   12545706.61    2746073.80
 *   ```
 *
 * **Note**: `+lat_ts` and `+k_0` are mutually exclusive. If both are used, `+lat_ts` takes
 * precedence over `+k_0`.
 *
 * ## Parameters
 * - `lat_ts`: Latitude of true scale
 * - `k_0`: Scaling factor
 * - `lon_0`: Longitude of origin
 * - `x_0`: False easting
 * - `y_0`: False northing
 * - `ellps`: Ellipsoid
 * - `R`: Radius of the sphere
 *
 * ## Mathematical Definition
 *
 * **Spherical Form**
 * - **Forward Projection**:
 *   $$x = k_0 \cdot R \cdot \lambda$$
 *   $$y = k_0 \cdot R \cdot \psi$$
 *   where
 *   $$\psi = \ln\left(\tan\left(\frac{\pi}{4} + \frac{\phi}{2}\right)\right)$$
 * - **Inverse Projection**:
 *   $$\lambda = x / (k_0 \cdot R)$$
 *   $$\psi = y / (k_0 \cdot R)$$
 *   $$\phi = \frac{\pi}{2} - 2 \cdot \arctan\left(\exp(-\psi)\right)$$
 *
 * **Ellipsoidal Form**
 * - **Forward Projection**:
 *   $$x = k_0 \cdot a \cdot \lambda$$
 *   $$y = k_0 \cdot a \cdot \psi$$
 *   where
 *   $$\psi = \ln\left(\tan\left(\frac{\pi}{4} + \frac{\phi}{2}\right)\right) - 0.5 \cdot e \cdot \ln\left(\frac{1 + e \cdot \sin(\phi)}{1 - e \cdot \sin(\phi)}\right)$$
 * - **Inverse Projection**:
 *   $$\lambda = x / (k_0 \cdot a)$$
 *   $$\psi = y / (k_0 \cdot a)$$
 *   $$\phi = \arctan(\tau)$$
 *   where
 *   $$\tau = \tan(\phi)$$
 *
 * ## Further Reading
 * - [Wikipedia: Mercator Projection](https://en.wikipedia.org/wiki/Mercator_projection)
 * - [Wolfram Mathworld: Mercator Projection](http://mathworld.wolfram.com/MercatorProjection.html)
 *
 * ![Mercator Projection](https://github.com/Open-S2/gis-tools/blob/master/assets/proj4/projections/images/merc.png?raw=true)
 */
export class Mercator extends ProjectionBase implements ProjectionTransform {
  name = 'Mercator';
  static names = [
    'Mercator',
    'Popular Visualisation Pseudo Mercator',
    'Mercator_1SP',
    'Mercator_Auxiliary_Sphere',
    'merc',
  ];
  // Mercator specific variables

  /**
   * Preps an Mercator projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    const { sin, cos, sqrt } = Math;
    super(params);
    const con = this.b / this.a;
    this.es = 1 - con * con;
    this.e = sqrt(this.es);
    if (this.latTs !== undefined) {
      if (this.sphere) {
        this.k0 = cos(this.latTs);
      } else {
        this.k0 = msfnz(this.e, sin(this.latTs), cos(this.latTs));
      }
    } else {
      if (this.k0 === undefined) {
        if (this.k !== undefined) {
          this.k0 = this.k;
        } else {
          this.k0 = 1;
        }
      }
    }
  }

  /**
   * Mercator forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const { abs, sin, log, tan } = Math;
    const { x: lon, y: lat } = p;
    // convert to radians
    // if (lat * R2D > 90 && lat * R2D < -90 && lon * R2D > 180 && lon * R2D < -180) {
    //   return;
    // }

    let x, y;
    if (abs(abs(lat) - HALF_PI) <= EPSLN) {
      throw new Error('cass:pj_init_merc: lat == +/-90');
    } else {
      if (this.sphere) {
        x = this.x0 + this.a * this.k0 * adjustLon(lon - this.long0);
        y = this.y0 + this.a * this.k0 * log(tan(QUART_PI + 0.5 * lat));
      } else {
        const sinphi = sin(lat);
        const ts = tsfnz(this.e, lat, sinphi);
        x = this.x0 + this.a * this.k0 * adjustLon(lon - this.long0);
        y = this.y0 - this.a * this.k0 * log(ts);
      }
      p.x = x;
      p.y = y;
    }
  }

  /**
   * Mercator inverse equations--mapping x-y to lon-lat
   * @param p - Mercator point
   */
  inverse(p: VectorPoint): void {
    const { atan, exp } = Math;
    const x = p.x - this.x0;
    const y = p.y - this.y0;
    let lat: number;

    if (this.sphere) {
      lat = HALF_PI - 2 * atan(exp(-y / (this.a * this.k0)));
    } else {
      const ts = exp(-y / (this.a * this.k0));
      lat = phi2z(this.e, ts);
      if (lat === -9999) throw new Error('cass:pj_init_merc: lat == +/-90');
    }
    const lon = adjustLon(this.long0 + x / (this.a * this.k0));

    p.x = lon;
    p.y = lat;
  }
}
