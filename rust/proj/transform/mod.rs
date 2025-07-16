/// Stepping Guide
pub mod step;
/// Transformer is a tool to convert between projections
pub mod transformer;

use super::{BaseProjection, Coords, Proj};
use alloc::{boxed::Box, rc::Rc};
use core::cell::RefCell;
use s2json::VectorPoint;
pub use step::*;
pub use transformer::*;

/// A Projection Transform Definition
/// Temporary placeholder
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectionTransform {
    /// Projection guide
    pub proj: Rc<RefCell<Proj>>,
    /// mutation method
    pub method: Step,
    // These PJs are used for implementing cs2cs style coordinate handling in the 4D API
    /// Axis swapping if needed
    pub axisswap: Option<Box<ProjectionTransform>>,
    /// Cartesian if needed
    pub cart: Option<Box<ProjectionTransform>>,
    /// Cartesian to WGS84 if needed
    pub cart_wgs84: Option<Box<ProjectionTransform>>,
    /// Helmert if needed
    pub helmert: Option<Box<ProjectionTransform>>,
    /// Horizontal grid shift if needed
    pub hgridshift: Option<Box<ProjectionTransform>>,
    /// Vertical grid shift if needed
    pub vgridshift: Option<Box<ProjectionTransform>>,
}
impl Default for ProjectionTransform {
    fn default() -> Self {
        let proj = Rc::new(RefCell::new(Proj::default()));
        let method = BaseProjection::new(proj.clone());
        Self {
            proj,
            method: method.into(),
            axisswap: None,
            cart: None,
            cart_wgs84: None,
            helmert: None,
            hgridshift: None,
            vgridshift: None,
        }
    }
}
impl ProjectionTransform {
    /// Create the WGS 84 definition
    pub fn wgs84() -> Self {
        Self {
            proj: Rc::new(RefCell::new(Proj::default())),
            method: BaseProjection::to_step(),
            axisswap: None,
            cart: None,
            cart_wgs84: None,
            helmert: None,
            hgridshift: None,
            vgridshift: None,
        }
    }
}

/// Projection trait to modify a Point's values
pub trait TransformCoordinates: Clone + Default {
    /// Get Geodetic X coordinates
    fn x(&self) -> f64;
    /// Get Geodetic Y coordinates
    fn y(&self) -> f64;
    /// Get Geodetic Z coordinates
    fn z(&self) -> f64;
    /// Get the temporal coordinate
    fn t(&self) -> f64;
    /// Set Geodetic X coordinates
    fn set_x(&mut self, x: f64);
    /// Set Geodetic Y coordinates
    fn set_y(&mut self, y: f64);
    /// Set Geodetic Z coordinates
    fn set_z(&mut self, z: f64);
    /// Set the temporal coordinate
    fn set_t(&mut self, t: f64);

    // Top level get and set
    /// Get a coordinate at index
    fn get(&self, idx: usize) -> f64 {
        match idx {
            0 => self.x(),
            1 => self.y(),
            2 => self.z(),
            3 => self.t(),
            _ => panic!("Invalid index"),
        }
    }
    /// Set a coordinate at index
    fn set(&mut self, idx: usize, val: f64) {
        match idx {
            0 => self.set_x(val),
            1 => self.set_y(val),
            2 => self.set_z(val),
            3 => self.set_t(val),
            _ => panic!("Invalid index"),
        }
    }

    // All X based convenience methods

    /// Get U
    fn u(&self) -> f64 {
        self.x()
    }
    /// Set U
    fn set_u(&mut self, u: f64) {
        self.set_x(u)
    }
    /// Get lambda (radial longitude)
    fn lam(&self) -> f64 {
        self.x()
    }
    /// Set lambda (radial longitude)
    fn set_lam(&mut self, lam: f64) {
        self.set_x(lam)
    }
    /// Get S (Geodesic length)
    fn s(&self) -> f64 {
        self.x()
    }
    /// Set S (Geodesic length)
    fn set_s(&mut self, s: f64) {
        self.set_x(s)
    }
    /// Get Omega (Rotation)
    fn o(&self) -> f64 {
        self.x()
    }
    /// Set Omega (Rotation)
    fn set_o(&mut self, o: f64) {
        self.set_x(o)
    }
    /// Get East (Directional coordinate)
    fn e(&self) -> f64 {
        self.x()
    }
    /// Set East (Directional coordinate)
    fn set_e(&mut self, e: f64) {
        self.set_x(e)
    }

    // All Y based convenience methods

    /// Get V (UV space)
    fn v(&self) -> f64 {
        self.y()
    }
    /// Set V (UV space)
    fn set_v(&mut self, v: f64) {
        self.set_y(v)
    }
    /// Get Phi (radial latitude)
    fn phi(&self) -> f64 {
        self.y()
    }
    /// Set Phi (radial latitude)
    fn set_phi(&mut self, phi: f64) {
        self.set_y(phi)
    }
    /// Get fwd azi (Geodesic measurement)
    fn a1(&self) -> f64 {
        self.y()
    }
    /// Set fwd azi (Geodesic measurement)
    fn set_a1(&mut self, t: f64) {
        self.set_y(t)
    }
    /// Get Phi (Rotations measurement)
    fn p(&self) -> f64 {
        self.y()
    }
    /// Set Phi (Rotations measurement)
    fn set_p(&mut self, t: f64) {
        self.set_y(t)
    }
    /// Get North (Directional coordinate)
    fn n(&self) -> f64 {
        self.y()
    }
    /// Set North (Directional coordinate)
    fn set_n(&mut self, n: f64) {
        self.set_y(n)
    }

    // All Z based convenience methods

    /// Check if Z exists
    fn has_z(&self) -> bool;

    /// Get W
    fn w(&self) -> f64 {
        self.z()
    }
    /// Set W
    fn set_w(&mut self, w: f64) {
        self.set_z(w)
    }
    /// Get rev azi (Geodesic measurement)
    fn a2(&self) -> f64 {
        self.z()
    }
    /// Set rev azi (Geodesic measurement)
    fn set_a2(&mut self, a2: f64) {
        self.set_z(a2)
    }
    /// Get Kappa (Rotation)
    fn k(&self) -> f64 {
        self.z()
    }
    /// Set Kappa (Rotation)
    fn set_k(&mut self, k: f64) {
        self.set_z(k)
    }
    /// Get Up (Directional coordinate)
    fn up(&self) -> f64 {
        self.z()
    }
    /// Set Up (Directional coordinate)
    fn set_up(&mut self, up: f64) {
        self.set_z(up)
    }
}

impl<M: Default + Clone> TransformCoordinates for VectorPoint<M> {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn z(&self) -> f64 {
        self.z.unwrap_or(0.)
    }
    fn t(&self) -> f64 {
        0.
    }
    fn set_x(&mut self, x: f64) {
        self.x = x
    }
    fn set_y(&mut self, y: f64) {
        self.y = y
    }
    fn set_z(&mut self, z: f64) {
        self.z = Some(z);
    }
    fn set_t(&mut self, t: f64) {
        self.t = Some(t);
    }

    fn has_z(&self) -> bool {
        self.z.is_some()
    }
}
impl TransformCoordinates for Coords {
    fn x(&self) -> f64 {
        self.0
    }
    fn y(&self) -> f64 {
        self.1
    }
    fn z(&self) -> f64 {
        self.2
    }
    fn t(&self) -> f64 {
        self.3
    }
    fn set_x(&mut self, x: f64) {
        self.0 = x
    }
    fn set_y(&mut self, y: f64) {
        self.1 = y
    }
    fn set_z(&mut self, z: f64) {
        self.2 = z
    }
    fn set_t(&mut self, t: f64) {
        self.3 = t
    }

    fn has_z(&self) -> bool {
        self.z() != 0.
    }
}
