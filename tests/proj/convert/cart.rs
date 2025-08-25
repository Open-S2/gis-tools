#[cfg(test)]
mod tests {
    use core::cell::RefCell;
    use core::f64::consts::PI;
    use gistools::proj::{
        CartesianConverter, CoordinateStep, IoUnits, Proj, ProjectCoordinates, ProjectionTransform,
        derive_sphere,
    };
    use gistools::proj::{Coords, geocentric_radius};
    use gistools::proj::{geodetic, normal_radius_of_curvature};
    use std::rc::Rc;

    // ---- CartesianConverter ----

    fn make_proj() -> Rc<RefCell<Proj>> {
        let mut p = Proj::default();
        derive_sphere(&mut p);
        Rc::new(RefCell::new(p))
    }

    #[test]
    fn normal_radius_of_curvature_sphere() {
        let a = 6371.0;
        let es = 0.0;
        assert_eq!(normal_radius_of_curvature(a, es, 0.5), a);
    }

    #[test]
    fn normal_radius_of_curvature_ellipsoid() {
        let a = 6378137.0;
        let es = 0.00669437999014;
        let sinphi = 0.5;
        let n = normal_radius_of_curvature(a, es, sinphi);
        assert!(n > 0.0);
    }

    #[test]
    fn geocentric_radius_matches_expectation() {
        let a = 6378137.0;
        let b_div_a = 0.996647189335;
        let r = geocentric_radius(a, b_div_a, 1.0, 0.0);
        assert!((r - a).abs() < 1e-6);
    }

    #[test]
    fn converter_sets_proj_fields() {
        let proj = make_proj();
        let conv = CartesianConverter::new(proj.clone());
        let p = proj.borrow();
        assert_eq!(p.left, IoUnits::RADIANS);
        assert_eq!(p.right, IoUnits::CARTESIAN);
        assert!(p.is_ll);
        assert_eq!(conv.name(), "cartesian");
        assert!(CartesianConverter::names().contains(&"cart"));
        assert_eq!(conv.code(), -1);
    }

    #[test]
    fn forward_and_inverse_roundtrip_equator() {
        let proj = make_proj();
        let conv = CartesianConverter::new(proj.clone());

        // φ=0 (equator), λ=0, h=0
        let mut coords = Coords::new(0.0, 0.0, 0.0, 0.0);
        conv.forward(&mut coords); // to Cartesian
        let Coords(x, y, z, _) = coords;
        assert!(x.abs() > 0.0);
        assert!(y.abs() < 1e-12);
        assert!(z.abs() < 1e-6);

        conv.inverse(&mut coords); // back to geodetic
        let Coords(phi, lam, h, _) = coords;
        assert!(phi.abs() < 1e-8);
        assert!(lam.abs() < 1e-8);
        assert!(h.abs() < 1e-6);
    }

    #[test]
    fn forward_and_inverse_roundtrip_mid_latitude() {
        let proj = make_proj();
        let conv = CartesianConverter::new(proj.clone());
        let _proj_transform: ProjectionTransform = ProjectionTransform::from(conv.clone());

        // φ=45°, λ=30°, h=1000 m
        let mut coords = Coords::new(PI / 4.0, PI / 6.0, 1000.0, 0.0);
        conv.forward(&mut coords);
        let Coords(x, y, z, _) = coords;
        assert!(x != 0.0 && y != 0.0 && z != 0.0);

        conv.inverse(&mut coords);
        let Coords(phi, lam, h, _) = coords;
        assert!((phi - PI / 4.0).abs() < 1e-6);
        assert!((lam - PI / 6.0).abs() < 1e-6);
        assert!((h - 1000.0).abs() < 1e-2);
    }

    #[test]
    fn geodetic_handles_pole_case() {
        let proj = make_proj();
        let mut coords = Coords::new(0.0, 0.0, 6378137.0, 0.0); // near pole in Cartesian
        geodetic(&proj.borrow(), &mut coords);

        let Coords(phi, _, _, _) = coords;
        assert!(phi == 0.);
    }
}
