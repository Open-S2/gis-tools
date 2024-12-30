import { EPSLN } from '../constants';
import { ProjectionBase } from './base';
import { adjustLon, asinz, msfnz, qsfnz } from '../common';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

/**
 * # Albers Conic Equal Area Projection
 *
 * **Classification**: Conic
 *
 * **Available forms**: Forward and inverse, spherical and ellipsoidal
 *
 * **Defined area**: Global
 *
 * **Alias**: `aea`
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=aea +lat_1=29.5 +lat_2=42.5
 * ```
 *
 * ## Required Parameters
 * - `lat1`
 * - `lat2`
 *
 * ## Optional Parameters
 * - `lon0`
 * - `ellps`
 * - `R`
 * - `x0`
 * - `y0`
 *
 * ## References
 * - https://en.wikipedia.org/wiki/Albers_projection
 *
 * ![Albers Conic Equal Area Projection](https://github.com/Open-S2/s2-tools/blob/master/assets/proj4/projections/images/aea.png?raw=true)
 */
export class AlbersConicEqualArea extends ProjectionBase implements ProjectionTransform {
  name = 'Albers_Conic_Equal_Area';
  static names = ['Albers_Conic_Equal_Area', 'Albers', 'aea'];
  // AlbersConicEqualArea specific variables
  ns0 = 0;
  temp = 0;
  e3 = 0;
  sin_po = 0;
  cos_po = 0;
  t1 = 0;
  con = 0;
  ms1 = 0;
  qs1 = 0;
  t2 = 0;
  ms2 = 0;
  qs2 = 0;
  t3 = 0;
  qs0 = 0;
  c = 0;
  rh = 0;

  /**
   * Preps an Albers Conic Equal Area projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    const { abs, pow, sin, cos, sqrt } = Math;
    super(params);
    if (abs(this.lat1 + this.lat2) < EPSLN) {
      throw new Error('cass:pj_init_aea: lat1 == lat2');
    }
    this.temp = this.b / this.a;
    this.es = 1 - pow(this.temp, 2);
    this.e3 = sqrt(this.es);

    this.sin_po = sin(this.lat1);
    this.cos_po = cos(this.lat1);
    this.t1 = this.sin_po;
    this.con = this.sin_po;
    this.ms1 = msfnz(this.e3, this.sin_po, this.cos_po);
    this.qs1 = qsfnz(this.e3, this.sin_po);

    this.sin_po = sin(this.lat2);
    this.cos_po = cos(this.lat2);
    this.t2 = this.sin_po;
    this.ms2 = msfnz(this.e3, this.sin_po, this.cos_po);
    this.qs2 = qsfnz(this.e3, this.sin_po);

    this.sin_po = sin(this.lat0);
    this.cos_po = cos(this.lat0);
    this.t3 = this.sin_po;
    this.qs0 = qsfnz(this.e3, this.sin_po);

    if (abs(this.lat1 - this.lat2) > EPSLN) {
      this.ns0 = (this.ms1 * this.ms1 - this.ms2 * this.ms2) / (this.qs2 - this.qs1);
    } else {
      this.ns0 = this.con;
    }
    this.c = this.ms1 * this.ms1 + this.ns0 * this.qs1;
    this.rh = (this.a * sqrt(this.c - this.ns0 * this.qs0)) / this.ns0;
  }

  /**
   * Albers Conical Equal Area forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const { sin, cos, sqrt } = Math;
    const lon = p.x;
    const lat = p.y;

    const sin_phi = sin(lat);

    const qs = qsfnz(this.e3, sin_phi);
    const rh1 = (this.a * sqrt(this.c - this.ns0 * qs)) / this.ns0;
    const theta = this.ns0 * adjustLon(lon - this.long0);
    const x = rh1 * sin(theta) + this.x0;
    const y = this.rh - rh1 * cos(theta) + this.y0;

    p.x = x;
    p.y = y;
  }

  /**
   * Albers Conical Equal Area inverse equations--mapping x-y to lon-lat
   * @param p - Albers Conic Equal Area point
   */
  inverse(p: VectorPoint): void {
    const { sqrt, atan2, asin } = Math;
    let rh1, qs, con, theta;
    let lat = 0;

    p.x -= this.x0;
    p.y = this.rh - p.y + this.y0;
    if (this.ns0 >= 0) {
      rh1 = sqrt(p.x * p.x + p.y * p.y);
      con = 1;
    } else {
      rh1 = -sqrt(p.x * p.x + p.y * p.y);
      con = -1;
    }
    theta = 0;
    if (rh1 !== 0) {
      theta = atan2(con * p.x, con * p.y);
    }
    con = (rh1 * this.ns0) / this.a;
    if (this.sphere) {
      lat = asin((this.c - con * con) / (2 * this.ns0));
    } else {
      qs = (this.c - con * con) / this.ns0;
      lat = phi1z(this.e3, qs);
    }

    p.x = adjustLon(theta / this.ns0 + this.long0);
    p.y = lat;
  }
}

/**
 * Function to compute phi1, the latitude for the inverse of the
 * Albers Conical Equal-Area projection.
 * @param eccent - eccentricity
 * @param qs - qs
 * @returns - phi1
 */
export function phi1z(eccent: number, qs: number): number {
  const { abs, sin, cos, log } = Math;
  let sinphi, cosphi, con, com, dphi;
  let phi = asinz(0.5 * qs);
  if (eccent < EPSLN) {
    return phi;
  }

  const eccnts = eccent * eccent;
  for (let i = 1; i <= 25; i++) {
    sinphi = sin(phi);
    cosphi = cos(phi);
    con = eccent * sinphi;
    com = 1 - con * con;
    dphi =
      ((0.5 * com * com) / cosphi) *
      (qs / (1 - eccnts) - sinphi / com + (0.5 / eccent) * log((1 - con) / (1 + con)));
    phi = phi + dphi;
    if (abs(dphi) <= 1e-7) {
      return phi;
    }
  }

  throw new Error('PHI1Z-CONV:Latitude failed to converge after 25 iterations');
}
