use crate::{geometry::convert, readers::FeatureIterator, writers::Writer};
use alloc::{collections::BTreeSet, format, vec::Vec};
use s2json::{BBox3D, JSONCollection, MValueCompatible, Projection, VectorFeature};
use serde::Serialize;

/// User defined function on how to process the features
pub type OnFeature<M, P, D> = fn(feature: VectorFeature<M, P, D>) -> Option<VectorFeature<M, P, D>>;

/// User defined options on how to store the features
#[derive(Debug)]
pub struct ToJSONOptions<M: Clone, P: MValueCompatible + Serialize, D: MValueCompatible + Serialize>
{
    projection: Option<Projection>,
    build_bbox: Option<bool>,
    on_feature: Option<OnFeature<M, P, D>>,
}
impl<M: Clone, P: MValueCompatible + Serialize, D: MValueCompatible + Serialize> Default
    for ToJSONOptions<M, P, D>
{
    fn default() -> Self {
        ToJSONOptions { projection: None, build_bbox: None, on_feature: None }
    }
}

/// Given a writer and an array of iterators, write the input features to the writer as a JSON object
pub fn to_json<
    T: Writer,
    M: Clone + Serialize,
    P: MValueCompatible + Serialize,
    D: MValueCompatible + Serialize,
    I: FeatureIterator<M, P, D>,
>(
    writer: &mut T,
    iterators: Vec<I>,
    opts: Option<ToJSONOptions<M, P, D>>,
) {
    let opts = opts.unwrap_or_default();
    let projection = opts.projection.unwrap_or(Projection::S2);
    let on_feature = opts.on_feature.unwrap_or(Some);
    let build_bbox = opts.build_bbox.unwrap_or(true);
    let mut bbox = BBox3D::default();
    let mut faces: BTreeSet<u8> = BTreeSet::new();

    writer.append_string("{\n\t\"type\": \"${type}\",\n");
    writer.append_string("\t\"features\": [\n");

    let mut first = true;
    for iterator in iterators {
        for feature in iterator {
            let converted_features = convert(
                projection,
                &JSONCollection::VectorFeature(feature),
                None,
                None,
                Some(build_bbox),
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
                writer.append_string("\t\t");
                writer.append_string(&serde_json::to_string(&user_feature).unwrap());
            }
        }
    }

    writer.append_string("\n\t],");
    let faces_vec = faces.iter().collect::<Vec<_>>();
    writer.append_string(&format!("\n\t\"faces\": {:?}", faces_vec));
    if build_bbox {
        writer
            .append_string(&format!(",\n\t\"bbox\": {:?}", &serde_json::to_string(&bbox).unwrap()));
    }
    writer.append_string("\n}");
}

/// Given a writer and an array of iterators, write the input features to the writer as JSON-LD
pub fn to_jsonld<
    T: Writer,
    M: Clone + Serialize,
    P: MValueCompatible + Serialize,
    D: MValueCompatible + Serialize,
    I: FeatureIterator<M, P, D>,
>(
    writer: &mut T,
    iterators: Vec<I>,
    opts: Option<ToJSONOptions<M, P, D>>,
) {
    let opts = opts.unwrap_or_default();
    let projection = opts.projection.unwrap_or(Projection::S2);
    let on_feature = opts.on_feature.unwrap_or(Some);
    let build_bbox = opts.build_bbox.unwrap_or(true);

    for iterator in iterators {
        for feature in iterator {
            let converted_features = convert(
                projection,
                &JSONCollection::VectorFeature(feature),
                None,
                None,
                Some(build_bbox),
            );
            for converted_feature in converted_features {
                let user_feature = on_feature(converted_feature);
                if let Some(user_feature) = user_feature {
                    let feature_str = serde_json::to_string(&user_feature).unwrap();
                    writer.append_string(&feature_str);
                    writer.append_string("\n");
                }
            }
        }
    }
}
