use super::{Interpolatable, get_distance};
use crate::tools::GetInterpolateValue;
use s2json::{GetM, GetXY, GetZ};

/// # Nearest Neighbor Interpolation
///
/// ## Description
/// Finds the nearest point in the reference data to the given point and returns its value.
///
/// ## Usage
pub fn nearest_interpolation<
    M: Clone,
    P: GetXY + GetZ,
    R: GetM<M> + GetXY + GetZ,
    V: Interpolatable,
>(
    point: &P,
    ref_data: &[R],
    get_value: GetInterpolateValue<R, V>,
) -> V {
    // Find the nearest point
    let mut nearest_point: Option<&R> = None;
    let mut min_distance = f64::INFINITY;

    for ref_point in ref_data {
        let dist = get_distance(point, ref_point);
        if dist < min_distance || nearest_point.is_none() {
            min_distance = dist;
            nearest_point = Some(ref_point);
        }
    }

    // Return the value of the nearest point
    if let Some(nearest_point) = nearest_point { get_value(nearest_point) } else { V::default() }
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use super::*;
    use crate::{
        readers::RGBA,
        tools::{VectorPointRGBA, default_get_interpolate_current_value},
    };
    use alloc::vec;
    use s2json::{MValue, VectorPoint};
    use std::vec::Vec;

    #[test]
    fn test_nearest_interpolation() {
        let ref_data = vec![
            VectorPoint::<MValue>::new(0., 0., Some(1.), None),
            VectorPoint::new(1., 0., Some(2.), None),
            VectorPoint::new(0., 1., Some(3.), None),
            VectorPoint::new(1., 1., Some(4.), None),
        ];

        // test 1
        let point: VectorPoint = VectorPoint::new(0.5, 0.5, None, None);
        let result =
            nearest_interpolation(&point, &ref_data, default_get_interpolate_current_value);
        assert_eq!(result, 1.);

        // test 2
        let point: VectorPoint = VectorPoint::new(0.65, 0.15, None, None);
        let result =
            nearest_interpolation(&point, &ref_data, default_get_interpolate_current_value);
        assert_eq!(result, 1.);

        // test 3
        let ref_data: Vec<VectorPoint> = vec![];
        let result =
            nearest_interpolation(&point, &ref_data, default_get_interpolate_current_value);
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
        let point: VectorPoint = VectorPoint::new(0.5, 0.5, None, None);
        let result = nearest_interpolation(&point, &ref_data, |p| p.m.unwrap());
        assert_eq!(result.to_u8s(), (20, 20, 60, 255));

        // test 2
        let point: VectorPoint = VectorPoint::new(0.65, 0.15, None, None);
        let result = nearest_interpolation(&point, &ref_data, |p| p.m.unwrap());
        assert_eq!(result.to_u8s(), (30, 100, 60, 255));

        // test 3
        let ref_data: Vec<VectorPoint<RGBA>> = vec![];
        let result = nearest_interpolation(&point, &ref_data, |p| p.m.unwrap());
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
        let point: VectorPoint<MValue> = VectorPoint::new(0.5, 0.5, None, None);
        let result =
            nearest_interpolation(&point, &ref_data, |p| RGBA::from(p.m.as_ref().unwrap()));
        assert_eq!(result.to_u8s(), (20, 20, 60, 255));

        // test 2
        let point: VectorPoint<MValue> = VectorPoint::new(0.65, 0.15, None, None);
        let result =
            nearest_interpolation(&point, &ref_data, |p| RGBA::from(p.m.as_ref().unwrap()));
        assert_eq!(result.to_u8s(), (30, 100, 60, 255));

        // test 3
        let result = nearest_interpolation(&point, &[] as &[VectorPoint<MValue>], |p| {
            RGBA::from(p.m.as_ref().unwrap())
        });
        assert_eq!(result.to_u8s(), (0, 0, 0, 255));
    }
}
