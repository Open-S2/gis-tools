#[cfg(test)]
// #[coverage(off)]
mod tests {
    extern crate alloc;

    use alloc::{string::String, vec};
    use parsers::{BufferWriter, FileReader, Writer};
    use readers::json::JSONReader;
    use s2json::{MValueCompatible, Projection};
    use serde::{Deserialize, Serialize};
    use std::path::PathBuf;
    use writers::{ToJSONOptions, to_json, to_jsonld};

    #[test]
    fn test_to_json() {
        #[derive(Debug, Default, Clone, MValueCompatible, PartialEq, Serialize, Deserialize)]
        #[serde(default)]
        struct Props {
            name: String,
        }

        let mut path = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
        path = path.join("tests/converters/fixtures/points.geojson");

        let reader: JSONReader<FileReader, (), Props, ()> =
            JSONReader::new(FileReader::from(path), None);
        let mut writer = BufferWriter::default();

        // write
        to_json(&mut writer, vec![&reader], None);

        // validate
        let writer_str: String = String::from_utf8_lossy(&writer.take()).into();
        let expected = r#"{"type": "S2FeatureCollection","features": [{"type":"S2Feature","face":3,"properties":{"name":"Melbourne"},"geometry":{"type":"Point","is3D":false,"coordinates":{"x":0.9803070552829272,"y":0.1191097721694171},"bbox":[144.9584,-37.8173,144.9584,-37.8173,1.7976931348623157e308,-1.7976931348623157e308]}},{"type":"S2Feature","face":3,"properties":{"name":"Canberra"},"geometry":{"type":"Point","is3D":false,"coordinates":{"x":0.9321761149504832,"y":0.16402766817497416},"bbox":[149.1009,-35.3039,149.1009,-35.3039,1.7976931348623157e308,-1.7976931348623157e308]}},{"type":"S2Feature","face":3,"properties":{"name":"Sydney"},"geometry":{"type":"Point","is3D":false,"coordinates":{"x":0.908036698755368,"y":0.1863228168096237},"bbox":[151.2144,-33.8766,151.2144,-33.8766,1.7976931348623157e308,-1.7976931348623157e308]}}],"faces": [3],"bbox": "[144.9584,-37.8173,151.2144,-33.8766,1.7976931348623157e308,-1.7976931348623157e308]"}"#;
        assert_eq!(remove_newlines_and_tabs(&writer_str), remove_newlines_and_tabs(expected));
    }

    #[test]
    fn test_to_json_flat() {
        #[derive(Debug, Default, Clone, MValueCompatible, PartialEq, Serialize, Deserialize)]
        #[serde(default)]
        struct Props {
            name: String,
        }

        let mut path = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
        path = path.join("tests/converters/fixtures/points.geojson");

        let reader: JSONReader<FileReader, (), Props, ()> =
            JSONReader::new(FileReader::from(path), None);
        let mut writer = BufferWriter::default();

        // write
        to_json(
            &mut writer,
            vec![&reader],
            Some(ToJSONOptions {
                projection: Some(Projection::WG),
                geojson: Some(true),
                ..Default::default()
            }),
        );

        // validate
        let writer_str: String = String::from_utf8_lossy(&writer.take()).into();
        let expected = r#"{"type":"FeatureCollection","features":[{"type":"Feature","properties":{"name":"Melbourne"},"geometry":{"type":"Point","coordinates":[144.9584,-37.8173],"bbox":[144.9584,-37.8173,144.9584,-37.8173]}},{"type":"Feature","properties":{"name":"Canberra"},"geometry":{"type":"Point","coordinates":[149.1009,-35.3039],"bbox":[149.1009,-35.3039,149.1009,-35.3039]}},{"type":"Feature","properties":{"name":"Sydney"},"geometry":{"type":"Point","coordinates":[151.2144,-33.8766],"bbox":[151.2144,-33.8766,151.2144,-33.8766]}}],"faces":[0],"bbox":"[144.9584,-37.8173,151.2144,-33.8766,1.7976931348623157e308,-1.7976931348623157e308]"}"#;
        assert_eq!(remove_newlines_and_tabs(&writer_str), remove_newlines_and_tabs(expected));
    }

    #[test]
    fn test_to_jsonld() {
        #[derive(Debug, Default, Clone, MValueCompatible, PartialEq, Serialize, Deserialize)]
        #[serde(default)]
        struct Props {
            name: String,
        }

        let mut path = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
        path = path.join("tests/converters/fixtures/points.geojson");

        let reader: JSONReader<FileReader, (), Props, ()> =
            JSONReader::new(FileReader::from(path), None);
        let mut writer = BufferWriter::default();

        // write
        to_jsonld(&mut writer, vec![&reader], None);

        // validate
        let writer_str: String = String::from_utf8_lossy(&writer.take()).into();
        let expected = r#"{"type":"S2Feature","face":3,"properties":{"name":"Melbourne"},"geometry":{"type":"Point","is3D":false,"coordinates":{"x":0.9803070552829272,"y":0.1191097721694171},"bbox":[144.9584,-37.8173,144.9584,-37.8173,1.7976931348623157e308,-1.7976931348623157e308]}}{"type":"S2Feature","face":3,"properties":{"name":"Canberra"},"geometry":{"type":"Point","is3D":false,"coordinates":{"x":0.9321761149504832,"y":0.16402766817497416},"bbox":[149.1009,-35.3039,149.1009,-35.3039,1.7976931348623157e308,-1.7976931348623157e308]}}{"type":"S2Feature","face":3,"properties":{"name":"Sydney"},"geometry":{"type":"Point","is3D":false,"coordinates":{"x":0.908036698755368,"y":0.1863228168096237},"bbox":[151.2144,-33.8766,151.2144,-33.8766,1.7976931348623157e308,-1.7976931348623157e308]}}"#;
        assert_eq!(remove_newlines_and_tabs(&writer_str), remove_newlines_and_tabs(expected));
    }

    #[test]
    fn test_to_jsonld_flat() {
        #[derive(Debug, Default, Clone, MValueCompatible, PartialEq, Serialize, Deserialize)]
        #[serde(default)]
        struct Props {
            name: String,
        }

        let mut path = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
        path = path.join("tests/converters/fixtures/points.geojson");

        let reader: JSONReader<FileReader, (), Props, ()> =
            JSONReader::new(FileReader::from(path), None);
        let mut writer = BufferWriter::default();

        // write
        to_jsonld(
            &mut writer,
            vec![&reader],
            Some(ToJSONOptions {
                projection: Some(Projection::WG),
                geojson: Some(true),
                ..Default::default()
            }),
        );

        // validate
        let writer_str: String = String::from_utf8_lossy(&writer.take()).into();
        let expected = r#"{"type":"Feature","properties":{"name":"Melbourne"},"geometry":{"type":"Point","coordinates":[144.9584,-37.8173],"bbox":[144.9584,-37.8173,144.9584,-37.8173]}}{"type":"Feature","properties":{"name":"Canberra"},"geometry":{"type":"Point","coordinates":[149.1009,-35.3039],"bbox":[149.1009,-35.3039,149.1009,-35.3039]}}{"type":"Feature","properties":{"name":"Sydney"},"geometry":{"type":"Point","coordinates":[151.2144,-33.8766],"bbox":[151.2144,-33.8766,151.2144,-33.8766]}}"#;
        assert_eq!(remove_newlines_and_tabs(&writer_str), remove_newlines_and_tabs(expected));
    }

    fn remove_newlines_and_tabs(input: &str) -> String {
        input.chars().filter(|c| *c != '\n' && *c != '\t' && *c != ' ').collect()
    }
}
