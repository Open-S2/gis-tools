import { ProjectionBase } from '.';
import { EPSLN, HALF_PI, QUART_PI, SPI, TWO_PI } from '../constants';

import type { VectorPoint } from 's2-tools/geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

const { abs, sin, cos, sqrt, atan2, atan, tan, acos } = Math;

/** Face enum */
enum FACE_ENUM {
  FRONT = 1,
  RIGHT = 2,
  BACK = 3,
  LEFT = 4,
  TOP = 5,
  BOTTOM = 6,
}

/** Area enum */
enum AREA_ENUM {
  AREA_0 = 1,
  AREA_1 = 2,
  AREA_2 = 3,
  AREA_3 = 4,
}

/** Area value object to track */
interface Area {
  value: number;
}

/**
 * # Quadrilateralized Spherical Cube
 *
 * The purpose of the Quadrilateralized Spherical Cube (QSC) projection is to project
 * a sphere surface onto the six sides of a cube:
 *
 * **Classification**: Azimuthal
 *
 * **Available forms**: Forward and inverse, ellipsoidal
 *
 * **Defined area**: Global
 *
 * **Alias**: qsc
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=qsc
 * ```
 *
 * For this purpose, other alternatives can be used, notably `gnom` or
 * `healpix`. However, QSC projection has the following favorable properties:
 *
 * It is an equal-area projection, and at the same time introduces only limited angular
 * distortions. It treats all cube sides equally, i.e. it does not use different
 * projections for polar areas and equatorial areas. These properties make QSC
 * projection a good choice for planetary-scale terrain rendering. Map data can be
 * organized in quadtree structures for each cube side. See `LambersKolb2012` for an example.
 *
 * The QSC projection was introduced by `ONeilLaubscher1976`,
 * building on previous work by `ChanONeil1975`. For clarity: The
 * earlier QSC variant described in `ChanONeil1975` became known as the COBE QSC since it
 * was used by the NASA Cosmic Background Explorer (COBE) project; it is an approximately
 * equal-area projection and is not the same as the QSC projection.
 * See also `CalabrettaGreisen2002` Sec. 5.6.2 and 5.6.3 for a description of both and
 * some analysis.
 *
 * In this implementation, the QSC projection projects onto one side of a circumscribed
 * cube. The cube side is selected by choosing one of the following six projection centers:
 *
 * `+lat_0=0 +lon_0=0`   | front cube side    |
 *
 * `+lat_0=0 +lon_0=90`  | right cube side    |
 *
 * `+lat_0=0 +lon_0=180` | back cube side     |
 *
 * `+lat_0=0 +lon_0=-90` | left cube side     |
 *
 * `+lat_0=90`           | top cube side      |
 *
 * `+lat_0=-90`          | bottom cube side   |
 *
 * ## Required Parameters
 * - `+lat_0=<value>`: Latitude of the projection center.
 * - `+lon_0=<value>`: Longitude of the projection center.
 *
 * ## Optional Parameters
 * - `+ellps=<value>`: Ellipsoid parameters (default: `WGS84`).
 * - `+x_0=<value>`: False easting.
 * - `+y_0=<value>`: False northing.
 *
 * ## Usage Example
 * ```
 * gdalwarp -t_srs "+wktext +proj=qsc +units=m +ellps=WGS84  +lat_0=0 +lon_0=0"        \
 *     -wo SOURCE_EXTRA=100 -wo SAMPLE_GRID=YES -te -6378137 -6378137 6378137 6378137  \
 *     worldmap.tiff frontside.tiff
 *
 * gdalwarp -t_srs "+wktext +proj=qsc +units=m +ellps=WGS84 +lat_0=0 +lon_0=90"        \
 *     -wo SOURCE_EXTRA=100 -wo SAMPLE_GRID=YES -te -6378137 -6378137 6378137 6378137  \
 *     worldmap.tiff rightside.tiff
 *
 * gdalwarp -t_srs "+wktext +proj=qsc +units=m +ellps=WGS84 +lat_0=0 +lon_0=180"       \
 *     -wo SOURCE_EXTRA=100 -wo SAMPLE_GRID=YES -te -6378137 -6378137 6378137 6378137  \
 *     worldmap.tiff backside.tiff
 *
 * gdalwarp -t_srs "+wktext +proj=qsc +units=m +ellps=WGS84 +lat_0=0 +lon_0=-90"       \
 *     -wo SOURCE_EXTRA=100 -wo SAMPLE_GRID=YES -te -6378137 -6378137 6378137 6378137  \
 *     worldmap.tiff leftside.tiff
 *
 * gdalwarp -t_srs "+wktext +proj=qsc +units=m +ellps=WGS84 +lat_0=90 +lon_0=0"        \
 *     -wo SOURCE_EXTRA=100 -wo SAMPLE_GRID=YES -te -6378137 -6378137 6378137 6378137  \
 *     worldmap.tiff topside.tiff
 *
 * gdalwarp -t_srs "+wktext +proj=qsc +units=m +ellps=WGS84 +lat_0=-90 +lon_0=0"       \
 *     -wo SOURCE_EXTRA=100 -wo SAMPLE_GRID=YES -te -6378137 -6378137 6378137 6378137  \
 *     worldmap.tiff bottomside.tiff
 * ```
 *
 * ## Further Reading
 * - [Wikipedia](https://en.wikipedia.org/wiki/Quadrilateralized_spherical_cube)
 * - [NASA](https://lambda.gsfc.nasa.gov/product/cobe/skymap_info_new.cfm)
 *
 * ![Quadrilateralized Spherical Cube](./images/qsc_concept.jpg)
 */
export class QuadrilateralizedSphericalCube extends ProjectionBase implements ProjectionTransform {
  name = 'QuadrilateralizedSphericalCube';
  static names = [
    'QuadrilateralizedSphericalCube',
    'Quadrilateralized Spherical Cube',
    'Quadrilateralized_Spherical_Cube',
    'qsc',
  ];
  // QuadrilateralizedSphericalCube specific variables
  declare x0: number;
  declare y0: number;
  declare lat0: number;
  declare long0: number;
  declare latTs: number;
  oneMinusF = 0;
  oneMinusFSquared = 0;
  face: FACE_ENUM;
  declare area: AREA_ENUM;

  /**
   * Preps an QuadrilateralizedSphericalCube projection
   * QSC projection rewritten from the original PROJ4
   * https://github.com/OSGeo/proj.4/blob/master/src/PJ_qsc.c
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);

    this.x0 = this.x0 ?? 0;
    this.y0 = this.y0 ?? 0;
    this.lat0 = this.lat0 ?? 0;
    this.long0 = this.long0 ?? 0;
    this.latTs = this.latTs ?? 0;

    /* Determine the cube face from the center of projection. */
    if (this.lat0 >= HALF_PI - QUART_PI / 2.0) {
      this.face = FACE_ENUM.TOP;
    } else if (this.lat0 <= -(HALF_PI - QUART_PI / 2.0)) {
      this.face = FACE_ENUM.BOTTOM;
    } else if (abs(this.long0) <= QUART_PI) {
      this.face = FACE_ENUM.FRONT;
    } else if (abs(this.long0) <= HALF_PI + QUART_PI) {
      this.face = this.long0 > 0.0 ? FACE_ENUM.RIGHT : FACE_ENUM.LEFT;
    } else {
      this.face = FACE_ENUM.BACK;
    }

    /* Fill in useful values for the ellipsoid <-> sphere shift
     * described in [LK12]. */
    if (this.es !== 0) {
      this.oneMinusF = 1 - (this.a - this.b) / this.a;
      this.oneMinusFSquared = this.oneMinusF * this.oneMinusF;
    }
  }

  /**
   * QuadrilateralizedSphericalCube forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const xy = { x: 0, y: 0 };
    let lat, lon;
    let theta, phi;
    let mu;
    /* nu; */
    const area = { value: 0 };
    // move lon according to projection's lon
    p.x -= this.long0;
    /* Convert the geodetic latitude to a geocentric latitude.
     * This corresponds to the shift from the ellipsoid to the sphere
     * described in [LK12]. */
    if (this.es !== 0) {
      //if (P->es != 0) {
      lat = atan(this.oneMinusFSquared * tan(p.y));
    } else {
      lat = p.y;
    }
    /* Convert the input lat, lon into theta, phi as used by QSC.
     * This depends on the cube face and the area on it.
     * For the top and bottom face, we can compute theta and phi
     * directly from phi, lam. For the other faces, we must use
     * unit sphere cartesian coordinates as an intermediate step. */
    lon = p.x; //lon = lp.lam;
    if (this.face === FACE_ENUM.TOP) {
      phi = HALF_PI - lat;
      if (lon >= QUART_PI && lon <= HALF_PI + QUART_PI) {
        area.value = AREA_ENUM.AREA_0;
        theta = lon - HALF_PI;
      } else if (lon > HALF_PI + QUART_PI || lon <= -(HALF_PI + QUART_PI)) {
        area.value = AREA_ENUM.AREA_1;
        theta = lon > 0.0 ? lon - SPI : lon + SPI;
      } else if (lon > -(HALF_PI + QUART_PI) && lon <= -QUART_PI) {
        area.value = AREA_ENUM.AREA_2;
        theta = lon + HALF_PI;
      } else {
        area.value = AREA_ENUM.AREA_3;
        theta = lon;
      }
    } else if (this.face === FACE_ENUM.BOTTOM) {
      phi = HALF_PI + lat;
      if (lon >= QUART_PI && lon <= HALF_PI + QUART_PI) {
        area.value = AREA_ENUM.AREA_0;
        theta = -lon + HALF_PI;
      } else if (lon < QUART_PI && lon >= -QUART_PI) {
        area.value = AREA_ENUM.AREA_1;
        theta = -lon;
      } else if (lon < -QUART_PI && lon >= -(HALF_PI + QUART_PI)) {
        area.value = AREA_ENUM.AREA_2;
        theta = -lon - HALF_PI;
      } else {
        area.value = AREA_ENUM.AREA_3;
        theta = lon > 0.0 ? -lon + SPI : -lon - SPI;
      }
    } else {
      if (this.face === FACE_ENUM.RIGHT) {
        lon = qscShiftLonOrigin(lon, HALF_PI);
      } else if (this.face === FACE_ENUM.BACK) {
        lon = qscShiftLonOrigin(lon, SPI);
      } else if (this.face === FACE_ENUM.LEFT) {
        lon = qscShiftLonOrigin(lon, -HALF_PI);
      }
      const sinlat = sin(lat);
      const coslat = cos(lat);
      const sinlon = sin(lon);
      const coslon = cos(lon);
      const q = coslat * coslon;
      const r = coslat * sinlon;
      const s = sinlat;
      if (this.face === FACE_ENUM.FRONT) {
        phi = acos(q);
        theta = qscFwdEquatFaceTheta(phi, s, r, area);
      } else if (this.face === FACE_ENUM.RIGHT) {
        phi = acos(r);
        theta = qscFwdEquatFaceTheta(phi, s, -q, area);
      } else if (this.face === FACE_ENUM.BACK) {
        phi = acos(-q);
        theta = qscFwdEquatFaceTheta(phi, s, -r, area);
      } else if (this.face === FACE_ENUM.LEFT) {
        phi = acos(-r);
        theta = qscFwdEquatFaceTheta(phi, s, q, area);
      } else {
        /* Impossible */
        phi = theta = 0;
        area.value = AREA_ENUM.AREA_0;
      }
    }
    /* Compute mu and nu for the area of definition.
     * For mu, see Eq. (3-21) in [OL76], but note the typos:
     * compare with Eq. (3-14). For nu, see Eq. (3-38). */
    mu = atan((12 / SPI) * (theta + acos(sin(theta) * cos(QUART_PI)) - HALF_PI));
    const t = sqrt((1 - cos(phi)) / (cos(mu) * cos(mu)) / (1 - cos(atan(1 / cos(theta)))));
    /* Apply the result to the real area. */
    if (area.value === AREA_ENUM.AREA_1) {
      mu += HALF_PI;
    } else if (area.value === AREA_ENUM.AREA_2) {
      mu += SPI;
    } else if (area.value === AREA_ENUM.AREA_3) {
      mu += 1.5 * SPI;
    }
    /* Now compute x, y from mu and nu */
    xy.x = t * cos(mu);
    xy.y = t * sin(mu);
    xy.x = xy.x * this.a + this.x0;
    xy.y = xy.y * this.a + this.y0;
    p.x = xy.x;
    p.y = xy.y;
  }

  /**
   * QuadrilateralizedSphericalCube inverse equations--mapping x-y to lon-lat
   * @param p - QuadrilateralizedSphericalCube point
   */
  inverse(p: VectorPoint): void {
    const lp = { lam: 0, phi: 0 };
    let mu;
    let t;
    let phi;
    const area = { value: 0 };
    /* de-offset */
    p.x = (p.x - this.x0) / this.a;
    p.y = (p.y - this.y0) / this.a;
    /* Convert the input x, y to the mu and nu angles as used by QSC.
     * This depends on the area of the cube face. */
    const nu = atan(sqrt(p.x * p.x + p.y * p.y));
    mu = atan2(p.y, p.x);
    if (p.x >= 0.0 && p.x >= abs(p.y)) {
      area.value = AREA_ENUM.AREA_0;
    } else if (p.y >= 0.0 && p.y >= abs(p.x)) {
      area.value = AREA_ENUM.AREA_1;
      mu -= HALF_PI;
    } else if (p.x < 0.0 && -p.x >= abs(p.y)) {
      area.value = AREA_ENUM.AREA_2;
      mu = mu < 0.0 ? mu + SPI : mu - SPI;
    } else {
      area.value = AREA_ENUM.AREA_3;
      mu += HALF_PI;
    }
    /* Compute phi and theta for the area of definition.
     * The inverse projection is not described in the original paper, but some
     * good hints can be found here (as of 2011-12-14):
     * http://fits.gsfc.nasa.gov/fitsbits/saf.93/saf.9302
     * (search for "Message-Id: <9302181759.AA25477 at fits.cv.nrao.edu>") */
    t = (SPI / 12) * tan(mu);
    const tantheta = sin(t) / (cos(t) - 1 / sqrt(2));
    const theta = atan(tantheta);
    const cosmu = cos(mu);
    const tannu = tan(nu);
    let cosphi = 1 - cosmu * cosmu * tannu * tannu * (1 - cos(atan(1 / cos(theta))));
    if (cosphi < -1) {
      cosphi = -1;
    } else if (cosphi > +1) {
      cosphi = +1;
    }
    /* Apply the result to the real area on the cube face.
     * For the top and bottom face, we can compute phi and lam directly.
     * For the other faces, we must use unit sphere cartesian coordinates
     * as an intermediate step. */
    if (this.face === FACE_ENUM.TOP) {
      phi = acos(cosphi);
      lp.phi = HALF_PI - phi;
      if (area.value === AREA_ENUM.AREA_0) {
        lp.lam = theta + HALF_PI;
      } else if (area.value === AREA_ENUM.AREA_1) {
        lp.lam = theta < 0.0 ? theta + SPI : theta - SPI;
      } else if (area.value === AREA_ENUM.AREA_2) {
        lp.lam = theta - HALF_PI;
      } /* area.value == AREA_ENUM.AREA_3 */ else {
        lp.lam = theta;
      }
    } else if (this.face === FACE_ENUM.BOTTOM) {
      phi = acos(cosphi);
      lp.phi = phi - HALF_PI;
      if (area.value === AREA_ENUM.AREA_0) {
        lp.lam = -theta + HALF_PI;
      } else if (area.value === AREA_ENUM.AREA_1) {
        lp.lam = -theta;
      } else if (area.value === AREA_ENUM.AREA_2) {
        lp.lam = -theta - HALF_PI;
      } /* area.value == AREA_ENUM.AREA_3 */ else {
        lp.lam = theta < 0.0 ? -theta - SPI : -theta + SPI;
      }
    } else {
      /* Compute phi and lam via cartesian unit sphere coordinates. */
      let q, r, s;
      q = cosphi;
      t = q * q;
      if (t >= 1) {
        s = 0;
      } else {
        s = sqrt(1 - t) * sin(theta);
      }
      t += s * s;
      if (t >= 1) {
        r = 0;
      } else {
        r = sqrt(1 - t);
      }
      /* Rotate q,r,s into the correct area. */
      if (area.value === AREA_ENUM.AREA_1) {
        t = r;
        r = -s;
        s = t;
      } else if (area.value === AREA_ENUM.AREA_2) {
        r = -r;
        s = -s;
      } else if (area.value === AREA_ENUM.AREA_3) {
        t = r;
        r = s;
        s = -t;
      }
      /* Rotate q,r,s into the correct cube face. */
      if (this.face === FACE_ENUM.RIGHT) {
        t = q;
        q = -r;
        r = t;
      } else if (this.face === FACE_ENUM.BACK) {
        q = -q;
        r = -r;
      } else if (this.face === FACE_ENUM.LEFT) {
        t = q;
        q = r;
        r = -t;
      }
      /* Now compute phi and lam from the unit sphere coordinates. */
      lp.phi = acos(-s) - HALF_PI;
      lp.lam = atan2(r, q);
      if (this.face === FACE_ENUM.RIGHT) {
        lp.lam = qscShiftLonOrigin(lp.lam, -HALF_PI);
      } else if (this.face === FACE_ENUM.BACK) {
        lp.lam = qscShiftLonOrigin(lp.lam, -SPI);
      } else if (this.face === FACE_ENUM.LEFT) {
        lp.lam = qscShiftLonOrigin(lp.lam, HALF_PI);
      }
    }
    /* Apply the shift from the sphere to the ellipsoid as described
     * in [LK12]. */
    if (this.es !== 0) {
      const invert_sign = lp.phi < 0 ? 1 : 0;
      const tanphi = tan(lp.phi);
      const xa = this.b / sqrt(tanphi * tanphi + this.oneMinusFSquared);
      lp.phi = atan(sqrt(this.a * this.a - xa * xa) / (this.oneMinusF * xa));
      if (invert_sign !== 0) {
        lp.phi = -lp.phi;
      }
    }
    lp.lam += this.long0;
    p.x = lp.lam;
    p.y = lp.phi;
  }
}

/**
 * Helper function for forward projection: compute the theta angle
 * @param phi - phi
 * @param y - y
 * @param x - x
 * @param area - area
 * @returns - theta
 */
function qscFwdEquatFaceTheta(phi: number, y: number, x: number, area: Area): number {
  let theta;
  if (phi < EPSLN) {
    area.value = AREA_ENUM.AREA_0;
    theta = 0.0;
  } else {
    theta = atan2(y, x);
    if (abs(theta) <= QUART_PI) {
      area.value = AREA_ENUM.AREA_0;
    } else if (theta > QUART_PI && theta <= HALF_PI + QUART_PI) {
      area.value = AREA_ENUM.AREA_1;
      theta -= HALF_PI;
    } else if (theta > HALF_PI + QUART_PI || theta <= -(HALF_PI + QUART_PI)) {
      area.value = AREA_ENUM.AREA_2;
      theta = theta >= 0.0 ? theta - SPI : theta + SPI;
    } else {
      area.value = AREA_ENUM.AREA_3;
      theta += HALF_PI;
    }
  }
  return theta;
}

/**
 * Helper function: shift the longitude.
 * @param lon - longitude
 * @param offset - shift amount
 * @returns - shifted longitude
 */
function qscShiftLonOrigin(lon: number, offset: number): number {
  let slon = lon + offset;
  if (slon < -SPI) {
    slon += TWO_PI;
  } else if (slon > SPI) {
    slon -= TWO_PI;
  }

  return slon;
}
