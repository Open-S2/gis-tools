use s2json::MValueCompatible;
pub use s2json::{MValue, ValueType, VectorPoint};

use super::{average_interpolation, get_channel, RgbaChannel, VectorPointRGBA};
use crate::readers::RGBA;
use crate::tools::{default_get_interpolate_current_value, GetInterpolateValue};

use libm::pow;

/// # Inverse Distance Weighting Interpolation
///
/// ## Description
/// Given a reference of data, interpolate a point using inverse distance weighting
///
/// ## Usage
pub fn idw_interpolation<T: MValueCompatible>(
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
        let d2 = pow(point.distance(ref_point), 2.);
        let value = get_value(ref_point);
        if d2 == 0. {
            return value;
        }
        numerator += value / d2;
        denom += 1. / d2;
    }
    numerator / denom
}

/// Helper function for idw_interpolation on RGB(A) data.
/// Light in RGB data is logarithmically weighted, so we need to expand each component by n^2 to
/// get the correct weight for each component.
pub fn idw_interpolation_rgba(point: &VectorPoint, ref_data: &[VectorPointRGBA]) -> RGBA {
    if ref_data.is_empty() {
        return RGBA::default();
    }
    let r = idw_interpolation(point, ref_data, Some(|p| get_channel(p, RgbaChannel::R)));
    let g = idw_interpolation(point, ref_data, Some(|p| get_channel(p, RgbaChannel::G)));
    let b = idw_interpolation(point, ref_data, Some(|p| get_channel(p, RgbaChannel::B)));
    let a = average_interpolation(point, ref_data, Some(|p| get_channel(p, RgbaChannel::A)));

    RGBA::new(r, g, b, a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_idw_interpolation() {
        let ref_data = vec![
            VectorPoint::<MValue>::new(0., 0., Some(1.), None),
            VectorPoint::new(1., 0., Some(2.), None),
            VectorPoint::new(0., 1., Some(3.), None),
            VectorPoint::new(1., 1., Some(4.), None),
        ];

        // test 1
        let point = VectorPoint::new(0.5, 0.5, None, None);
        let result = idw_interpolation(&point, &ref_data, None);
        assert_eq!(result, 2.4999999999999996);

        // test 2
        let point = VectorPoint::new(0.65, 0.15, None, None);
        let result = idw_interpolation(&point, &ref_data, None);
        assert_eq!(result, 2.088659617630171);

        // test 3
        let result = idw_interpolation::<MValue>(&point, &[], None);
        assert_eq!(result, 0.);
    }

    #[test]
    fn test_idw_interpolation_rgba() {
        let ref_data = vec![
            VectorPointRGBA::new(0., 0., None, Some(RGBA::from_u8s(20, 20, 60, 255))),
            VectorPoint::new(1., 0., None, Some(RGBA::from_u8s(30, 100, 60, 255))),
            VectorPoint::new(0., 1., None, Some(RGBA::from_u8s(127, 127, 60, 255))),
            VectorPoint::new(1., 1., None, Some(RGBA::from_u8s(255, 255, 60, 255))),
        ];

        // test 1
        let point = VectorPoint::new(0.5, 0.5, None, None);
        let result = idw_interpolation_rgba(&point, &ref_data);
        assert_eq!(result.to_u8s(), (84, 107, 60, 255));

        // test 2
        let point = VectorPoint::new(0.65, 0.15, None, None);
        let result = idw_interpolation_rgba(&point, &ref_data);
        assert_eq!(result.to_u8s(), (46, 92, 60, 255));

        // test 3
        let result = idw_interpolation_rgba(&point, &[]);
        assert_eq!(result.to_u8s(), (0, 0, 0, 255));
    }
}
