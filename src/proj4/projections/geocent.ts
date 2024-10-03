import { ProjectionBase } from '.';
import { geocentricToGeodetic, geodeticToGeocentric } from '../datum';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

/** Geocentric Projection */
export class Geocentric extends ProjectionBase implements ProjectionTransform {
  name = 'Geocentric';
  static names = ['Geocentric', 'geocent'];
  // Geocentric specific variables

  /**
   * Preps an Geocentric projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);
  }

  /**
   * Geocentric forward equations--mapping lon-lat to x-y
   *
   * The function Convert_Geodetic_To_Geocentric converts geodetic coordinates
   * (latitude, longitude, and height) to geocentric coordinates (X, Y, Z),
   * according to the current ellipsoid parameters.
   *
   * Latitude  : Geodetic latitude in radians                     (input)
   * Longitude : Geodetic longitude in radians                    (input)
   * Height    : Geodetic height, in meters                       (input)
   * X         : Calculated Geocentric X coordinate, in meters    (output)
   * Y         : Calculated Geocentric Y coordinate, in meters    (output)
   * Z         : Calculated Geocentric Z coordinate, in meters    (output)
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    geodeticToGeocentric(p, this.es, this.a);
  }

  /**
   * Geocentric inverse equations--mapping x-y to lon-lat
   * @param p - Geocentric point
   */
  inverse(p: VectorPoint): void {
    geocentricToGeodetic(p, this.es, this.a, this.b);
  }
}
