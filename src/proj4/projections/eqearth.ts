/**
 * Copyright 2018 Bernie Jenny, Monash University, Melbourne, Australia.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * Equal Earth is a projection inspired by the Robinson projection, but unlike
 * the Robinson projection retains the relative size of areas. The projection
 * was designed in 2018 by Bojan Savric, Tom Patterson and Bernhard Jenny.
 *
 * Publication:
 * Bojan Savric, Tom Patterson & Bernhard Jenny (2018). The Equal Earth map
 * projection, International Journal of Geographical Information Science,
 * DOI: 10.1080/13658816.2018.1504949
 *
 * Code released August 2018
 * Ported to JavaScript and adapted for mapshaper-proj by Matthew Bloch August 2018
 * Modified for proj4js by Andreas Hocevar by Andreas Hocevar March 2024
 */

import { ProjectionBase } from '.';
import { adjustLon } from '../common';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

/**
 * # Equal Earth
 *
 * **Classification**: Pseudo cylindrical
 *
 * **Available forms**: Forward and inverse, spherical and ellipsoidal projection
 *
 * **Defined area**: Global
 *
 * **Alias**: eqearth
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=eqearth
 * ```
 *
 * ## Usage
 *
 * The Equal Earth projection is designed for world maps and retains the relative size of areas. It was inspired by the Robinson projection.
 *
 * Example:
 * ```
 * $ echo 122 47 | proj +proj=eqearth +R=1
 * 1.55    0.89
 * ```
 *
 * ## Parameters
 *
 * **Note**: All parameters for this projection are optional.
 *
 * ### Optional
 * - `+lon_0` (Central meridian)
 * - `+ellps` (Ellipsoid name)
 * - `+R` (Radius of the sphere)
 * - `+x_0` (False easting)
 * - `+y_0` (False northing)
 *
 * ## Further Reading
 * - [The Equal Earth map projection](https://www.researchgate.net/profile/Bojan_Savric2/publication/326879978_The_Equal_Earth_map_projection/links/5b69d0ae299bf14c6d951b77/The-Equal-Earth-map-projection.pdf) by Bojan Savric, Tom Patterson & Bernhard Jenny (2018)
 *
 * ![Equal Earth](https://github.com/Open-S2/s2-tools/blob/master/assets/proj4/projections/images/eqearth.png?raw=true)
 */
export class EqualEarth extends ProjectionBase implements ProjectionTransform {
  name = 'EqualEarth';
  static names = ['EqualEarth', 'Equal_Area_Cylindrical', 'eqearth', 'Equal Earth', 'Equal_Earth'];
  // EqualEarth specific variables
  es: number;
  A1 = 1.340264;
  A2 = -0.081106;
  A3 = 0.000893;
  A4 = 0.003796;
  M = Math.sqrt(3) / 2.0;

  /**
   * Preps an EqualEarth projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);
    this.es = 0;
    this.long0 = this.long0 !== undefined ? this.long0 : 0;
  }

  /**
   * EqualEarth forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const { sin, cos, asin } = Math;
    const { M, A1, A2, A3, A4, a, x0, y0, long0 } = this;
    const lam = adjustLon(p.x - long0);
    const phi = p.y;
    const paramLat = asin(M * sin(phi)),
      paramLatSq = paramLat * paramLat,
      paramLatPow6 = paramLatSq * paramLatSq * paramLatSq;
    p.x =
      (lam * cos(paramLat)) /
      (M * (A1 + 3 * A2 * paramLatSq + paramLatPow6 * (7 * A3 + 9 * A4 * paramLatSq)));
    p.y = paramLat * (A1 + A2 * paramLatSq + paramLatPow6 * (A3 + A4 * paramLatSq));
    p.x = a * p.x + x0;
    p.y = a * p.y + y0;
  }

  /**
   * EqualEarth inverse equations--mapping x-y to lon-lat
   * @param p - EqualEarth point
   */
  inverse(p: VectorPoint): void {
    const { abs, sin, cos, asin } = Math;
    const { M, A1, A2, A3, A4, a, x0, y0, long0 } = this;
    p.x = (p.x - x0) / a;
    p.y = (p.y - y0) / a;
    const EPS = 1e-9;
    const NITER = 12;
    let paramLat = p.y,
      paramLatSq,
      paramLatPow6,
      fy,
      fpy,
      dlat,
      i;
    for (i = 0; i < NITER; ++i) {
      paramLatSq = paramLat * paramLat;
      paramLatPow6 = paramLatSq * paramLatSq * paramLatSq;
      fy = paramLat * (A1 + A2 * paramLatSq + paramLatPow6 * (A3 + A4 * paramLatSq)) - p.y;
      fpy = A1 + 3 * A2 * paramLatSq + paramLatPow6 * (7 * A3 + 9 * A4 * paramLatSq);
      paramLat -= dlat = fy / fpy;
      if (abs(dlat) < EPS) {
        break;
      }
    }
    paramLatSq = paramLat * paramLat;
    paramLatPow6 = paramLatSq * paramLatSq * paramLatSq;
    p.x =
      (M * p.x * (A1 + 3 * A2 * paramLatSq + paramLatPow6 * (7 * A3 + 9 * A4 * paramLatSq))) /
      cos(paramLat);
    p.y = asin(sin(paramLat) / M);
    p.x = adjustLon(p.x + long0);
  }
}
