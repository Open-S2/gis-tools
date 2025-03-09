use core::marker::PhantomData;

use alloc::{vec, vec::Vec};

use s2json::{
    BBox3D, MValue, MValueCompatible, VectorFeature, VectorFeatureType, VectorGeometry,
    VectorGeometryType, VectorLineString, VectorLineStringGeometry, VectorMultiLineString,
    VectorMultiPoint, VectorMultiPointGeometry, VectorPoint, VectorPointGeometry,
    VectorPolygonGeometry,
};

use crate::{
    proj::Transformer,
    readers::{BufferReader, FeatureIterator, Reader},
};

use super::DataBaseFile;

/// A Shapefile Header describing the internal data
#[derive(Debug, PartialEq)]
pub struct SHPHeader {
    length: usize,
    version: i32,
    shp_code: i32,
    bbox: BBox3D,
}

/// A Shapefile Row explaining how to read the feature
#[derive(Debug)]
pub struct SHPRow {
    id: u64,
    #[allow(dead_code)]
    len: usize,
    _type: i32,
    data: Vec<u8>,
}

/// # The Shapefile Reader
///
/// ## Description
/// Reads data from a shapefile implementing the {@link FeatureIterator} interface
#[derive(Debug)]
pub struct ShapeFileReader<
    T: Reader,
    M: Clone = (),
    P: MValueCompatible = MValue,
    D: MValueCompatible = MValue,
> {
    /// The input reader
    reader: T,
    header: SHPHeader,
    dbf: Option<DataBaseFile<T, P>>, // Use the same lifetime for dbf
    transform: Option<Transformer>,
    row_offsets: Vec<usize>,
    _pos: usize,
    _phantom: PhantomData<VectorFeature<M, P, D>>,
}

impl<T: Reader, M: Clone, P: MValueCompatible, D: MValueCompatible> ShapeFileReader<T, M, P, D> {
    /// Create a new Shapefile Reader
    pub fn new(
        mut reader: T,
        dbf: Option<DataBaseFile<T, P>>,
        transform: Option<Transformer>,
    ) -> ShapeFileReader<T, M, P, D> {
        let header = ShapeFileReader::<T, M, P, D>::parse_header(&mut reader);
        let row_offsets = ShapeFileReader::<T, M, P, D>::parse_row_offsets(&mut reader);
        ShapeFileReader::<T, M, P, D> {
            reader,
            header,
            dbf,
            row_offsets,
            transform,
            _pos: 0,
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
            length: (reader.int32_be(Some(6 << 2)) << 1) as usize,
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
    fn parse_row_offsets(reader: &mut T) -> Vec<usize> {
        let mut res = vec![];

        let mut offset = 100;
        let len = reader.len() - 8;
        while offset <= len {
            let offset_length = (reader.int32_be(Some(offset + 4)) << 1) as usize;
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

    fn parse_next_row(&mut self) -> Option<VectorFeature<M, P, D>> {
        if self._pos >= self.row_offsets.len() {
            return None;
        }
        let idx = self._pos;
        self._pos += 1;
        let row_offset = self.row_offsets[idx];

        let row = self.get_row(row_offset);
        row.as_ref()?;
        let SHPRow { id, _type, data, .. } = row.unwrap();
        let geometry = self.parse_geometry(_type, &data);
        geometry.as_ref()?;
        let mut properties: P = P::default();
        if let Some(dbf) = &mut self.dbf {
            if let Some(props) = dbf.get_properties(idx) {
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
    fn get_row(&mut self, offset: usize) -> Option<SHPRow> {
        let id = self.reader.int32_be(Some(offset)) as u64;
        let len = (self.reader.int32_be(Some(offset + 4)) << 1) as usize;
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

    fn parse_geometry(&mut self, _type: i32, data: &[u8]) -> Option<VectorGeometry<D>> {
        let mut reader: BufferReader = data.into();
        let is_3d = _type == 11 || _type == 13 || _type == 15 || _type == 18;
        if _type == 1 || _type == 11 {
            Some(VectorGeometry::Point(VectorPointGeometry {
                _type: VectorGeometryType::Point,
                is_3d,
                coordinates: self.parse_point(&mut reader, 0, if is_3d { Some(16) } else { None }),
                ..Default::default()
            }))
        } else if _type == 8 || _type == 18 {
            self.parse_multi_point(&mut reader, is_3d)
        } else if _type == 3 || _type == 5 || _type == 13 || _type == 15 {
            let is_poly = _type == 5 || _type == 15;
            self.parse_multi_line(&mut reader, is_poly, is_3d)
        } else {
            panic!("invalid shape type: {}", _type);
        }
    }

    /// Parse a point
    fn parse_point(
        &mut self,
        data: &mut BufferReader,
        offset: usize,
        offset_3d: Option<usize>,
    ) -> VectorPoint<D> {
        let mut z: Option<f64> = None;
        if let Some(offset) = offset_3d {
            z = Some(data.f64_le(Some(offset + 16)));
        }
        let mut point =
            VectorPoint::new(data.f64_le(Some(offset)), data.f64_le(Some(offset + 8)), z, None);
        if let Some(transformer) = &self.transform {
            point = transformer.forward(point);
        }

        point
    }

    fn parse_multi_point(
        &mut self,
        data: &mut BufferReader,
        is_3d: bool,
    ) -> Option<VectorGeometry<D>> {
        let num_points = data.int32_le(Some(32)) as usize;
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

        let mut coordinates: VectorMultiPoint<D> = vec![];
        let mut index = 0;
        while index < num_points {
            let point = self.parse_point(data, offset, if is_3d { Some(z_offset) } else { None });
            offset += 16;
            if is_3d {
                z_offset += 8;
                bbox.extend_from_point(&point);
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
        &mut self,
        data: &mut BufferReader,
        is_poly: bool,
        is_3d: bool,
    ) -> Option<VectorGeometry<D>> {
        let num_parts = data.int32_le(Some(32)) as usize; // The number of rings in the polygon.
        let num_points = data.int32_le(Some(36)) as usize; // the total number of points in the polygon.
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
        let mut parts: Vec<usize> = vec![];
        let mut done = 0;
        while done < num_parts {
            parts.push(data.int32_le(Some(offset)) as usize);
            offset += 4;
            done += 1;
        }

        // build coordinates
        let mut index = 0;
        let mut coordinates: VectorMultiLineString<D> = vec![];
        for i in 0..num_parts {
            let part_end = parts.get(i + 1).unwrap_or(&num_points);
            // build a line for part
            let mut line: VectorLineString<D> = vec![];
            while index < *part_end {
                let point =
                    self.parse_point(data, offset, if is_3d { Some(z_offset) } else { None });
                offset += 16;
                if is_3d {
                    z_offset += 8;
                    bbox.extend_from_point(&point);
                }
                line.push(point);
                index += 1;
            }
            coordinates.push(line);
        }

        if !is_poly && num_parts == 1 {
            Some(VectorGeometry::LineString(VectorLineStringGeometry {
                _type: VectorGeometryType::LineString,
                is_3d,
                coordinates: core::mem::take(&mut coordinates[0]),
                bbox: Some(bbox),
                ..Default::default()
            }))
        } else {
            Some(VectorGeometry::Polygon(VectorPolygonGeometry {
                _type: VectorGeometryType::Polygon,
                is_3d,
                coordinates,
                bbox: Some(bbox),
                ..Default::default()
            }))
        }
    }
}
impl<T: Reader, M: Clone, P: MValueCompatible, D: MValueCompatible> Iterator
    for ShapeFileReader<T, M, P, D>
{
    type Item = VectorFeature<M, P, D>;
    fn next(&mut self) -> Option<Self::Item> {
        self.parse_next_row()
    }
}
// Let the library know this struct is compatible as a VectorFeature iterator
impl<T: Reader, M: Clone, P: MValueCompatible, D: MValueCompatible> FeatureIterator<M, P, D>
    for ShapeFileReader<T, M, P, D>
{
}
