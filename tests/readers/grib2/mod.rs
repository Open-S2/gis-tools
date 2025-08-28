#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;

    use crate::spawn_test_server;
    use alloc::{vec, vec::Vec};
    use gistools::{
        parsers::{Buffer, BufferReader, FeatureReader},
        readers::{
            GISReader, GRIB2Reader, Grib2AtmosGFSProduct, Grib2GFSDomain, Grib2GFSHour,
            Grib2GFSSource, Grib2LocalUseSection, Grib2SectionLocations, Grib2WaveGFSProduct,
            ReaderType, TableCategory, fetch_gfs_data, parse_idx,
        },
    };
    use s2json::VectorPoint;
    use std::{
        cmp::Ordering,
        fs::File,
        io::{BufRead, BufReader},
        path::PathBuf,
    };

    #[test]
    fn grib2_parsed_idx() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/grib2/fixtures/ref_sec0.gdas.t12z.pgrb2.1p00.anl.75r.grib2.txt");

        let data = std::fs::read_to_string(path).unwrap();
        let sections = parse_idx(
            data,
            vec![":DZDT:0.01 mb:".into(), ":TMP:0.4 mb:".into(), ":ABSV:0.4 mb:anl:".into()],
            None,
        );

        assert_eq!(
            sections,
            vec![
                Grib2SectionLocations {
                    start: 1231864,
                    line: "12:1231864:d=2024042612:DZDT:0.01 mb:anl:".into(),
                    end: Some(1337928),
                    name: ":DZDT:0.01 mb:".into(),
                },
                Grib2SectionLocations {
                    start: 7024838,
                    line: "68:7024838:d=2024042612:TMP:0.4 mb:anl:".into(),
                    end: Some(7122757),
                    name: ":TMP:0.4 mb:".into(),
                },
                Grib2SectionLocations {
                    start: 7710271,
                    line: "75:7710271:d=2024042612:ABSV:0.4 mb:anl:".into(),
                    end: None,
                    name: ":ABSV:0.4 mb:anl:".into(),
                },
            ]
        )
    }

    #[test]
    fn grib2_base_case() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/grib2/fixtures/ref_simple_packing.grib2.spread.txt");
        let file = File::open(path).unwrap();
        let mut expected_lines = BufReader::new(file).lines();
        // Skip the header
        expected_lines.next();
        let mut expected_points: Vec<VectorPoint<f64>> = vec![];
        for line in expected_lines {
            let line = line.unwrap();
            let mut parts = line.split(',').map(str::trim);
            let lon = parts.next().unwrap().parse::<f64>().unwrap();
            let lat = parts.next().unwrap().parse::<f64>().unwrap();
            let value = parts.next().unwrap().parse::<f64>().unwrap();
            expected_points.push(VectorPoint::new_xy(lon, lat, Some(value)));
        }

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/grib2/fixtures/ref_simple_packing.grib2");

        let bytes = std::fs::read(path.clone()).unwrap();
        let grib2_reader = GRIB2Reader::new(BufferReader::from(bytes).into(), vec![]);

        let mut points = grib2_reader.get_data().unwrap();
        points.sort_by(|a, b| {
            if a.y > b.y {
                return Ordering::Greater;
            } else if a.y < b.y {
                return Ordering::Less;
            } else if a.x > b.x {
                return Ordering::Greater;
            } else if a.x < b.x {
                return Ordering::Less;
            } else {
                return Ordering::Equal;
            }
        });

        for i in 0..points.len() {
            assert_eq!(points[i].x, expected_points[i].x);
            assert_eq!(points[i].y, expected_points[i].y);
            assert_eq!(points[i].z, expected_points[i].z);
            let m_value = points[i]
                .m
                .as_ref()
                .unwrap()
                .get("0")
                .unwrap()
                .to_prim()
                .unwrap()
                .to_f64()
                .unwrap();
            assert_eq!(m_value, expected_points[i].m.unwrap());
        }
    }

    #[test]
    fn test_grib2_gis_reader() {
        // file
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/grib2/fixtures/ref_simple_packing.grib2");
        let gis_reader = GISReader::from_path(path.clone(), None, None);
        assert_eq!(gis_reader.get_type(), ReaderType::GRIB2);
        let features: Vec<_> = gis_reader.iter().collect();
        assert_eq!(features.len(), 1);

        // buffer
        let bytes = std::fs::read(path.clone()).unwrap();
        let gis_reader = GISReader::from_buffer(bytes, ReaderType::GRIB2, None);
        let features: Vec<_> = gis_reader.par_iter(1, 0).collect();
        assert_eq!(features.len(), 1);
    }

    #[test]
    fn test_grib2_local_use_section() {
        let mut buf = Buffer::new(vec![]);
        buf.set_u8(0);
        buf.set_u32(655360);
        buf.copy_from_slice(5, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let data = buf.take();
        let local_use = Grib2LocalUseSection::new(&BufferReader::from(data));
        assert_eq!(
            local_use,
            Grib2LocalUseSection {
                section_number: 0,
                length: 10,
                contents: BufferReader::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
            }
        );
    }

    #[test]
    fn test_grib2_filter_sections() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/grib2/fixtures/ref_sec0.gdas.t12z.pgrb2.1p00.anl.75r.grib2.txt");

        let idx_data = std::fs::read_to_string(path).unwrap();
        let sections = parse_idx(
            idx_data,
            vec![":DZDT:0.01 mb:".into(), ":TMP:0.4 mb:".into(), ":ABSV:0.4 mb:anl:".into()],
            None,
        );

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/grib2/fixtures/ref_sec0.gdas.t12z.pgrb2.1p00.anl.75r.grib2");
        let bytes = std::fs::read(path.clone()).unwrap();
        let grib2_reader = GRIB2Reader::from_idx(&BufferReader::from(bytes), sections);

        let packet_products: Vec<_> = grib2_reader
            .packets
            .borrow()
            .iter()
            .map(|p| {
                let product_definition = p.product_definition.as_ref().unwrap();
                product_definition.values.values().clone()
            })
            .collect();

        assert_eq!(
            packet_products,
            vec![
                TableCategory {
                    parameter: "Vertical Velocity (Geometric)".into(),
                    units: "m s-1".into(),
                    abbrev: "DZDT".into()
                },
                TableCategory {
                    parameter: "Temperature".into(),
                    units: "K".into(),
                    abbrev: "TMP".into()
                },
                TableCategory {
                    parameter: "Absolute Vorticity".into(),
                    units: "s-1".into(),
                    abbrev: "ABSV".into()
                }
            ]
        );
    }

    #[test]
    fn test_fetch_gfs_atmos() {
        smol::block_on(async {
            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            path.push("tests/readers/grib2/fixtures/");
            let path_str: String = path.to_str().unwrap().into();
            let server = spawn_test_server(&path_str);
            let grib2_reader = fetch_gfs_data(
                Grib2GFSSource::Other(format!("{server}/")),
                Grib2AtmosGFSProduct::Pgrb2b1p00,
                Grib2GFSDomain::Atmos,
                "2024".into(),
                "12".into(),
                "14".into(),
                "12".into(),
                Some("003".into()),
                Some(vec!["TMP:2 m".into()]),
            )
            .await;

            let packet_products: Vec<_> = grib2_reader
                .packets
                .borrow()
                .iter()
                .map(|p| {
                    let product_definition = p.product_definition.as_ref().unwrap();
                    product_definition.values.values().clone()
                })
                .collect();

            assert_eq!(
                packet_products,
                vec![TableCategory {
                    parameter: "Temperature".into(),
                    units: "K".into(),
                    abbrev: "TMP".into()
                }]
            );
        });
    }

    // #[test]
    // fn grib2_waves() {
    //     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     path.push("tests/readers/grib2/fixtures/gfs.20250219/00/wave/gridded/gfs_wave_global.csv");
    //     let file = File::open(path).unwrap();
    //     let mut expected_lines = BufReader::new(file).lines();
    //     // Skip the header
    //     expected_lines.next();
    //     let mut expected_points: Vec<VectorPoint<(f64, f64)>> = vec![];
    //     for line in expected_lines {
    //         let line = line.unwrap();
    //         let mut parts = line.split(',').map(str::trim);
    //         let lon = parts.next().unwrap().parse::<f64>().unwrap();
    //         let lat = parts.next().unwrap().parse::<f64>().unwrap();
    //         let u = parts.next().unwrap_or_default().parse::<f64>().unwrap_or_default();
    //         let v = parts.next().unwrap_or_default().parse::<f64>().unwrap_or_default();
    //         expected_points.push(VectorPoint::new_xy(lon, lat, Some((u, v))));
    //     }

    //     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     path.push("tests/readers/grib2/fixtures/gfs.20250219/00/wave/gridded/gfswave.t00z.global.0p16.f000.grib2");

    //     let bytes = std::fs::read(path.clone()).unwrap();
    //     let grib2_reader = GRIB2Reader::new(BufferReader::from(bytes).into(), vec![]);

    //     let mut points = grib2_reader.get_data().unwrap();
    //     points.sort_by(|a, b| {
    //         if a.y > b.y {
    //             return Ordering::Greater;
    //         } else if a.y < b.y {
    //             return Ordering::Less;
    //         } else if a.x > b.x {
    //             return Ordering::Greater;
    //         } else if a.x < b.x {
    //             return Ordering::Less;
    //         } else {
    //             return Ordering::Equal;
    //         }
    //     });

    //     for i in 0..points.len() {
    //         assert_eq!(points[i].x, expected_points[i].x);
    //         assert_eq!(points[i].y, expected_points[i].y);
    //         assert_eq!(points[i].z, expected_points[i].z);
    //         // let m_value = points[i]
    //         //     .m
    //         //     .as_ref()
    //         //     .unwrap()
    //         //     .get("0")
    //         //     .unwrap()
    //         //     .to_prim()
    //         //     .unwrap()
    //         //     .to_f64()
    //         //     .unwrap();
    //         // assert_eq!(m_value, expected_points[i].m.unwrap());
    //     }
    // }

    #[test]
    fn test_grib2_gfs_source() {
        let variants = [
            ("aws", Grib2GFSSource::Aws),
            ("ftpprd", Grib2GFSSource::Ftpprd),
            ("nomads", Grib2GFSSource::Nomads),
            ("google", Grib2GFSSource::Google),
            ("azure", Grib2GFSSource::Azure),
            ("custom", Grib2GFSSource::Other("custom".into())),
        ];

        for (s, expected) in variants {
            let conv: Grib2GFSSource = s.into();
            assert_eq!(conv, expected);
            if let Grib2GFSSource::Other(v) = conv {
                assert_eq!(v, s);
            } else {
                assert!(conv.to_url().starts_with("http"));
            }
        }
    }

    #[test]
    fn test_grib2_atmos_gfs_product() {
        let variants = [
            ("pgrb2.0p25", Grib2AtmosGFSProduct::Pgrb20p25),
            ("pgrb2.0p50", Grib2AtmosGFSProduct::Pgrb20p50),
            ("pgrb2.1p00", Grib2AtmosGFSProduct::Pgrb21p00),
            ("pgrb2b.0p25", Grib2AtmosGFSProduct::Pgrb2b0p25),
            ("pgrb2b.0p50", Grib2AtmosGFSProduct::Pgrb2b0p50),
            ("pgrb2b.1p00", Grib2AtmosGFSProduct::Pgrb2b1p00),
            ("pgrb2full.0p50", Grib2AtmosGFSProduct::Pgrb2full0p50),
            ("sfluxgrb", Grib2AtmosGFSProduct::Sfluxgrb),
            ("goesimpgrb2.0p25", Grib2AtmosGFSProduct::Goesimpgrb20p25),
            ("custom", Grib2AtmosGFSProduct::Other("custom".into())),
        ];

        for (s, expected) in variants {
            let conv: Grib2AtmosGFSProduct = s.into();
            assert_eq!(conv, expected);
            let back: String = conv.clone().into();
            match expected {
                Grib2AtmosGFSProduct::Other(_) => assert_eq!(back, s),
                _ => assert_eq!(back, s),
            }
        }
    }

    #[test]
    fn test_grib2_wave_gfs_product() {
        let variants = [
            ("arctic.9km", Grib2WaveGFSProduct::Arctic9km),
            ("atlocn.0p16", Grib2WaveGFSProduct::Atlocn0p16),
            ("epacif.0p16", Grib2WaveGFSProduct::Epacif0p16),
            ("global.0p16", Grib2WaveGFSProduct::Global0p16),
            ("global.0p25", Grib2WaveGFSProduct::Global0p25),
            ("gsouth.0p25", Grib2WaveGFSProduct::Gsouth0p25),
            ("wcoast.0p16", Grib2WaveGFSProduct::Wcoast0p16),
            ("custom", Grib2WaveGFSProduct::Other("custom".into())),
        ];

        for (s, expected) in variants {
            let conv: Grib2WaveGFSProduct = s.into();
            assert_eq!(conv, expected);
            let back: String = conv.clone().into();
            match expected {
                Grib2WaveGFSProduct::Other(_) => assert_eq!(back, s),
                _ => assert_eq!(back, s),
            }
        }
    }

    #[test]
    fn test_grib2_gfs_hour() {
        let variants = [
            ("00", Grib2GFSHour::Hour0),
            ("06", Grib2GFSHour::Hour6),
            ("12", Grib2GFSHour::Hour12),
            ("18", Grib2GFSHour::Hour18),
        ];

        for (s, expected) in variants {
            let conv: Grib2GFSHour = s.into();
            assert_eq!(conv, expected);
            let back: String = conv.into();
            assert_eq!(back, s);
        }
    }

    #[test]
    #[should_panic]
    fn test_invalid_hour_panics() {
        let _: Grib2GFSHour = "03".into();
    }
}
