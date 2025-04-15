use super::{
    Ellipsoid, Extent, Id, ProjBBox, TimeExtent, Unit, UnitObject, UnitType, Usage,
    ValueInMetreOrValueAndUnit, VerticalExtent,
};
use crate::readers::{WKTParser, WKTValue};

impl WKTParser for Usage {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut usage = Usage::default();
        if let WKTValue::Array(arr) = val {
            if !arr.is_empty() && arr[0].to_string() == "SCOPE" {
                if let WKTValue::Array(arr) = &arr[1] {
                    usage.scope = arr.first().map(|s| s.to_string()).unwrap_or_default();
                }
            }
            if arr.len() >= 2 {
                match arr[2].to_string().as_str() {
                    "TIMEEXTENT" => {
                        usage.extent = Some(Extent::TimeExtent(TimeExtent::from_wkt(&arr[3])));
                    }
                    "VERTICALEXTENT" => {
                        usage.extent =
                            Some(Extent::VerticalExtent(VerticalExtent::from_wkt(&arr[3])));
                    }
                    "BBOX" => {
                        usage.extent = Some(Extent::BBox(ProjBBox::from_wkt(&arr[3])));
                    }
                    "AREA" => {
                        if let WKTValue::Array(arr) = &arr[3] {
                            usage.extent = Some(Extent::Area(
                                arr.first().map(|s| s.to_string()).unwrap_or_default(),
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }
        usage
    }
}

impl WKTParser for ProjBBox {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut bbox = ProjBBox::default();
        if let WKTValue::Array(arr) = val {
            if arr.len() >= 4 {
                bbox.south_latitude = arr[0].to_float();
                bbox.west_longitude = arr[1].to_float();
                bbox.north_latitude = arr[2].to_float();
                bbox.east_longitude = arr[3].to_float();
            }
        }
        bbox
    }
}

impl WKTParser for VerticalExtent {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut ve = VerticalExtent::default();
        if let WKTValue::Array(arr) = val {
            if arr.len() >= 2 {
                ve.minimum = arr[0].to_float();
                ve.maximum = arr[1].to_float();
                if arr.len() == 4 && arr[2].to_string() == "LENGTHUNIT" {
                    ve.unit = Unit::from_wkt(&arr[3]);
                    ve.unit.set_unit_type(UnitType::LinearUnit);
                }
            }
        }
        ve
    }
}

impl WKTParser for Unit {
    /// Creates a new LengthUnit from a WKTValue
    fn from_wkt(unit_xml: &WKTValue) -> Self {
        let mut unit = UnitObject::default();
        if let WKTValue::Array(arr) = unit_xml {
            if !arr.is_empty() {
                unit.name = arr[0].to_string();
            }
            if arr.len() >= 2 {
                unit.conversion_factor = Some(arr[1].to_float());
            }
        }
        Unit::UnitObject(unit)
    }
}

impl WKTParser for TimeExtent {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut te = TimeExtent::default();
        if let WKTValue::Array(arr) = val {
            if arr.len() >= 2 {
                te.start = arr.first().map(|v| v.to_string()).unwrap_or_default();
                te.end = arr.get(1).map(|v| v.to_string()).unwrap_or_default();
            }
        }
        te
    }
}

impl WKTParser for Id {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut id = Id::default();
        if let WKTValue::Array(arr) = val {
            if arr.len() >= 2 {
                id.authority = arr[0].to_string();
                id.code = arr[1].to_string().into();

                let mut i = 2;
                while i < arr.len() {
                    match arr[i].to_string().as_str() {
                        "CITATION" => {
                            if let WKTValue::Array(arr) = &arr[i + 1] {
                                id.authority_citation =
                                    Some(arr.first().map(|s| s.to_string()).unwrap_or_default());
                            }
                            i += 2;
                        }
                        "URI" => {
                            if let WKTValue::Array(arr) = &arr[i + 1] {
                                id.uri =
                                    Some(arr.first().map(|s| s.to_string()).unwrap_or_default());
                            }
                            i += 2;
                        }
                        other => {
                            id.version = Some(other.into());
                            i += 1;
                        }
                    }
                }
            }
        }
        id
    }
}

impl WKTParser for Ellipsoid {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut ellipsoid = Ellipsoid::default();
        let mut unit = Unit::default();
        let mut semi_major_axis = 0.0;
        let mut inverse_flattening = 0.0;

        if let WKTValue::Array(arr) = val {
            if arr.len() >= 3 {
                ellipsoid.name = arr[0].to_string();
                semi_major_axis = arr[1].to_float();
                inverse_flattening = arr[2].to_float();

                let mut i = 3;
                while i < arr.len() {
                    if let Some(WKTValue::String(item_keyword)) = arr.get(i) {
                        match item_keyword.as_str() {
                            "LENGTHUNIT" => {
                                unit = Unit::from_wkt(&arr[i + 1]);
                                unit.set_unit_type(UnitType::LinearUnit);
                                i += 1;
                            }
                            "ID" => {
                                ellipsoid.base_properties.ids.push(Id::from_wkt(&arr[i + 1]));
                                i += 1;
                            }
                            _ => {}
                        }
                    }
                    i += 1;
                }
            }
        }
        // update semi-major axis and inverse-flattening, but if unit exists, build it correctly
        if let Unit::UnitObject(_) = unit {
            ellipsoid.semi_major_axis =
                Some(ValueInMetreOrValueAndUnit::from_unit(unit.clone(), semi_major_axis));
            ellipsoid.inverse_flattening =
                Some(ValueInMetreOrValueAndUnit::from_unit(unit, inverse_flattening));
        } else {
            ellipsoid.semi_major_axis = Some(semi_major_axis.into());
            ellipsoid.inverse_flattening = Some(inverse_flattening.into());
        }
        ellipsoid
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{proj::ValueAndUnit, readers::parse_wkt_object};
    use alloc::vec;

    #[test]
    fn test_length_unit_from_wkt() {
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
        let wkt_str1 = r#"TIMEEXTENT[2013-01-01,2013-12-31]"#;
        let wkt_value1 = parse_wkt_object(wkt_str1);
        if let WKTValue::Array(arr) = wkt_value1 {
            let te1 = TimeExtent::from_wkt(&arr[1]);
            assert_eq!(te1.start, "2013-01-01");
            assert_eq!(te1.end, "2013-12-31");
        } else {
            panic!("Expected an array");
        }

        let wkt_str2 = r#"TIMEEXTENT["Jurassic","Quaternary"]"#;
        let wkt_value2 = parse_wkt_object(wkt_str2);
        if let WKTValue::Array(arr) = wkt_value2 {
            let te2 = TimeExtent::from_wkt(&arr[1]);
            assert_eq!(te2.start, "Jurassic");
            assert_eq!(te2.end, "Quaternary");
        } else {
            panic!("Expected an array");
        }

        let wkt_str_invalid_count = r#"TIMEEXTENT[2013-01-01]"#;
        let wkt_value_invalid_count = parse_wkt_object(wkt_str_invalid_count);
        if let WKTValue::Array(arr) = wkt_value_invalid_count {
            let te_invalid = TimeExtent::from_wkt(&arr[1]);
            assert_eq!(te_invalid, TimeExtent::default());
        } else {
            panic!("Expected an array");
        }

        let wkt_str_empty = r#"TIMEEXTENT[]"#;
        let wkt_value_empty = parse_wkt_object(wkt_str_empty);
        if let WKTValue::Array(arr) = wkt_value_empty {
            let te_empty = TimeExtent::from_wkt(&arr[1]);
            assert_eq!(te_empty, TimeExtent::default());
        } else {
            panic!("Expected an array");
        }
    }

    #[test]
    fn test_usage_from_wkt() {
        let wkt_str_ve = r#"USAGE[SCOPE["Large scale topographic mapping and cadastre."],VERTICALEXTENT[-1000,0]]"#;
        let wkt_value_ve = parse_wkt_object(wkt_str_ve);
        if let WKTValue::Array(arr) = wkt_value_ve {
            let usage = Usage::from_wkt(&arr[1]);
            assert_eq!(usage.scope, "Large scale topographic mapping and cadastre.");
            assert!(usage.extent.is_some());
            if let Some(Extent::VerticalExtent(ve)) = usage.extent {
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
            let usage = Usage::from_wkt(&arr[1]);
            assert_eq!(usage.scope, "Validity period.");
            assert!(usage.extent.is_some());
            if let Some(Extent::TimeExtent(te)) = usage.extent {
                assert_eq!(te.start, "2023-01-01");
                assert_eq!(te.end, "2023-12-31");
            } else {
                panic!("Expected TimeExtent");
            }
        } else {
            panic!("Expected an array");
        }

        // Add tests for BBOX and AREA when their WKT parsing is implemented
        let wkt_str_bbox = r#"USAGE[SCOPE["Geographic coverage."],BBOX[10,20,30,40]]"#;
        let wkt_value_bbox = parse_wkt_object(wkt_str_bbox);
        if let WKTValue::Array(arr) = wkt_value_bbox {
            let usage = Usage::from_wkt(&arr[1]);
            assert_eq!(usage.scope, "Geographic coverage.");
            assert!(usage.extent.is_some());
            if let Some(Extent::BBox(b)) = usage.extent {
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
            let usage = Usage::from_wkt(&arr[1]);
            assert_eq!(usage.scope, "Description of area.");
            assert!(usage.extent.is_some());
            if let Some(Extent::Area(a)) = usage.extent {
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
            let usage = Usage::from_wkt(&arr[1]);
            assert_eq!(usage.scope, "");
            assert!(usage.extent.is_none());
        } else {
            panic!("Expected an array");
        }

        let wkt_str_empty = r#"USAGE[]"#;
        let wkt_value_empty = parse_wkt_object(wkt_str_empty);
        if let WKTValue::Array(arr) = wkt_value_empty {
            let usage_empty = Usage::from_wkt(&arr[1]);
            assert_eq!(usage_empty, Usage::default());
        } else {
            panic!("Expected an array");
        }
    }

    #[test]
    fn test_identifier_from_wkt() {
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
            assert_eq!(ellipsoid.base_properties.ids, vec![]);
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
                ellipsoid.base_properties.ids,
                vec![Id { authority: "EPSG".into(), code: "7030".into(), ..Default::default() }]
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
                ellipsoid.base_properties.ids,
                vec![Id { authority: "EPSG".into(), code: "7008".into(), ..Default::default() }]
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
                ellipsoid.base_properties.ids,
                vec![
                    Id { authority: "EPSG".into(), code: "7001".into(), ..Default::default() },
                    Id { authority: "IAU".into(), code: "629".into(), ..Default::default() },
                ]
            );
        } else {
            panic!("Expected an array");
        }
    }
}

// TODO:

// COMMON:
// - PARAMETER
// - PROJECTION
// - DATUM
// - - VDATUM  |  VRF  |  VERTICALDATUM
// - - TDATUM | TIMEDATUM
// - - EDATUM | ENGINEERINGDATUM
// - - PDATUM | PARAMETRICDATUM
// - - DATUM | GEODETICDATUM | TRF
// - PRIMEM | PRIMEMERIDIAN | MERIDIAN
// - UNIT
// - AXIS (and AxisDirection)
// - BEARING (used by axis rarely)

// Coordinate operations:
// - Transformation
// - Conversion
// - ConcatenatedOperation

// Common CRS:
// - CRS & CS
// - - PROJCS
// - - BoundCRS
// - - CompoundCRS | COMPD_CS
// - - EngineeringCRS | ENGCRS | LOCAL_CS
// - - ParametricCRS
// - - PROJCRS | PROJECTEDCRS | PROJCS
// - - TemporalCRS | TIMECRS
// - - VERTCRS | VERTICALCRS | VERT_CS
// - - GEOGCRS | GEOGRAPHICCRS | GEODCRS | GEODETICCRS | BASEGEODCRS

// used by transformation:
// - SOURCECRS
// - TARGETCRS

// ------------------------------------------------------------------------------------

// EXIST, esoteric:
// - ORDER
// - ANCHOR
// - ANCHOREPOCH
// - CONVERSION
// - METHOD
// - GEOIDMODEL
// - PARAMETERFILE
// - COORDINATEOPERATION
// - INTERPOLATIONCRS
// - OPERATIONACCURACY
// - CONCATENATEDOPERATION
// - STEP
// - ABRIDGEDTRANSFORMATION
// - DERIVINGCONVERSION
// - CALENDAR
// - TIMEORIGIN
// - DYNAMIC
// - FRAMEEPOCH
// - MODEL
// - VELOCITYGRID
// - ENSEMBLE
// - MEMBER
// - ENSEMBLEACCURACY
// - DERIVEDPROJCRS
// - BASEPROJCRS
// - PARAMETRICCRS
// - PARAMETRICUNIT
// - BASEVERTCRS
// - BASEENGCRS
// - BASEPARAMCRS
// - BASETIMECRS
// - TRF
// - VRF
// - TEMPORALQUANTITY
// - ENGINEERINGCRS
// - EPOCH
// - COORDEPOCH
// - COORDINATEMETADATA
// - POINTMOTIONOPERATION
// - VERSION
// - AXISMINVALUE
// - AXISMAXVALUE
// - RANGEMEANING
// - exact
// - wraparound

// CS types - esoteric?:
// - AFFINE
// - CARTESIAN
// - CYLINDRICAL
// - ELLIPSOIDAL
// - LINEAR
// - PARAMETRIC
// - POLAR
// - SPHERICAL
// - VERTICAL
// - TEMPORAL
// - TEMPORALCOUNT
// - TEMPORALMEASURE
// - ORDINAL
// - TEMPORALDATETIME

// 9.3.4      Map projection parameter
// Parameter name is for human readability. For interoperability it is the method formula and its parameters that are critical in determining the equivalence of methods. See Annex F. Identifiers for commonly encountered map projection methods are given in F.2; their parameters are listed in F.3.
// The map projection parameters required are specific to the map projection method and will be listed sequentially. The order within the sequence is not significant but should be logical.
// <map projection parameter unit> is an optional attribute, for reasons of backward compatibility. Best practice is that it is included explicitly in WKT strings.

// <parameter keyword>
// ::=
// PARAMETER
// <parameter name>
// ::=
// <quoted Latin text>                                       !! See 7.2
// <parameter value>
// ::=
// <signed numeric literal>
// <map projection parameter unit>
// ::=
// <length unit> | <angle unit> | <scale unit>
// !! See 9.3.4 for constraints

// Used by EPSG project v12:
// ID: 214278
// EPSG: 214276
// PARAMETER: 40825
// LENGTHUNIT: 40304
// AXIS: 28074
// ANGLEUNIT: 23917
// MEMBER: 19846
// CS: 14093
// ELLIPSOID: 12351
// DATUM: 10182
// METHOD: 8986
// SCALEUNIT: 5952
// CONVERSION: 5857
// BASEGEOGCRS: 5834
// PROJCRS: 5830
// GEOGCRS: 5768
// SOURCECRS: 3145
// TARGETCRS: 3140
// OPERATIONACCURACY: 2953
// COORDINATEOPERATION: 2948
// VERSION: 2948
// ENSEMBLE: 2191
// ENSEMBLEACCURACY: 2191
// GEOIDMODEL: 1820
// VERTCRS: 1713
// VDATUM: 1691
// PARAMETERFILE: 1427
// ANCHOREPOCH: 1155
// DYNAMIC: 1024
// FRAMEEPOCH: 1024
// GEODCRS: 749
// COMPOUNDCRS: 703
// STEP: 396
// DEFININGTRANSFORMATION: 313
// NADCON: 288 (name of method)
// TIMEUNIT: 217
// CONCATENATEDOPERATION: 192
// DERIVINGCONVERSION: 176
// BASEVERTCRS: 172
// PRIMEM: 150
// MERIDIAN: 102
// ENGCRS: 29
// EDATUM: 29
// POINTMOTIONOPERATION: 5
// DERIVEDPROJECTED: 4
// BASEPROJCRS: 4
