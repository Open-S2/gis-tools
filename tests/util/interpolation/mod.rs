mod average;
mod idw;
mod lanczos;
mod nearest;

#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate std;

    use std::{vec, vec::Vec};

    use gistools::util::{
        InterpolationMethod, default_get_interpolate_current_value, get_interpolation,
    };
    use s2json::VectorPoint;

    #[test]
    fn test_get_interpolation() {
        let point: VectorPoint = VectorPoint::new(0.5, 0.5, None, None);
        let ref_data: Vec<VectorPoint> = vec![
            VectorPoint::new(0., 0., Some(1.), None),
            VectorPoint::new(1., 0., Some(2.), None),
            VectorPoint::new(0., 1., Some(3.), None),
            VectorPoint::new(1., 1., Some(4.), None),
        ];

        // AVERAGE
        let method = InterpolationMethod::Average;
        let interpolation = get_interpolation(method);
        let result = interpolation(&point, &ref_data, default_get_interpolate_current_value);
        assert_eq!(result, 2.5);

        // IDW
        let method = InterpolationMethod::IDW;
        let interpolation = get_interpolation(method);
        let result = interpolation(&point, &ref_data, default_get_interpolate_current_value);
        assert_eq!(result, 1.5826612903225805);

        // LANCZOS
        let method = InterpolationMethod::Lanczos;
        let interpolation = get_interpolation(method);
        let result = interpolation(&point, &ref_data, default_get_interpolate_current_value);
        assert_eq!(result, 1.0);

        // NEAREST
        let method = InterpolationMethod::Nearest;
        let interpolation = get_interpolation(method);
        let result = interpolation(&point, &ref_data, default_get_interpolate_current_value);
        assert_eq!(result, 1.);
    }
}
