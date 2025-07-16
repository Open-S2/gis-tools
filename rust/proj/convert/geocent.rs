use crate::proj::{CoordinateStep, IoUnits, Proj, ProjectCoordinates, TransformCoordinates};
use alloc::rc::Rc;
use core::cell::RefCell;

/// # Conversion from geographic to geocentric latitude and back.
///
/// Stub projection for geocentric.  The transformation isn't
/// really done here since this code is 2D.  The real transformation
/// is handled by pj_transform.c.
#[derive(Debug, Clone, PartialEq)]
pub struct GeocentricConverter {
    proj: Rc<RefCell<Proj>>,
}
impl ProjectCoordinates for GeocentricConverter {
    fn code(&self) -> i64 {
        -1
    }

    fn name(&self) -> &'static str {
        "geocentric latitude"
    }

    fn names() -> &'static [&'static str] {
        &["geocent", "geocentric latitude"]
    }
}
impl CoordinateStep for GeocentricConverter {
    fn new(proj: Rc<RefCell<Proj>>) -> Self {
        {
            let proj = &mut proj.borrow_mut();
            proj.left = IoUnits::RADIANS;
            proj.right = IoUnits::CARTESIAN;
            proj.x0 = 0.;
            proj.y0 = 0.;
            proj.is_geocent = true;
        }
        GeocentricConverter { proj }
    }
    /// Geographical to geocentric
    fn forward<P: TransformCoordinates>(&self, _coords: &mut P) {}
    /// Geocentric to geographical
    fn inverse<P: TransformCoordinates>(&self, _coords: &mut P) {}
}
