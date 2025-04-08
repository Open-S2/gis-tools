use super::DataBaseFile;
use crate::{
    proj::Transformer,
    readers::{BufferReader, FeatureReader, Reader},
};
use alloc::{vec, vec::Vec};
use core::marker::PhantomData;
use s2json::{
    BBox3D, MValueCompatible, Properties, VectorFeature, VectorFeatureType, VectorGeometry,
    VectorGeometryType, VectorLineString, VectorMultiLineString, VectorMultiPoint,
    VectorMultiPointGeometry, VectorPoint, VectorPointGeometry,
};

/// A Shapefile Header describing the internal data
#[derive(Debug, PartialEq)]
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
/// Reads data from a shapefile implementing the {@link FeatureIterator} interface
#[derive(Debug)]
pub struct ShapeFileReader<T: Reader, P: MValueCompatible = Properties> {
    /// The input reader
    reader: T,
    header: SHPHeader,
    dbf: Option<DataBaseFile<T, P>>, // Use the same lifetime for dbf
    transform: Option<Transformer>,
    row_offsets: Vec<u64>,
    _phantom: PhantomData<VectorFeature<(), P, ()>>,
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

    fn parse_row(&self, index: usize) -> Option<VectorFeature<(), P, ()>> {
        if index >= self.row_offsets.len() {
            return None;
        }
        let row_offset = self.row_offsets.get(index)?;
        let SHPRow { id, _type, data, .. } = self.get_row(*row_offset)?;
        let geometry = self.parse_geometry(_type, &data);
        geometry.as_ref()?;
        let mut properties: P = P::default();
        if let Some(dbf) = &self.dbf {
            if let Some(props) = dbf.get_properties(index as u64) {
                properties = props;
            }
        }

        Some(VectorFeature {
            id: Some(id),
            _type: VectorFeatureType::VectorFeature,
            face: 0.into(),
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

    fn parse_geometry(&self, _type: i32, data: &[u8]) -> Option<VectorGeometry<()>> {
        let reader: BufferReader = data.into();
        let is_3d = _type == 11 || _type == 13 || _type == 15 || _type == 18;
        if _type == 1 || _type == 11 {
            Some(VectorGeometry::Point(VectorPointGeometry {
                _type: VectorGeometryType::Point,
                is_3d,
                coordinates: self.parse_point(&reader, 0, if is_3d { Some(16) } else { None }),
                ..Default::default()
            }))
        } else if _type == 8 || _type == 18 {
            self.parse_multi_point(&reader, is_3d)
        } else if _type == 3 || _type == 5 || _type == 13 || _type == 15 {
            let is_poly = _type == 5 || _type == 15;
            self.parse_multi_line(&reader, is_poly, is_3d)
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
    ) -> VectorPoint<()> {
        let mut z: Option<f64> = None;
        if let Some(offset) = offset_3d {
            z = Some(data.f64_le(Some(offset)));
        }
        let mut point =
            VectorPoint::new(data.f64_le(Some(offset)), data.f64_le(Some(offset + 8)), z, None);
        if let Some(transformer) = &self.transform {
            point = transformer.forward(point);
        }

        point
    }

    fn parse_multi_point(&self, data: &BufferReader, is_3d: bool) -> Option<VectorGeometry<()>> {
        let num_points = data.int32_le(Some(32)) as u64;
        if num_points == 0 {
            return None;
        }
        let mut offset = 0;
        let mut z_offset = 36 + 16 * num_points;
        // grab the min-max
        let mins = self.parse_point(data, offset, None);
        let maxs = self.parse_point(data, offset + 16, None);
        offset += 36;
        let mut bbox = BBox3D::new(mins.x, mins.y, maxs.x, maxs.y, 0., 0.);
        if is_3d {
            bbox.near = data.f64_le(Some(z_offset));
            bbox.far = data.f64_le(Some(z_offset + 8));
            z_offset += 16;
        }

        let mut coordinates: VectorMultiPoint<()> = vec![];
        let mut index = 0;
        while index < num_points {
            let point = self.parse_point(data, offset, if is_3d { Some(z_offset) } else { None });
            offset += 16;
            if is_3d {
                z_offset += 8;
                bbox.extend_from_point(&point); // shapefiles often don't store the bbox z-values
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
    ) -> Option<VectorGeometry<()>> {
        let num_parts = data.int32_le(Some(32)) as u64; // The number of rings in the polygon.
        let num_points = data.int32_le(Some(36)) as u64; // the total number of points in the polygon.
        if num_points == 0 || num_parts == 0 {
            return None;
        }
        let mut offset = 0;
        let mut z_offset = 40 + 4 * num_parts + 16 * num_points;
        // grab the min-max
        let mins = self.parse_point(data, offset, None);
        let maxs = self.parse_point(data, offset + 16, None);
        let mut bbox = BBox3D::new(mins.x, mins.y, maxs.x, maxs.y, 0., 0.);
        offset += 40;
        if is_3d {
            bbox.near = data.f64_le(Some(z_offset));
            bbox.far = data.f64_le(Some(z_offset + 8));
            z_offset += 16;
        }

        // build parts
        let mut parts: Vec<u64> = vec![];
        let mut done = 0;
        while done < num_parts {
            parts.push(data.int32_le(Some(offset)) as u64);
            offset += 4;
            done += 1;
        }

        // build coordinates
        let mut index = 0;
        let mut coordinates: VectorMultiLineString<()> = vec![];
        for i in 0..num_parts {
            let part_end = parts.get(i as usize + 1).unwrap_or(&num_points);
            // build a line for part
            let mut line: VectorLineString<()> = vec![];
            while index < *part_end {
                let point =
                    self.parse_point(data, offset, if is_3d { Some(z_offset) } else { None });
                offset += 16;
                if is_3d {
                    z_offset += 8;
                    bbox.extend_from_point(&point); // shapefiles often don't store the bbox z-values
                }
                line.push(point);
                index += 1;
            }
            coordinates.push(line);
        }

        if !is_poly {
            if num_parts == 1 {
                Some(VectorGeometry::new_linestring(
                    core::mem::take(&mut coordinates[0]),
                    Some(bbox),
                ))
            } else {
                Some(VectorGeometry::new_multilinestring(coordinates, Some(bbox)))
            }
        } else {
            Some(VectorGeometry::new_polygon(coordinates, Some(bbox)))
        }
    }
}
/// The GPX Iterator tool
pub struct ShapefileIterator<'a, T: Reader, P: MValueCompatible> {
    reader: &'a ShapeFileReader<T, P>,
    index: usize,
}
impl<T: Reader, P: MValueCompatible> Iterator for ShapefileIterator<'_, T, P> {
    type Item = VectorFeature<(), P, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.reader.parse_row(self.index - 1)
    }
}
/// A feature reader trait with a callback-based approach
impl<T: Reader, P: MValueCompatible> FeatureReader<(), P, ()> for ShapeFileReader<T, P> {
    type FeatureIterator<'a>
        = ShapefileIterator<'a, T, P>
    where
        T: 'a,
        P: 'a;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        ShapefileIterator { reader: self, index: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::readers::FileReader;
    use alloc::string::String;
    use s2json::MValue;
    use std::path::PathBuf;

    #[test]
    fn test_shapefile() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/shapefile/fixtures/utf.shp");

        let shp: ShapeFileReader<FileReader, MValue> =
            ShapeFileReader::new(FileReader::from(path.clone()), None, None);

        let header = shp.get_header();
        assert_eq!(
            header,
            &SHPHeader {
                bbox: BBox3D::new(
                    -108.97956848144531,
                    41.244772343082076,
                    -108.6328125,
                    41.253032440653186,
                    0.,
                    0.
                ),
                length: 156,
                shp_code: 1,
                version: 1000
            }
        );

        let features: Vec<_> = shp.iter().collect();
        assert_eq!(features.len(), 2);

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(1),
                    geometry: VectorGeometry::new_point(
                        VectorPoint::new_xy(-108.6328125, 41.244772343082076, Some(())),
                        None
                    ),
                    ..Default::default()
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(2),
                    geometry: VectorGeometry::new_point(
                        VectorPoint::new_xy(-108.97956848144531, 41.253032440653186, Some(())),
                        None
                    ),
                    ..Default::default()
                }
            ]
        )
    }

    #[test]
    fn test_shapefile_with_utf() {
        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
        struct FieldStruct {
            field: String,
        }
        impl FieldStruct {
            fn new(field: String) -> Self {
                Self { field }
            }
        }

        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let pbf_path = path.join("tests/readers/shapefile/fixtures/utf.shp");
        let dbf_path = path.join("tests/readers/shapefile/fixtures/utf.dbf");

        let dbf: DataBaseFile<FileReader, FieldStruct> =
            DataBaseFile::new(FileReader::from(dbf_path.clone()), Some("utf-8".into()));
        let shp: ShapeFileReader<FileReader, FieldStruct> =
            ShapeFileReader::new(FileReader::from(pbf_path.clone()), Some(dbf), None);

        let header = shp.get_header();
        assert_eq!(
            header,
            &SHPHeader {
                bbox: BBox3D::new(
                    -108.97956848144531,
                    41.244772343082076,
                    -108.6328125,
                    41.253032440653186,
                    0.,
                    0.
                ),
                length: 156,
                shp_code: 1,
                version: 1000
            }
        );

        let features: Vec<_> = shp.iter().collect();
        assert_eq!(features.len(), 2);

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(1),
                    properties: FieldStruct::new("💩".into()),
                    geometry: VectorGeometry::new_point(
                        VectorPoint::new_xy(-108.6328125, 41.244772343082076, Some(())),
                        None
                    ),
                    ..Default::default()
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(2),
                    properties: FieldStruct::new("Hněvošický háj".into()),
                    geometry: VectorGeometry::new_point(
                        VectorPoint::new_xy(-108.97956848144531, 41.253032440653186, Some(())),
                        None
                    ),
                    ..Default::default()
                }
            ]
        );
    }

    #[test]
    fn test_shapefile_polylines() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/shapefile/fixtures/export_polylinez.shp");

        let shp: ShapeFileReader<FileReader, MValue> =
            ShapeFileReader::new(FileReader::from(path.clone()), None, None);

        let header = shp.get_header();
        assert_eq!(
            header,
            &SHPHeader {
                bbox: BBox3D::new(-120., 38., -113., 45., 0., 0.),
                length: 384,
                shp_code: 13,
                version: 1000
            }
        );

        let features: Vec<_> = shp.iter().collect();
        assert_eq!(features.len(), 1);

        assert_eq!(
            features,
            vec![VectorFeature {
                _type: VectorFeatureType::VectorFeature,
                id: Some(1),
                geometry: VectorGeometry::new_multilinestring(
                    vec![
                        vec![
                            VectorPoint::new_xyz(-120., 45., 800., Some(())),
                            VectorPoint::new_xyz(-119., 44., 1100., Some(())),
                            VectorPoint::new_xyz(-118., 43., 2300., Some(())),
                        ],
                        vec![
                            VectorPoint::new_xyz(-115., 40., 0., Some(())),
                            VectorPoint::new_xyz(-114., 39., 0., Some(())),
                            VectorPoint::new_xyz(-113., 38., 0., Some(())),
                        ],
                    ],
                    Some(BBox3D::new(-120., 38., -113., 45., 0., 2300.)),
                ),
                ..Default::default()
            }]
        )
    }

    #[test]
    fn test_shapefile_multipointz() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/shapefile/fixtures/export_multipointz.shp");

        let shp: ShapeFileReader<FileReader, MValue> =
            ShapeFileReader::new(FileReader::from(path.clone()), None, None);

        let header = shp.get_header();
        assert_eq!(
            header,
            &SHPHeader {
                bbox: BBox3D::new(-123., 46., -121., 48., 0., 0.),
                length: 276,
                shp_code: 18,
                version: 1000
            }
        );

        let features: Vec<_> = shp.iter().collect();
        assert_eq!(features.len(), 1);

        assert_eq!(
            features,
            vec![VectorFeature {
                _type: VectorFeatureType::VectorFeature,
                id: Some(0),
                geometry: VectorGeometry::new_multipoint(
                    vec![
                        VectorPoint::new_xyz(-123., 48., 1200., None),
                        VectorPoint::new_xyz(-122., 47., 2500., None),
                        VectorPoint::new_xyz(-121., 46., 3600., None),
                    ],
                    Some(BBox3D::new(-123., 46., -121., 48., 1200., 3600.)),
                ),
                ..Default::default()
            }]
        )
    }
}
