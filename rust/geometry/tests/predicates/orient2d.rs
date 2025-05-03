#[cfg(test)]
// #[coverage(off)]
mod tests {
    extern crate std;

    use geometry::{orient2d, orient2dfast};
    use libm::pow;
    use std::{fs, path::PathBuf, vec::Vec};

    #[test]
    fn test_orient2d() {
        assert!(orient2d(0.0, 0.0, 1.0, 1.0, 0.0, 1.0) < 0.0, "counterclockwise");
        assert!(orient2d(0.0, 0.0, 0.0, 1.0, 1.0, 1.0) > 0.0, "clockwise");
        assert!(orient2d(0.0, 0.0, 0.5, 0.5, 1.0, 1.0) == 0.0, "collinear");
    }

    #[test]
    fn test_orient2d_grid() {
        let r = 0.95;
        let q = 18.;
        let p = 16.8;
        let w = pow(2., -43.);

        for i in 0..128 {
            for j in 0..128 {
                let x = r + (w * (i as f64)) / 128.;
                let y = r + (w * (j as f64)) / 128.;

                let o = orient2d(x, y, q, q, p, p);
                assert!(!o.is_nan(), "{},{}", x, y);
            }
        }
    }

    #[test]
    fn test_orient2d_variables() {
        let mut path = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
        path.push("tests/geometry/predicates/fixtures/orient2d.txt");
        let contents = fs::read_to_string(&path).expect("Failed to read file");

        for line in contents.lines() {
            let values: Vec<f64> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Invalid number in input"))
                .collect();

            let (ax, ay, bx, by, cx, cy, sign) =
                (values[1], values[2], values[3], values[4], values[5], values[6], values[7]);
            let result = orient2d(ax, ay, bx, by, cx, cy);

            assert_eq!(result.signum(), -sign);
        }
    }

    #[test]
    fn test_orient2dfast() {
        assert!(orient2dfast(0.0, 0.0, 1.0, 1.0, 0.0, 1.0) < 0.0, "counterclockwise");
        assert!(orient2dfast(0.0, 0.0, 0.0, 1.0, 1.0, 1.0) > 0.0, "clockwise");
        assert!(orient2dfast(0.0, 0.0, 0.5, 0.5, 1.0, 1.0) == 0.0, "collinear");
    }
}
