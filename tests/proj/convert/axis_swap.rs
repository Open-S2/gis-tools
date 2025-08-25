#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use gistools::proj::{
        AxisDirection, AxisSwap, AxisSwapConverter, CoordinateStep, Coords, Proj,
        ProjectCoordinates,
    };

    // ---- AxisSwap tests ----

    #[test]
    fn default_is_empty() {
        let swap = AxisSwap::default();
        assert!(swap.is_empty());
    }

    #[test]
    fn from_directions_east_north_up() {
        let dirs = vec![AxisDirection::East, AxisDirection::North, AxisDirection::Up];
        let swap = AxisSwap::from(dirs);
        assert_eq!(swap.axis, [0, 1, 2, 3]);
        assert_eq!(swap.sign, [1, 1, 1, 1]);
        // assert!(!swap.is_empty());
    }

    #[test]
    fn from_directions_with_negative_signs() {
        let dirs = vec![AxisDirection::West, AxisDirection::South, AxisDirection::Down];
        let swap = AxisSwap::from(dirs);
        assert_eq!(swap.axis[0], 0);
        assert_eq!(swap.axis[1], 1);
        assert_eq!(swap.axis[2], 2);
        assert_eq!(swap.sign[0], -1);
        assert_eq!(swap.sign[1], -1);
        assert_eq!(swap.sign[2], -1);
    }

    #[test]
    #[should_panic(expected = "Too many axes specified")]
    fn from_directions_too_many_panics() {
        let dirs = vec![
            AxisDirection::East,
            AxisDirection::North,
            AxisDirection::Up,
            AxisDirection::West,
            AxisDirection::South,
        ];
        let _ = AxisSwap::from(dirs);
    }

    #[test]
    #[should_panic(expected = "Duplicate axis mapping")]
    fn from_directions_duplicate_panics() {
        // East and West both map to axis 0
        let dirs = vec![AxisDirection::East, AxisDirection::West];
        let _ = AxisSwap::from(dirs);
    }

    // ---- AxisSwapConverter tests ----

    #[test]
    fn forward_with_empty_swap_is_noop() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        let converter = AxisSwapConverter::new(proj);
        let mut coords = Coords::new(1.0, 2.0, 3.0, 4.0);

        converter.forward(&mut coords);
        assert_eq!(coords, Coords::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn forward_applies_axis_swap() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        let mut converter = AxisSwapConverter::new(proj);
        converter.swap = AxisSwap::from(vec![AxisDirection::North, AxisDirection::West]);

        let mut coords = Coords::new(10.0, 20.0, 30.0, 40.0);
        converter.forward(&mut coords);

        // North = axis 1, sign +1 → takes value from y=20 into x
        // West = axis 0, sign -1 → takes value from x=10, negated, into y
        assert_eq!(coords.0, 20.0);
        assert_eq!(coords.1, -10.0);
    }

    #[test]
    fn inverse_reverses_axis_swap() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        let mut converter = AxisSwapConverter::new(proj);
        converter.swap = AxisSwap::from(vec![AxisDirection::North, AxisDirection::West]);

        let mut coords = Coords::new(20.0, -10.0, 30.0, 40.0);
        converter.inverse(&mut coords);

        // Reverse of forward above should recover original
        assert_eq!(coords.0, 10.0);
        assert_eq!(coords.1, 20.0);
    }

    #[test]
    fn code_and_name() {
        let proj = Rc::new(RefCell::new(Proj::default()));
        let converter = AxisSwapConverter::new(proj);
        assert_eq!(converter.code(), -1);
        assert_eq!(converter.name(), "axis swap");
        assert!(AxisSwapConverter::names().contains(&"axis swap"));
    }
}
