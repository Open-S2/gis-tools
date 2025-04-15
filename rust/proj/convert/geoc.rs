use crate::proj::{CoordinateStep, Direction, IoUnits, Proj, TransformCoordinates};
use libm::{atan, tan};

/// Convert geographical latitude to geocentric (or the other way round if
/// direction = PJ_INV)
///
/// The conversion involves a call to the tangent function, which goes
/// through the roof at the poles, so very close (the last centimeter) to the
/// poles no conversion takes place and the input latitude is copied directly to
/// the output.
///
/// Fortunately, the geocentric latitude converges to the geographical at
/// the poles, so the difference is negligible.
///
/// For the spherical case, the geographical latitude equals the geocentric,
/// and consequently, the input is copied directly to the output.
#[derive(Debug)]
pub struct GeocentricConverter {}
impl CoordinateStep for GeocentricConverter {
    fn new(proj: &mut Proj) -> Self {
        proj.left = IoUnits::RADIANS;
        proj.right = IoUnits::RADIANS;
        proj.is_ll = true;
        GeocentricConverter {}
    }
    /// Geographical to geocentric
    fn forward<P: TransformCoordinates>(&self, proj: &Proj, coords: &mut P) {
        geocentric_latitude(proj, Direction::FWD, coords);
    }
    /// Geocentric to geographical
    fn inverse<P: TransformCoordinates>(&self, proj: &Proj, coords: &mut P) {
        geocentric_latitude(proj, Direction::INV, coords);
    }
}

/// Geocentric latitude conversion function
pub fn geocentric_latitude<P: TransformCoordinates>(
    proj: &Proj,
    direction: Direction,
    coords: &mut P,
) {
    let limit = core::f64::consts::FRAC_PI_2 - 1e-9;
    let phi = coords.get_phi();
    if (phi > limit) || (phi < -limit) || (proj.es == 0.) {
        return;
    }
    if direction == Direction::FWD {
        coords.set_phi(atan(proj.one_es * tan(phi)));
    } else {
        coords.set_phi(atan(proj.rone_es * tan(phi)));
    }
}
