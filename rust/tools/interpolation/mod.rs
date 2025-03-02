/// Average Interpolation tools
pub mod average;
/// Inverse Distance Weighted Interpolation tools
pub mod idw;
/// Lanczos Interpolation tools
pub mod lanczos;
/// Nearest Interpolation tools
pub mod nearest;

pub use average::*;
pub use idw::*;
pub use lanczos::*;
pub use nearest::*;
use s2json::MValueCompatible;

use crate::readers::RGBA;

/// Interpolation method
#[derive(Debug, Clone, Copy, PartialEq, Default)]
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
pub type InterpolationFunction<T> = fn(
    point: &VectorPoint,
    ref_data: &[VectorPoint<T>],
    get_value: Option<GetInterpolateValue<T>>,
) -> f64;

/// Get the interpolation function based on the method type
/// Options are:
/// - average
/// - nearest
/// - idw
/// - lanczos [Best]
pub fn get_interpolation<T: MValueCompatible>(
    method: InterpolationMethod,
) -> InterpolationFunction<T> {
    match method {
        InterpolationMethod::Average => average_interpolation,
        InterpolationMethod::Nearest => nearest_interpolation,
        InterpolationMethod::IDW => idw_interpolation,
        InterpolationMethod::Lanczos => lanczos_interpolation,
    }
}

/// Interpolation function To get the RGBA value of a point
pub type RGBAInterpolationFunction = fn(point: &VectorPoint, ref_data: &[VectorPointRGBA]) -> RGBA;

/// Get the RGBA interpolation function based on the method type
/// Options are:
/// - average
/// - nearest
/// - idw
/// - lanczos [Best]
pub fn get_rgba_interpolation(method: InterpolationMethod) -> RGBAInterpolationFunction {
    match method {
        InterpolationMethod::Average => average_interpolation_rgba,
        InterpolationMethod::Nearest => nearest_interpolation_rgba,
        InterpolationMethod::IDW => idw_interpolation_rgba,
        InterpolationMethod::Lanczos => lanczos_interpolation_rgba,
    }
}

/// Function to get the value of a point
pub type GetInterpolateValue<T> = fn(point: &VectorPoint<T>) -> f64;

/// Default function to get the value of a point
pub fn default_get_interpolate_current_value<T: MValueCompatible>(point: &VectorPoint<T>) -> f64 {
    point.z.unwrap_or(0.0)
}

/// Vector Point with RGBA data
pub type VectorPointRGBA = VectorPoint<RGBA>;

/// RGBA Channel
pub enum RgbaChannel {
    /// Red
    R,
    /// Green
    G,
    /// Blue
    B,
    /// Alpha
    A,
}

fn get_channel(p: &VectorPointRGBA, channel: RgbaChannel) -> f64 {
    if let Some(rgba) = &p.m {
        return match channel {
            RgbaChannel::R => rgba.r,
            RgbaChannel::G => rgba.g,
            RgbaChannel::B => rgba.b,
            RgbaChannel::A => rgba.a,
        };
    }
    0.
}

#[cfg(test)]
mod tests {
    use std::{vec, vec::Vec};

    use super::*;

    #[test]
    fn test_get_channel() {
        let p =
            VectorPointRGBA { x: 0., y: 0., z: None, m: Some(RGBA::new(1., 2., 3., 4.)), t: None };

        assert_eq!(get_channel(&p, RgbaChannel::R), 1.);

        let p = VectorPointRGBA { x: 0., y: 0., z: None, m: None, t: None };

        assert_eq!(get_channel(&p, RgbaChannel::R), 0.);
    }

    #[test]
    fn test_get_interpolation() {
        let point = VectorPoint::new(0.5, 0.5, None, None);
        let ref_data: Vec<VectorPoint> = vec![
            VectorPoint::new(0., 0., Some(1.), None),
            VectorPoint::new(1., 0., Some(2.), None),
            VectorPoint::new(0., 1., Some(3.), None),
            VectorPoint::new(1., 1., Some(4.), None),
        ];

        // AVERAGE
        let method = InterpolationMethod::Average;
        let interpolation = get_interpolation(method);
        let result = interpolation(&point, &ref_data, None);
        assert_eq!(result, 2.5);

        // IDW
        let method = InterpolationMethod::IDW;
        let interpolation = get_interpolation(method);
        let result = interpolation(&point, &ref_data, None);
        assert_eq!(result, 2.4999999999999996);

        // LANCZOS
        let method = InterpolationMethod::Lanczos;
        let interpolation = get_interpolation(method);
        let result = interpolation(&point, &ref_data, None);
        assert_eq!(result, 2.5);

        // NEAREST
        let method = InterpolationMethod::Nearest;
        let interpolation = get_interpolation(method);
        let result = interpolation(&point, &ref_data, None);
        assert_eq!(result, 1.);
    }

    #[test]
    fn test_get_rgba_interpolation() {
        let point = VectorPoint::new(0.5, 0.5, None, None);
        let ref_data: vec::Vec<VectorPointRGBA> = vec![
            VectorPointRGBA::new(0., 0., None, Some(RGBA::from_u8s(20, 20, 60, 255))),
            VectorPoint::new(1., 0., None, Some(RGBA::from_u8s(30, 100, 60, 255))),
            VectorPoint::new(0., 1., None, Some(RGBA::from_u8s(127, 127, 60, 255))),
            VectorPoint::new(1., 1., None, Some(RGBA::from_u8s(255, 255, 60, 255))),
        ];

        // AVERAGE
        let method = InterpolationMethod::Average;
        let interpolation = get_rgba_interpolation(method);
        let result = interpolation(&point, &ref_data);
        assert_eq!(result.to_u8s(), (84, 107, 60, 255));

        // IDW
        let method = InterpolationMethod::IDW;
        let interpolation = get_rgba_interpolation(method);
        let result = interpolation(&point, &ref_data);
        assert_eq!(result.to_u8s(), (84, 107, 60, 255));

        // LANCZOS
        let method = InterpolationMethod::Lanczos;
        let interpolation = get_rgba_interpolation(method);
        let result = interpolation(&point, &ref_data);
        assert_eq!(result.to_u8s(), (84, 107, 60, 255));

        // NEAREST
        let method = InterpolationMethod::Nearest;
        let interpolation = get_rgba_interpolation(method);
        let result = interpolation(&point, &ref_data);
        assert_eq!(result.to_u8s(), (20, 20, 60, 255));
    }
}
