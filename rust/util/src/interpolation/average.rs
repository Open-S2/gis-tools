use crate::GetInterpolateValue;
use s2json::{GetM, GetXY, GetZ};

use super::Interpolatable;

/// # Average Neighbor Interpolation
///
/// ## Description
/// Finds the avarage point in the reference data to the given point and returns its value.
///
/// ## Usage
pub fn average_interpolation<M: Clone, P: GetXY + GetZ, R: GetM<M>, V: Interpolatable>(
    _point: &P,
    ref_data: &[R],
    get_value: GetInterpolateValue<R, V>,
) -> V {
    let mut res = V::default();
    for ref_point in ref_data {
        res += get_value(ref_point);
    }

    res /= ref_data.len() as f64;

    res
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use super::*;
    use crate::{RGBA, VectorPointRGBA, default_get_interpolate_current_value};
    use alloc::{vec, vec::Vec};
    use s2json::{MValue, VectorPoint};

    #[test]
    fn test_average_interpolation() {
        let point: VectorPoint = VectorPoint::new(0.5, 0.5, None, None);
        let ref_data: Vec<VectorPoint> = vec![
            VectorPoint::new(0., 0., Some(1.), None),
            VectorPoint::new(1., 0., Some(2.), None),
            VectorPoint::new(0., 1., Some(3.), None),
            VectorPoint::new(1., 1., Some(4.), None),
        ];
        let result =
            average_interpolation(&point, &ref_data, default_get_interpolate_current_value);
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
        let point: VectorPoint = VectorPoint::new(0.5, 0.5, None, None);
        let result =
            average_interpolation(&point, &ref_data, |p| p.m().cloned().unwrap_or_default());
        assert_eq!(result.to_u8s(), (84, 107, 60, 255));

        // test 2
        let point: VectorPoint = VectorPoint::new(0.65, 0.15, None, None);
        let result =
            average_interpolation(&point, &ref_data, |p| p.m().cloned().unwrap_or_default());
        assert_eq!(result.to_u8s(), (84, 107, 60, 255));

        // test 3
        let ref_data: Vec<VectorPoint<RGBA>> = vec![];
        let result =
            average_interpolation(&point, &ref_data, |p| p.m().cloned().unwrap_or_default());
        assert_eq!(result.to_u8s(), (0, 0, 0, 255));
    }

    #[test]
    fn test_average_interpolation_m_rgba() {
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
        let point: VectorPoint = VectorPoint::new(0.5, 0.5, None, None);
        let result = average_interpolation(&point, &ref_data, |p| RGBA::from(p.m().unwrap()));
        assert_eq!(result.to_u8s(), (84, 107, 60, 255));

        // test 2
        let point: VectorPoint = VectorPoint::new(0.65, 0.15, None, None);
        let result = average_interpolation(&point, &ref_data, |p| RGBA::from(p.m().unwrap()));
        assert_eq!(result.to_u8s(), (84, 107, 60, 255));

        // test 3
        let result =
            average_interpolation(&point, &[] as &[VectorPoint], |p| RGBA::from(p.m().unwrap()));
        assert_eq!(result.to_u8s(), (0, 0, 0, 255));
    }
}
