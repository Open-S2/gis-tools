#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;

    use gistools::readers::{
        GTFSIncrementality, GTFSRealtimeHeader, GTFSRealtimeReader,
        GTFSRealtimeWheelchairAccessible, GTFSVehicleStopStatus,
    };
    use std::path::PathBuf;

    #[test]
    fn gtfs_vehicle_position() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gtfs/fixtures/vehicle_position.pb");

        let data = std::fs::read(path).unwrap();
        let reader = GTFSRealtimeReader::new(data, None);

        assert_eq!(
            reader.header,
            GTFSRealtimeHeader {
                gtfs_realtime_version: "1.0".to_string(),
                incrementality: GTFSIncrementality::FullDataset,
                timestamp: None,
                feed_version: None
            }
        );

        let entities = &reader.entities;
        assert_eq!(entities.len(), 1);

        let first_entity = &entities[0];
        assert_eq!(first_entity.id, "1");
        assert_eq!(first_entity.is_deleted, false);
        assert!(first_entity.vehicle_position.is_some());

        let vehicle_position = first_entity.vehicle_position.as_ref().unwrap();
        assert!(vehicle_position.trip.is_some());
        assert!(vehicle_position.current_stop_sequence.is_none());
        assert_eq!(vehicle_position.current_status, GTFSVehicleStopStatus::InTransitTo);
        assert!(vehicle_position.timestamp.is_none());
        assert!(vehicle_position.vehicle.is_some());
        assert!(vehicle_position.occupancy_status.is_none());
        assert!(vehicle_position.occupancy_percentage.is_none());
        assert!(vehicle_position.multi_carriage_details.is_empty());

        let trip = vehicle_position.trip.as_ref().unwrap();
        assert_eq!(trip.trip_id, Some("t0".into()));
        assert!(trip.start_time.is_none());
        assert!(trip.start_date.is_none());
        assert!(trip.schedule_relationship.is_none());
        assert!(trip.route_id.is_none());
        assert!(trip.direction_id.is_none());
        assert!(trip.modified_trip.is_none());

        let position = &vehicle_position.position;
        assert_eq!(position.latitude, 47.0);
        assert_eq!(position.longitude, -122.0);
        assert!(position.bearing.is_none());
        assert!(position.odometer.is_none());
        assert!(position.speed.is_none());

        let vehicle = vehicle_position.vehicle.as_ref().unwrap();
        assert_eq!(vehicle.id, Some("1".into()));
        assert!(vehicle.label.is_none());
        assert!(vehicle.license_plate.is_none());
        assert_eq!(vehicle.wheelchair_accessible, GTFSRealtimeWheelchairAccessible::NoValue);
    }

    #[test]
    fn gtfs_successfully_read_test_data_1() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gtfs/fixtures/realtime_test_data_1.pb");

        let data = std::fs::read(path).unwrap();
        let _reader = GTFSRealtimeReader::new(data, None);
    }

    #[test]
    fn gtfs_successfully_read_test_data_2() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gtfs/fixtures/realtime_test_data_2.pb");

        let data = std::fs::read(path).unwrap();
        let _reader = GTFSRealtimeReader::new(data, None);
    }

    #[test]
    fn gtfs_successfully_read_test_data_3() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gtfs/fixtures/realtime_test_data_3.pb");

        let data = std::fs::read(path).unwrap();
        let _reader = GTFSRealtimeReader::new(data, None);
    }

    #[test]
    fn gtfs_successfully_read_test_data_4() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gtfs/fixtures/realtime_test_data_4.pb");

        let data = std::fs::read(path).unwrap();
        let _reader = GTFSRealtimeReader::new(data, None);
    }

    #[test]
    fn gtfs_successfully_read_test_data_5() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gtfs/fixtures/realtime_test_data_5.pb");

        let data = std::fs::read(path).unwrap();
        let _reader = GTFSRealtimeReader::new(data, None);
    }

    #[test]
    fn gtfs_successfully_read_test_data_6() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gtfs/fixtures/realtime_test_data_6.pb");

        let data = std::fs::read(path).unwrap();
        let _reader = GTFSRealtimeReader::new(data, None);
    }

    #[test]
    fn gtfs_successfully_read_test_data_7() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gtfs/fixtures/realtime_test_data_7.pb");

        let data = std::fs::read(path).unwrap();
        let _reader = GTFSRealtimeReader::new(data, None);
    }

    #[test]
    fn gtfs_successfully_read_test_data_8() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gtfs/fixtures/realtime_test_data_8.pb");

        let data = std::fs::read(path).unwrap();
        let _reader = GTFSRealtimeReader::new(data, None);
    }

    #[test]
    fn gtfs_successfully_read_test_data_9() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gtfs/fixtures/realtime_test_data_9.pb");

        let data = std::fs::read(path).unwrap();
        let _reader = GTFSRealtimeReader::new(data, None);
    }

    #[test]
    fn gtfs_successfully_read_test_data_10() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gtfs/fixtures/realtime_test_data_10.pb");

        let data = std::fs::read(path).unwrap();
        let _reader = GTFSRealtimeReader::new(data, None);
    }

    #[test]
    fn gtfs_successfully_read_test_data_11() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gtfs/fixtures/realtime_test_data_11.pb");

        let data = std::fs::read(path).unwrap();
        let _reader = GTFSRealtimeReader::new(data, None);
    }
}
