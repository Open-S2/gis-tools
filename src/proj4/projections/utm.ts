import { D2R } from '../constants';
import { ExtendedTransverseMercator } from './etmerc';
import { adjustZone } from '../common';

import type { ProjectionParams, ProjectionTransform } from '.';

/**
 * # Universal Transverse Mercator (UTM)
 *
 * **Classification**: Transverse cylindrical, conformal
 *
 * **Available forms**: Forward and inverse, ellipsoidal only
 *
 * **Defined area**: Within the used zone, but transformations of coordinates in adjacent zones can be accurate
 *
 * **Alias**: utm
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=utm
 * ```
 *
 * ## Required Parameters
 * - `+zone=<value>`: Select which UTM zone to use. Can be a value between 1-60.
 *
 * ## Optional Parameters
 * - `+south`: Add this flag when using the UTM on the southern hemisphere.
 * - `+approx`: Use a faster, less accurate algorithm for the Transverse Mercator. (added in PROJ 6.0.0)
 * - `+algo=auto/evenden_snyder/poder_engsager`: Selects the algorithm to use. Defaults to `poder_engsager`. (added in PROJ 7.1)
 * - `+ellps=<value>`
 *
 * ## Usage Examples
 *
 * Convert geodetic coordinates to UTM Zone 32 on the northern hemisphere:
 * ```
 * $ echo 12 56 | proj +proj=utm +zone=32
 * 687071.44       6210141.33
 * ```
 *
 * Convert geodetic coordinates to UTM Zone 59 on the southern hemisphere:
 * ```
 * $ echo 174 -44 | proj +proj=utm +zone=59 +south
 * 740526.32       5123750.87
 * ```
 *
 * Show the relationship of UTM to TM:
 * ```
 * $ echo 121 24 | proj +proj=utm +lon_0=123 | proj -I +proj=tmerc +lon_0=123 +x_0=500000 +k=0.9996
 * 121dE 24dN
 * ```
 *
 * ## Further Reading
 * - [Wikipedia](https://en.wikipedia.org/wiki/Universal_Transverse_Mercator_coordinate_system)
 *
 * ![Universal Transverse Mercator (UTM) zones](https://github.com/Open-S2/gis-tools/blob/master/assets/proj4/projections/images/utm.png?raw=true)
 */
export class UniversalTransverseMercator
  extends ExtendedTransverseMercator
  implements ProjectionTransform
{
  name = 'UniversalTransverseMercator';
  static names = [
    'UniversalTransverseMercator',
    'Universal Transverse Mercator',
    'Universal Transverse Mercator System',
    'utm',
  ];
  // UniversalTransverseMercator specific variables

  /**
   * Preps an UniversalTransverseMercator projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params, (etmerc: ExtendedTransverseMercator) => {
      const zone = adjustZone(etmerc.zone, etmerc.long0);
      if (zone === undefined) {
        throw new Error('unknown utm zone');
      }
      etmerc.lat0 = 0;
      etmerc.long0 = (6 * Math.abs(zone) - 183) * D2R;
      etmerc.x0 = 500000;
      etmerc.y0 = etmerc.utmSouth ? 10000000 : 0;
      etmerc.k0 = 0.9996;
    });
  }
}
