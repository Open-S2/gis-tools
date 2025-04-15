use super::{get_ellipsoid, RA4, RA6, SIXTH, WGS84};
use alloc::string::String;
use libm::sqrt;

/// Describes an ellipsoid's eccentricity
#[derive(Debug, Default, Clone)]
pub struct EccentricityParams {
    /// Semi-major axis
    pub a: Option<f64>,
    /// Semi-minor axis
    pub b: Option<f64>,
    /// Eccentricity
    pub es: Option<f64>,
    /// Eccentricity
    pub e: Option<f64>,
    /// Second eccentricity
    pub ep2: Option<f64>,
    /// True sphere
    pub r_a: bool,
}

/// Derives an ellipsoid's eccentricity for an object
/// @param el - ellipsoid object to modify
pub fn derive_eccentricity(el: &mut EccentricityParams) {
    let mut a = el.a.unwrap_or(0.0);
    let b = el.b.unwrap_or(0.0);
    let mut a2 = a * a; // used in geocentric
    let b2 = b * b; // used in geocentric
    let mut es = (a2 - b2) / a2; // e ^ 2
    let mut e = 0.0;
    if el.r_a {
        a *= 1. - es * (SIXTH + es * (RA4 + es * RA6));
        a2 = a * a;
        es = 0.;
    } else {
        e = sqrt(es); // eccentricity
    }
    let ep2 = (a2 - b2) / b2; // used in geocentric

    el.es = Some(es);
    el.e = Some(e);
    el.ep2 = Some(ep2);
}

/// Describes a sphere's eccentricity and if it is a true sphere or not
#[derive(Debug, Default, Clone)]
pub struct SphereParams {
    /// Ellipsoid name
    pub ellps: Option<String>,
    /// Semi-major axis
    pub a: Option<f64>,
    /// Semi-minor axis
    pub b: Option<f64>,
    /// Eccentricity
    pub rf: Option<f64>,
    /// True sphere
    pub sphere: Option<bool>,
}

/// Builds a sphere with ellipsoid parameters
/// @param obj - an object with/wihtout sphere properties and builds the sphere
pub fn derive_sphere(obj: &mut SphereParams) {
    if obj.a.is_none() {
        // do we have an ellipsoid?
        let ellps = obj.ellps.clone().unwrap_or("".into());
        let ellipse = get_ellipsoid(ellps.as_str());
        let ellipse = ellipse.unwrap_or(WGS84);
        obj.a = Some(ellipse.a);
        obj.b = ellipse.b;
        obj.rf = ellipse.rf;
    }
    if obj.b.is_none() && obj.rf.is_some() {
        obj.b = Some((1.0 - 1.0 / obj.rf.unwrap_or(0.0)) * obj.a.unwrap_or(0.0));
    }
    if obj.b.is_some() && obj.rf.is_none() {
        let a = obj.a.unwrap_or(0.0);
        let b = obj.b.unwrap_or(0.0);
        obj.rf = Some((a - b) / a);
    }
    let rf = obj.rf.unwrap_or(0.0);
    let a = obj.a.unwrap_or(0.0);
    if rf == 0.0 || (obj.b.is_some() && (a - obj.b.unwrap_or(0.0)).abs() < f64::EPSILON) {
        obj.sphere = Some(true);
        obj.b = obj.a;
    }
}
