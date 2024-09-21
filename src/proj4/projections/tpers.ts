import { ProjectionBase } from '.';
import { hypot } from '../common';
import { D2R, EPSLN, HALF_PI } from '../constants';

import type { VectorPoint } from 's2-tools/geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

const { abs, sin, cos, sqrt, atan2, asin } = Math;

/** The 4 possible modes of the TiltedPerspective projection */
export enum MODE {
  N_POLE = 0,
  S_POLE = 1,
  EQUIT = 2,
  OBLIQ = 3,
}

/**
 * # Tilted Perspective
 *
 * Tilted Perspective is similar to `nsper` in that it simulates a
 * perspective view from a height. Where `nsper` projects onto a plane tangent to
 * the surface, Tilted Perspective orients the plane towards the direction of the
 * view. Thus, extra parameters specifying azimuth and tilt are required beyond
 * `nsper`'s `h`. As with `nsper`, `lat_0` & `lon_0` are
 * also required for satellite position.
 *
 * **Classification**: Azimuthal
 *
 * **Available forms**: Forward and inverse, spherical projection
 *
 * **Defined area**: Global
 *
 * **Alias**: tpers
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=tpers +h=5500000 +lat_0=40
 * ```
 *
 * ## Required Parameters
 * - `+h`: Height of the perspective view.
 * - `+lat_0`: Latitude of the projection center.
 * - `+lon_0`: Longitude of the projection center.
 *
 * ## Optional Parameters
 * - `+azi=<value>`: Bearing in degrees away from north. *Defaults to 0.0.*
 * - `+tilt=<value>`: Angle in degrees away from nadir. *Defaults to 0.0.*
 *
 * ![Tilted perspective](./images/tpers.png)
 */
export class TiltedPerspective extends ProjectionBase implements ProjectionTransform {
  name = 'TiltedPerspective';
  static names = ['TiltedPerspective', 'Tilted_Perspective', 'tpers'];
  // TiltedPerspective specific variables
  mode: MODE;
  declare h: number; // default is Karman line, no default in PROJ.7
  declare azi: number; // default is North
  declare tilt: number; // default is Nadir
  declare long0: number; // default is Greenwich, conversion to rad is automatic
  declare lat0: number; // default is Equator, conversion to rad is automatic
  declare sinph0: number;
  declare cosph0: number;
  pn1: number;
  p: number;
  rp: number;
  h1: number;
  pfact: number;
  cg: number;
  sg: number;
  cw: number;
  sw: number;

  /**
   * Preps an TiltedPerspective projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);

    if (this.h === undefined) this.h = 100_000;
    if (this.azi === undefined) this.azi = 0;
    if (this.long0 === undefined) this.long0 = 0;
    if (this.lat0 === undefined) this.lat0 = 0;
    if (this.sinph0 === undefined) this.sinph0 = 0;
    if (this.cosph0 === undefined) this.cosph0 = 1;
    if (this.tilt === undefined) this.tilt = 0;
    // azi and tilt are in radians
    this.azi *= D2R;
    this.tilt *= D2R;

    if (abs(abs(this.lat0) - HALF_PI) < EPSLN) {
      this.mode = this.lat0 < 0 ? MODE.S_POLE : MODE.N_POLE;
    } else if (abs(this.lat0) < EPSLN) {
      this.mode = MODE.EQUIT;
    } else {
      this.mode = MODE.OBLIQ;
      this.sinph0 = sin(this.lat0);
      this.cosph0 = cos(this.lat0);
    }

    this.pn1 = this.h / this.a; // Normalize relative to the Earth's radius

    if (this.pn1 <= 0 || this.pn1 > 1e10) {
      throw new Error('Invalid height');
    }

    this.p = 1 + this.pn1;
    this.rp = 1 / this.p;
    this.h1 = 1 / this.pn1;
    this.pfact = (this.p + 1) * this.h1;
    this.es = 0;

    const omega = this.tilt;
    const gamma = this.azi;
    this.cg = cos(gamma);
    this.sg = sin(gamma);
    this.cw = cos(omega);
    this.sw = sin(omega);
  }

  /**
   * TiltedPerspective forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    p.x -= this.long0;
    const sinphi = sin(p.y);
    const cosphi = cos(p.y);
    const coslam = cos(p.x);
    let x, y;
    switch (this.mode) {
      case MODE.OBLIQ:
        y = this.sinph0 * sinphi + this.cosph0 * cosphi * coslam;
        break;
      case MODE.EQUIT:
        y = cosphi * coslam;
        break;
      case MODE.S_POLE:
        y = -sinphi;
        break;
      case MODE.N_POLE:
        y = sinphi;
        break;
    }
    y = this.pn1 / (this.p - y);
    x = y * cosphi * sin(p.x);
    switch (this.mode) {
      case MODE.OBLIQ:
        y *= this.cosph0 * sinphi - this.sinph0 * cosphi * coslam;
        break;
      case MODE.EQUIT:
        y *= sinphi;
        break;
      case MODE.N_POLE:
        y *= -(cosphi * coslam);
        break;
      case MODE.S_POLE:
        y *= cosphi * coslam;
        break;
    }
    // Tilt
    const yt = y * this.cg + x * this.sg;
    const ba = 1 / (yt * this.sw * this.h1 + this.cw);
    x = (x * this.cg - y * this.sg) * this.cw * ba;
    y = yt * ba;
    p.x = x * this.a;
    p.y = y * this.a;
  }

  /**
   * TiltedPerspective inverse equations--mapping x-y to lon-lat
   * @param p - TiltedPerspective point
   */
  inverse(p: VectorPoint): void {
    p.x /= this.a;
    p.y /= this.a;
    const r = { x: p.x, y: p.y };
    // Un-Tilt
    const yt = 1 / (this.pn1 - p.y * this.sw);
    const bm = this.pn1 * p.x * yt;
    const bq = this.pn1 * p.y * this.cw * yt;
    p.x = bm * this.cg + bq * this.sg;
    p.y = bq * this.cg - bm * this.sg;
    const rh = hypot(p.x, p.y);
    if (abs(rh) < EPSLN) {
      r.x = 0;
      r.y = p.y;
    } else {
      let sinz;
      sinz = 1 - rh * rh * this.pfact;
      sinz = (this.p - sqrt(sinz)) / (this.pn1 / rh + rh / this.pn1);
      const cosz = sqrt(1 - sinz * sinz);
      switch (this.mode) {
        case MODE.OBLIQ:
          r.y = asin(cosz * this.sinph0 + (p.y * sinz * this.cosph0) / rh);
          p.y = (cosz - this.sinph0 * sin(r.y)) * rh;
          p.x *= sinz * this.cosph0;
          break;
        case MODE.EQUIT:
          r.y = asin((p.y * sinz) / rh);
          p.y = cosz * rh;
          p.x *= sinz;
          break;
        case MODE.N_POLE:
          r.y = asin(cosz);
          p.y = -p.y;
          break;
        case MODE.S_POLE:
          r.y = -asin(cosz);
          break;
      }
      r.x = atan2(p.x, p.y);
    }
    p.x = r.x + this.long0;
    p.y = r.y;
  }
}
