use super::{Interpolatable, get_distance};
use crate::GetInterpolateValue;
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
