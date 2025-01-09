import { defaultGetInterpolateCurrentValue } from '.';

import type { GetInterpolateValue } from '.';
import type { Properties, VectorPoint } from '../..';

/** Kriging Model function */
export type KrigingFunction = (
  h: number,
  nugget: number,
  range: number,
  sill: number,
  A: number,
) => number;

/** Kriging Models available */
export type KrigingModel = 'gaussian' | 'exponential' | 'spherical';

/**
 * # Kriging Interpolator
 *
 * ## Description
 * Interpolation using the Kriging method. Find the best fit curve for a given set of data.
 * You can either compute the semivariance or the variogram (expected value).
 *
 * The various variogram models can be interpreted as kernel functions for 2-dimensional coordinates
 * a, b and parameters nugget, range, sill and A. Reparameterized as a linear function, with
 * w = [nugget, (sill-nugget)/range], this becomes:
 * - **Gaussian**: `k(a,b) = w[0] + w[1] * ( 1 - exp{ -( ||a-b|| / range )2 / A } )`
 * - **Exponential**: `k(a,b) = w[0] + w[1] * ( 1 - exp{ -( ||a-b|| / range ) / A } )`
 * - **Spherical**: `k(a,b) = w[0] + w[1] * ( 1.5 * ( ||a-b|| / range ) - 0.5 * ( ||a-b|| / range )3 )`
 *
 * Notice the σ2 (sigma2) and α (alpha) variables, these correspond to the variance parameters of
 * the gaussian process and the prior of the variogram model, respectively. A diffuse α prior is
 * typically used; a formal mathematical definition of the model is provided below.
 *
 * The variance parameter α of the prior distribution for w should be manually set, according to:
 * - `w ~ N(w|0, αI)`
 *
 * Using the fitted kernel function hyperparameters and setting K as the Gram matrix, the prior and
 * likelihood for the gaussian process become:
 * - `y ~ N(y|0, K)`
 * - `t|y ~ N(t|y, σ2I)
 *
 * ## Usage
 * ```ts
 * import { KrigingInterpolator } from 'gis-tools';
 * import type { VectorPoint } from 'gis-tools';
 *
 * // We have m-value data that we want to interpolate
 * interface TempData { temp: number; }
 *
 * // given a point we are interested in
 * const point: VectorPoint = { x: 20, y: -40 };
 * //  get a collection of points relative to the point
 * const data: VectorPoint<TempData>[] = [...];
 *
 * // interpolate
 * const interpolator = new KrigingInterpolator<TempData>(data, 'gaussian', (p) => p.m.temp);
 * const interpolatedValue = interpolator.predict(point);
 * ```
 *
 * ## Links
 * - https://pro.arcgis.com/en/pro-app/latest/tool-reference/3d-analyst/how-kriging-works.htm
 */
export class KrigingInterpolator<T extends Properties = Properties> {
  t: number[] = [];
  nugget = 0;
  range = 0;
  sill = 0;
  A = 1 / 3;
  K: number[] = [];
  M: number[] = [];
  n = 0;
  model: KrigingFunction;
  /**
   * @param refData - reference data to interpolate from
   * @param model - kriging model
   * @param getValue - function to get value from reference data. Can be the z value or a property in the m-values
   * @param sigma2 - variance parameter of the gaussian model
   * @param alpha - diffuse α prior of the variogram model
   */
  constructor(
    private refData: VectorPoint<T>[],
    model: KrigingModel = 'gaussian',
    getValue: GetInterpolateValue = defaultGetInterpolateCurrentValue,
    sigma2 = 0,
    alpha = 100,
  ) {
    const { abs, pow } = Math;
    this.t = refData.map((p) => getValue(p));
    // setup model
    switch (model) {
      case 'exponential':
        this.model = krigingVariogramExponential;
        break;
      case 'spherical':
        this.model = krigingVariogramSpherical;
        break;
      case 'gaussian':
      default:
        this.model = krigingVariogramGaussian;
        break;
    }
    // Lag distance/semivariance
    let i, j, k, l;
    const { t } = this;
    let n = t.length;
    const distance = Array((n * n - n) / 2);
    for (i = 0, k = 0; i < n; i++) {
      const { x: xi, y: yi } = this.refData[i];
      for (j = 0; j < i; j++, k++) {
        const { x: xj, y: yj } = this.refData[j];
        distance[k] = Array(2);
        distance[k][0] = pow(pow(xi - xj, 2) + pow(yi - yj, 2), 0.5);
        distance[k][1] = abs(t[i] - t[j]);
      }
    }
    distance.sort((a, b) => a[0] - b[0]);
    this.range = distance[(n * n - n) / 2 - 1][0];

    // Bin lag distance
    const lags = (n * n - n) / 2 > 30 ? 30 : (n * n - n) / 2;
    const tolerance = this.range / lags;
    const lag = rep([0], lags);
    const semi = rep([0], lags);
    if (lags < 30) {
      for (l = 0; l < lags; l++) {
        lag[l] = distance[l][0];
        semi[l] = distance[l][1];
      }
    } else {
      for (i = 0, j = 0, k = 0, l = 0; i < lags && j < (n * n - n) / 2; i++, k = 0) {
        while (distance[j][0] <= (i + 1) * tolerance) {
          lag[l] += distance[j][0];
          semi[l] += distance[j][1];
          j++;
          k++;
          if (j >= (n * n - n) / 2) break;
        }
        if (k > 0) {
          lag[l] /= k;
          semi[l] /= k;
          l++;
        }
      }
      if (l < 2) return; // Error: Not enough points
    }
    // feature transformation
    n = l;
    this.range = lag[n - 1] - lag[0];
    const X = rep([1], 2 * n);
    const Y = Array(n);
    const A = this.A;

    for (i = 0; i < n; i++) {
      switch (model) {
        case 'gaussian':
          X[i * 2 + 1] = 1.0 - Math.exp(-(1.0 / A) * Math.pow(lag[i] / this.range, 2));
          break;
        case 'exponential':
          X[i * 2 + 1] = 1.0 - Math.exp((-(1.0 / A) * lag[i]) / this.range);
          break;
        case 'spherical':
          X[i * 2 + 1] = 1.5 * (lag[i] / this.range) - 0.5 * Math.pow(lag[i] / this.range, 3);
          break;
      }

      Y[i] = semi[i];
    }

    // Least squares
    const Xt = krigingMatrixTranspose(X, n, 2);
    let Z = krigingMatrixMultiply(Xt, X, 2, n, 2);
    Z = krigingMatrixAdd(Z, krigingMatrixDiag(1 / alpha, 2), 2, 2);
    const cloneZ = Z.slice(0);
    if (krigingMatrixChol(Z, 2)) {
      krigingMatrixChol2inv(Z, 2);
    } else {
      krigingMatrixSolve(cloneZ, 2);
      Z = cloneZ;
    }
    const W = krigingMatrixMultiply(krigingMatrixMultiply(Z, Xt, 2, 2, n), Y, 2, n, 1);

    // Variogram parameters
    this.nugget = W[0];
    this.sill = W[1] * this.range + this.nugget;
    n = this.n = this.refData.length;

    // Gram matrix with prior
    const K = Array(n * n);
    for (i = 0; i < n; i++) {
      const { x: refiX, y: refiY } = this.refData[i];
      for (j = 0; j < i; j++) {
        const { x: refjX, y: refjY } = this.refData[j];
        K[i * n + j] = this.model(
          Math.pow(Math.pow(refiX - refjX, 2) + Math.pow(refiY - refjY, 2), 0.5),
          this.nugget,
          this.range,
          this.sill,
          this.A,
        );
        K[j * n + i] = K[i * n + j];
      }
      K[i * n + i] = this.model(0, this.nugget, this.range, this.sill, this.A);
    }

    // Inverse penalized Gram matrix projected to target vector
    let C = krigingMatrixAdd(K, krigingMatrixDiag(sigma2, n), n, n);
    const cloneC = C.slice(0);
    if (krigingMatrixChol(C, n)) {
      krigingMatrixChol2inv(C, n);
    } else {
      krigingMatrixSolve(cloneC, n);
      C = cloneC;
    }

    // Copy unprojected inverted matrix as K
    const Ks = C.slice(0);
    const M = krigingMatrixMultiply(C, t, n, n, 1);
    this.K = Ks;
    this.M = M;
  }

  /**
   * Model prediction
   * @param x - x
   * @param y - y
   * @returns - predicted value
   */
  predict(x: number, y: number): number {
    const k = new Array(this.n);
    for (let i = 0; i < this.n; i++) {
      const { x: refX, y: refY } = this.refData[i];
      k[i] = this.model(
        Math.pow(Math.pow(x - refX, 2) + Math.pow(y - refY, 2), 0.5),
        this.nugget,
        this.range,
        this.sill,
        this.A,
      );
    }

    return krigingMatrixMultiply(k, this.M, 1, this.n, 1)[0];
  }

  /**
   * Variance prediction
   * @param x - x value
   * @param y - y value
   * @returns - predicted variance
   */
  variance(x: number, y: number): number {
    let i;
    const k = Array(this.n);
    for (i = 0; i < this.n; i++) {
      const { x: refX, y: refY } = this.refData[i];
      k[i] = this.model(
        Math.pow(Math.pow(x - refX, 2) + Math.pow(y - refY, 2), 0.5),
        this.nugget,
        this.range,
        this.sill,
        this.A,
      );
    }
    return (
      this.model(0, this.nugget, this.range, this.sill, this.A) +
      krigingMatrixMultiply(krigingMatrixMultiply(k, this.K, 1, this.n, this.n), k, 1, this.n, 1)[0]
    );
  }
}

/**
 * Matrix diagonal algebra
 * @param c - diagonal value
 * @param n - matrix size
 * @returns - diagonal matrix
 */
function krigingMatrixDiag(c: number, n: number): number[] {
  let i;
  const Z = rep([0], n * n);
  for (i = 0; i < n; i++) Z[i * n + i] = c;
  return Z;
}

/**
 * Matrix transpose
 * @param X - matrix
 * @param n - matrix row size
 * @param m - matrix column size
 * @returns - transposed matrix
 */
function krigingMatrixTranspose(X: number[], n: number, m: number): number[] {
  let i;
  let j;
  const Z = new Array(m * n);
  for (i = 0; i < n; i++) {
    for (j = 0; j < m; j++) {
      Z[j * n + i] = X[i * m + j];
    }
  }
  return Z;
}

/**
 * Matrix addition
 * @param X - first matrix
 * @param Y - second matrix
 * @param n - matrix row size
 * @param m - matrix column size
 * @returns - added matrix
 */
function krigingMatrixAdd(X: number[], Y: number[], n: number, m: number): number[] {
  let i;
  let j;
  const Z = new Array(n * m);
  for (i = 0; i < n; i++) {
    for (j = 0; j < m; j++) {
      Z[i * m + j] = X[i * m + j] + Y[i * m + j];
    }
  }
  return Z;
}

/**
 * Naive matrix multiplication
 * @param X - matrix X
 * @param Y - matrix Y
 * @param n - matrix row size
 * @param m - matrix X column size
 * @param p - matrix Y column size
 * @returns - multiplied matrix
 */
function krigingMatrixMultiply(
  X: number[],
  Y: number[],
  n: number,
  m: number,
  p: number,
): number[] {
  let i, j, k;
  const Z = new Array(n * p);

  for (i = 0; i < n; i++) {
    for (j = 0; j < p; j++) {
      Z[i * p + j] = 0;
      for (k = 0; k < m; k++) {
        Z[i * p + j] += X[i * m + k] * Y[k * p + j];
      }
    }
  }

  return Z;
}

/**
 * Cholesky decomposition
 * @param X - matrix
 * @param n - matrix size
 * @returns - true if successful
 */
function krigingMatrixChol(X: number[], n: number): boolean {
  let i;
  let j;
  let k;
  const p = new Array(n);
  for (i = 0; i < n; i++) p[i] = X[i * n + i];
  for (i = 0; i < n; i++) {
    for (j = 0; j < i; j++) {
      p[i] -= X[i * n + j] * X[i * n + j];
    }
    if (p[i] <= 0) return false;
    p[i] = Math.sqrt(p[i]);
    for (j = i + 1; j < n; j++) {
      for (k = 0; k < i; k++) {
        X[j * n + i] -= X[j * n + k] * X[i * n + k];
      }
      X[j * n + i] /= p[i];
    }
  }
  for (i = 0; i < n; i++) X[i * n + i] = p[i];
  return true;
}

/**
 * Inversion of cholesky decomposition
 * @param X - matrix
 * @param n - matrix size
 */
function krigingMatrixChol2inv(X: number[], n: number): void {
  let i, j, k, sum;
  for (i = 0; i < n; i++) {
    X[i * n + i] = 1 / X[i * n + i];
    for (j = i + 1; j < n; j++) {
      sum = 0;
      for (k = i; k < j; k++) {
        sum -= X[j * n + k] * X[k * n + i];
      }
      X[j * n + i] = sum / X[j * n + j];
    }
  }
  for (i = 0; i < n; i++) {
    for (j = i + 1; j < n; j++) {
      X[i * n + j] = 0;
    }
  }
  for (i = 0; i < n; i++) {
    X[i * n + i] *= X[i * n + i];
    for (k = i + 1; k < n; k++) {
      X[i * n + i] += X[k * n + i] * X[k * n + i];
    }
    for (j = i + 1; j < n; j++) {
      for (k = j; k < n; k++) {
        X[i * n + j] += X[k * n + i] * X[k * n + j];
      }
    }
  }
  for (i = 0; i < n; i++) {
    for (j = 0; j < i; j++) {
      X[i * n + j] = X[j * n + i];
    }
  }
}

/**
 * Inversion via gauss-jordan elimination
 * @param X - matrix
 * @param n - matrix size
 * @returns - true if successful
 */
function krigingMatrixSolve(X: number[], n: number): boolean {
  const m = n;
  const b = new Array(n * n);
  const indxc = new Array(n);
  const indxr = new Array(n);
  const ipiv = new Array(n);
  let i,
    icol = 0,
    irow = 0,
    j,
    k,
    l,
    ll;
  let big, dum, pivinv, temp;

  for (i = 0; i < n; i++) {
    for (j = 0; j < n; j++) {
      if (i === j) b[i * n + j] = 1;
      else b[i * n + j] = 0;
    }
  }
  for (j = 0; j < n; j++) ipiv[j] = 0;
  for (i = 0; i < n; i++) {
    big = 0;
    for (j = 0; j < n; j++) {
      if (ipiv[j] !== 1) {
        for (k = 0; k < n; k++) {
          if (ipiv[k] === 0) {
            if (Math.abs(X[j * n + k]) >= big) {
              big = Math.abs(X[j * n + k]);
              irow = j;
              icol = k;
            }
          }
        }
      }
    }
    ++ipiv[icol];

    if (irow !== icol) {
      for (l = 0; l < n; l++) {
        temp = X[irow * n + l];
        X[irow * n + l] = X[icol * n + l];
        X[icol * n + l] = temp;
      }
      for (l = 0; l < m; l++) {
        temp = b[irow * n + l];
        b[irow * n + l] = b[icol * n + l];
        b[icol * n + l] = temp;
      }
    }
    indxr[i] = irow;
    indxc[i] = icol;

    if (X[icol * n + icol] === 0) return false; // Singular

    pivinv = 1 / X[icol * n + icol];
    X[icol * n + icol] = 1;
    for (l = 0; l < n; l++) X[icol * n + l] *= pivinv;
    for (l = 0; l < m; l++) b[icol * n + l] *= pivinv;

    for (ll = 0; ll < n; ll++) {
      if (ll !== icol) {
        dum = X[ll * n + icol];
        X[ll * n + icol] = 0;
        for (l = 0; l < n; l++) X[ll * n + l] -= X[icol * n + l] * dum;
        for (l = 0; l < m; l++) b[ll * n + l] -= b[icol * n + l] * dum;
      }
    }
  }
  for (l = n - 1; l >= 0; l--) {
    if (indxr[l] !== indxc[l]) {
      for (k = 0; k < n; k++) {
        temp = X[k * n + indxr[l]];
        X[k * n + indxr[l]] = X[k * n + indxc[l]];
        X[k * n + indxc[l]] = temp;
      }
    }
  }

  return true;
}

/**
 * Variogram Gaussian Model
 * @param h - distance
 * @param nugget - nugget
 * @param range - range
 * @param sill - sill
 * @param A - A
 * @returns - predicted gaussian value
 */
function krigingVariogramGaussian(
  h: number,
  nugget: number,
  range: number,
  sill: number,
  A: number,
): number {
  return nugget + ((sill - nugget) / range) * (1.0 - Math.exp(-(1.0 / A) * Math.pow(h / range, 2)));
}

/**
 * Variogram Exponential Model
 * @param h - distance
 * @param nugget - nugget
 * @param range - range
 * @param sill - sill
 * @param A - A
 * @returns - predicted exponential value
 */
function krigingVariogramExponential(
  h: number,
  nugget: number,
  range: number,
  sill: number,
  A: number,
): number {
  return nugget + ((sill - nugget) / range) * (1.0 - Math.exp(-(1.0 / A) * (h / range)));
}

/**
 * Variogram Spherical Model
 * @param h - distance
 * @param nugget - nugget
 * @param range - range
 * @param sill - sill
 * @param _A - A (unused)
 * @returns - predicted spherical value
 */
function krigingVariogramSpherical(
  h: number,
  nugget: number,
  range: number,
  sill: number,
  _A: number,
): number {
  if (h > range) return nugget + (sill - nugget) / range;
  return nugget + ((sill - nugget) / range) * (1.5 * (h / range) - 0.5 * Math.pow(h / range, 3));
}

/**
 * Creates a new array of n size that is all set to the first element of arr
 * @param arr - array
 * @param n - number
 * @returns - array all set to the first element of arr
 */
function rep(arr: number[], n: number): number[] {
  return new Array(n).map(Number.prototype.valueOf, arr[0]);
}
