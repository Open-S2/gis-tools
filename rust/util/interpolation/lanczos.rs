use super::{Interpolatable, get_distance};
use crate::util::GetInterpolateValue;
use core::f64::consts::PI;
use libm::{fabs, sin};
use s2json::{GetM, GetXY, GetZ};

/// # Lanczos Interpolation
///
/// ## Description
/// Perform interpolation using the Lanczos filter. This method uses a kernel-based approach
/// to weigh contributions from nearby points, providing a balance between smoothing and sharpness.
///
/// ## Usage
pub fn lanczos_interpolation<
    M: Clone,
    P: GetXY + GetZ,
    R: GetM<M> + GetXY + GetZ,
    V: Interpolatable,
>(
    point: &P,
    ref_data: &[R],
    get_value: GetInterpolateValue<R, V>,
) -> V {
    if ref_data.is_empty() {
        return V::default();
    }

    let mut numerator = V::default();
    let mut denom = V::default();

    for ref_point in ref_data {
        let weight = lanczos_kernel(get_distance(point, ref_point), 2.);
        let mut value = get_value(ref_point);
        value *= weight;
        numerator += value;
        denom += weight;
    }

    // Avoid division by zero
    if denom == 0. {
        return V::default();
    }
    numerator /= denom;

    numerator
}

/// Lanczos kernel function - returns the weight based on the distance from the target point
/// <https://en.wikipedia.org/wiki/Lanczos_resampling>
pub fn lanczos_kernel(x: f64, a: f64) -> f64 {
    if x == 0. {
        return 1.; // sinc(0) = 1
    }
    if fabs(x) >= a {
        return 0.; // Outside the kernel radius
    }
    let pi_x = PI * x;
    (sin(pi_x) / pi_x) * (sin(pi_x / a) / (pi_x / a))
}
