import { D2R, PJD_NODATUM, R2D } from '../constants';

import type { DatumParams } from '../../readers/wkt';
import type { GridDefinition } from '../../readers/nadgrid';
import type { ProjectionTransform } from '.';
import type { VectorPoint } from '../../geometry';

/** Define the projection with all it's variable components */
export interface ProjectionParams {
  name?: string;
  PROJECTION?: string;
  datumCode?: string;
  datumType?: number;
  srsCode?: string;
  long0?: number;
  long1?: number;
  long2?: number;
  longc?: number;
  lat0?: number;
  lat1?: number;
  lat2?: number;
  lamc?: number;
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
  rc?: number;
  es?: number;
  ep2?: number;
  alpha?: number;
  gamma?: number;
  zone?: number;
  h?: number;
  azi?: number;
  tilt?: number;
  sweep?: 'x' | 'y';
  rectifiedGridAngle?: number;
  utmSouth?: boolean;
  datumParams?: DatumParams;
  scaleFactor?: number;
  toMeter?: number;
  units?: string;
  fromGreenwich?: number;
  approx?: boolean;
  axis?: string;
  nadgrids?: string;
  sphere?: boolean;
  ellps?: string;
  noDefs?: boolean;
  noOff?: boolean;
  noRot?: boolean;
  rA?: boolean;
  projName?: string;
  grids?: GridDefinition[];
}

/** Base class for all projections */
export class ProjectionBase implements ProjectionTransform {
  name = 'longlat';
  projName?: string;
  static names: string[] = ['longlat', 'identity'];
  datumCode = 'none';
  datumType = PJD_NODATUM;
  datumParams: DatumParams = [0, 0, 0, 0, 0, 0, 0];
  srsCode = '';
  // these are all variables must have a default value across all projections
  long0 = 0;
  long1 = 0;
  lat0 = 0;
  lat1 = 0;
  lat2 = 0;
  latTs?: number;
  a = 0;
  b = 0;
  e = 0;
  x0 = 0;
  y0 = 0;
  k?: number;
  k0 = 1;
  rf = 0;
  rA = false;
  rc?: number;
  es = 0;
  ep2 = 0;
  alpha?: number;
  gamma?: number;
  zone?: number;
  rectifiedGridAngle?: number;
  utmSouth = false;
  toMeter?: number;
  units = 'm';
  fromGreenwich = 0;
  approx = false;
  axis = 'enu';
  nadgrids = '@null';
  grids?: GridDefinition[];
  sphere = false;
  ellps = 'wgs84'; // Ellipsoid name

  /** @param params - projection specific parameters */
  constructor(params?: ProjectionParams) {
    Object.assign(this, params ?? {});
  }

  /**
   * Forward projection from x-y to lon-lat. In this case, radians to degrees
   * @param p - Vector Point. This is a placeholder for a lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    p.x *= R2D;
    p.y *= R2D;
  }

  /**
   * Inverse projection from lon-lat to x-y. In this case, degrees to radians
   * @param p - Vector Point. This is a placeholder for a lon-lat WGS84 point
   */
  inverse(p: VectorPoint): void {
    p.x *= D2R;
    p.y *= D2R;
  }
}
