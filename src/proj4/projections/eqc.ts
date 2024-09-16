import { ProjectionBase } from '.';
import { adjustLat, adjustLon } from '../common';

import type { VectorPoint } from 's2-tools/geometry';
import type { ProjectionParams, ProjectionTransform } from '.';

/**
 * # Equidistant Cylindrical (Plate Carrée)
 *
 * **Classification**: Conformal cylindrical
 *
 * **Available forms**: Forward and inverse
 *
 * **Defined area**: Global, but best used near the equator
 *
 * **Alias**: eqc, plate_carrée, simple_cylindrical
 *
 * **Domain**: 2D
 *
 * **Input type**: Geodetic coordinates
 *
 * **Output type**: Projected coordinates
 *
 * ## Projection String
 * ```
 * +proj=eqc
 * ```
 *
 * ## Usage
 *
 * Because of the distortions introduced by this projection, it has little use in navigation or
 * cadastral mapping and finds its main use in thematic mapping.
 * In particular, the Plate Carrée has become a standard for global raster datasets, such as
 * Celestia and NASA World Wind, because of the particularly simple relationship between the
 * position of an image pixel on the map and its corresponding geographic location on Earth.
 *
 * ### Special Cases of Cylindrical Equidistant Projection:
 *
 * - Plain/Plane Chart: 0°
 * - Simple Cylindrical: 0°
 * - Plate Carrée: 0°
 * - Ronald Miller—minimum overall scale distortion: 37°30'
 * - E. Grafarend and A. Niermann: 42°
 * - Ronald Miller—minimum continental scale distortion: 43°30'
 * - Gall Isographic: 45°
 * - Ronald Miller Equirectangular: 50°30'
 * - E. Grafarend and A. Niermann minimum linear distortion: 61°7'
 *
 * ## Example
 *
 * Example using EPSG 32662 (WGS84 Plate Carrée):
 * ```bash
 * echo 2 47 | proj +proj=eqc +ellps=WGS84
 * ```
 * Output: 222638.98 5232016.07
 *
 * Example using Plate Carrée projection with true scale at latitude 30° and central meridian 90°W:
 * ```bash
 * echo -88 30 | proj +proj=eqc +lat_ts=30 +lon_0=90w
 * ```
 * Output: 192811.01 3339584.72
 *
 * ## Parameters
 *
 * - `+lon_0` (Central meridian)
 * - `+lat_0` (Latitude of origin)
 * - `+lat_ts` (Latitude of true scale)
 * - `+x_0` (False easting)
 * - `+y_0` (False northing)
 * - `+ellps` (Ellipsoid name)
 * - `+R` (Radius of the sphere)
 *
 * ## Mathematical Definition
 *
 * ### Forward projection:
 * ```
 * x = λ * cos(φ_ts)
 * y = φ - φ_0
 * ```
 *
 * ### Inverse projection:
 * ```
 * λ = x / cos(φ_ts)
 * φ = y + φ_0
 * ```
 *
 * ## Further Reading
 *
 * - [Wikipedia](https://en.wikipedia.org/wiki/Equirectangular_projection)
 * - [Wolfram Mathworld](http://mathworld.wolfram.com/CylindricalEquidistantProjection.html)
 */
export class EquidistantCylindrical extends ProjectionBase implements ProjectionTransform {
  name = 'Equidistant Cylindrical (Plate Carre)';
  names = [this.name, 'Equirectangular', 'Equidistant_Cylindrical', 'eqc'];
  // EquidistantCylindricalProjection specific variables
  rc: number;

  /**
   * Preps an EquidistantCylindricalProjection projection
   * @param params - projection specific parameters
   */
  constructor(params?: ProjectionParams) {
    super(params);

    this.x0 = this.x0 || 0;
    this.y0 = this.y0 || 0;
    this.lat0 = this.lat0 || 0;
    this.long0 = this.long0 || 0;
    this.latTs = this.latTs || 0;

    this.rc = Math.cos(this.latTs);
  }

  /**
   * EquidistantCylindricalProjection forward equations--mapping lon-lat to x-y
   * @param p - lon-lat WGS84 point
   * @returns - EquidistantCylindricalProjection point
   */
  forward(p: VectorPoint): VectorPoint {
    const { x: lon, y: lat } = p;
    const dlon = adjustLon(lon - this.long0);
    const dlat = adjustLat(lat - this.lat0);
    p.x = this.x0 + this.a * dlon * this.rc;
    p.y = this.y0 + this.a * dlat;

    return p;
  }

  /**
   * EquidistantCylindricalProjection inverse equations--mapping x-y to lon-lat
   * @param p - EquidistantCylindricalProjection point
   * @returns - lon-lat WGS84 point
   */
  inverse(p: VectorPoint): VectorPoint {
    const { x, y } = p;
    p.x = adjustLon(this.long0 + (x - this.x0) / (this.a * this.rc));
    p.y = adjustLat(this.lat0 + (y - this.y0) / this.a);
    return p;
  }
}
