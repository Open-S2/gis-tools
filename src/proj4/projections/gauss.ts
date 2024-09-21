import { ProjectionBase } from '.';
import { srat } from '../common';
import { HALF_PI, QUART_PI } from '../constants';

import type { VectorPoint } from 's2-tools/geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

const { abs, pow, sin, cos, sqrt, asin, tan, atan } = Math;

/**
 * Gauss Kruger (deprecated form of Transverse Mercator)
 */
export class GaussKruger extends ProjectionBase implements ProjectionTransform {
  name = 'GaussKruger';
  static names = ['GaussKruger', 'gauss', 'Gauss Kruger', 'Gauss_Kruger'];
  // GaussKruger specific variables
  C: number;
  phic0: number;
  rc: number;
  ratexp: number;
  K: number;

  /**
   * Preps an GaussKruger projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);

    const sphi = sin(this.lat0);
    let cphi = cos(this.lat0);
    cphi *= cphi;
    this.rc = sqrt(1 - this.es) / (1 - this.es * sphi * sphi);
    this.C = sqrt(1 + (this.es * cphi * cphi) / (1 - this.es));
    this.phic0 = asin(sphi / this.C);
    this.ratexp = 0.5 * this.C * this.e;
    this.K =
      tan(0.5 * this.phic0 + QUART_PI) /
      (pow(tan(0.5 * this.lat0 + QUART_PI), this.C) * srat(this.e * sphi, this.ratexp));
  }

  /**
   * GaussKruger forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const { x: lon, y: lat } = p;
    p.y =
      2 *
        atan(
          this.K * pow(tan(0.5 * lat + QUART_PI), this.C) * srat(this.e * sin(lat), this.ratexp),
        ) -
      HALF_PI;
    p.x = this.C * lon;
  }

  /**
   * GaussKruger inverse equations--mapping x-y to lon-lat
   * @param p - GaussKruger point
   */
  inverse(p: VectorPoint): void {
    const DEL_TOL = 1e-14;
    const lon = p.x / this.C;
    let lat = p.y;
    const num = pow(tan(0.5 * lat + QUART_PI) / this.K, 1 / this.C);
    let i = 0;
    for (i = 20; i > 0; --i) {
      lat = 2 * atan(num * srat(this.e * sin(p.y), -0.5 * this.e)) - HALF_PI;
      if (abs(lat - p.y) < DEL_TOL) {
        break;
      }
      p.y = lat;
    }
    /* convergence failed */
    if (i === 0) throw new Error('convergence failed');
    p.x = lon;
    p.y = lat;
  }
}
