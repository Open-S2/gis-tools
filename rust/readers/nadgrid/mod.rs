use crate::parsers::{FeatureReader, Reader};
use alloc::string::String;
use alloc::{vec, vec::Vec};
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

// /**
//  * # NAD Grid V2 Reader
//  *
//  * ## Description
//  * Store Grids from a NTv2 file (.gsb)
//  *
//  * ## Usage
//  * ```ts
//  * // TODO
//  * ```
//  */
// pub struct NadGridStore<T: Reader> {
//     grids: BTreeMap<String, NadGridReader<T>>,
// }
// impl<T: Reader> NadGridStore<T> {
//     /**
//      * Insert a new NadGrid into the store
//      * @param grid - a nadgrid class to store
//      */
//     pub fn add_grid(&mut self, grid: NadGridReader<T>) {
//         self.grids.insert(grid.key.clone(), grid);
//     }

//     /**
//      * Get a grid from the store given a key or name
//      * @param key - the key or name of the grid
//      * @returns - the grid
//      */
//     pub fn get_grid(&self, key: &str) -> Option<&NadGridReader<T>> {
//         self.grids.get(key)
//     }

//     /**
//      * Add a grid given a data input
//      * @param key - the key or name of the grid
//      * @param input - the input data to parse
//      */
//     pub fn add_grid_from_reader(&mut self, key: String, input: T) {
//         self.add_grid(NadGridReader::new(key, input));
//     }

//     /**
//      * Get grid definitions from a string name
//      * @param keys - complex string of grid keys to test against
//      * @returns - an array of grid definitions
//      */
//     pub fn get_grids_from_string(&self, keys: Option<String>) -> Vec<NadGridDefinition<T>> {
//         let mut res = vec![];
//         if keys.is_none() {
//             return res;
//         }
//         for grid in
//             keys.unwrap_or_default().split(',').map(|s| s.trim().into()).collect::<Vec<String>>()
//         {
//             if let Some(g) = self.get_grid_from_string(grid) {
//                 res.push(g);
//             }
//         }
//         res
//     }

//     /**
//      * Get a grid definition from a string
//      * @param name - a single grid name to test against
//      * @returns - a grid definition
//      */
//     pub fn get_grid_from_string(&self, mut name: String) -> Option<NadGridDefinition<T>> {
//         if name.is_empty() {
//             return None;
//         }
//         let optional = name.chars().nth(0) == Some('@');
//         if optional {
//             name = (&name[1..]).into();
//         }
//         if &name == "null" {
//             return Some(NadGridDefinition {
//                 name: "null".into(),
//                 mandatory: !optional,
//                 grid: None,
//                 is_null: true,
//             });
//         }
//         Some(NadGridDefinition {
//             name: name.clone(),
//             mandatory: !optional,
//             grid: self.grids.get(&name),
//             is_null: false,
//         })
//     }
// }

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

/**
 * # NAD Grid Reader
 *
 * ## Description
 * Loads/reads a binary NTv2 file (.gsb) implementing the {@link FeatureIterator} interface
 *
 * It should be noted that a proj4 Transformer usually uses this class internally. But if you want
 * to manually parse a .gsb file, you can use this class directly.
 *
 * ## Usage
 *
 * ```ts
 * // TODO
 * ```
 *
 * ## Links
 * - https://web.archive.org/web/20140127204822if_/http://www.mgs.gov.on.ca:80/stdprodconsume/groups/content/@mgs/@iandit/documents/resourcelist/stel02_047447.pdf
 * - http://mimaka.com/help/gs/html/004_NTV2%20Data%20Format.htm
 */
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
        self.header.n_fields = if le { reader.int32_le(Some(8)) } else { reader.int32_be(Some(8)) };
        self.header.n_subgrid_fields =
            if le { reader.int32_le(Some(24)) } else { reader.int32_be(Some(24)) };
        self.header.n_subgrids =
            if le { reader.int32_le(Some(40)) } else { reader.int32_be(Some(40)) };
        self.header.shift_type = reader.parse_string(Some(56), Some(8));
        self.header.from_semi_major_axis =
            if le { reader.f64_le(Some(120)) } else { reader.f64_be(Some(120)) };
        self.header.from_semi_minor_axis =
            if le { reader.f64_le(Some(136)) } else { reader.f64_be(Some(136)) };
        self.header.to_semi_major_axis =
            if le { reader.f64_le(Some(152)) } else { reader.f64_be(Some(152)) };
        self.header.to_semi_minor_axis =
            if le { reader.f64_le(Some(168)) } else { reader.f64_be(Some(168)) };
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

    /**
     * @param offset - offset to read in the subgrid header
     * @returns - the subgrid header
     */
    fn read_sub_grid_header(&self, offset: u64) -> NadSubGridHeader {
        let NadGridReader { reader, is_little_endian, .. } = self;
        let le = *is_little_endian;
        NadSubGridHeader {
            name: reader.parse_string(Some(offset + 8), Some(8)),
            parent: reader.parse_string(Some(offset + 24), Some(8)),
            lower_latitude: if le {
                reader.f64_le(Some(offset + 72))
            } else {
                reader.f64_be(Some(offset + 72))
            },
            upper_latitude: if le {
                reader.f64_le(Some(offset + 88))
            } else {
                reader.f64_be(Some(offset + 88))
            },
            lower_longitude: if le {
                reader.f64_le(Some(offset + 104))
            } else {
                reader.f64_be(Some(offset + 104))
            },
            upper_longitude: if le {
                reader.f64_le(Some(offset + 120))
            } else {
                reader.f64_be(Some(offset + 120))
            },
            latitude_interval: if le {
                reader.f64_le(Some(offset + 136))
            } else {
                reader.f64_be(Some(offset + 136))
            },
            longitude_interval: if le {
                reader.f64_le(Some(offset + 152))
            } else {
                reader.f64_be(Some(offset + 152))
            },
            grid_node_count: if le {
                reader.int32_le(Some(offset + 168))
            } else {
                reader.int32_be(Some(offset + 168))
            },
        }
    }

    /**
     * @param offset - offset of the grid
     * @param grid_header - header of the grid
     * @returns - an array of grid nodes
     */
    fn read_grid_nodes(&self, offset: u64, grid_header: &NadSubGridHeader) -> Vec<NadGridNode> {
        let NadGridReader { reader, is_little_endian, .. } = self;
        let le = *is_little_endian;
        let node_count = grid_header.grid_node_count as u64;
        let nodes_offset = offset + 176;
        let grid_record_length: u64 = 16;
        let mut grid_shift_records = vec![];
        let mut i: u64 = 0;
        while i < node_count {
            grid_shift_records.push(NadGridNode {
                latitude_shift: if le {
                    reader.f32_le(Some(nodes_offset + i * grid_record_length)) as f64
                } else {
                    reader.f32_be(Some(nodes_offset + i * grid_record_length)) as f64
                },
                longitude_shift: if le {
                    reader.f32_le(Some(nodes_offset + i * grid_record_length + 4)) as f64
                } else {
                    reader.f32_be(Some(nodes_offset + i * grid_record_length + 4)) as f64
                },
                latitude_accuracy: if le {
                    reader.f32_le(Some(nodes_offset + i * grid_record_length + 8)) as f64
                } else {
                    reader.f32_be(Some(nodes_offset + i * grid_record_length + 8)) as f64
                },
                longitude_accuracy: if le {
                    reader.f32_le(Some(nodes_offset + i * grid_record_length + 12)) as f64
                } else {
                    reader.f32_be(Some(nodes_offset + i * grid_record_length + 12)) as f64
                },
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
