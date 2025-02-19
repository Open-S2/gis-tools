/**
 *
 */
export enum pjIoUnits {
  PJ_IO_UNITS_WHATEVER = 0 /* Doesn't matter (or depends on pipeline neighbours) */,
  PJ_IO_UNITS_CLASSIC = 1 /* Scaled meters (right), projected system */,
  PJ_IO_UNITS_PROJECTED = 2 /* Meters, projected system */,
  PJ_IO_UNITS_CARTESIAN = 3 /* Meters, 3D cartesian system */,
  PJ_IO_UNITS_RADIANS = 4 /* Radians */,
  PJ_IO_UNITS_DEGREES = 5 /* Degrees */,
}

/** \brief Object category. */
export enum PJ_CATEGORY {
  PJ_CATEGORY_ELLIPSOID,
  PJ_CATEGORY_PRIME_MERIDIAN,
  PJ_CATEGORY_DATUM,
  PJ_CATEGORY_CRS,
  PJ_CATEGORY_COORDINATE_OPERATION,
  PJ_CATEGORY_DATUM_ENSEMBLE,
}

/**
 *
 */
export enum PJ_TYPE {
  PJ_TYPE_UNKNOWN,

  PJ_TYPE_ELLIPSOID,

  PJ_TYPE_PRIME_MERIDIAN,

  PJ_TYPE_GEODETIC_REFERENCE_FRAME,
  PJ_TYPE_DYNAMIC_GEODETIC_REFERENCE_FRAME,
  PJ_TYPE_VERTICAL_REFERENCE_FRAME,
  PJ_TYPE_DYNAMIC_VERTICAL_REFERENCE_FRAME,
  PJ_TYPE_DATUM_ENSEMBLE,

  /** Abstract type, not returned by proj_get_type() */
  PJ_TYPE_CRS,

  PJ_TYPE_GEODETIC_CRS,
  PJ_TYPE_GEOCENTRIC_CRS,

  /**
   * proj_get_type() will never return that type, but
   * PJ_TYPE_GEOGRAPHIC_2D_CRS or PJ_TYPE_GEOGRAPHIC_3D_CRS.
   */
  PJ_TYPE_GEOGRAPHIC_CRS,

  PJ_TYPE_GEOGRAPHIC_2D_CRS,
  PJ_TYPE_GEOGRAPHIC_3D_CRS,
  PJ_TYPE_VERTICAL_CRS,
  PJ_TYPE_PROJECTED_CRS,
  PJ_TYPE_COMPOUND_CRS,
  PJ_TYPE_TEMPORAL_CRS,
  PJ_TYPE_ENGINEERING_CRS,
  PJ_TYPE_BOUND_CRS,
  PJ_TYPE_OTHER_CRS,

  PJ_TYPE_CONVERSION,
  PJ_TYPE_TRANSFORMATION,
  PJ_TYPE_CONCATENATED_OPERATION,
  PJ_TYPE_OTHER_COORDINATE_OPERATION,

  PJ_TYPE_TEMPORAL_DATUM,
  PJ_TYPE_ENGINEERING_DATUM,
  PJ_TYPE_PARAMETRIC_DATUM,

  PJ_TYPE_DERIVED_PROJECTED_CRS,

  PJ_TYPE_COORDINATE_METADATA,
}

/**
 *
 */
export interface PJ_DATUMS {
  /** datum keyword */
  id: string;
  /** ie. "to_wgs84=..." */
  defn: string;
  /** ie from ellipse table */
  ellipse_id: string;
  /** EPSG code, etc */
  comments: string;
}

/**
 *
 */
export interface DERIVS {
  /** (F64) derivatives of x for lambda-phi */
  x_l: number;
  /** (F64) derivatives of x for lambda-phi */
  x_p: number;
  /** (F64) derivatives of y for lambda-phi */
  y_l: number;
  /** (F64) derivatives of y for lambda-phi */
  y_p: number;
}

/**
 *
 */
export interface FACTORS {
  der: DERIVS;
  /** (F64) meridional */
  h: number;
  /** (F64) parallel scales */
  k: number;
  /** (F64) angular distortion */
  omega: number;
  /** (F64) angular theta prime */
  thetap: number;
  /** (F64) convergence */
  conv: number;
  /** (F64) areal scale factor */
  s: number;
  /** (F64) max-min scale error */
  a: number;
  /** (F64) max-min scale error */
  b: number;
  /** (I32) always 0 */
  code: number;
}

/** \brief Grid description */
export interface GridDescription {
  /** Grid short filename */
  shortName: string;
  /** Grid full path name (if found) */
  fullName: string;
  /** Package name (or empty) */
  packageName: string;
  /** Grid URL (if packageName is empty), or package URL (or empty) */
  url: string;
  /** Whether url can be fetched directly. */
  directDownload: boolean;
  /** Whether the grid is released with an open license. */
  openLicense: boolean;
  /** Whether GRID is available. */
  available: boolean;
}
