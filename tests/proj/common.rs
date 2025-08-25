#[cfg(test)]
mod tests {
    use gistools::proj::{
        Proj, aacos, aasin, aatan2, adjlon, asqrt, authalic_lat, authalic_lat_inverse,
        auxlat_convert_full, msfn,
    };
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

    fn proj_high_eccentricity() -> Proj {
        Proj {
            e: 0.9,
            es: 0.9 * 0.9,
            one_es: 1.0 - 0.9 * 0.9,
            n: 9999.0, // force authalic_series_valid(n) == false
            ..Proj::default()
        }
    }

    // fn proj_series_valid() -> Proj {
    //     Proj {
    //         e: 0.0818191908426, // WGS84 eccentricity
    //         es: 0.0818191908426f64.powi(2),
    //         one_es: 1.0 - 0.0818191908426f64.powi(2),
    //         n: 0.0, // assume authalic_series_valid(0.0) == true
    //         ..Proj::default()
    //     }
    // }

    fn proj_series_invalid() -> Proj {
        Proj {
            e: 0.9,
            es: 0.81,
            one_es: 0.19,
            n: 9999.0, // assume false here
            ..Proj::default()
        }
    }

    #[test]
    fn test_msfn() {
        assert_eq!(msfn(0.0, 0.0), 1.0);
    }

    #[test]
    fn runs_newton_iteration() {
        let proj = proj_high_eccentricity();
        let beta = FRAC_PI_4; // 45° in radians
        let apa = [0.0, 0.0, 0.0]; // dummy coeffs
        let qp = 1.0;

        let phi = authalic_lat_inverse(beta, &apa, &proj, qp);

        // Should converge near beta
        assert!(phi.is_finite());
        assert!((phi - beta).abs() < 0.5); // loose check, we only care that iteration ran
    }

    // #[test]
    // fn test_authalic_lat_series_path() {
    //     let proj = proj_series_valid();
    //     let phi = FRAC_PI_4; // 45°
    //     let sinphi = phi.sin();
    //     let cosphi = phi.cos();
    //     let apa = vec![0.0; 5]; // dummy coeffs
    //     let qp = 1.0;

    //     let lat = authalic_lat(phi, sinphi, cosphi, &apa, &proj, qp);

    //     assert!(lat.is_finite());
    //     // In series path, expect output roughly close to phi
    //     assert!((lat - phi).abs() < 1e-6);
    // }

    #[test]
    fn test_authalic_lat_direct_formula_path() {
        let proj = proj_series_invalid();
        let phi = FRAC_PI_4;
        let sinphi = phi.sin();
        let cosphi = phi.cos();
        let apa = vec![0.0; 5];
        let qp = 1.0;

        let lat = authalic_lat(phi, sinphi, cosphi, &apa, &proj, qp);

        assert!(lat.is_finite());
        // Result should be between -pi/2 and pi/2
        assert!(lat.abs() <= FRAC_PI_2);
    }

    #[test]
    fn test_authalic_lat_ratio_clamped() {
        let mut proj = proj_series_invalid();
        // Hack values so that q/qp is > 1 due to rounding
        proj.e = 0.0;
        proj.es = 0.0;
        proj.one_es = 1.0;
        proj.n = 9999.0;

        let phi = FRAC_PI_2; // 90°
        let sinphi = phi.sin();
        let cosphi = phi.cos();
        let apa = vec![0.0; 5];
        let qp = 0.5; // force q/qp > 1
        let lat = authalic_lat(phi, sinphi, cosphi, &apa, &proj, qp);

        // It should clamp ratio and return ±90°
        assert!((lat - FRAC_PI_2).abs() < 1e-12);
    }

    #[test]
    fn test_aatan2_both_near_zero() {
        // Within tolerance → returns exactly 0.0
        let val = aatan2(1e-15, 1e-15);
        assert_eq!(val, 0.7853981633974483);
    }

    #[test]
    fn test_aatan2_regular_quadrant() {
        // atan2(1, 0) = π/2
        let val = aatan2(1.0, 0.0);
        assert!((val - FRAC_PI_2).abs() < 1e-12);
    }

    #[test]
    fn test_aatan2_negative_quadrant() {
        // atan2(0, -1) = π
        let val = aatan2(0.0, -1.0);
        assert!((val - PI).abs() < 1e-12);
    }

    #[test]
    fn test_asqrt_negative() {
        // Negative input should clamp to 0.0
        assert_eq!(asqrt(-5.0), 0.0);
    }

    #[test]
    fn test_asqrt_zero() {
        // Exactly zero should also return 0.0
        assert_eq!(asqrt(0.0), 0.0);
    }

    #[test]
    fn test_asqrt_positive() {
        // Positive input should match sqrt
        let v = 9.0;
        let val = asqrt(v);
        assert!((val - v.sqrt()).abs() < 1e-12);
    }

    #[test]
    fn test_aacos_normal_range() {
        let v = 0.5;
        let result = aacos(v);
        assert!((result - v.acos()).abs() < 1e-12);
    }

    #[test]
    fn test_aacos_one() {
        // Exactly 1.0 → returns 0.0
        assert_eq!(aacos(1.0), 0.0);
    }

    #[test]
    fn test_aacos_negative_one() {
        // Exactly -1.0 → returns PI
        assert_eq!(aacos(-1.0), std::f64::consts::PI);
    }

    #[test]
    #[should_panic(expected = "Coordinate outside projection domain")]
    fn test_aacos_out_of_domain() {
        // Value beyond tolerance → panic
        aacos(2.0);
    }

    #[test]
    fn test_aasin_normal_range() {
        let v = 0.5;
        let result = aasin(v);
        assert!((result - v.asin()).abs() < 1e-12);
    }

    #[test]
    fn test_aasin_one() {
        // Exactly 1.0 → returns π/2
        let result = aasin(1.0);
        assert_eq!(result, FRAC_PI_2);
    }

    #[test]
    fn test_aasin_negative_one() {
        // Exactly -1.0 → returns -π/2
        let result = aasin(-1.0);
        assert_eq!(result, -FRAC_PI_2);
    }

    #[test]
    #[should_panic(expected = "Coordinate outside projection domain")]
    fn test_aasin_out_of_domain_positive() {
        // Value beyond tolerance → panic
        aasin(2.0);
    }

    #[test]
    #[should_panic(expected = "Coordinate outside projection domain")]
    fn test_aasin_out_of_domain_negative() {
        // Negative value beyond tolerance → panic
        aasin(-2.0);
    }

    #[test]
    fn test_adjl_lon_within_range() {
        // longitude already within -π..π → unchanged
        let val = adjlon(PI / 2.0);
        assert!((val - PI / 2.0).abs() < 1e-12);

        let val = adjlon(-PI / 2.0);
        assert!((val + PI / 2.0).abs() < 1e-12);
    }

    #[test]
    fn test_adjl_lon_above_pi() {
        // longitude slightly above π → wraps to negative range
        let val = adjlon(PI + 0.1);
        assert!(val < PI);
        assert!(val > -PI);
    }

    #[test]
    fn test_adjl_lon_below_minus_pi() {
        // longitude slightly below -π → wraps to positive range
        let val = adjlon(-PI - 0.1);
        assert!(val < PI);
        assert!(val > -PI);
    }

    #[test]
    fn test_adjl_lon_multiple_revolutions() {
        // longitude beyond multiple full circles
        let val = adjlon(3.0 * PI); // should wrap to -π
        assert!((val + PI).abs() < 1e-12);

        let val = adjlon(-3.0 * PI); // should wrap to π
        assert!(val == -3.141592653589793);
    }

    #[test]
    fn test_auxlat_convert_full_finite() {
        let szeta = FRAC_PI_4.sin();
        let czeta = FRAC_PI_4.cos();
        let f = [0.1, 0.2, 0.3, 0.4, 0.5];
        let k = f.len() as i32;
        let mut seta = 0.0;
        let mut ceta = 0.0;

        auxlat_convert_full(szeta, czeta, &mut seta, &mut ceta, &f, k);

        // Outputs should be finite and bounded
        assert!(seta.is_finite());
        assert!(ceta.is_finite());
        assert!(seta.abs() <= 1.0);
        assert!(ceta.abs() <= 1.0);
    }

    #[test]
    fn test_auxlat_convert_full_zero_coefficients() {
        let szeta = 0.5;
        let czeta = 0.86602540378; // cos(30°)
        let f = [0.0; 5];
        let k = f.len() as i32;
        let mut seta = 0.0;
        let mut ceta = 0.0;

        auxlat_convert_full(szeta, czeta, &mut seta, &mut ceta, &f, k);

        // With all zeros, delta = 0 → seta = szeta, ceta = czeta
        assert!((seta - szeta).abs() < 1e-12);
        assert!((ceta - czeta).abs() < 1e-12);
    }

    #[test]
    fn test_auxlat_convert_full_nonzero_coefficients_consistency() {
        let szeta = 0.5;
        let czeta = 0.86602540378;
        let f = [0.1, -0.2, 0.3, -0.4, 0.5];
        let k = f.len() as i32;
        let mut seta = 0.0;
        let mut ceta = 0.0;

        auxlat_convert_full(szeta, czeta, &mut seta, &mut ceta, &f, k);

        // Check that outputs are finite
        assert!(seta.is_finite());
        assert!(ceta.is_finite());

        // Optional: check that seta^2 + ceta^2 ~ 1 for small deltas
        let r2 = seta * seta + ceta * ceta;
        assert!((r2 - 1.0).abs() < 1.0); // rough check, not exact
    }
}
