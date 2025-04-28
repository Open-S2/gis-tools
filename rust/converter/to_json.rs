use super::OnFeature;
use crate::{
    geometry::{ConvertVectorFeatureWM, convert},
    readers::FeatureReader,
    writers::Writer,
};
use alloc::{collections::BTreeSet, format, vec::Vec};
use s2json::{BBox3D, JSONCollection, Projection};
use serde::Serialize;

/// User defined options on how to store the features
#[derive(Debug)]
pub struct ToJSONOptions<M: Clone, P: Clone + Default + Serialize, D: Clone + Default + Serialize> {
    /// Projection can be S2 or WG
    projection: Option<Projection>,
    /// Build the bounding box
    build_bbox: Option<bool>,
    /// Set to true to get a GeoJSON object
    geojson: Option<bool>,
    /// User defined function on how to store the feature
    on_feature: Option<OnFeature<M, P, D>>,
}
impl<M: Clone, P: Clone + Default + Serialize, D: Clone + Default + Serialize> Default
    for ToJSONOptions<M, P, D>
{
    fn default() -> Self {
        ToJSONOptions { projection: None, build_bbox: None, geojson: None, on_feature: None }
    }
}

/// Given a writer and an array of readers, write the input features to the writer as a JSON object
pub fn to_json<
    T: Writer,
    M: Clone + Serialize,
    P: Clone + Default + Serialize,
    D: Clone + Default + Serialize,
    I: FeatureReader<M, P, D>,
>(
    writer: &mut T,
    readers: Vec<&I>,
    opts: Option<ToJSONOptions<M, P, D>>,
) {
    let opts = opts.unwrap_or_default();
    let projection = opts.projection.unwrap_or(Projection::S2);
    let on_feature = opts.on_feature.unwrap_or(Some);
    let build_bbox = opts.build_bbox.unwrap_or(true);
    let mut bbox = BBox3D::default();
    let mut faces: BTreeSet<u8> = BTreeSet::new();
    let r#type =
        if projection == Projection::S2 { "S2FeatureCollection" } else { "FeatureCollection" };

    writer.append_string("{\n\t\"type\": \"");
    writer.append_string(r#type);
    writer.append_string("\",\n");
    writer.append_string("\t\"features\": [\n");

    let mut first = true;
    for reader in readers {
        for feature in reader.iter() {
            let converted_features = convert(
                projection,
                &JSONCollection::VectorFeature(feature),
                Some(build_bbox),
                None,
            );
            for converted_feature in converted_features {
                let user_feature = on_feature(converted_feature);
                if user_feature.is_none() {
                    continue;
                }
                let user_feature = user_feature.unwrap();
                faces.insert(user_feature.face.into());
                if build_bbox {
                    if let Some(feature_bbox) = user_feature.geometry.bbox() {
                        bbox.merge_in_place(feature_bbox);
                    }
                }
                if !first {
                    writer.append_string(",\n");
                } else {
                    first = false;
                }
                let feature_str = match opts.geojson.unwrap_or(false) {
                    true => serde_json::to_string(&user_feature.to_feature(true)).unwrap(),
                    false => serde_json::to_string(&user_feature).unwrap(),
                };
                writer.append_string("\t\t");
                writer.append_string(&feature_str);
            }
        }
    }

    writer.append_string("\n\t],");
    let faces_vec: Vec<&u8> = faces.iter().collect();
    writer.append_string(&format!("\n\t\"faces\": {:?}", faces_vec));
    if build_bbox {
        writer
            .append_string(&format!(",\n\t\"bbox\": {:?}", &serde_json::to_string(&bbox).unwrap()));
    }
    writer.append_string("\n}");
}

/// Given a writer and an array of readers, write the input features to the writer as JSON-LD
pub fn to_jsonld<
    T: Writer,
    M: Clone + Serialize,
    P: Clone + Default + Serialize,
    D: Clone + Default + Serialize,
    I: FeatureReader<M, P, D>,
>(
    writer: &mut T,
    readers: Vec<&I>,
    opts: Option<ToJSONOptions<M, P, D>>,
) {
    let opts = opts.unwrap_or_default();
    let projection = opts.projection.unwrap_or(Projection::S2);
    let on_feature = opts.on_feature.unwrap_or(Some);
    let build_bbox = opts.build_bbox.unwrap_or(true);

    for reader in readers {
        for feature in reader.iter() {
            let converted_features = convert(
                projection,
                &JSONCollection::VectorFeature(feature),
                Some(build_bbox),
                None,
            );
            for converted_feature in converted_features {
                let user_feature = on_feature(converted_feature);
                if let Some(user_feature) = user_feature {
                    let feature_str = match opts.geojson.unwrap_or(false) {
                        true => serde_json::to_string(&user_feature.to_feature(true)).unwrap(),
                        false => serde_json::to_string(&user_feature).unwrap(),
                    };
                    writer.append_string(&feature_str);
                    writer.append_string("\n");
                }
            }
        }
    }
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use super::*;
    use crate::{
        readers::{FileReader, json::JSONReader},
        writers::BufferWriter,
    };
    use alloc::{string::String, vec};
    use s2json::MValueCompatible;
    use serde::Deserialize;
    use std::path::PathBuf;

    #[test]
    fn test_to_json() {
        #[derive(Debug, Default, Clone, MValueCompatible, PartialEq, Serialize, Deserialize)]
        #[serde(default)]
        struct Props {
            name: String,
        }

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
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

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
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

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
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

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
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
