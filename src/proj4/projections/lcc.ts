import { ProjectionBase } from '.';
import { EPSLN, HALF_PI } from '../constants';
import { adjustLon, msfnz, phi2z, sign, tsfnz } from '../common';

import type { VectorPoint } from 's2-tools/geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

const { abs, pow, sin, cos, sqrt, atan2, log, PI } = Math;

/**
 * # Lambert Conformal Conic
 *
 *  Lambert Conformal Conic projection (LCC) is a conic map projection
 * used for aeronautical charts, portions of the State Plane Coordinate
 * System, and many national and regional mapping systems. It is one of
 * seven projections introduced by Johann Heinrich Lambert in 1772.
 *
 * It has several different forms: with one and two standard parallels
 * (referred to as 1SP and 2SP in EPSG guidance notes). Additionally we
 * provide "2SP Michigan" form which is very similar to normal 2SP, but
 * with a scaling factor on the ellipsoid (given as `k_0` parameter).
 * It is implemented as per EPSG Guidance Note 7-2 (version 54, August
 * 2018, page 25). It is used in a few systems in the EPSG database which
 * justifies adding this otherwise non-standard projection.
 *
 * **Classification**: Conformal conic
 *
 * **Available forms**: Forward and inverse, spherical and ellipsoidal
 * - One or two standard parallels (1SP and 2SP).
 * - "LCC 2SP Michigan" form can be used by setting the `+k_0` parameter to specify ellipsoid scale.
 *
 * **Defined area**: Best for regions predominantly east-west in extent and located in the middle north or south latitudes.
 *
 * **Alias**: lcc
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=lcc +lon_0=-90 +lat_1=33 +lat_2=45
 * ```
 *
 * ## Required Parameters
 * - `+lat_1`: Latitude of the first standard parallel.
 *
 * ## Optional Parameters
 * - `+lon_0`: Longitude of projection center. Defaults to `0`.
 * - `+lat_0`: Latitude of projection center. Defaults to `0`.
 * - `+lat_2`: Latitude of the second standard parallel.
 * - `+ellps`: Ellipsoid. Defaults to `WGS84`.
 * - `+R`: Radius of the sphere.
 * - `+x_0`: False easting. Defaults to `0`.
 * - `+y_0`: False northing. Defaults to `0`.
 * - `+k_0`: Scale factor at natural origin (for LCC 1SP) or ellipsoid scale factor (for LCC 2SP Michigan). Defaults to `1.0`.
 *
 * ![Lambert Conformal Conic](./images/lcc.png)
 *
 * ## Further reading
 * - [Wikipedia on Lambert Conformal Conic](https://en.wikipedia.org/wiki/Lambert_conformal_conic_projection)
 * - [Wolfram Mathworld on Lambert Conformal Conic](http://mathworld.wolfram.com/LambertConformalConicProjection.html)
 * - [John P. Snyder "Map projections: A working manual"](https://pubs.er.usgs.gov/publication/pp1395)
 * - [ArcGIS documentation on "Lambert Conformal Conic"](http://desktop.arcgis.com/en/arcmap/10.3/guide-books/map-projections/lambert-conformal-conic.htm)
 * - [EPSG Guidance Note 7-2](http://www.epsg.org/Guidancenotes.aspx)
 */
export class LambertConformalConic extends ProjectionBase implements ProjectionTransform {
  name = 'LambertConformalConic';
  static names = [
    'LambertConformalConic',
    'Lambert Tangential Conformal Conic Projection',
    'Lambert_Conformal_Conic',
    'Lambert_Conformal_Conic_1SP',
    'Lambert_Conformal_Conic_2SP',
    'lcc',
    'Lambert Conic Conformal (1SP)',
    'Lambert Conic Conformal (2SP)',
  ];
  // LambertConformalConic specific variables
  ns = 0;
  f0 = 0;
  rh = 0;

  /**
   * Preps an LambertConformalConic projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);

    // double lat0;                    /* the reference latitude               */
    // double long0;                   /* the reference longitude              */
    // double lat1;                    /* first standard parallel              */
    // double lat2;                    /* second standard parallel             */
    // double r_maj;                   /* major axis                           */
    // double r_min;                   /* minor axis                           */
    // double false_east;              /* x offset in meters                   */
    // double false_north;             /* y offset in meters                   */
    //the above value can be set with proj4.defs
    //example: proj4.defs("EPSG:2154","+proj=lcc +lat_1=49 +lat_2=44 +lat_0=46.5 +lon_0=3 +x_0=700000 +y_0=6600000 +ellps=GRS80 +towgs84=0,0,0,0,0,0,0 +units=m +no_defs");

    if (this.lat2 === 0) this.lat2 = this.lat1;
    this.x0 = this.x0 ?? 0;
    this.y0 = this.y0 ?? 0;
    // Standard Parallels cannot be equal and on opposite sides of the equator
    if (abs(this.lat1 + this.lat2) < EPSLN) {
      return;
    }

    const temp = this.b / this.a;
    this.e = sqrt(1 - temp * temp);

    const sin1 = sin(this.lat1);
    const cos1 = cos(this.lat1);
    const ms1 = msfnz(this.e, sin1, cos1);
    const ts1 = tsfnz(this.e, this.lat1, sin1);

    const sin2 = sin(this.lat2);
    const cos2 = cos(this.lat2);
    const ms2 = msfnz(this.e, sin2, cos2);
    const ts2 = tsfnz(this.e, this.lat2, sin2);

    const ts0 = tsfnz(this.e, this.lat0, sin(this.lat0));

    if (abs(this.lat1 - this.lat2) > EPSLN) {
      this.ns = log(ms1 / ms2) / log(ts1 / ts2);
    } else {
      this.ns = sin1;
    }
    if (isNaN(this.ns)) {
      this.ns = sin1;
    }
    this.f0 = ms1 / (this.ns * pow(ts1, this.ns));
    this.rh = this.a * this.f0 * pow(ts0, this.ns);
  }

  /**
   * LambertConformalConic forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    const lon = p.x;
    let lat = p.y;
    // singular cases :
    if (abs(2 * abs(lat) - PI) <= EPSLN) {
      lat = sign(lat) * (HALF_PI - 2 * EPSLN);
    }
    let con = abs(abs(lat) - HALF_PI);
    let ts, rh1;
    if (con > EPSLN) {
      ts = tsfnz(this.e, lat, sin(lat));
      rh1 = this.a * this.f0 * pow(ts, this.ns);
    } else {
      con = lat * this.ns;
      if (con <= 0) {
        throw new Error('latitude out of range');
      }
      rh1 = 0;
    }
    const theta = this.ns * adjustLon(lon - this.long0);
    p.x = this.k0 * (rh1 * sin(theta)) + this.x0;
    p.y = this.k0 * (this.rh - rh1 * cos(theta)) + this.y0;
  }

  /**
   * LambertConformalConic inverse equations--mapping x-y to lon-lat
   * @param p - LambertConformalConic point
   */
  inverse(p: VectorPoint): void {
    let rh1, con, ts;
    let lat;
    const x = (p.x - this.x0) / this.k0;
    const y = this.rh - (p.y - this.y0) / this.k0;
    if (this.ns > 0) {
      rh1 = sqrt(x * x + y * y);
      con = 1;
    } else {
      rh1 = -sqrt(x * x + y * y);
      con = -1;
    }
    let theta = 0;
    if (rh1 !== 0) {
      theta = atan2(con * x, con * y);
    }
    if (rh1 !== 0 || this.ns > 0) {
      con = 1 / this.ns;
      ts = pow(rh1 / (this.a * this.f0), con);
      lat = phi2z(this.e, ts);
      if (lat === -9999) {
        throw new Error('Invalid latitude');
      }
    } else {
      lat = -HALF_PI;
    }
    const lon = adjustLon(theta / this.ns + this.long0);
    p.x = lon;
    p.y = lat;
  }
}
