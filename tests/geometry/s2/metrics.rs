#[cfg(test)]
#[allow(clippy::approx_constant)]
// #[coverage(off)]
mod tests {
    use gistools::geometry::K_AVG_ANGLE_SPAN;

    #[test]
    fn test_k_avg_angle_span() {
        // get_value
        assert_eq!(K_AVG_ANGLE_SPAN.get_value(0), 3.141592653589793);
        assert_eq!(K_AVG_ANGLE_SPAN.get_value(1), 1.5707963267948966);
        assert_eq!(K_AVG_ANGLE_SPAN.get_value(2), 0.7853981633974483);
        assert_eq!(K_AVG_ANGLE_SPAN.get_value(3), 0.39269908169872414);

        // get_closest_level
        assert_eq!(K_AVG_ANGLE_SPAN.get_closest_level(0.), 30);
        assert_eq!(K_AVG_ANGLE_SPAN.get_closest_level(3.141592653589793), 0);
        assert_eq!(K_AVG_ANGLE_SPAN.get_closest_level(1.5707963267948966), 0);
        assert_eq!(K_AVG_ANGLE_SPAN.get_closest_level(0.7853981633974483), 1);
        assert_eq!(K_AVG_ANGLE_SPAN.get_closest_level(0.77), 1);
        assert_eq!(K_AVG_ANGLE_SPAN.get_closest_level(0.44), 2);
        assert_eq!(K_AVG_ANGLE_SPAN.get_closest_level(0.39269908169872414), 2);
        assert_eq!(K_AVG_ANGLE_SPAN.get_closest_level(0.19634954084936207), 3);

        // get_level_for_max_value
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_max_value(0.), 30);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_max_value(3.141592653589793), 0);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_max_value(1.5707963267948966), 0);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_max_value(0.77), 2);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_max_value(0.44), 2);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_max_value(0.39269908169872414), 2);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_max_value(0.19634954084936207), 3);

        // get_level_for_min_value
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_min_value(0.), 30);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_min_value(1.5707963267948966), 0);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_min_value(0.77), 1);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_min_value(0.44), 1);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_min_value(0.39269908169872414), 2);
        assert_eq!(K_AVG_ANGLE_SPAN.get_level_for_min_value(0.19634954084936207), 3);
    }
}
