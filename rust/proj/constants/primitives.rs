/// 3 parameter transform
pub const PJD_3PARAM: u8 = 1;
/// 7 parameter transform
pub const PJD_7PARAM: u8 = 2;
/// Grid shift transform
pub const PJD_GRIDSHIFT: u8 = 3;
/// WGS84 or equivalent
pub const PJD_WGS84: u8 = 4; // WGS84 or equivalent
/// No datum applied
pub const PJD_NODATUM: u8 = 5; // WGS84 or equivalent
/// SRS WGS84 semimajor only used in grid shift transforms
pub const SRS_WGS84_SEMIMAJOR: f64 = 6378137.0; // only used in grid shift transforms
/// SRS WGS84 semiminor only used in grid shift transforms
pub const SRS_WGS84_SEMIMINOR: f64 = 6356752.314; // only used in grid shift transforms
/// SRS WGS84 esquared only used in grid shift transforms
pub const SRS_WGS84_ESQUARED: f64 = 0.0066943799901413165; // only used in grid shift transforms
/// 1/4 π - Sec to Radians
pub const SEC_TO_RAD: f64 = 4.848_136_811_095_36e-6;
/// 1/2 π
pub const HALF_PI: f64 = core::f64::consts::FRAC_PI_2; // Math.PI / 2;
/// ellipoid pj_set_ell.c
pub const SIXTH: f64 = 0.166_666_666_666_666_66;
/// 1/6
pub const RA4: f64 = 0.047_222_222_222_222_22;
/// 17/360
pub const RA6: f64 = 0.022_156_084_656_084_655;

/// An arc minute is 1/60th of a degree [(π/180) / 60 radians]
pub const MIN2R: f64 = 0.0002908882086657216;
/// An arc second is 1/60th of an arc minute, or 1/3600th of a degree [(π/180) / 3600 radians]
pub const SEC2R: f64 = 0.00000484813681109536;
/// The grad, or gradian, is a unit of angular measure where a right angle is 100 grads,
/// and a full circle is 400 grads.
/// [1 grad = π/200 radians]
pub const GRD2R: f64 = 0.015707963267948967;
/// The gon is equivalent to the grad and is primarily used in surveying.
/// A full circle is 400 gons.
/// [1 gon = π/200 radians]
pub const GON2R: f64 = GRD2R;
/// 1/4 π
pub const QUART_PI: f64 = core::f64::consts::FRAC_PI_4; // Math.PI / 4;
/// 2 π
pub const TWO_PI: f64 = core::f64::consts::TAU; // Math.PI * 2;
/// SPI is slightly greater than Math.PI, so values that exceed the -180..180
/// degree range by a tiny amount don't get wrapped. This prevents points that
/// have drifted from their original location along the 180th meridian (due to
/// floating point error) from changing their sign.
#[allow(clippy::approx_constant)]
pub const SPI: f64 = 3.14159265359;

/// feet to meters
pub const FT_TO_M: f64 = 0.3048;
/// US feet to meters (1200 / 3937)
pub const US_FT_TO_M: f64 = 0.3048006096012192; // 1200 / 3937
/// US (modified) feet to meters (1200 / 3937)
pub const US_MOD_FT_TO_M: f64 = 0.304_812_252_984_506; // 1200 / 3937
/// Linear foot Clarke to meters
pub const CLARKE_FT_TO_M: f64 = 0.3047972654; // Linear_Foot_Clarke
/// Linear foot Indian to meters
pub const INDIAN_FT_TO_M: f64 = 0.3047995; // Linear_Foot_Indian
/// Linear foot Link to meters
pub const LINK_FT_TO_M: f64 = 0.201168; // Linear_Foot_Link
