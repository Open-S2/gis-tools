#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::geometry::{
        K_LIMIT_IJ, K_MAX_SI_TI, S2Point, ST, UV, face_xyz_to_uv, face_xyz_to_uvw, get_u_axis,
        get_u_norm, get_v_axis, get_v_norm, ij_to_st, si_ti_to_st, st_to_si_ti, st_to_uvlinear,
        st_to_uvtan, to_face_st, to_face_uv, uv_to_st_tan, uv_to_stlinear, valid_face_xyz_to_uv,
    };

    #[test]
    fn test_st_to_uvlinear() {
        assert_eq!(st_to_uvlinear(0.0), -1.);
        assert_eq!(st_to_uvlinear(0.5), 0.);
        assert_eq!(st_to_uvlinear(1.0), 1.0);
    }

    #[test]
    fn test_st_to_uvtan() {
        assert_eq!(st_to_uvtan(0.0), -1.);
        assert_eq!(st_to_uvtan(0.5), 0.);
        assert_eq!(st_to_uvtan(1.0), 1.0);
    }

    #[test]
    fn test_uv_to_stlinear() {
        assert_eq!(uv_to_stlinear(-1.), 0.);
        assert_eq!(uv_to_stlinear(0.), 0.5);
        assert_eq!(uv_to_stlinear(1.0), 1.0);
    }

    #[test]
    fn test_uv_to_st_tan() {
        assert_eq!(uv_to_st_tan(-1.), 0.);
        assert_eq!(uv_to_st_tan(0.), 0.5);
        assert_eq!(uv_to_st_tan(1.0), 1.0);
    }

    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    fn test_ij_to_st() {
        // NEEDS TO FAIL
        ij_to_st(K_LIMIT_IJ + 1);
    }

    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    fn test_si_ti_to_st() {
        // needs to fail
        si_ti_to_st(K_MAX_SI_TI + 1);
    }

    #[test]
    fn test_st_to_si_ti() {
        assert_eq!(st_to_si_ti(0.0), 0);
        assert_eq!(st_to_si_ti(0.5), K_MAX_SI_TI / 2);
        assert_eq!(st_to_si_ti(1.0), K_MAX_SI_TI);
    }

    #[test]
    fn test_to_face_st() {
        assert_eq!(to_face_st(&S2Point { x: 1.0, y: 0.0, z: 0.0 }, 0), ST { s: 0.5, t: 0.5 });
    }

    #[test]
    fn test_to_face_uv() {
        let p = S2Point::new(1.0, 0.0, 0.0);
        assert_eq!(to_face_uv(&p, 0), UV { u: 0., v: 0. });
    }

    #[test]
    fn test_face_xyz_to_uv() {
        let p = S2Point::new(1.0, 0.0, 0.0);
        assert_eq!(face_xyz_to_uv(0, &p), (true, 0., 0.));
        assert_eq!(face_xyz_to_uv(1, &p), (false, 0., 0.));
        assert_eq!(face_xyz_to_uv(2, &p), (false, 0., 0.));
        assert_eq!(face_xyz_to_uv(3, &p), (false, 0., 0.));
        assert_eq!(face_xyz_to_uv(4, &p), (false, 0., 0.));
        assert_eq!(face_xyz_to_uv(5, &p), (false, 0., 0.));
    }

    #[test]
    fn test_face_xyz_to_uvw() {
        let p = S2Point::new(1.0, 0.0, 0.0);
        let uvw_0 = face_xyz_to_uvw(0, &p);
        assert_eq!(uvw_0, S2Point::new(0., 0., 1.));
        let uvw_1 = face_xyz_to_uvw(1, &p);
        assert_eq!(uvw_1, S2Point::new(-1., 0., 0.));
        let uvw_2 = face_xyz_to_uvw(2, &p);
        assert_eq!(uvw_2, S2Point::new(-1., 0., 0.));
        let uvw_3 = face_xyz_to_uvw(3, &p);
        assert_eq!(uvw_3, S2Point::new(0., 0., -1.));
        let uvw_4 = face_xyz_to_uvw(4, &p);
        assert_eq!(uvw_4, S2Point::new(0., 1., 0.));
        let uvw_5 = face_xyz_to_uvw(5, &p);
        assert_eq!(uvw_5, S2Point::new(0., 1., 0.));
    }

    #[test]
    fn test_get_u_norm() {
        assert_eq!(get_u_norm::<S2Point>(0, 0.), S2Point::new(0., -1., 0.));
        assert_eq!(get_u_norm::<S2Point>(0, 0.25), S2Point::new(0.25, -1., 0.));

        assert_eq!(get_u_norm::<S2Point>(1, 0.), S2Point::new(1., 0., 0.));
        assert_eq!(get_u_norm::<S2Point>(1, 1.), S2Point::new(1., 1., 0.));

        assert_eq!(get_u_norm::<S2Point>(2, 0.), S2Point::new(1., 0., 0.));
        assert_eq!(get_u_norm::<S2Point>(2, 1.), S2Point::new(1., 0., 1.));

        assert_eq!(get_u_norm::<S2Point>(3, 0.), S2Point::new(0., 0., 1.));
        assert_eq!(get_u_norm::<S2Point>(3, 1.), S2Point::new(-1., 0., 1.));

        assert_eq!(get_u_norm::<S2Point>(4, 0.), S2Point::new(0., 0., 1.));
        assert_eq!(get_u_norm::<S2Point>(4, 1.), S2Point::new(0., -1., 1.));

        assert_eq!(get_u_norm::<S2Point>(5, 0.), S2Point::new(0., -1., 0.));
        assert_eq!(get_u_norm::<S2Point>(5, 1.), S2Point::new(0., -1., -1.));
    }

    #[test]
    fn test_get_v_norm() {
        assert_eq!(get_v_norm::<S2Point>(0, 0.), S2Point::new(0., 0., 1.));
        assert_eq!(get_v_norm::<S2Point>(1, 0.), S2Point::new(0., 0., 1.));
        assert_eq!(get_v_norm::<S2Point>(2, 0.), S2Point::new(0., -1., 0.));
        assert_eq!(get_v_norm::<S2Point>(3, 0.), S2Point::new(0., -1., 0.));
        assert_eq!(get_v_norm::<S2Point>(4, 0.), S2Point::new(1., 0., 0.));
        assert_eq!(get_v_norm::<S2Point>(5, 0.), S2Point::new(1., 0., 0.));
    }

    #[test]
    fn test_get_u_axis() {
        assert_eq!(get_u_axis::<S2Point>(0), S2Point::new(0., 1., 0.));
    }

    #[test]
    fn test_get_v_axis() {
        assert_eq!(get_v_axis::<S2Point>(0), S2Point::new(0., 0., 1.));
    }

    #[test]
    #[should_panic(expected = "internal error: entered unreachable code")]
    fn test_valid_face_xyz_to_uv() {
        valid_face_xyz_to_uv(0, &S2Point::new(0., 0., 0.));
    }
}
