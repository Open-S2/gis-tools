/// Average Interpolation tools
pub mod average;
/// Inverse Distance Weighted Interpolation tools
pub mod idw;
/// Lanczos Interpolation tools
pub mod lanczos;
/// Nearest Interpolation tools
pub mod nearest;

use crate::parsers::RGBA;
pub use average::*;
use core::ops::{AddAssign, DivAssign, MulAssign};
pub use idw::*;
pub use lanczos::*;
use libm::pow;
pub use nearest::*;
use s2json::{GetM, GetXY, GetZ, VectorPoint};
use serde::{Deserialize, Serialize};

/// Interpolation method
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InterpolationMethod {
    /// Average interpolation
    Average,
    /// Nearest interpolation
    Nearest,
    /// Inverse Distance Weighted interpolation
    IDW,
    /// Lanczos interpolation
    #[default]
    Lanczos,
}

/// Interpolation function To get the value of a point
pub type InterpolationFunction<P, R, V> =
    fn(point: &P, ref_data: &[R], get_value: GetInterpolateValue<R, V>) -> V;

/// A trait for values that can be used in interpolation
pub trait Interpolatable:
    Default
    + AddAssign<Self>
    + AddAssign<f64>
    + DivAssign<f64>
    + DivAssign<Self>
    + MulAssign<f64>
    + PartialEq<f64>
    + PartialEq<Self>
    + Clone
    + Copy
where
    Self: Sized,
{
}
impl<T> Interpolatable for T where
    T: Default
        + AddAssign<T>
        + AddAssign<f64>
        + DivAssign<f64>
        + DivAssign<T>
        + MulAssign<f64>
        + PartialEq<f64>
        + PartialEq<T>
        + Clone
        + Copy
        + Sized
{
}

/// Get the interpolation function based on the method type
/// Options are:
/// - average
/// - nearest
/// - idw
/// - lanczos [Best]
pub fn get_interpolation<
    M: Clone,
    P: GetXY + GetZ,
    R: GetM<M> + GetXY + GetZ,
    V: Interpolatable,
>(
    method: InterpolationMethod,
) -> InterpolationFunction<P, R, V> {
    match method {
        InterpolationMethod::Average => average_interpolation,
        InterpolationMethod::Nearest => nearest_interpolation,
        InterpolationMethod::IDW => idw_interpolation,
        InterpolationMethod::Lanczos => lanczos_interpolation,
    }
}

/// Function to get the value of a point
pub type GetInterpolateValue<R, V> = fn(point: &R) -> V;

/// Default function to get the value of a point
pub fn default_get_interpolate_current_value<T: GetZ>(point: &T) -> f64 {
    point.z().unwrap_or_default()
}

/// Get the distance between two points
pub fn get_distance<A: GetXY + GetZ, B: GetXY + GetZ>(a: &A, b: &B) -> f64 {
    let dx = a.x() - b.x();
    let dy = a.y() - b.y();
    let dz = a.z().unwrap_or_default() - b.z().unwrap_or_default();
    pow(dx * dx + dy * dy + dz * dz, 0.5)
}

/// Vector Point with RGBA data
pub type VectorPointRGBA = VectorPoint<RGBA>;
