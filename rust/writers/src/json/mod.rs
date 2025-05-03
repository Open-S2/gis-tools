use super::OnFeature;
use alloc::{collections::BTreeSet, format, vec::Vec};
use geometry::{ConvertVectorFeatureWM, convert};
use parsers::{FeatureReader, Writer};
use s2json::{BBox3D, JSONCollection, Projection};
use serde::Serialize;

/// User defined options on how to store the features
#[derive(Debug)]
pub struct ToJSONOptions<M: Clone, P: Clone + Default + Serialize, D: Clone + Default + Serialize> {
    /// Projection can be S2 or WG
    pub projection: Option<Projection>,
    /// Build the bounding box
    pub build_bbox: Option<bool>,
    /// Set to true to get a GeoJSON object
    pub geojson: Option<bool>,
    /// User defined function on how to store the feature
    pub on_feature: Option<OnFeature<M, P, D>>,
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
