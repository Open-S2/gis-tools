import { ProjectionBase } from '.';
import { D2R, EPSLN, HALF_PI, QUART_PI, TWO_PI } from '../constants';
import { adjustLon, phi2z, tsfnz } from '../common';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

const { abs, pow, sin, cos, sqrt, atan2, asin, log, atan, PI, tan, exp } = Math;

/**
 * # Oblique Mercator
 *
 * **Classification**: Conformal cylindrical
 *
 * **Available forms**: Forward and inverse, spherical and ellipsoidal
 *
 * **Defined area**: Global, but reasonably accurate only within 15 degrees of the oblique central line
 *
 * **Alias**: omerc
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=omerc +lat_1=45 +lat_2=55
 * ```
 *
 * ## Required Parameters
 * - `+lat_1=<value>`: Latitude of the first point on the central line.
 * - `+lat_2=<value>`: Latitude of the second point on the central line.
 *
 * ## Optional Parameters
 * - `+alpha=<value>`: Azimuth of the centerline clockwise from north at the center point of the line.
 * - `+gamma=<value>`: Azimuth of the centerline clockwise from north of the rectified bearing of the centerline.
 * - `+lonc=<value>`: Longitude of the projection center (overrides `+lon_0`).
 * - `+lat_0=<value>`: Latitude of the projection center.
 * - `+no_rot`: Disables rectification (historical reason).
 * - `+no_off`: Disables origin offset to the center of projection.
 * - `+k_0=<value>`: Scale factor at the central line.
 * - `+x_0=<value>`: False easting.
 * - `+y_0=<value>`: False northing.
 *
 * ## Usage Example
 * ```
 * echo 12 55 | proj +proj=omerc +alpha=90 +ellps=GRS80
 * echo 12 55 | proj +proj=omerc +alpha=0 +R=6400000
 * echo 12 55 | proj +proj=omerc +lon_1=0 +lat_1=-1 +lon_2=0 +lat_2=0 +R=6400000
 * echo 12 55 | proj +proj=tmerc +R=6400000
 * echo 12 55 | proj +proj=omerc +lon_1=-1 +lat_1=1 +lon_2=0 +lat_2=0 +ellps=GRS80
 * echo 10.536498003 56.229892362 | cs2cs +proj=longlat +ellps=GRS80 +to +proj=omerc +axis=wnu +lonc=9.46 +lat_0=56.13333333 +x_0=-266906.229 +y_0=189617.957 +k=0.9999537 +alpha=-0.76324 +gamma=0 +ellps=GRS80
 * ```
 *
 * ## Caveats
 * The two-point method with no rectification is probably only marginally useful.
 *
 * ![Oblique Mercator](https://github.com/Open-S2/s2-tools/blob/master/assets/proj4/projections/images/omerc.png?raw=true)
 */
export class HotineObliqueMercator extends ProjectionBase implements ProjectionTransform {
  name = 'HotineObliqueMercator';
  static names = [
    'HotineObliqueMercator',
    'Hotine_Oblique_Mercator',
    'Hotine Oblique Mercator',
    'Hotine_Oblique_Mercator_Azimuth_Natural_Origin',
    'Hotine_Oblique_Mercator_Two_Point_Natural_Origin',
    'Hotine_Oblique_Mercator_Azimuth_Center',
    'Oblique_Mercator',
    'omerc',
  ];
  // HotineObliqueMercator specific variables
  noOff: boolean;
  noRot: boolean;
  TOL = 1e-7;
  lam0: number;
  declare long2: number;
  declare longc: number;
  A = 0;
  B = 0;
  E = 0;
  singam: number;
  cosgam: number;
  sinrot: number;
  cosrot: number;
  rB: number;
  ArB: number;
  BrA: number;
  u0: number;
  vPoleN: number;
  vPoleS: number;

  /**
   * Preps an HotineObliqueMercator projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);
    const { TOL } = this;

    let con,
      cosph0,
      D,
      F,
      H,
      L,
      sinph0,
      p,
      J,
      gamma = 0,
      gamma0,
      lamc = 0,
      lam1 = 0,
      lam2 = 0,
      phi1 = 0,
      phi2 = 0,
      alphaC = 0;

    // only Type A uses the noOff or noUoff property
    // https://github.com/OSGeo/proj.4/issues/104
    this.noOff = isTypeA(params ?? {});
    this.noRot = params?.noRot ?? false;

    let alp = false;
    if ('alpha' in this) {
      alp = true;
    }

    let gam = false;
    if (this.rectifiedGridAngle !== undefined) {
      gam = true;
      gamma = this.rectifiedGridAngle * D2R;
    }

    if (alp) {
      alphaC = this.alpha ?? 0;
    }

    if (alp || gam) {
      lamc = this.longc;
    } else {
      lam1 = this.long1;
      phi1 = this.lat1;
      lam2 = this.long2;
      phi2 = this.lat2;

      if (
        abs(phi1 - phi2) <= TOL ||
        (con = abs(phi1)) <= TOL ||
        abs(con - HALF_PI) <= TOL ||
        abs(abs(this.lat0) - HALF_PI) <= TOL ||
        abs(abs(phi2) - HALF_PI) <= TOL
      ) {
        throw new Error();
      }
    }

    const one_es = 1.0 - this.es;
    const com = sqrt(one_es);

    if (abs(this.lat0) > EPSLN) {
      sinph0 = sin(this.lat0);
      cosph0 = cos(this.lat0);
      con = 1 - this.es * sinph0 * sinph0;
      this.B = cosph0 * cosph0;
      this.B = sqrt(1 + (this.es * this.B * this.B) / one_es);
      this.A = (this.B * this.k0 * com) / con;
      D = (this.B * com) / (cosph0 * sqrt(con));
      F = D * D - 1;

      if (F <= 0) {
        F = 0;
      } else {
        F = sqrt(F);
        if (this.lat0 < 0) {
          F = -F;
        }
      }

      this.E = F += D;
      this.E *= pow(tsfnz(this.e, this.lat0, sinph0), this.B);
    } else {
      this.B = 1 / com;
      this.A = this.k0;
      this.E = D = F = 1;
    }

    if (alp || gam) {
      if (alp) {
        gamma0 = asin(sin(alphaC) / D);
        if (!gam) {
          gamma = alphaC;
        }
      } else {
        gamma0 = gamma;
        alphaC = asin(D * sin(gamma0));
      }
      this.lam0 = lamc - asin(0.5 * (F - 1 / F) * tan(gamma0)) / this.B;
    } else {
      H = pow(tsfnz(this.e, phi1, sin(phi1)), this.B);
      L = pow(tsfnz(this.e, phi2, sin(phi2)), this.B);
      F = this.E / H;
      p = (L - H) / (L + H);
      J = this.E * this.E;
      J = (J - L * H) / (J + L * H);
      con = lam1 - lam2;

      if (con < -PI) {
        lam2 -= TWO_PI;
      } else if (con > PI) {
        lam2 += TWO_PI;
      }

      this.lam0 = adjustLon(
        0.5 * (lam1 + lam2) - atan((J * tan(0.5 * this.B * (lam1 - lam2))) / p) / this.B,
      );
      gamma0 = atan((2 * sin(this.B * adjustLon(lam1 - this.lam0))) / (F - 1 / F));
      gamma = alphaC = asin(D * sin(gamma0));
    }

    this.singam = sin(gamma0);
    this.cosgam = cos(gamma0);
    this.sinrot = sin(gamma);
    this.cosrot = cos(gamma);

    this.rB = 1 / this.B;
    this.ArB = this.A * this.rB;
    this.BrA = 1 / this.ArB;

    if (this.noOff) {
      this.u0 = 0;
    } else {
      this.u0 = abs(this.ArB * atan(sqrt(D * D - 1) / cos(alphaC)));

      if (this.lat0 < 0) {
        this.u0 = -this.u0;
      }
    }

    F = 0.5 * gamma0;
    this.vPoleN = this.ArB * log(tan(QUART_PI - F));
    this.vPoleS = this.ArB * log(tan(QUART_PI + F));
  }

  /**
   * HotineObliqueMercator forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    let S, T, U, V, W, temp, u, v;
    p.x = p.x - this.lam0;
    if (abs(abs(p.y) - HALF_PI) > EPSLN) {
      W = this.E / pow(tsfnz(this.e, p.y, sin(p.y)), this.B);
      temp = 1 / W;
      S = 0.5 * (W - temp);
      T = 0.5 * (W + temp);
      V = sin(this.B * p.x);
      U = (S * this.singam - V * this.cosgam) / T;
      if (abs(abs(U) - 1.0) < EPSLN) {
        throw new Error();
      }
      v = 0.5 * this.ArB * log((1 - U) / (1 + U));
      temp = cos(this.B * p.x);
      if (abs(temp) < this.TOL) {
        u = this.A * p.x;
      } else {
        u = this.ArB * atan2(S * this.cosgam + V * this.singam, temp);
      }
    } else {
      v = p.y > 0 ? this.vPoleN : this.vPoleS;
      u = this.ArB * p.y;
    }
    if (this.noRot) {
      p.x = u;
      p.y = v;
    } else {
      u -= this.u0;
      p.x = v * this.cosrot + u * this.sinrot;
      p.y = u * this.cosrot - v * this.sinrot;
    }
    p.x = this.a * p.x + this.x0;
    p.y = this.a * p.y + this.y0;
  }

  /**
   * HotineObliqueMercator inverse equations--mapping x-y to lon-lat
   * @param p - HotineObliqueMercator point
   */
  inverse(p: VectorPoint): void {
    let u, v;
    p.x = (p.x - this.x0) * (1.0 / this.a);
    p.y = (p.y - this.y0) * (1.0 / this.a);
    if (this.noRot) {
      v = p.y;
      u = p.x;
    } else {
      v = p.x * this.cosrot - p.y * this.sinrot;
      u = p.y * this.cosrot + p.x * this.sinrot + this.u0;
    }
    const Qp = exp(-this.BrA * v);
    const Sp = 0.5 * (Qp - 1 / Qp);
    const Tp = 0.5 * (Qp + 1 / Qp);
    const Vp = sin(this.BrA * u);
    const Up = (Vp * this.cosgam + Sp * this.singam) / Tp;
    if (abs(abs(Up) - 1) < EPSLN) {
      p.x = 0;
      p.y = Up < 0 ? -HALF_PI : HALF_PI;
    } else {
      p.y = this.E / sqrt((1 + Up) / (1 - Up));
      p.y = phi2z(this.e, pow(p.y, 1 / this.B));
      if (p.y === Infinity) {
        throw new Error();
      }
      p.x = -this.rB * atan2(Sp * this.cosgam - Vp * this.singam, cos(this.BrA * u));
    }
    p.x += this.lam0;
  }
}

/**
 * @param P - projection parameters
 * @returns - true if projection is of type A
 */
function isTypeA(P: ProjectionParams): boolean {
  const typeAProjections = [
    'Hotine_Oblique_Mercator',
    'Hotine_Oblique_Mercator_Azimuth_Natural_Origin',
  ];

  return (
    'noUoff' in P ||
    'noOff' in P ||
    typeAProjections.indexOf(P.PROJECTION ?? P.name ?? 'XXXXXX') !== -1
  );
}
