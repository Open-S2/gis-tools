/// Large JSON Parsing
pub mod large_json;
/// NewLine Delimited JSON Reader
pub mod line_delimited;

use super::FeatureReader;
use crate::geometry::ConvertFeature;
use alloc::{vec, vec::Vec};
pub use large_json::*;
pub use line_delimited::*;
use s2json::{
    Feature, FeatureCollection, Features, JSONCollection, S2FeatureCollection, VectorFeature,
};
use serde::{Deserialize, de::DeserializeOwned};

/// Error type for ToGisJSON
#[derive(Debug, Deserialize)]
pub enum ToGISJSONError {
    /// Invalid JSON
    InvalidJSON,
}

/// Converts a String to a JSONCollection. Supports S2 and WGS84 JSON
pub trait ToGisJSON {
    /// Converts a String to a JSONCollection. Supports S2 and WGS84 JSON
    fn to_gis_json<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<JSONCollection<M, P, D>, ToGISJSONError>;
    /// Converts a String to a FeatureCollection
    fn to_feature_collection<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<FeatureCollection<M, P, D>, ToGISJSONError>;
    /// Converts a String to a FeatureCollection
    fn to_s2_feature_collection<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<S2FeatureCollection<M, P, D>, ToGISJSONError>;
    /// Converts a String to a Feature
    fn to_feature<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<Feature<M, P, D>, ToGISJSONError>;
    /// Converts a String to a VectorFeature
    fn to_vector_feature<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<VectorFeature<M, P, D>, ToGISJSONError>;
    /// Converts a String to a WMFeature
    fn to_features<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<Features<M, P, D>, ToGISJSONError>;
}

impl ToGisJSON for &str {
    fn to_gis_json<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<JSONCollection<M, P, D>, ToGISJSONError> {
        serde_json::from_str(self).map_err(|_| ToGISJSONError::InvalidJSON)
    }

    fn to_feature_collection<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<FeatureCollection<M, P, D>, ToGISJSONError> {
        serde_json::from_str(self).map_err(|_| ToGISJSONError::InvalidJSON)
    }

    fn to_s2_feature_collection<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<S2FeatureCollection<M, P, D>, ToGISJSONError> {
        serde_json::from_str(self).map_err(|_| ToGISJSONError::InvalidJSON)
    }

    fn to_feature<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<Feature<M, P, D>, ToGISJSONError> {
        serde_json::from_str(self).map_err(|_| ToGISJSONError::InvalidJSON)
    }

    fn to_vector_feature<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<VectorFeature<M, P, D>, ToGISJSONError> {
        serde_json::from_str(self).map_err(|_| ToGISJSONError::InvalidJSON)
    }

    fn to_features<
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    >(
        &self,
    ) -> Result<Features<M, P, D>, ToGISJSONError> {
        serde_json::from_str(self).map_err(|_| ToGISJSONError::InvalidJSON)
    }
}

/// JSON Collection Reader
#[derive(Debug, Clone)]
pub struct JSONCollectionReader<M: Clone, P: Clone + Default, D: Clone + Default> {
    /// Collection of features
    pub features: Vec<VectorFeature<M, P, D>>,
}
impl<M: Clone, P: Clone + Default, D: Clone + Default> JSONCollectionReader<M, P, D> {
    /// Mutable iterator
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, VectorFeature<M, P, D>> {
        self.features.iter_mut()
    }
}
impl<M: Clone, P: Clone + Default, D: Clone + Default> From<&mut JSONCollection<M, P, D>>
    for JSONCollectionReader<M, P, D>
{
    fn from(collection: &mut JSONCollection<M, P, D>) -> Self {
        let mut features: Vec<VectorFeature<M, P, D>> = vec![];
        match collection {
            JSONCollection::FeatureCollection(collection) => {
                for feature in &mut collection.features {
                    match feature {
                        Features::Feature(feature) => {
                            features.push(feature.to_vector(Some(true)));
                        }
                        Features::VectorFeature(vf) => {
                            features.push(core::mem::take(vf));
                        }
                    }
                }
            }
            JSONCollection::S2FeatureCollection(collection) => {
                features.extend(core::mem::take(&mut collection.features));
            }
            JSONCollection::Feature(feature) => {
                features.push(feature.to_vector(Some(true)));
            }
            JSONCollection::VectorFeature(vf) => {
                features.push(core::mem::take(vf));
            }
        };

        JSONCollectionReader { features }
    }
}
impl<M: Clone + Default, P: Clone + Default, D: Clone + Default>
    From<&mut FeatureCollection<M, P, D>> for JSONCollectionReader<M, P, D>
{
    fn from(collection: &mut FeatureCollection<M, P, D>) -> JSONCollectionReader<M, P, D> {
        JSONCollectionReader::from(&mut JSONCollection::FeatureCollection(core::mem::take(
            collection,
        )))
    }
}
impl<M: Clone + Default, P: Clone + Default, D: Clone + Default>
    From<&mut S2FeatureCollection<M, P, D>> for JSONCollectionReader<M, P, D>
{
    fn from(collection: &mut S2FeatureCollection<M, P, D>) -> JSONCollectionReader<M, P, D> {
        JSONCollectionReader::from(&mut JSONCollection::S2FeatureCollection(core::mem::take(
            collection,
        )))
    }
}
impl<M: Clone, P: Clone + Default, D: Clone + Default> From<&mut VectorFeature<M, P, D>>
    for JSONCollectionReader<M, P, D>
{
    fn from(collection: &mut VectorFeature<M, P, D>) -> JSONCollectionReader<M, P, D> {
        JSONCollectionReader::from(&mut JSONCollection::VectorFeature(core::mem::take(collection)))
    }
}
impl<M: Clone, P: Clone + Default, D: Clone + Default> From<&mut Feature<M, P, D>>
    for JSONCollectionReader<M, P, D>
{
    fn from(feature: &mut Feature<M, P, D>) -> JSONCollectionReader<M, P, D> {
        JSONCollectionReader::from(&mut JSONCollection::Feature(core::mem::take(feature)))
    }
}
impl<M: Clone, P: Clone + Default, D: Clone + Default> IntoIterator
    for JSONCollectionReader<M, P, D>
{
    type Item = VectorFeature<M, P, D>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.features.into_iter()
    }
}
/// The JSON Iterator tool
#[derive(Debug)]
pub struct JSONCollectionIterator<
    'a,
    M: Clone + DeserializeOwned,
    P: Clone + Default + DeserializeOwned,
    D: Clone + Default + DeserializeOwned,
> {
    reader: &'a JSONCollectionReader<M, P, D>,
    offset: usize,
    size: usize,
}
impl<
    M: Clone + DeserializeOwned,
    P: Clone + Default + DeserializeOwned,
    D: Clone + Default + DeserializeOwned,
> Iterator for JSONCollectionIterator<'_, M, P, D>
{
    type Item = VectorFeature<M, P, D>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.size {
            return None;
        }
        self.offset += 1;
        self.reader.features.get(self.offset - 1).cloned()
    }
}
/// A feature reader trait with a callback-based approach
impl<
    M: Clone + DeserializeOwned,
    P: Clone + Default + DeserializeOwned,
    D: Clone + Default + DeserializeOwned,
> FeatureReader<M, P, D> for JSONCollectionReader<M, P, D>
{
    type FeatureIterator<'a>
        = JSONCollectionIterator<'a, M, P, D>
    where
        M: 'a,
        P: 'a,
        D: 'a;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        JSONCollectionIterator { reader: self, offset: 0, size: self.features.len() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;
    use s2json::{
        BBox3D, BaseGeometry, FeatureCollectionType, FeatureType, Geometry, GeometryType, MValue,
        MValueCompatible, Point, S2FeatureCollectionType, VectorBaseGeometry, VectorFeatureType,
        VectorGeometry, VectorGeometryType, VectorPoint,
    };
    use serde::Serialize;

    #[test]
    fn test_json_feature() {
        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Serialize, Deserialize)]
        struct Test {
            name: String,
        }

        let json_str = r#"{
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [100.0, 0.0]
            },
            "properties": {
                "name": "Tokyo"
            }
        }"#;
        let mut json: Feature<(), Test, MValue> = json_str.to_feature().unwrap();
        assert_eq!(
            json,
            Feature {
                _type: FeatureType::Feature,
                id: None,
                properties: Test { name: "Tokyo".into() },
                geometry: Geometry::Point(BaseGeometry {
                    _type: GeometryType::Point,
                    coordinates: Point(100.0, 0.0),
                    ..Default::default()
                }),
                ..Default::default()
            }
        );

        let collection = JSONCollectionReader::from(&mut json);
        assert_eq!(collection.features.len(), 1);

        let json_collection = JSONCollection::Feature(json.clone());
        let json_coll_str = serde_json::to_string(&json_collection).unwrap();
        assert_eq!(
            json_coll_str,
            "{\"type\":\"Feature\",\"properties\":{\"name\":\"\"},\"geometry\":{\"type\":\"Point\"\
             ,\"coordinates\":[0.0,0.0]}}"
        );

        let json_collection: JSONCollection<(), Test, MValue> = json_str.to_gis_json().unwrap();
        assert_eq!(
            json_collection,
            JSONCollection::Feature(Feature {
                _type: FeatureType::Feature,
                id: None,
                properties: Test { name: "Tokyo".into() },
                geometry: Geometry::Point(BaseGeometry {
                    _type: GeometryType::Point,
                    coordinates: Point(100.0, 0.0),
                    ..Default::default()
                }),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_json_feature_collection() {
        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Deserialize)]
        struct Test {
            name: String,
        }

        let json_str = r#"{
            "type": "FeatureCollection",
            "features": [{
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [100.0, 0.0]
            },
            "properties": {
                "name": "Tokyo"
            }
        }]
        }
        "#;
        let mut json: FeatureCollection<(), Test, MValue> =
            json_str.to_feature_collection().unwrap();
        assert_eq!(
            json,
            FeatureCollection {
                features: vec![Features::Feature(Feature {
                    _type: FeatureType::Feature,
                    id: None,
                    properties: Test { name: "Tokyo".into() },
                    geometry: Geometry::Point(BaseGeometry {
                        _type: GeometryType::Point,
                        coordinates: Point(100.0, 0.0),
                        ..Default::default()
                    }),
                    ..Default::default()
                })],
                _type: FeatureCollectionType::FeatureCollection,
                attributions: None,
                bbox: None,
            }
        );

        let mut collection = JSONCollectionReader::from(&mut json);
        assert_eq!(collection.features.len(), 1);

        let data: Vec<VectorFeature<(), Test, MValue>> = collection.iter().collect();

        assert_eq!(
            data,
            vec![VectorFeature {
                _type: VectorFeatureType::VectorFeature,
                id: None,
                face: 0.into(),
                properties: Test { name: "Tokyo".into() },
                geometry: VectorGeometry::Point(VectorBaseGeometry {
                    _type: VectorGeometryType::Point,
                    is_3d: false,
                    coordinates: VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                    bbox: Some(BBox3D {
                        left: 100.0,
                        bottom: 0.0,
                        right: 100.0,
                        top: 0.0,
                        near: 1.7976931348623157e308,
                        far: -1.7976931348623157e308
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }]
        );

        let data: Vec<VectorFeature<(), Test, MValue>> = collection.clone().into_iter().collect();

        assert_eq!(
            data,
            vec![VectorFeature {
                _type: VectorFeatureType::VectorFeature,
                id: None,
                face: 0.into(),
                properties: Test { name: "Tokyo".into() },
                geometry: VectorGeometry::Point(VectorBaseGeometry {
                    _type: VectorGeometryType::Point,
                    is_3d: false,
                    coordinates: VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                    bbox: Some(BBox3D {
                        left: 100.0,
                        bottom: 0.0,
                        right: 100.0,
                        top: 0.0,
                        near: 1.7976931348623157e308,
                        far: -1.7976931348623157e308
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }]
        );

        let data: Vec<VectorFeature<(), Test, MValue>> =
            collection.iter_mut().map(|f| f.clone()).collect();

        assert_eq!(
            data,
            vec![VectorFeature {
                _type: VectorFeatureType::VectorFeature,
                id: None,
                face: 0.into(),
                properties: Test { name: "Tokyo".into() },
                geometry: VectorGeometry::Point(VectorBaseGeometry {
                    _type: VectorGeometryType::Point,
                    is_3d: false,
                    coordinates: VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                    bbox: Some(BBox3D {
                        left: 100.0,
                        bottom: 0.0,
                        right: 100.0,
                        top: 0.0,
                        near: 1.7976931348623157e308,
                        far: -1.7976931348623157e308
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }]
        );

        let data: Vec<VectorFeature<(), Test, MValue>> = collection.iter().collect();

        assert_eq!(
            data,
            vec![VectorFeature {
                _type: VectorFeatureType::VectorFeature,
                id: None,
                face: 0.into(),
                properties: Test { name: "Tokyo".into() },
                geometry: VectorGeometry::Point(VectorBaseGeometry {
                    _type: VectorGeometryType::Point,
                    is_3d: false,
                    coordinates: VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                    bbox: Some(BBox3D {
                        left: 100.0,
                        bottom: 0.0,
                        right: 100.0,
                        top: 0.0,
                        near: 1.7976931348623157e308,
                        far: -1.7976931348623157e308
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }]
        );
    }

    #[test]
    fn test_json_vector_feature() {
        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Deserialize)]
        struct Test {
            name: String,
        }

        let json_str = r#"{
            "type": "VectorFeature",
            "face": 0,
            "properties": { "name": "Tokyo" },
            "geometry": {
                "type": "Point",
                "coordinates": { "x": 100.0, "y": 0.0 },
                "is3D": false
            }
        }"#;
        let mut json: VectorFeature<(), Test, MValue> = json_str.to_vector_feature().unwrap();
        assert_eq!(
            json,
            VectorFeature {
                _type: VectorFeatureType::VectorFeature,
                id: None,
                properties: Test { name: "Tokyo".into() },
                geometry: VectorGeometry::Point(VectorBaseGeometry {
                    _type: VectorGeometryType::Point,
                    coordinates: VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                    ..Default::default()
                }),
                ..Default::default()
            }
        );

        let collection = JSONCollectionReader::from(&mut json);
        assert_eq!(collection.features.len(), 1);
    }

    #[test]
    fn test_json_vector_feature_collection() {
        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Deserialize)]
        struct Test {
            name: String,
        }

        let json_str = r#"{
            "type": "FeatureCollection",
            "features": [{
            "type": "VectorFeature",
            "face": 0,
            "properties": { "name": "Tokyo" },
            "geometry": {
                "type": "Point",
                "coordinates": { "x": 100.0, "y": 0.0 },
                "is3D": false
            }
        }]
        }
        "#;
        let mut json: FeatureCollection<(), Test, MValue> =
            json_str.to_feature_collection().unwrap();
        assert_eq!(
            json,
            FeatureCollection {
                features: vec![Features::VectorFeature(VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    properties: Test { name: "Tokyo".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        coordinates: VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                        ..Default::default()
                    }),
                    ..Default::default()
                })],
                _type: FeatureCollectionType::FeatureCollection,
                attributions: None,
                bbox: None,
            }
        );

        let collection = JSONCollectionReader::from(&mut json);
        assert_eq!(collection.features.len(), 1);
    }

    #[test]
    fn test_json_s2_feature() {
        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Deserialize)]
        struct Test {
            name: String,
        }

        let json_str = r#"{
            "type": "S2Feature",
            "face": 1,
            "properties": { "name": "Tokyo" },
            "geometry": {
                "type": "Point",
                "coordinates": { "x": 100.0, "y": 0.0 },
                "is3D": false
            }
        }"#;
        let mut json: VectorFeature<(), Test, MValue> = json_str.to_vector_feature().unwrap();
        assert_eq!(
            json,
            VectorFeature {
                _type: VectorFeatureType::S2Feature,
                face: 1.into(),
                properties: Test { name: "Tokyo".into() },
                geometry: VectorGeometry::Point(VectorBaseGeometry {
                    _type: VectorGeometryType::Point,
                    coordinates: VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                    ..Default::default()
                }),
                ..Default::default()
            }
        );

        let collection = JSONCollectionReader::from(&mut json);
        assert_eq!(collection.features.len(), 1);
    }

    #[test]
    fn test_json_vector_s2_collection() {
        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Deserialize)]
        struct Test {
            name: String,
        }

        let json_str = r#"{
            "type": "S2FeatureCollection",
            "faces": [1],
            "features": [{
            "type": "S2Feature",
            "face": 1,
            "properties": { "name": "Tokyo" },
            "geometry": {
                "type": "Point",
                "coordinates": { "x": 100.0, "y": 0.0 },
                "is3D": false
            }
        }]
        }
        "#;
        let mut json: S2FeatureCollection<(), Test, MValue> =
            json_str.to_s2_feature_collection().unwrap();
        assert_eq!(
            json,
            S2FeatureCollection {
                features: vec![VectorFeature {
                    _type: VectorFeatureType::S2Feature,
                    face: 1.into(),
                    properties: Test { name: "Tokyo".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        coordinates: VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                        ..Default::default()
                    }),
                    ..Default::default()
                }],
                _type: S2FeatureCollectionType::S2FeatureCollection,
                attributions: None,
                bbox: None,
                faces: vec![1.into()],
            }
        );

        let collection = JSONCollectionReader::from(&mut json);
        assert_eq!(collection.features.len(), 1);
    }
}
