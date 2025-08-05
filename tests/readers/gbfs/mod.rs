mod v1_1;

#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::readers::{GBFSSystem, parse_gtfs_systems};
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
}
