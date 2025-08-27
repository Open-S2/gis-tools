mod v1_1;

#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::readers::{GBFSSystem, gbfs_bool_or_int, parse_gtfs_systems};
    use serde::{
        Deserialize, Deserializer,
        de::{Error as _, Unexpected, Visitor},
        forward_to_deserialize_any,
    };
    use serde_json::{from_value, json};
    use std::{fs, path::PathBuf};

    #[test]
    fn test_parse_gtfs_systems() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gbfs/fixtures/systems.csv");
        let file_str = fs::read_to_string(path).unwrap();
        let systems = parse_gtfs_systems(file_str.as_str());
        assert_eq!(systems.len(), 960);

        assert_eq!(
            systems[0..5],
            vec![
                GBFSSystem {
                    country_code: "AE".into(),
                    name: "Careem BIKE".into(),
                    location: "Dubai".into(),
                    system_id: "careem_bike".into(),
                    url: "https://www.careem.com/en-ae/careem-bike/".into(),
                    auto_discovery_url:
                        "https://dubai.publicbikesystem.net/customer/gbfs/v2/gbfs.json".into(),
                    supported_versions: vec!["1.1".into(), "2.3".into()],
                    auth_info: None
                },
                GBFSSystem {
                    country_code: "AR".into(),
                    name: "Bike Nordelta".into(),
                    location: "Buenos Aires".into(),
                    system_id: "bike_nordelta".into(),
                    url: "https://bikeitau.com.br/nordelta/".into(),
                    auto_discovery_url:
                        "https://nordelta.publicbikesystem.net/customer/gbfs/v2/gbfs.json".into(),
                    supported_versions: vec!["1.1".into(), "2.3".into()],
                    auth_info: None
                },
                GBFSSystem {
                    country_code: "AR".into(),
                    name: "Ecobici".into(),
                    location: "Buenos Aires".into(),
                    system_id: "bike_buenosaires".into(),
                    url: "https://www.buenosaires.gob.ar/ecobici".into(),
                    auto_discovery_url:
                        "https://buenosaires.publicbikesystem.net/customer/gbfs/v2/gbfs.json".into(),
                    supported_versions: vec!["1.1".into(), "2.3".into()],
                    auth_info: None
                },
                GBFSSystem {
                    country_code: "AR".into(),
                    name: "MiBiciTuBici".into(),
                    location: "Rosario".into(),
                    system_id: "biketobike".into(),
                    url: "https://www.mibicitubici.gob.ar/".into(),
                    auto_discovery_url: "https://www.mibicitubici.gob.ar/opendata/gbfs.json".into(),
                    supported_versions: vec!["1.0".into()],
                    auth_info: None
                },
                GBFSSystem {
                    country_code: "AT".into(),
                    name: "city bike Linz".into(),
                    location: "Linz".into(),
                    system_id: "nextbike_al".into(),
                    url: "https://citybikelinz.at/".into(),
                    auto_discovery_url:
                        "https://gbfs.nextbike.net/maps/gbfs/v2/nextbike_al/gbfs.json".into(),
                    supported_versions: vec!["2.3".into()],
                    auth_info: None
                }
            ]
        );
    }

    #[test]
    fn test_gbfs_bool_or_int() {
        #[derive(Debug, Deserialize)]
        struct Wrapper {
            #[serde(deserialize_with = "gbfs_bool_or_int")]
            value: bool,
        }

        // true
        let val: Wrapper = from_value(json!({ "value": true })).unwrap();
        assert!(val.value);

        // false
        let val: Wrapper = from_value(json!({ "value": false })).unwrap();
        assert!(!val.value);

        // 1 u64
        let val: Wrapper = from_value(json!({ "value": 1_u64 })).unwrap();
        assert!(val.value);

        // 0 u64
        let val: Wrapper = from_value(json!({ "value": 0_u64 })).unwrap();
        assert!(!val.value);

        // 1 i64
        let val: Wrapper = from_value(json!({ "value": 1_i64 })).unwrap();
        assert!(val.value);

        // -1 i64
        let val: Result<Wrapper, _> = from_value(json!({ "value": -1_i64 }));
        assert!(val.is_err());

        // 0 i64
        let val: Wrapper = from_value(json!({ "value": -0_i64 })).unwrap();
        assert!(!val.value);

        // string true
        let val: Wrapper = from_value(json!({ "value": "true" })).unwrap();
        assert!(val.value);

        // string false
        let val: Wrapper = from_value(json!({ "value": "false" })).unwrap();
        assert!(!val.value);

        // string 0
        let val: Wrapper = from_value(json!({ "value": "0" })).unwrap();
        assert!(!val.value);

        // string 1
        let val: Wrapper = from_value(json!({ "value": "1" })).unwrap();
        assert!(val.value);

        // error
        let res: Result<Wrapper, _> = from_value(json!({ "value": 3 }));
        assert!(res.is_err());

        let res: Result<Wrapper, _> = from_value(json!({ "value": "yes" }));
        assert!(res.is_err());
    }

    #[test]
    fn test_expectation_error_triggers() {
        struct Dummy;

        impl<'de> Deserializer<'de> for Dummy {
            type Error = serde::de::value::Error;

            fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
            where
                V: Visitor<'de>,
            {
                // Trigger error using invalid type
                Err(Self::Error::invalid_type(Unexpected::Unit, &visitor))
            }

            // implement all other deserialize_* as unimplemented so `deserialize_any` is called
            forward_to_deserialize_any! {
                bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes byte_buf
                option unit unit_struct newtype_struct seq tuple
                tuple_struct map struct enum identifier ignored_any
            }
        }

        let result = gbfs_bool_or_int(Dummy);
        assert!(result.is_err());

        // Optional: verify that the error contains the string from `expecting`
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("a boolean or an integer 0/1"));
    }
}
