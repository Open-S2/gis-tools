use crate::util::GetInterpolateValue;
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
