import { EPSLN } from '../constants';
import { ProjectionBase } from '.';
import { adjustLat, adjustLon, e0fn, e1fn, e2fn, e3fn, gN, mlfn } from '../common';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

const { abs, pow, sin, cos, sqrt, asin, tan } = Math;

/**
 * # Polyconic (American)
 *
 * **Classification**: Pseudoconical
 *
 * **Available forms**: Forward and inverse, spherical and ellipsoidal
 *
 * **Defined area**: Global
 *
 * **Alias**: poly
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=poly
 * ```
 *
 * ## Required Parameters
 * - None
 *
 * ## Optional Parameters
 * - `+lon_0=<value>`: Central meridian.
 * - `+ellps=<value>`: Ellipsoid used.
 * - `+R=<value>`: Radius of the projection sphere.
 * - `+x_0=<value>`: False easting.
 * - `+y_0=<value>`: False northing.
 *
 * ![Polyconic (American)](./images/poly.png)
 */
export class Polyconic extends ProjectionBase implements ProjectionTransform {
  name = 'Polyconic';
  static names = ['Polyconic', 'poly'];
  // Polyconic specific variables
  temp: number;
  e0: number;
  e1: number;
  e2: number;
  e3: number;
  ml0: number;

  /**
   * Preps an Polyconic projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);

    this.temp = this.b / this.a;
    this.es = 1 - pow(this.temp, 2); // devait etre dans tmerc.js mais n y est pas donc je commente sinon retour de valeurs nulles
    this.e = sqrt(this.es);
    this.e0 = e0fn(this.es);
    this.e1 = e1fn(this.es);
    this.e2 = e2fn(this.es);
    this.e3 = e3fn(this.es);
    this.ml0 = this.a * mlfn(this.e0, this.e1, this.e2, this.e3, this.lat0); //si que des zeros le calcul ne se fait pas
  }

  /**
   * Polyconic forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const { x: lon, y: lat } = p;
    let x, y;
    const dlon = adjustLon(lon - this.long0);
    const el = dlon * sin(lat);
    if (this.sphere) {
      if (abs(lat) <= EPSLN) {
        x = this.a * dlon;
        y = -1 * this.a * this.lat0;
      } else {
        x = (this.a * sin(el)) / tan(lat);
        y = this.a * (adjustLat(lat - this.lat0) + (1 - cos(el)) / tan(lat));
      }
    } else {
      if (abs(lat) <= EPSLN) {
        x = this.a * dlon;
        y = -1 * this.ml0;
      } else {
        const nl = gN(this.a, this.e, sin(lat)) / tan(lat);
        x = nl * sin(el);
        y = this.a * mlfn(this.e0, this.e1, this.e2, this.e3, lat) - this.ml0 + nl * (1 - cos(el));
      }
    }
    p.x = x + this.x0;
    p.y = y + this.y0;
  }

  /**
   * Polyconic inverse equations--mapping x-y to lon-lat
   * @param p - Polyconic point
   */
  inverse(p: VectorPoint): void {
    let lon, i;
    let lat = 0;
    let al, bl;
    let phi, dphi;
    const x = p.x - this.x0;
    const y = p.y - this.y0;
    if (this.sphere) {
      if (abs(y + this.a * this.lat0) <= EPSLN) {
        lon = adjustLon(x / this.a + this.long0);
        lat = 0;
      } else {
        al = this.lat0 + y / this.a;
        bl = (x * x) / this.a / this.a + al * al;
        phi = al;
        let tanphi;
        for (i = 20; i !== 0; --i) {
          tanphi = tan(phi);
          dphi =
            (-1 * (al * (phi * tanphi + 1) - phi - 0.5 * (phi * phi + bl) * tanphi)) /
            ((phi - al) / tanphi - 1);
          phi += dphi;
          if (abs(dphi) <= EPSLN) {
            lat = phi;
            break;
          }
        }
        lon = adjustLon(this.long0 + asin((x * tan(phi)) / this.a) / sin(lat));
      }
    } else {
      if (abs(y + this.ml0) <= EPSLN) {
        lon = adjustLon(this.long0 + x / this.a);
      } else {
        al = (this.ml0 + y) / this.a;
        bl = (x * x) / this.a / this.a + al * al;
        phi = al;
        let cl, mln, mlnp, ma;
        let con;
        for (i = 20; i !== 0; --i) {
          con = this.e * sin(phi);
          cl = sqrt(1 - con * con) * tan(phi);
          mln = this.a * mlfn(this.e0, this.e1, this.e2, this.e3, phi);
          mlnp =
            this.e0 -
            2 * this.e1 * cos(2 * phi) +
            4 * this.e2 * cos(4 * phi) -
            6 * this.e3 * cos(6 * phi);
          ma = mln / this.a;
          dphi =
            (al * (cl * ma + 1) - ma - 0.5 * cl * (ma * ma + bl)) /
            ((this.es * sin(2 * phi) * (ma * ma + bl - 2 * al * ma)) / (4 * cl) +
              (al - ma) * (cl * mlnp - 2 / sin(2 * phi)) -
              mlnp);
          phi -= dphi;
          if (abs(dphi) <= EPSLN) {
            lat = phi;
            break;
          }
        }
        // lat = phi4z(this.e, this.e0, this.e1, this.e2, this.e3, al, bl, 0, 0);
        cl = sqrt(1 - this.es * pow(sin(lat), 2)) * tan(lat);
        lon = adjustLon(this.long0 + asin((x * cl) / this.a) / sin(lat));
      }
    }
    p.x = lon;
    p.y = lat;
  }
}
