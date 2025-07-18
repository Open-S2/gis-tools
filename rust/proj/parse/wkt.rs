use super::{
    Axis, Conversion, CoordinateSystem, Datum, DatumEnsemble, DatumEnsembleMember, Ellipsoid,
    EngineeringDatum, GeodeticCRS, GeodeticReferenceFrame, Id, Meridian, Method, ObjectUsage,
    ParameterValue, ParametricDatum, PrimeMeridian, ProjBBox, ProjJSON, ProjectedCRS,
    TemporalDatum, TemporalExtent, ToProjJSON, Unit, UnitObject, UnitType,
    ValueInDegreeOrValueAndUnit, ValueInMetreOrValueAndUnit, VerticalExtent,
    VerticalReferenceFrame,
};
use crate::{
    parsers::{WKTParser, WKTValue, parse_wkt_object},
    proj::AxisDirection,
};
use alloc::format;

impl WKTParser for ObjectUsage {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut usage = ObjectUsage::default();
        if let WKTValue::Array(arr) = val {
            if !arr.is_empty() && arr[0].to_string() == "SCOPE" {
                if let WKTValue::Array(arr) = &arr[1] {
                    usage.scope = arr.first().map(|s| s.to_string()).unwrap_or_default();
                }
            }
            if arr.len() >= 2 {
                handle_common_fields(&mut usage, arr, 2);
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
            if let Some(name) = arr.first() {
                unit.name = name.to_string();
            }
            if let Some(conversion_factor) = arr.get(1) {
                unit.conversion_factor = Some(conversion_factor.to_float());
            }
            handle_common_fields(&mut unit, arr, 2);
        }
        Unit::UnitObject(unit)
    }
}

impl WKTParser for TemporalExtent {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut te = TemporalExtent::default();
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
            if semi_major_axis != 0.0 {
                ellipsoid.semi_major_axis = Some(semi_major_axis.into());
            }
            if inverse_flattening != 0.0 {
                ellipsoid.inverse_flattening = Some(inverse_flattening.into());
            }
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
                parameter_value.name = arr[0].to_string().to_lowercase().replace(" ", "_");
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
                axis.abbreviation = arr[0].to_string();
                axis.name = axis.abbreviation.clone();
                axis.direction = AxisDirection::from(arr[1].to_string());
                handle_common_fields(&mut axis, arr, 2);
            }
            // NOTE: BEARING, ORDER, and ANGLEUNIT exist, but add no value in modern WKT
        }
        axis.adjust_if_needed();
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
            handle_common_fields(&mut datum_ensemble_member, arr, 1);
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
            handle_common_fields(&mut method, arr, 1);
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
            if let Some(lon) = arr.get(1) {
                pm.longitude =
                    ValueInDegreeOrValueAndUnit::from_unit(Unit::new_deg(), lon.to_float());
            }
            handle_common_fields(&mut pm, arr, 2);
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
            handle_common_fields(&mut meridian, arr, 2);
        }
        meridian
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

impl WKTParser for VerticalReferenceFrame {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut vertical_reference_frame = VerticalReferenceFrame::default();
        if let WKTValue::Array(arr) = val {
            if let Some(name) = arr.first() {
                vertical_reference_frame.name = name.to_string();
            }
            handle_common_fields(&mut vertical_reference_frame, arr, 1);
        }
        vertical_reference_frame
    }
}

impl WKTParser for TemporalDatum {
    fn from_wkt(val: &WKTValue) -> Self {
        // TODO: CALENDAR and TIME_ORIGIN exist, but add no value in modern WKT.
        // Should be handled as "common fields" though.
        let mut temporal_datum = TemporalDatum::default();
        if let WKTValue::Array(arr) = val {
            if let Some(name) = arr.first() {
                temporal_datum.name = name.to_string();
            }
            handle_common_fields(&mut temporal_datum, arr, 1);
        }
        temporal_datum
    }
}

impl WKTParser for EngineeringDatum {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut engineering_datum = EngineeringDatum::default();
        if let WKTValue::Array(arr) = val {
            if let Some(name) = arr.first() {
                engineering_datum.name = name.to_string();
            }
            handle_common_fields(&mut engineering_datum, arr, 1);
        }
        engineering_datum
    }
}

impl WKTParser for ParametricDatum {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut parametric_datum = ParametricDatum::default();
        if let WKTValue::Array(arr) = val {
            if let Some(name) = arr.first() {
                parametric_datum.name = name.to_string();
            }
            handle_common_fields(&mut parametric_datum, arr, 1);
        }
        parametric_datum
    }
}

impl WKTParser for Conversion {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut conversion = Conversion::default();
        if let WKTValue::Array(arr) = val {
            if let Some(name) = arr.first() {
                conversion.name = name.to_string();
            }
            handle_common_fields(&mut conversion, arr, 1);
        }
        conversion
    }
}

impl WKTParser for GeodeticCRS {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut geodetic_crs = GeodeticCRS::default();
        if let WKTValue::Array(arr) = val {
            if let Some(name) = arr.first() {
                geodetic_crs.name = name.to_string();
            }
            handle_common_fields(&mut geodetic_crs, arr, 1);
        }
        geodetic_crs
    }
}

impl WKTParser for ProjectedCRS {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut projected_crs = ProjectedCRS::default();
        if let WKTValue::Array(arr) = val {
            if let Some(name) = arr.first() {
                projected_crs.name = name.to_string();
            }
            handle_common_fields(&mut projected_crs, arr, 1);
        }
        projected_crs
    }
}

const TOP_LEVEL_PROJ_KEYWORDS: [&str; 27] = [
    "BoundCRS",
    "PROJCRS",
    "PROJCS",
    "BASEPROJCRS",
    "PROJECTEDCRS",
    "COORDINATEOPERATION",
    "VERTCRS",
    "VERTICALCRS",
    "VERT_CS",
    "GEOGCS",
    "GEOGCRS",
    "GEODCRS",
    "BASEGEODCRS",
    "BASEGEOGCRS",
    "GEODETICCRS",
    "GEOGRAPHICCRS",
    "COMPOUNDCRS",
    "COMPD_CS",
    "CONCATENATEDOPERATION",
    "DERIVEDPROJECTED",
    "EngineeringCRS",
    "ENGCRS",
    "LOCAL_CS",
    "ParametricCRS",
    "POINTMOTIONOPERATION",
    "TemporalCRS",
    "TIMECRS",
];

impl ProjJSON {
    /// Convert a string to WKT object and then to ProjJSON object
    pub fn parse_wkt(val: &str) -> Self {
        let wkt = parse_wkt_object(val);
        Self::from_wkt(&wkt)
    }
}
impl WKTParser for ProjJSON {
    fn from_wkt(val: &WKTValue) -> Self {
        let mut proj_json = ProjJSON::default();
        if let WKTValue::Array(arr) = val {
            // ensure the name at the beginning is one of the top level types
            if let Some(name) = arr.first() {
                let name = name.to_string();
                if !TOP_LEVEL_PROJ_KEYWORDS.contains(&name.as_str()) {
                    panic!("Expected one of {}", TOP_LEVEL_PROJ_KEYWORDS.join(", "));
                }
            }
            handle_common_fields(&mut proj_json, arr, 0);
        }
        proj_json
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
                    let WKTValue::Array(arr) = &arr[i + 1] else {
                        continue;
                    };
                    let epoch = arr.get(0).map(|s| s.to_float()).unwrap_or_default();
                    res.set_epoch(epoch);
                    i += 1;
                }
                "FRAMEEPOCH" => {
                    res.set_frame_epoch(arr[i + 1].to_float());
                    i += 1;
                }
                "ENSEMBLE" => {
                    res.set_ensemble(DatumEnsemble::from_wkt(&arr[i + 1]));
                    i += 1;
                }
                "DATUM" | "GEODETICDATUM" | "TRF" => {
                    res.set_datum(Datum::GeodeticReferenceFrame(GeodeticReferenceFrame::from_wkt(
                        &arr[i + 1],
                    )));
                    i += 1;
                }
                "VDATUM" | "VRF" | "VERTICALDATUM" => {
                    res.set_datum(Datum::VerticalReferenceFrame(VerticalReferenceFrame::from_wkt(
                        &arr[i + 1],
                    )));
                    i += 1;
                }
                "TDATUM" | "TIMEDATUM" => {
                    res.set_datum(Datum::TemporalDatum(TemporalDatum::from_wkt(&arr[i + 1])));
                    i += 1;
                }
                "EDATUM" | "ENGINEERINGDATUM" => {
                    res.set_datum(Datum::EngineeringDatum(EngineeringDatum::from_wkt(&arr[i + 1])));
                    i += 1;
                }
                "PDATUM" | "PARAMETRICDATUM" => {
                    res.set_datum(Datum::ParametricDatum(ParametricDatum::from_wkt(&arr[i + 1])));
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
                "TIMEEXTENT" => {
                    res.set_temporal_extent(TemporalExtent::from_wkt(&arr[i + 1]));
                    i += 1;
                }
                "VERTICALEXTENT" => {
                    res.set_vertical_extent(VerticalExtent::from_wkt(&arr[i + 1]));
                    i += 1;
                }
                "BBOX" => {
                    res.set_bbox(ProjBBox::from_wkt(&arr[i + 1]));
                    i += 1;
                }
                "AREA" => {
                    if let WKTValue::Array(arr) = &arr[i + 1] {
                        res.set_area(arr.first().map(|s| s.to_string()));
                        i += 1;
                    }
                }
                "DERIVINGCONVERSION" | "CONVERSION" => {
                    res.set_conversion(Conversion::from_wkt(&arr[i + 1]));
                    i += 1;
                }
                "GEOGCS" | "GEOGCRS" | "GEOGRAPHICCRS" | "GEODCRS" | "GEODETICCRS"
                | "BASEGEODCRS" | "BASEGEOGCRS" => {
                    res.set_geodetic_crs(GeodeticCRS::from_wkt(&arr[i + 1]));
                    i += 1;
                }
                "PROJCRS" | "PROJECTEDCRS" | "PROJCS" | "BASEPROJCRS" => {
                    res.set_projected_crs(ProjectedCRS::from_wkt(&arr[i + 1]));
                    i += 1;
                }
                "ANCHOR" => {
                    let WKTValue::Array(arr) = &arr[i + 1] else {
                        continue;
                    };
                    let anchor = arr.get(0).map(|s| s.to_string()).unwrap_or_default();
                    res.set_anchor(anchor);
                    i += 1;
                }
                "USAGE" => {
                    res.set_usage(ObjectUsage::from_wkt(&arr[i + 1]));
                    i += 1;
                }
                "PROJECTION" => {
                    let WKTValue::Array(arr) = &arr[i + 1] else {
                        continue;
                    };
                    let proj = arr.get(0).map(|s| s.to_string()).unwrap_or_default();
                    res.set_projection(proj);
                    i += 1;
                }
                "ORDER" => {
                    let WKTValue::Array(arr) = &arr[i + 1] else {
                        continue;
                    };
                    let order = arr.get(0).map(|s| s.to_float() as usize).unwrap_or_default();
                    res.set_order(order);
                    i += 1;
                }
                // TODO: MODEL -> DYNAMIC[FRAMEEPOCH[2010.0],MODEL["NAD83(CSRS)v6 velocity grid"]] -> Stored as DeformationModel
                _ => {}
            }
        }
        i += 1;
    }
}

// We removed _ and capitols in names, so if the input is "false_easting" -> "false easting"
// Annex F of https://docs.ogc.org/is/18-010r7/18-010r7.html#221 has all names, codes and aliases.

//? Used by EPSG project v12 [COMPLETE]:
// ID: 214278
// EPSG: 214276
// LENGTHUNIT: 40304
// ELLIPSOID: 12351
// PARAMETER: 40825
// AXIS: 28074
// ANGLEUNIT: 23917
// MEMBER: 19846
// CS: 14093
// DATUM: 10182
// METHOD: 8986
// SCALEUNIT: 5952
// CONVERSION: 5857
// BASEGEOGCRS: 5834
// PROJCRS: 5830
// GEOGCRS: 5768
// OPERATIONACCURACY: 2953
// ENSEMBLE: 2191
// ENSEMBLEACCURACY: 2191
// VDATUM: 1691
// PARAMETERFILE: 1427 <----- PARAMETERFILE["Latitude difference file","alaska.las"],
// ANCHOREPOCH: 1155
// FRAMEEPOCH: 1024
// GEODCRS: 749
// TIMEUNIT: 217
// PRIMEM: 150 (PRIMEM | PRIMEMERIDIAN)
// MERIDIAN: 102
// EDATUM: 29
// BASEPROJCRS: 4

//? Used by EPSG project v12 [TODO]:

// NON OPERATIONS 1::
// GEOIDMODEL: 1820
// VERTCRS: 1713
// DYNAMIC: 1024 <------ DYNAMIC[FRAMEEPOCH[2010.0],MODEL["NAD83(CSRS)v6 velocity grid"]]
// COMPOUNDCRS: 703

// NON OPERATIONS 2::
// DEFININGTRANSFORMATION: 313
// DERIVINGCONVERSION: 176
// BASEVERTCRS: 172
// ENGCRS: 29
// DERIVEDPROJECTED: 4

// OPERATIONS:
// SOURCECRS: 3145
// TARGETCRS: 3140
// COORDINATEOPERATION: 2948 <------ THIS DOESNT EXIST IN STANDARD?
// VERSION: 2948 <------ THIS DOESNT EXIST IN STANDARD?
// STEP: 396
// CONCATENATEDOPERATION: 192
// POINTMOTIONOPERATION: 5

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
// ✅ PARAMETER
// ✅ PROJECTION
// ✅ DATUM
// ✅ - VDATUM  |  VRF  |  VERTICALDATUM
// ✅ - TDATUM | TIMEDATUM
// ✅ - EDATUM | ENGINEERINGDATUM
// ✅ - PDATUM | PARAMETRICDATUM
// ✅ - DATUM | GEODETICDATUM | TRF
// ✅ PRIMEM | PRIMEMERIDIAN | MERIDIAN
// ✅ UNIT
// ✅ AXIS (and AxisDirection)
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
// ✅ - PROJCRS | PROJECTEDCRS | PROJCS
// - - TemporalCRS | TIMECRS
// - - VERTCRS | VERTICALCRS | VERT_CS
// ✅ - GEOGCS | GEOGCRS | GEOGRAPHICCRS | GEODCRS | GEODETICCRS | BASEGEODCRS | BASEGEOGCRS

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
