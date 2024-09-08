import { FT_TO_M } from '../constants';

import type { VectorPoint } from 's2json-spec';

export * from './aea';

/**
 *
 */
export class ProjectionTransform {
  name: string = 'default';
  names: string[] = [];
  datum = 'none';
  // these are all variables must have a default value across all projections
  lon0 = 0;
  lon1 = 0;
  lon2 = 0;
  long0 = 0;
  long1 = 0;
  longc = 0;
  lat0 = 0;
  lat1 = 0;
  lat2 = 0;
  latTs = 0;
  a = 0;
  b = 0;
  e = 0;
  r = 0;
  x0 = 0;
  y0 = 0;
  k0 = 0;
  rf = 0;
  ra = 0;
  es = 0;
  alpha = 0;
  zone = 0;
  rectifiedGridAngle = 0;
  utmSouth = false;
  datumParams: number[] = [];
  toMeter = FT_TO_M;
  units = 'ft';
  fromGreenwich = 0;
  approx = false;
  axis = 'enu';
  nadgrids = '@null';
  sphere?: number;

  /**
   * @param srsCode
   */
  constructor(srsCode?: string) {}

  /**
   * @param p
   */
  forward(p: VectorPoint): VectorPoint {
    return p;
  }
  /**
   * @param p
   */
  inverse(p: VectorPoint): VectorPoint {
    return p;
  }
}

/**
 *
 */
export class LonLatProjection extends ProjectionTransform {
  name = 'longlat';
  names = [this.name, 'identity'];
  /**
   * @param srsCode
   */
  constructor(srsCode?: string) {
    super(srsCode);
  }
}
