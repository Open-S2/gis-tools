use super::{
    Axis, CoordinateSystem, DatumEnsemble, DatumEnsembleMember, Ellipsoid, Extent,
    GeodeticReferenceFrame, Id, Meridian, Method, ParameterValue, PrimeMeridian, ProjBBox,
    TimeExtent, ToProjJSON, Unit, UnitObject, UnitType, Usage, ValueInDegreeOrValueAndUnit,
    ValueInMetreOrValueAndUnit, VerticalExtent,
};
use crate::{
    proj::Datum,
    readers::{WKTParser, WKTValue},
};
use alloc::format;

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
                if arr.len() >= 4 && arr[2].to_string() == "LENGTHUNIT" {
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

                // get unit
                handle_common_fields(&mut unit, arr, 3);
                // get ID
                handle_common_fields(&mut ellipsoid, arr, 3);
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

impl WKTParser for ParameterValue {
    fn from_wkt(wkt: &WKTValue) -> Self {
        let mut parameter_value = ParameterValue::default();

        if let WKTValue::Array(arr) = wkt {
            if arr.len() >= 2 {
                // FIRST value is the name
                parameter_value.name = arr[0].to_string();
                parameter_value.value = arr[1].to_string().into();
            }
            handle_common_fields(&mut parameter_value, arr, 2);
        }
        parameter_value
    }
}

impl WKTParser for CoordinateSystem {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut cs = CoordinateSystem::default();
        if let WKTValue::Array(arr) = val {
            if !arr.is_empty() {
                cs.subtype = serde_json::from_str(&format!("\"{}\"", arr[0].to_string()))
                    .unwrap_or_default();
            }
        }
        cs
    }
}

impl WKTParser for Axis {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut axis = Axis::default();
        if let WKTValue::Array(arr) = val {
            if arr.len() >= 2 {
                axis.name = arr[0].to_string(); // TODO input is BOTH name and abbr... parse correctly (not really important)
                axis.direction = serde_json::from_str(&format!("\"{}\"", arr[1].to_string()))
                    .unwrap_or_default();
            }
            // NOTE: BEARING and ORDER exist, but add no value in modern WKT
        }
        axis
    }
}

impl WKTParser for DatumEnsembleMember {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut datum_ensemble_member = DatumEnsembleMember::default();
        if let WKTValue::Array(arr) = val {
            if let Some(name) = arr.first() {
                datum_ensemble_member.name = name.to_string();
            }
            for id in arr.iter().skip(1) {
                datum_ensemble_member.ids.push(Id::from_wkt(id));
            }
        }
        datum_ensemble_member
    }
}

impl WKTParser for Method {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut method = Method::default();
        if let WKTValue::Array(arr) = val {
            if let Some(name) = arr.first() {
                method.name = name.to_string();
            }
            for id in arr.iter().skip(1) {
                method.ids.push(Id::from_wkt(id));
            }
        }
        method
    }
}

impl WKTParser for DatumEnsemble {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut ensemble = DatumEnsemble::default();
        if let WKTValue::Array(arr) = val {
            if let Some(name) = arr.first() {
                ensemble.name = name.to_string();
            }
            handle_common_fields(&mut ensemble, arr, 1);
        }
        ensemble
    }
}

impl WKTParser for PrimeMeridian {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut pm = PrimeMeridian::default();
        if let WKTValue::Array(arr) = val {
            if let Some(name) = arr.first() {
                pm.name = name.to_string();
            }
            let mut unit = Unit::default();
            if let Some(unit_xml) = arr.get(2) {
                unit = Unit::from_wkt(unit_xml);
            }
            if let Some(lon) = arr.get(1) {
                pm.longitude = ValueInDegreeOrValueAndUnit::from_unit(unit, lon.to_float());
            }
        }
        pm
    }
}

impl WKTParser for Meridian {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut meridian = Meridian::default();
        if let WKTValue::Array(arr) = val {
            let mut unit = Unit::default();
            if let Some(unit_xml) = arr.get(1) {
                unit = Unit::from_wkt(unit_xml);
            }
            if let Some(lon) = arr.first() {
                meridian.longitude = ValueInDegreeOrValueAndUnit::from_unit(unit, lon.to_float());
            }
        }
        meridian
    }
}

impl WKTParser for Datum {
    fn from_wkt(val: &WKTValue) -> Self {
        // TODO: MAY BE MORE than GeodeticReferenceFrame, but I have yet to interact with one
        Datum::GeodeticReferenceFrame(GeodeticReferenceFrame::from_wkt(val))
    }
}

impl WKTParser for GeodeticReferenceFrame {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut geodetic_reference_frame = GeodeticReferenceFrame::default();
        if let WKTValue::Array(arr) = val {
            if let Some(name) = arr.first() {
                geodetic_reference_frame.name = name.to_string();
            }
            handle_common_fields(&mut geodetic_reference_frame, arr, 1);
        }
        geodetic_reference_frame
    }
}

/// Helper function to handle insert common fields into various ProjJSON objects
fn handle_common_fields<T: ToProjJSON>(res: &mut T, arr: &[WKTValue], start_index: usize) {
    let mut i = start_index;
    while i < arr.len() {
        if let Some(WKTValue::String(item_keyword)) = arr.get(i) {
            let key = item_keyword.as_str();
            match key {
                "ID" | "AUTHORITY" => {
                    res.set_id(Id::from_wkt(&arr[i + 1]));
                    i += 1;
                }
                "UNIT" | "LENGTHUNIT" | "ANGLEUNIT" | "SCALEUNIT" | "TIMEUNIT"
                | "PARAMETRICUNIT" => {
                    let mut unit = Unit::from_wkt(&arr[i + 1]);
                    unit.set_unit_type(key.into());
                    res.set_unit(unit);
                    i += 1;
                }
                "CS" => {
                    res.set_coordinate_system(CoordinateSystem::from_wkt(&arr[i + 1]));
                    i += 1;
                }
                "AXIS" => {
                    res.set_axis(Axis::from_wkt(&arr[i + 1]));
                    i += 1;
                }
                "MEMBERS" => {
                    res.set_member(DatumEnsembleMember::from_wkt(&arr[i + 1]));
                    i += 1;
                }
                "ELLIPSOID" | "SPHEROID" => {
                    res.set_ellipsoid(Ellipsoid::from_wkt(&arr[i + 1]));
                    i += 1;
                }
                "ENSEMBLEACCURACY" | "OPERATIONACCURACY" => {
                    res.set_accuracy(arr[i + 1].to_string());
                    i += 1;
                }
                "EPOCH" | "ANCHOREPOCH" | "COORDEPOCH" => {
                    res.set_epoch(arr[i + 1].to_float());
                }
                "FRAMEEPOCH" => {
                    res.set_frame_epoch(arr[i + 1].to_float());
                }
                "ENSEMBLE" => {
                    res.set_ensemble(DatumEnsemble::from_wkt(&arr[i + 1]));
                    i += 1;
                }
                "DATUM" | "GEODETICDATUM" | "TRF" => {
                    res.set_datum(Datum::from_wkt(&arr[i + 1]));
                    i += 1;
                }
                "METHOD" => {
                    res.set_method(Method::from_wkt(&arr[i + 1]));
                    i += 1;
                }
                "PARAMETER" | "PARAMETERFILE" => {
                    let mut param = ParameterValue::from_wkt(&arr[i + 1]);
                    param.is_file = key == "PARAMETERFILE";
                    res.set_parameter(param);
                    i += 1;
                }
                "PRIMEM" | "PRIMEMERIDIAN" => {
                    res.set_prime_meridian(PrimeMeridian::from_wkt(&arr[i + 1]));
                    i += 1;
                }
                "MERIDIAN" => {
                    res.set_meridian(Meridian::from_wkt(&arr[i + 1]));
                    i += 1;
                }
                // TODO: ANCHOR, EXTENT ([Time|Temporal|Vertical]Extent, bbox, area)
                _ => {}
            }
        }
        i += 1;
    }
    // TODO: Instead let's use the internal ToProjJSON trait for all structs that have a CoordinateSystem to dive down
    // repeat if needed
    if let Some(cs) = res.get_coordinate_system() {
        handle_common_fields(cs, arr, start_index);
    }
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use std::string::ToString;

    use super::*;
    use crate::{
        proj::{AxisDirection, CoordinateSystemSubtype, ValueAndUnit},
        readers::parse_wkt_object,
    };
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
                ellipsoid.ids,
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
                ellipsoid.ids,
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
            assert!(parameter.ids.len() == 1);
            if let Some(id) = parameter.ids.first() {
                assert_eq!(id.authority, "EPSG".to_string());
                assert_eq!(id.code, "8831".into());
            } else {
                panic!("Expected an ID");
            }
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
            assert!(parameter.ids.len() == 1);
            if let Some(id) = parameter.ids.first() {
                assert_eq!(id.authority, "EPSG".to_string());
                assert_eq!(id.code, "8821".into());
            } else {
                panic!("Expected an ID");
            }
        } else {
            panic!("Expected an array");
        }
    }

    #[test]
    fn test_cs_value_from_wkt() {
        let wkt_str = r#"CS[ellipsoidal,2]"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let cs = CoordinateSystem::from_wkt(&arr[1]);
            assert_eq!(cs.subtype, CoordinateSystemSubtype::Ellipsoidal);
        }
    }

    #[test]
    fn test_axis_value_from_wkt() {
        let wkt_str = r#"AXIS["latitude",north,ORDER[1]],"#;
        let wkt_value = parse_wkt_object(wkt_str);
        if let WKTValue::Array(arr) = wkt_value {
            let axis = Axis::from_wkt(&arr[1]);
            assert_eq!(axis.name, "latitude");
            assert_eq!(axis.direction, AxisDirection::North);
        }
    }
}

// TODO:

// BE SURE to remove _ and capitols in names, so if the input is "false_easting" -> "false easting"
// Annex F of https://docs.ogc.org/is/18-010r7/18-010r7.html#221 has all names, codes and aliases.

// XXX: IMPORTANT! When parsing a proj with CS, AXIS and UNIT come AFTER and are not stored inside the CS (WTF? LMAO)

//? Used by EPSG project v12 [COMPLETE]:
// ID: 214278
// EPSG: 214276
// LENGTHUNIT: 40304
// ELLIPSOID: 12351
// PARAMETER: 40825
// AXIS: 28074
// ANGLEUNIT: 23917
// CS: 14093
// SCALEUNIT: 5952
// TIMEUNIT: 217
// MEMBER: 19846
// METHOD: 8986
// OPERATIONACCURACY: 2953
// ENSEMBLE: 2191
// ENSEMBLEACCURACY: 2191
// PARAMETERFILE: 1427 <----- PARAMETERFILE["Latitude difference file","alaska.las"],
// ANCHOREPOCH: 1155
// FRAMEEPOCH: 1024
// PRIMEM: 150 (PRIMEM | PRIMEMERIDIAN)
// MERIDIAN: 102

//? Used by EPSG project v12 [TODO]:
// DATUM: 10182 -> GeodeticReferenceFrame  ->   (8.2 - Geodetic reference frame (geodetic datum)) -> [ELLIPSOID, LENGTH?, ID?]
// DATUMS (ALL) -> WKT ANCHOR and PRIMEM may exist outside the datum (right after it)
// VDATUM: 1691 (VDATUM  |  VRF  |  VERTICALDATUM)
// EDATUM: 29 (EDATUM | ENGINEERINGDATUM)

// CONVERSION: 5857
// BASEGEOGCRS: 5834
// PROJCRS: 5830
// GEOGCRS: 5768

// SOURCECRS: 3145
// TARGETCRS: 3140
// COORDINATEOPERATION: 2948 <------ THIS DOESNT EXIST IN STANDARD?
// VERSION: 2948 <------ THIS DOESNT EXIST IN STANDARD?

// GEOIDMODEL: 1820
// VERTCRS: 1713
// DYNAMIC: 1024

// GEODCRS: 749
// COMPOUNDCRS: 703
// STEP: 396
// DEFININGTRANSFORMATION: 313
// NADCON: 288 (name of method)

// CONCATENATEDOPERATION: 192
// DERIVINGCONVERSION: 176
// BASEVERTCRS: 172
// ENGCRS: 29
// POINTMOTIONOPERATION: 5
// DERIVEDPROJECTED: 4
// BASEPROJCRS: 4

// Ex.0:
// PROJCRS["WGS 84 / Pseudo-Mercator",
//    BASEGEOGCRS["WGS 84",
//        ENSEMBLE["World Geodetic System 1984 ensemble",
//            MEMBER["World Geodetic System 1984 (Transit)", ID["EPSG",1166]],
//            MEMBER["World Geodetic System 1984 (G730)", ID["EPSG",1152]],
//            MEMBER["World Geodetic System 1984 (G873)", ID["EPSG",1153]],
//            MEMBER["World Geodetic System 1984 (G1150)", ID["EPSG",1154]],
//            MEMBER["World Geodetic System 1984 (G1674)", ID["EPSG",1155]],
//            MEMBER["World Geodetic System 1984 (G1762)", ID["EPSG",1156]],
//            MEMBER["World Geodetic System 1984 (G2139)", ID["EPSG",1309]],
//            MEMBER["World Geodetic System 1984 (G2296)", ID["EPSG",1383]],
//            ELLIPSOID["WGS 84",6378137,298.257223563,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",7030]],
//            ENSEMBLEACCURACY[2],
//            ID["EPSG",6326]],
//       ID["EPSG",4326]],
//    CONVERSION["Popular Visualisation Pseudo-Mercator",
//        METHOD["Popular Visualisation Pseudo Mercator",ID["EPSG",1024]],
//        PARAMETER["Latitude of natural origin",0,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8801]],
//        PARAMETER["Longitude of natural origin",0,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],
//        PARAMETER["False easting",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],
//        PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],
//        ID["EPSG",3856]],
//    CS[Cartesian,2,ID["EPSG",4499]],
//    AXIS["Easting (X)",east],
//    AXIS["Northing (Y)",north],
//    LENGTHUNIT["metre",1,ID["EPSG",9001]],
//    ID["EPSG",3857]]

// EX:
// PROJCS["WGS 84 / Pseudo-Mercator",
//      GEOGCS["WGS 84",
//          DATUM["WGS_1984",
//              SPHEROID["WGS 84",6378137,298.257223563,
//                  AUTHORITY["EPSG","7030"]],
//              AUTHORITY["EPSG","6326"]],
//          PRIMEM["Greenwich",0,
//              AUTHORITY["EPSG","8901"]],
//          UNIT["degree",0.0174532925199433,
//              AUTHORITY["EPSG","9122"]],
//          AUTHORITY["EPSG","4326"]],
//      PROJECTION["Mercator_1SP"],
//      PARAMETER["central_meridian",0],
//      PARAMETER["scale_factor",1],
//      PARAMETER["false_easting",0],
//      PARAMETER["false_northing",0],
//      UNIT["metre",1,
//          AUTHORITY["EPSG","9001"]],
//      AXIS["Easting",EAST],
//      AXIS["Northing",NORTH],
//      EXTENSION["PROJ4","+proj=merc +a=6378137 +b=6378137 +lat_ts=0 +lon_0=0 +x_0=0 +y_0=0 +k=1 +units=m +nadgrids=@null +wktext +no_defs"],
//      AUTHORITY["EPSG","3857"]]

// EX 2:
// PROJCRS[
//  "Xian 1980 / 3-degree Gauss-Kruger zone 30",
//  BASEGEOGCRS[
//      "Xian 1980",
//      DATUM[
//          "Xian 1980",
//          ELLIPSOID[
//              "IAG 1975",6378140,298.257,LENGTHUNIT["metre",1,ID["EPSG",9001]],
//              ID["EPSG",7049]],
//          ID["EPSG",6610]],
//      ID["EPSG",4610]],
//  CONVERSION["3-degree Gauss-Kruger zone 30",
//      METHOD["Transverse Mercator",ID["EPSG",9807]],
//      PARAMETER["Latitude of natural origin",0,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],
//          ID["EPSG",8801]],
//      PARAMETER["Longitude of natural origin",90,ANGLEUNIT["degree",0.0174532925199433,ID["EPSG",9102]],ID["EPSG",8802]],
//      PARAMETER["Scale factor at natural origin",1,SCALEUNIT["unity",1,ID["EPSG",9201]],ID["EPSG",8805]],
//      PARAMETER["False easting",30500000,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8806]],
//      PARAMETER["False northing",0,LENGTHUNIT["metre",1,ID["EPSG",9001]],ID["EPSG",8807]],ID["EPSG",16290]],
//      CS[Cartesian,2,ID["EPSG",4530]],
//      AXIS["Northing (X)",north],
//      AXIS["Easting (Y)",east],
//      LENGTHUNIT["metre",1,ID["EPSG",9001]],
//      ID["EPSG",2354]]

// ------------------------------------------------------------------------------------------

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
// - - BoundCRS
// - - CompoundCRS | COMPD_CS
// - - EngineeringCRS | ENGCRS | LOCAL_CS
// - - ParametricCRS
// - - PROJCRS | PROJECTEDCRS | PROJCS
// - - TemporalCRS | TIMECRS
// - - VERTCRS | VERTICALCRS | VERT_CS
// - - GEOGCS | GEOGCRS | GEOGRAPHICCRS | GEODCRS | GEODETICCRS | BASEGEODCRS | BASEGEOGCRS

// used by transformation:
// - SOURCECRS
// - TARGETCRS

// ------------------------------------------------------------------------------------

// EXISTS BUT NOT USEFUL:
// - EXTENSION

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
