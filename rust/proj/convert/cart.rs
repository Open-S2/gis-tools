use crate::proj::{
    CoordinateStep, IoUnits, Proj, ProjectCoordinates, ProjectionTransform, Step,
    TransformCoordinates,
};
use alloc::rc::Rc;
use core::{cell::RefCell, f64::consts::FRAC_PI_2};
use libm::{atan, atan2, cos, fabs, sin, sqrt};

/// # CARTESIAN / GEODETIC CONVERSIONS
///
/// This material follows:
///
/// Bernhard Hofmann-Wellenhof & Helmut Moritz:
/// Physical Geodesy, 2nd edition.
/// Springer, 2005.
///
/// chapter 5.6: Coordinate transformations
/// (HM, below),
///
/// and
///
/// Wikipedia: Geographic Coordinate Conversion,
/// <https://en.wikipedia.org/wiki/Geographic_coordinate_conversion>
///
/// (WP, below).
///
/// The cartesian-to-geodetic conversion is based on Bowring's
/// celebrated method:
///
/// B. R. Bowring:
/// Transformation from spatial to geographical coordinates
/// Survey Review 23(181), pp. 323-327, 1976
///
/// (BB, below),
///
/// but could probably use some TLC from a newer and faster
/// algorithm:
///
/// Toshio Fukushima:
/// Transformation from Cartesian to Geodetic Coordinates
/// Accelerated by Halley’s Method
/// Journal of Geodesy, February 2006
///
/// (TF, below).
///
/// Close to the poles, we avoid singularities by switching to an
/// approximation requiring knowledge of the geocentric radius
/// at the given latitude. For this, we use an adaptation of the
/// formula given in:
///
/// Wikipedia: Earth Radius
/// <https://en.wikipedia.org/wiki/Earth_radius#Radius_at_a_given_geodetic_latitude>
/// (Derivation and commentary at <https://gis.stackexchange.com/q/20200>)
///
/// (WP2, below)
///
/// These routines are probably not as robust at those in
/// geocent.c, at least they haven't been through as heavy
/// use as their geocent sisters. Some care has been taken
/// to avoid singularities, but extreme cases (e.g. setting
/// es, the squared eccentricity, to 1), will cause havoc.
#[derive(Debug, Clone, PartialEq)]
pub struct CartesianConverter {
    proj: Rc<RefCell<Proj>>,
}
impl ProjectCoordinates for CartesianConverter {
    fn code(&self) -> i64 {
        -1
    }

    fn name(&self) -> &'static str {
        "cartesian"
    }

    fn names() -> &'static [&'static str] {
        &["cartesian", "cart"]
    }
}
impl CoordinateStep for CartesianConverter {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        {
            let proj = &mut proj.borrow_mut();
            proj.left = IoUnits::RADIANS;
            proj.right = IoUnits::CARTESIAN;
            proj.is_ll = true;
        }
        CartesianConverter { proj }
    }
    /// Geographical to geocentric
    fn forward<P: TransformCoordinates>(&self, coords: &mut P) {
        cartesian(&self.proj.borrow(), coords);
    }
    /// Geocentric to geographical
    fn inverse<P: TransformCoordinates>(&self, coords: &mut P) {
        geodetic(&self.proj.borrow(), coords);
    }
}
impl From<CartesianConverter> for ProjectionTransform {
    fn from(c: CartesianConverter) -> ProjectionTransform {
        ProjectionTransform {
            proj: c.proj.clone(),
            method: Step::Cartesian(c.into()),
            ..Default::default()
        }
    }
}

/// Return the normal radius of curvature of an ellipsoid
/// with semimajor axis a and squared eccentricity es.
pub fn normal_radius_of_curvature(a: f64, es: f64, sinphi: f64) -> f64 {
    if es == 0. {
        return a;
    }
    // This is from WP.  HM formula 2-149 gives an a,b version
    a / sqrt(1. - es * sinphi * sinphi)
}

/// Return the geocentric radius at latitude phi, of an ellipsoid
/// with semimajor axis a and semiminor axis b.
///
/// This is from WP2, but uses hypot() for potentially better
/// numerical robustness
pub fn geocentric_radius(a: f64, b_div_a: f64, cosphi: f64, sinphi: f64) -> f64 {
    // Non-optimized version:
    // let b = a * b_div_a;
    // return hypot(a * a * cosphi, b * b * sinphi) /
    //        hypot(a * cosphi, b * sinphi);
    let cosphi_squared = cosphi * cosphi;
    let sinphi_squared = sinphi * sinphi;
    let b_div_a_squared = b_div_a * b_div_a;
    let b_div_a_squared_mul_sinphi_squared = b_div_a_squared * sinphi_squared;
    a * sqrt(
        (cosphi_squared + b_div_a_squared * b_div_a_squared_mul_sinphi_squared)
            / (cosphi_squared + b_div_a_squared_mul_sinphi_squared),
    )
}

/// Cartesian to geodetic
pub fn cartesian<P: TransformCoordinates>(proj: &Proj, coords: &mut P) {
    let cosphi: f64 = cos(coords.phi());
    let sinphi = sin(coords.phi());
    let n = normal_radius_of_curvature(proj.a, proj.es, sinphi);

    // HM formula 5-27 (z formula follows WP)
    let z = coords.z();
    let lam = coords.lam();
    coords.set_x((n + z) * cosphi * cos(lam));
    coords.set_y((n + z) * cosphi * sin(lam));
    coords.set_z((n * (1. - proj.es) + z) * sinphi);
}

/// Geodetic to cartesian
pub fn geodetic<P: TransformCoordinates>(proj: &Proj, coords: &mut P) {
    let phi;
    // Normalize (x,y,z) to the unit sphere/ellipsoid.
    let x_div_a = coords.x() * proj.ra;
    let y_div_a = coords.y() * proj.ra;
    let z_div_a = coords.z() * proj.ra;

    // Perpendicular distance from point to Z-axis (HM eq. 5-28)
    let p_div_a = sqrt(x_div_a * x_div_a + y_div_a * y_div_a);

    let b_div_a = 1. - proj.f; // = proj.b / proj.a
    let p_div_a_b_div_a = p_div_a * b_div_a;
    let norm = sqrt(z_div_a * z_div_a + p_div_a_b_div_a * p_div_a_b_div_a);
    let c;
    let s;
    if norm != 0. {
        let inv_norm = 1.0 / norm;
        c = p_div_a_b_div_a * inv_norm;
        s = z_div_a * inv_norm;
    } else {
        c = 1.;
        s = 0.;
    }

    let y_phi = z_div_a + proj.e2s * b_div_a * s * s * s;
    let x_phi = p_div_a - proj.es * c * c * c;
    let norm_phi = sqrt(y_phi * y_phi + x_phi * x_phi);
    let mut cosphi;
    let mut sinphi;
    if norm_phi != 0. {
        let inv_norm_phi = 1.0 / norm_phi;
        cosphi = x_phi * inv_norm_phi;
        sinphi = y_phi * inv_norm_phi;
    } else {
        cosphi = 1.;
        sinphi = 0.;
    }
    if x_phi <= 0. {
        // this happen on non-sphere ellipsoid when x,y,z is very close to 0
        // there is no single solution to the cart->geodetic conversion in
        // that case, clamp to -90/90 deg and avoid a discontinuous boundary
        // near the poles
        phi = if coords.z() >= 0. { FRAC_PI_2 } else { -FRAC_PI_2 };
        cosphi = 0.;
        sinphi = if coords.z() >= 0. { 1. } else { -1. };
    } else {
        phi = atan(y_phi / x_phi);
    }
    let lam = atan2(y_div_a, x_div_a);
    let z = if cosphi < 1e-6 {
        // poleward of 89.99994 deg, we avoid division by zero
        // by computing the height as the cartesian z value
        // minus the geocentric radius of the Earth at the given
        // latitude
        let r = geocentric_radius(proj.a, b_div_a, cosphi, sinphi);
        fabs(coords.z()) - r
    } else {
        let n = normal_radius_of_curvature(proj.a, proj.es, sinphi);
        proj.a * p_div_a / cosphi - n
    };

    coords.set_phi(phi);
    coords.set_lam(lam);
    coords.set_z(z);
}
