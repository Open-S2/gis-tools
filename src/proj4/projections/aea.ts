import { EPSLN } from '../constants';
import { ProjectionTransform } from '.';
import { adjustLon, asinz, msfnz, qsfnz } from '../common';

import type { VectorPoint } from 's2json-spec';

/**
 *
 */
export class AlbersConicEqualArea extends ProjectionTransform {
  name = 'Albers_Conic_Equal_Area';
  names = [this.name, 'Albers', 'aea'];
  // AlbersConicEqualArea specific variables
  ns0: number;
  temp: number;
  e3: number;
  sin_po: number;
  cos_po: number;
  t1: number;
  con: number;
  ms1: number;
  qs1: number;
  t2: number;
  ms2: number;
  qs2: number;
  t3: number;
  qs0: number;
  c: number;
  rh: number;

  /**
   * @param srsCode
   */
  constructor(srsCode?: string) {
    super(srsCode);
    if (Math.abs(this.lat1 + this.lat2) < EPSLN) {
      throw new Error('cass:pj_init_aea: lat1 == lat2');
    }
    this.temp = this.b / this.a;
    this.es = 1 - Math.pow(this.temp, 2);
    this.e3 = Math.sqrt(this.es);

    this.sin_po = Math.sin(this.lat1);
    this.cos_po = Math.cos(this.lat1);
    this.t1 = this.sin_po;
    this.con = this.sin_po;
    this.ms1 = msfnz(this.e3, this.sin_po, this.cos_po);
    this.qs1 = qsfnz(this.e3, this.sin_po);

    this.sin_po = Math.sin(this.lat2);
    this.cos_po = Math.cos(this.lat2);
    this.t2 = this.sin_po;
    this.ms2 = msfnz(this.e3, this.sin_po, this.cos_po);
    this.qs2 = qsfnz(this.e3, this.sin_po);

    this.sin_po = Math.sin(this.lat0);
    this.cos_po = Math.cos(this.lat0);
    this.t3 = this.sin_po;
    this.qs0 = qsfnz(this.e3, this.sin_po);

    if (Math.abs(this.lat1 - this.lat2) > EPSLN) {
      this.ns0 = (this.ms1 * this.ms1 - this.ms2 * this.ms2) / (this.qs2 - this.qs1);
    } else {
      this.ns0 = this.con;
    }
    this.c = this.ms1 * this.ms1 + this.ns0 * this.qs1;
    this.rh = (this.a * Math.sqrt(this.c - this.ns0 * this.qs0)) / this.ns0;
  }

  /**
   * Albers Conical Equal Area forward equations--mapping lat,long to x,y
   * @param p - lon-lat WGS84 point
   * @returns - an Albers Conic Equal Area point
   */
  forward(p: VectorPoint): VectorPoint {
    const lon = p.x;
    const lat = p.y;

    const sin_phi = Math.sin(lat);

    const qs = qsfnz(this.e3, sin_phi);
    const rh1 = (this.a * Math.sqrt(this.c - this.ns0 * qs)) / this.ns0;
    const theta = this.ns0 * adjustLon(lon - this.long0);
    const x = rh1 * Math.sin(theta) + this.x0;
    const y = this.rh - rh1 * Math.cos(theta) + this.y0;

    p.x = x;
    p.y = y;
    return p;
  }

  /**
   * @param p - Albers Conic Equal Area point
   * @returns - lon-lat WGS84 point
   */
  inverse(p: VectorPoint): VectorPoint {
    let rh1, qs, con, theta;
    let lat = 0;

    p.x -= this.x0;
    p.y = this.rh - p.y + this.y0;
    if (this.ns0 >= 0) {
      rh1 = Math.sqrt(p.x * p.x + p.y * p.y);
      con = 1;
    } else {
      rh1 = -Math.sqrt(p.x * p.x + p.y * p.y);
      con = -1;
    }
    theta = 0;
    if (rh1 !== 0) {
      theta = Math.atan2(con * p.x, con * p.y);
    }
    con = (rh1 * this.ns0) / this.a;
    if (this.sphere) {
      lat = Math.asin((this.c - con * con) / (2 * this.ns0));
    } else {
      qs = (this.c - con * con) / this.ns0;
      lat = phi1z(this.e3, qs);
    }

    p.x = adjustLon(theta / this.ns0 + this.long0);
    p.y = lat;
    return p;
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
  let sinphi, cosphi, con, com, dphi;
  let phi = asinz(0.5 * qs);
  if (eccent < EPSLN) {
    return phi;
  }

  const eccnts = eccent * eccent;
  for (let i = 1; i <= 25; i++) {
    sinphi = Math.sin(phi);
    cosphi = Math.cos(phi);
    con = eccent * sinphi;
    com = 1 - con * con;
    dphi =
      ((0.5 * com * com) / cosphi) *
      (qs / (1 - eccnts) - sinphi / com + (0.5 / eccent) * Math.log((1 - con) / (1 + con)));
    phi = phi + dphi;
    if (Math.abs(dphi) <= 1e-7) {
      return phi;
    }
  }

  throw new Error('PHI1Z-CONV:Latitude failed to converge after 25 iterations');
}
