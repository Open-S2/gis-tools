#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;

    use gistools::{
        readers::{
            GTFSIncrementality, GTFSRealtimeAlert, GTFSRealtimeEntity, GTFSRealtimeEntityMessage,
            GTFSRealtimeEntitySelector, GTFSRealtimeHeader, GTFSRealtimeModifiedTripSelector,
            GTFSRealtimeMultiCarriageDetails, GTFSRealtimePosition, GTFSRealtimeReader,
            GTFSRealtimeShape, GTFSRealtimeStop, GTFSRealtimeStopTimeEvent,
            GTFSRealtimeStopTimeProperties, GTFSRealtimeStopTimeUpdate, GTFSRealtimeTimeRange,
            GTFSRealtimeTranslatedString, GTFSRealtimeTranslation, GTFSRealtimeTripDescriptor,
            GTFSRealtimeTripModifications, GTFSRealtimeTripProperties, GTFSRealtimeTripUpdate,
            GTFSRealtimeVehicleDescriptor, GTFSRealtimeVehiclePosition,
            GTFSRealtimeWheelchairAccessible, GTFSVehicleStopStatus,
        },
        util::Date,
    };
    use pbf::{ProtoRead, Protobuf};
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

    #[test]
    fn gtfs_realtime_gtfsrealtime_translated_string() {
        let mut test = GTFSRealtimeTranslatedString::default();
        assert_eq!(test.to_string(None), "");

        test.translations.push(GTFSRealtimeTranslation {
            language: Some("en".to_string()),
            text: "test".to_string(),
        });
        assert_eq!(test.to_string(None), "test");
    }

    #[test]
    fn gtfs_realtime_entity_message() {
        // deleted
        let reatime = GTFSRealtimeEntity { is_deleted: true, ..Default::default() };
        let message: GTFSRealtimeEntityMessage = (&reatime).into();
        assert_eq!(message, GTFSRealtimeEntityMessage::Deleted);

        // nothing is deleted
        let reatime = GTFSRealtimeEntity::default();
        let message: GTFSRealtimeEntityMessage = (&reatime).into();
        assert_eq!(message, GTFSRealtimeEntityMessage::Deleted);

        // trip_update
        let reatime = GTFSRealtimeEntity {
            trip_update: Some(GTFSRealtimeTripUpdate::default()),
            ..Default::default()
        };
        let message: GTFSRealtimeEntityMessage = (&reatime).into();
        assert_eq!(
            message,
            GTFSRealtimeEntityMessage::TripUpdate(GTFSRealtimeTripUpdate::default())
        );

        // vehicle_position
        let reatime = GTFSRealtimeEntity {
            vehicle_position: Some(GTFSRealtimeVehiclePosition::default()),
            ..Default::default()
        };
        let message: GTFSRealtimeEntityMessage = (&reatime).into();
        assert_eq!(
            message,
            GTFSRealtimeEntityMessage::VehiclePosition(GTFSRealtimeVehiclePosition::default())
        );

        // alert
        let reatime =
            GTFSRealtimeEntity { alert: Some(GTFSRealtimeAlert::default()), ..Default::default() };
        let message: GTFSRealtimeEntityMessage = (&reatime).into();
        assert_eq!(message, GTFSRealtimeEntityMessage::Alert(GTFSRealtimeAlert::default()));

        // shape
        let reatime =
            GTFSRealtimeEntity { shape: Some(GTFSRealtimeShape::default()), ..Default::default() };
        let message: GTFSRealtimeEntityMessage = (&reatime).into();
        assert_eq!(message, GTFSRealtimeEntityMessage::Shape(GTFSRealtimeShape::default()));

        // stop
        let reatime =
            GTFSRealtimeEntity { stop: Some(GTFSRealtimeStop::default()), ..Default::default() };
        let message: GTFSRealtimeEntityMessage = (&reatime).into();
        assert_eq!(message, GTFSRealtimeEntityMessage::Stop(GTFSRealtimeStop::default()));

        // trip_modifications
        let reatime = GTFSRealtimeEntity {
            trip_modifications: Some(GTFSRealtimeTripModifications::default()),
            ..Default::default()
        };
        let message: GTFSRealtimeEntityMessage = (&reatime).into();
        assert_eq!(
            message,
            GTFSRealtimeEntityMessage::TripModifications(GTFSRealtimeTripModifications::default())
        );
    }

    #[test]
    fn gtfs_realtime_modified_trip_descriptor() {
        let trip_descriptor = GTFSRealtimeModifiedTripSelector {
            modifications_id: Some("id_1".into()),
            affected_trip_id: Some("trip_id_1".into()),
            start_time: Some("00:00:00".into()),
            start_date: Some(Date::new(2020, 1, 1)),
        };
        let mut pb = Protobuf::default();
        pb.write_fields(&trip_descriptor);
        pb.set_pos(0);
        let mut trip_descriptor2 = GTFSRealtimeModifiedTripSelector::default();
        pb.read_fields(&mut trip_descriptor2, None);
        assert_eq!(trip_descriptor, trip_descriptor2);
    }

    #[test]
    #[should_panic]
    fn gtfs_realtime_gtfsrealtime_translated_string_should_panic() {
        let mut test = GTFSRealtimeTranslatedString::default();
        test.read(0, &mut Protobuf::default());
    }

    #[test]
    #[should_panic]
    fn gtfs_realtime_gtfsrealtime_translated_should_panic() {
        let mut test = GTFSRealtimeTranslation::default();
        test.read(0, &mut Protobuf::default());
    }

    #[test]
    #[should_panic]
    fn gtfs_realtime_vehicle_position_should_panic() {
        let mut test = GTFSRealtimeVehiclePosition::default();
        test.read(0, &mut Protobuf::default());
    }

    #[test]
    #[should_panic]
    fn gtfs_realtime_vehicle_descriptor_should_panic() {
        let mut test = GTFSRealtimeVehicleDescriptor::default();
        test.read(0, &mut Protobuf::default());
    }

    #[test]
    #[should_panic]
    fn gtfs_realtime_multi_carriage_details_should_panic() {
        let mut test = GTFSRealtimeMultiCarriageDetails::default();
        test.read(0, &mut Protobuf::default());
    }

    #[test]
    #[should_panic]
    fn gtfs_realtime_position_should_panic() {
        let mut test = GTFSRealtimePosition::default();
        test.read(0, &mut Protobuf::default());
    }

    #[test]
    #[should_panic]
    fn gtfs_realtime_reader_should_panic() {
        let mut test = GTFSRealtimeReader::default();
        test.read(0, &mut Protobuf::default());
    }

    #[test]
    #[should_panic]
    fn gtfs_realtime_header_should_panic() {
        let mut test = GTFSRealtimeHeader::default();
        test.read(0, &mut Protobuf::default());
    }

    #[test]
    #[should_panic]
    fn gtfs_realtime_entity_selector_should_panic() {
        let mut test = GTFSRealtimeEntitySelector::default();
        test.read(0, &mut Protobuf::default());
    }

    #[test]
    #[should_panic]
    fn gtfs_realtime_entity_should_panic() {
        let mut test = GTFSRealtimeEntity::default();
        test.read(0, &mut Protobuf::default());
    }

    #[test]
    #[should_panic]
    fn gtfs_realtime_range_panic() {
        let mut test = GTFSRealtimeTimeRange::default();
        test.read(0, &mut Protobuf::default());
    }

    #[test]
    #[should_panic]
    fn gtfs_realtime_alert_panic() {
        let mut test = GTFSRealtimeAlert::default();
        test.read(0, &mut Protobuf::default());
    }

    #[test]
    #[should_panic]
    fn gtfs_realtime_trip_update_panic() {
        let mut test = GTFSRealtimeTripUpdate::default();
        test.read(0, &mut Protobuf::default());
    }

    #[test]
    #[should_panic]
    fn gtfs_realtime_trip_properties_panic() {
        let mut test = GTFSRealtimeTripProperties::default();
        test.read(0, &mut Protobuf::default());
    }

    #[test]
    #[should_panic]
    fn gtfs_realtime_stop_time_properties_panic() {
        let mut test = GTFSRealtimeStopTimeProperties::default();
        test.read(0, &mut Protobuf::default());
    }

    #[test]
    #[should_panic]
    fn gtfs_realtime_stop_time_update_panic() {
        let mut test = GTFSRealtimeStopTimeUpdate::default();
        test.read(0, &mut Protobuf::default());
    }

    #[test]
    #[should_panic]
    fn gtfs_realtime_modified_trip_selector_panic() {
        let mut test = GTFSRealtimeModifiedTripSelector::default();
        test.read(0, &mut Protobuf::default());
    }

    #[test]
    #[should_panic]
    fn gtfs_realtime_trip_descriptor_panic() {
        let mut test = GTFSRealtimeTripDescriptor::default();
        test.read(0, &mut Protobuf::default());
    }

    #[test]
    #[should_panic]
    fn gtfs_realtime_stop_time_event_panic() {
        let mut test = GTFSRealtimeStopTimeEvent::default();
        test.read(0, &mut Protobuf::default());
    }
}
