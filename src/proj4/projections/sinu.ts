import { ProjectionBase } from '.';
import { EPSLN, HALF_PI } from '../constants';
import { adjustLat, adjustLon, asinz, pjEnfn, pjInvMlfn, pjMlfn } from '../common';

import type { En } from '../common';
import type { VectorPoint } from 's2-tools/geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

const { abs, sin, cos, sqrt, asin } = Math;

/**
 * # Sinusoidal (Sanson-Flamsteed)
 *
 * **Classification**: Pseudocylindrical
 *
 * **Available forms**: Forward and inverse, spherical and ellipsoidal
 *
 * **Defined area**: Global
 *
 * **Alias**: sinu
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=sinu
 * ```
 *
 * ## Parameters
 *
 * All parameters are optional.
 *
 * - `+lon_0=<value>`: Central meridian.
 * - `+R=<value>`: Radius of the sphere or semi-major axis of the ellipsoid.
 * - `+x_0=<value>`: False easting.
 * - `+y_0=<value>`: False northing.
 *
 * ## Mathematical Definition
 *
 * MacBryde and Thomas developed generalized formulas for several of the
 * pseudocylindricals with sinusoidal meridians. The formulas describing the Sinusoidal
 * projection are:
 *
 * Forward projection:
 * ```
 * x = C * λ * (m + cos(θ)) / (m + 1)
 * y = C * θ
 * ```
 *
 * Inverse projection:
 * ```
 * λ = x * (m + 1) / (C * (m + cos(y / C)))
 * θ = y / C
 * ```
 *
 * Where:
 * ```
 * C = sqrt((m + 1) / n)
 * ```
 *
 * ## Further Reading
 * - [Wikipedia](https://en.wikipedia.org/wiki/Sinusoidal_projection)
 *
 * ![Sinusoidal (Sanson-Flamsteed)](./images/sinu.png)
 */
export class Sinusoidal extends ProjectionBase implements ProjectionTransform {
  name = 'Sinusoidal';
  static names = ['Sinusoidal', 'sinu'];
  // Sinusoidal specific variables
  declare en: En;
  declare n: number;
  declare m: number;
  declare Cy: number;
  declare Cx: number;

  /**
   * Preps an Sinusoidal projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);

    if (!this.sphere) {
      this.en = pjEnfn(this.es);
    } else {
      this.n = 1;
      this.m = 0;
      this.es = 0;
      this.Cy = sqrt((this.m + 1) / this.n);
      this.Cx = this.Cy / (this.m + 1);
    }
  }

  /**
   * Sinusoidal forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    let x, y;
    let lon = p.x;
    let lat = p.y;
    /* Forward equations
        -----------------*/
    lon = adjustLon(lon - this.long0);
    if (this.sphere) {
      if (!this.m) {
        lat = this.n !== 1 ? asin(this.n * sin(lat)) : lat;
      } else {
        const k = this.n * sin(lat);
        for (let i = 20; i; --i) {
          const V = (this.m * lat + sin(lat) - k) / (this.m + cos(lat));
          lat -= V;
          if (abs(V) < EPSLN) {
            break;
          }
        }
      }
      x = this.a * this.Cx * lon * (this.m + cos(lat));
      y = this.a * this.Cy * lat;
    } else {
      const s = sin(lat);
      const c = cos(lat);
      y = this.a * pjMlfn(lat, s, c, this.en);
      x = (this.a * lon * c) / sqrt(1 - this.es * s * s);
    }
    p.x = x;
    p.y = y;
  }

  /**
   * Sinusoidal inverse equations--mapping x-y to lon-lat
   * @param p - Sinusoidal point
   */
  inverse(p: VectorPoint): void {
    let lat, temp, lon, s;
    p.x -= this.x0;
    lon = p.x / this.a;
    p.y -= this.y0;
    lat = p.y / this.a;
    if (this.sphere) {
      lat /= this.Cy;
      lon = lon / (this.Cx * (this.m + cos(lat)));
      if (this.m) {
        lat = asinz((this.m * lat + sin(lat)) / this.n);
      } else if (this.n !== 1) {
        lat = asinz(sin(lat) / this.n);
      }
      lon = adjustLon(lon + this.long0);
      lat = adjustLat(lat);
    } else {
      lat = pjInvMlfn(p.y / this.a, this.es, this.en);
      s = abs(lat);
      if (s < HALF_PI) {
        s = sin(lat);
        temp = this.long0 + (p.x * sqrt(1 - this.es * s * s)) / (this.a * cos(lat));
        //temp = this.long0 + p.x / (this.a * cos(lat));
        lon = adjustLon(temp);
      } else if (s - EPSLN < HALF_PI) {
        lon = this.long0;
      }
    }
    p.x = lon;
    p.y = lat;
  }
}
