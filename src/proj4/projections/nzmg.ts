import { ProjectionBase } from '.';
import { SEC_TO_RAD } from '../constants';

import type { VectorPoint } from '../../geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

/**
 * # New Zealand Map Grid (EPSG:27200)
 *
 * **Classification**: Custom grid-based projection
 *
 * **Available forms**: Forward and inverse, spherical and ellipsoidal
 *
 * **Defined area**: New Zealand
 *
 * **Alias**: nzmg
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=nzmg
 * ```
 *
 * ## Required Parameters
 * - None (all standard projection parameters are hard-coded for this projection)
 *
 * ## Reference:
 * - Department of Land and Survey Technical Circular 1973/32
 * http://www.linz.govt.nz/docs/miscellaneous/nz-map-definition.pdf
 * - OSG Technical Report 4.1
 * http://www.linz.govt.nz/docs/miscellaneous/nzmg.pdf
 *
 * ![New Zealand Map Grid (EPSG:27200)](./images/nzmg.png)
 */
export class NewZealandMapGrid extends ProjectionBase implements ProjectionTransform {
  name = 'NewZealandMapGrid';
  static names = ['NewZealandMapGrid', 'New_Zealand_Map_Grid', 'nzmg'];
  // NewZealandMapGrid specific variables

  // * iterations: Number of iterations to refine inverse transform.
  // *     0 -> km accuracy
  // *     1 -> m accuracy -- suitable for most mapping applications
  // *     2 -> mm accuracy
  iterations = 1;

  A = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
  B_re = [0, 0, 0, 0, 0, 0];
  B_im = [0, 0, 0, 0, 0, 0];
  C_re = [0, 0, 0, 0, 0, 0];
  C_im = [0, 0, 0, 0, 0, 0];
  D = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

  /**
   * Preps an NewZealandMapGrid projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);

    this.A[1] = 0.6399175073;
    this.A[2] = -0.1358797613;
    this.A[3] = 0.063294409;
    this.A[4] = -0.02526853;
    this.A[5] = 0.0117879;
    this.A[6] = -0.0055161;
    this.A[7] = 0.0026906;
    this.A[8] = -0.001333;
    this.A[9] = 0.00067;
    this.A[10] = -0.00034;

    this.B_re[1] = 0.7557853228;
    this.B_im[1] = 0;
    this.B_re[2] = 0.249204646;
    this.B_im[2] = 0.003371507;
    this.B_re[3] = -0.001541739;
    this.B_im[3] = 0.04105856;
    this.B_re[4] = -0.10162907;
    this.B_im[4] = 0.01727609;
    this.B_re[5] = -0.26623489;
    this.B_im[5] = -0.36249218;
    this.B_re[6] = -0.6870983;
    this.B_im[6] = -1.1651967;

    this.C_re[1] = 1.3231270439;
    this.C_im[1] = 0;
    this.C_re[2] = -0.577245789;
    this.C_im[2] = -0.007809598;
    this.C_re[3] = 0.508307513;
    this.C_im[3] = -0.112208952;
    this.C_re[4] = -0.15094762;
    this.C_im[4] = 0.18200602;
    this.C_re[5] = 1.01418179;
    this.C_im[5] = 1.64497696;
    this.C_re[6] = 1.9660549;
    this.C_im[6] = 2.5127645;

    this.D[1] = 1.5627014243;
    this.D[2] = 0.5185406398;
    this.D[3] = -0.03333098;
    this.D[4] = -0.1052906;
    this.D[5] = -0.0368594;
    this.D[6] = 0.007317;
    this.D[7] = 0.0122;
    this.D[8] = 0.00394;
    this.D[9] = -0.0013;
  }

  /**
   * NewZealandMapGrid forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const { x: lon, y: lat } = p;
    let n;
    const deltaLat = lat - this.lat0;
    const deltaLon = lon - this.long0;
    // 1. Calculate d_phi and d_psi    ...                          // and d_lambda
    // For this algorithm, deltaLatitude is in seconds of arc x 10-5, so we need to scale to those units. Longitude is radians.
    const d_phi = (deltaLat / SEC_TO_RAD) * 1e-5;
    const d_lambda = deltaLon;
    let d_phi_n = 1; // d_phi^0
    let d_psi = 0;
    for (n = 1; n <= 10; n++) {
      d_phi_n = d_phi_n * d_phi;
      d_psi = d_psi + this.A[n] * d_phi_n;
    }
    // 2. Calculate theta
    const th_re = d_psi;
    const th_im = d_lambda;
    // 3. Calculate z
    let th_n_re = 1;
    let th_n_im = 0; // theta^0
    let th_n_re1;
    let th_n_im1;
    let z_re = 0;
    let z_im = 0;
    for (n = 1; n <= 6; n++) {
      th_n_re1 = th_n_re * th_re - th_n_im * th_im;
      th_n_im1 = th_n_im * th_re + th_n_re * th_im;
      th_n_re = th_n_re1;
      th_n_im = th_n_im1;
      z_re = z_re + this.B_re[n] * th_n_re - this.B_im[n] * th_n_im;
      z_im = z_im + this.B_im[n] * th_n_re + this.B_re[n] * th_n_im;
    }
    // 4. Calculate easting and northing
    p.x = z_im * this.a + this.x0;
    p.y = z_re * this.a + this.y0;
  }

  /**
   * NewZealandMapGrid inverse equations--mapping x-y to lon-lat
   * @param p - NewZealandMapGrid point
   */
  inverse(p: VectorPoint): void {
    const { x, y } = p;
    let n;
    const delta_x = x - this.x0;
    const delta_y = y - this.y0;
    // 1. Calculate z
    const z_re = delta_y / this.a;
    const z_im = delta_x / this.a;
    // 2a. Calculate theta - first approximation gives km accuracy
    let z_n_re = 1;
    let z_n_im = 0; // z^0
    let z_n_re1;
    let z_n_im1;
    let th_re = 0;
    let th_im = 0;
    for (n = 1; n <= 6; n++) {
      z_n_re1 = z_n_re * z_re - z_n_im * z_im;
      z_n_im1 = z_n_im * z_re + z_n_re * z_im;
      z_n_re = z_n_re1;
      z_n_im = z_n_im1;
      th_re = th_re + this.C_re[n] * z_n_re - this.C_im[n] * z_n_im;
      th_im = th_im + this.C_im[n] * z_n_re + this.C_re[n] * z_n_im;
    }
    // 2b. Iterate to refine the accuracy of the calculation
    //        0 iterations gives km accuracy
    //        1 iteration gives m accuracy -- good enough for most mapping applications
    //        2 iterations bives mm accuracy
    for (let i = 0; i < this.iterations; i++) {
      let th_n_re = th_re;
      let th_n_im = th_im;
      let th_n_re1;
      let th_n_im1;
      let num_re = z_re;
      let num_im = z_im;
      for (n = 2; n <= 6; n++) {
        th_n_re1 = th_n_re * th_re - th_n_im * th_im;
        th_n_im1 = th_n_im * th_re + th_n_re * th_im;
        th_n_re = th_n_re1;
        th_n_im = th_n_im1;
        num_re = num_re + (n - 1) * (this.B_re[n] * th_n_re - this.B_im[n] * th_n_im);
        num_im = num_im + (n - 1) * (this.B_im[n] * th_n_re + this.B_re[n] * th_n_im);
      }
      th_n_re = 1;
      th_n_im = 0;
      let den_re = this.B_re[1];
      let den_im = this.B_im[1];
      for (n = 2; n <= 6; n++) {
        th_n_re1 = th_n_re * th_re - th_n_im * th_im;
        th_n_im1 = th_n_im * th_re + th_n_re * th_im;
        th_n_re = th_n_re1;
        th_n_im = th_n_im1;
        den_re = den_re + n * (this.B_re[n] * th_n_re - this.B_im[n] * th_n_im);
        den_im = den_im + n * (this.B_im[n] * th_n_re + this.B_re[n] * th_n_im);
      }
      // Complex division
      const den2 = den_re * den_re + den_im * den_im;
      th_re = (num_re * den_re + num_im * den_im) / den2;
      th_im = (num_im * den_re - num_re * den_im) / den2;
    }
    // 3. Calculate d_phi              ...                                    // and d_lambda
    const d_psi = th_re;
    const d_lambda = th_im;
    let d_psi_n = 1; // d_psi^0
    let d_phi = 0;
    for (n = 1; n <= 9; n++) {
      d_psi_n = d_psi_n * d_psi;
      d_phi = d_phi + this.D[n] * d_psi_n;
    }
    // 4. Calculate latitude and longitude
    // d_phi is calcuated in second of arc * 10^-5, so we need to scale back to radians. d_lambda is in radians.
    const lat = this.lat0 + d_phi * SEC_TO_RAD * 1e5;
    const lon = this.long0 + d_lambda;
    p.x = lon;
    p.y = lat;
  }
}
