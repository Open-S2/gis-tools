#[cfg(test)]
#[coverage(off)]
mod tests {
    use proj::{AxisDirection, CRS, GeodeticCRS, Id, ProjJSON, ProjectedCRS};

    // TODO: https://proj.org/en/stable/specifications/projjson.html#schema <- more examples to play with

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
}
