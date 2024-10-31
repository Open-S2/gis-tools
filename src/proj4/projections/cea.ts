import { ProjectionBase } from './base';
import { adjustLon, iqsfnz, msfnz, qsfnz } from '../common';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

const { sin, cos, asin } = Math;

/**
 * # Equal Area Cylindrical Projection
 *
 * **Classification**: Cylindrical
 *
 * **Available forms**: Forward and Inverse, spherical and ellipsoidal
 *
 * **Defined area**: Global
 *
 * **Alias**: `cea`
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=cea
 * ```
 *
 * ## Named Specializations
 *
 * The Equal Area Cylindrical projection is sometimes known under other names when instantiated with particular values of the `lat_ts` parameter:
 *
 * - **Lambert cylindrical equal-area**: `lat_ts = 0`
 * - **Behrmann**: `lat_ts = 30`
 * - **Gall-Peters**: `lat_ts = 45`
 *
 * ## Optional Parameters
 * - `lat_ts`: Latitude of true scale
 * - `lon_0`: Longitude of origin
 * - `ellps`: Ellipsoid
 * - `R`: Radius of the sphere
 * - `k_0`: Scaling factor
 * - `x_0`: False easting
 * - `y_0`: False northing
 *
 * > **Note**: `lat_ts` and `k_0` are mutually exclusive. If `lat_ts` is specified, it is equivalent to setting `k_0` to:
 * >
 * > ```
 * > k_0 = cos(lat_ts) / sqrt(1 - e^2 * sin^2(lat_ts))
 * > ```
 *
 * reference:
 * "Cartographic Projection Procedures for the UNIX Environment-
 * A User's Manual" by Gerald I. Evenden,
 * USGS Open File Report 90-284and Release 4 Interim Reports (2003)
 *
 * ![Equal Area Cylindrical Projection](https://github.com/Open-S2/s2-tools/blob/master/assets/proj4/projections/images/cae.png?raw=true)
 */
export class CylindricalEqualArea extends ProjectionBase implements ProjectionTransform {
  name = 'Equal_Area_Cylindrical';
  static names = ['Equal_Area_Cylindrical', 'cea'];
  // Equal Area Cylindrical specific variables
  declare latTs: number;

  /**
   * Preps an Equal Area Cylindrical projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);
    if (this.latTs === undefined) this.latTs = 0;
    if (!this.sphere) {
      this.k0 = msfnz(this.e, sin(this.latTs), cos(this.latTs));
    }
  }

  /**
   * Equal Area Cylindrical forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const lon = p.x;
    const lat = p.y;
    let x, y;
    /* Forward equations
      -----------------*/
    const dlon = adjustLon(lon - this.long0);
    if (this.sphere) {
      x = this.x0 + this.a * dlon * cos(this.latTs);
      y = this.y0 + (this.a * sin(lat)) / cos(this.latTs);
    } else {
      const qs = qsfnz(this.e, sin(lat));
      x = this.x0 + this.a * this.k0 * dlon;
      y = this.y0 + (this.a * qs * 0.5) / this.k0;
    }

    p.x = x;
    p.y = y;
  }

  /**
   * Equal Area Cylindrical inverse equations--mapping x-y to lon-lat
   * @param p - Equal Area Cylindrical point
   */
  inverse(p: VectorPoint): void {
    p.x -= this.x0;
    p.y -= this.y0;
    let lon, lat;

    if (this.sphere) {
      lon = adjustLon(this.long0 + p.x / this.a / cos(this.latTs));
      lat = asin((p.y / this.a) * cos(this.latTs));
    } else {
      lat = iqsfnz(this.e, (2 * p.y * this.k0) / this.a);
      lon = adjustLon(this.long0 + p.x / (this.a * this.k0));
    }

    p.x = lon;
    p.y = lat;
  }
}
