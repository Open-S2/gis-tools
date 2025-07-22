#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::proj::{
        AbridgedTransformation, Axis, AxisDirection, BaseUnit, BoundCRS, CRS, CompoundCRS,
        ConcatenatedOperation, Conversion, CoordinateMetadata, CoordinateSystem, Datum,
        DatumEnsemble, DatumEnsembleMember, DerivedEngineeringCRS, DerivedGeodeticCRS,
        DerivedParametricCRS, DerivedProjectedCRS, DerivedTemporalCRS, DerivedVerticalCRS,
        DynamicGeodeticReferenceFrame, DynamicVerticalReferenceFrame, Ellipsoid, EngineeringCRS,
        EngineeringDatum, GeodeticCRS, GeodeticReferenceFrame, Id, Meridian, Method, ObjectUsage,
        ParameterValue, ParametricCRS, ParametricDatum, PointMotionOperation, PrimeMeridian,
        ProjBBox, ProjJSON, ProjValue, ProjectedCRS, ProjectionTransform, SingleOperation,
        TemporalCRS, TemporalDatum, TemporalExtent, ToProjJSON, Transformation, Unit, UnitObject,
        UnitType, ValueInDegreeOrValueAndUnit, ValueInMetreOrValueAndUnit, VerticalCRS,
        VerticalExtent, VerticalReferenceFrame,
    };

    // https://proj.org/en/stable/specifications/projjson.html#schema <- more examples to play with

    #[test]
    fn test_axis_direction_from_string() {
        assert_eq!(AxisDirection::from("north".to_string()), AxisDirection::North);
        assert_eq!(
            AxisDirection::from("northNorthEast".to_string()),
            AxisDirection::NorthNorthEast
        );
        assert_eq!(AxisDirection::from("east".to_string()), AxisDirection::East);
        assert_eq!(
            AxisDirection::from("southSouthWest".to_string()),
            AxisDirection::SouthSouthWest
        );
        assert_eq!(AxisDirection::from("up".to_string()), AxisDirection::Up);
        assert_eq!(AxisDirection::from("East ".to_string()), AxisDirection::East);
        assert_eq!(
            AxisDirection::from(" South South West ".to_string()),
            AxisDirection::SouthSouthWest
        );
        assert_eq!(AxisDirection::from(" Up ".to_string()), AxisDirection::Up);
        assert_eq!(AxisDirection::from("Geocentric_X".to_string()), AxisDirection::GeocentricX);
        assert_eq!(
            AxisDirection::from("Column_Positive".to_string()),
            AxisDirection::ColumnPositive
        );
        assert_eq!(AxisDirection::from("Row_Negative".to_string()), AxisDirection::RowNegative);
        assert_eq!(AxisDirection::from("Display_Right".to_string()), AxisDirection::DisplayRight);
        assert_eq!(AxisDirection::from("North".to_string()), AxisDirection::North);
        assert_eq!(
            AxisDirection::from("North-North-East".to_string()),
            AxisDirection::NorthNorthEast
        );
        assert_eq!(AxisDirection::from("East".to_string()), AxisDirection::East);
        assert_eq!(
            AxisDirection::from("South-South-West".to_string()),
            AxisDirection::SouthSouthWest
        );
        assert_eq!(AxisDirection::from("Up".to_string()), AxisDirection::Up);
        assert_eq!(
            AxisDirection::from("North_North East".to_string()),
            AxisDirection::NorthNorthEast
        );
        assert_eq!(
            AxisDirection::from("East-North East".to_string()),
            AxisDirection::EastNorthEast
        );
        assert_eq!(
            AxisDirection::from("South South_West".to_string()),
            AxisDirection::SouthSouthWest
        );

        assert_eq!(AxisDirection::from("Unknown".to_string()), AxisDirection::Unspecified);
        assert_eq!(AxisDirection::from("".to_string()), AxisDirection::Unspecified);
        assert_eq!(AxisDirection::from(" ".to_string()), AxisDirection::Unspecified);
        assert_eq!(AxisDirection::from("__--".to_string()), AxisDirection::Unspecified);

        assert_eq!(AxisDirection::from("north".to_string()), AxisDirection::North);
        assert_eq!(
            AxisDirection::from("northNorthEast".to_string()),
            AxisDirection::NorthNorthEast
        );
        assert_eq!(AxisDirection::from("geocentricY".to_string()), AxisDirection::GeocentricY);
    }

    #[test]
    fn id() {
        let json = r#"{
            "authority": "EPSG",
            "code": 8251
        }"#;

        let id: Id = serde_json::from_str(json).unwrap();

        assert_eq!(id.authority, "EPSG");
        assert_eq!(id.code.i64(), 8251);
    }

    #[test]
    fn it_works() {
        let json = r#"{
                "type": "GeographicCRS",
                "name": "NAD83(CSRS)v6",
                "datum": {
                    "type": "GeodeticReferenceFrame",
                    "name": "North American Datum of 1983 (CSRS) version 6",
                    "ellipsoid": {
                    "name": "GRS 1980",
                    "semi_major_axis": 6378137,
                    "inverse_flattening": 298.257222101
                    }
                },
                "coordinate_system": {
                    "type": "CoordinateSystem",
                    "name": "Geodetic",
                    "subtype": "ellipsoidal",
                    "axis": [
                        {
                            "name": "Geodetic latitude",
                            "abbreviation": "Lat",
                            "direction": "north",
                            "unit": "degree"
                        },
                        {
                            "name": "Geodetic longitude",
                            "abbreviation": "Lon",
                            "direction": "east",
                            "unit": "degree"
                        },
                        {
                            "name": "Ellipsoidal height",
                            "abbreviation": "h",
                            "direction": "up",
                            "unit": "metre"
                        }
                    ]
                },
                "scope": "Geodesy.",
                "area": "Canada - onshore and offshore - Alberta; British Columbia; Manitoba; New Brunswick; Newfoundland and Labrador; Northwest Territories; Nova Scotia; Nunavut; Ontario; Prince Edward Island; Quebec; Saskatchewan; Yukon.",
                "bbox": {
                    "south_latitude": 38.21,
                    "west_longitude": -141.01,
                    "north_latitude": 86.46,
                    "east_longitude": -40.73
                },
                "id": {
                    "authority": "EPSG",
                    "code": 8251
                }
            }"#;

        let proj_json: GeodeticCRS = serde_json::from_str(json).unwrap();
        assert_eq!(proj_json.r#type, Some("GeographicCRS".into()));
    }

    #[test]
    fn proj_crs() {
        let json = r#"{
            "$schema": "https://proj.org/schemas/v0.7/projjson.schema.json",
            "type": "ProjectedCRS",
            "name": "WGS 84 / Pseudo-Mercator",
            "base_crs": {
                "name": "WGS 84",
                "datum_ensemble": {
                    "name": "World Geodetic System 1984 ensemble",
                    "members": [
                        {
                            "name": "World Geodetic System 1984 (Transit)",
                            "id": {
                                "authority": "EPSG",
                                "code": 1166
                            }
                        },
                        {
                            "name": "World Geodetic System 1984 (G730)",
                            "id": {
                                "authority": "EPSG",
                                "code": 1152
                            }
                        },
                        {
                            "name": "World Geodetic System 1984 (G873)",
                            "id": {
                                "authority": "EPSG",
                                "code": 1153
                            }
                        },
                        {
                            "name": "World Geodetic System 1984 (G1150)",
                            "id": {
                                "authority": "EPSG",
                                "code": 1154
                            }
                        },
                        {
                            "name": "World Geodetic System 1984 (G1674)",
                            "id": {
                                "authority": "EPSG",
                                "code": 1155
                            }
                        },
                        {
                            "name": "World Geodetic System 1984 (G1762)",
                            "id": {
                                "authority": "EPSG",
                                "code": 1156
                            }
                        },
                        {
                            "name": "World Geodetic System 1984 (G2139)",
                            "id": {
                                "authority": "EPSG",
                                "code": 1309
                            }
                        },
                        {
                            "name": "World Geodetic System 1984 (G2296)",
                            "id": {
                                "authority": "EPSG",
                                "code": 1383
                            }
                        }
                    ],
                    "ellipsoid": {
                        "name": "WGS 84",
                        "semi_major_axis": 6378137,
                        "inverse_flattening": 298.257223563
                    },
                    "accuracy": "2.0",
                    "id": {
                        "authority": "EPSG",
                        "code": 6326
                    }
                },
                "coordinate_system": {
                    "subtype": "ellipsoidal",
                    "axis": [
                        {
                            "name": "Geodetic latitude",
                            "abbreviation": "Lat",
                            "direction": "north",
                            "unit": "degree"
                        },
                        {
                            "name": "Geodetic longitude",
                            "abbreviation": "Lon",
                            "direction": "east",
                            "unit": "degree"
                        }
                    ]
                },
                "id": {
                    "authority": "EPSG",
                    "code": 4326
                }
            },
            "conversion": {
                "name": "Popular Visualisation Pseudo-Mercator",
                "method": {
                    "name": "Popular Visualisation Pseudo Mercator",
                    "id": {
                        "authority": "EPSG",
                        "code": 1024
                    }
                },
                "parameters": [
                    {
                        "name": "Latitude of natural origin",
                        "value": 0,
                        "unit": "degree",
                        "id": {
                            "authority": "EPSG",
                            "code": 8801
                        }
                    },
                    {
                        "name": "Longitude of natural origin",
                        "value": 0,
                        "unit": "degree",
                        "id": {
                            "authority": "EPSG",
                            "code": 8802
                        }
                    },
                    {
                        "name": "False easting",
                        "value": 0,
                        "unit": "metre",
                        "id": {
                            "authority": "EPSG",
                            "code": 8806
                        }
                    },
                    {
                        "name": "False northing",
                        "value": 0,
                        "unit": "metre",
                        "id": {
                            "authority": "EPSG",
                            "code": 8807
                        }
                    }
                ]
            },
            "coordinate_system": {
                "subtype": "Cartesian",
                "axis": [
                    {
                        "name": "Easting",
                        "abbreviation": "X",
                        "direction": "east",
                        "unit": "metre"
                    },
                    {
                        "name": "Northing",
                        "abbreviation": "Y",
                        "direction": "north",
                        "unit": "metre"
                    }
                ]
            },
            "scope": "Web mapping and visualisation.",
            "area": "World between 85.06°S and 85.06°N.",
            "bbox": {
                "south_latitude": -85.06,
                "west_longitude": -180,
                "north_latitude": 85.06,
                "east_longitude": 180
            },
            "id": {
                "authority": "EPSG",
                "code": 3857
            }
        }"#;

        let proj: ProjectedCRS = serde_json::from_str(json).unwrap();
        assert_eq!(proj.usage.unwrap().id.unwrap().code.i64(), 3857);

        let full: ProjJSON = serde_json::from_str(json).unwrap();
        if let ProjJSON::CRS(crs) = full {
            if let CRS::ProjectedCRS(proj) = *crs {
                assert_eq!(proj.usage.unwrap().id.unwrap().code.i64(), 3857);
            }
        } else {
            panic!("Expected ProjectedCRS");
        }
    }

    #[test]
    fn test_to_proj_json_dead_ends() {
        let mut test = Datum::default();
        test.set_usage(ObjectUsage::default());
        test.set_anchor("test".into());
        test.set_unit(Unit::default());
        test.set_axis(Axis::default());
        test.set_coordinate_system(CoordinateSystem::default());
        test.set_temporal_extent(TemporalExtent::default());
        test.set_vertical_extent(VerticalExtent::default());
        test.set_bbox(ProjBBox::default());
        test.set_area(Some("test".to_string()));
        test.set_method(Method::default());
        test.set_ensemble(DatumEnsemble::default());
        test.set_member(DatumEnsembleMember::default());
        test.set_ellipsoid(Ellipsoid::default());
        test.set_accuracy("test".into());
        test.set_epoch(1.1);
        test.set_frame_epoch(1.1);
        test.set_datum(Datum::default());
        test.set_parameter(ParameterValue::default());
        test.set_meridian(Meridian::default());
        test.set_conversion(Conversion::default());
        test.set_geodetic_crs(GeodeticCRS::default());
        test.set_projected_crs(ProjectedCRS::default());
        test.set_projection("test".into());
        test.set_order(1);
        assert_eq!(test, Datum::default());

        let mut test = Unit::default();
        test.set_id(Id::default());
        test.set_ensemble(DatumEnsemble::default());
        test.set_prime_meridian(PrimeMeridian::default());
        assert_eq!(test, Unit::default());
    }

    #[test]
    fn test_proj_json_no_bugs() {
        let mut test = ProjJSON::default();
        test.set_id(Id::default());
        test.set_datum(Datum::default());
        test.set_ensemble(DatumEnsemble::default());
        test.set_prime_meridian(PrimeMeridian::default());
    }

    #[test]
    fn test_crs_no_bugs() {
        // set ids
        let mut test = CRS::BoundCRS(Box::new(BoundCRS::default()));
        test.set_id(Id::default());
        let mut test = CRS::CompoundCRS(Box::new(CompoundCRS::default()));
        test.set_id(Id::default());
        let mut test = CRS::DerivedEngineeringCRS(Box::new(DerivedEngineeringCRS::default()));
        test.set_id(Id::default());
        let mut test = CRS::DerivedGeodeticCRS(Box::new(DerivedGeodeticCRS::default()));
        test.set_id(Id::default());
        let mut test = CRS::DerivedParametricCRS(Box::new(DerivedParametricCRS::default()));
        test.set_id(Id::default());
        let mut test = CRS::DerivedProjectedCRS(Box::new(DerivedProjectedCRS::default()));
        test.set_id(Id::default());
        let mut test = CRS::DerivedTemporalCRS(Box::new(DerivedTemporalCRS::default()));
        test.set_id(Id::default());
        let mut test = CRS::DerivedVerticalCRS(Box::new(DerivedVerticalCRS::default()));
        test.set_id(Id::default());
        let mut test = CRS::EngineeringCRS(Box::new(EngineeringCRS::default()));
        test.set_id(Id::default());
        let mut test = CRS::GeodeticCRS(Box::new(GeodeticCRS::default()));
        test.set_id(Id::default());
        let mut test = CRS::ParametricCRS(Box::new(ParametricCRS::default()));
        test.set_id(Id::default());
        let mut test = CRS::ProjectedCRS(Box::new(ProjectedCRS::default()));
        test.set_id(Id::default());
        let mut test = CRS::TemporalCRS(Box::new(TemporalCRS::default()));
        test.set_id(Id::default());
        let mut test = CRS::VerticalCRS(Box::new(VerticalCRS::default()));
        test.set_id(Id::default());

        let mut test = CRS::default();
        test.set_geodetic_crs(GeodeticCRS::default());
        test.set_projected_crs(ProjectedCRS::default());
    }

    #[test]
    fn test_datum_no_bugs() {
        let mut test = Datum::default();
        test.set_prime_meridian(PrimeMeridian::default());
        let mut test =
            Datum::DynamicGeodeticReferenceFrame(DynamicGeodeticReferenceFrame::default());
        test.set_prime_meridian(PrimeMeridian::default());

        // ids
        let mut test = Datum::GeodeticReferenceFrame(GeodeticReferenceFrame::default());
        test.set_id(Id::default());
        let mut test = Datum::VerticalReferenceFrame(VerticalReferenceFrame::default());
        test.set_id(Id::default());
        let mut test =
            Datum::DynamicGeodeticReferenceFrame(DynamicGeodeticReferenceFrame::default());
        test.set_id(Id::default());
        let mut test =
            Datum::DynamicVerticalReferenceFrame(DynamicVerticalReferenceFrame::default());
        test.set_id(Id::default());
        let mut test = Datum::TemporalDatum(TemporalDatum::default());
        test.set_id(Id::default());
        let mut test = Datum::ParametricDatum(ParametricDatum::default());
        test.set_id(Id::default());
        let mut test = Datum::EngineeringDatum(EngineeringDatum::default());
        test.set_id(Id::default());
    }

    #[test]
    fn test_proj_value() {
        // compares
        let a = ProjValue::Bool(true);
        let b = ProjValue::Bool(true);
        assert!(a == b);
        let a = ProjValue::F64(0.1);
        let b = ProjValue::F64(0.1);
        assert!(a == b);
        let a = ProjValue::I64(1);
        let b = ProjValue::I64(1);
        assert!(a == b);
        let a = ProjValue::I64(1);
        let b = ProjValue::Bool(false);
        assert!(a != b);

        // bools
        let test = ProjValue::Bool(true);
        assert!(test.bool());
        let test = ProjValue::F64(0.1);
        assert!(test.bool());
        let test = ProjValue::I64(1);
        assert!(test.bool());
        let test = ProjValue::String("true".to_string());
        assert!(test.bool());
        let test = ProjValue::String("1".to_string());
        assert!(test.bool());

        // f64
        let test = ProjValue::Bool(true);
        assert_eq!(test.f64(), 1.0);
        let test = ProjValue::Bool(false);
        assert_eq!(test.f64(), 0.0);
        let test = ProjValue::F64(0.1);
        assert_eq!(test.f64(), 0.1);
        let test = ProjValue::I64(1);
        assert_eq!(test.f64(), 1.0);
        let test = ProjValue::String("true".to_string());
        assert_eq!(test.f64(), 0.0);
        let test = ProjValue::String("1".to_string());
        assert_eq!(test.f64(), 1.0);

        // i64
        let test = ProjValue::Bool(true);
        assert_eq!(test.i64(), 1);
        let test = ProjValue::Bool(false);
        assert_eq!(test.i64(), 0);
        let test = ProjValue::F64(0.1);
        assert_eq!(test.i64(), 0);
        let test = ProjValue::I64(1);
        assert_eq!(test.i64(), 1);
        let test = ProjValue::String("true".to_string());
        assert_eq!(test.i64(), 0);
        let test = ProjValue::String("1".to_string());
        assert_eq!(test.i64(), 1);

        // string
        let test = ProjValue::Bool(true);
        assert_eq!(test.string(), "true".to_string());
        let test = ProjValue::Bool(false);
        assert_eq!(test.string(), "false".to_string());
        let test = ProjValue::F64(0.1);
        assert_eq!(test.string(), "0.1".to_string());
        let test = ProjValue::I64(1);
        assert_eq!(test.string(), "1".to_string());
        let test = ProjValue::String("true".to_string());
        assert_eq!(test.string(), "true".to_string());
        let test = ProjValue::String("1".to_string());
        assert_eq!(test.string(), "1".to_string());

        // converts
        let test = ProjValue::String("test".into());
        let test_str: String = test.into();
        assert_eq!(test_str, "test".to_string());

        let test: f64 = 0.1;
        let test_proj: ProjValue = test.into();
        assert_eq!(test_proj, ProjValue::F64(0.1));
    }

    #[test]
    fn test_parametric_crs() {
        let mut test = ParametricCRS::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_accuracy("test".into());
        test.set_projection("test".into());
        test.set_method(Method::default());
        test.set_parameter(ParameterValue::default());

        test.set_axis(Axis::default());
    }

    #[test]
    fn test_parametric_datum() {
        let mut test = ParametricDatum::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_accuracy("test".into());
        test.set_projection("test".into());
        test.set_method(Method::default());
        test.set_parameter(ParameterValue::default());
        test.set_anchor("test".into());
    }

    #[test]
    fn test_point_motion_operation() {
        let mut test = PointMotionOperation::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_accuracy("test".into());
        test.set_projection("test".into());
        test.set_method(Method::default());
        test.set_parameter(ParameterValue::default());
    }

    #[test]
    fn test_param_values() {
        let mut test = ParameterValue::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.ids.len(), 3);
    }

    #[test]
    fn test_method() {
        let mut test = Method::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.ids.len(), 3);
    }

    #[test]
    fn test_unit_object() {
        let mut test = UnitObject::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.ids.len(), 3);

        test.set_unit_type(UnitType::AngularUnit);
        assert_eq!(test.meters(), 1.);
    }

    #[test]
    fn test_unit() {
        let mut test = Unit::default();
        test.set_unit_type(UnitType::AngularUnit);

        let test = Unit::BaseUnit(BaseUnit::Metre);
        assert_eq!(test.meters(), 1.);
        let test = Unit::BaseUnit(BaseUnit::Degree);
        assert_eq!(test.meters(), 0.017453292519943295);
        let test = Unit::BaseUnit(BaseUnit::Unity);
        assert_eq!(test.meters(), 1.);
    }

    #[test]
    fn test_bound_crs() {
        let mut test = BoundCRS::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
    }

    #[test]
    fn test_concatenated_operation() {
        let mut test = ConcatenatedOperation::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_accuracy("test".into());
    }

    #[test]
    fn test_abridged_transformation() {
        let mut test = AbridgedTransformation::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.ids.len(), 3);

        test.set_method(Method::default());
        test.set_parameter(ParameterValue::default());
        test.set_projection("test".into());
    }

    #[test]
    fn test_compound_crs() {
        let mut test = CompoundCRS::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_usage(ObjectUsage::default());
        test.set_axis(Axis::default());
    }

    #[test]
    fn test_engineering_crs() {
        let mut test = EngineeringCRS::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_unit(Unit::default());
        test.set_axis(Axis::default());
        test.set_coordinate_system(CoordinateSystem::default());
    }

    #[test]
    fn test_engineering_datum() {
        let mut test = EngineeringDatum::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_unit(Unit::default());
        test.set_anchor("test".into());
    }

    #[test]
    fn test_axis() {
        let mut test = Axis::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.ids.len(), 3);
        test.set_unit(Unit::default());
        test.set_meridian(Meridian::default());
        test.name = "z".into();
        test.adjust_if_needed();
        assert_eq!(test.order, 3);
    }

    #[test]
    fn test_meridian() {
        let mut test = Meridian::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.ids.len(), 3);
    }

    #[test]
    fn test_value_in_degree_or_value_and_unit() {
        let mut test = ValueInDegreeOrValueAndUnit::default();
        assert_eq!(test, ValueInDegreeOrValueAndUnit::F64(0.));
        assert_eq!(test.rad(), 0.);
        test.set_unit(Unit::default());
    }

    #[test]
    fn test_value_in_meter_or_value_and_unit() {
        let test = ValueInMetreOrValueAndUnit::default();
        assert_eq!(test, ValueInMetreOrValueAndUnit::F64(0.));
    }

    #[test]
    fn test_single_operation() {
        let mut test = SingleOperation::Conversion(Box::new(Conversion::default()));
        test.set_id(Id::default());
        test.set_conversion(Conversion::default());
        let mut test = SingleOperation::Transformation(Box::new(Transformation::default()));
        test.set_id(Id::default());
        let mut test =
            SingleOperation::PointMotionOperation(Box::new(PointMotionOperation::default()));
        test.set_id(Id::default());
    }

    #[test]
    fn test_derived_engineering_crs() {
        let mut test = DerivedEngineeringCRS::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_coordinate_system(CoordinateSystem::default());
        test.set_conversion(Conversion::default());
        test.set_projected_crs(ProjectedCRS::default());
        test.set_axis(Axis::default());
        test.set_unit(Unit::default());
    }

    #[test]
    fn test_derived_geodetic_crs() {
        let mut test = DerivedGeodeticCRS::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_coordinate_system(CoordinateSystem::default());
        test.set_geodetic_crs(GeodeticCRS::default());
        test.set_conversion(Conversion::default());
        test.set_projected_crs(ProjectedCRS::default());
        test.set_axis(Axis::default());
        test.set_unit(Unit::default());
    }

    #[test]
    fn test_geodetic_crs() {
        let mut test = GeodeticCRS::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_coordinate_system(CoordinateSystem::default());
        test.set_conversion(Conversion::default());
        test.set_projected_crs(ProjectedCRS::default());
        test.set_axis(Axis::default());
        test.set_unit(Unit::default());

        let _ = test.to_projection_transform(&mut ProjectionTransform::default());
    }

    #[test]
    fn test_geodetic_reference_frame() {
        let mut test = GeodeticReferenceFrame::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_coordinate_system(CoordinateSystem::default());
        test.set_conversion(Conversion::default());
        test.set_projected_crs(ProjectedCRS::default());
        test.set_axis(Axis::default());
        test.set_unit(Unit::default());
        test.set_anchor("test".into());
        test.set_epoch(1.1);
    }

    #[test]
    fn test_derived_parametric_crs() {
        let mut test = DerivedParametricCRS::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_coordinate_system(CoordinateSystem::default());
        test.set_conversion(Conversion::default());
        test.set_projected_crs(ProjectedCRS::default());
        test.set_axis(Axis::default());
        test.set_unit(Unit::default());
    }

    #[test]
    fn test_derived_projected_crs() {
        let mut test = DerivedProjectedCRS::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_coordinate_system(CoordinateSystem::default());
        test.set_conversion(Conversion::default());
        test.set_projected_crs(ProjectedCRS::default());
        test.set_axis(Axis::default());
        test.set_unit(Unit::default());
    }

    #[test]
    fn test_derived_temporal_crs() {
        let mut test = DerivedTemporalCRS::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_coordinate_system(CoordinateSystem::default());
        test.set_conversion(Conversion::default());
        test.set_axis(Axis::default());
        test.set_unit(Unit::default());
    }

    #[test]
    fn test_derived_vertical_crs() {
        let mut test = DerivedVerticalCRS::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_axis(Axis::default());
        test.set_unit(Unit::default());
        test.set_coordinate_system(CoordinateSystem::default());
        test.set_conversion(Conversion::default());
    }

    #[test]
    fn test_dynamic_geodetic_reference_frame() {
        let mut test = DynamicGeodeticReferenceFrame::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_anchor("test".into());
        test.set_epoch(1.1);
        test.set_frame_epoch(1.1);
        test.set_ellipsoid(Ellipsoid::default());
        test.set_prime_meridian(PrimeMeridian::default());
    }

    #[test]
    fn test_datum_ensemble_member() {
        let mut test = DatumEnsembleMember::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.ids.len(), 3);
        test.set_member(DatumEnsembleMember::default());
    }

    #[test]
    fn test_datum_ensemble() {
        let mut test = DatumEnsemble::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.ids.len(), 3);
        test.set_member(DatumEnsembleMember::default());
    }

    #[test]
    fn test_prime_maridian() {
        let mut test = PrimeMeridian::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.ids.len(), 3);
    }

    #[test]
    fn test_projected_crs() {
        let mut test = ProjectedCRS::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_method(Method::default());
    }

    #[test]
    fn test_conversion() {
        let mut test = Conversion::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.ids.len(), 3);
    }

    #[test]
    fn test_coordinate_metadata() {
        let mut test = CoordinateMetadata::default();
        test.set_epoch(1.1);
    }

    #[test]
    fn test_coordinate_system() {
        let mut test = CoordinateSystem::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.ids.len(), 3);
    }

    #[test]
    fn test_transformation() {
        let mut test = Transformation::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_accuracy("test".into());
        test.set_parameter(ParameterValue::default());
        test.set_projection("test".into());
        test.set_method(Method::default());
    }

    #[test]
    fn test_temporal_crs() {
        let mut test = TemporalCRS::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_coordinate_system(CoordinateSystem::default());
        test.set_axis(Axis::default());
        test.set_unit(Unit::default());
    }

    #[test]
    fn test_temporal_datum() {
        let mut test = TemporalDatum::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
    }

    #[test]
    fn test_vertical_crs() {
        let mut test = VerticalCRS::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_coordinate_system(CoordinateSystem::default());
        test.set_datum(Datum::default());
        test.set_ensemble(DatumEnsemble::default());
        test.set_prime_meridian(PrimeMeridian::default());
        test.set_axis(Axis::default());
        test.set_unit(Unit::default());
    }

    #[test]
    fn test_vertical_reference_frame() {
        let mut test = VerticalReferenceFrame::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_anchor("test".into());
        test.set_epoch(1.1);
    }

    #[test]
    fn test_dynamic_vertical_reference_frame() {
        let mut test = DynamicVerticalReferenceFrame::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_usage(ObjectUsage::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.usages.len(), 4);
        test.set_anchor("test".into());
        test.set_anchor("test".into());
        test.set_epoch(1.1);
        test.set_frame_epoch(1.1);
    }

    #[test]
    fn test_object_usage() {
        let mut test = ObjectUsage::default();
        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.ids.len(), 3);
    }

    #[test]
    fn test_ellipsoid() {
        let test =
            Ellipsoid { radius: Some(ValueInMetreOrValueAndUnit::F64(0.5)), ..Default::default() };
        test.to_projection_transform(&mut ProjectionTransform::default());

        let mut test = Ellipsoid {
            semi_minor_axis: Some(ValueInMetreOrValueAndUnit::F64(0.5)),
            ..Default::default()
        };
        test.to_projection_transform(&mut ProjectionTransform::default());

        test.set_id(Id::default());
        test.set_id(Id::default());
        test.set_id(Id::default());
        assert_eq!(test.ids.len(), 3);
    }
}
