import { EPSLN } from '../constants';
import { ProjectionBase } from '.';
import { adjustLon } from '../common';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

/**
 * # Mollweide
 *
 * **Classification**: Pseudocylindrical
 *
 * **Available forms**: Forward and inverse, spherical projection
 *
 * **Defined area**: Global
 *
 * **Alias**: moll
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=moll
 * ```
 *
 * ## Required Parameters
 * - None
 *
 * ## Optional Parameters
 * - `+lon_0`: Longitude of projection center. Defaults to `0`.
 * - `+R`: Radius of the sphere.
 * - `+x_0`: False easting. Defaults to `0`.
 * - `+y_0`: False northing. Defaults to `0`.
 *
 * ## Further reading
 * - [Wikipedia on Mollweide Projection](https://en.wikipedia.org/wiki/Mollweide_projection)
 *
 * ![Mollweide](https://github.com/Open-S2/gis-tools/blob/master/assets/proj4/projections/images/moll.png?raw=true)
 */
export class Mollweide extends ProjectionBase implements ProjectionTransform {
  name = 'Mollweide';
  static names = ['Mollweide', 'moll'];
  // Mollweide specific variables

  /**
   * Preps an Mollweide projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);
  }

  /**
   * Mollweide forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const { abs, PI, sin, cos } = Math;
    const lon = p.x;
    const lat = p.y;
    let delta_lon = adjustLon(lon - this.long0);
    let theta = lat;
    const con = PI * sin(lat);
    /* Iterate using the Newton-Raphson method to find theta */
    while (true) {
      const delta_theta = -(theta + sin(theta) - con) / (1 + cos(theta));
      theta += delta_theta;
      if (abs(delta_theta) < EPSLN) {
        break;
      }
    }
    theta /= 2;
    // If the latitude is 90 deg, force the x coordinate to be "0 + false easting"
    // this is done here because of precision problems with "cos(theta)"
    if (PI / 2 - abs(lat) < EPSLN) {
      delta_lon = 0;
    }
    const x = 0.900316316158 * this.a * delta_lon * cos(theta) + this.x0;
    const y = 1.4142135623731 * this.a * sin(theta) + this.y0;
    p.x = x;
    p.y = y;
  }

  /**
   * Mollweide inverse equations--mapping x-y to lon-lat
   * @param p - Mollweide point
   */
  inverse(p: VectorPoint): void {
    const { abs, PI, sin, cos, asin } = Math;
    let arg;
    /* Inverse equations
          -----------------*/
    p.x -= this.x0;
    p.y -= this.y0;
    arg = p.y / (1.4142135623731 * this.a);
    /* Because of division by zero problems, 'arg' can not be 1.  Therefore
           a number very close to one is used instead.
           -------------------------------------------------------------------*/
    if (abs(arg) > 0.999999999999) {
      arg = 0.999999999999;
    }
    const theta = asin(arg);
    let lon = adjustLon(this.long0 + p.x / (0.900316316158 * this.a * cos(theta)));
    if (lon < -PI) {
      lon = -PI;
    }
    if (lon > PI) {
      lon = PI;
    }
    arg = (2 * theta + sin(2 * theta)) / PI;
    if (abs(arg) > 1) {
      arg = 1;
    }
    const lat = asin(arg);
    p.x = lon;
    p.y = lat;
  }
}
