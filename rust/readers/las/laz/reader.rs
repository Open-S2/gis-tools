use super::{
    ItemReader, ItemReaders, arithmetic_decoder::ArithmeticDecoder,
    integer_compressor::IntegerCompressor, modify_point14_raw_input,
};
use crate::{
    parsers::{BufferReader, FeatureReader, Reader},
    proj::Transformer,
    readers::{
        GeoStore, LASExtendedVariableLengthRecord, LASHeader, LASPoint, LASReaderOptions,
        LASVectorFeature, LAZCompressor, LAZHeader, LAZHeaderItem, LAZHeaderItemType,
        build_geo_key_directory, build_wkt, las_parse_variable_length_records,
        laz::{v1::LAZPoint10v1Reader, v2::LAZPoint10v2Reader},
        v1::{LAZGpsTime11v1Reader, LAZbyte10v1Reader, LAZrgb12v1Reader, LAZwavepacket13v1Reader},
        v2::{LAZGpsTime11v2Reader, LAZbyte10v2Reader, LAZrgb12v2Reader},
        v3::{
            LAZPoint14v3Reader, LAZbyte14v3Reader, LAZrgb14v3Reader, LAZrgbNir14v3Reader,
            LAZwavepacket14v3Reader,
        },
    },
};
use alloc::{collections::BTreeMap, rc::Rc, string::String, vec, vec::Vec};
use core::{cell::RefCell, fmt::Debug};
use s2json::{Properties, VectorFeature, VectorGeometry, VectorPoint};

/// A state for the decompression
#[derive(Debug)]
pub struct LAZState<T: Reader + Debug> {
    chunk_size: u32,
    chunk_count: u32,
    curr_chunk: u32,
    number_chunks: u32,
    tabled_chunks: u32,
    chunk_totals: Vec<u32>,
    chunk_starts: Vec<i64>,
    point_start: i64,
    readers: Vec<ItemReaders<T>>,
}
impl<T: Reader + Debug> Default for LAZState<T> {
    fn default() -> Self {
        Self {
            chunk_size: u32::MAX,
            chunk_count: 0,
            curr_chunk: 0,
            number_chunks: 0,
            tabled_chunks: 0,
            chunk_totals: vec![],
            chunk_starts: vec![],
            point_start: 0,
            readers: vec![],
        }
    }
}
impl<T: Reader + Debug> LAZState<T> {
    /// Setup
    pub fn setup(&mut self, header: &LAZHeader) {
        *self = Default::default();
        if header.compressor != LAZCompressor::Pointwise {
            self.chunk_count = header.chunk_size;
            if header.chunk_size != 0 {
                self.chunk_size = header.chunk_size;
            }
            self.number_chunks = u32::MAX;
        }
    }
}

/// # LASzip Reader
///
/// ## Description
/// Reads LAS zipped data. Supports LAS 1.4 specification although missing some support.
/// [See specification](https://downloads.rapidlasso.de/doc/LAZ_Specification_1.4_R1.pdf)
/// Implements the {@link FeatureIterator} interface
///
/// Data is stored like so:
/// ```txt
/// |            PUBLIC HEADER BLOCK           |
/// |          VARIABLE LENGTH RECORDS         |
/// |             POINT DATA RECORDS           |
/// | Extended Variable Length Records (EVLRs) |
/// |  Field Chunk table start position (EOF)  |
/// ```
///
/// ## Usage
/// ```ts
/// // TODO
/// ```
///
/// ## Links
/// - https://www.usgs.gov/ngp-standards-and-specifications/lidar-base-specification-online
/// - https://www.asprs.org/wp-content/uploads/2010/12/LAS_1_4_r13.pdf
/// - https://liblas.org/development/index.html
/// - https://downloads.rapidlasso.de/doc/LAZ_Specification_1.4_R1.pdf
/// - https://github.com/PDAL/PDAL
/// - https://github.com/libLAS/libLAS (deprecated for PDAL)
/// - https://github.com/LASzip
#[derive(Debug)]
pub struct LAZReader<T: Reader + Debug> {
    reader: Rc<RefCell<T>>,
    /// LAS Header Block
    pub header: LASHeader,
    /// LAZ Header Block
    pub laz_header: LAZHeader,
    /// Extended VARIABLE LENGTH RECORDS
    pub variable_length_records: BTreeMap<u32, LASExtendedVariableLengthRecord>,
    /// WKT projection string
    pub wkt: Option<String>,
    /// GeoKeyDirectory
    pub geo_key_directory: GeoStore,
    dec: Rc<RefCell<ArithmeticDecoder<T>>>,
    // decompress_selective: u32, // all
    // Is true if self file uses layered compression for LAS 1.4.
    layered_las14_compression: bool,
    transformer: Transformer,
    dont_transform: bool,
    state: RefCell<LAZState<T>>,
}
impl<T: Reader + Debug> LAZReader<T> {
    /// Create a new LAZReader
    pub fn new(reader: T, options: Option<LASReaderOptions>) -> Self {
        let options = options.unwrap_or_default();
        let header = LASHeader::from_reader(&reader);
        let variable_length_records = las_parse_variable_length_records(&header, &reader);
        let mut transformer = Transformer::new();
        for (epsg_code, wkt) in options.epsg_codes.iter() {
            transformer.insert_epsg_code(epsg_code.clone(), wkt.clone());
        }
        let wkt = build_wkt(&header, &variable_length_records, &mut transformer);
        let geo_key_directory = build_geo_key_directory(&variable_length_records, &mut transformer);
        let reader = Rc::new(RefCell::new(reader));
        let mut laz_reader = Self {
            reader: reader.clone(),
            header,
            variable_length_records,
            laz_header: LAZHeader::default(),
            wkt,
            geo_key_directory,
            transformer,
            dec: Rc::new(RefCell::new(ArithmeticDecoder::new(reader))),
            // decompress_selective: 0xffffffff,
            layered_las14_compression: false,
            state: LAZState::default().into(),
            dont_transform: options.dont_transform,
        };
        laz_reader.build_laz();
        laz_reader.parse_extended_variable_length_records();
        // setup other decoding variables
        laz_reader.layered_las14_compression =
            laz_reader.laz_header.compressor == LAZCompressor::LayeredAndChunked;
        laz_reader.state.borrow_mut().setup(&laz_reader.laz_header);

        laz_reader
    }

    /// Get the number of points stored
    pub fn len(&self) -> u64 {
        self.header.num_points as u64
    }

    /// Check if the reader is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// If the LAZ variable length record is present, build a LAZ parser
    fn build_laz(&mut self) {
        if self.header.point_data_format_id >= 127 {
            let laz_data_record = self.variable_length_records.get(&22204);
            if let Some(laz_data_record) = laz_data_record {
                if let Some(laz_data) = &laz_data_record.data {
                    self.laz_header = LAZHeader::from_bytes(laz_data.to_vec());
                    return;
                }
            }
        }
        panic!("LAZ data, but LAZ record not found.");
    }

    /// The Compressed Data Block can be followed by any number of EVLRs, which are identical to the
    /// LAS 1.4 specification. The EVLR is similar to a VLR, but can carry a larger payload, as the Record
    /// Length After Header field is 8 bytes instead of 2 bytes. The number of EVLRs is specified in the
    /// Number of Extended Variable Length Records field in the Public Header Block. The start of the
    /// first EVLR is at the file offset indicated by the Start of first Extended Variable Length Record in
    /// the Public Header Block.
    ///
    /// The Extended Variable Length Records must be accessed sequentially, since the size of each
    /// variable length record is contained in the Extended Variable Length Record Header. Each Extended
    /// Variable Length Record Header (i.e. without the optional payload data) is 60 bytes in length.
    fn parse_extended_variable_length_records(&mut self) {
        let Self { reader, laz_header, .. } = self;
        let LAZHeader { offset_special_evlrs, num_special_evlrs, .. } = laz_header;
        let mut position = *offset_special_evlrs as u64;
        let reader = reader.borrow();
        for _ in 0..*num_special_evlrs {
            let record = LASExtendedVariableLengthRecord::from_reader_extended(&*reader, position);
            position += 60 + record.record_length;
            self.variable_length_records.insert(record.record_id as u32, record);
        }
    }

    /// returns the next point
    pub fn get_point(&self) -> Option<VectorPoint<LASPoint>> {
        if self.header.num_points == 0 {
            return None;
        }
        let LAZHeader { compressor, .. } = self.laz_header;
        let mut context = 0;
        // no decoder means it wasn't compressed, just pull the point
        let uncompressed = self.set_next_chunk();
        self.state.borrow_mut().chunk_count += 1;
        let mut point = LASPoint::default();
        if uncompressed || compressor == LAZCompressor::None {
            self.first_chunk_read(&mut context, &mut point);
        } else {
            self.pointwise_compress_read(&mut point, &mut context);
        }
        let mut vp = point.to_vector_point(&self.header);
        // transform if needed
        if !self.dont_transform {
            self.transformer.forward_mut(&mut vp);
        }

        Some(vp)
    }

    /// Reads a point in at index as a feature
    pub fn get_feature(&self) -> Option<LASVectorFeature> {
        self.get_point().map(|point| {
            VectorFeature::new_wm(
                None,
                Properties::default(),
                VectorGeometry::new_point(point, None),
                None,
            )
        })
    }

    /// If we are at the end of the chunk, we need to set up the next chunk.
    /// @returns - true if we are starting a new chunk, false otherwise
    fn set_next_chunk(&self) -> bool {
        if self.state.borrow().chunk_count == self.state.borrow().chunk_size {
            if self.state.borrow().point_start != 0 {
                self.state.borrow_mut().curr_chunk += 1;
            }
            self.init_dec();
            let state = &mut self.state.borrow_mut();
            let curr_chunk = state.curr_chunk;
            let tabled_chunks = state.tabled_chunks;
            if curr_chunk == tabled_chunks {
                // no or incomplete chunk table?
                state.chunk_starts[tabled_chunks as usize] = state.point_start; // needs fixing
                state.tabled_chunks += 1;
            } else if !state.chunk_totals.is_empty() {
                // variable sized chunks?
                state.chunk_size = state.chunk_totals[curr_chunk as usize + 1]
                    - state.chunk_totals[curr_chunk as usize];
            }
            state.chunk_count = 0;
            return true;
        }
        false
    }

    /// Initialize decoder
    fn init_dec(&self) {
        // maybe read chunk table (only if chunking enabled)
        if self.state.borrow().number_chunks == u32::MAX {
            self.read_chunk_table();
            let state = &mut self.state.borrow_mut();
            state.curr_chunk = 0;
            if !state.chunk_totals.is_empty() {
                state.chunk_size = state.chunk_totals[1];
            }
        }
        self.state.borrow_mut().point_start = self.reader.borrow().tell() as i64;
    }

    /// @param context - current context
    /// @param point_data - where to store the decompressed data
    fn first_chunk_read(&self, context: &mut u32, point: &mut LASPoint) {
        let state = &mut self.state.borrow_mut();
        let LAZHeader { items, .. } = &self.laz_header;
        let mut point_data: Vec<BufferReader> = vec![];
        state.readers.clear();
        // first read in the raw data
        for item in items.iter() {
            state.readers.push(self.get_point_compressed_reader(item));
            let LAZHeaderItem { r#type, size, .. } = *item;
            let mut raw_data: BufferReader = self.reader.borrow().seek_slice(size as usize).into();
            if r#type == LAZHeaderItemType::Point14 {
                raw_data = modify_point14_raw_input(&raw_data);
            }
            point_data.push(raw_data);
        }
        // now choose how we initialize
        if self.layered_las14_compression {
            // set decoder and grap count size
            (*self.dec).borrow_mut().init(false);
            let _count = self.reader.borrow().uint32_le(None);
            // chunk sizes then init
            for reader in state.readers.iter_mut() {
                reader.chunk_sizes(&(*self.reader.borrow()));
            }
            for (i, reader) in state.readers.iter_mut().enumerate() {
                reader.init(&point_data[i], point, context);
            }
        } else {
            // initialize readers then init decoder
            for (i, reader) in state.readers.iter_mut().enumerate() {
                reader.init(&point_data[i], point, context);
            }
            (*self.dec).borrow_mut().init(true);
        }
    }

    /// @param item - the item to build readers for
    /// returns the compression reader for the item
    fn get_point_compressed_reader(&self, item: &LAZHeaderItem) -> ItemReaders<T> {
        let LAZHeaderItem { r#type, size, version } = *item;
        if r#type == LAZHeaderItemType::Point10 {
            if version == 1 {
                return LAZPoint10v1Reader::new(self.dec.clone()).into();
            } else if version == 2 {
                return LAZPoint10v2Reader::new(self.dec.clone()).into();
            }
        } else if r#type == LAZHeaderItemType::GpsTime11 {
            if version == 1 {
                return LAZGpsTime11v1Reader::new(self.dec.clone()).into();
            } else if version == 2 {
                return LAZGpsTime11v2Reader::new(self.dec.clone()).into();
            }
        } else if r#type == LAZHeaderItemType::Rgb12 {
            if version == 1 {
                return LAZrgb12v1Reader::new(self.dec.clone()).into();
            } else if version == 2 {
                return LAZrgb12v2Reader::new(self.dec.clone()).into();
            }
        } else if (r#type == LAZHeaderItemType::WavePacket13) && (version == 1) {
            return LAZwavepacket13v1Reader::new(self.dec.clone()).into();
        } else if r#type == LAZHeaderItemType::Byte {
            if version == 1 {
                return LAZbyte10v1Reader::new(self.dec.clone(), size as u32).into();
            } else if version == 2 {
                return LAZbyte10v2Reader::new(self.dec.clone(), size as u32).into();
            }
        } else if r#type == LAZHeaderItemType::Point14 {
            if version == 3 || version == 4 {
                return LAZPoint14v3Reader::new(self.dec.clone(), None).into();
            }
        } else if r#type == LAZHeaderItemType::Rgb14 {
            if version == 3 || version == 4 {
                return LAZrgb14v3Reader::new(self.dec.clone(), None).into();
            }
        } else if r#type == LAZHeaderItemType::RgbNir14 {
            if version == 3 || version == 4 {
                return LAZrgbNir14v3Reader::new(self.dec.clone(), None).into();
            }
        } else if r#type == LAZHeaderItemType::WavePacket14 {
            if version == 3 || version == 4 {
                return LAZwavepacket14v3Reader::new(self.dec.clone(), None).into();
            }
        } else if r#type == LAZHeaderItemType::Byte14 && (version == 3 || version == 4) {
            return LAZbyte14v3Reader::new(self.dec.clone(), size as u32, None).into();
        }
        panic!("Unsupported compressed point type: {:?} & version: {}", r#type, version);
    }

    /// @param point_data - where to store the decompressed data
    /// @param context - the context
    fn pointwise_compress_read(&self, point: &mut LASPoint, context: &mut u32) {
        for reader in self.state.borrow_mut().readers.iter_mut() {
            reader.read(point, context);
        }
    }

    /// If chunking is enabled, read the chunk table
    fn read_chunk_table(&self) {
        let Self { reader, .. } = self;
        let reader = reader.borrow();
        let mut chunk_table_start_position = reader.int64_le(None);
        // self is where the chunks start
        let chunks_start = reader.tell() as i64; // I64

        if chunk_table_start_position == -1 {
            // the compressor was writing to a non-seekable stream and wrote the chunk table start at the end
            // read the last 8 bytes
            chunk_table_start_position = reader.int64_le(Some(reader.len() - 8));
        }

        // read the chunk table
        // move to where the chunk table starts
        reader.seek(chunk_table_start_position as u64);
        // fail if the version is wrong
        let version = reader.uint32_le(None);
        if version != 0 {
            panic!("Bad version number. Aborting.");
        }
        // build the chunk table
        let state = &mut self.state.borrow_mut();
        state.number_chunks = reader.uint32_le(None);
        state.chunk_totals = vec![];
        // set chunk start and totals
        if state.chunk_size == u32::MAX {
            state.chunk_totals = vec![0; state.number_chunks as usize + 1];
        } else {
            state.chunk_starts.push(chunks_start);
        }
        state.tabled_chunks = 1;
        if state.number_chunks > 0 {
            (*self.dec).borrow_mut().init(true);
            let mut ic = IntegerCompressor::new(self.dec.clone(), Some(32), Some(2), None, None);
            ic.init_decompressor();
            for i in 1..=state.number_chunks as usize {
                if state.chunk_size == u32::MAX {
                    let chunk_total = ic
                        .decompress(if i > 1 { state.chunk_totals[i - 1] as i32 } else { 0 }, 0)
                        as u32;
                    state.chunk_totals.push(chunk_total);
                }
                let chunk_start = ic
                    .decompress(if i > 1 { state.chunk_starts[i - 1] as i32 } else { 0 }, 1)
                    as i64;
                state.chunk_starts.push(chunk_start);
                state.tabled_chunks += 1;
            }
            for i in 1..=state.number_chunks as usize {
                if state.chunk_size == u32::MAX {
                    state.chunk_totals[i] += state.chunk_totals[i - 1];
                }
                state.chunk_starts[i] += state.chunk_starts[i - 1];
            }
        }

        reader.seek(chunks_start as u64);
    }
}

/// The LAZ Iterator tool
#[derive(Debug)]
pub struct LAZIterator<'a, T: Reader + Debug> {
    reader: &'a LAZReader<T>,
    index: u64,
}
impl<T: Reader + Debug> Iterator for LAZIterator<'_, T> {
    type Item = LASVectorFeature;

    fn next(&mut self) -> Option<Self::Item> {
        let las = &self.reader;
        self.index += 1;
        if self.index > las.len() {
            return None;
        }
        las.get_feature()
    }
}
/// A feature reader trait with a callback-based approach
impl<T: Reader + Debug> FeatureReader<(), Properties, LASPoint> for LAZReader<T> {
    type FeatureIterator<'a>
        = LAZIterator<'a, T>
    where
        T: 'a;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        self.reader.borrow().seek(self.header.offset_to_points as u64);
        self.state.borrow_mut().setup(&self.laz_header);
        LAZIterator { reader: self, index: 0 }
    }

    #[cfg(feature = "std")]
    fn par_iter(&self, _pool_size: usize, _thread_id: usize) -> Self::FeatureIterator<'_> {
        self.reader.borrow().seek(self.header.offset_to_points as u64);
        self.state.borrow_mut().setup(&self.laz_header);
        self.iter()
    }
}
