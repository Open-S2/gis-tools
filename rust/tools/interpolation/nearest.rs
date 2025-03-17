use super::{get_channel, get_channel_m, RgbaChannel};
use crate::{
    readers::RGBA,
    tools::{default_get_interpolate_current_value, GetInterpolateValue, VectorPointRGBA},
};
use s2json::{MValueCompatible, VectorPoint};

/// # Nearest Neighbor Interpolation
///
/// ## Description
/// Finds the nearest point in the reference data to the given point and returns its value.
///
/// ## Usage
pub fn nearest_interpolation<T: MValueCompatible>(
    point: &VectorPoint,
    ref_data: &[VectorPoint<T>],
    get_value: Option<GetInterpolateValue<T>>,
) -> f64 {
    if ref_data.is_empty() {
        return 0.;
    }
    let get_value = get_value.unwrap_or(default_get_interpolate_current_value);

    // Find the nearest point
    let mut nearest_point: Option<&VectorPoint<T>> = None;
    let mut min_distance = f64::INFINITY;

    for ref_point in ref_data {
        let dist = point.distance(ref_point);
        if dist < min_distance || nearest_point.is_none() {
            min_distance = dist;
            nearest_point = Some(ref_point);
        }
    }

    // Return the value of the nearest point
    if let Some(nearest_point) = nearest_point {
        get_value(nearest_point)
    } else {
        0.
    }
}

/// Helper function for nearest_interpolation on RGB(A) data.
/// Light in RGB data is logarithmically weighted, so we need to expand each component by n^2 to
/// get the correct weight for each component.
pub fn nearest_interpolation_rgba(point: &VectorPoint, ref_data: &[VectorPointRGBA]) -> RGBA {
    if ref_data.is_empty() {
        return RGBA::default();
    }
    let r = nearest_interpolation(point, ref_data, Some(|p| get_channel(p, RgbaChannel::R)));
    let g = nearest_interpolation(point, ref_data, Some(|p| get_channel(p, RgbaChannel::G)));
    let b = nearest_interpolation(point, ref_data, Some(|p| get_channel(p, RgbaChannel::B)));
    let a = nearest_interpolation(point, ref_data, Some(|p| get_channel(p, RgbaChannel::A)));

    RGBA::new(r, g, b, a)
}

/// Helper function for nearest_interpolation on RGB(A) data.
/// Light in RGB data is logarithmically weighted, so we need to expand each component by n^2 to
/// get the correct weight for each component.
pub fn nearest_interpolation_m_rgba(point: &VectorPoint, ref_data: &[VectorPoint]) -> RGBA {
    if ref_data.is_empty() {
        return RGBA::default();
    }
    let r = nearest_interpolation(point, ref_data, Some(|p| get_channel_m(p, RgbaChannel::R)));
    let g = nearest_interpolation(point, ref_data, Some(|p| get_channel_m(p, RgbaChannel::G)));
    let b = nearest_interpolation(point, ref_data, Some(|p| get_channel_m(p, RgbaChannel::B)));
    let a = nearest_interpolation(point, ref_data, Some(|p| get_channel_m(p, RgbaChannel::A)));

    RGBA::new(r, g, b, a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use s2json::MValue;

    #[test]
    fn test_nearest_interpolation() {
        let ref_data = vec![
            VectorPoint::<MValue>::new(0., 0., Some(1.), None),
            VectorPoint::new(1., 0., Some(2.), None),
            VectorPoint::new(0., 1., Some(3.), None),
            VectorPoint::new(1., 1., Some(4.), None),
        ];

        // test 1
        let point = VectorPoint::new(0.5, 0.5, None, None);
        let result = nearest_interpolation(&point, &ref_data, None);
        assert_eq!(result, 1.);

        // test 2
        let point = VectorPoint::new(0.65, 0.15, None, None);
        let result = nearest_interpolation(&point, &ref_data, None);
        assert_eq!(result, 2.);

        // test 3
        let result = nearest_interpolation::<MValue>(&point, &[], None);
        assert_eq!(result, 0.);
    }

    #[test]
    fn test_nearest_interpolation_rgba() {
        let ref_data = vec![
            VectorPointRGBA::new(0., 0., None, Some(RGBA::from_u8s(20, 20, 60, 255))),
            VectorPoint::new(1., 0., None, Some(RGBA::from_u8s(30, 100, 60, 255))),
            VectorPoint::new(0., 1., None, Some(RGBA::from_u8s(127, 127, 60, 255))),
            VectorPoint::new(1., 1., None, Some(RGBA::from_u8s(255, 255, 60, 255))),
        ];

        // test 1
        let point = VectorPoint::new(0.5, 0.5, None, None);
        let result = nearest_interpolation_rgba(&point, &ref_data);
        assert_eq!(result.to_u8s(), (20, 20, 60, 255));

        // test 2
        let point = VectorPoint::new(0.65, 0.15, None, None);
        let result = nearest_interpolation_rgba(&point, &ref_data);
        assert_eq!(result.to_u8s(), (30, 100, 60, 255));

        // test 3
        let result = nearest_interpolation_rgba(&point, &[]);
        assert_eq!(result.to_u8s(), (0, 0, 0, 255));
    }

    #[test]
    fn test_nearest_interpolation_m_rgba() {
        let ref_data: vec::Vec<VectorPoint> = vec![
            VectorPoint::new(
                0.,
                0.,
                None,
                Some(MValue::from([
                    ("r".into(), 20_u64.into()),
                    ("g".into(), 20_u64.into()),
                    ("b".into(), 60_u64.into()),
                    ("a".into(), 255_u64.into()),
                ])),
            ),
            VectorPoint::new(
                1.,
                0.,
                None,
                Some(MValue::from([
                    ("r".into(), 30_u64.into()),
                    ("g".into(), 100_u64.into()),
                    ("b".into(), 60_u64.into()),
                    ("a".into(), 255_u64.into()),
                ])),
            ),
            VectorPoint::new(
                0.,
                1.,
                None,
                Some(MValue::from([
                    ("r".into(), 127_u64.into()),
                    ("g".into(), 127_u64.into()),
                    ("b".into(), 60_u64.into()),
                    ("a".into(), 255_u64.into()),
                ])),
            ),
            VectorPoint::new(
                1.,
                1.,
                None,
                Some(MValue::from([
                    ("r".into(), 255_u64.into()),
                    ("g".into(), 255_u64.into()),
                    ("b".into(), 60_u64.into()),
                    ("a".into(), 255_u64.into()),
                ])),
            ),
        ];

        // test 1
        let point = VectorPoint::new(0.5, 0.5, None, None);
        let result = nearest_interpolation_m_rgba(&point, &ref_data);
        assert_eq!(result.to_u8s(), (20, 20, 60, 255));

        // test 2
        let point = VectorPoint::new(0.65, 0.15, None, None);
        let result = nearest_interpolation_m_rgba(&point, &ref_data);
        assert_eq!(result.to_u8s(), (30, 100, 60, 255));

        // test 3
        let result = nearest_interpolation_m_rgba(&point, &[]);
        assert_eq!(result.to_u8s(), (0, 0, 0, 255));
    }
}
