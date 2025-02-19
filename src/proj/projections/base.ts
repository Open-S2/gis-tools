import { CoordOperation } from '.';
import { D2R, GridDescription, PJD_UNKNOWN, PJ_TYPE, R2D, pjIoUnits } from '../constants';

import type { DatumParams } from '../../readers/wkt';
import type { ProjectionTransform } from '.';
import type { VectorPoint } from '../../geometry';

/** Define the projection with all it's variable components */
export interface ProjectionParams {
  todo?: string;
}

/** Base class for all projections */
export class ProjectionBase implements ProjectionTransform {
  shortName = 'longlat';
  static names: string[] = ['longlat', 'identity'];
  descr = '';
  params: ProjectionParams = {};
  /** Parent PJ of pipeline steps - undefined if not a pipeline step */
  parent?: ProjectionTransform;

  /*************************************************************************************

                          E L L I P S O I D     P A R A M E T E R S

    **************************************************************************************

        Despite YAGNI, we add a large number of ellipsoidal shape parameters,
    which are not yet set up in pj_init. They are, however, inexpensive to
    compute, compared to the overall time taken for setting up the complex PJ
    object (cf. e.g. https://en.wikipedia.org/wiki/Angular_eccentricity).

        But during single point projections it will often be a useful thing to
    have these readily available without having to recompute at every pj_fwd /
    pj_inv call.

        With this wide selection, we should be ready for quite a number of
    geodetic algorithms, without having to incur further ABI breakage.

    **************************************************************************************/

  /** The linear parameters */

  /** (F64) semimajor axis (radius if eccentricity==0) */
  a = 0;
  /** (F64) semiminor axis */
  b = 0;
  /** (F64) 1 / a */
  ra = 0;
  /** (F64) 1 / b */
  rb = 0;

  /** The eccentricities */

  /** (F64) angular eccentricity */
  alpha = 0;
  /** (F64) first  eccentricity */
  e = 0;
  /** (F64) first  eccentricity squared */
  es = 0;
  /** (F64) second eccentricity */
  e2 = 0;
  /** (F64) second eccentricity squared */
  e2s = 0;
  /** (F64) third  eccentricity */
  e3 = 0;
  /** (F64) third  eccentricity squared */
  e3s = 0;
  /** (F64) 1 - e^2 */
  oneEs = 0;
  /** (F64) 1 / oneEs */
  roneEs = 0;

  /** The flattenings */

  /** (F64) first  flattening */
  f = 0;
  /** (F64) second flattening */
  f2 = 0;
  /** (F64) third  flattening */
  n = 0;
  /** (F64) 1 / f  */
  rf = 0;
  /** (F64) 1 / f2 */
  rf2 = 0;
  /** (F64) 1 / n  */
  rn = 0;

  /** This one's for GRS80 */
  /** (F64) "Dynamic form factor" */
  J = 0;
  /** (F64) es and a before any +proj related adjustment */
  esOrig = 0;
  /** (F64) semimajor axis before any +proj related adjustment */
  aOrig = 0;

  /*************************************************************************************

                          C O O R D I N A T E   H A N D L I N G

    **************************************************************************************/

  /** (I32) Over-range flag */
  over = 0;
  /** (I32) Geocentric latitude flag */
  geoc = 0;
  /** (I32) proj=latlong ... not really a projection at all */
  is_latlong = 0;
  /** (I32) proj=geocent ... not really a projection at all */
  is_geocent = 0;
  /** (I32) 0 for operations that are purely cartesian */
  need_ellps = 0;
  /** (I32) Whether to skip forward prepare */
  skip_fwd_prepare = 0;
  /** (I32) Whether to skip forward finalize */
  skip_fwd_finalize = 0;
  /** (I32) Whether to skip inverse prepare */
  skip_inv_prepare = 0;
  /** (I32) Whether to skip inverse finalize */
  skip_inv_finalize = 0;

  /** Flags for input/output coordinate types */
  left = pjIoUnits.PJ_IO_UNITS_WHATEVER;
  right = pjIoUnits.PJ_IO_UNITS_WHATEVER;

  /** These PJs are used for implementing cs2cs style coordinate handling in the 4D API */
  // TODO:!!!!!!!
  // PJ *axisswap = nullptr;
  // PJ *cart = nullptr;
  // PJ *cart_wgs84 = nullptr;
  // PJ *helmert = nullptr;
  // PJ *hgridshift = nullptr;
  // PJ *vgridshift = nullptr;

  /*************************************************************************************

                       C A R T O G R A P H I C       O F F S E T S

    **************************************************************************************/

  /** (F64) central meridian */
  lam0 = 0;
  /** (F64) central parallel */
  phi0 = 0;
  /** (F64) false easting */
  x0 = 0;
  /** (F64) false northing  */
  y0 = 0;
  /** (F64) height origin */
  z0 = 0;
  /** (F64) time origin */
  t0 = 0;

  /*************************************************************************************
 
                                  S C A L I N G

  **************************************************************************************/

  /** (F64) General scaling factor - e.g. the 0.9996 of UTM */
  k0 = 0;
  /** (F64) Plane coordinate scaling. Internal unit [m] */
  to_meter = 0;
  /** (F64) Plane coordinate scaling. Internal unit [m] */
  fr_meter = 0;
  /** (F64) Vertical scaling. Internal unit [m] */
  vto_meter = 0;
  /** (F64) Vertical scaling. Internal unit [m] */
  vfr_meter = 0;

  /*************************************************************************************
 
                D A T U M S   A N D   H E I G H T   S Y S T E M S

  **************************************************************************************

      It may be possible, and meaningful, to move the list parts of this up to
  the PJ_CONTEXT level.

  **************************************************************************************/

  datum_type = PJD_UNKNOWN; /* PJD_UNKNOWN/3PARAM/7PARAM/GRIDSHIFT/WGS84 */
  datum_params: DatumParams = [0, 0, 0, 0, 0, 0, 0]; /* (F64) Parameters for 3PARAM and 7PARAM */

  has_geoid_vgrids = 0; /* (I32) used by legacy transform.cpp */

  from_greenwich = 0; /* (F64) prime meridian offset (in radians) */
  long_wrap_center = 0; /* (F64) 0.0 for -180 to 180, actually in radians*/
  is_long_wrap_set = 0; /* (I32) 1 if long_wrap_center is set */
  axis: [number, number, number, number] = [
    0, 0, 0, 0,
  ]; /* Axis order, pj_transform/pj_adjust_axis */

  /*************************************************************************************
  ISO-19111 interface
  **************************************************************************************/

  iso_obj?: Record<string, unknown>;
  iso_obj_is_coordinate_operation = false;
  coordinateEpoch = 0; // F64
  hasCoordinateEpoch = false;

  // cached results
  lastWKT?: string;
  lastPROJString?: string;
  lastJSONString?: string;
  gridsNeededAsked = false;
  gridsNeeded: GridDescription[] = [];

  // cache pj_get_type() result to help for repeated calls to proj_factors()
  type = PJ_TYPE.PJ_TYPE_UNKNOWN;

  /*************************************************************************************
  proj_create_crs_to_crs() alternative coordinate operations
  **************************************************************************************/
  alternativeCoordinateOperations?: CoordOperation[];
  iCurCoordOp = -1; // I32
  errorIfBestTransformationNotAvailable = false;
  warnIfBestTransformationNotAvailable = true; /* to remove in PROJ 10? */
  skipNonInstantiable = true;

  // Used internally by proj_factors()
  // cached_op_for_proj_factors?: PJ;

  /** @param params - projection specific parameters */
  constructor(params?: ProjectionParams) {
    Object.assign(this, params ?? {});
  }

  /**
   * Forward projection from x-y to lon-lat. In this case, radians to degrees
   * @param p - Vector Point. This is a placeholder for a lon-lat WGS84 point
   */
  forward(p: VectorPoint): void {
    p.x *= R2D;
    p.y *= R2D;
  }

  /**
   * Inverse projection from lon-lat to x-y. In this case, degrees to radians
   * @param p - Vector Point. This is a placeholder for a lon-lat WGS84 point
   */
  inverse(p: VectorPoint): void {
    p.x *= D2R;
    p.y *= D2R;
  }

  // TODO: SUpport other kinds of forward/inverse mutations
  // PJ_XY (*fwd)(PJ_LP, PJ *) = nullptr;
  // PJ_LP (*inv)(PJ_XY, PJ *) = nullptr;
  // PJ_XYZ (*fwd3d)(PJ_LPZ, PJ *) = nullptr;
  // PJ_LPZ (*inv3d)(PJ_XYZ, PJ *) = nullptr;
}
