#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{Proj, WGS84, derive_eccentricity, derive_sphere};

    fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
        (a - b).abs() < eps
    }

    #[test]
    fn test_eccentricity_for_ellipsoid() {
        let mut proj = Proj { a: 6378137.0, b: 6356752.314245, ..Default::default() };
        derive_eccentricity(&mut proj);

        assert!(proj.e > 0.0);
        assert!(proj.es > 0.0);
        // assert!(approx_eq(proj.alpha.sin(), proj.e3 * (2.0 - proj.e3 * proj.e3).sqrt(), 1e-12));
        assert!(proj.n > 0.0);
        assert!(proj.ra > 0.0);
        assert!(proj.rb > 0.0);
        assert!(proj.one_es > 0.0);
        assert!(proj.rone_es > 0.0);
        assert!(proj.e2 > 0.0);
    }

    #[test]
    fn test_eccentricity_for_sphere() {
        let mut proj = Proj { a: 6371000.0, b: 6371000.0, sphere: true, ..Default::default() };
        derive_eccentricity(&mut proj);

        assert_eq!(proj.es, 0.0);
        assert_eq!(proj.e, 0.0);
        assert_eq!(proj.e2, 0.0);
        assert_eq!(proj.e2s, 0.0);
        assert_eq!(proj.e3, 0.0);
        assert_eq!(proj.e3s, 0.0);
        assert_eq!(proj.f, 0.0);
        assert_eq!(proj.rf, f64::INFINITY);
        assert_eq!(proj.f2, 0.0);
        assert_eq!(proj.rf2, f64::INFINITY);
        assert_eq!(proj.n, 0.0);
        assert_eq!(proj.rn, f64::INFINITY);
    }

    #[test]
    fn test_one_es_zero_case() {
        // Forces one_es = 0 by setting a = b = 1.0, which gives es = 0.0, one_es = 1.0
        // but to hit 0.0 exactly we simulate it directly.
        let mut proj = Proj { a: 1.0, b: 0.0, ..Default::default() };
        derive_eccentricity(&mut proj);
        assert_eq!(proj.one_es, 0.0); // not zero in this case
    }

    #[test]
    fn test_inverse_axes_and_reciprocals() {
        let mut proj = Proj { a: 6378137.0, b: 6356752.314245, ..Default::default() };
        derive_eccentricity(&mut proj);

        assert!(approx_eq(proj.ra, 1.0 / proj.a, 1e-12));
        assert!(approx_eq(proj.rb, 1.0 / proj.b, 1e-12));
        assert!(approx_eq(proj.one_es, 1.0 - proj.es, 1e-12));
        assert!(approx_eq(proj.rone_es, 1.0 / proj.one_es, 1e-12));
    }

    #[test]
    fn test_derive_sphere_sets_a_from_ellps() {
        let mut proj = Proj { a: 0.0, ellps: "WGS84".to_string(), ..Default::default() };
        derive_sphere(&mut proj);
        assert_eq!(proj.a, WGS84.a);
        // assert_eq!(proj.b, WGS84.b.unwrap());
        assert_eq!(proj.rf, WGS84.rf.unwrap());
    }

    #[test]
    fn test_derive_sphere_computes_b_from_rf() {
        let mut proj = Proj { a: 6378137.0, rf: 298.257223563, ..Default::default() };
        derive_sphere(&mut proj);
        assert!((proj.b - 6356752.314245).abs() < 1e-6);
    }

    #[test]
    fn test_derive_sphere_computes_rf_from_b() {
        let mut proj = Proj { a: 6378137.0, b: 6356752.314245, ..Default::default() };
        derive_sphere(&mut proj);
        assert!((proj.rf - 0.003352810664747).abs() < 1e-12);
    }

    #[test]
    fn test_derive_sphere_sets_sphere_true_if_b_equals_a() {
        let mut proj = Proj { a: 6371000.0, b: 6371000.0, ..Default::default() };
        derive_sphere(&mut proj);
        assert!(proj.sphere);
    }

    #[test]
    fn test_derive_sphere_sets_b_equal_to_a_if_rf_zero() {
        let mut proj = Proj { a: 6371000.0, rf: 0.0, ..Default::default() };
        derive_sphere(&mut proj);
        assert!(proj.sphere);
        assert_eq!(proj.b, proj.a);
    }
}
