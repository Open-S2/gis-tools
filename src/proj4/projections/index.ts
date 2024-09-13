import { AlbersConicEqualArea } from './aea';
import { AzimuthalEquidistantProjection } from './aeqd';
import { BonneWernerProjection } from './bonne';
import { FT_TO_M } from '../constants';

import type { VectorPoint } from 's2json-spec';

export * from './aea';
export * from './aeqd';
export * from './bonne';

/** All projections need these parameters */
export interface ProjectionTransformDefinition {
  name: string;
  names: string[];
  datum: string;
  forward: (p: VectorPoint) => VectorPoint;
  inverse: (p: VectorPoint) => VectorPoint;
}

/** Base class for all projections */
export class ProjectionTransform implements ProjectionTransformDefinition {
  name: string = '';
  names: string[] = [];
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
  k0 = 1;
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
  ellps = 'wgs84'; // Ellipsoid name

  /**
   * @param srsCode - projection code to transform from
   * @param names - optionally update projection names
   */
  constructor(srsCode?: string, names?: string[]) {
    if (srsCode !== undefined) this.srsCode = srsCode;
    if (names !== undefined && names.length > 0) {
      this.names = names;
      this.name = names[0];
    }
  }

  /**
   * Forward projection from x-y to lon-lat. In this case, it is a no-op
   * @param p - Vector Point. This is a placeholder for a lon-lat WGS84 point
   * @returns - the point itself
   */
  forward(p: VectorPoint): VectorPoint {
    return { ...p };
  }
  /**
   * Inverse projection from lon-lat to x-y. In this case, it is a no-op
   * @param p - Vector Point. This is a placeholder for a lon-lat WGS84 point
   * @returns - the point itself
   */
  inverse(p: VectorPoint): VectorPoint {
    return { ...p };
  }
}

/** WGS84 projection */
export class WGS84Projection extends ProjectionTransform implements ProjectionTransformDefinition {
  /** Prepares a WGS84 projection read to convert to and from lon-lat */
  constructor() {
    super('+title=WGS 84 (long/lat) +proj=longlat +ellps=WGS84 +datum=WGS84 +units=degrees', [
      'WGS84',
      'EPSG:4326',
    ]);
  }
}

/** NAD83 projection */
export class NAD83Projection extends ProjectionTransform implements ProjectionTransformDefinition {
  /** Prepares a NAD83 projection read to convert to and from lon-lat */
  constructor() {
    super(
      '+title=NAD83 (long/lat) +proj=longlat +a=6378137.0 +b=6356752.31414036 +ellps=GRS80 +datum=NAD83 +units=degrees',
      ['NAD83', 'EPSG:4269'],
    );
  }
}

/** PseudoMercator projection */
export class PseudoMercatorProjection
  extends ProjectionTransform
  implements ProjectionTransformDefinition
{
  /** Prepares a PseudoMercator projection read to convert to and from lon-lat */
  constructor() {
    super(
      '+title=WGS 84 / Pseudo-Mercator +proj=merc +a=6378137 +b=6378137 +lat_ts=0.0 +lon_0=0.0 +x_0=0.0 +y_0=0 +k=1.0 +units=m +nadgrids=@null +no_defs',
      ['PseudoMercator', 'EPSG:3857', 'EPSG:3785', 'EPSG:900913', 'EPSG:102113', 'GOOGLE'],
    );
  }
}

/** Contains all projections */
export const ALL_PROJECTIONS: ProjectionTransformDefinition[] = [
  new AlbersConicEqualArea(),
  new AzimuthalEquidistantProjection(),
  new BonneWernerProjection(),
];

/**
 * Builds most commonly used projections
 * @returns - an array of default projections
 */
export const DEFAULT_PROJECTIONS: [
  WGS_84: ProjectionTransformDefinition,
  NAD_83: ProjectionTransformDefinition,
  PSEUDO_MERCATOR: ProjectionTransformDefinition,
] = [new WGS84Projection(), new NAD83Projection(), new PseudoMercatorProjection()];
