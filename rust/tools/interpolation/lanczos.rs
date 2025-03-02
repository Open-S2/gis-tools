use s2json::MValueCompatible;
pub use s2json::{MValue, ValueType, VectorPoint};

use super::{average_interpolation, get_channel, RgbaChannel, VectorPointRGBA};
use crate::readers::RGBA;
use crate::tools::{default_get_interpolate_current_value, GetInterpolateValue};

use libm::{fabs, sin};

use core::f64::consts::PI;

/// # Lanczos Interpolation
///
/// ## Description
/// Perform interpolation using the Lanczos filter. This method uses a kernel-based approach
/// to weigh contributions from nearby points, providing a balance between smoothing and sharpness.
///
/// ## Usage
pub fn lanczos_interpolation<T: MValueCompatible>(
    point: &VectorPoint,
    ref_data: &[VectorPoint<T>],
    get_value: Option<GetInterpolateValue<T>>,
) -> f64 {
    if ref_data.is_empty() {
        return 0.;
    }
    let get_value = get_value.unwrap_or(default_get_interpolate_current_value);

    let mut numerator = 0.;
    let mut denom = 0.;

    for ref_point in ref_data {
        let weight = lanczos_kernel(point.distance(ref_point), 2.);
        let value = get_value(ref_point);
        numerator += value * weight;
        denom += weight;
    }

    // Avoid division by zero
    if denom == 0. {
        return 0.;
    }
    numerator / denom
}

/// Helper function for lanczos_interpolation on RGB(A) data.
/// Light in RGB data is logarithmically weighted, so we need to expand each component by n^2 to
/// get the correct weight for each component.
pub fn lanczos_interpolation_rgba(point: &VectorPoint, ref_data: &[VectorPointRGBA]) -> RGBA {
    if ref_data.is_empty() {
        return RGBA::default();
    }
    let r = lanczos_interpolation(point, ref_data, Some(|p| get_channel(p, RgbaChannel::R)));
    let g = lanczos_interpolation(point, ref_data, Some(|p| get_channel(p, RgbaChannel::G)));
    let b = lanczos_interpolation(point, ref_data, Some(|p| get_channel(p, RgbaChannel::B)));
    let a = average_interpolation(point, ref_data, Some(|p| get_channel(p, RgbaChannel::A)));

    RGBA::new(r, g, b, a)
}

/// Lanczos kernel function - returns the weight based on the distance from the target point
/// https://en.wikipedia.org/wiki/Lanczos_resampling
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn lanczos_kernel_test() {
        assert_eq!(lanczos_kernel(0., 2.), 1.);
        assert_eq!(lanczos_kernel(0.5, 2.), 0.5731591682507563);
        assert_eq!(lanczos_kernel(50000000., 2.), 0.);
    }

    #[test]
    fn test_lanczos_interpolation() {
        let ref_data = vec![
            VectorPoint::<MValue>::new(0., 0., Some(1.), None),
            VectorPoint::new(1., 0., Some(2.), None),
            VectorPoint::new(0., 1., Some(3.), None),
            VectorPoint::new(1., 1., Some(4.), None),
        ];

        // test 1
        let point = VectorPoint::new(0.5, 0.5, None, None);
        let result = lanczos_interpolation(&point, &ref_data, None);
        assert_eq!(result, 2.5);

        // test 2
        let point = VectorPoint::new(0.65, 0.15, None, None);
        let result = lanczos_interpolation(&point, &ref_data, None);
        assert_eq!(result, 1.7622380738712637);

        // test 3
        let result = lanczos_interpolation::<MValue>(&point, &[], None);
        assert_eq!(result, 0.);
    }

    #[test]
    fn test_lanczos_interpolation_rgba() {
        let ref_data = vec![
            VectorPointRGBA::new(0., 0., None, Some(RGBA::from_u8s(20, 20, 60, 255))),
            VectorPoint::new(1., 0., None, Some(RGBA::from_u8s(30, 100, 60, 255))),
            VectorPoint::new(0., 1., None, Some(RGBA::from_u8s(127, 127, 60, 255))),
            VectorPoint::new(1., 1., None, Some(RGBA::from_u8s(255, 255, 60, 255))),
        ];

        // test 1
        let point = VectorPoint::new(0.5, 0.5, None, None);
        let result = lanczos_interpolation_rgba(&point, &ref_data);
        assert_eq!(result.to_u8s(), (84, 107, 60, 255));

        // test 2
        let point = VectorPoint::new(0.65, 0.15, None, None);
        let result = lanczos_interpolation_rgba(&point, &ref_data);
        assert_eq!(result.to_u8s(), (30, 72, 60, 255));

        // test 3
        let result = lanczos_interpolation_rgba(&point, &[]);
        assert_eq!(result.to_u8s(), (0, 0, 0, 255));
    }
}
