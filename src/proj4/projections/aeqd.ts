import { ProjectionBase } from './base';
import { EPSLN, HALF_PI } from '../constants';
import { adjustLon, asinz, e0fn, e1fn, e2fn, e3fn, gN, imlfn, mlfn } from '../common';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

const { abs, pow, sin, cos, sqrt, atan2, asin, acos, PI, tan, atan } = Math;

/**
 * # Azimuthal Equidistant Projection
 *
 * **Classification**: Azimuthal
 *
 * **Available forms**: Forward and inverse, spherical and ellipsoidal
 *
 * **Defined area**: Global
 *
 * **Alias**: `aeqd`
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=aeqd
 * ```
 *
 * ## Required Parameters
 * None
 *
 * ## Optional Parameters
 * - `guam`: Use Guam ellipsoidal formulas (accurate near Guam: λ ≈ 144.5°, φ ≈ 13.5°)
 * - `lat0`: Latitude of origin
 * - `lon0`: Longitude of origin
 * - `x0`: False easting
 * - `y0`: False northing
 * - `ellps`: Ellipsoid name
 * - `R`: Radius of sphere
 *
 * ![Azimuthal Equidistant Projection](https://github.com/OSGeo/PROJ/blob/38dd7c2446f3500a43f0257f5a4833d6aa5aab0b/docs/source/operations/projections/images/aeqd.png?raw=true)
 */
export class AzimuthalEquidistant extends ProjectionBase implements ProjectionTransform {
  name = 'Azimuthal_Equidistant';
  static names = ['Azimuthal_Equidistant', 'aeqd'];
  // AzimuthalEquidistant specific variables
  sinP12 = 0;
  cosP12 = 0;

  /**
   * Preps an Albers Conic Equal Area projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);
    this.sinP12 = sin(this.lat0);
    this.cosP12 = cos(this.lat0);
  }

  /**
   * Azimuthal Equidistant forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const lon = p.x;
    const lat = p.y;
    const sinphi = sin(p.y);
    const cosphi = cos(p.y);
    const dlon = adjustLon(lon - this.long0);
    let e0,
      e1,
      e2,
      e3,
      Mlp,
      Ml,
      tanphi,
      Nl1,
      Nl,
      psi,
      Az,
      G,
      H,
      GH,
      Hs,
      c,
      kp,
      cos_c,
      s,
      s2,
      s3,
      s4,
      s5;
    if (this.sphere) {
      if (abs(this.sinP12 - 1) <= EPSLN) {
        //North Pole case
        p.x = this.x0 + this.a * (HALF_PI - lat) * sin(dlon);
        p.y = this.y0 - this.a * (HALF_PI - lat) * cos(dlon);
        return;
      } else if (abs(this.sinP12 + 1) <= EPSLN) {
        //South Pole case
        p.x = this.x0 + this.a * (HALF_PI + lat) * sin(dlon);
        p.y = this.y0 + this.a * (HALF_PI + lat) * cos(dlon);
        return;
      } else {
        //default case
        cos_c = this.sinP12 * sinphi + this.cosP12 * cosphi * cos(dlon);
        c = acos(cos_c);
        kp = c !== 0 ? c / sin(c) : 1;
        p.x = this.x0 + this.a * kp * cosphi * sin(dlon);
        p.y = this.y0 + this.a * kp * (this.cosP12 * sinphi - this.sinP12 * cosphi * cos(dlon));
        return;
      }
    } else {
      e0 = e0fn(this.es);
      e1 = e1fn(this.es);
      e2 = e2fn(this.es);
      e3 = e3fn(this.es);
      if (abs(this.sinP12 - 1) <= EPSLN) {
        //North Pole case
        Mlp = this.a * mlfn(e0, e1, e2, e3, HALF_PI);
        Ml = this.a * mlfn(e0, e1, e2, e3, lat);
        p.x = this.x0 + (Mlp - Ml) * sin(dlon);
        p.y = this.y0 - (Mlp - Ml) * cos(dlon);
        return;
      } else if (abs(this.sinP12 + 1) <= EPSLN) {
        //South Pole case
        Mlp = this.a * mlfn(e0, e1, e2, e3, HALF_PI);
        Ml = this.a * mlfn(e0, e1, e2, e3, lat);
        p.x = this.x0 + (Mlp + Ml) * sin(dlon);
        p.y = this.y0 + (Mlp + Ml) * cos(dlon);
        return;
      } else {
        //Default case
        tanphi = sinphi / cosphi;
        Nl1 = gN(this.a, this.e, this.sinP12);
        Nl = gN(this.a, this.e, sinphi);
        psi = atan((1 - this.es) * tanphi + (this.es * Nl1 * this.sinP12) / (Nl * cosphi));
        Az = atan2(sin(dlon), this.cosP12 * tan(psi) - this.sinP12 * cos(dlon));
        if (Az === 0) {
          s = asin(this.cosP12 * sin(psi) - this.sinP12 * cos(psi));
        } else if (abs(abs(Az) - PI) <= EPSLN) {
          s = -asin(this.cosP12 * sin(psi) - this.sinP12 * cos(psi));
        } else {
          s = asin((sin(dlon) * cos(psi)) / sin(Az));
        }
        G = (this.e * this.sinP12) / sqrt(1 - this.es);
        H = (this.e * this.cosP12 * cos(Az)) / sqrt(1 - this.es);
        GH = G * H;
        Hs = H * H;
        s2 = s * s;
        s3 = s2 * s;
        s4 = s3 * s;
        s5 = s4 * s;
        c =
          Nl1 *
          s *
          (1 -
            (s2 * Hs * (1 - Hs)) / 6 +
            (s3 / 8) * GH * (1 - 2 * Hs) +
            (s4 / 120) * (Hs * (4 - 7 * Hs) - 3 * G * G * (1 - 7 * Hs)) -
            (s5 / 48) * GH);
        p.x = this.x0 + c * sin(Az);
        p.y = this.y0 + c * cos(Az);
        return;
      }
    }
  }

  /**
   * Azimuthal Equidistant inverse equations--mapping x-y to lon-lat
   * @param p - Azimuthal Equidistant point
   */
  inverse(p: VectorPoint): void {
    p.x -= this.x0;
    p.y -= this.y0;
    let rh,
      z,
      sinz,
      cosz,
      lon,
      lat,
      con,
      e0,
      e1,
      e2,
      e3,
      Mlp,
      M,
      N1,
      psi,
      Az,
      cosAz,
      tmp,
      A,
      B,
      D,
      Ee,
      F,
      sinpsi;
    if (this.sphere) {
      rh = sqrt(p.x * p.x + p.y * p.y);
      if (rh > 2 * HALF_PI * this.a) {
        throw Error('Point is not in the projection');
      }
      z = rh / this.a;

      sinz = sin(z);
      cosz = cos(z);

      lon = this.long0;
      if (abs(rh) <= EPSLN) {
        lat = this.lat0;
      } else {
        lat = asinz(cosz * this.sinP12 + (p.y * sinz * this.cosP12) / rh);
        con = abs(this.lat0) - HALF_PI;
        if (abs(con) <= EPSLN) {
          if (this.lat0 >= 0) {
            lon = adjustLon(this.long0 + atan2(p.x, -p.y));
          } else {
            lon = adjustLon(this.long0 - atan2(-p.x, p.y));
          }
        } else {
          /*con = cosz - this.sinP12 * sin(lat);
        if ((abs(con) < EPSLN) && (abs(p.x) < EPSLN)) {
          //no-op, just keep the lon value as is
        } else {
          var temp = atan2((p.x * sinz * this.cosP12), (con * rh));
          lon = adjustLon(this.long0 + atan2((p.x * sinz * this.cosP12), (con * rh)));
        }*/
          lon = adjustLon(
            this.long0 + atan2(p.x * sinz, rh * this.cosP12 * cosz - p.y * this.sinP12 * sinz),
          );
        }
      }

      p.x = lon;
      p.y = lat;
      return;
    } else {
      e0 = e0fn(this.es);
      e1 = e1fn(this.es);
      e2 = e2fn(this.es);
      e3 = e3fn(this.es);
      if (abs(this.sinP12 - 1) <= EPSLN) {
        //North pole case
        Mlp = this.a * mlfn(e0, e1, e2, e3, HALF_PI);
        rh = sqrt(p.x * p.x + p.y * p.y);
        M = Mlp - rh;
        lat = imlfn(M / this.a, e0, e1, e2, e3);
        lon = adjustLon(this.long0 + atan2(p.x, -1 * p.y));
        p.x = lon;
        p.y = lat;
        return;
      } else if (abs(this.sinP12 + 1) <= EPSLN) {
        //South pole case
        Mlp = this.a * mlfn(e0, e1, e2, e3, HALF_PI);
        rh = sqrt(p.x * p.x + p.y * p.y);
        M = rh - Mlp;

        lat = imlfn(M / this.a, e0, e1, e2, e3);
        lon = adjustLon(this.long0 + atan2(p.x, p.y));
        p.x = lon;
        p.y = lat;
        return;
      } else {
        //default case
        rh = sqrt(p.x * p.x + p.y * p.y);
        Az = atan2(p.x, p.y);
        N1 = gN(this.a, this.e, this.sinP12);
        cosAz = cos(Az);
        tmp = this.e * this.cosP12 * cosAz;
        A = (-tmp * tmp) / (1 - this.es);
        B = (3 * this.es * (1 - A) * this.sinP12 * this.cosP12 * cosAz) / (1 - this.es);
        D = rh / N1;
        Ee = D - (A * (1 + A) * pow(D, 3)) / 6 - (B * (1 + 3 * A) * pow(D, 4)) / 24;
        F = 1 - (A * Ee * Ee) / 2 - (D * Ee * Ee * Ee) / 6;
        psi = asin(this.sinP12 * cos(Ee) + this.cosP12 * sin(Ee) * cosAz);
        lon = adjustLon(this.long0 + asin((sin(Az) * sin(Ee)) / cos(psi)));
        sinpsi = sin(psi);
        lat = atan2((sinpsi - this.es * F * this.sinP12) * tan(psi), sinpsi * (1 - this.es));
        p.x = lon;
        p.y = lat;
      }
    }
  }
}
