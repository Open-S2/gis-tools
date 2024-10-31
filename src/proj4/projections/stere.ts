import { ProjectionBase } from '.';
import { EPSLN, HALF_PI } from '../constants';
import { adjustLon, msfnz, phi2z, sign, tsfnz } from '../common';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

const { abs, pow, sin, cos, sqrt, atan2, asin, tan, atan, PI } = Math;

/**
 * # Stereographic
 *
 * **Classification**: Azimuthal
 *
 * **Available forms**: Forward and inverse, spherical and ellipsoidal
 *
 * **Defined area**: Global
 *
 * **Alias**: stere
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=stere +lat_0=90 +latTs=75
 * ```
 *
 * Note:
 * This projection method gives different results than the :ref:`sterea`
 * method in the non-polar cases (i.e. the oblique and equatorial case). The later
 * projection method is the one referenced by EPSG as "Oblique Stereographic".
 *
 * ## Required Parameters
 * - None
 *
 * ## Optional Parameters
 * - `+lat_0=<value>`: Latitude of origin.
 * - `+latTs=<value>`: Latitude where scale is not distorted.
 * - `+k_0=<value>`: Scale factor.
 * - `+lon_0=<value>`: Central meridian.
 * - `+ellps=<value>`: Ellipsoid used.
 * - `+R=<value>`: Radius of the projection sphere.
 * - `+x_0=<value>`: False easting.
 * - `+y_0=<value>`: False northing.
 *
 * ![Stereographic](https://github.com/Open-S2/s2-tools/blob/master/assets/proj4/projections/images/stere.png?raw=true)
 */
export class StereographicSouthPole extends ProjectionBase implements ProjectionTransform {
  name = 'StereographicSouthPole';
  static names = [
    'StereographicSouthPole',
    'stere',
    'Stereographic_South_Pole',
    'Polar Stereographic (variant B)',
    'Polar_Stereographic',
  ];
  // StereographicSouthPole specific variables
  coslat0: number;
  sinlat0: number;
  con = 0;
  cons = 0;
  ms1 = 0;
  X0 = 0;
  cosX0 = 0;
  sinX0 = 0;

  /**
   * Preps an StereographicSouthPole projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);

    this.x0 = this.x0 ?? 0;
    this.y0 = this.y0 ?? 0;
    this.lat0 = this.lat0 ?? 0;
    this.long0 = this.long0 ?? 0;

    this.coslat0 = cos(this.lat0);
    this.sinlat0 = sin(this.lat0);
    if (this.sphere) {
      if (this.k0 === 1 && this.latTs !== undefined && abs(this.coslat0) <= EPSLN) {
        this.k0 = 0.5 * (1 + sign(this.lat0) * sin(this.latTs));
      }
    } else {
      if (abs(this.coslat0) <= EPSLN) {
        if (this.lat0 > 0) {
          //North pole
          //trace('stere:north pole');
          this.con = 1;
        } else {
          //South pole
          //trace('stere:south pole');
          this.con = -1;
        }
      }
      this.cons = sqrt(pow(1 + this.e, 1 + this.e) * pow(1 - this.e, 1 - this.e));
      if (
        this.k0 === 1 &&
        this.latTs !== undefined &&
        abs(this.coslat0) <= EPSLN &&
        abs(cos(this.latTs)) > EPSLN
      ) {
        // When k0 is 1 (default value) and latTs is a vaild number and lat0 is at a pole and latTs is not at a pole
        // Recalculate k0 using formula 21-35 from p161 of Snyder, 1987
        this.k0 =
          (0.5 * this.cons * msfnz(this.e, sin(this.latTs), cos(this.latTs))) /
          tsfnz(this.e, this.con * this.latTs, this.con * sin(this.latTs));
      }
      this.ms1 = msfnz(this.e, this.sinlat0, this.coslat0);
      this.X0 = 2 * atan(this.#ssfn(this.lat0, this.sinlat0, this.e)) - HALF_PI;
      this.cosX0 = cos(this.X0);
      this.sinX0 = sin(this.X0);
    }
  }

  /**
   * StereographicSouthPole forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const { x: lon, y: lat } = p;
    const sinlat = sin(lat);
    const coslat = cos(lat);
    let A, X, sinX, cosX, ts, rh;
    const dlon = adjustLon(lon - this.long0);
    if (abs(abs(lon - this.long0) - PI) <= EPSLN && abs(lat + this.lat0) <= EPSLN) {
      //case of the origine point
      throw new Error('cass:pj_init_stere: lon == long0 == 180');
    }
    if (this.sphere) {
      //trace('stere:sphere case');
      A = (2 * this.k0) / (1 + this.sinlat0 * sinlat + this.coslat0 * coslat * cos(dlon));
      p.x = this.a * A * coslat * sin(dlon) + this.x0;
      p.y = this.a * A * (this.coslat0 * sinlat - this.sinlat0 * coslat * cos(dlon)) + this.y0;
      return;
    } else {
      X = 2 * atan(this.#ssfn(lat, sinlat, this.e)) - HALF_PI;
      cosX = cos(X);
      sinX = sin(X);
      if (abs(this.coslat0) <= EPSLN) {
        ts = tsfnz(this.e, lat * this.con, this.con * sinlat);
        rh = (2 * this.a * this.k0 * ts) / this.cons;
        p.x = this.x0 + rh * sin(lon - this.long0);
        p.y = this.y0 - this.con * rh * cos(lon - this.long0);
        //trace(p.toString());
        return;
      } else if (abs(this.sinlat0) < EPSLN) {
        //Eq
        //trace('stere:equateur');
        A = (2 * this.a * this.k0) / (1 + cosX * cos(dlon));
        p.y = A * sinX;
      } else {
        //other case
        //trace('stere:normal case');
        A =
          (2 * this.a * this.k0 * this.ms1) /
          (this.cosX0 * (1 + this.sinX0 * sinX + this.cosX0 * cosX * cos(dlon)));
        p.y = A * (this.cosX0 * sinX - this.sinX0 * cosX * cos(dlon)) + this.y0;
      }
      p.x = A * cosX * sin(dlon) + this.x0;
    }
  }

  /**
   * StereographicSouthPole inverse equations--mapping x-y to lon-lat
   * @param p - StereographicSouthPole point
   */
  inverse(p: VectorPoint): void {
    p.x -= this.x0;
    p.y -= this.y0;
    let lon, lat, ts, ce, Chi;
    const rh = sqrt(p.x * p.x + p.y * p.y);
    if (this.sphere) {
      const c = 2 * atan(rh / (2 * this.a * this.k0));
      lon = this.long0;
      lat = this.lat0;
      if (rh <= EPSLN) {
        p.x = lon;
        p.y = lat;
        return;
      }
      lat = asin(cos(c) * this.sinlat0 + (p.y * sin(c) * this.coslat0) / rh);
      if (abs(this.coslat0) < EPSLN) {
        if (this.lat0 > 0) {
          lon = adjustLon(this.long0 + atan2(p.x, -1 * p.y));
        } else {
          lon = adjustLon(this.long0 + atan2(p.x, p.y));
        }
      } else {
        lon = adjustLon(
          this.long0 +
            atan2(p.x * sin(c), rh * this.coslat0 * cos(c) - p.y * this.sinlat0 * sin(c)),
        );
      }
      p.x = lon;
      p.y = lat;
      return;
    } else {
      if (abs(this.coslat0) <= EPSLN) {
        if (rh <= EPSLN) {
          lat = this.lat0;
          lon = this.long0;
          p.x = lon;
          p.y = lat;
          //trace(p.toString());
          return;
        }
        p.x *= this.con;
        p.y *= this.con;
        ts = (rh * this.cons) / (2 * this.a * this.k0);
        lat = this.con * phi2z(this.e, ts);
        lon = this.con * adjustLon(this.con * this.long0 + atan2(p.x, -1 * p.y));
      } else {
        ce = 2 * atan((rh * this.cosX0) / (2 * this.a * this.k0 * this.ms1));
        lon = this.long0;
        if (rh <= EPSLN) {
          Chi = this.X0;
        } else {
          Chi = asin(cos(ce) * this.sinX0 + (p.y * sin(ce) * this.cosX0) / rh);
          lon = adjustLon(
            this.long0 +
              atan2(p.x * sin(ce), rh * this.cosX0 * cos(ce) - p.y * this.sinX0 * sin(ce)),
          );
        }
        lat = -1 * phi2z(this.e, tan(0.5 * (HALF_PI + Chi)));
      }
    }
    p.x = lon;
    p.y = lat;
  }

  /**
   * @param phit - phi
   * @param sinphi - sin(phi)
   * @param eccen - eccentricity
   * @returns - tan(0.5*(HALF_PI+phit))
   */
  #ssfn(phit: number, sinphi: number, eccen: number): number {
    sinphi *= eccen;
    return tan(0.5 * (HALF_PI + phit)) * pow((1 - sinphi) / (1 + sinphi), 0.5 * eccen);
  }
}
