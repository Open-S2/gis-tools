import { D2R, FT_TO_M, R2D } from '../constants';

import type { ProjectionTransform } from '.';
import type { VectorPoint } from 's2-tools/geometry';

/** Define the projection with all it's variable components */
export interface ProjectionParams {
  name?: string;
  datum?: string;
  srsCode?: string;
  long0?: number;
  long1?: number;
  long2?: number;
  longc?: number;
  lat0?: number;
  lat1?: number;
  lat2?: number;
  latTs?: number;
  a?: number;
  b?: number;
  e?: number;
  R?: number;
  x0?: number;
  y0?: number;
  k?: number;
  k0?: number;
  rf?: number;
  rA?: number;
  rc?: number;
  es?: number;
  ep2?: number;
  alpha?: number;
  gamma?: number;
  zone?: number;
  rectifiedGridAngle?: number;
  utmSouth?: boolean;
  datumParams?: number[];
  toMeter?: number;
  units?: string;
  fromGreenwich?: number;
  approx?: boolean;
  axis?: string;
  nadgrids?: string;
  sphere?: boolean;
  ellps?: string;
  noDefs?: boolean;
}

/** Base class for all projections */
export class ProjectionBase implements ProjectionTransform {
  name: string = '';
  static names: string[] = ['longlat', 'identity'];
  datum = 'none';
  srsCode = '';
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
  R = 0;
  x0 = 0;
  y0 = 0;
  k?: number;
  k0?: number;
  rf = 0;
  rA = 0;
  rc?: number;
  es = 0;
  ep2 = 0;
  alpha?: number;
  gamma?: number;
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
  sphere = false;
  ellps = 'wgs84'; // Ellipsoid name

  /** @param params - projection specific parameters */
  constructor(params?: ProjectionParams) {
    params = params ?? ({} as ProjectionParams);
    Object.assign(this, params ?? {});

    // var nadgrids = getNadgrids(json.nadgrids);
    // var datumObj = json.datum || datum(json.datumCode, json.datum_params, sphere_.a, sphere_.b, ecc.es, ecc.ep2,
    //   nadgrids);
    // // add in the datum object
    // this.datum = datumObj;
  }

  /**
   * Forward projection from x-y to lon-lat. In this case, radians to degrees
   * @param p - Vector Point. This is a placeholder for a lon-lat WGS84 point
   * @returns - the point itself
   */
  forward(p: VectorPoint): VectorPoint {
    const { x, y, z, m } = p;
    return { x: x * R2D, y: y * R2D, z, m };
  }

  /**
   * Inverse projection from lon-lat to x-y. In this case, degrees to radians
   * @param p - Vector Point. This is a placeholder for a lon-lat WGS84 point
   * @returns - the point itself
   */
  inverse(p: VectorPoint): VectorPoint {
    const { x, y, z, m } = p;
    return { x: x * D2R, y: y * D2R, z, m };
  }
}
