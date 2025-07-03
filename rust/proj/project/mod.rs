/// Albers Conic Equal Area Projection
pub mod aea;
/// Azimuthal Equidistant Projection
pub mod aeqd;
/// Airy Projection
pub mod airy;
/// Bonne Projection
pub mod bonne;
/// Cassini-Soldner Projection
pub mod cass;
/// Equal Area Cylindrical Projection
pub mod cea;
/// Equidistant Cylindrical Projection
pub mod eqc;
/// Equidistant Conic Projection
pub mod eqdc;
/// Equal Earth Projection
pub mod eqearth;
/// Gnomonic Projection
pub mod gnom;
/// Goode Homolosine Projection
pub mod goode;
/// Gauss-Schreiber Transverse Mercator (aka Gauss-Laborde Reunion) Projection
pub mod gstmerc;
/// Krovak Projections
pub mod krovak;
/// Lambert Azimuthal Equal Area Projection
pub mod laea;
/// Lambert Conformal Conic Projection
pub mod lcc;
/// Lambert Conformal Conic Alternative Projection
pub mod lcca;
/// Mercator/Web Mercator Projection
pub mod merc;
/// Military Grid Reference System Projection
pub mod mgrs;
/// Miller Cylindrical Projection
pub mod mill;
/// Mollweide Projections
pub mod moll;
/// New Zealand Map Grid Projection
pub mod nzmg;
/// Oblique Cylindrical Equal Area Projection
pub mod ocea;
/// Oblated Equal Area Projection
pub mod oea;
/// Oblique Mercator Projection
pub mod omerc;
/// Orthographic Projection
pub mod ortho;
/// Polyconic (American) Projection
pub mod poly;
/// Robinson Projection
pub mod robin;
/// Sinusoidal Projections
pub mod sinu;
/// Swiss Oblique Cylindrical Projection
pub mod somerc;
/// Stereographic Projection
pub mod stere;
/// Oblique Stereographic Alternative Projection
pub mod sterea;
/// Transverse Central Cylindrical Projection
pub mod tcc;
/// Transverse Cylindrical Equal Area Projection
pub mod tcea;
/// Transverse Mercator implementations
pub mod tmerc;
/// Van der Grinten (I) Projection
pub mod vandg;

use super::{CoordinateStep, DatumType, Proj, Step, TransformCoordinates};
pub use aea::*;
pub use aeqd::*;
pub use airy::*;
use alloc::rc::Rc;
pub use bonne::*;
pub use cass::*;
pub use cea::*;
use core::cell::RefCell;
pub use eqc::*;
pub use eqdc::*;
pub use eqearth::*;
pub use gnom::*;
pub use goode::*;
pub use gstmerc::*;
pub use krovak::*;
pub use laea::*;
pub use lcc::*;
pub use lcca::*;
pub use merc::*;
pub use mgrs::*;
pub use mill::*;
pub use moll::*;
pub use nzmg::*;
pub use ocea::*;
pub use oea::*;
pub use omerc::*;
pub use ortho::*;
pub use poly::*;
pub use robin::*;
pub use sinu::*;
pub use somerc::*;
pub use stere::*;
pub use sterea::*;
pub use tcc::*;
pub use tcea::*;
pub use tmerc::*;
pub use vandg::*;

/// Projection trait. All projections must implement this
pub trait ProjectCoordinates {
    /// ESPG code for this projection
    fn code(&self) -> i64;
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

/// Projection trait. All projections must implement this
pub type LonLatProjection = BaseProjection;

/// Base class for all projections
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BaseProjection {}
impl BaseProjection {
    /// Create a list of steps for the base projection
    pub fn to_step() -> Step {
        let base_proj = BaseProjection {};
        base_proj.into()
    }
}
impl ProjectCoordinates for BaseProjection {
    fn code(&self) -> i64 {
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
    fn new(_proj: Rc<RefCell<Proj>>) -> Self {
        BaseProjection {}
    }
    /// Forward projection from x-y to lon-lat. In this case, radians to degrees.
    /// Input point is a placeholder for a lon-lat WGS84 point in radians
    fn forward<P: TransformCoordinates>(&self, p: &mut P) {
        p.set_x(p.lam().to_degrees());
        p.set_y(p.phi().to_degrees());
    }
    /// Inverse projection from lon-lat to x-y. In this case, degrees to radians.
    /// Input point is a placeholder for a lon-lat WGS84 point in degrees
    fn inverse<P: TransformCoordinates>(&self, p: &mut P) {
        p.set_lam(p.x().to_radians());
        p.set_phi(p.y().to_radians());
    }
}
impl From<BaseProjection> for Step {
    fn from(p: BaseProjection) -> Step {
        Step::Base(p.into())
    }
}
