use crate::space::EARTH_RADIUS_KM;

/// Number of seconds in a day
pub const MINUTES_PER_DAY: f64 = 1440.0;
/// Number of seconds in an orbit (in km3 / s2)
pub const MU: f64 = 398600.8;
/// Number of seconds in a minute
/// 60.0 / f64::sqrt((EARTH_RADIUS_KM * EARTH_RADIUS_KM * EARTH_RADIUS_KM) / MU);
pub const XKE: f64 = 0.07436691613317342;
/// Velocity in km per second
pub const VKMPERSEC: f64 = (EARTH_RADIUS_KM * XKE) / 60.0;
/// Velocity in km per minute
pub const TUMIN: f64 = 1.0 / XKE;
/// J2
pub const J2: f64 = 0.001082616;
/// J3
pub const J3: f64 = -0.00000253881;
/// J4
pub const J4: f64 = -0.00000165597;
/// J3 / J2
pub const J3_J2: f64 = J3 / J2;
/// 2 / 3
pub const X2_3: f64 = 2.0 / 3.0;
