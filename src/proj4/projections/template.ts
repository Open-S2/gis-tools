import { EPSLN } from '../constants';
import { WGS84Projection } from '.';
import { adjustLat, adjustLon } from '../common';

import type { ProjectionTransformDefinition } from '.';
import type { VectorPoint } from 's2-tools/geometry';

const { abs, pow, sin, cos, sqrt, atan2, asin, log } = Math;

/**
 *
 */
export class XXXXXXXXXXXX extends WGS84Projection implements ProjectionTransformDefinition {
  name = 'XXXXXXXXXXXX';
  names = [this.name, 'XXXXXXXXXXXX', 'XXXXXXXXXXXX'];
  // XXXXXXXXXXXX specific variables

  /** Preps an XXXXXXXXXXXX projection */
  constructor() {
    super();
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
