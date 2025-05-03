use super::{Interpolatable, get_distance};
use crate::util::GetInterpolateValue;
use libm::pow;
use s2json::{GetM, GetXY, GetZ};

/// # Inverse Distance Weighting Interpolation
///
/// ## Description
/// Given a reference of data, interpolate a point using inverse distance weighting
///
/// ## Usage
pub fn idw_interpolation<
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
        let d2 = pow(get_distance(point, ref_point), 2.);
        let mut value = get_value(ref_point);
        if d2 == 0. {
            return value;
        }
        value /= d2;
        numerator += value;
        denom += 1. / d2;
    }
    numerator /= denom;

    numerator
}
