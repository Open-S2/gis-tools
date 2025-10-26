#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::{
        space::{Classification, Method, OperationMode, Satellite, TLEData, TLEDataCelestrak},
        util::Date,
    };
    use s2json::VectorPoint;

    #[test]
    fn test_sat_tle() {
        let tle = r#"0 VANGUARD 1
1 00005U 58002B   23048.45156751  .00000181  00000-0  25486-3 0  9999
2 00005  34.2454 104.3484 1845489  60.4612 316.7290 10.85063109311153"#;

        let parsed_tle = TLEData::from(tle);

        assert_eq!(
            parsed_tle,
            TLEData {
                name: "VANGUARD 1".into(),
                number: 5.0,
                class: Classification::U,
                id: "58002B".into(),
                date: Date { year: 2023, month: 2, day: 17, hour: 10, minute: 50, second: 15 },
                epochdays: 48.45156751,
                fdmm: 1.81e-6,
                sdmm: 0.0,
                drag: 0.00025486,
                ephemeris: 0.0,
                esn: 999.0,
                inclination: 34.2454,
                ascension: 104.3484,
                eccentricity: 0.1845489,
                perigee: 60.4612,
                anomaly: 316.729,
                motion: 10.85063109,
                revolution: 31115.0,
                rms: None
            }
        );

        let sat = Satellite::new(&parsed_tle, None);

        // expect(sat.jdsatepoch).toEqual(2459992.95156751);
        assert_eq!(sat.jdsatepoch, 2459992.9515625);
        // expect(sat.fdmm).toEqual(0.00000181);
        assert_eq!(sat.fdmm, 1.81e-6);
        // expect(sat.sdmm).toEqual(0);
        assert_eq!(sat.sdmm, 0.0);
        // expect(sat.drag).toEqual(0.00025486);
        assert_eq!(sat.drag, 0.00025486);
        // expect(sat.ephemeris).toEqual(0);
        assert_eq!(sat.ephemeris, 0.0);
        // expect(sat.esn).toEqual(999);
        assert_eq!(sat.esn, 999.0);
        // expect(sat.inclination).toEqual(0.5976949836624661);
        assert_eq!(sat.inclination, 0.5976949836624661);
        // expect(sat.ascension).toEqual(1.8212231491880508);
        assert_eq!(sat.ascension, 1.8212231491880508);
        // expect(sat.eccentricity).toEqual(0.1845489);
        assert_eq!(sat.eccentricity, 0.1845489);
        // expect(sat.perigee).toEqual(1.0552470097067956);
        assert_eq!(sat.perigee, 1.0552470097067956);
        // expect(sat.anomaly).toEqual(5.52796388654912);
        assert_eq!(sat.anomaly, 5.52796388654912);
        // expect(sat.motion).toEqual(0.04732152797251528);
        assert_eq!(sat.motion, 0.04732152797251528);
        // expect(sat.revolution).toEqual(31115);
        assert_eq!(sat.revolution, 31115.0);
        // expect(sat.opsmode).toEqual('i');
        assert_eq!(sat.opsmode, OperationMode::I);
        // expect(sat.rms).toBeUndefined();
        assert_eq!(sat.rms, None);
        // expect(sat.isimp).toEqual(0);
        assert_eq!(sat.isimp, 0.);
        // expect(sat.method).toEqual('n');
        assert_eq!(sat.method, Method::N);

        let prop_res = sat.sgp4(0.).unwrap();
        assert_eq!(
            prop_res.position,
            VectorPoint::new_xyz(
                -1888.380209293852,
                7382.363975001675,
                -0.002344430599056936,
                None
            )
        );
        assert_eq!(
            prop_res.velocity,
            VectorPoint::new_xyz(-5.7648543073985845, -2.6236442939223923, 4.246696588857323, None)
        );

        let prop_res = sat.propagate(&Date::from("2023-03-01T00:00:00.000Z")).unwrap();
        assert_eq!(
            prop_res.position,
            VectorPoint::new_xyz(
                -511.59664504700675,
                -8089.517506478646,
                -1669.8462475060617,
                None
            )
        );
        assert_eq!(
            prop_res.velocity,
            VectorPoint::new_xyz(5.818646422857667, -0.868646960485309, -3.9078275187843676, None)
        );

        assert_eq!(
            sat.gpu(),
            vec![
                5.52796388654912,
                0.04732152797251528,
                0.1845489,
                0.5976949836624661,
                1.0,
                1.0,
                0.00025486,
                0.0473448075185996,
                1.0552470097067956,
                5.4585673733659655e-5,
                1.8212231491880508,
                -3.7353169000612346e-5,
                -1.5581585858464615e-41,
                1.236851805808103e-37,
                7.533404547019768e-34,
                2.351372158223078e-32,
                1.8552777087121545e-37,
                0.0,
                2.4006326060906796e-40,
                0.7348621431923709,
                -1.200619534803893e-36,
                3.6172837200200485,
                2.436630860723393e-73,
                7.099882752433462e-109,
                2.4056625776151735e-144,
                -0.6854499062015231,
                2.7425913386295462e-73,
                6.2763409195776565e-109,
                1.7484671241513535e-144,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                1.928650307648475,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.000659830582190508,
                0.0012883490581074237,
                1.0499759291281268,
                0.31667469029062434,
                3.78327716796563,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0
            ]
        );
    }

    #[test]
    fn test_tledata_from_celestrak() {
        let src = TLEDataCelestrak {
            object_name: "ISS (ZARYA)".into(),
            object_id: "1998-067A".into(),
            epoch: "2024-10-01T12:34:56.000000Z".into(),
            mean_motion: 15.72125391,
            eccentricity: 0.0006703,
            inclination: 51.6416,
            ra_of_asc_node: 247.4627,
            arg_of_pericenter: 130.5360,
            mean_anomaly: 325.0288,
            ephemeris_type: 0.0,
            classification_type: "U".into(),
            norad_cat_id: 25544.0,
            element_set_no: 999.0,
            rev_at_epoch: 27393.0,
            bstar: 0.00014523,
            mean_motion_dot: 0.00016717,
            mean_motion_ddot: 0.0,
            rms: "0.0001".into(),
            data_source: "Celestrak".into(),
        };

        let converted = TLEData::from(&src);

        assert_eq!(
            converted,
            TLEData {
                name: "ISS (ZARYA)".into(),
                number: 25544.0,
                class: Classification::U,
                id: "1998-067A".into(),
                date: Date { year: 2024, month: 10, day: 1, hour: 12, minute: 34, second: 56 },
                epochdays: 305.52425925945863,
                fdmm: 0.00016717,
                sdmm: 0.0,
                drag: 0.00014523,
                ephemeris: 0.0,
                esn: 999.0,
                inclination: 51.6416,
                ascension: 247.4627,
                eccentricity: 0.0006703,
                perigee: 130.536,
                anomaly: 325.0288,
                motion: 15.72125391,
                revolution: 27393.0,
                rms: Some(0.0001)
            }
        );
    }
}
