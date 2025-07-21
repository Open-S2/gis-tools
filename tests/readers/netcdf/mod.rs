#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;

    use alloc::vec;
    use gistools::{
        parsers::{BufferReader, FeatureReader, FileReader},
        readers::{
            CDFDataType, CDFDimension, CDFRecordDimension, CDFValue, CDFVariable, GISReader,
            NetCDFReader, NetCDFReaderOptions, ReaderType, netcdf_type_to_bytes,
        },
    };
    use s2json::{GetXY, VectorPoint};
    use std::{collections::BTreeMap, fs, path::PathBuf};

    #[test]
    #[should_panic(expected = "Not a valid NetCDF file: should start with CDF")]
    fn test_netcdf_reader() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/netcdf/fixtures/not_nc.txt");

        let _ = NetCDFReader::new(FileReader::from(path.clone()), None);
    }

    #[test]
    fn test_netcdf_madis_sao() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/netcdf/fixtures/madis-sao.nc");

        let netcdf_reader = NetCDFReader::new(FileReader::from(path.clone()), None);

        assert!(!netcdf_reader.is64);
        assert_eq!(
            netcdf_reader.record_dimension,
            CDFRecordDimension {
                size: 178,
                id: Some(21),
                name: Some("recNum".into()),
                record_step: Some(1220)
            }
        );

        assert_eq!(
            netcdf_reader.dimensions,
            vec![
                CDFDimension { index: 0, name: "maxAutoStaLen".into(), size: 6 },
                CDFDimension { index: 1, name: "maxAutoWeather".into(), size: 5 },
                CDFDimension { index: 2, name: "maxAutoWeaLen".into(), size: 12 },
                CDFDimension { index: 3, name: "maxCldTypeLen".into(), size: 5 },
                CDFDimension { index: 4, name: "maxCloudTypes".into(), size: 5 },
                CDFDimension { index: 5, name: "maxDataSrcLen".into(), size: 8 },
                CDFDimension { index: 6, name: "maxRepLen".into(), size: 5 },
                CDFDimension { index: 7, name: "maxSAOLen".into(), size: 256 },
                CDFDimension { index: 8, name: "maxSkyCover".into(), size: 5 },
                CDFDimension { index: 9, name: "maxSkyLen".into(), size: 8 },
                CDFDimension { index: 10, name: "maxSkyMethLen".into(), size: 3 },
                CDFDimension { index: 11, name: "maxStaNamLen".into(), size: 5 },
                CDFDimension { index: 12, name: "maxWeatherNum".into(), size: 5 },
                CDFDimension { index: 13, name: "maxWeatherLen".into(), size: 40 },
                CDFDimension { index: 14, name: "QCcheckNum".into(), size: 10 },
                CDFDimension { index: 15, name: "QCcheckNameLen".into(), size: 60 },
                CDFDimension { index: 16, name: "ICcheckNum".into(), size: 55 },
                CDFDimension { index: 17, name: "ICcheckNameLen".into(), size: 72 },
                CDFDimension { index: 18, name: "maxStaticIds".into(), size: 350 },
                CDFDimension { index: 19, name: "totalIdLen".into(), size: 6 },
                CDFDimension { index: 20, name: "nInventoryBins".into(), size: 24 },
                CDFDimension { index: 21, name: "recNum".into(), size: 0 },
            ]
        );

        assert_eq!(
            netcdf_reader.variables[0],
            CDFVariable {
                name: "nStaticIds".into(),
                dimensions: vec![],
                attributes: BTreeMap::from([("_FillValue".into(), (0.0).into())]),
                r#type: 4.into(),
                size: 4,
                offset: 39208,
                record: false
            }
        );
        assert_eq!(
            netcdf_reader.variables[11],
            CDFVariable {
                name: "wmoId".into(),
                dimensions: vec![CDFDimension { index: 21, name: "recNum".into(), size: 0 }],
                attributes: BTreeMap::from([
                    ("_FillValue".into(), (-2147483647.0).into()),
                    ("long_name".into(), "WMO numeric station ID".into()),
                    ("reference".into(), "station table".into()),
                    ("valid_range".into(), (vec![1.0, 89999.0]).into()),
                ]),
                r#type: 4.into(),
                size: 4,
                offset: 48884,
                record: true
            }
        );
    }

    #[test]
    fn test_netcdf_ichthyop() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/netcdf/fixtures/ichthyop.nc");
        let data = fs::read(path.clone()).unwrap();

        let netcdf_reader = NetCDFReader::new(
            BufferReader::from(data),
            Some(NetCDFReaderOptions {
                lon_key: Some("lon".into()),
                lat_key: Some("lat".into()),
                height_key: Some("depth".into()),
                prop_fields: Some(vec!["depth".into()]),
            }),
        );

        assert!(!netcdf_reader.is_empty());
        assert_eq!(netcdf_reader.len(), 49);

        for i in 0..49 {
            let _ = netcdf_reader.get_properties(i);
        }

        let point = netcdf_reader.get_point(0).unwrap();
        assert_eq!(
            VectorPoint::from_xy(point.x(), point.y()),
            VectorPoint::new(-9.00235366821289, 53.26256561279297, None, None)
        );

        let features: Vec<_> = netcdf_reader.iter().collect();
        assert_eq!(features.len(), 49);

        let geo = features[0].geometry.point().unwrap();
        assert_eq!(
            VectorPoint::from_xy(geo.x(), geo.y()),
            VectorPoint::new(-9.00235366821289, 53.26256561279297, None, None)
        );

        // par iter
        let features: Vec<_> = netcdf_reader.par_iter(1, 0).collect();
        assert_eq!(features.len(), 49);

        let geo = features[0].geometry.point().unwrap();
        assert_eq!(
            VectorPoint::from_xy(geo.x(), geo.y()),
            VectorPoint::new(-9.00235366821289, 53.26256561279297, None, None)
        );
    }

    #[test]
    fn test_netcdf_64_bit() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/netcdf/fixtures/model1_md2.nc");

        let netcdf_reader = NetCDFReader::new(FileReader::from(path.clone()), None);

        assert!(netcdf_reader.is_empty());
        assert!(netcdf_reader.is64);
        assert_eq!(netcdf_reader.len(), 0);

        let variable = netcdf_reader.get_data_variable("cell_angular".into()).unwrap();
        assert_eq!(variable[0], "a".into());
        let variable = netcdf_reader.get_data_variable("cell_spatial".into()).unwrap();
        assert_eq!(variable[0], "a".into());
    }

    #[test]
    fn test_netcdf_cdf_value() {
        let test = CDFValue::default();
        assert_eq!(test, CDFValue::Number(0.0));

        assert_eq!(test.to_num(), 0.0);
        assert_eq!(CDFValue::Number(1.1).to_num(), 1.1);
        assert_eq!(CDFValue::Number(1.1).get_index(0), 0.);
        assert_eq!(CDFValue::Array(vec![1.1]).to_num(), 0.);
    }

    #[test]
    fn test_netcdf_cdf_data_type() {
        let test = CDFDataType::default();
        assert_eq!(test, CDFDataType::BYTE);
        assert_eq!(CDFDataType::BYTE, 1_u64.into());
        assert_eq!(CDFDataType::BYTE, 0_u64.into());
        assert_eq!(CDFDataType::BYTE, 7_u64.into());
        assert_eq!(CDFDataType::CHAR, 2_u64.into());
        assert_eq!(CDFDataType::SHORT, 3_u64.into());
        assert_eq!(CDFDataType::INT, 4_u64.into());
        assert_eq!(CDFDataType::FLOAT, 5_u64.into());
        assert_eq!(CDFDataType::DOUBLE, 6_u64.into());

        assert_eq!(netcdf_type_to_bytes(CDFDataType::BYTE), 1);
        assert_eq!(netcdf_type_to_bytes(CDFDataType::CHAR), 1);
        assert_eq!(netcdf_type_to_bytes(CDFDataType::SHORT), 2);
        assert_eq!(netcdf_type_to_bytes(CDFDataType::INT), 4);
        assert_eq!(netcdf_type_to_bytes(CDFDataType::FLOAT), 4);
        assert_eq!(netcdf_type_to_bytes(CDFDataType::DOUBLE), 8);
    }

    #[test]
    fn test_netcdf_agilent() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/netcdf/fixtures/agilent_hplc.cdf");

        let netcdf_reader = NetCDFReader::new(FileReader::from(path.clone()), None);

        let features: Vec<_> = netcdf_reader.iter().collect();
        assert_eq!(features.len(), 0);
    }

    #[test]
    fn test_netcdf_gis_reader() {
        // file
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/netcdf/fixtures/agilent_hplc.cdf");
        let gis_reader = GISReader::from_path(path.clone(), None, None);
        assert_eq!(gis_reader.get_type(), ReaderType::NetCDF);
        let features: Vec<_> = gis_reader.iter().collect();
        assert_eq!(features.len(), 0);

        // buffer
        let bytes = std::fs::read(path.clone()).unwrap();
        let gis_reader = GISReader::from_buffer(bytes, ReaderType::NetCDF, None);
        let features: Vec<_> = gis_reader.par_iter(1, 0).collect();
        assert_eq!(features.len(), 0);
    }
}
