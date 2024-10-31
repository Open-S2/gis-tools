import { ProjectionBase } from '.';
import { EPSLN, HALF_PI, QUART_PI } from '../constants';
import { adjustLon, qsfnz } from '../common';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

export const S_POLE = 1;

export const N_POLE = 2;
export const EQUIT = 3;
export const OBLIQ = 4;

// /* determine latitude from authalic latitude */
const P00 = 0.3333333333333333;
const P01 = 0.17222222222222222;
const P02 = 0.1025793650793651;
const P10 = 0.06388888888888888;
const P11 = 0.0664021164021164;
const P20 = 0.01641501294219154;

const { abs, sin, cos, sqrt, atan2, asin } = Math;

/** Lambert Azimuthal Equal Area projection parameters */
export type APA = [number, number, number];

/**
 * # Lambert Azimuthal Equal Area
 *
 * **Classification**: Azimuthal
 *
 * **Available forms**: Forward and inverse, spherical and ellipsoidal
 *
 * **Defined area**: Global
 *
 * **Alias**: laea
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=laea
 * ```
 *
 * ## Required Parameters
 * - None, all parameters are optional for this projection.
 *
 * ## Optional Parameters
 * - `+lon_0`: Longitude of projection center. Defaults to `0`.
 * - `+lat_0`: Latitude of projection center. Defaults to `0`.
 * - `+ellps`: Ellipsoid. Defaults to `WGS84`.
 * - `+R`: Radius of the sphere.
 * - `+x_0`: False easting. Defaults to `0`.
 * - `+y_0`: False northing. Defaults to `0`.
 *
 * Reference
 * "New Equal-Area Map Projections for Noncircular Regions", John P. Snyder,
 * The American Cartographer, Vol 15, No. 4, October 1988, pp. 341-355.
 *
 * ![Lambert Azimuthal Equal Area](https://github.com/Open-S2/s2-tools/blob/master/assets/proj4/projections/images/laea.png?raw=true)
 */
export class LambertAzimuthalEqualArea extends ProjectionBase implements ProjectionTransform {
  name = 'LambertAzimuthalEqualArea';
  static names = [
    'LambertAzimuthalEqualArea',
    'Lambert Azimuthal Equal Area',
    'Lambert_Azimuthal_Equal_Area',
    'laea',
  ];
  // LambertAzimuthalEqualArea specific variables
  mode: number;
  S_POLE = S_POLE;
  N_POLE = N_POLE;
  EQUIT = EQUIT;
  OBLIQ = OBLIQ;
  qp = 0;
  mmf = 0;
  apa: APA = [0, 0, 0];
  dd = 0;
  rq = 0;
  xmf = 0;
  ymf = 0;
  sinb1 = 0;
  cosb1 = 0;
  sinph0 = 0;
  cosph0 = 0;

  /**
   * Preps an LambertAzimuthalEqualArea projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);

    const t = abs(this.lat0);
    if (abs(t - HALF_PI) < EPSLN) {
      this.mode = this.lat0 < 0 ? this.S_POLE : this.N_POLE;
    } else if (abs(t) < EPSLN) {
      this.mode = this.EQUIT;
    } else {
      this.mode = this.OBLIQ;
    }
    if (this.es > 0) {
      let sinphi;

      this.qp = qsfnz(this.e, 1);
      this.mmf = 0.5 / (1 - this.es);
      this.apa = authset(this.es);
      switch (this.mode) {
        case this.N_POLE:
          this.dd = 1;
          break;
        case this.S_POLE:
          this.dd = 1;
          break;
        case this.EQUIT:
          this.rq = sqrt(0.5 * this.qp);
          this.dd = 1 / this.rq;
          this.xmf = 1;
          this.ymf = 0.5 * this.qp;
          break;
        case this.OBLIQ:
          this.rq = sqrt(0.5 * this.qp);
          sinphi = sin(this.lat0);
          this.sinb1 = qsfnz(this.e, sinphi) / this.qp;
          this.cosb1 = sqrt(1 - this.sinb1 * this.sinb1);
          this.dd = cos(this.lat0) / (sqrt(1 - this.es * sinphi * sinphi) * this.rq * this.cosb1);
          this.ymf = (this.xmf = this.rq) / this.dd;
          this.xmf *= this.dd;
          break;
      }
    } else {
      if (this.mode === this.OBLIQ) {
        this.sinph0 = sin(this.lat0);
        this.cosph0 = cos(this.lat0);
      }
    }
  }

  /**
   * LambertAzimuthalEqualArea forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    let x, y, coslam, sinlam, sinphi, q, sinb, cosb, b, cosphi;
    let lam = p.x;
    const phi = p.y;

    lam = adjustLon(lam - this.long0);
    if (this.sphere) {
      sinphi = sin(phi);
      cosphi = cos(phi);
      coslam = cos(lam);
      if (this.mode === this.OBLIQ || this.mode === this.EQUIT) {
        y =
          this.mode === this.EQUIT
            ? 1 + cosphi * coslam
            : 1 + this.sinph0 * sinphi + this.cosph0 * cosphi * coslam;
        if (y <= EPSLN) {
          throw new Error('Invalid point');
        }
        y = sqrt(2 / y);
        x = y * cosphi * sin(lam);
        y *=
          this.mode === this.EQUIT ? sinphi : this.cosph0 * sinphi - this.sinph0 * cosphi * coslam;
      } else if (this.mode === this.N_POLE || this.mode === this.S_POLE) {
        if (this.mode === this.N_POLE) {
          coslam = -coslam;
        }
        if (abs(phi + this.lat0) < EPSLN) {
          throw new Error('Invalid point');
        }
        y = QUART_PI - phi * 0.5;
        y = 2 * (this.mode === this.S_POLE ? cos(y) : sin(y));
        x = y * sin(lam);
        y *= coslam;
      }
    } else {
      sinb = 0;
      cosb = 0;
      b = 0;
      coslam = cos(lam);
      sinlam = sin(lam);
      sinphi = sin(phi);
      q = qsfnz(this.e, sinphi);
      if (this.mode === this.OBLIQ || this.mode === this.EQUIT) {
        sinb = q / this.qp;
        cosb = sqrt(1 - sinb * sinb);
      }
      switch (this.mode) {
        case this.OBLIQ:
          b = 1 + this.sinb1 * sinb + this.cosb1 * cosb * coslam;
          break;
        case this.EQUIT:
          b = 1 + cosb * coslam;
          break;
        case this.N_POLE:
          b = HALF_PI + phi;
          q = this.qp - q;
          break;
        case this.S_POLE:
          b = phi - HALF_PI;
          q = this.qp + q;
          break;
      }
      if (abs(b) < EPSLN) {
        throw new Error('Invalid point');
      }
      switch (this.mode) {
        case this.OBLIQ:
        case this.EQUIT:
          b = sqrt(2 / b);
          if (this.mode === this.OBLIQ) {
            y = this.ymf * b * (this.cosb1 * sinb - this.sinb1 * cosb * coslam);
          } else {
            y = (b = sqrt(2 / (1 + cosb * coslam))) * sinb * this.ymf;
          }
          x = this.xmf * b * cosb * sinlam;
          break;
        case this.N_POLE:
        case this.S_POLE:
          if (q >= 0) {
            x = (b = sqrt(q)) * sinlam;
            y = coslam * (this.mode === this.S_POLE ? b : -b);
          } else {
            x = y = 0;
          }
          break;
      }
    }

    if (x === undefined || y === undefined) {
      throw new Error('Invalid point');
    }
    p.x = this.a * x + this.x0;
    p.y = this.a * y + this.y0;
  }

  /**
   * LambertAzimuthalEqualArea inverse equations--mapping x-y to lon-lat
   * @param p - LambertAzimuthalEqualArea point
   */
  inverse(p: VectorPoint): void {
    p.x -= this.x0;
    p.y -= this.y0;
    let x = p.x / this.a;
    let y = p.y / this.a;
    let lam, phi, cCe, sCe, q, rho, ab;
    if (this.sphere) {
      let cosz = 0,
        sinz = 0;
      const rh = sqrt(x * x + y * y);
      phi = rh * 0.5;
      if (phi > 1) {
        throw new Error('Point is outside the sphere');
      }
      phi = 2 * asin(phi);
      if (this.mode === this.OBLIQ || this.mode === this.EQUIT) {
        sinz = sin(phi);
        cosz = cos(phi);
      }
      switch (this.mode) {
        case this.EQUIT:
          phi = abs(rh) <= EPSLN ? 0 : asin((y * sinz) / rh);
          x *= sinz;
          y = cosz * rh;
          break;
        case this.OBLIQ:
          phi =
            abs(rh) <= EPSLN ? this.lat0 : asin(cosz * this.sinph0 + (y * sinz * this.cosph0) / rh);
          x *= sinz * this.cosph0;
          y = (cosz - sin(phi) * this.sinph0) * rh;
          break;
        case this.N_POLE:
          y = -y;
          phi = HALF_PI - phi;
          break;
        case this.S_POLE:
          phi -= HALF_PI;
          break;
      }
      lam = y === 0 && (this.mode === this.EQUIT || this.mode === this.OBLIQ) ? 0 : atan2(x, y);
    } else {
      ab = 0;
      if (this.mode === this.OBLIQ || this.mode === this.EQUIT) {
        x /= this.dd;
        y *= this.dd;
        rho = sqrt(x * x + y * y);
        if (rho < EPSLN) {
          p.x = this.long0;
          p.y = this.lat0;
          return;
        }
        sCe = 2 * asin((0.5 * rho) / this.rq);
        cCe = cos(sCe);
        x *= sCe = sin(sCe);
        if (this.mode === this.OBLIQ) {
          ab = cCe * this.sinb1 + (y * sCe * this.cosb1) / rho;
          q = this.qp * ab;
          y = rho * this.cosb1 * cCe - y * this.sinb1 * sCe;
        } else {
          ab = (y * sCe) / rho;
          q = this.qp * ab;
          y = rho * cCe;
        }
      } else if (this.mode === this.N_POLE || this.mode === this.S_POLE) {
        if (this.mode === this.N_POLE) {
          y = -y;
        }
        q = x * x + y * y;
        if (q === 0) {
          p.x = this.long0;
          p.y = this.lat0;
          return;
        }
        ab = 1 - q / this.qp;
        if (this.mode === this.S_POLE) {
          ab = -ab;
        }
      }
      lam = atan2(x, y);
      phi = authlat(asin(ab), this.apa);
    }
    p.x = adjustLon(this.long0 + lam);
    p.y = phi;
  }
}

/**
 * @param es - eccentricity
 * @returns [APA0, APA1, APA2]
 */
function authset(es: number): APA {
  let t;
  const APA: APA = [0, 0, 0];
  APA[0] = es * P00;
  t = es * es;
  APA[0] += t * P01;
  APA[1] = t * P10;
  t *= es;
  APA[0] += t * P02;
  APA[1] += t * P11;
  APA[2] = t * P20;

  return APA;
}

/**
 * @param beta - geodetic latitude
 * @param APA - [APA0, APA1, APA2]
 * @returns authalic latitude
 */
function authlat(beta: number, APA: APA): number {
  const t = beta + beta;
  return beta + APA[0] * sin(t) + APA[1] * sin(t + t) + APA[2] * sin(t + t + t);
}
