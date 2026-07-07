use crate::{
    geometry::convert,
    parsers::{FeatureReader, Writer},
    writers::{OnFeature, to_dbf},
};
use alloc::slice;
use s2json::{
    BBox3D, JSONCollection, MValue, MValueCompatible, Projection, VectorFeature, VectorGeometry,
    VectorGeometryType, VectorLineString, VectorMultiPoint, VectorPoint, VectorPolygonGeometry,
};
use serde::Serialize;

static PRJ: &str = "GEOGCS[\"WGS 84\",
  DATUM[\"WGS_1984\",
    SPHEROID[\"WGS 84\",6378137,298.257223563,AUTHORITY[\"EPSG\",\"7030\"]],
    AUTHORITY[\"EPSG\",\"6326\"]],
  PRIMEM[\"Greenwich\",0,AUTHORITY[\"EPSG\",\"8901\"]],
  UNIT[\"degree\",0.0174532925199433,AUTHORITY[\"EPSG\",\"9122\"]],
  AXIS[\"Latitude\",NORTH],
  AXIS[\"Longitude\",EAST],
  AUTHORITY[\"EPSG\",\"4326\"]]
";

/// Shapefile Shape Type
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(missing_docs)]
pub enum SHPShapeType {
    #[default]
    NULL = 0,
    POINT = 1,
    POLYLINE = 3,
    POLYGON = 5,
    MULTIPOINT = 8,
    POINTZ = 11,
    POLYLINEZ = 13,
    POLYGONZ = 15,
    MULTIPOINTZ = 18,
    POINTM = 21,
    POLYLINEM = 23,
    POLYGONM = 25,
    MULTIPOINTM = 28,
}
impl SHPShapeType {
    /// Create a Shapefile Shape Type given a GeoJSON Geometry
    pub fn from_geo<M: MValueCompatible>(geo: &VectorGeometry<M>, has_m: bool) -> Self {
        match geo {
            VectorGeometry::Point(p) => {
                if has_m {
                    SHPShapeType::POINTM
                } else if p.is_3d {
                    SHPShapeType::POINTZ
                } else {
                    SHPShapeType::POINT
                }
            }
            VectorGeometry::MultiPoint(m) => {
                if has_m {
                    SHPShapeType::MULTIPOINTM
                } else if m.is_3d {
                    SHPShapeType::MULTIPOINTZ
                } else {
                    SHPShapeType::MULTIPOINT
                }
            }
            VectorGeometry::LineString(l) => {
                if has_m {
                    SHPShapeType::POLYLINEM
                } else if l.is_3d {
                    SHPShapeType::POLYLINEZ
                } else {
                    SHPShapeType::POLYLINE
                }
            }
            VectorGeometry::MultiLineString(l) => {
                if has_m {
                    SHPShapeType::POLYLINEM
                } else if l.is_3d {
                    SHPShapeType::POLYLINEZ
                } else {
                    SHPShapeType::POLYLINE
                }
            }
            VectorGeometry::Polygon(p) => {
                if has_m {
                    SHPShapeType::POLYGONM
                } else if p.is_3d {
                    SHPShapeType::POLYGONZ
                } else {
                    SHPShapeType::POLYGON
                }
            }
            VectorGeometry::MultiPolygon(p) => {
                if has_m {
                    SHPShapeType::POLYGONM
                } else if p.is_3d {
                    SHPShapeType::POLYGONZ
                } else {
                    SHPShapeType::POLYGON
                }
            }
        }
    }
}

/// # Shapefile Writer
///
/// ## Description
///
/// Given a writer and an array of iterators, write the input features property data into a SHP file
///
/// NOTE: The correct way to store geometry in a shapefile is to only store one kind of geometry.
/// However, this libraries writer and reader do not enforce this.
///
/// ## Usage
///
/// #### Write to files
///
/// ```rust,ignore
/// use gistools::{readers::JSONReader, parsers::{FileWriter, FileReader}, writers::to_shp};
/// use s2json::{MValue, MValueCompatible};
/// use serde::{Deserialize, Serialize};
/// use std::path::PathBuf;
///
/// // read in data
/// #[derive(Debug, Default, Clone, MValueCompatible, PartialEq, Serialize, Deserialize)]
/// #[serde(default)]
/// struct Props {
///     name: String,
/// }
/// let cargo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
/// let path = cargo_path.join("tests/writers/fixtures/points.geojson");
/// let reader: JSONReader<FileReader, (), Props, MValue> =
///     JSONReader::new(FileReader::from(path));
///
/// // setup writers
/// let shp_path = cargo_path.join("tests/writers/fixtures/points.shp");
/// let mut shp_writer = FileWriter::new(shp_path).unwrap();
/// let dbf_path = cargo_path.join("tests/writers/fixtures/points.dbf");
/// let mut dbf_writer = FileWriter::new(dbf_path).unwrap();
/// let shx_path = cargo_path.join("tests/writers/fixtures/points.shx");
/// let mut shx_writer = FileWriter::new(shx_path).unwrap();
/// let prj_path = cargo_path.join("tests/writers/fixtures/points.prj");
/// let mut prj_writer = FileWriter::new(prj_path).unwrap();

/// // write to files
/// to_shp(
///     &mut shp_writer,
///     vec![&reader],
///     Some(&mut dbf_writer),
///     Some(&mut shx_writer),
///     Some(&mut prj_writer),
///     None,
///     None,
/// );
/// ```
///
/// #### Zip the files
///
/// ```rust,ignore
/// use gistools::util::{zip_folder, WriteZipItem};
/// use std::{path::PathBuf, fs::read};
///
/// let cargo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
/// let shp_path = cargo_path.join("tests/writers/fixtures/points.shp");
/// let dbf_path = cargo_path.join("tests/writers/fixtures/points.dbf");
/// let shx_path = cargo_path.join("tests/writers/fixtures/points.shx");
/// let prj_path = cargo_path.join("tests/writers/fixtures/points.prj");
/// let shp_file = read(shp_path).unwrap();
/// let dbf_file = read(dbf_path).unwrap();
/// let shx_file = read(shx_path).unwrap();
/// let prj_file = read(prj_path).unwrap();
///
/// let zipped_data = zip_folder(vec![
///    WriteZipItem {
///        filename: "points.shp".into(),
///        comment: Some("shapefile data".into()),
///        bytes: shp_file,
///    },
///    WriteZipItem {
///        filename: "points.dbf".into(),
///        comment: Some("properties data".into()),
///        bytes: dbf_file,
///    },
///    WriteZipItem {
///        filename: "points.shx".into(),
///        comment: Some("index data".into()),
///        bytes: shx_file,
///    },
///    WriteZipItem {
///        filename: "points.prj".into(),
///        comment: Some("projection".into()),
///        bytes: prj_file,
///    },
/// ])
/// .unwrap();
/// ```
///
/// ## Links
/// - <https://www.esri.com/content/dam/esrisites/sitecore-archive/Files/Pdfs/library/whitepapers/pdfs/shapefile.pdf>
///
/// ## Parameters
/// - `shp_writer`: the shapefile data container to write to
/// - `iterators`: the collection of iterators to write
/// - `dbf_writer`: the dbf data container to write to if provided (properties data)
/// - `shx_writer`: the shx data container to write to if provided (index data)
/// - `prj_writer`: the prj data container to write to if provided (projection)
/// - `on_feature`: A fn that takes a feature and returns a feature
/// - `m_value`: If the data has the measurement modifier to the z value and how to find it
pub fn to_shp<
    S: Writer,
    F: Writer,
    X: Writer,
    J: Writer,
    M: Clone + Serialize,
    P: MValueCompatible,
    D: MValueCompatible,
    I: FeatureReader<M, P, D>,
>(
    shp_writer: &mut S,
    iterators: Vec<&I>,
    dbf_writer: Option<&mut F>,
    shx_writer: Option<&mut X>,
    prj_writer: Option<&mut J>,
    on_feature: Option<OnFeature<M, P, D>>,
    m_value: Option<fn(m: Option<MValue>) -> Option<f64>>,
) {
    // write to shp and shx
    write_shp(shp_writer, &iterators, shx_writer, on_feature, m_value);
    // write to dbf
    if let Some(dbf_writer) = dbf_writer {
        to_dbf(dbf_writer, iterators);
    }
    // write to prj
    if let Some(prj_writer) = prj_writer {
        prj_writer.append_string(PRJ);
    }
}

fn write_shp<
    S: Writer,
    X: Writer,
    M: Clone + Serialize,
    P: MValueCompatible,
    D: MValueCompatible,
    I: FeatureReader<M, P, D>,
>(
    shp_writer: &mut S,
    iterators: &[&I],
    mut shx_writer: Option<&mut X>,
    on_feature: Option<OnFeature<M, P, D>>,
    m_value: Option<fn(m: Option<MValue>) -> Option<f64>>,
) {
    let on_feature = on_feature.unwrap_or(Some);
    let mut global_bbox = BBox3D::new(
        f64::INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NEG_INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
    );
    let mut index = 0;
    let mut determined_type: Option<SHPShapeType> = None;

    shp_writer.append(&[0; 100]);
    if let Some(shx_writer) = shx_writer.as_mut() {
        shx_writer.append(&[0; 100]);
    }

    for iterator in iterators {
        for feature in iterator.iter() {
            let converted_features =
                convert(Projection::WG, &JSONCollection::VectorFeature(feature), Some(true), None);
            for mut converted_feature in converted_features {
                global_bbox.merge_in_place(&converted_feature.geometry.bbox());
                let Some(user_feature) = on_feature(converted_feature) else {
                    continue;
                };
                if determined_type.is_none() {
                    determined_type =
                        Some(SHPShapeType::from_geo(&user_feature.geometry, m_value.is_some()));
                }
                match &user_feature.geometry {
                    VectorGeometry::MultiPolygon(mp) => {
                        let polygons = &mp.coordinates;
                        for polygon in polygons.iter() {
                            let poly_feature: VectorFeature<M, P, D> = VectorFeature {
                                _type: user_feature._type.clone(),
                                properties: user_feature.properties.clone(),
                                geometry: VectorGeometry::Polygon(VectorPolygonGeometry::<D> {
                                    _type: VectorGeometryType::Polygon,
                                    bbox: Some(BBox3D::from_polygon(polygon)),
                                    coordinates: polygon.clone(),
                                    is_3d: mp.is_3d,
                                    ..Default::default()
                                }),
                                ..Default::default()
                            };
                            let shp_offset = shp_writer.tell() as i32;
                            write_feature(
                                &poly_feature,
                                shp_writer,
                                index,
                                shp_offset,
                                shx_writer.as_deref_mut(),
                                m_value,
                            );
                            index += 1;
                        }
                    }
                    _ => {
                        let shp_offset = shp_writer.tell() as i32;
                        write_feature(
                            &user_feature,
                            shp_writer,
                            index,
                            shp_offset,
                            shx_writer.as_deref_mut(),
                            m_value,
                        );
                        index += 1;
                    }
                }
            }
        }
    }

    // lastly store the file header
    let r#type = determined_type.unwrap_or(SHPShapeType::NULL) as i32;
    write_file_header(shp_writer, r#type, global_bbox);
    if let Some(shx_writer) = shx_writer {
        write_file_header(shx_writer, r#type, global_bbox);
    }
}

fn write_file_header<W: Writer>(writer: &mut W, r#type: i32, bbox: BBox3D) {
    let mut header = vec![0u8; 100];
    // File Code
    header[0..4].copy_from_slice(&(9994_i32).to_be_bytes());
    // File Length
    header[24..28].copy_from_slice(&(writer.tell() as i32).to_be_bytes());
    // version
    header[28..32].copy_from_slice(&(1000_i32).to_le_bytes());
    // Shape Type
    header[32..36].copy_from_slice(&r#type.to_le_bytes());
    // bbox
    header[36..44].copy_from_slice(&bbox.left.to_le_bytes());
    header[44..52].copy_from_slice(&bbox.bottom.to_le_bytes());
    header[52..60].copy_from_slice(&bbox.right.to_le_bytes());
    header[60..68].copy_from_slice(&bbox.top.to_le_bytes());
    header[68..76].copy_from_slice(&bbox.near.to_le_bytes());
    header[76..84].copy_from_slice(&bbox.far.to_le_bytes());
    //   // TODO:
    //   // Byte 84* Bounding Box Mmin Double Little
    //   // Byte 92* Bounding Box Mmax Double Little

    writer.write(&header, 0);
}

fn write_feature<
    S: Writer,
    X: Writer,
    M: Clone + Serialize,
    P: MValueCompatible,
    D: MValueCompatible,
>(
    feature: &VectorFeature<M, P, D>,
    shp_writer: &mut S,
    index: i32,
    shp_offset: i32,
    shx_writer: Option<&mut X>,
    m_value: Option<fn(m: Option<MValue>) -> Option<f64>>,
) {
    let has_m = m_value.is_some();

    let data = match &feature.geometry {
        VectorGeometry::Point(p) => {
            if p.is_3d {
                write_point_z(&p.coordinates, m_value)
            } else if has_m {
                write_point_m(&p.coordinates, m_value.unwrap())
            } else {
                write_point(&p.coordinates)
            }
        }
        VectorGeometry::MultiPoint(mp) => {
            if mp.is_3d {
                write_multi_point_z(&mp.coordinates, mp.bbox.unwrap_or_default(), m_value)
            } else if has_m {
                write_multi_point_m(&mp.coordinates, mp.bbox.unwrap_or_default(), m_value.unwrap())
            } else {
                write_multi_point(&mp.coordinates, mp.bbox.unwrap_or_default())
            }
        }
        VectorGeometry::LineString(l) => {
            if l.is_3d {
                write_line_strings_z(
                    slice::from_ref(&l.coordinates),
                    l.bbox.unwrap_or_default(),
                    m_value,
                    None,
                )
            } else if has_m {
                write_line_strings_m(
                    slice::from_ref(&l.coordinates),
                    l.bbox.unwrap_or_default(),
                    m_value.unwrap(),
                    None,
                )
            } else {
                write_line_strings(
                    slice::from_ref(&l.coordinates),
                    l.bbox.unwrap_or_default(),
                    None,
                )
            }
        }
        VectorGeometry::MultiLineString(ml) => {
            if ml.is_3d {
                write_line_strings_z(&ml.coordinates, ml.bbox.unwrap_or_default(), m_value, None)
            } else if has_m {
                write_line_strings_m(
                    &ml.coordinates,
                    ml.bbox.unwrap_or_default(),
                    m_value.unwrap(),
                    None,
                )
            } else {
                write_line_strings(&ml.coordinates, ml.bbox.unwrap_or_default(), None)
            }
        }
        VectorGeometry::Polygon(p) => {
            if p.is_3d {
                write_line_strings_z(
                    &p.coordinates,
                    p.bbox.unwrap_or_default(),
                    m_value,
                    Some(SHPShapeType::POLYGONZ),
                )
            } else if has_m {
                write_line_strings_m(
                    &p.coordinates,
                    p.bbox.unwrap_or_default(),
                    m_value.unwrap(),
                    Some(SHPShapeType::POLYGONM),
                )
            } else {
                write_line_strings(
                    &p.coordinates,
                    p.bbox.unwrap_or_default(),
                    Some(SHPShapeType::POLYGON),
                )
            }
        }
        _ => write_null(),
    };

    write_record_header(shp_writer, index, data.len() as i32);
    shp_writer.append(&data);
    if let Some(shx_writer) = shx_writer {
        write_index_record(shx_writer, shp_offset, data.len() as i32);
    }
}

fn write_index_record<W: Writer>(shx_writer: &mut W, shp_offset: i32, content_length: i32) {
    let mut header = vec![0; 8];
    header[0..4].copy_from_slice(&(shp_offset >> 1).to_be_bytes());
    header[4..8].copy_from_slice(&(content_length >> 1).to_be_bytes());
    shx_writer.append(&header);
}

fn write_record_header<W: Writer>(shp_writer: &mut W, record_num: i32, content_length: i32) {
    let mut header = vec![0; 8];
    header[0..4].copy_from_slice(&(record_num).to_be_bytes());
    header[4..8].copy_from_slice(&(content_length >> 1).to_be_bytes());
    shp_writer.append(&header);
}

// Byte 0 Shape Type 0 Integer 1 Little
fn write_null() -> Vec<u8> {
    let mut view = vec![0; 20];
    view[0..4].copy_from_slice(&(SHPShapeType::NULL as i32).to_le_bytes()); // NULL
    view
}

// Byte 0 Shape Type 1 Integer 1 Little
// Byte 4 X X Double 1 Little
// Byte 12 Y Y Double 1 Little
fn write_point<M: MValueCompatible>(point: &VectorPoint<M>) -> Vec<u8> {
    let mut view = vec![0; 20];

    view[0..4].copy_from_slice(&(SHPShapeType::POINT as i32).to_le_bytes()); // Type 1 (POINT)
    view[4..12].copy_from_slice(&point.x.to_le_bytes()); // X
    view[12..20].copy_from_slice(&point.y.to_le_bytes()); // Y

    view
}

// Byte 0 Shape Type 8 Integer 1 Little
// Byte 4 Box Box Double 4 Little
// Byte 36 NumPoints NumPoints Integer 1 Little
// Byte 40 Points Points Point NumPoints Little
fn write_multi_point<M: MValueCompatible>(points: &VectorMultiPoint<M>, bbox: BBox3D) -> Vec<u8> {
    let num_points = points.len();

    // Header is 40 bytes. Each XY pair is exactly 16 bytes (2 * 8-byte Float64)
    let buffer_size = 40 + num_points * 16;
    let mut view = vec![0; buffer_size];
    // 1. Write Header Metadata
    view[0..4].copy_from_slice(&(SHPShapeType::MULTIPOINT as i32).to_le_bytes()); // Type 8 (MULTIPOINT)
    view[4..12].copy_from_slice(&bbox.left.to_le_bytes()); // xmin
    view[12..20].copy_from_slice(&bbox.bottom.to_le_bytes()); // ymin
    view[20..28].copy_from_slice(&bbox.right.to_le_bytes()); // xmax
    view[28..36].copy_from_slice(&bbox.top.to_le_bytes()); // ymax
    view[36..40].copy_from_slice(&(num_points as i32).to_le_bytes()); // NumPoints
    // 2. Stream XY Points sequentially
    let mut offset = 40;
    for point in points.iter() {
        view[offset..(offset + 8)].copy_from_slice(&point.x.to_le_bytes());
        view[(offset + 8)..(offset + 16)].copy_from_slice(&point.y.to_le_bytes());
        offset += 16; // Step forward by exactly 16 bytes per point pair
    }

    view
}

// Byte 0 Shape Type 3 Integer 1 Little
// Byte 4 Box Box Double 4 Little
// Byte 36 NumParts NumParts Integer 1 Little
// Byte 40 NumPoints NumPoints Integer 1 Little
// Byte 44 Parts Parts Integer NumParts Little
// Byte X Points Points Point NumPoints Little
// Note: X = 44 + 4 * NumParts
fn write_line_strings<M: MValueCompatible>(
    lines: &[VectorLineString<M>],
    bbox: BBox3D,
    r#type: Option<SHPShapeType>,
) -> Vec<u8> {
    let r#type = r#type.unwrap_or(SHPShapeType::POLYLINE);
    let total_lines = lines.iter().fold(0, |acc, line| acc + line.len());
    let total_parts = lines.len();

    // Header Metadata is exactly 44 bytes (Offsets 0 - 43)
    let buffer_size = 44 + total_parts * 4 + total_lines * 16;
    let mut view = vec![0; buffer_size];

    // 1. Write Header Metadata
    view[0..4].copy_from_slice(&(r#type as i32).to_le_bytes()); // type
    view[4..12].copy_from_slice(&bbox.left.to_le_bytes()); // xmin
    view[12..20].copy_from_slice(&bbox.bottom.to_le_bytes()); // ymin
    view[20..28].copy_from_slice(&bbox.right.to_le_bytes()); // xmax
    view[28..36].copy_from_slice(&bbox.top.to_le_bytes()); // ymax
    view[36..40].copy_from_slice(&(total_parts as i32).to_le_bytes()); // total_parts
    view[40..44].copy_from_slice(&(total_lines as i32).to_le_bytes()); // total_lines
    // 2. Write Parts Index Array (Cumulative 0-based index offsets)
    let mut offset = 44;
    let mut part_index_accumulator = 0;
    for point in lines.iter() {
        view[offset..(offset + 4)].copy_from_slice(&(part_index_accumulator as i32).to_le_bytes());
        offset += 4;
        part_index_accumulator += point.len();
    }
    // 3. Stream all X, Y coordinates sequentially across all parts
    for line in lines.iter() {
        for point in line.iter() {
            view[offset..(offset + 8)].copy_from_slice(&point.x.to_le_bytes());
            view[(offset + 8)..(offset + 16)].copy_from_slice(&point.y.to_le_bytes());
            offset += 16; // Step forward by exactly 16 bytes per point pair
        }
    }

    view
}

// Byte 0 Shape Type 21 Integer 1 Little
// Byte 4 X X Double 1 Little
// Byte 12 Y Y Double 1 Little
// Byte 20 M M Double 1 Little
fn write_point_m<M: MValueCompatible>(
    point: &VectorPoint<M>,
    m_value: fn(m: Option<MValue>) -> Option<f64>,
) -> Vec<u8> {
    let mut view = vec![0; 28];

    view[0..4].copy_from_slice(&(SHPShapeType::POINTM as i32).to_le_bytes()); // Type 21 (POINTM)
    view[4..12].copy_from_slice(&point.x.to_le_bytes()); // X
    view[12..20].copy_from_slice(&point.y.to_le_bytes()); // Y
    // M
    let m = m_value(point.m.as_ref().map(|m| m.clone().into())).unwrap_or(f64::MIN).to_le_bytes();
    view[20..28].copy_from_slice(&m);

    view
}

// Byte 0 Shape Type 28 Integer 1 Little
// Byte 4 Box Box Double 4 Little
// Byte 36 NumPoints NumPoints Integer 1 Little
// Byte 40 Points Points Point NumPoints Little
// Byte X* Mmin Mmin Double 1 Little
// Byte X+8* Mmax Mmax Double 1 Little
// Byte X+16* Marray Marray Double NumPoints Little
// Note: X = 40 + (16 * NumPoints)
// * optional
fn write_multi_point_m<M: MValueCompatible>(
    points: &VectorMultiPoint<M>,
    bbox: BBox3D,
    m_value: fn(m: Option<MValue>) -> Option<f64>,
) -> Vec<u8> {
    let num_points = points.len();
    let m_no_data = f64::MIN;

    // Calculate explicit spec-compliant buffer size:
    // Header(40) + XY Array(num_points * 16) + M Range(16) + M Array(num_points * 8)
    let buffer_size = 40 + num_points * 16 + 16 + num_points * 8;
    let mut view = vec![0; buffer_size];

    // 1. Write Header Metadata
    view[0..4].copy_from_slice(&(SHPShapeType::MULTIPOINTM as i32).to_le_bytes()); // Type 28 (MULTIPOINTM)
    view[4..12].copy_from_slice(&bbox.left.to_le_bytes()); // xmin
    view[12..20].copy_from_slice(&bbox.bottom.to_le_bytes()); // ymin
    view[20..28].copy_from_slice(&bbox.right.to_le_bytes()); // xmax
    view[28..36].copy_from_slice(&bbox.top.to_le_bytes()); // ymax
    view[36..40].copy_from_slice(&(num_points as i32).to_le_bytes()); // NumPoints

    // 2. Phase A: Stream all X, Y coordinates sequentially
    let mut offset = 40;
    for point in points.iter() {
        view[offset..(offset + 8)].copy_from_slice(&point.x.to_le_bytes());
        view[(offset + 8)..(offset + 16)].copy_from_slice(&point.y.to_le_bytes());
        offset += 16;
    }

    // 3. Phase B: Calculate M boundaries and stream the standalone M block
    let mut min_m = f64::INFINITY;
    let mut max_m = f64::NEG_INFINITY;
    let m_range_start_offset = offset;
    offset += 16; // Skip past the 16-byte M-range placeholder for now

    for point in points.iter() {
        let m = m_value(point.m.as_ref().map(|m| m.clone().into())).unwrap_or(m_no_data);
        if m != m_no_data {
            min_m = f64::min(min_m, m);
            max_m = f64::max(max_m, m);
        }
        view[offset..(offset + 8)].copy_from_slice(&m.to_le_bytes());
        offset += 8;
    }

    // Fallback if no valid measures were found
    if min_m == f64::INFINITY {
        min_m = 0.0;
        max_m = 0.0;
    }

    // Go back and populate the M Range bounding box right before the M array
    view[m_range_start_offset..(m_range_start_offset + 8)].copy_from_slice(&min_m.to_le_bytes());
    view[(m_range_start_offset + 8)..(m_range_start_offset + 16)]
        .copy_from_slice(&max_m.to_le_bytes());

    view
}

// Byte 0 Shape Type 23 Integer 1 Little
// Byte 4 Box Box Double 4 Little
// Byte 36 NumParts NumParts Integer 1 Little
// Byte 40 NumPoints NumPoints Integer 1 Little
// Byte 44 Parts Parts Integer NumParts Little
// Byte X Points Points Point NumPoints Little
// Byte Y* Mmin Mmin Double 1 Little
// Byte Y + 8* Mmax Mmax Double 1 Little
// Byte Y + 16* Marray Marray Double NumPoints Little
// Note: X = 44 + (4 * NumParts), Y = X + (16 * NumPoints)
// * optional
fn write_line_strings_m<M: MValueCompatible>(
    lines: &[VectorLineString<M>],
    bbox: BBox3D,
    m_value: fn(m: Option<MValue>) -> Option<f64>,
    r#type: Option<SHPShapeType>,
) -> Vec<u8> {
    let r#type = r#type.unwrap_or(SHPShapeType::POLYLINEM);
    let total_points = lines.iter().fold(0, |acc, line| acc + line.len());
    let total_parts = lines.len();
    let m_no_data = f64::MIN;

    // Calculate explicit buffer size:
    // Header(44) + Parts Array(total_parts * 4) + XY Array(total_points * 16) + M Range(16) + M Array(total_points * 8)
    let buffer_size = 44 + total_parts * 4 + total_points * 16 + 16 + total_points * 8;
    let mut view = vec![0; buffer_size];

    // 1. Write Header Metadata
    view[0..4].copy_from_slice(&(r#type as i32).to_le_bytes()); // TYPE
    view[4..12].copy_from_slice(&bbox.left.to_le_bytes()); // xmin
    view[12..20].copy_from_slice(&bbox.bottom.to_le_bytes()); // ymin
    view[20..28].copy_from_slice(&bbox.right.to_le_bytes()); // xmax
    view[28..36].copy_from_slice(&bbox.top.to_le_bytes()); // ymax
    view[36..40].copy_from_slice(&(total_parts as i32).to_le_bytes());
    view[40..44].copy_from_slice(&(total_points as i32).to_le_bytes());

    // 2. Write Parts Index Array (Cumulative offsets, not part lengths!)
    let mut offset = 44;
    let mut part_index_accumulator = 0;
    for line in lines.iter() {
        view[offset..(offset + 4)].copy_from_slice(&(part_index_accumulator as i32).to_le_bytes());
        offset += 4;
        part_index_accumulator += line.len();
    }

    // 3. Stream all X, Y coordinates sequentially
    for line in lines.iter() {
        for point in line.iter() {
            view[offset..(offset + 8)].copy_from_slice(&point.x.to_le_bytes());
            view[(offset + 8)..(offset + 16)].copy_from_slice(&point.y.to_le_bytes());
            offset += 16;
        }
    }

    // 4. Calculate M boundaries and stream the standalone M block
    let mut min_m = f64::INFINITY;
    let mut max_m = f64::NEG_INFINITY;
    let m_range_start_offset = offset;
    offset += 16; // Skip past the 16-byte M-range placeholder for now

    for line in lines.iter() {
        for point in line.iter() {
            let m = m_value(point.m.as_ref().map(|m| m.clone().into())).unwrap_or(m_no_data);

            if m != m_no_data {
                if m < min_m {
                    min_m = m;
                }
                if m > max_m {
                    max_m = m;
                }
            }

            view[offset..(offset + 8)].copy_from_slice(&m.to_le_bytes());
            offset += 8;
        }
    }

    // Clean up bounds if no valid measures were encountered
    if min_m == f64::INFINITY {
        min_m = 0.0;
        max_m = 0.0;
    }

    // Go back and populate the M Range bounding box right before the M array
    view[m_range_start_offset..(m_range_start_offset + 8)].copy_from_slice(&min_m.to_le_bytes());
    view[m_range_start_offset + 8..(m_range_start_offset + 16)]
        .copy_from_slice(&max_m.to_le_bytes());

    view
}

// Byte 0 Shape Type 11 Integer 1 Little
// Byte 4 X X Double 1 Little
// Byte 12 Y Y Double 1 Little
// Byte 20 Z Z Double 1 Little
// Byte 28 Measure M Double 1 Little
// Byte 0 Shape Type 11 Integer 1 Little
// Byte 4 X X Double 1 Little
// Byte 12 Y Y Double 1 Little
// Byte 20 Z Z Double 1 Little
// Byte 28* M M Double 1 Little
// * optional
fn write_point_z<M: MValueCompatible>(
    point: &VectorPoint<M>,
    m_value: Option<fn(m: Option<MValue>) -> Option<f64>>,
) -> Vec<u8> {
    let has_m = m_value.is_some();
    let m_no_data = f64::MIN;

    // Sized strictly: 28 bytes for X/Y/Z, 36 bytes if it includes M
    let buffer_size = if has_m { 36 } else { 28 };
    let mut view = vec![0; buffer_size];
    view[0..4].copy_from_slice(&(SHPShapeType::POINTZ as i32).to_le_bytes()); // Type 11 (POINTZ)
    view[4..12].copy_from_slice(&point.x.to_le_bytes()); // X
    view[12..20].copy_from_slice(&point.y.to_le_bytes()); // Y
    view[20..28].copy_from_slice(&point.z.unwrap_or(0.0).to_le_bytes()); // Z

    if let Some(m_value) = m_value {
        view[28..36].copy_from_slice(
            &m_value(point.m.as_ref().map(|m| m.clone().into())).unwrap_or(m_no_data).to_le_bytes(),
        );
    }

    view
}

// Byte 0 Shape Type 18 Integer 1 Little
// Byte 4 Box Box Double 4 Little
// Byte 36 NumPoints NumPoints Integer 1 Little
// Byte 40 Points Points Point NumPoints Little
// Byte X Zmin Zmin Double 1 Little
// Byte X+8 Zmax Zmax Double 1 Little
// Byte X+16 Zarray Zarray Double NumPoints Little
// Byte Y* Mmin Mmin Double 1 Little
// Byte Y+8* Mmax Mmax Double 1 Little
// Byte Y+16* Marray Marray Double NumPoints Little
// Note: X = 40 + (16 * NumPoints); Y = X + 16 + (8 * NumPoints)
// * optional
fn write_multi_point_z<M: MValueCompatible>(
    points: &VectorMultiPoint<M>,
    bbox: BBox3D,
    m_value: Option<fn(m: Option<MValue>) -> Option<f64>>,
) -> Vec<u8> {
    let num_points = points.len();
    let has_m = m_value.is_some();
    let m_no_data = f64::MIN;

    // 1. Calculate dynamic buffer allocation
    let mut buffer_size = 40 + num_points * 16; // Header (40) + XY Array (N * 16)
    buffer_size += 16 + num_points * 8; // Z Range (16) + Z Array (N * 8)
    if has_m {
        buffer_size += 16 + num_points * 8; // M Range (16) + M Array (N * 8)
    }

    let mut view = vec![0; buffer_size];

    // 2. Write Main Metadata Header
    view[0..4].copy_from_slice(&(SHPShapeType::MULTIPOINTZ as i32).to_le_bytes()); // Type 18 (MULTIPOINTZ)
    view[4..12].copy_from_slice(&bbox.left.to_le_bytes()); // xmin
    view[12..20].copy_from_slice(&bbox.bottom.to_le_bytes()); // ymin
    view[20..28].copy_from_slice(&bbox.right.to_le_bytes()); // xmax
    view[28..36].copy_from_slice(&bbox.top.to_le_bytes()); // ymax
    view[36..40].copy_from_slice(&(num_points as i32).to_le_bytes()); // NumPoints

    // 3. Phase A: Stream all X, Y coordinates sequentially
    let mut offset = 40;
    for point in points {
        view[offset..(offset + 8)].copy_from_slice(&point.x.to_le_bytes());
        view[(offset + 8)..(offset + 16)].copy_from_slice(&point.y.to_le_bytes());
        offset += 16;
    }

    // 4. Phase B: Calculate Z boundaries and stream the standalone Z block
    let mut min_z = f64::INFINITY;
    let mut max_z = f64::NEG_INFINITY;
    let z_range_offset = offset;
    offset += 16; // Skip past the 16-byte Z-range placeholder for now

    for point in points {
        let z = point.z.unwrap_or_default();
        if z < min_z {
            min_z = z;
        }
        if z > max_z {
            max_z = z;
        }
        view[offset..(offset + 8)].copy_from_slice(&z.to_le_bytes());
        offset += 8;
    }
    // Write the actual Z range limits back into the placeholder spot
    if min_z.is_infinite() {
        min_z = 0.0;
    }
    view[z_range_offset..(z_range_offset + 8)].copy_from_slice(&min_z.to_le_bytes());
    if max_z.is_infinite() {
        max_z = 0.0;
    }
    view[(z_range_offset + 8)..(z_range_offset + 16)].copy_from_slice(&max_z.to_le_bytes());

    // 5. Phase C: Optional M Block serialization
    if let Some(m_value) = m_value {
        let mut min_m = f64::INFINITY;
        let mut max_m = f64::NEG_INFINITY;
        let m_range_offset = offset;
        offset += 16; // Skip past the 16-byte M-range placeholder

        for point in points {
            let m = m_value(point.m.as_ref().map(|m| m.clone().into())).unwrap_or(m_no_data);
            if m != m_no_data {
                min_m = f64::min(min_m, m);
                max_m = f64::max(max_m, m);
            }
            view[offset..(offset + 8)].copy_from_slice(&m.to_le_bytes());
            offset += 8;
        }
        // Write the actual M range limits back into the placeholder spot
        if min_m.is_infinite() {
            min_m = 0.0;
        }
        view[m_range_offset..(m_range_offset + 8)].copy_from_slice(&min_m.to_le_bytes());
        if max_m.is_infinite() {
            max_m = 0.0;
        }
        view[(m_range_offset + 8)..(m_range_offset + 16)].copy_from_slice(&max_m.to_le_bytes());
    }

    view
}

// Byte 0 Shape Type 13 Integer 1 Little
// Byte 4 Box Box Double 4 Little
// Byte 36 NumParts NumParts Integer 1 Little
// Byte 40 NumPoints NumPoints Integer 1 Little
// Byte 44 Parts Parts Integer NumParts Little
// Byte X Points Points Point NumPoints Little
// Byte Y Zmin Zmin Double 1 Little
// Byte Y + 8 Zmax Zmax Double 1 Little
// Byte Y + 16 Zarray Zarray Double NumPoints Little
// Byte Z* Mmin Mmin Double 1 Little
// Byte Z+8* Mmax Mmax Double 1 Little
// Byte Z+16* Marray Marray Double NumPoints Little
// Note: X = 44 + (4 * NumParts), Y = X + (16 * NumPoints), Z = Y + 16 + (8 * NumPoints)
// * optional
fn write_line_strings_z<M: MValueCompatible>(
    lines: &[VectorLineString<M>],
    bbox: BBox3D,
    m_value: Option<fn(m: Option<MValue>) -> Option<f64>>,
    r#type: Option<SHPShapeType>,
) -> Vec<u8> {
    let r#type = r#type.unwrap_or(SHPShapeType::POLYLINEZ);
    let total_points = lines.iter().fold(0, |acc, line| acc + line.len());
    let total_parts = lines.len();
    let has_m = m_value.is_some();
    let m_no_data = f64::MIN;

    // Calculate explicit buffer size:
    // Header(44) + Parts(4 * total_parts) + Points(16 * total_points) + Z Range(16) + Z Array(8 * total_points)
    let mut buffer_size = 44 + total_parts * 4 + total_points * 16 + 16 + total_points * 8; // Base line structure
    buffer_size += 16 + total_points * 8; // Required Z dimension blocks
    if has_m {
        buffer_size += 16 + total_points * 8; // Optional M dimension blocks
    }

    let mut view = vec![0; buffer_size];

    view[0..4].copy_from_slice(&(r#type as i32).to_le_bytes());
    view[4..12].copy_from_slice(&bbox.left.to_le_bytes());
    view[12..20].copy_from_slice(&bbox.bottom.to_le_bytes());
    view[20..28].copy_from_slice(&bbox.right.to_le_bytes());
    view[28..36].copy_from_slice(&bbox.top.to_le_bytes());
    view[36..40].copy_from_slice(&(total_parts as i32).to_le_bytes());
    view[40..44].copy_from_slice(&(total_points as i32).to_le_bytes());

    let mut offset = 44;
    let mut part_index_accumulator: i32 = 0;
    for line in lines.iter() {
        view[offset..offset + 4].copy_from_slice(&part_index_accumulator.to_le_bytes());
        offset += 4;
        part_index_accumulator += line.len() as i32;
    }

    // 1. Write complete X/Y Block
    for line in lines.iter() {
        for point in line.iter() {
            view[offset..offset + 8].copy_from_slice(&point.x.to_le_bytes());
            view[offset + 8..offset + 16].copy_from_slice(&point.y.to_le_bytes());
            offset += 16;
        }
    }
    // 2. Write Z bounding limits followed immediately by flat Z array
    let mut min_z = f64::INFINITY;
    let mut max_z = f64::NEG_INFINITY;
    let z_start_offset = offset;
    offset += 16; // Advance past placeholders to stream values
    for line in lines.iter() {
        for point in line.iter() {
            let z = point.z.unwrap_or_default();
            if z < min_z {
                min_z = z;
            }
            if z > max_z {
                max_z = z;
            }
            view[offset..offset + 8].copy_from_slice(&z.to_le_bytes());
            offset += 8;
        }
    }
    if min_z.is_infinite() {
        min_z = 0.0;
    }
    view[z_start_offset..z_start_offset + 8].copy_from_slice(&min_z.to_le_bytes());
    if max_z.is_infinite() {
        max_z = 0.0;
    }
    view[z_start_offset + 8..z_start_offset + 16].copy_from_slice(&max_z.to_le_bytes());

    // 3. Write optional M bounding limits followed immediately by flat M array
    if let Some(m_value) = m_value {
        let mut min_m = f64::INFINITY;
        let mut max_m = f64::NEG_INFINITY;
        let m_start_offset = offset;
        offset += 16;
        for line in lines.iter() {
            for point in line.iter() {
                let m = m_value(point.m.as_ref().map(|m| m.clone().into())).unwrap_or(m_no_data);
                if m != m_no_data {
                    if m < min_m {
                        min_m = m;
                    }
                    if m > max_m {
                        max_m = m;
                    }
                }
                view[offset..offset + 8].copy_from_slice(&m.to_le_bytes());
                offset += 8;
            }
        }
        if min_m.is_infinite() {
            min_m = 0.0;
        }
        view[m_start_offset..m_start_offset + 8].copy_from_slice(&min_m.to_le_bytes());
        if max_m.is_infinite() {
            max_m = 0.0;
        }
        view[m_start_offset + 8..m_start_offset + 16].copy_from_slice(&max_m.to_le_bytes());
    }

    view
}
