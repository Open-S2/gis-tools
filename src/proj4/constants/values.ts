/* eslint-disable no-loss-of-precision */
export const PJD_3PARAM = 1;
export const PJD_7PARAM = 2;
export const PJD_GRIDSHIFT = 3;
export const PJD_WGS84 = 4; // WGS84 or equivalent
export const PJD_NODATUM = 5; // WGS84 or equivalent
export const SRS_WGS84_SEMIMAJOR = 6378137.0; // only used in grid shift transforms
export const SRS_WGS84_SEMIMINOR = 6356752.314; // only used in grid shift transforms
export const SRS_WGS84_ESQUARED = 0.0066943799901413165; // only used in grid shift transforms
export const SEC_TO_RAD = 4.84813681109535993589914102357e-6;
export const HALF_PI = Math.PI / 2;
// ellipoid pj_set_ell.c
export const SIXTH = 0.1666666666666666667;
/* 1/6 */
export const RA4 = 0.04722222222222222222;
/* 17/360 */
export const RA6 = 0.02215608465608465608;
export const EPSLN = 1.0e-10;
// you'd think you could use Number.EPSILON above but that makes
// Mollweide get into an infinate loop.

/** 1 degree = π/180 radians */
export const D2R = 0.01745329251994329577;
/** 1 radian = 180/pi degrees */
export const R2D = 57.29577951308232088;
/** An arc minute is 1/60th of a degree [(π/180) / 60 radians] */
export const MIN2R = 0.0002908882086657216;
/** An arc second is 1/60th of an arc minute, or 1/3600th of a degree [(π/180) / 3600 radians] */
export const SEC2R = 0.00000484813681109536;
/**
 * The grad, or gradian, is a unit of angular measure where a right angle is 100 grads,
 * and a full circle is 400 grads.
 * [1 grad = π/200 radians]
 */
export const GRD2R = 0.015707963267948967;
/**
 * The gon is equivalent to the grad and is primarily used in surveying.
 * A full circle is 400 gons.
 * [1 gon = π/200 radians]
 */
export const GON2R = GRD2R;
/** 1/4 π */
export const QUART_PI = Math.PI / 4;
/** 2 π */
export const TWO_PI = Math.PI * 2;
// SPI is slightly greater than Math.PI, so values that exceed the -180..180
// degree range by a tiny amount don't get wrapped. This prevents points that
// have drifted from their original location along the 180th meridian (due to
// floating point error) from changing their sign.
export const SPI = 3.14159265359;

export const FT_TO_M = 0.3048;
export const US_FT_TO_M = 1200 / 3937;
export const US_MOD_FT_TO_M = 0.30481225298450597;
