/* eslint-disable no-loss-of-precision */

/** Maximum latitudinal overshoot accepted */
export const PJ_EPS_LAT = 1e-12;

export const M_PI = 3.14159265358979323846;
export const M_1_PI = 0.318309886183790671538;
export const M_PI_2 = 1.57079632679489661923;
export const M_PI_4 = 0.78539816339744830962;
export const M_2_PI = 0.63661977236758134308;
export const M_SQRT2 = 1.4142135623730950488;
export const M_FORTPI = M_PI_4; /* pi/4 */
export const M_HALFPI = M_PI_2; /* pi/2 */
export const M_PI_HALFPI = 4.71238898038468985769; /* 1.5*pi */
export const M_TWOPI = 6.28318530717958647693; /* 2*pi */
export const M_TWO_D_PI = M_2_PI; /* 2/pi */
export const M_TWOPI_HALFPI = 7.85398163397448309616; /* 2.5*pi */

/** datum_type values */
export const PJD_UNKNOWN = 0;
export const PJD_3PARAM = 1;
export const PJD_7PARAM = 2;
export const PJD_GRIDSHIFT = 3;
export const PJD_WGS84 = 4; /* WGS84 (or anything considered equivalent) */

export const GRAD_TO_RAD = 0.015707963267948967;
export const RAD_TO_DEG = 57.295779513082321;
export const R2D = RAD_TO_DEG;
export const DEG_TO_RAD = 0.017453292519943296;
export const D2R = DEG_TO_RAD;

export const LinearUnitDesc = {
  mm: 0.001, // EPSG: 1025 (millimetre)
  cm: 0.01, // EPSG: 1033 (centimetre)
  m: 1.0, // EPSG: 9001 (metre)
  meter: 1.0, // EPSG: 9001 (metre)
  metre: 1.0, // EPSG: 9001 (metre)
  ft: 0.3048, // EPSG: 9002 (foot)
  'us-ft': 0.3048006096012192, // EPSG: 9003 (US survey foot)
  fath: 1.8288, // EPSG: 9014 (fathom)
  kmi: 1852, // EPSG: 9030 (nautical mile)
  'us-ch': 20.11684023368047, // EPSG: 9033 (US survey chain)
  'us-mi': 1609.347218694437, // EPSG: 9035 (US survey mile)
  km: 1000.0, // EPSG: 9036 (kilometre)
  'ind-ft': 0.30479841, // EPSG: 9081 (Indian foot (1937))
  'ind-yd': 0.91439523, // EPSG: 9085 (Indian yard (1937))
  mi: 1609.344, // EPSG: 9093 (Statute mile)
  yd: 0.9144, // EPSG: 9096 (yard)
  ch: 20.1168, // EPSG: 9097 (chain)
  link: 0.201168, // EPSG: 9098 (link)
  dm: 0.1, // EPSG: 0 (no EPSG equivalent) (decimetre)
  in: 0.0254, // EPSG: 0 (no EPSG equivalent) (inch)
  'us-in': 0.025400050800101, // EPSG: 0 (no EPSG equivalent) (US survey inch)
  'us-yd': 0.914401828803658, // EPSG: 0 (no EPSG equivalent) (US survey yard)
  'ind-ch': 20.11669506, // EPSG: 0 (no EPSG equivalent) (Indian chain)
} as const;
