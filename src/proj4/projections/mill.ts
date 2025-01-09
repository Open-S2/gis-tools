import { ProjectionBase } from '.';
import { adjustLon } from '../common';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

/**
 * # Miller Cylindrical
 *
 * The Miller cylindrical projection is a modified Mercator projection, proposed by
 * Osborn Maitland Miller in 1942.
 *
 * **Classification**: Neither conformal nor equal area cylindrical
 *
 * **Available forms**: Forward and inverse spherical
 *
 * **Defined area**: Global, but best used near the equator
 *
 * **Alias**: mill
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=mill
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
 * ## Usage Example
 * Using Central meridian 90°W:
 * ```bash
 * $ echo -100 35 | proj +proj=mill +lon_0=90w
 * -1113194.91      4061217.24
 * ```
 *
 * ## Mathematical Definition
 * ### Forward projection:
 * ```
 * x = \lambda
 * y = 1.25 * \ln \left[ \tan \left(\frac{\pi}{4} + 0.4 * \phi \right) \right]
 * ```
 * ### Inverse projection:
 * ```
 * \lambda = x
 * \phi = 2.5 * ( \arctan \left[ e^{0.8 * y} \right] - \frac{\pi}{4} )
 * ```
 *
 * ## Further reading
 * - [Wikipedia on Miller Cylindrical](https://en.wikipedia.org/wiki/Miller_cylindrical_projection)
 * - "New Equal-Area Map Projections for Noncircular Regions", John P. Snyder, The American Cartographer, Vol 15, No. 4, October 1988, pp. 341-355.
 *
 * ![Miller Cylindrical](https://github.com/Open-S2/gis-tools/blob/master/assets/proj4/projections/images/mill.png?raw=true)
 */
export class MillerCylindrical extends ProjectionBase implements ProjectionTransform {
  name = 'MillerCylindrical';
  static names = ['MillerCylindrical', 'Miller_Cylindrical', 'mill'];
  // MillerCylindrical specific variables

  /**
   * Preps an MillerCylindrical projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);
  }

  /**
   * MillerCylindrical forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const { tan, PI, log } = Math;
    const { x: lon, y: lat } = p;
    const dlon = adjustLon(lon - this.long0);
    const x = this.x0 + this.a * dlon;
    const y = this.y0 + this.a * log(tan(PI / 4 + lat / 2.5)) * 1.25;
    p.x = x;
    p.y = y;
  }

  /**
   * MillerCylindrical inverse equations--mapping x-y to lon-lat
   * @param p - MillerCylindrical point
   */
  inverse(p: VectorPoint): void {
    const { atan, PI, exp } = Math;
    p.x -= this.x0;
    p.y -= this.y0;
    const lon = adjustLon(this.long0 + p.x / this.a);
    const lat = 2.5 * (atan(exp((0.8 * p.y) / this.a)) - PI / 4);
    p.x = lon;
    p.y = lat;
  }
}
