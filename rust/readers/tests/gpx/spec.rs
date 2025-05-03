#[cfg(test)]
#[coverage(off)]
mod tests {
    use std::str::FromStr;

    use parsers::{XMLTagItem, xml::XMLTag};
    use readers::{
        GPX, GPXBounds, GPXCopyright, GPXEmail, GPXFixType, GPXLink, GPXMetadata, GPXPerson,
        GPXRoute, GPXTrack, GPXTrackSegment, GPXWaypoint,
    };
    use s2json::{PrimitiveValue, ValueType};

    #[test]
    fn test_gpx_link_new() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<link href="https://example.com"><text>Example Link</text><type>text/html</type></link>"#.into(),
            inner: Some(r#"<text>Example Link</text><type>text/html</type>"#.into()),
            start: 0,
            end: 78,
        });

        let link = GPXLink::new(xml_tag);

        assert_eq!(link.href, "https://example.com");
        assert_eq!(link.text, Some("Example Link".into()));
        assert_eq!(link.r#type, Some("text/html".into()));

        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<link href="https://example.com"/>"#.into(),
            inner: None,
            start: 0,
            end: 32,
        });

        let link = GPXLink::new(xml_tag);

        assert_eq!(link.href, "https://example.com");
        assert_eq!(link.text, None);
        assert_eq!(link.r#type, None);
    }

    #[test]
    fn test_gpx_bounds_new() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<bounds minlat="1.0" minlon="2.0" maxlat="3.0" maxlon="4.0"/>"#.into(),
            inner: None,
            start: 0,
            end: 58,
        });

        let bounds = GPXBounds::new(xml_tag);

        assert_eq!(bounds.minlat, 1.0);
        assert_eq!(bounds.minlon, 2.0);
        assert_eq!(bounds.maxlat, 3.0);
        assert_eq!(bounds.maxlon, 4.0);

        let xml_tag_invalid = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<bounds minlat="invalid" minlon="2.0" maxlat="3.0" maxlon="4.0"/>"#.into(),
            inner: None,
            start: 0,
            end: 67,
        });

        let bounds_invalid = GPXBounds::new(xml_tag_invalid);

        assert_eq!(bounds_invalid.minlat, 0.0);
        assert_eq!(bounds_invalid.minlon, 2.0);
        assert_eq!(bounds_invalid.maxlat, 3.0);
        assert_eq!(bounds_invalid.maxlon, 4.0);

        let xml_tag_missing = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<bounds/>"#.into(),
            inner: None,
            start: 0,
            end: 10,
        });

        let bounds_missing = GPXBounds::new(xml_tag_missing);

        assert_eq!(bounds_missing.minlat, 0.0);
        assert_eq!(bounds_missing.minlon, 0.0);
        assert_eq!(bounds_missing.maxlat, 0.0);
        assert_eq!(bounds_missing.maxlon, 0.0);
    }

    #[test]
    fn test_gpx_email_new() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<email id="user" domain="example.com"/>"#.into(),
            inner: None,
            start: 0,
            end: 42,
        });

        let email = GPXEmail::new(xml_tag);

        assert_eq!(email.id, "user");
        assert_eq!(email.domain, "example.com");

        let xml_tag_missing = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<email/>"#.into(),
            inner: None,
            start: 0,
            end: 8,
        });

        let email_missing = GPXEmail::new(xml_tag_missing);

        assert_eq!(email_missing.id, "");
        assert_eq!(email_missing.domain, "");

        let xml_tag_empty = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<email id="" domain=""/>"#.into(),
            inner: None,
            start: 0,
            end: 23,
        });

        let email_empty = GPXEmail::new(xml_tag_empty);

        assert_eq!(email_empty.id, "");
        assert_eq!(email_empty.domain, "");
    }

    #[test]
    fn test_gpx_person_new() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<person><name>John Doe</name><email id="user" domain="example.com"/><link href="https://example.com"/></person>"#.into(),
            inner: Some(r#"<name>John Doe</name><email id="user" domain="example.com"/><link href="https://example.com"/>"#.into()),
            start: 0,
            end: 121,
        });

        let person = GPXPerson::new(xml_tag);

        assert_eq!(person.name, Some("John Doe".into()));
        assert_eq!(
            person.email,
            Some(GPXEmail { id: "user".into(), domain: "example.com".into() })
        );
        assert_eq!(
            person.link,
            Some(GPXLink { href: "https://example.com".into(), text: None, r#type: None })
        );

        let xml_tag_missing = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<person/>"#.into(),
            inner: Some("".into()),
            start: 0,
            end: 9,
        });

        let person_missing = GPXPerson::new(xml_tag_missing);

        assert_eq!(person_missing.name, None);
        assert_eq!(person_missing.email, None);
        assert_eq!(person_missing.link, None);

        let xml_tag_partial = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<person><name>John Doe</name></person>"#.into(),
            inner: Some(r#"<name>John Doe</name>"#.into()),
            start: 0,
            end: 36,
        });

        let person_partial = GPXPerson::new(xml_tag_partial);

        assert_eq!(person_partial.name, Some("John Doe".into()));
        assert_eq!(person_partial.email, None);
        assert_eq!(person_partial.link, None);
    }

    #[test]
    fn test_gpx_copyright_new() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<copyright><author>John Doe</author><year>2023</year><license>https://example.com/license</license></copyright>"#.into(),
            inner: Some(r#"<author>John Doe</author><year>2023</year><license>https://example.com/license</license>"#.into()),
            start: 0,
            end: 112,
        });

        let copyright = GPXCopyright::new(xml_tag);

        assert_eq!(copyright.author, "John Doe");
        assert_eq!(copyright.year, Some("2023".into()));
        assert_eq!(copyright.license, Some("https://example.com/license".into()));

        let xml_tag_missing = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<copyright/>"#.into(),
            inner: Some("".into()),
            start: 0,
            end: 11,
        });

        let copyright_missing = GPXCopyright::new(xml_tag_missing);

        assert_eq!(copyright_missing.author, "");
        assert_eq!(copyright_missing.year, None);
        assert_eq!(copyright_missing.license, None);

        let xml_tag_partial = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<copyright><author>John Doe</author></copyright>"#.into(),
            inner: Some(r#"<author>John Doe</author>"#.into()),
            start: 0,
            end: 42,
        });

        let copyright_partial = GPXCopyright::new(xml_tag_partial);

        assert_eq!(copyright_partial.author, "John Doe");
        assert_eq!(copyright_partial.year, None);
        assert_eq!(copyright_partial.license, None);
    }

    #[test]
    fn test_gpx_waypoint_new() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<wpt lat="1.0" lon="2.0"><ele>10.0</ele><time>2023-10-27T12:00:00Z</time><name>Test Waypoint</name><link href="https://example.com"/></wpt>"#.into(),
            inner: Some(r#"<ele>10.0</ele><time>2023-10-27T12:00:00Z</time><name>Test Waypoint</name><link href="https://example.com"/>"#.into()),
            start: 0,
            end: 181,
        });

        let waypoint = GPXWaypoint::new(xml_tag);

        assert_eq!(waypoint.lat, 1.0);
        assert_eq!(waypoint.lon, 2.0);
        assert_eq!(waypoint.ele, Some(10.0));
        assert_eq!(waypoint.time, Some("2023-10-27T12:00:00Z".into()));
        assert_eq!(waypoint.name, Some("Test Waypoint".into()));
        assert_eq!(waypoint.link.as_ref().unwrap()[0].href, "https://example.com");
    }
    #[test]
    fn test_gpx_waypoint_new_minimal() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<wpt lat="1.0" lon="2.0"/>"#.into(),
            inner: Some("".into()),
            start: 0,
            end: 23,
        });

        let waypoint = GPXWaypoint::new(xml_tag);

        assert_eq!(waypoint.lat, 1.0);
        assert_eq!(waypoint.lon, 2.0);
        assert_eq!(waypoint.ele, None);
        assert_eq!(waypoint.time, None);
        assert_eq!(waypoint.name, None);
        assert_eq!(waypoint.link, None);
    }

    #[test]
    fn test_gpx_waypoint_multiple_links() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
        outer: r#"<wpt lat="1.0" lon="2.0"><link href="https://example.com"/><link href="https://example2.com"/></wpt>"#.into(),
        inner: Some(r#"<link href="https://example.com"/><link href="https://example2.com"/>"#.into()),
        start: 0,
        end: 100,
      });

        let waypoint = GPXWaypoint::new(xml_tag);
        assert_eq!(waypoint.link.as_ref().unwrap().len(), 2);
        assert_eq!(waypoint.link.as_ref().unwrap()[0].href, "https://example.com");
        assert_eq!(waypoint.link.as_ref().unwrap()[1].href, "https://example2.com");
    }

    #[test]
    fn test_gpx_track_segment_new() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<trkseg><trkpt lat="1.0" lon="2.0"/><trkpt lat="3.0" lon="4.0"/></trkseg>"#
                .into(),
            inner: Some(r#"<trkpt lat="1.0" lon="2.0"/><trkpt lat="3.0" lon="4.0"/>"#.into()),
            start: 0,
            end: 80,
        });

        let track_segment = GPXTrackSegment::new(xml_tag);

        assert_eq!(track_segment.trkpt.as_ref().unwrap().len(), 2);
        assert_eq!(track_segment.trkpt.as_ref().unwrap()[0].lat, 1.0);
        assert_eq!(track_segment.trkpt.as_ref().unwrap()[0].lon, 2.0);
        assert_eq!(track_segment.trkpt.as_ref().unwrap()[1].lat, 3.0);
        assert_eq!(track_segment.trkpt.as_ref().unwrap()[1].lon, 4.0);
    }

    #[test]
    fn test_gpx_track_segment_new_empty() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<trkseg/>"#.into(),
            inner: Some("".into()),
            start: 0,
            end: 10,
        });

        let track_segment = GPXTrackSegment::new(xml_tag);

        assert_eq!(track_segment.trkpt, None);
    }

    #[test]
    fn test_gpx_route_new() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<rte><name>Test Route</name><rtept lat="1.0" lon="2.0"/><rtept lat="3.0" lon="4.0"/></rte>"#.into(),
            inner: Some(r#"<name>Test Route</name><rtept lat="1.0" lon="2.0"/><rtept lat="3.0" lon="4.0"/>"#.into()),
            start: 0,
            end: 93,
        });

        let route = GPXRoute::new(xml_tag);

        assert_eq!(route.name, Some("Test Route".into()));
        assert_eq!(route.rtept.as_ref().unwrap().len(), 2);
        assert_eq!(route.rtept.as_ref().unwrap()[0].lat, 1.0);
        assert_eq!(route.rtept.as_ref().unwrap()[0].lon, 2.0);
        assert_eq!(route.rtept.as_ref().unwrap()[1].lat, 3.0);
        assert_eq!(route.rtept.as_ref().unwrap()[1].lon, 4.0);
    }

    #[test]
    fn test_gpx_route_new_empty() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<rte/>"#.into(),
            inner: Some("".into()),
            start: 0,
            end: 6,
        });

        let route = GPXRoute::new(xml_tag);

        assert_eq!(route.name, None);
        assert_eq!(route.rtept, None);
    }

    #[test]
    fn test_gpx_route_new_links() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<rte><link href="https://example.com"/><link href="https://example2.com"/></rte>"#.into(),
            inner: Some(r#"<link href="https://example.com"/><link href="https://example2.com"/>"#.into()),
            start: 0,
            end: 80,
        });

        let route = GPXRoute::new(xml_tag);

        assert_eq!(route.link.as_ref().unwrap().len(), 2);
        assert_eq!(route.link.as_ref().unwrap()[0].href, "https://example.com");
        assert_eq!(route.link.as_ref().unwrap()[1].href, "https://example2.com");
    }

    #[test]
    fn test_gpx_metadata_new() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<metadata><name>Test Metadata</name><author><name>John Doe</name></author><bounds minlat="1.0" minlon="2.0" maxlat="3.0" maxlon="4.0"/></metadata>"#.into(),
            inner: Some(r#"<name>Test Metadata</name><author><name>John Doe</name></author><bounds minlat="1.0" minlon="2.0" maxlat="3.0" maxlon="4.0"/>"#.into()),
            start: 0,
            end: 181,
        });

        let metadata = GPXMetadata::new(xml_tag);

        assert_eq!(metadata.name, Some("Test Metadata".into()));
        assert_eq!(metadata.author.as_ref().unwrap().name, Some("John Doe".into()));
        assert_eq!(metadata.bounds.as_ref().unwrap().minlat, 1.0);
        assert_eq!(metadata.bounds.as_ref().unwrap().minlon, 2.0);
        assert_eq!(metadata.bounds.as_ref().unwrap().maxlat, 3.0);
        assert_eq!(metadata.bounds.as_ref().unwrap().maxlon, 4.0);
    }

    #[test]
    fn test_gpx_metadata_new_empty() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<metadata/>"#.into(),
            inner: Some("".into()),
            start: 0,
            end: 11,
        });

        let metadata = GPXMetadata::new(xml_tag);

        assert_eq!(metadata.name, None);
        assert_eq!(metadata.author, None);
        assert_eq!(metadata.bounds, None);
    }

    #[test]
    fn test_gpx_track_new() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer:
                r#"<trk><name>Test Track</name><trkseg><trkpt lat="1.0" lon="2.0"/></trkseg></trk>"#
                    .into(),
            inner: Some(
                r#"<name>Test Track</name><trkseg><trkpt lat="1.0" lon="2.0"/></trkseg>"#.into(),
            ),
            start: 0,
            end: 80,
        });

        let track = GPXTrack::new(xml_tag);

        assert_eq!(track.name, Some("Test Track".into()));
        assert_eq!(track.trkseg.as_ref().unwrap().len(), 1);
        assert_eq!(track.trkseg.as_ref().unwrap()[0].trkpt.as_ref().unwrap()[0].lat, 1.0);
        assert_eq!(track.trkseg.as_ref().unwrap()[0].trkpt.as_ref().unwrap()[0].lon, 2.0);
    }

    #[test]
    fn test_gpx_track_new_empty() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<trk/>"#.into(),
            inner: Some("".into()),
            start: 0,
            end: 6,
        });

        let track = GPXTrack::new(xml_tag);

        assert_eq!(track.name, None);
        assert_eq!(track.trkseg, None);
    }

    #[test]
    fn test_gpx_track_new_links() {
        let xml_tag = XMLTagItem::XMLTag(XMLTag {
            outer: r#"<trk><link href="https://example.com"/><link href="https://example2.com"/></trk>"#.into(),
            inner: Some(r#"<link href="https://example.com"/><link href="https://example2.com"/>"#.into()),
            start: 0,
            end: 80,
        });

        let track = GPXTrack::new(xml_tag);

        assert_eq!(track.link.as_ref().unwrap().len(), 2);
        assert_eq!(track.link.as_ref().unwrap()[0].href, "https://example.com");
        assert_eq!(track.link.as_ref().unwrap()[1].href, "https://example2.com");
    }

    #[test]
    fn test_gpx_new() {
        let gpx_xml = r#"
            <gpx version="1.1" creator="Test Creator">
                <metadata><name>Test Metadata</name></metadata>
                <wpt lat="1.0" lon="2.0"/>
                <rte><name>Test Route</name></rte>
                <trk><name>Test Track</name></trk>
            </gpx>
        "#;

        let gpx = GPX::new(gpx_xml);

        assert_eq!(gpx.version, "1.1");
        assert_eq!(gpx.creator, "Test Creator");
        assert_eq!(gpx.metadata.as_ref().unwrap().name, Some("Test Metadata".into()));
        assert_eq!(gpx.wpt.as_ref().unwrap().len(), 1);
        assert_eq!(gpx.rte.as_ref().unwrap().len(), 1);
        assert_eq!(gpx.trk.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_gpx_new_empty() {
        let gpx_xml = r#"<gpx version="1.1" creator="Test Creator"></gpx>"#;

        let gpx = GPX::new(gpx_xml);

        assert_eq!(gpx.version, "1.1");
        assert_eq!(gpx.creator, "Test Creator");
        assert_eq!(gpx.metadata, None);
        assert_eq!(gpx.wpt, None);
        assert_eq!(gpx.rte, None);
        assert_eq!(gpx.trk, None);
    }

    #[test]
    fn test_gpx_fix_type_default() {
        assert_eq!(GPXFixType::default(), GPXFixType::None);
    }

    #[test]
    fn test_gpx_fix_type_display() {
        assert_eq!(GPXFixType::None.to_string(), "none");
        assert_eq!(GPXFixType::D2.to_string(), "2d");
        assert_eq!(GPXFixType::D3.to_string(), "3d");
        assert_eq!(GPXFixType::Dgps.to_string(), "dgps");
        assert_eq!(GPXFixType::Pps.to_string(), "pps");
    }

    #[test]
    fn test_gpx_fix_type_from_str() {
        assert_eq!(GPXFixType::from_str("none").unwrap(), GPXFixType::None);
        assert_eq!(GPXFixType::from_str("2d").unwrap(), GPXFixType::D2);
        assert_eq!(GPXFixType::from_str("3d").unwrap(), GPXFixType::D3);
        assert_eq!(GPXFixType::from_str("dgps").unwrap(), GPXFixType::Dgps);
        assert_eq!(GPXFixType::from_str("pps").unwrap(), GPXFixType::Pps);
        assert_eq!(GPXFixType::from_str("invalid").unwrap(), GPXFixType::None);
    }

    #[test]
    fn test_gpx_fix_type_from_string() {
        assert_eq!(GPXFixType::try_from("none".to_string()).unwrap(), GPXFixType::None);
        assert_eq!(GPXFixType::try_from("2d".to_string()).unwrap(), GPXFixType::D2);
        assert_eq!(GPXFixType::try_from("3d".to_string()).unwrap(), GPXFixType::D3);
        assert_eq!(GPXFixType::try_from("dgps".to_string()).unwrap(), GPXFixType::Dgps);
        assert_eq!(GPXFixType::try_from("pps".to_string()).unwrap(), GPXFixType::Pps);
        assert_eq!(GPXFixType::try_from("invalid".to_string()).unwrap(), GPXFixType::None);
    }

    #[test]
    fn test_gpx_fix_type_to_value_type() {
        let none_value: ValueType = GPXFixType::None.into();
        assert_eq!(none_value, ValueType::Primitive(PrimitiveValue::String("none".to_string())));

        let d2_value: ValueType = GPXFixType::D2.into();
        assert_eq!(d2_value, ValueType::Primitive(PrimitiveValue::String("2d".to_string())));

        let d3_value: ValueType = GPXFixType::D3.into();
        assert_eq!(d3_value, ValueType::Primitive(PrimitiveValue::String("3d".to_string())));

        let dgps_value: ValueType = GPXFixType::Dgps.into();
        assert_eq!(dgps_value, ValueType::Primitive(PrimitiveValue::String("dgps".to_string())));

        let pps_value: ValueType = GPXFixType::Pps.into();
        assert_eq!(pps_value, ValueType::Primitive(PrimitiveValue::String("pps".to_string())));
    }

    #[test]
    fn test_gpx_fix_type_from_value_type() {
        let none_value = ValueType::Primitive(PrimitiveValue::String("none".to_string()));
        assert_eq!(GPXFixType::from(&none_value), GPXFixType::None);

        let d2_value = ValueType::Primitive(PrimitiveValue::String("2d".to_string()));
        assert_eq!(GPXFixType::from(&d2_value), GPXFixType::D2);

        let d3_value = ValueType::Primitive(PrimitiveValue::String("3d".to_string()));
        assert_eq!(GPXFixType::from(&d3_value), GPXFixType::D3);

        let dgps_value = ValueType::Primitive(PrimitiveValue::String("dgps".to_string()));
        assert_eq!(GPXFixType::from(&dgps_value), GPXFixType::Dgps);

        let pps_value = ValueType::Primitive(PrimitiveValue::String("pps".to_string()));
        assert_eq!(GPXFixType::from(&pps_value), GPXFixType::Pps);

        let invalid_value = ValueType::Primitive(PrimitiveValue::String("invalid".to_string()));
        assert_eq!(GPXFixType::from(&invalid_value), GPXFixType::None);

        let non_string_value = ValueType::Nested(Default::default());
        assert_eq!(GPXFixType::from(&non_string_value), GPXFixType::None);
    }

    #[test]
    fn test_gpx_fix_type_serde_json() {
        let none_json = serde_json::to_string(&GPXFixType::None).unwrap();
        assert_eq!(none_json, "\"none\"");
        assert_eq!(serde_json::from_str::<GPXFixType>(&none_json).unwrap(), GPXFixType::None);

        let d2_json = serde_json::to_string(&GPXFixType::D2).unwrap();
        assert_eq!(d2_json, "\"2d\"");
        assert_eq!(serde_json::from_str::<GPXFixType>(&d2_json).unwrap(), GPXFixType::D2);

        let d3_json = serde_json::to_string(&GPXFixType::D3).unwrap();
        assert_eq!(d3_json, "\"3d\"");
        assert_eq!(serde_json::from_str::<GPXFixType>(&d3_json).unwrap(), GPXFixType::D3);

        let dgps_json = serde_json::to_string(&GPXFixType::Dgps).unwrap();
        assert_eq!(dgps_json, "\"dgps\"");
        assert_eq!(serde_json::from_str::<GPXFixType>(&dgps_json).unwrap(), GPXFixType::Dgps);

        let pps_json = serde_json::to_string(&GPXFixType::Pps).unwrap();
        assert_eq!(pps_json, "\"pps\"");
        assert_eq!(serde_json::from_str::<GPXFixType>(&pps_json).unwrap(), GPXFixType::Pps);
    }
}
