/// Albers Conic Equal Area Projection
pub mod aea;
/// Mercator/Web Mercator Projection
pub mod merc;
/// Military Grid Reference System Projection
pub mod mgrs;

use super::{CoordinateStep, DatumType, Proj, TransformCoordinates};
pub use aea::*;
pub use merc::*;

/// Projection trait. All projections must implement this
pub trait ProjectCoordinates {
    /// ESPG code for this projection
    fn code(&self) -> u32;
    /// Projection name
    fn name(&self) -> &'static str;
    /// Returns the list of canonical names for this projection.
    /// This is an associated function, similar to a static method.
    fn names() -> &'static [&'static str];
    /// get the datum type. Defaults to no datum
    fn datum_type() -> u8 {
        DatumType::NoDatum as u8
    }
}

/// Base class for all projections
#[derive(Debug)]
pub struct BaseProjection {}
impl ProjectCoordinates for BaseProjection {
    fn code(&self) -> u32 {
        0
    }
    fn name(&self) -> &'static str {
        "longlat"
    }
    fn names() -> &'static [&'static str] {
        &["longlat", "identity"]
    }
}
impl CoordinateStep for BaseProjection {
    fn new(_proj: &mut Proj) -> Self {
        BaseProjection {}
    }
    /// Forward projection from x-y to lon-lat. In this case, radians to degrees.
    /// Input point is a placeholder for a lon-lat WGS84 point in radians
    fn forward<P: TransformCoordinates>(&self, _proj: &Proj, p: &mut P) {
        p.set_x(p.get_lam().to_degrees());
        p.set_y(p.get_phi().to_degrees());
    }
    /// Inverse projection from lon-lat to x-y. In this case, degrees to radians.
    /// Input point is a placeholder for a lon-lat WGS84 point in degrees
    fn inverse<P: TransformCoordinates>(&self, _proj: &Proj, p: &mut P) {
        p.set_lam(p.get_x().to_radians());
        p.set_phi(p.get_y().to_radians());
    }
}
