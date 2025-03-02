use s2json::MValueCompatible;
pub use s2json::{MValue, ValueType, VectorPoint};

use super::{get_channel, RgbaChannel};
use crate::readers::RGBA;
use crate::tools::{default_get_interpolate_current_value, GetInterpolateValue, VectorPointRGBA};

/// # Average Neighbor Interpolation
///
/// ## Description
/// Finds the avarage point in the reference data to the given point and returns its value.
///
/// ## Usage
pub fn average_interpolation<T: MValueCompatible>(
    _point: &VectorPoint,
    ref_data: &[VectorPoint<T>],
    get_value: Option<GetInterpolateValue<T>>,
) -> f64 {
    let get_value = get_value.unwrap_or(default_get_interpolate_current_value);
    let mut total = 0.;
    for ref_point in ref_data {
        total += get_value(ref_point);
    }

    total / ref_data.len() as f64
}

/// Helper function for {@link averageInterpolation} on RGB(A) data.
/// Light in RGB data is logarithmically weighted, so we need to expand each component by n^2 to
/// get the correct weight for each component.
pub fn average_interpolation_rgba(point: &VectorPoint, ref_data: &[VectorPointRGBA]) -> RGBA {
    if ref_data.is_empty() {
        return RGBA::default();
    }
    let r = average_interpolation(point, ref_data, Some(|p| get_channel(p, RgbaChannel::R)));
    let g = average_interpolation(point, ref_data, Some(|p| get_channel(p, RgbaChannel::G)));
    let b = average_interpolation(point, ref_data, Some(|p| get_channel(p, RgbaChannel::B)));
    let a = average_interpolation(point, ref_data, Some(|p| get_channel(p, RgbaChannel::A)));

    RGBA::new(r, g, b, a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::{vec, vec::Vec};

    #[test]
    fn test_average_interpolation() {
        let point = VectorPoint::new(0.5, 0.5, None, None);
        let ref_data: Vec<VectorPoint> = vec![
            VectorPoint::new(0., 0., Some(1.), None),
            VectorPoint::new(1., 0., Some(2.), None),
            VectorPoint::new(0., 1., Some(3.), None),
            VectorPoint::new(1., 1., Some(4.), None),
        ];
        let result = average_interpolation(&point, &ref_data, None);
        assert_eq!(result, 2.5);
    }

    #[test]
    fn test_average_interpolation_rgba() {
        let ref_data: vec::Vec<VectorPointRGBA> = vec![
            VectorPointRGBA::new(0., 0., None, Some(RGBA::from_u8s(20, 20, 60, 255))),
            VectorPoint::new(1., 0., None, Some(RGBA::from_u8s(30, 100, 60, 255))),
            VectorPoint::new(0., 1., None, Some(RGBA::from_u8s(127, 127, 60, 255))),
            VectorPoint::new(1., 1., None, Some(RGBA::from_u8s(255, 255, 60, 255))),
        ];

        // test 1
        let point = VectorPoint::new(0.5, 0.5, None, None);
        let result = average_interpolation_rgba(&point, &ref_data);
        assert_eq!(result.to_u8s(), (84, 107, 60, 255));

        // test 2
        let point = VectorPoint::new(0.65, 0.15, None, None);
        let result = average_interpolation_rgba(&point, &ref_data);
        assert_eq!(result.to_u8s(), (84, 107, 60, 255));

        // test 3
        let result = average_interpolation_rgba(&point, &[]);
        assert_eq!(result.to_u8s(), (0, 0, 0, 255));
    }
}
