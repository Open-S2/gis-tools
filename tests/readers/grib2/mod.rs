#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;

    use alloc::{vec, vec::Vec};
    use gistools::{
        parsers::{Buffer, BufferReader, FeatureReader},
        readers::{
            GISReader, GRIB2Reader, Grib2LocalUseSection, Grib2SectionLocations, ReaderType,
            parse_idx,
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
}
