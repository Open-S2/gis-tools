import { ProjectionBase } from '.';
import { hypot } from '../common';

import type { VectorPoint } from 's2-tools/geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

const { sin, cos, sqrt, tan, atan, atan2 } = Math;

/**
 * # Geostationary Satellite View (geos)
 *
 * **Classification**: Azimuthal
 *
 * **Available forms**: Forward and inverse, spherical and ellipsoidal
 *
 * **Defined area**: Global
 *
 * **Alias**: geos
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=geos +h=35785831.0 +lon_0=-60 +sweep=y
 * ```
 *
 * ## Required Parameters
 * - `+h`: The height of the satellite above the earth.
 *
 * ## Optional Parameters
 * - `+sweep`: Sweep angle axis of the viewing instrument, can be "x" or "y" (default is "y").
 * - `+lon_0`: Central meridian (longitude of origin).
 * - `+R`: Earth radius.
 * - `+ellps`: Ellipsoid.
 * - `+x_0`: False easting.
 * - `+y_0`: False northing.
 *
 * ![Geostationary Satellite View](./images/geos.png)
 */
export class GeostationarySatelliteView extends ProjectionBase implements ProjectionTransform {
  name = 'GeostationarySatelliteView';
  static names = [
    'GeostationarySatelliteView',
    'Geostationary Satellite View',
    'Geostationary_Satellite',
    'geos',
  ];
  // GeostationarySatelliteView specific variables
  declare sweep: string;
  flip_axis: number;
  declare h: number;
  radiusG: number;
  radiusG1: number;
  radiusP: number;
  radiusP2: number;
  radiusPInv2: number;
  C: number;
  shape: string;

  /**
   * Preps an GeostationarySatelliteView projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);
    if (this.sweep === undefined) this.sweep = 'y';
    if (this.h === undefined) this.h = 35785831.0;

    this.flip_axis = this.sweep === 'x' ? 1 : 0;
    this.radiusG1 = this.h / this.a;

    if (this.radiusG1 <= 0 || this.radiusG1 > 1e10) throw new Error('h/a out of range');

    this.radiusG = 1.0 + this.radiusG1;
    this.C = this.radiusG * this.radiusG - 1.0;

    if (this.es !== 0.0) {
      const one_es = 1.0 - this.es;
      const rone_es = 1 / one_es;

      this.radiusP = sqrt(one_es);
      this.radiusP2 = one_es;
      this.radiusPInv2 = rone_es;

      this.shape = 'ellipse'; // Use as a condition in the forward and inverse functions.
    } else {
      this.radiusP = 1.0;
      this.radiusP2 = 1.0;
      this.radiusPInv2 = 1.0;

      this.shape = 'sphere'; // Use as a condition in the forward and inverse functions.
    }
  }

  /**
   * GeostationarySatelliteView forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    let { x: lon, y: lat } = p;
    let tmp, v_x, v_y, v_z;
    lon = lon - this.long0;
    if (this.shape === 'ellipse') {
      lat = atan(this.radiusP2 * tan(lat));
      const r = this.radiusP / hypot(this.radiusP * cos(lat), sin(lat));
      v_x = r * cos(lon) * cos(lat);
      v_y = r * sin(lon) * cos(lat);
      v_z = r * sin(lat);
      if ((this.radiusG - v_x) * v_x - v_y * v_y - v_z * v_z * this.radiusPInv2 < 0.0) {
        throw new Error('h/a out of range');
      }
      tmp = this.radiusG - v_x;
      if (this.flip_axis) {
        p.x = this.radiusG1 * atan(v_y / hypot(v_z, tmp));
        p.y = this.radiusG1 * atan(v_z / tmp);
      } else {
        p.x = this.radiusG1 * atan(v_y / tmp);
        p.y = this.radiusG1 * atan(v_z / hypot(v_y, tmp));
      }
    } else if (this.shape === 'sphere') {
      tmp = cos(lat);
      v_x = cos(lon) * tmp;
      v_y = sin(lon) * tmp;
      v_z = sin(lat);
      tmp = this.radiusG - v_x;
      if (this.flip_axis) {
        p.x = this.radiusG1 * atan(v_y / hypot(v_z, tmp));
        p.y = this.radiusG1 * atan(v_z / tmp);
      } else {
        p.x = this.radiusG1 * atan(v_y / tmp);
        p.y = this.radiusG1 * atan(v_z / hypot(v_y, tmp));
      }
    }
    p.x = p.x * this.a;
    p.y = p.y * this.a;
  }

  /**
   * GeostationarySatelliteView inverse equations--mapping x-y to lon-lat
   * @param p - GeostationarySatelliteView point
   */
  inverse(p: VectorPoint): void {
    let v_x = -1.0;
    let v_y = 0.0;
    let v_z = 0.0;
    let a, b, det, k;
    p.x = p.x / this.a;
    p.y = p.y / this.a;
    if (this.shape === 'ellipse') {
      if (this.flip_axis) {
        v_z = tan(p.y / this.radiusG1);
        v_y = tan(p.x / this.radiusG1) * hypot(1.0, v_z);
      } else {
        v_y = tan(p.x / this.radiusG1);
        v_z = tan(p.y / this.radiusG1) * hypot(1.0, v_y);
      }
      const v_zp = v_z / this.radiusP;
      a = v_y * v_y + v_zp * v_zp + v_x * v_x;
      b = 2 * this.radiusG * v_x;
      det = b * b - 4 * a * this.C;
      if (det < 0.0) {
        throw new Error('det < 0');
      }
      k = (-b - sqrt(det)) / (2.0 * a);
      v_x = this.radiusG + k * v_x;
      v_y *= k;
      v_z *= k;
      p.x = atan2(v_y, v_x);
      p.y = atan((v_z * cos(p.x)) / v_x);
      p.y = atan(this.radiusPInv2 * tan(p.y));
    } else if (this.shape === 'sphere') {
      if (this.flip_axis) {
        v_z = tan(p.y / this.radiusG1);
        v_y = tan(p.x / this.radiusG1) * sqrt(1.0 + v_z * v_z);
      } else {
        v_y = tan(p.x / this.radiusG1);
        v_z = tan(p.y / this.radiusG1) * sqrt(1.0 + v_y * v_y);
      }
      a = v_y * v_y + v_z * v_z + v_x * v_x;
      b = 2 * this.radiusG * v_x;
      det = b * b - 4 * a * this.C;
      if (det < 0.0) {
        throw new Error('det < 0');
      }
      k = (-b - sqrt(det)) / (2.0 * a);
      v_x = this.radiusG + k * v_x;
      v_y *= k;
      v_z *= k;
      p.x = atan2(v_y, v_x);
      p.y = atan((v_z * cos(p.x)) / v_x);
    }
    p.x = p.x + this.long0;
  }
}
