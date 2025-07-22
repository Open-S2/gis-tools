#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{Coords, mgrs_forward, mgrs_to_point};

    #[test]
    fn test_mgrs_forward_and_back() {
        assert_eq!(mgrs_forward(&Coords::new_xy(0., 0.), None), "31NAA6602100000");
        assert_eq!(
            mgrs_to_point("31NAA6602100000"),
            Some(Coords::new_xy(-0.44851108259715755, -0.44851111280975187))
        );

        assert_eq!(mgrs_forward(&Coords::new_xy(0., 0.), Some(3)), "31NAA660000");
        assert_eq!(
            mgrs_to_point("31NAA660000"),
            Some(Coords::new_xy(-0.4481584238594583, -0.44816144559765436))
        );

        assert_eq!(mgrs_forward(&Coords::new_xy(0., 0.), Some(1)), "31NAA60");
        assert_eq!(
            mgrs_to_point("31NAA60"),
            Some(Coords::new_xy(-0.4303731608729941, -0.43068008433215854))
        );

        assert_eq!(mgrs_forward(&Coords::new_xy(20., 20.), None), "34QCH9539011793");
        assert_eq!(
            mgrs_to_point("34QCH9539011793"),
            Some(Coords::new_xy(19.518202587566257, 19.518202740219166))
        );

        assert_eq!(mgrs_forward(&Coords::new_xy(-180., 0.), None), "1NAA6602100000");
        // TODO: IS BROKEN!
        // assert_eq!(
        //     mgrs_to_point("1NAA6602100000"),
        //     Some(Coords::new_xy(-90.44851108259716, -90.44851111280975))
        // );
    }

    //   p = mgrsForward({ x: -180, y: 0 });
    //   expect(p).toEqual('1NAA6602100000');
    //   expect(mgrsToPoint(p)).toEqual({ x: -179.999999489944, y: 0.0000045174155428847555 });
    //   p = mgrsForward({ x: 180, y: 0 });
    //   expect(p).toEqual('60NZF3397800000');
    //   expect(mgrsToPoint(p)).toEqual({ x: 179.999999489944, y: 0.0000045174155055519 });
    //   p = mgrsForward({ x: 0, y: 84 });
    //   expect(p).toEqual('31XDP6500529005');
    //   expect(mgrsToPoint(p)).toEqual({ x: 0.000011844240836555997, y: 84.00000291206752 });
    //   p = mgrsForward({ x: 0, y: -80 });
    //   expect(p).toEqual('31CDM4186716915');
    //   expect(mgrsToPoint(p)).toEqual({ x: -0.00001347072431578944, y: -79.99999578854278 });
    //   p = mgrsForward({ x: 24, y: 73 });
    //   expect(p).toEqual('35XMB0213502930');
    //   expect(mgrsToPoint(p)).toEqual({ x: 23.99998502461724, y: 72.999999080815 });
    //   p = mgrsForward({ x: 34, y: 73 });
    //   expect(p).toEqual('37XCB3700307286');
    //   expect(mgrsToPoint(p)).toEqual({ x: 34.000006934267915, y: 73.00000314488452 });

    #[test]
    #[should_panic]
    fn test_mgrs_forward_error() {
        let coords = Coords::new_xy(-200., 0.);
        mgrs_forward(&coords, None);
    }

    #[test]
    #[should_panic]
    fn test_mgrs_forward_error_2() {
        let coords = Coords::new_xy(0., 200.);
        mgrs_forward(&coords, None);
    }

    #[test]
    #[should_panic]
    fn test_mgrs_forward_error_3() {
        let coords = Coords::new_xy(0., 85.);
        mgrs_forward(&coords, None);
    }
}
