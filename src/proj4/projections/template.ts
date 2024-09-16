import { EPSLN } from '../constants';
import { ProjectionBase } from '.';
import { adjustLat, adjustLon } from '../common';

import type { VectorPoint } from 's2-tools/geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

const { abs, pow, sin, cos, sqrt, atan2, asin, log } = Math;

/**
 *
 */
export class XXXXXXXXXXXX extends ProjectionBase implements ProjectionTransform {
  name = 'XXXXXXXXXXXX';
  names = [this.name, 'XXXXXXXXXXXX', 'XXXXXXXXXXXX'];
  // XXXXXXXXXXXX specific variables

  /**
   * Preps an XXXXXXXXXXXX projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);
  }

  /**
   * XXXXXXXXXXXX forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   * @returns - XXXXXXXXXXXX point
   */
  forward(p: VectorPoint): VectorPoint {}

  /**
   * XXXXXXXXXXXX inverse equations--mapping x-y to lon-lat
   * @param p - XXXXXXXXXXXX point
   * @returns - lon-lat WGS84 point
   */
  inverse(p: VectorPoint): VectorPoint {}
}
