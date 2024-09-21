import { ProjectionBase } from '.';

import type { VectorPoint } from 's2-tools/geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

const { abs, pow, sin, cos, sqrt, asin, atan, exp, log, tan, PI } = Math;

/**
 * # Swiss Oblique Mercator
 *
 * **Classification**: Oblique Mercator
 *
 * **Available forms**: Forward and inverse, ellipsoidal only
 *
 * **Defined area**: Global
 *
 * **Alias**: somerc
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=somerc
 * ```
 *
 * ## Required Parameters
 * - None
 *
 * ## Optional Parameters
 * - `+lon_0=<value>`: Central meridian.
 * - `+ellps=<value>`: Ellipsoid used.
 * - `+R=<value>`: Radius of the projection sphere.
 * - `+k_0=<value>`: Scale factor.
 * - `+x_0=<value>`: False easting.
 * - `+y_0=<value>`: False northing.
 *
 * ## References:
 * Formules et constantes pour le Calcul pour la
 * projection cylindrique conforme à axe oblique et pour la transformation entre
 * des systèmes de référence.
 * http://www.swisstopo.admin.ch/internet/swisstopo/fr/home/topics/survey/sys/refsys/switzerland.parsysrelated1.31216.downloadList.77004.DownloadFile.tmp/swissprojectionfr.pdf
 *
 * ![Swiss Oblique Mercator](./images/somerc.png)
 */
export class SwissObliqueMercator extends ProjectionBase implements ProjectionTransform {
  name = 'SwissObliqueMercator';
  static names = ['SwissObliqueMercator', 'Swiss Oblique Mercator', 'somerc'];
  // SwissObliqueMercator specific variables
  lambda0: number;
  alpha: number;
  b0: number;
  K: number;
  R: number;

  /**
   * Preps an SwissObliqueMercator projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);

    const phy0 = this.lat0;
    this.lambda0 = this.long0;
    const sinPhy0 = sin(phy0);
    const semiMajorAxis = this.a;
    const invF = this.rf;
    const flattening = 1 / invF;
    const e2 = 2 * flattening - pow(flattening, 2);
    const e = (this.e = sqrt(e2));
    this.R = (this.k0 * semiMajorAxis * sqrt(1 - e2)) / (1 - e2 * pow(sinPhy0, 2));
    this.alpha = sqrt(1 + (e2 / (1 - e2)) * pow(cos(phy0), 4));
    this.b0 = asin(sinPhy0 / this.alpha);
    const k1 = log(tan(PI / 4 + this.b0 / 2));
    const k2 = log(tan(PI / 4 + phy0 / 2));
    const k3 = log((1 + e * sinPhy0) / (1 - e * sinPhy0));
    this.K = k1 - this.alpha * k2 + ((this.alpha * e) / 2) * k3;
  }

  /**
   * SwissObliqueMercator forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const Sa1 = log(tan(PI / 4 - p.y / 2));
    const Sa2 = (this.e / 2) * log((1 + this.e * sin(p.y)) / (1 - this.e * sin(p.y)));
    const S = -this.alpha * (Sa1 + Sa2) + this.K;
    // spheric latitude
    const b = 2 * (atan(exp(S)) - PI / 4);
    // spheric longitude
    const I = this.alpha * (p.x - this.lambda0);
    // psoeudo equatorial rotation
    const rotI = atan(sin(I) / (sin(this.b0) * tan(b) + cos(this.b0) * cos(I)));
    const rotB = asin(cos(this.b0) * sin(b) - sin(this.b0) * cos(b) * cos(I));
    p.y = (this.R / 2) * log((1 + sin(rotB)) / (1 - sin(rotB))) + this.y0;
    p.x = this.R * rotI + this.x0;
  }

  /**
   * SwissObliqueMercator inverse equations--mapping x-y to lon-lat
   * @param p - SwissObliqueMercator point
   */
  inverse(p: VectorPoint): void {
    const Y = p.x - this.x0;
    const X = p.y - this.y0;
    const rotI = Y / this.R;
    const rotB = 2 * (atan(exp(X / this.R)) - PI / 4);
    const b = asin(cos(this.b0) * sin(rotB) + sin(this.b0) * cos(rotB) * cos(rotI));
    const I = atan(sin(rotI) / (cos(this.b0) * cos(rotI) - sin(this.b0) * tan(rotB)));
    const lambda = this.lambda0 + I / this.alpha;
    let S = 0;
    let phy = b;
    let prevPhy = -1000;
    let iteration = 0;
    while (abs(phy - prevPhy) > 0.0000001) {
      if (++iteration > 20) {
        throw new Error('omercFwdInfinity');
      }
      //S = log(tan(PI / 4 + phy / 2));
      S =
        (1 / this.alpha) * (log(tan(PI / 4 + b / 2)) - this.K) +
        this.e * log(tan(PI / 4 + asin(this.e * sin(phy)) / 2));
      prevPhy = phy;
      phy = 2 * atan(exp(S)) - PI / 2;
    }
    p.x = lambda;
    p.y = phy;
  }
}
