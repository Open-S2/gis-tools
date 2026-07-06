use super::DataBaseFile;
use crate::{
    parsers::{BufferReader, FeatureReader, Reader},
    proj::Transformer,
};
use alloc::{vec, vec::Vec};
use core::marker::PhantomData;
use s2json::{
    BBox3D, Face, MValue, MValueCompatible, Properties, VectorFeature, VectorFeatureType,
    VectorGeometry, VectorGeometryType, VectorLineString, VectorMultiLineString, VectorMultiPoint,
    VectorMultiPointGeometry, VectorPoint, VectorPointGeometry,
};

/// A Shapefile Header describing the internal data
#[derive(Debug, Clone, PartialEq)]
pub struct SHPHeader {
    /// The length of the file
    pub length: u64,
    /// The shapefile version
    pub version: i32,
    /// The shape code
    pub shp_code: i32,
    /// The bounding box
    pub bbox: BBox3D,
}

/// A Shapefile Row explaining how to read the feature
#[derive(Debug)]
pub struct SHPRow {
    id: u64,
    #[allow(dead_code)]
    len: u64,
    _type: i32,
    data: Vec<u8>,
}

/// # The Shapefile Reader
///
/// ## Description
/// Reads data from a shapefile
///
/// Implements the [`FeatureReader`] trait
///
/// ## Usage
///
/// NOTE: It's recommended to not parse the shapefile directly but instead:
/// - [`crate::readers::file::shapefile_from_path`]
/// - [`crate::readers::mmap::shapefile_from_path`]
///
/// This ensures the other files paired with the shapefile are loaded to properly handle the
/// projection and properties data.
///
/// ## Usage
///
/// The methods you have access to:
/// - [`ShapeFileReader::new`]: Create a new ShapeFileReader
/// - [`ShapeFileReader::get_header`]: Get the file header data
/// - [`ShapeFileReader::iter`]: Iterate over the features in the shapefile
///
/// ### From Path (Recommended as it will ensure to pull in associated files):
/// ```rust
/// use gistools::{parsers::{FileReader, FeatureReader}, readers::{ShapeFileReader, file::shapefile_from_path}};
/// use s2json::MValue;
/// use std::{collections::BTreeMap, path::PathBuf};
///
/// let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
/// path.push("tests/readers/shapefile/fixtures/utf.shp");
/// let path_str = path.to_str().unwrap();
///
/// #[derive(Default, Debug, Clone, MValue, PartialEq)]
/// struct Props {
///     field: String,
/// }
///
/// let shp: ShapeFileReader<FileReader, Props> =
///     shapefile_from_path(path_str, BTreeMap::from([("a".into(), "b".into())]));
///
/// let features: Vec<_> = shp.iter().collect();
/// assert_eq!(features.len(), 2);
/// ```
///
/// ## Links
/// - <https://en.wikipedia.org/wiki/Shapefile>
#[derive(Debug, Clone)]
pub struct ShapeFileReader<T: Reader, P: MValueCompatible = Properties> {
    /// The input reader
    reader: T,
    header: SHPHeader,
    dbf: Option<DataBaseFile<T, P>>, // Use the same lifetime for dbf
    transform: Option<Transformer>,
    row_offsets: Vec<u64>,
    _phantom: PhantomData<VectorFeature<(), P, MValue>>,
}
impl<T: Reader, P: MValueCompatible> ShapeFileReader<T, P> {
    /// Create a new Shapefile Reader
    pub fn new(
        mut reader: T,
        dbf: Option<DataBaseFile<T, P>>,
        transform: Option<Transformer>,
    ) -> ShapeFileReader<T, P> {
        let header = ShapeFileReader::<T, P>::parse_header(&mut reader);
        let row_offsets = ShapeFileReader::<T, P>::parse_row_offsets(&mut reader);
        ShapeFileReader::<T, P> {
            reader,
            header,
            dbf,
            row_offsets,
            transform,
            _phantom: PhantomData,
        }
    }

    /// Return a reference to the header
    pub fn get_header(&self) -> &SHPHeader {
        &self.header
    }

    /// Returns a header object from the reader
    fn parse_header(reader: &mut T) -> SHPHeader {
        let mut header = SHPHeader {
            length: (reader.int32_be(Some(6 << 2)) << 1) as u64,
            version: reader.int32_le(Some(7 << 2)),
            shp_code: reader.int32_le(Some(8 << 2)),
            bbox: BBox3D::new(
                reader.f64_le(Some(9 << 2)),
                reader.f64_le(Some(11 << 2)),
                reader.f64_le(Some(13 << 2)),
                reader.f64_le(Some(15 << 2)),
                reader.f64_le(Some(17 << 2)),
                reader.f64_le(Some(19 << 2)),
            ),
        };
        if header.shp_code > 20 {
            header.shp_code -= 20;
        }

        header
    }

    /// Returns the rows starting positions from the reader
    fn parse_row_offsets(reader: &mut T) -> Vec<u64> {
        let mut res = vec![];

        let mut offset = 100;
        let len = reader.len() - 8;
        while offset <= len {
            let offset_length = (reader.int32_be(Some(offset + 4)) << 1) as u64;
            let _type = reader.int32_le(Some(offset + 8));
            if offset_length == 0 {
                break;
            }
            if _type != 0 {
                res.push(offset);
            }
            offset += 8 + offset_length;
        }

        res
    }

    fn parse_row(&self, index: usize) -> Option<VectorFeature<(), P, MValue>> {
        if index >= self.row_offsets.len() {
            return None;
        }
        let row_offset = self.row_offsets.get(index)?;
        let SHPRow { id, _type, data, .. } = self.get_row(*row_offset)?;
        let geometry = self.parse_geometry(_type, &data);
        geometry.as_ref()?;
        let mut properties: P = P::default();
        if let Some(dbf) = &self.dbf
            && let Some(props) = dbf.get_properties(index as u64)
        {
            properties = props;
        }

        Some(VectorFeature {
            id: Some(id),
            _type: VectorFeatureType::VectorFeature,
            face: Face::WM,
            properties,
            geometry: geometry.unwrap(),
            metadata: None,
        })
    }

    /// Get a row
    fn get_row(&self, offset: u64) -> Option<SHPRow> {
        let id = self.reader.int32_be(Some(offset)) as u64;
        let len = (self.reader.int32_be(Some(offset + 4)) << 1) as u64;
        if len == 0 || offset + len + 8 > self.reader.len() {
            return None;
        }
        Some(SHPRow {
            id,
            len,
            data: self.reader.slice(Some(offset + 12), Some(offset + 12 + len - 4)),
            _type: self.reader.int32_le(Some(offset + 8)),
        })
    }

    fn parse_geometry(&self, _type: i32, data: &[u8]) -> Option<VectorGeometry<MValue>> {
        let reader: BufferReader = data.into();

        // ESRI Type flags:
        let is_3d = (11..=18).contains(&_type);
        let has_m = (21..=28).contains(&_type) || is_3d;

        if _type == 1 || _type == 11 || _type == 21 {
            let m_offset = if _type == 21 {
                Some(16)
            } else if is_3d && data.len() >= 32 {
                Some(24)
            } else {
                None
            };

            Some(VectorGeometry::Point(VectorPointGeometry {
                _type: VectorGeometryType::Point,
                is_3d,
                coordinates: self.parse_point(
                    &reader,
                    0,
                    if is_3d { Some(16) } else { None },
                    m_offset,
                ),
                ..Default::default()
            }))
        } else if _type == 8 || _type == 18 || _type == 28 {
            self.parse_multi_point(&reader, is_3d, has_m)
        } else if _type == 3
            || _type == 5
            || _type == 13
            || _type == 15
            || _type == 23
            || _type == 25
        {
            let is_poly = _type == 5 || _type == 15 || _type == 25;
            self.parse_multi_line(&reader, is_poly, is_3d, has_m)
        } else {
            panic!("invalid shape type: {}", _type);
        }
    }

    /// Parse a point
    fn parse_point(
        &self,
        data: &BufferReader,
        offset: u64,
        offset_3d: Option<u64>,
        offset_m: Option<u64>,
    ) -> VectorPoint<MValue> {
        let z = offset_3d.map(|off| data.f64_le(Some(off)));

        let m_value = offset_m.map(|off| data.f64_le(Some(off)));
        let m = match m_value {
            Some(m_val) if m_val != f64::MIN => {
                Some(MValue::from([("value".into(), m_val.into())]))
            }
            _ => None,
        };

        let mut point =
            VectorPoint::new(data.f64_le(Some(offset)), data.f64_le(Some(offset + 8)), z, m);

        if let Some(transformer) = &self.transform {
            point = transformer.forward(&point);
        }

        point
    }

    fn parse_multi_point(
        &self,
        data: &BufferReader,
        is_3d: bool,
        has_m: bool,
    ) -> Option<VectorGeometry<MValue>> {
        let num_points = data.int32_le(Some(32)) as u64;
        if num_points == 0 {
            return None;
        }

        let mut offset = 36;
        let mut z_offset = 36 + 16 * num_points;

        // Grab the min-max bounds
        let mins = self.parse_point(data, 0, None, None);
        let maxs = self.parse_point(data, 16, None, None);
        let mut bbox = BBox3D::new(mins.x, mins.y, maxs.x, maxs.y, 0., 0.);

        if is_3d {
            bbox.near = data.f64_le(Some(z_offset));
            bbox.far = data.f64_le(Some(z_offset + 8));
            z_offset += 16;
        }

        // M array block starts cleanly past the XY array and any Z Range/Z Array blocks
        let mut m_offset = 36 + 16 * num_points + if is_3d { 16 + 8 * num_points } else { 0 };

        // Safe guard to check if the optional M block actually exists in this record buffer
        // Assuming data has a .len() method returning total byte length
        let holds_m = has_m && m_offset + 16 + 8 * num_points <= data.len();
        if holds_m {
            m_offset += 16; // Skip past the 16-byte M Range [minM, maxM] block
        }

        let mut coordinates: VectorMultiPoint<MValue> = vec![];
        let mut index = 0;
        while index < num_points {
            let point = self.parse_point(
                data,
                offset,
                if is_3d { Some(z_offset) } else { None },
                if holds_m { Some(m_offset) } else { None },
            );

            offset += 16;
            if is_3d {
                z_offset += 8;
                bbox.extend_from_point(&point);
            }
            if holds_m {
                m_offset += 8;
            }
            coordinates.push(point);
            index += 1;
        }

        if num_points == 1 {
            Some(VectorGeometry::Point(VectorPointGeometry {
                _type: VectorGeometryType::Point,
                is_3d,
                coordinates: core::mem::take(&mut coordinates[0]),
                bbox: Some(bbox),
                ..Default::default()
            }))
        } else {
            Some(VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                _type: VectorGeometryType::MultiPoint,
                is_3d,
                coordinates,
                bbox: Some(bbox),
                ..Default::default()
            }))
        }
    }

    fn parse_multi_line(
        &self,
        data: &BufferReader,
        is_poly: bool,
        is_3d: bool,
        has_m: bool,
    ) -> Option<VectorGeometry<MValue>> {
        let num_parts = data.int32_le(Some(32)) as u64; // The number of rings/parts
        let num_points = data.int32_le(Some(36)) as u64; // The total number of points
        if num_points == 0 || num_parts == 0 {
            return None;
        }

        // Restore original working offsets
        let mut offset = 40 + 4 * num_parts; // Points array starts exactly after the parts directory
        let mut z_offset = offset + 16 * num_points; // Z block starts after all XY points

        // Grab the min-max bounds
        let mins = self.parse_point(data, 0, None, None);
        let maxs = self.parse_point(data, 16, None, None);
        let mut bbox = BBox3D::new(mins.x, mins.y, maxs.x, maxs.y, 0., 0.);

        if is_3d {
            bbox.near = data.f64_le(Some(z_offset));
            bbox.far = data.f64_le(Some(z_offset + 8));
            z_offset += 16; // Skip past the 16-byte Z Range block
        }

        // Compute the exact trailing M block offset
        let mut m_offset =
            40 + 4 * num_parts + 16 * num_points + if is_3d { 16 + 8 * num_points } else { 0 };

        // Safe guard to verify the optional M block actually exists in this record buffer
        let holds_m = has_m && m_offset + 16 + 8 * num_points <= data.len();
        if holds_m {
            m_offset += 16; // Skip past the 16-byte M Range [minM, maxM] block
        }

        // Parse the parts directory correctly starting at byte 40
        let mut parts: Vec<u64> = vec![];
        let mut done = 0;
        while done < num_parts {
            parts.push(data.int32_le(Some(40 + done * 4)) as u64);
            done += 1;
        }

        // Build coordinates
        let mut index = 0;
        let mut coordinates: VectorMultiLineString<MValue> = vec![];
        for i in 0..num_parts {
            let part_end = parts.get(i as usize + 1).unwrap_or(&num_points);
            let mut line: VectorLineString<MValue> = vec![];

            while index < *part_end {
                let point = self.parse_point(
                    data,
                    offset,
                    if is_3d { Some(z_offset) } else { None },
                    if holds_m { Some(m_offset) } else { None },
                );

                offset += 16;
                if is_3d {
                    z_offset += 8;
                    bbox.extend_from_point(&point);
                }
                if holds_m {
                    m_offset += 8;
                }
                line.push(point);
                index += 1;
            }
            coordinates.push(line);
        }

        if !is_poly && num_parts == 1 {
            Some(VectorGeometry::new_linestring(core::mem::take(&mut coordinates[0]), Some(bbox)))
        } else if is_poly {
            Some(VectorGeometry::new_polygon(coordinates, Some(bbox)))
        } else {
            Some(VectorGeometry::new_multilinestring(coordinates, Some(bbox)))
        }
    }
}
/// The GPX Iterator tool
#[derive(Debug)]
pub struct ShapefileIterator<'a, T: Reader, P: MValueCompatible> {
    reader: &'a ShapeFileReader<T, P>,
    index: usize,
    stride: usize,
}
impl<T: Reader, P: MValueCompatible> Iterator for ShapefileIterator<'_, T, P> {
    type Item = VectorFeature<(), P, MValue>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(feature) = self.reader.parse_row(self.index) {
            self.index += self.stride;
            return Some(feature);
        }
        None
    }
}
/// A feature reader trait with a callback-based approach
impl<T: Reader, P: MValueCompatible> FeatureReader<(), P, MValue> for ShapeFileReader<T, P> {
    type FeatureIterator<'a>
        = ShapefileIterator<'a, T, P>
    where
        T: 'a,
        P: 'a;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        ShapefileIterator { reader: self, index: 0, stride: 1 }
    }
    // The assumption here is that the reader has been cloned already
    fn par_iter(&self, pool_size: usize, thread_id: usize) -> Self::FeatureIterator<'_> {
        ShapefileIterator { reader: self, index: thread_id, stride: pool_size }
    }
}
