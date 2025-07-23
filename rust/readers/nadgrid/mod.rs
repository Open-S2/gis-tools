use crate::parsers::{FeatureReader, Reader};
use alloc::{string::String, vec, vec::Vec};
use libm::round;
use s2json::{
    MValue, Point, Properties, VectorFeature, VectorGeometry, VectorMultiPoint, VectorPoint,
};

/// Seconds to degrees (S / 3_600)
const SEC2DEG: f64 = 0.00000484813681109536;

/// A Subgrid contained inside a NadGrid
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NadSubGrid {
    /// CVS => lonLat coords
    pub cvs: VectorMultiPoint,
    /// ll => lower_lon_lat
    pub ll: VectorPoint,
    /// del => lon_lat_interval
    pub del: VectorPoint,
    /// lim => lon_lat_column_count
    pub lim: VectorPoint,
    /// count
    pub count: i32,
}

/// A Subgrid contained inside a NadGrid
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NadMetadata {
    /// ll => lower_lon_lat
    pub ll: Point,
    /// del => lon_lat_interval
    pub del: Point,
    /// lim => lon_lat_column_count
    pub lim: Point,
    /// count
    pub count: i32,
}

/// An LAS Shaped Vector Feature
pub type NadVectorFeature = VectorFeature<NadMetadata, Properties, MValue>;

/// A grid wrapper around a parsed .gsb file
#[derive(Default, Debug, Clone)]
pub struct NadGridDefinition<'a, T: Reader> {
    /// Grid name
    pub name: String,
    /// If the grid is mandatory
    pub mandatory: bool,
    /// Grid data
    pub grid: Option<&'a NadGridReader<T>>,
    /// If the grid is null
    pub is_null: bool,
}

/// The header of a NTv2 file
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NadGridHeader {
    /// Grid fields count
    pub n_fields: i32,
    /// Subgrid fields count
    pub n_subgrid_fields: i32,
    /// Subgrids count
    pub n_subgrids: i32,
    /// Shift type
    pub shift_type: String,
    /// from semi major axis
    pub from_semi_major_axis: f64,
    /// from semi minor axis
    pub from_semi_minor_axis: f64,
    /// to semi major axis
    pub to_semi_major_axis: f64,
    /// to semi minor axis
    pub to_semi_minor_axis: f64,
}

/// Each subgrid has it's own data on how to decode the points inside the subgrid
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NadSubGridHeader {
    /// The name of the subgrid
    pub name: String,
    /// The name of the parent grid
    pub parent: String,
    /// The lower latitude of the subgrid
    pub lower_latitude: f64,
    /// The upper latitude of the subgrid
    pub upper_latitude: f64,
    /// The lower longitude of the subgrid
    pub lower_longitude: f64,
    /// The upper longitude of the subgrid
    pub upper_longitude: f64,
    /// The latitude interval
    pub latitude_interval: f64,
    /// The longitude interval
    pub longitude_interval: f64,
    /// The number of points in the subgrid
    pub grid_node_count: i32,
}

/// A single Node describing how to decode the point
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NadGridNode {
    /// The latitude shift
    pub latitude_shift: f64,
    /// The longitude shift
    pub longitude_shift: f64,
    /// The latitude accuracy
    pub latitude_accuracy: f64,
    /// The longitude accuracy
    pub longitude_accuracy: f64,
}

/// The metadata inside each vector feature
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NadGridMetadata {
    /// The lower longitude and latitude
    pub lower_lon_lat: VectorPoint,
    /// The longitude and latitude interval
    pub lon_lat_interval: VectorPoint,
    /// The number of longitude and latitude columns
    pub lon_lat_column_count: VectorPoint,
    /// The number of points
    pub count: u64,
}

/// # NAD Grid Reader
///
/// ## Description
/// Loads/reads a binary NTv2 file (.gsb)
///
/// Implements the [`FeatureReader`] trait
///
/// It should be noted that a proj4 Transformer usually uses this class internally. But if you want
/// to manually parse a .gsb file, you can use this class directly.
///
/// ## Usage
///
/// The methods you have access to:
/// - [`NadGridReader::new`]: Create a new NadGridReader
/// - [`NadGridReader::len`]: Get the length of the feature count
/// - [`NadGridReader::is_empty`]: Check if the reader is empty
/// - [`NadGridReader::header`]: Read the header
/// - [`NadGridReader::get_points`]: Read a subgrid into a Point
/// - [`NadGridReader::get_feature`]: Read a subgrid into a Vector Feature
/// - [`NadGridReader::iter`]: Create an iterator to collect the features
/// - [`NadGridReader::par_iter`]: Create a parallel iterator to collect the features
///
/// ```rust
/// use gistools::{parsers::{FeatureReader, FileReader}, readers::NadGridReader};
/// use std::path::PathBuf;
///
/// let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
/// path.push("tests/proj4/fixtures/BETA2007.gsb");
///
/// let nadgrid_reader = NadGridReader::new("test".into(), FileReader::from(path.clone()));
/// let features = nadgrid_reader.iter().collect::<Vec<_>>();
/// assert_eq!(features.len(), 1);
/// ```
///
/// ## Links
/// - https://web.archive.org/web/20140127204822if_/http://www.mgs.gov.on.ca:80/stdprodconsume/groups/content/@mgs/@iandit/documents/resourcelist/stel02_047447.pdf
/// - http://mimaka.com/help/gs/html/004_NTV2%20Data%20Format.htm
#[derive(Debug)]
pub struct NadGridReader<T: Reader> {
    /// The name of the grid
    pub key: String,
    reader: T,
    is_little_endian: bool,
    header: NadGridHeader,
    subgrids: Vec<NadSubGrid>,
}
impl<T: Reader> NadGridReader<T> {
    /// Create a new NadGridReader
    pub fn new(key: String, reader: T) -> Self {
        let mut nad_grid = Self {
            key,
            reader,
            is_little_endian: false,
            header: NadGridHeader::default(),
            subgrids: vec![],
        };
        nad_grid.detect_little_endian();
        nad_grid.read_header();
        nad_grid.read_sub_grids();

        nad_grid
    }

    /// Get the length of the feature count
    pub fn len(&self) -> u64 {
        self.subgrids.len() as u64
    }

    /// Check if the reader is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// read the header
    pub fn header(&self) -> &NadGridHeader {
        &self.header
    }

    /// Read a subgrid into a Point
    pub fn get_points(&self, index: u64) -> Option<VectorMultiPoint> {
        if let Some(grid) = self.subgrids.get(index as usize) {
            return Some(grid.cvs.clone());
        }
        None
    }

    /// Read a subgrid into a Vector Feature
    pub fn get_feature(&self, index: u64) -> Option<NadVectorFeature> {
        if let Some(grid) = self.subgrids.get(index as usize) {
            let NadSubGrid { cvs, ll, del, lim, count, .. } = grid;
            return Some(VectorFeature::new_wm(
                None,
                Properties::default(),
                VectorGeometry::new_multipoint(cvs.clone(), None),
                Some(NadMetadata {
                    ll: Point(ll.x, ll.y),
                    del: Point(del.x, del.y),
                    lim: Point(lim.x, lim.y),
                    count: *count,
                }),
            ));
        }
        None
    }

    // INTERNAL

    /// Set the little endian flag
    fn detect_little_endian(&mut self) {
        let NadGridReader { reader, .. } = self;
        let mut n_fields = reader.int32_be(Some(8));
        if n_fields == 11 {
            return;
        }
        n_fields = reader.int32_le(Some(8));
        if n_fields != 11 {
            panic!("Failed to detect nadgrid endian-ness, defaulting to little-endian");
        }
        self.is_little_endian = true;
    }

    /// Parse the main header data. Describes the subgrids to decode lon-lat
    fn read_header(&mut self) {
        let NadGridReader { reader, is_little_endian, .. } = self;
        let le = *is_little_endian;
        self.header.n_fields = reader.int32(Some(8), Some(le));
        self.header.n_subgrid_fields = reader.int32(Some(24), Some(le));
        self.header.n_subgrids = reader.int32(Some(40), Some(le));
        self.header.shift_type = reader.parse_string(Some(56), Some(8));
        self.header.from_semi_major_axis = reader.f64(Some(120), Some(le));
        self.header.from_semi_minor_axis = reader.f64(Some(136), Some(le));
        self.header.to_semi_major_axis = reader.f64(Some(152), Some(le));
        self.header.to_semi_minor_axis = reader.f64(Some(168), Some(le));
    }

    /// Build all grid data
    fn read_sub_grids(&mut self) {
        let mut grid_offset = 176;
        let mut i = 0;
        while i < self.header.n_subgrids {
            let sub_header = self.read_sub_grid_header(grid_offset);
            let nodes = self.read_grid_nodes(grid_offset, &sub_header);
            let lon_column_count = round(
                1.0 + (sub_header.upper_longitude - sub_header.lower_longitude)
                    / sub_header.longitude_interval,
            );
            let lat_column_count = round(
                1.0 + (sub_header.upper_latitude - sub_header.lower_latitude)
                    / sub_header.latitude_interval,
            );

            self.subgrids.push(NadSubGrid {
                cvs: nodes
                    .iter()
                    .map(|node| {
                        let NadGridNode { longitude_shift, latitude_shift, .. } = node;
                        VectorPoint::new_xy(
                            longitude_shift * SEC2DEG,
                            latitude_shift * SEC2DEG,
                            None,
                        )
                    })
                    .collect(),
                ll: VectorPoint::new_xy(
                    sub_header.lower_longitude * SEC2DEG,
                    sub_header.lower_latitude * SEC2DEG,
                    None,
                ),
                del: VectorPoint::new_xy(
                    sub_header.longitude_interval * SEC2DEG,
                    sub_header.latitude_interval * SEC2DEG,
                    None,
                ),
                lim: VectorPoint::new_xy(lon_column_count, lat_column_count, None),
                count: sub_header.grid_node_count,
            });
            grid_offset += 176 + sub_header.grid_node_count as u64 * 16;

            i += 1;
        }
    }

    /// Read a subgrid header
    ///
    /// ## Parameters
    /// - `offset`: offset to read in the subgrid header
    ///
    /// ## Returns
    /// The subgrid header
    fn read_sub_grid_header(&self, offset: u64) -> NadSubGridHeader {
        let NadGridReader { reader, is_little_endian, .. } = self;
        let le = *is_little_endian;
        NadSubGridHeader {
            name: reader.parse_string(Some(offset + 8), Some(8)),
            parent: reader.parse_string(Some(offset + 24), Some(8)),
            lower_latitude: reader.f64(Some(offset + 72), Some(le)),
            upper_latitude: reader.f64(Some(offset + 88), Some(le)),
            lower_longitude: reader.f64(Some(offset + 104), Some(le)),
            upper_longitude: reader.f64(Some(offset + 120), Some(le)),
            latitude_interval: reader.f64(Some(offset + 136), Some(le)),
            longitude_interval: reader.f64(Some(offset + 152), Some(le)),
            grid_node_count: reader.int32(Some(offset + 168), Some(le)),
        }
    }

    /// Read the grid nodes
    ///
    /// ## Parameters
    /// - `offset`: offset of the grid
    /// - `grid_header`: header of the grid
    ///
    /// ## Returns
    /// An array of grid nodes
    fn read_grid_nodes(&self, offset: u64, grid_header: &NadSubGridHeader) -> Vec<NadGridNode> {
        let NadGridReader { reader, is_little_endian, .. } = self;
        let le = *is_little_endian;
        let node_count = grid_header.grid_node_count as u64;
        let nodes_offset = offset + 176;
        let grl: u64 = 16; // grid_record_length
        let mut grid_shift_records = vec![];
        let mut i: u64 = 0;
        while i < node_count {
            grid_shift_records.push(NadGridNode {
                latitude_shift: reader.f32(Some(nodes_offset + i * grl), Some(le)) as f64,
                longitude_shift: reader.f32(Some(nodes_offset + i * grl + 4), Some(le)) as f64,
                latitude_accuracy: reader.f32(Some(nodes_offset + i * grl + 8), Some(le)) as f64,
                longitude_accuracy: reader.f32(Some(nodes_offset + i * grl + 12), Some(le)) as f64,
            });
            i += 1;
        }
        grid_shift_records
    }
}

/// The NadGrid Iterator tool
#[derive(Debug)]
pub struct NadGridIterator<'a, T: Reader> {
    reader: &'a NadGridReader<T>,
    index: u64,
}
impl<T: Reader> Iterator for NadGridIterator<'_, T> {
    type Item = NadVectorFeature;

    fn next(&mut self) -> Option<Self::Item> {
        let cdf_reader = &self.reader;
        if let Some(point) = cdf_reader.get_feature(self.index) {
            self.index += 1;
            Some(point)
        } else {
            None
        }
    }
}
/// A feature reader trait with a callback-based approach
impl<T: Reader> FeatureReader<NadMetadata, Properties, MValue> for NadGridReader<T> {
    type FeatureIterator<'a>
        = NadGridIterator<'a, T>
    where
        T: 'a;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        NadGridIterator { reader: self, index: 0 }
    }

    #[cfg(feature = "std")]
    fn par_iter(&self, _pool_size: usize, _thread_id: usize) -> Self::FeatureIterator<'_> {
        self.iter()
    }
}
