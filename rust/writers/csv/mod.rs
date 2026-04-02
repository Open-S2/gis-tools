use super::OnFeature;
use crate::{
    geometry::convert,
    parsers::{FeatureReader, Writer},
};
use alloc::{format, vec::Vec};
use s2json::{
    JSONCollection, MValue, MValueCompatible, Projection, ValueType, VectorFeature, VectorGeometry,
};
use serde::Serialize;

/// User defined options on how to store the features
pub struct ToCSVOptions<M: Clone, P: Clone + Default + Serialize, D: Clone + Default + Serialize> {
    /// The delimiter to use to separate lines [Default=',']
    pub delimiter: Option<String>,
    /// The lineDelimiter to use to separate lines [Default='\n']
    pub line_delimiter: Option<String>,
    /// If provided the lookup of the longitude [Default='lon']
    pub lon_key: Option<String>,
    /// If provided the lookup of the latitude [Default='lat']
    pub lat_key: Option<String>,
    /// If provided the lookup for the height value [Default=undefined]
    pub height_key: Option<String>,
    /// List of parameters to include in the feature
    pub properties: Option<Vec<String>>,
    /// User defined function on how to store the feature
    pub on_feature: Option<OnFeature<M, P, D>>,
}
impl<M: Clone, P: Clone + Default + Serialize, D: Clone + Default + Serialize> Default
    for ToCSVOptions<M, P, D>
{
    fn default() -> Self {
        ToCSVOptions {
            delimiter: None,
            line_delimiter: None,
            lon_key: None,
            lat_key: None,
            height_key: None,
            properties: None,
            on_feature: None,
        }
    }
}

/// # To CSV
///
/// ## Description
/// Given a writer and an array of readers, write the input features to the writer as CSV data
///
/// ## Parameters
/// - `writer`: the buffer or file to write to. See [`Writer`] for what writers are available
/// - `readers`: Any reader that implements the [`FeatureReader`] trait can be used
/// - `opts`: user defined options on how to write the features
///
/// ## Usage
/// ```rust
/// use gistools::{readers::JSONReader, parsers::{BufferWriter, FileReader}, writers::{to_csv, ToCSVOptions}};
/// use s2json::{MValueCompatible, Projection};
/// use serde::{Deserialize, Serialize};
/// use std::path::PathBuf;
///
/// #[derive(Debug, Default, Clone, MValueCompatible, PartialEq, Serialize, Deserialize)]
/// #[serde(default)]
/// struct Props {
///     name: String,
/// }
///
/// let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
/// path = path.join("tests/writers/fixtures/points.geojson");
///
/// let reader: JSONReader<FileReader, (), Props, ()> = JSONReader::new(FileReader::from(path));
/// let mut writer = BufferWriter::default();
///
/// // write
/// to_csv(
///     &mut writer,
///     vec![&reader],
///     Some(ToCSVOptions {
///         properties: Some(vec!["name".into()]),
///         ..Default::default()
///     }),
/// );
/// ```
pub fn to_csv<
    T: Writer,
    M: Clone + Serialize,
    P: Clone + Default + Serialize + MValueCompatible,
    D: Clone + Default + Serialize,
    I: FeatureReader<M, P, D>,
>(
    writer: &mut T,
    readers: Vec<&I>,
    opts: Option<ToCSVOptions<M, P, D>>,
) {
    let opts = opts.unwrap_or_default();
    let on_feature = opts.on_feature.unwrap_or(Some);
    let delimiter = opts.delimiter.as_deref().unwrap_or(",");
    let line_delimiter = opts.line_delimiter.as_deref().unwrap_or("\n");
    let lon_key = opts.lon_key.as_deref().unwrap_or("lon");
    let lat_key = opts.lat_key.as_deref().unwrap_or("lat");
    let height_key = opts.height_key.as_deref();
    let props = opts.properties.unwrap_or_default();

    // setup the CSV first descripter line
    let mut start_str = format!("{lon_key}{delimiter}{lat_key}");
    if let Some(height_key) = height_key {
        start_str = format!("{start_str}{delimiter}{height_key}");
    }
    for property in &props {
        start_str = format!("{start_str}{delimiter}{property}");
    }
    writer.append_string(&format!("{start_str}{line_delimiter}"));

    for reader in readers {
        for feature in reader.iter() {
            let converted_features =
                convert(Projection::WG, &JSONCollection::VectorFeature(feature), Some(false), None);
            for converted_feature in converted_features {
                let Some(user_feature) = on_feature(converted_feature) else {
                    continue;
                };
                let VectorFeature { geometry, properties, .. } = user_feature;
                let points = match geometry {
                    VectorGeometry::Point(g) => vec![g.coordinates],
                    VectorGeometry::MultiPoint(g) => g.coordinates,
                    _ => vec![],
                };
                let m_value: MValue = properties.into();
                for point in points {
                    // write each point to CSV file
                    let mut output_string = format!("{}{delimiter}{}", point.x, point.y);
                    if height_key.is_some() {
                        output_string = format!(
                            "{output_string}{delimiter}{}",
                            point.z.map(|z| z.to_string()).unwrap_or_default()
                        );
                    }
                    for prop in &props {
                        // grab the property and store
                        let value = m_value
                            .get(prop)
                            .map(|v| {
                                if let ValueType::Primitive(p) = v {
                                    p.to_string().unwrap_or_default()
                                } else {
                                    "".to_string()
                                }
                            })
                            .unwrap_or_default();
                        output_string = format!("{output_string}{delimiter}{}", value.as_str());
                    }
                    writer.append_string(&format!("{output_string}{line_delimiter}"));
                }
            }
        }
    }
}
