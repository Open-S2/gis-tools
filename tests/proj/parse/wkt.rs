#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;
    extern crate std;

    use alloc::vec;
    use gistools::{
        parsers::{WKTParser, WKTValue, parse_wkt_object},
        proj::{
            Axis, AxisDirection, CRS, Conversion, CoordinateSystem, CoordinateSystemSubtype,
            DatumEnsemble, DatumEnsembleMember, Ellipsoid, EngineeringDatum, GeodeticCRS,
            GeodeticReferenceFrame, Id, Meridian, Method, ObjectUsage, ParameterValue,
            ParametricDatum, PrimeMeridian, ProjBBox, ProjJSON, ProjValue, ProjectedCRS,
            TemporalDatum, TemporalExtent, Unit, UnitObject, UnitType, ValueAndUnit,
            ValueInDegreeOrValueAndUnit, ValueInMetreOrValueAndUnit, VerticalExtent,
            VerticalReferenceFrame,
        },
    };

    #[test]
    fn test_length_unit_from_wkt() {
        let wkt_str = r#"LENGTHUNIT[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = Unit::from_wkt(&arr[1]);
            assert_eq!(
                usage,
                Unit::UnitObject(UnitObject {
                    r#type: UnitType::Unit,
                    name: "".into(),
                    conversion_factor: None,
                    id: None,
                    ids: vec![]
                })
            );
        } else {
            panic!("Expected an array");
        }

        let usage = Unit::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(
            usage,
            Unit::UnitObject(UnitObject {
                r#type: UnitType::Unit,
                name: "".into(),
                conversion_factor: None,
                id: None,
                ids: vec![]
            })
        );

        let wkt_str_metre = r#"LENGTHUNIT["metre",1.0]"#;
        let wkt_value_metre = parse_wkt_object(wkt_str_metre);
        if let WKTValue::Array(arr) = wkt_value_metre {
            if let Unit::UnitObject(unit_metre) = Unit::from_wkt(&arr[1]) {
                assert_eq!(unit_metre.name, "metre");
                assert_eq!(unit_metre.conversion_factor, Some(1.0));
            } else {
                panic!("Expected a unit object");
            }
        } else {
            panic!("Expected an array");
        }

        let wkt_str_foot = r#"LENGTHUNIT["foot",0.3048]"#;
        let wkt_value_foot = parse_wkt_object(wkt_str_foot);
        if let WKTValue::Array(arr) = wkt_value_foot {
            if let Unit::UnitObject(unit_foot) = Unit::from_wkt(&arr[1]) {
                assert_eq!(unit_foot.name, "foot");
                assert_eq!(unit_foot.conversion_factor, Some(0.3048));
            }
        } else {
            panic!("Expected an array");
        }
    }

    #[test]
    fn test_vertical_extent_from_wkt() {
        let wkt_str = r#"VERTICALEXTENT[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = VerticalExtent::from_wkt(&arr[1]);
            assert_eq!(usage, VerticalExtent::default());
        } else {
            panic!("Expected an array");
        }

        let usage = VerticalExtent::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, VerticalExtent::default());

        let wkt_str1 = r#"VERTICALEXTENT[-1000,0,LENGTHUNIT["metre",1.0]]"#;
        let wkt_value1 = parse_wkt_object(wkt_str1);
        if let WKTValue::Array(arr) = wkt_value1 {
            let ve1 = VerticalExtent::from_wkt(&arr[1]);
            assert_eq!(ve1.minimum, -1000.0);
            assert_eq!(ve1.maximum, 0.0);
            if let Unit::UnitObject(unit) = ve1.unit {
                assert_eq!(unit.name, "metre");
                assert_eq!(unit.conversion_factor, Some(1.0));
            }
        } else {
            panic!("Expected an array");
        }

        let wkt_str2 = r#"VERTICALEXTENT[-1000,0]"#;
        let wkt_value2 = parse_wkt_object(wkt_str2);
        if let WKTValue::Array(arr) = wkt_value2 {
            let ve2 = VerticalExtent::from_wkt(&arr[1]);
            assert_eq!(ve2.minimum, -1000.0);
            assert_eq!(ve2.maximum, 0.0);
            assert_eq!(ve2.unit, Unit::default());
        } else {
            panic!("Expected an array");
        }

        let wkt_str3 = r#"VERTICALEXTENT[-50,100,LENGTHUNIT["foot",0.3048]]"#;
        let wkt_value3 = parse_wkt_object(wkt_str3);
        if let WKTValue::Array(arr) = wkt_value3 {
            let ve3 = VerticalExtent::from_wkt(&arr[1]);
            assert_eq!(ve3.minimum, -50.0);
            assert_eq!(ve3.maximum, 100.0);
            if let Unit::UnitObject(unit) = ve3.unit {
                assert_eq!(unit.r#type, UnitType::LinearUnit);
                assert_eq!(unit.name, "foot");
                assert_eq!(unit.conversion_factor, Some(0.3048));
            }
        } else {
            panic!("Expected an array");
        }

        let wkt_str_invalid_format = r#"VERTICALEXTENT[-1000]"#;
        let wkt_value_invalid_format = parse_wkt_object(wkt_str_invalid_format);
        if let WKTValue::Array(arr) = wkt_value_invalid_format {
            let ve_invalid = VerticalExtent::from_wkt(&arr[1]);
            assert_eq!(ve_invalid, VerticalExtent::default());
        } else {
            panic!("Expected an array");
        }

        let wkt_str_nan = r#"VERTICALEXTENT[abc,def]"#;
        let wkt_value_nan = parse_wkt_object(wkt_str_nan);
        if let WKTValue::Array(arr) = wkt_value_nan {
            let ve_nan = VerticalExtent::from_wkt(&arr[1]);
            assert_eq!(ve_nan, VerticalExtent::default());
        } else {
            panic!("Expected an array");
        }

        let wkt_str_empty = r#"VERTICALEXTENT[]"#;
        let wkt_value_empty = parse_wkt_object(wkt_str_empty);
        if let WKTValue::Array(arr) = wkt_value_empty {
            let ve_empty = VerticalExtent::from_wkt(&arr[1]);
            assert_eq!(ve_empty, VerticalExtent::default());
        } else {
            panic!("Expected an array");
        }
    }

    #[test]
    fn test_temporal_extent_from_wkt() {
        let wkt_str = r#"TIMEEXTENT[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = TemporalExtent::from_wkt(&arr[1]);
            assert_eq!(usage, TemporalExtent::default());
        } else {
            panic!("Expected an array");
        }

        let usage = TemporalExtent::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, TemporalExtent::default());

        let wkt_str1 = r#"TIMEEXTENT[2013-01-01,2013-12-31]"#;
        let wkt_value1 = parse_wkt_object(wkt_str1);
        if let WKTValue::Array(arr) = wkt_value1 {
            let te1 = TemporalExtent::from_wkt(&arr[1]);
            assert_eq!(te1.start, "2013-01-01");
            assert_eq!(te1.end, "2013-12-31");
        } else {
            panic!("Expected an array");
        }

        let wkt_str2 = r#"TIMEEXTENT["Jurassic","Quaternary"]"#;
        let wkt_value2 = parse_wkt_object(wkt_str2);
        if let WKTValue::Array(arr) = wkt_value2 {
            let te2 = TemporalExtent::from_wkt(&arr[1]);
            assert_eq!(te2.start, "Jurassic");
            assert_eq!(te2.end, "Quaternary");
        } else {
            panic!("Expected an array");
        }

        let wkt_str_invalid_count = r#"TIMEEXTENT[2013-01-01]"#;
        let wkt_value_invalid_count = parse_wkt_object(wkt_str_invalid_count);
        if let WKTValue::Array(arr) = wkt_value_invalid_count {
            let te_invalid = TemporalExtent::from_wkt(&arr[1]);
            assert_eq!(te_invalid, TemporalExtent::default());
        } else {
            panic!("Expected an array");
        }

        let wkt_str_empty = r#"TIMEEXTENT[]"#;
        let wkt_value_empty = parse_wkt_object(wkt_str_empty);
        if let WKTValue::Array(arr) = wkt_value_empty {
            let te_empty = TemporalExtent::from_wkt(&arr[1]);
            assert_eq!(te_empty, TemporalExtent::default());
        } else {
            panic!("Expected an array");
        }
    }

    #[test]
    fn test_usage_from_wkt() {
        let wkt_str = r#"USAGE[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = ObjectUsage::from_wkt(&arr[1]);
            assert_eq!(usage, ObjectUsage::default());
        } else {
            panic!("Expected an array");
        }

        let usage = ObjectUsage::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, ObjectUsage::default());

        let wkt_str_ve = r#"USAGE[SCOPE["Large scale topographic mapping and cadastre."],VERTICALEXTENT[-1000,0]]"#;
        let wkt_value_ve = parse_wkt_object(wkt_str_ve);
        if let WKTValue::Array(arr) = wkt_value_ve {
            let usage = ObjectUsage::from_wkt(&arr[1]);
            assert_eq!(usage.scope, "Large scale topographic mapping and cadastre.");
            assert!(usage.vertical_extent.is_some());
            if let Some(ve) = usage.vertical_extent {
                assert_eq!(ve.minimum, -1000.0);
                assert_eq!(ve.maximum, 0.0);
            } else {
                panic!("Expected VerticalExtent");
            }
        } else {
            panic!("Expected an array");
        }

        let wkt_str_te = r#"USAGE[SCOPE["Validity period."],TIMEEXTENT[2023-01-01,2023-12-31]]"#;
        let wkt_value_te = parse_wkt_object(wkt_str_te);
        if let WKTValue::Array(arr) = wkt_value_te {
            let usage = ObjectUsage::from_wkt(&arr[1]);
            assert_eq!(usage.scope, "Validity period.");
            assert!(usage.temporal_extent.is_some());
            if let Some(te) = usage.temporal_extent {
                assert_eq!(te.start, "2023-01-01");
                assert_eq!(te.end, "2023-12-31");
            } else {
                panic!("Expected TemporalExtent");
            }
        } else {
            panic!("Expected an array");
        }

        // Add tests for BBOX and AREA when their WKT parsing is implemented
        let wkt_str_bbox = r#"USAGE[SCOPE["Geographic coverage."],BBOX[10,20,30,40]]"#;
        let wkt_value_bbox = parse_wkt_object(wkt_str_bbox);
        if let WKTValue::Array(arr) = wkt_value_bbox {
            let usage = ObjectUsage::from_wkt(&arr[1]);
            assert_eq!(usage.scope, "Geographic coverage.");
            assert!(usage.bbox.is_some());
            if let Some(b) = usage.bbox {
                // Temporarily assuming VerticalExtent
                assert_eq!(b.south_latitude, 10.0);
                assert_eq!(b.west_longitude, 20.0);
                assert_eq!(b.north_latitude, 30.0);
                assert_eq!(b.east_longitude, 40.0);
            } else {
                panic!("Expected BBox");
            }
        } else {
            panic!("Expected an array");
        }

        let wkt_str_area = r#"USAGE[SCOPE["Description of area."],AREA["Some area description."]]"#;
        let wkt_value_area = parse_wkt_object(wkt_str_area);
        if let WKTValue::Array(arr) = wkt_value_area {
            let usage = ObjectUsage::from_wkt(&arr[1]);
            assert_eq!(usage.scope, "Description of area.");
            assert!(usage.area.is_some());
            if let Some(a) = usage.area {
                // Temporarily assuming VerticalExtent
                assert_eq!(a, "Some area description.");
            } else {
                panic!("Expected Area");
            }
        } else {
            panic!("Expected an array");
        }

        let wkt_str_invalid = r#"USAGE[INVALID["data"],OTHER["stuff"]]"#;
        let wkt_value_invalid = parse_wkt_object(wkt_str_invalid);
        if let WKTValue::Array(arr) = wkt_value_invalid {
            let usage = ObjectUsage::from_wkt(&arr[1]);
            assert_eq!(usage.scope, "");
        } else {
            panic!("Expected an array");
        }

        let wkt_str_empty = r#"USAGE[]"#;
        let wkt_value_empty = parse_wkt_object(wkt_str_empty);
        if let WKTValue::Array(arr) = wkt_value_empty {
            let usage_empty = ObjectUsage::from_wkt(&arr[1]);
            assert_eq!(usage_empty, ObjectUsage::default());
        } else {
            panic!("Expected an array");
        }
    }

    #[test]
    fn test_identifier_from_wkt() {
        let wkt_str = r#"ID[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = Id::from_wkt(&arr[1]);
            assert_eq!(usage, Id::default());
        } else {
            panic!("Expected an array");
        }

        let wkt_str = r#"ID["Authority name","Abcd_Ef",CITATION[],URI[]]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = Id::from_wkt(&arr[1]);
            assert_eq!(
                usage,
                Id {
                    authority: "Authority name".into(),
                    code: "Abcd_Ef".into(),
                    authority_citation: Some("".into()),
                    uri: Some("".into()),
                    ..Default::default()
                }
            );
        } else {
            panic!("Expected an array");
        }

        let usage = Id::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, Id::default());

        let wkt_str1 = r#"ID["Authority name","Abcd_Ef",7.1]"#;
        let wkt_value1 = parse_wkt_object(wkt_str1);
        if let WKTValue::Array(arr) = wkt_value1 {
            let id1 = Id::from_wkt(&arr[1]);
            assert_eq!(id1.authority, "Authority name");
            assert_eq!(id1.code.string(), "Abcd_Ef");
            assert_eq!(id1.version, Some("7.1".into()));
            assert_eq!(id1.authority_citation, None);
            assert_eq!(id1.uri, None);
        } else {
            panic!("Expected an array");
        }

        let wkt_str2 = r#"ID["EPSG",4326]"#;
        let wkt_value2 = parse_wkt_object(wkt_str2);
        if let WKTValue::Array(arr) = wkt_value2 {
            let id2 = Id::from_wkt(&arr[1]);
            assert_eq!(id2.authority, "EPSG");
            assert_eq!(id2.code.string(), "4326");
            assert_eq!(id2.version, None);
            assert_eq!(id2.authority_citation, None);
            assert_eq!(id2.uri, None);
        } else {
            panic!("Expected an array");
        }

        let wkt_str3 = r#"ID["EPSG",4326,URI["urn:ogc:def:crs:EPSG::4326"]]"#;
        let wkt_value3 = parse_wkt_object(wkt_str3);
        if let WKTValue::Array(arr) = wkt_value3 {
            let id3 = Id::from_wkt(&arr[1]);
            assert_eq!(id3.authority, "EPSG");
            assert_eq!(id3.code.i64(), 4326);
            assert_eq!(id3.version, None);
            assert_eq!(id3.authority_citation, None);
            assert!(id3.uri.is_some());
            assert_eq!(id3.uri.unwrap(), "urn:ogc:def:crs:EPSG::4326");
        } else {
            panic!("Expected an array");
        }

        let wkt_str4 = r#"ID["EuroGeographics","ES_ED50 (BAL99) to ETRS89","2001-04-20"]"#;
        let wkt_value4 = parse_wkt_object(wkt_str4);
        if let WKTValue::Array(arr) = wkt_value4 {
            let id4 = Id::from_wkt(&arr[1]);
            assert_eq!(id4.authority, "EuroGeographics");
            assert_eq!(id4.code.string(), "ES_ED50 (BAL99) to ETRS89");
            assert_eq!(id4.version, Some("2001-04-20".into()));
            assert_eq!(id4.authority_citation, None);
            assert_eq!(id4.uri, None);
        } else {
            panic!("Expected an array");
        }

        let wkt_str_citation = r#"ID["Authority","ID1",CITATION["Some citation text"]]"#;
        let wkt_value_citation = parse_wkt_object(wkt_str_citation);
        if let WKTValue::Array(arr) = wkt_value_citation {
            let id_citation = Id::from_wkt(&arr[1]);
            assert!(id_citation.authority_citation.is_some());
            assert_eq!(id_citation.authority_citation.unwrap(), "Some citation text");
        } else {
            panic!("Expected an array");
        }

        let wkt_str_uri_citation =
            r#"ID["Authority","ID1",CITATION["Some citation"],URI["http://example.com"]]"#;
        let wkt_value_uri_citation = parse_wkt_object(wkt_str_uri_citation);
        if let WKTValue::Array(arr) = wkt_value_uri_citation {
            let id_uri_citation = Id::from_wkt(&arr[1]);
            assert!(id_uri_citation.authority_citation.is_some());
            assert_eq!(id_uri_citation.authority_citation.unwrap(), "Some citation");
            assert!(id_uri_citation.uri.is_some());
            assert_eq!(id_uri_citation.uri.unwrap(), "http://example.com");
        } else {
            panic!("Expected an array");
        }

        let wkt_str_empty = r#"ID[]"#;
        let wkt_value_empty = parse_wkt_object(wkt_str_empty);
        if let WKTValue::Array(arr) = wkt_value_empty {
            let id_empty = Id::from_wkt(&arr[1]);
            assert_eq!(id_empty, Id::default());
        } else {
            panic!("Expected an array");
        }
    }

    #[test]
    fn test_parse_ellipsoid_minimal() {
        let wkt_str = r#"ELLIPSOID[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = Ellipsoid::from_wkt(&arr[1]);
            assert_eq!(usage, Ellipsoid::default());
        } else {
            panic!("Expected an array");
        }

        let usage = Ellipsoid::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, Ellipsoid::default());

        let wkt_str = r#"ELLIPSOID["GRS 1980",6378137,298.257222101]"#;
        let wkt_obj = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_obj {
            let ellipsoid = Ellipsoid::from_wkt(&arr[1]);
            assert_eq!(ellipsoid.name, "GRS 1980");
            assert_eq!(ellipsoid.semi_major_axis, Some((6378137.0).into()));
            assert_eq!(ellipsoid.inverse_flattening, Some((298.257222101).into()));
        } else {
            panic!("Expected an array");
        }
    }

    #[test]
    fn test_parse_ellipsoid_with_lengthunit() {
        let wkt_str = r#"ELLIPSOID["Airy 1830",6377563.396,299.3249646,LENGTHUNIT["metre",1.0]]"#;
        let wkt_obj = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_obj {
            let ellipsoid = Ellipsoid::from_wkt(&arr[1]);
            assert_eq!(ellipsoid.name, "Airy 1830");
            assert_eq!(
                ellipsoid.semi_major_axis,
                Some(ValueInMetreOrValueAndUnit::ValueAndUnit(ValueAndUnit {
                    value: 6377563.396,
                    unit: Unit::UnitObject(UnitObject {
                        r#type: UnitType::LinearUnit,
                        name: "metre".into(),
                        conversion_factor: Some(1.0),
                        ..Default::default()
                    }),
                }))
            );
            assert_eq!(
                ellipsoid.inverse_flattening,
                Some(ValueInMetreOrValueAndUnit::ValueAndUnit(ValueAndUnit {
                    value: 299.3249646,
                    unit: Unit::UnitObject(UnitObject {
                        r#type: UnitType::LinearUnit,
                        name: "metre".into(),
                        conversion_factor: Some(1.0),
                        ..Default::default()
                    })
                }))
            );
            assert_eq!(ellipsoid.ids, vec![]);
        } else {
            panic!("Expected an array");
        }
    }

    #[test]
    fn test_parse_ellipsoid_with_id() {
        let wkt_str = r#"ELLIPSOID["WGS 84",6378137,298.257223563,ID["EPSG",7030]]"#;
        let wkt_obj = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_obj {
            let ellipsoid = Ellipsoid::from_wkt(&arr[1]);
            assert_eq!(ellipsoid.name, "WGS 84");
            assert_eq!(ellipsoid.semi_major_axis, Some((6378137.0).into()));
            assert_eq!(ellipsoid.inverse_flattening, Some((298.257223563).into()));
            assert_eq!(
                ellipsoid.id,
                Some(Id { authority: "EPSG".into(), code: "7030".into(), ..Default::default() })
            );
        } else {
            panic!("Expected an array");
        }
    }

    #[test]
    fn test_parse_ellipsoid_with_lengthunit_and_id() {
        let wkt_str = r#"ELLIPSOID["Clarke 1866",6378206.4,294.9786982,LENGTHUNIT["US survey foot",0.304800609601219],ID["EPSG",7008]]"#;
        let wkt_obj = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_obj {
            let ellipsoid = Ellipsoid::from_wkt(&arr[1]);
            assert_eq!(ellipsoid.name, "Clarke 1866");
            assert_eq!(
                ellipsoid.semi_major_axis,
                Some(ValueInMetreOrValueAndUnit::ValueAndUnit(ValueAndUnit {
                    value: 6378206.4,
                    unit: Unit::UnitObject(UnitObject {
                        r#type: UnitType::LinearUnit,
                        name: "US survey foot".into(),
                        conversion_factor: Some(0.304800609601219),
                        ..Default::default()
                    }),
                }))
            );
            assert_eq!(
                ellipsoid.inverse_flattening,
                Some(ValueInMetreOrValueAndUnit::ValueAndUnit(ValueAndUnit {
                    value: 294.9786982,
                    unit: Unit::UnitObject(UnitObject {
                        r#type: UnitType::LinearUnit,
                        name: "US survey foot".into(),
                        conversion_factor: Some(0.304800609601219),
                        ..Default::default()
                    })
                }))
            );
            assert_eq!(
                ellipsoid.id,
                Some(Id { authority: "EPSG".into(), code: "7008".into(), ..Default::default() })
            );
        } else {
            panic!("Expected an array");
        }
    }

    #[test]
    fn test_parse_ellipsoid_with_multiple_ids() {
        let wkt_str =
            r#"ELLIPSOID["Airy 1830",6377563.396,299.3249646,ID["EPSG",7001],ID["IAU",629]]"#;
        let wkt_obj = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_obj {
            let ellipsoid = Ellipsoid::from_wkt(&arr[1]);
            assert_eq!(ellipsoid.name, "Airy 1830");
            assert_eq!(ellipsoid.semi_major_axis, Some((6377563.396).into()));
            assert_eq!(ellipsoid.inverse_flattening, Some((299.3249646).into()));
            assert_eq!(
                ellipsoid.ids,
                vec![
                    Id { authority: "EPSG".into(), code: "7001".into(), ..Default::default() },
                    Id { authority: "IAU".into(), code: "629".into(), ..Default::default() },
                ]
            );
        } else {
            panic!("Expected an array");
        }
    }

    #[test]
    fn test_parameter_value_from_wkt() {
        let wkt_str = r#"PARAMETER[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = ParameterValue::from_wkt(&arr[1]);
            assert_eq!(usage, ParameterValue::default());
        } else {
            panic!("Expected an array");
        }

        let usage = ParameterValue::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, ParameterValue::default());

        let wkt_str_number = r#"PARAMETER["semi_major","6378137.0"]"#;
        let wkt_value_number = parse_wkt_object(wkt_str_number);
        if let WKTValue::Array(arr) = wkt_value_number {
            let parameter = ParameterValue::from_wkt(&arr[1]);
            assert_eq!(parameter.name, "semi_major");
            assert_eq!(parameter.value.f64(), 6378137.0);
            assert_eq!(parameter.unit, None);
            assert!(parameter.ids.is_empty());
        } else {
            panic!("Expected an array");
        }

        let wkt_str_with_unit =
            r#"PARAMETER["central_meridian","0.0",UNIT["degree","0.0174532925199433"]]"#;
        let wkt_value_with_unit = parse_wkt_object(wkt_str_with_unit);
        if let WKTValue::Array(arr) = wkt_value_with_unit {
            let parameter = ParameterValue::from_wkt(&arr[1]);
            assert_eq!(parameter.name, "central_meridian");
            assert_eq!(parameter.value.f64(), 0.0);
            assert!(parameter.unit.is_some());
            if let Some(Unit::UnitObject(unit)) = parameter.unit {
                assert_eq!(unit.name, "degree");
                assert_eq!(unit.conversion_factor, Some(0.0174532925199433));
            } else {
                panic!("Expected a UnitObject");
            }
            assert!(parameter.ids.is_empty());
        } else {
            panic!("Expected an array");
        }

        let wkt_str_with_id = r#"PARAMETER["standard_parallel_1",30.0,ID["EPSG","8831"]]"#;
        let wkt_value_with_id = parse_wkt_object(wkt_str_with_id);
        if let WKTValue::Array(arr) = wkt_value_with_id {
            let parameter = ParameterValue::from_wkt(&arr[1]);
            assert_eq!(parameter.name, "standard_parallel_1");
            assert_eq!(parameter.value.f64(), 30.0);
            assert_eq!(
                parameter.id,
                Some(Id { authority: "EPSG".into(), code: "8831".into(), ..Default::default() })
            );
            assert_eq!(parameter.unit, None);
        } else {
            panic!("Expected an array");
        }

        let wkt_str_with_all = r#"PARAMETER["latitude_of_origin",0.5,UNIT["degree",0.0174532925199433],ID["EPSG",8821]]"#;
        let wkt_value_with_all = parse_wkt_object(wkt_str_with_all);
        if let WKTValue::Array(arr) = wkt_value_with_all {
            let parameter = ParameterValue::from_wkt(&arr[1]);
            assert_eq!(parameter.name, "latitude_of_origin");
            assert_eq!(parameter.value.f64(), 0.5);
            assert!(parameter.unit.is_some());
            if let Some(Unit::UnitObject(unit)) = parameter.unit {
                assert_eq!(unit.name, "degree");
                assert_eq!(unit.conversion_factor, Some(0.0174532925199433));
            } else {
                panic!("Expected a UnitObject");
            }
            assert_eq!(
                parameter.id,
                Some(Id { authority: "EPSG".into(), code: "8821".into(), ..Default::default() },)
            )
        } else {
            panic!("Expected an array");
        }
    }

    #[test]
    fn test_cs_value_from_wkt() {
        let wkt_str = r#"CS[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = CoordinateSystem::from_wkt(&arr[1]);
            assert_eq!(usage, CoordinateSystem::default());
        } else {
            panic!("Expected an array");
        }

        let usage = CoordinateSystem::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, CoordinateSystem::default());

        let wkt_str = r#"CS[ellipsoidal,2]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let cs = CoordinateSystem::from_wkt(&arr[1]);
            assert_eq!(cs.subtype, CoordinateSystemSubtype::Ellipsoidal);
        }
    }

    #[test]
    fn test_axis_value_from_wkt() {
        let wkt_str = r#"AXIS[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = Axis::from_wkt(&arr[1]);
            assert_eq!(usage, Axis::default());
        } else {
            panic!("Expected an array");
        }

        let usage = Axis::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, Axis::default());

        let wkt_str = r#"AXIS["latitude",north,ORDER[1]],"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let axis = Axis::from_wkt(&arr[1]);
            assert_eq!(axis.name, "latitude");
            assert_eq!(axis.direction, AxisDirection::North);
            assert_eq!(axis.order, 1);
        }
    }

    #[test]
    fn test_conversion() {
        let wkt_str = r#"CONVERSION[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = Conversion::from_wkt(&arr[1]);
            assert_eq!(usage, Conversion::default());
        } else {
            panic!("Expected an array");
        }

        let usage = Conversion::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, Conversion::default());

        let wkt_str = r#"CONVERSION["UTM zone 10N",METHOD["Transverse Mercator",ID["EPSG",9807]],PARAMETER["Latitude of natural origin",0,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8801]],PARAMETER["Longitude of natural origin",-123,ANGLEUNIT["degree",0.0174532925199433],ID["EPSG",8802]],PARAMETER["Scale factor at natural origin",0.9996,SCALEUNIT["unity",1.0],ID["EPSG",8805]],PARAMETER["False easting",500000,LENGTHUNIT["metre",1.0],ID["EPSG",8806]],PARAMETER["False northing",0,LENGTHUNIT["metre",1.0],ID["EPSG",8807]]]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let conversion = Conversion::from_wkt(&arr[1]);
            assert_eq!(conversion.name, "UTM zone 10N");
            // assert_eq!(conversion.method, Some("Transverse Mercator".into()));
            assert_eq!(conversion.parameters.len(), 5);
        }
    }

    #[test]
    fn test_method() {
        let wkt_str = r#"METHOD[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = Method::from_wkt(&arr[1]);
            assert_eq!(usage, Method::default());
        } else {
            panic!("Expected an array");
        }

        let usage = Method::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, Method::default());

        let wkt_str = r#"METHOD["Transverse Mercator",ID["EPSG",9807]]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let method = Method::from_wkt(&arr[1]);
            assert_eq!(method.name, "Transverse Mercator");
            assert_eq!(
                method.id,
                Some(Id { authority: "EPSG".into(), code: "9807".into(), ..Default::default() })
            );
        }
    }

    #[test]
    fn test_proj_bbox() {
        let wkt_str = r#"BBOX[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = ProjBBox::from_wkt(&arr[1]);
            assert_eq!(usage, ProjBBox::default());
        } else {
            panic!("Expected an array");
        }

        let usage = ProjBBox::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, ProjBBox::default());
    }

    #[test]
    fn test_proj() {
        let wkt_string = r#"PROJCRS["WGS 84 / Pseudo-Mercator",
            BASEGEOGCRS["WGS 84",
                ENSEMBLE["World Geodetic System 1984 ensemble",
                    MEMBER["World Geodetic System 1984 (Transit)", ID["EPSG",1166]],
                    MEMBER["World Geodetic System 1984 (G730)", ID["EPSG",1152]],
                    MEMBER["World Geodetic System 1984 (G873)", ID["EPSG",1153]],
                    MEMBER["World Geodetic System 1984 (G1150)", ID["EPSG",1154]],
                    MEMBER["World Geodetic System 1984 (G1674)", ID["EPSG",1155]],
                    MEMBER["World Geodetic System 1984 (G1762)", ID["EPSG",1156]],
                    MEMBER["World Geodetic System 1984 (G2139)", ID["EPSG",1309]],
                    MEMBER["World Geodetic System 1984 (G2296)", ID["EPSG",1383]],
                    ELLIPSOID["WGS 84",6378137,298.257223563,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7030]],
                    ENSEMBLEACCURACY[2],
                    ID["EPSG",6326]],
               ID["EPSG",4326]],
            CONVERSION["Popular Visualisation Pseudo-Mercator",
                METHOD["Popular Visualisation Pseudo Mercator",ID["EPSG",1024]],
                PARAMETER["Latitude of natural origin",0,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8801]],
                PARAMETER["Longitude of natural origin",0,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],
                PARAMETER["False easting",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],
                PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],
                ID["EPSG",3856]],
            CS[Cartesian,2,ID["EPSG",4499]],
            AXIS["Easting (X)",east],
            AXIS["Northing (Y)",north],
            LENGTHUNIT["metre",1,ID["EPSG",9001]],
            ID["EPSG",3857]]"#;

        let proj_obj = ProjJSON::parse_wkt(wkt_string);
        // println!("proj_obj: {proj_obj:#?}");
        if let ProjJSON::CRS(crs) = proj_obj {
            if let CRS::ProjectedCRS(proj_crs) = crs.as_ref() {
                assert_eq!(
                    proj_crs.conversion.method.name,
                    "Popular Visualisation Pseudo Mercator"
                );
                assert_eq!(
                    proj_crs.conversion.method.id,
                    Some(Id {
                        authority: "EPSG".into(),
                        code: "1024".into(),
                        ..Default::default()
                    })
                )
            } else {
                panic!("Not a projected CRS");
            }
        } else {
            panic!("Not a CRS");
        }
        // assert_eq!(proj_obj.name, "WGS 84 / Pseudo-Mercator");
    }

    #[test]
    fn test_proj_name() {
        let wkt_str = "PROJCS[\"CH1903 / \
                       LV03\",GEOGCS[\"CH1903\",DATUM[\"D_CH1903\",SPHEROID[\"Bessel_1841\",\
                       6377397.155,299.1528128]],PRIMEM[\"Greenwich\",0],UNIT[\"Degree\",0.\
                       017453292519943295]],PROJECTION[\"Hotine_Oblique_Mercator_Azimuth_Center\"\
                       ],PARAMETER[\"latitude_of_center\",46.95240555555556],PARAMETER[\"\
                       longitude_of_center\",7.439583333333333],PARAMETER[\"azimuth\",90],\
                       PARAMETER[\"scale_factor\",1],PARAMETER[\"false_easting\",600000],\
                       PARAMETER[\"false_northing\",200000],UNIT[\"Meter\",1]]"
            .into();

        let _proj_obj = ProjJSON::parse_wkt(wkt_str);
        println!("proj_obj: {_proj_obj:#?}");
    }

    #[test]
    fn datum_ensemble() {
        let wkt_str = r#"ENSEMBLE[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = DatumEnsemble::from_wkt(&arr[1]);
            assert_eq!(usage, DatumEnsemble::default());
        } else {
            panic!("Expected an array");
        }

        let usage = DatumEnsemble::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, DatumEnsemble::default());
    }

    #[test]
    fn prime_meridian() {
        let wkt_str = r#"PRIMEMERIDIAN[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = PrimeMeridian::from_wkt(&arr[1]);
            assert_eq!(usage, PrimeMeridian::default());
        } else {
            panic!("Expected an array");
        }

        let usage = PrimeMeridian::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, PrimeMeridian::default());
    }

    #[test]
    fn reference_frame_geodetic() {
        let wkt_str = r#"GEODETICDATUM[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = GeodeticReferenceFrame::from_wkt(&arr[1]);
            assert_eq!(usage, GeodeticReferenceFrame::default());
        } else {
            panic!("Expected an array");
        }

        let usage = GeodeticReferenceFrame::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, GeodeticReferenceFrame::default());
    }

    #[test]
    fn geodetic_crs() {
        let wkt_str = r#"GEOGRAPHICCRS[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = GeodeticCRS::from_wkt(&arr[1]);
            assert_eq!(usage, GeodeticCRS::default());
        } else {
            panic!("Expected an array");
        }

        let usage = GeodeticCRS::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, GeodeticCRS::default());
    }

    #[test]
    fn projected_crs() {
        let wkt_str = r#"PROJECTEDCRS[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = ProjectedCRS::from_wkt(&arr[1]);
            assert_eq!(usage, ProjectedCRS::default());
        } else {
            panic!("Expected an array");
        }

        let usage = ProjectedCRS::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, ProjectedCRS::default());
    }

    #[test]
    #[should_panic(expected = "Expected one of")]
    fn proj_json_fail() {
        let _proj_json =
            ProjJSON::from_wkt(&WKTValue::Array(vec![WKTValue::String("none".to_string())]));

        let usage = ProjJSON::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, ProjJSON::default());
    }

    #[test]
    fn proj_json_default() {
        let proj_json = ProjJSON::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(proj_json, ProjJSON::default());

        let proj_json = ProjJSON::from_wkt(&WKTValue::Array(vec![]));
        assert_eq!(proj_json, ProjJSON::default());
    }

    #[test]
    fn wkt_catchall() {
        let wkt_str = r#"METHOD["",MEMBERS[],EPOCH[],FRAMEEPOCH[],TDATUM[],EDATUM[],PDATUM[],MERIDIAN[],AREA[],ANCHOR[],USAGE[],PROJECTION[],ORDER[]]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = Method::from_wkt(&arr[1]);
            assert_eq!(usage, Method::default());
        } else {
            panic!("Expected an array");
        }
    }

    #[test]
    fn datum_ensemble_member() {
        let wkt_str = r#"MEMBERS[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = DatumEnsembleMember::from_wkt(&arr[1]);
            assert_eq!(usage, DatumEnsembleMember::default());
        } else {
            panic!("Expected an array");
        }

        let usage = DatumEnsembleMember::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, DatumEnsembleMember::default());

        let wkt_str = r#"MEMBERS["members",ID["EPSG",1234]]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let members = DatumEnsembleMember::from_wkt(&arr[1]);
            assert_eq!(
                members,
                DatumEnsembleMember {
                    name: "members".into(),
                    id: Some(Id {
                        authority: "EPSG".into(),
                        code: ProjValue::String("1234".into()),
                        version: None,
                        authority_citation: None,
                        uri: None
                    }),
                    ids: vec![]
                }
            );
        }
    }

    #[test]
    fn meridian() {
        let wkt_str = r#"MERIDIAN[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = Meridian::from_wkt(&arr[1]);
            assert_eq!(usage, Meridian::default());
        } else {
            panic!("Expected an array");
        }

        let usage = Meridian::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, Meridian::default());

        let wkt_str = r#"MERIDIAN[LENGTHUNIT["metre",1.0],ID["EPSG",1234]]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let members = Meridian::from_wkt(&arr[1]);
            assert_eq!(
                members,
                Meridian {
                    id: Some(Id {
                        authority: "EPSG".into(),
                        code: ProjValue::String("1234".into()),
                        version: None,
                        authority_citation: None,
                        uri: None
                    }),
                    ids: vec![],
                    schema: None,
                    r#type: None,
                    longitude: ValueInDegreeOrValueAndUnit::ValueAndUnit(ValueAndUnit {
                        value: 0.0,
                        unit: Unit::UnitObject(UnitObject {
                            r#type: UnitType::Unit,
                            name: "metre".into(),
                            conversion_factor: Some(1.0),
                            id: None,
                            ids: vec![]
                        })
                    })
                }
            );
        }
    }

    #[test]
    fn vertical_reference_frame() {
        let wkt_str = r#"VERTICALDATUM[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = VerticalReferenceFrame::from_wkt(&arr[1]);
            assert_eq!(usage, VerticalReferenceFrame::default());
        } else {
            panic!("Expected an array");
        }

        let usage = VerticalReferenceFrame::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, VerticalReferenceFrame::default());

        let wkt_str = r#"VERTICALDATUM["TEST",ANCHOR["anchor",0],ANCHOREPOCH[1.1]]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let members = VerticalReferenceFrame::from_wkt(&arr[1]);
            assert_eq!(
                members,
                VerticalReferenceFrame {
                    r#type: None,
                    name: "TEST".into(),
                    anchor: Some("anchor".into()),
                    anchor_epoch: Some(1.1),
                    usage: None,
                    usages: vec![],
                }
            );
        }
    }

    #[test]
    fn temporal_datum() {
        let wkt_str = r#"TDATUM[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = TemporalDatum::from_wkt(&arr[1]);
            assert_eq!(usage, TemporalDatum::default());
        } else {
            panic!("Expected an array");
        }

        let usage = TemporalDatum::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, TemporalDatum::default());

        let wkt_str = r#"TDATUM["TEST",USAGE[SCOPE["Geographic coverage."],BBOX[10,20,30,40]]]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let members = TemporalDatum::from_wkt(&arr[1]);
            assert_eq!(
                members,
                TemporalDatum {
                    r#type: None,
                    name: "TEST".into(),
                    calendar: "".into(),
                    time_origin: None,
                    usage: Some(ObjectUsage {
                        schema: None,
                        scope: "Geographic coverage.".into(),
                        area: None,
                        bbox: Some(ProjBBox {
                            south_latitude: 10.0,
                            west_longitude: 20.0,
                            north_latitude: 30.0,
                            east_longitude: 40.0
                        }),
                        vertical_extent: None,
                        temporal_extent: None,
                        remarks: None,
                        id: None,
                        ids: vec![]
                    }),
                    usages: vec![]
                }
            );
        }
    }

    #[test]
    fn engineering_datum() {
        let wkt_str = r#"EDATUM[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = EngineeringDatum::from_wkt(&arr[1]);
            assert_eq!(usage, EngineeringDatum::default());
        } else {
            panic!("Expected an array");
        }

        let usage = EngineeringDatum::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, EngineeringDatum::default());

        let wkt_str = r#"EDATUM["TEST",USAGE[SCOPE["Geographic coverage."],BBOX[10,20,30,40]]]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let members = EngineeringDatum::from_wkt(&arr[1]);
            assert_eq!(
                members,
                EngineeringDatum {
                    r#type: None,
                    name: "TEST".into(),
                    usage: Some(ObjectUsage {
                        schema: None,
                        scope: "Geographic coverage.".into(),
                        area: None,
                        bbox: Some(ProjBBox {
                            south_latitude: 10.0,
                            west_longitude: 20.0,
                            north_latitude: 30.0,
                            east_longitude: 40.0
                        }),
                        vertical_extent: None,
                        temporal_extent: None,
                        remarks: None,
                        id: None,
                        ids: vec![]
                    }),
                    usages: vec![],
                    anchor: None,
                }
            );
        }
    }

    #[test]
    fn parametric_datum() {
        let wkt_str = r#"PDATUM[]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let usage = ParametricDatum::from_wkt(&arr[1]);
            assert_eq!(usage, ParametricDatum::default());
        } else {
            panic!("Expected an array");
        }

        let usage = ParametricDatum::from_wkt(&WKTValue::String("".to_string()));
        assert_eq!(usage, ParametricDatum::default());

        let wkt_str = r#"PDATUM["TEST",USAGE[SCOPE["Geographic coverage."],BBOX[10,20,30,40]]]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let members = ParametricDatum::from_wkt(&arr[1]);
            assert_eq!(
                members,
                ParametricDatum {
                    r#type: None,
                    name: "TEST".into(),
                    usage: Some(ObjectUsage {
                        schema: None,
                        scope: "Geographic coverage.".into(),
                        area: None,
                        bbox: Some(ProjBBox {
                            south_latitude: 10.0,
                            west_longitude: 20.0,
                            north_latitude: 30.0,
                            east_longitude: 40.0
                        }),
                        vertical_extent: None,
                        temporal_extent: None,
                        remarks: None,
                        id: None,
                        ids: vec![]
                    }),
                    usages: vec![],
                    anchor: "".into(),
                }
            );
        }
    }
}
