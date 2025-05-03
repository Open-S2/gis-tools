/// Transformer is a tool to convert between projections
pub mod transformer;

use super::Coords;
use s2json::VectorPoint;
pub use transformer::*;

/// Projection trait to modify a Point's values
pub trait TransformCoordinates: Clone + Default {
    /// Get Geodetic X coordinates
    fn get_x(&self) -> f64;
    /// Get Geodetic Y coordinates
    fn get_y(&self) -> f64;
    /// Get Geodetic Z coordinates
    fn get_z(&self) -> f64;
    /// Get the temporal coordinate
    fn get_t(&self) -> f64;
    /// Set Geodetic X coordinates
    fn set_x(&mut self, x: f64);
    /// Set Geodetic Y coordinates
    fn set_y(&mut self, y: f64);
    /// Set Geodetic Z coordinates
    fn set_z(&mut self, z: f64);
    /// Set the temporal coordinate
    fn set_t(&mut self, t: f64);

    // All X based convenience methods

    /// Get U
    fn get_u(&self) -> f64 {
        self.get_x()
    }
    /// Set U
    fn set_u(&mut self, u: f64) {
        self.set_x(u)
    }
    /// Get lambda (radial longitude)
    fn get_lam(&self) -> f64 {
        self.get_x()
    }
    /// Set lambda (radial longitude)
    fn set_lam(&mut self, lam: f64) {
        self.set_x(lam)
    }
    /// Get S (Geodesic length)
    fn get_s(&self) -> f64 {
        self.get_x()
    }
    /// Set S (Geodesic length)
    fn set_s(&mut self, s: f64) {
        self.set_x(s)
    }
    /// Get Omega (Rotation)
    fn get_o(&self) -> f64 {
        self.get_x()
    }
    /// Set Omega (Rotation)
    fn set_o(&mut self, o: f64) {
        self.set_x(o)
    }
    /// Get East (Directional coordinate)
    fn get_e(&self) -> f64 {
        self.get_x()
    }
    /// Set East (Directional coordinate)
    fn set_e(&mut self, e: f64) {
        self.set_x(e)
    }

    // All Y based convenience methods

    /// Get V (UV space)
    fn get_v(&self) -> f64 {
        self.get_y()
    }
    /// Set V (UV space)
    fn set_v(&mut self, v: f64) {
        self.set_y(v)
    }
    /// Get Phi (radial latitude)
    fn get_phi(&self) -> f64 {
        self.get_y()
    }
    /// Set Phi (radial latitude)
    fn set_phi(&mut self, phi: f64) {
        self.set_y(phi)
    }
    /// Get fwd azi (Geodesic measurement)
    fn get_a1(&self) -> f64 {
        self.get_y()
    }
    /// Set fwd azi (Geodesic measurement)
    fn set_a1(&mut self, t: f64) {
        self.set_y(t)
    }
    /// Get Phi (Rotations measurement)
    fn get_p(&self) -> f64 {
        self.get_y()
    }
    /// Set Phi (Rotations measurement)
    fn set_p(&mut self, t: f64) {
        self.set_y(t)
    }
    /// Get North (Directional coordinate)
    fn get_n(&self) -> f64 {
        self.get_y()
    }
    /// Set North (Directional coordinate)
    fn set_n(&mut self, n: f64) {
        self.set_y(n)
    }

    // All Z based convenience methods

    /// Get W
    fn get_w(&self) -> f64 {
        self.get_z()
    }
    /// Set W
    fn set_w(&mut self, w: f64) {
        self.set_z(w)
    }
    /// Get rev azi (Geodesic measurement)
    fn get_a2(&self) -> f64 {
        self.get_z()
    }
    /// Set rev azi (Geodesic measurement)
    fn set_a2(&mut self, a2: f64) {
        self.set_z(a2)
    }
    /// Get Kappa (Rotation)
    fn get_k(&self) -> f64 {
        self.get_z()
    }
    /// Set Kappa (Rotation)
    fn set_k(&mut self, k: f64) {
        self.set_z(k)
    }
    /// Get Up (Directional coordinate)
    fn get_up(&self) -> f64 {
        self.get_z()
    }
    /// Set Up (Directional coordinate)
    fn set_up(&mut self, up: f64) {
        self.set_z(up)
    }
}

impl<M: Default + Clone> TransformCoordinates for VectorPoint<M> {
    fn get_x(&self) -> f64 {
        self.x
    }
    fn get_y(&self) -> f64 {
        self.y
    }
    fn get_z(&self) -> f64 {
        self.z.unwrap_or(0.)
    }
    fn get_t(&self) -> f64 {
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
}
impl TransformCoordinates for Coords {
    fn get_x(&self) -> f64 {
        self.0
    }
    fn get_y(&self) -> f64 {
        self.1
    }
    fn get_z(&self) -> f64 {
        self.2
    }
    fn get_t(&self) -> f64 {
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
}
