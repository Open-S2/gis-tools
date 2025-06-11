use super::{
    ItemReader,
    arithmetic_decoder::{ArithmeticDecoder, ArithmeticModel},
    constants::{
        LASZIP_DECOMPRESS_SELECTIVE_ALL, LASZIP_DECOMPRESS_SELECTIVE_BYTE0,
        LASZIP_DECOMPRESS_SELECTIVE_CLASSIFICATION, LASZIP_DECOMPRESS_SELECTIVE_FLAGS,
        LASZIP_DECOMPRESS_SELECTIVE_GPS_TIME, LASZIP_DECOMPRESS_SELECTIVE_INTENSITY,
        LASZIP_DECOMPRESS_SELECTIVE_NIR, LASZIP_DECOMPRESS_SELECTIVE_POINT_SOURCE,
        LASZIP_DECOMPRESS_SELECTIVE_RGB, LASZIP_DECOMPRESS_SELECTIVE_SCAN_ANGLE,
        LASZIP_DECOMPRESS_SELECTIVE_USER_DATA, LASZIP_DECOMPRESS_SELECTIVE_WAVEPACKET,
        LASZIP_DECOMPRESS_SELECTIVE_Z,
    },
    integer_compressor::IntegerCompressor,
    v2::StreamingMedian5,
};
use crate::{
    parsers::{Buffer, BufferReader, RGBA, Reader},
    readers::{
        LASPoint, WavePacket,
        laz::constants::{NUMBER_RETURN_LEVEL_8CTX, NUMBER_RETURN_MAP_6CTX},
        util::{U64I64F64, ValueType64, i8_clamp, i16_quantize, u8_clamp, u8_fold, u32_zero_bit0},
    },
};
use alloc::{rc::Rc, vec, vec::Vec};
use core::cell::RefCell;

const LASZIP_GPSTIME_MULTI: i32 = 500;
const LASZIP_GPSTIME_MULTI_MINUS: i32 = -10;
const LASZIP_GPSTIME_MULTI_CODE_FULL: i32 = LASZIP_GPSTIME_MULTI - LASZIP_GPSTIME_MULTI_MINUS + 1;
const LASZIP_GPSTIME_MULTI_TOTAL: i32 = LASZIP_GPSTIME_MULTI - LASZIP_GPSTIME_MULTI_MINUS + 5;

/// LAS Point 1.4 context
#[derive(Debug)]
pub struct LASContextPoint14 {
    /// contexts last point
    pub last_item: LASPoint,
    /// If the context is unused
    pub unused: bool,
    /// last intensity
    pub last_intensity: [u16; 8],
    /// last x diff median5
    pub last_x_diff_median5: [StreamingMedian5; 12],
    /// last y diff median5
    pub last_y_diff_median5: [StreamingMedian5; 12],
    /// last height
    pub last_z: [i32; 8],
    /// last height diff median5
    pub m_changed_values: [Option<ArithmeticModel>; 8],
    /// last height diff median5
    pub m_scanner_channel: Option<ArithmeticModel>,
    /// last height diff median5
    pub m_number_of_returns: [Option<ArithmeticModel>; 16],
    /// last height diff median5
    pub m_return_number_gps_same: Option<ArithmeticModel>,
    /// last height diff median5
    pub m_return_number: [Option<ArithmeticModel>; 16],
    /// last height diff median5
    pub ic_dx: Option<IntegerCompressor<BufferReader>>,
    /// last height diff median5
    pub ic_dy: Option<IntegerCompressor<BufferReader>>,
    /// last height diff median5
    pub ic_z: Option<IntegerCompressor<BufferReader>>,
    /// arithmetic model classification
    pub m_classification: [Option<ArithmeticModel>; 64],
    /// arithmetic model flags
    pub m_flags: [Option<ArithmeticModel>; 64],
    /// arithmetic model user data
    pub m_user_data: [Option<ArithmeticModel>; 64],
    /// integer compression intensity
    pub ic_intensity: Option<IntegerCompressor<BufferReader>>,
    /// integer compression scan angle
    pub ic_scan_angle: Option<IntegerCompressor<BufferReader>>,
    /// integer compression source id
    pub ic_point_source_id: Option<IntegerCompressor<BufferReader>>,
    // GPS time stuff
    /// last gps value
    pub last: u32,
    /// next gps value
    pub next: u32,
    /// last gps time
    pub last_gpstime: [U64I64F64; 4],
    /// last gps time difference
    pub last_gpstime_diff: [i32; 4],
    /// multi extreme counter
    pub multi_extreme_counter: [i32; 4],
    /// gps time arithmetic model
    pub m_gpstime_multi: Option<ArithmeticModel>,
    /// gps time arithmetic model
    pub m_gpstime0diff: Option<ArithmeticModel>,
    /// gps time arithmetic model
    pub ic_gpstime: Option<IntegerCompressor<BufferReader>>,
}
impl Default for LASContextPoint14 {
    /// Create a new LASContextPoint14
    fn default() -> Self {
        Self {
            unused: false,
            last_item: LASPoint::default(),
            last_intensity: [0; 8],
            last_x_diff_median5: [StreamingMedian5::new(); 12],
            last_y_diff_median5: [StreamingMedian5::new(); 12],
            last_z: [0; 8],
            m_changed_values: [const { None }; 8],
            m_scanner_channel: None,
            m_number_of_returns: [const { None }; 16],
            m_return_number_gps_same: None,
            m_return_number: [const { None }; 16],
            ic_dx: None,
            ic_dy: None,
            ic_z: None,
            m_classification: [const { None }; 64],
            m_flags: [const { None }; 64],
            m_user_data: [const { None }; 64],
            ic_intensity: None,
            ic_scan_angle: None,
            ic_point_source_id: None,
            last: 0,
            next: 0,
            last_gpstime: [
                U64I64F64::new(0_u64, ValueType64::U64),
                U64I64F64::new(0_u64, ValueType64::U64),
                U64I64F64::new(0_u64, ValueType64::U64),
                U64I64F64::new(0_u64, ValueType64::U64),
            ],
            last_gpstime_diff: [0; 4],
            multi_extreme_counter: [0; 4],
            m_gpstime_multi: None,
            m_gpstime0diff: None,
            ic_gpstime: None,
        }
    }
}

/// LAS RGB 1.4 context
#[derive(Debug, Default)]
pub struct LAZContextRGB14 {
    /// true if unused
    pub unused: bool,
    /// last item data
    pub last_item: [u16; 3],
    // models
    /// bytes used
    pub m_byte_used: Option<ArithmeticModel>,
    /// RGB diff 0
    pub m_rgb_diff0: Option<ArithmeticModel>,
    /// RGB diff 1
    pub m_rgb_diff1: Option<ArithmeticModel>,
    /// RGB diff 2
    pub m_rgb_diff2: Option<ArithmeticModel>,
    /// RGB diff 3
    pub m_rgb_diff3: Option<ArithmeticModel>,
    /// RGB diff 4
    pub m_rgb_diff4: Option<ArithmeticModel>,
    /// RGB diff 5
    pub m_rgb_diff5: Option<ArithmeticModel>,
}

/// LAS RGB & NIR 1.4 context
#[derive(Debug, Default)]
pub struct LASContextRGBNir14 {
    /// true if unused
    pub unused: bool,
    /// last item data
    pub last_item: [u16; 4],
    // models
    /// bytes used
    pub m_rgb_bytes_used: Option<ArithmeticModel>,
    /// RGB diff 0
    pub m_rgb_diff0: Option<ArithmeticModel>,
    /// RGB diff 1
    pub m_rgb_diff1: Option<ArithmeticModel>,
    /// RGB diff 2
    pub m_rgb_diff2: Option<ArithmeticModel>,
    /// RGB diff 3
    pub m_rgb_diff3: Option<ArithmeticModel>,
    /// RGB diff 4
    pub m_rgb_diff4: Option<ArithmeticModel>,
    /// RGB diff 5
    pub m_rgb_diff5: Option<ArithmeticModel>,
    /// NIR bytes used
    pub m_nir_bytes_used: Option<ArithmeticModel>,
    /// NIR diff 0
    pub m_nir_diff0: Option<ArithmeticModel>,
    /// NIR diff 1
    pub m_nir_diff1: Option<ArithmeticModel>,
}

/// LAS WAVEPACKET 1.4 context
#[derive(Debug, Default)]
pub struct LASContextWavePacket14<T: Reader> {
    /// true if unused
    pub unused: bool,
    /// last item data
    pub last_item: [u8; 29],
    /// last difference
    pub last_diff32: i32,
    /// last offset difference
    pub sym_last_offset_diff: u32,
    // models
    /// packet index model
    pub m_packet_index: Option<ArithmeticModel>,
    /// offset diff models
    pub m_offset_diff: [Option<ArithmeticModel>; 4],
    /// packet size model
    pub ic_offset_diff: Option<IntegerCompressor<T>>,
    /// packet size integer compressor
    pub ic_packet_size: Option<IntegerCompressor<T>>,
    /// return point integer compressor
    pub ic_return_point: Option<IntegerCompressor<T>>,
    /// xyz integer compressor
    pub ic_xyz: Option<IntegerCompressor<T>>,
}

/// LAS BYTE 1.4 context
#[derive(Debug, Default)]
pub struct LAZContextByte14 {
    /// true if unused
    pub unused: bool,
    /// last item data
    pub last_item: Buffer,
    /// bytes model
    pub m_bytes: Vec<ArithmeticModel>,
}

/// Parse LAZ Point 1.4v3
#[derive(Debug)]
pub struct LAZPoint14v3Reader<T: Reader> {
    dec: Rc<RefCell<ArithmeticDecoder<T>>>,
    // streams
    instream_channel_returns_xy: Option<Rc<RefCell<BufferReader>>>,
    instream_z: Option<Rc<RefCell<BufferReader>>>,
    instream_classification: Option<Rc<RefCell<BufferReader>>>,
    instream_flags: Option<Rc<RefCell<BufferReader>>>,
    instream_intensity: Option<Rc<RefCell<BufferReader>>>,
    instream_scan_angle: Option<Rc<RefCell<BufferReader>>>,
    instream_user_data: Option<Rc<RefCell<BufferReader>>>,
    instream_point_source: Option<Rc<RefCell<BufferReader>>>,
    instream_gps_time: Option<Rc<RefCell<BufferReader>>>,
    // decoders
    dec_channel_returns_xy: Rc<RefCell<ArithmeticDecoder<BufferReader>>>,
    dec_z: Rc<RefCell<ArithmeticDecoder<BufferReader>>>,
    dec_classification: Rc<RefCell<ArithmeticDecoder<BufferReader>>>,
    dec_flags: Rc<RefCell<ArithmeticDecoder<BufferReader>>>,
    dec_intensity: Rc<RefCell<ArithmeticDecoder<BufferReader>>>,
    dec_scan_angle: Rc<RefCell<ArithmeticDecoder<BufferReader>>>,
    dec_user_data: Rc<RefCell<ArithmeticDecoder<BufferReader>>>,
    dec_point_source: Rc<RefCell<ArithmeticDecoder<BufferReader>>>,
    dec_gps_time: Rc<RefCell<ArithmeticDecoder<BufferReader>>>,
    // Point structure
    requested_z: bool,
    requested_classification: bool,
    requested_flags: bool,
    requested_intensity: bool,
    requested_scan_angle: bool,
    requested_user_data: bool,
    requested_point_source: bool,
    requested_gps_time: bool,
    // zero num_bytes and init booleans
    num_bytes_channel_returns_xy: u32,
    num_bytes_z: u32,
    num_bytes_classification: u32,
    num_bytes_flags: u32,
    num_bytes_intensity: u32,
    num_bytes_scan_angle: u32,
    num_bytes_user_data: u32,
    num_bytes_point_source: u32,
    num_bytes_gps_time: u32,
    changed_z: bool,
    changed_classification: bool,
    changed_flags: bool,
    changed_intensity: bool,
    changed_scan_angle: bool,
    changed_user_data: bool,
    changed_point_source: bool,
    changed_gps_time: bool,
    bytes: Option<Buffer>,
    num_bytes_allocated: u32,
    current_context: u32,
    contexts: [LASContextPoint14; 4],
}
impl<T: Reader> LAZPoint14v3Reader<T> {
    /// Create a new LAZPoint14v3Reader
    pub fn new(dec: Rc<RefCell<ArithmeticDecoder<T>>>, decompress_selective: Option<u32>) -> Self {
        let decompress_selective = decompress_selective.unwrap_or(LASZIP_DECOMPRESS_SELECTIVE_ALL);

        let tmp_decoders = Rc::new(RefCell::new(ArithmeticDecoder::new(Rc::new(RefCell::new(
            BufferReader::new(vec![0_u8; 0]),
        )))));
        let mut point_read = LAZPoint14v3Reader {
            dec,
            // streams
            instream_channel_returns_xy: None,
            instream_z: None,
            instream_classification: None,
            instream_flags: None,
            instream_intensity: None,
            instream_scan_angle: None,
            instream_user_data: None,
            instream_point_source: None,
            instream_gps_time: None,
            // decoders
            dec_channel_returns_xy: tmp_decoders.clone(),
            dec_z: tmp_decoders.clone(),
            dec_classification: tmp_decoders.clone(),
            dec_flags: tmp_decoders.clone(),
            dec_intensity: tmp_decoders.clone(),
            dec_scan_angle: tmp_decoders.clone(),
            dec_user_data: tmp_decoders.clone(),
            dec_point_source: tmp_decoders.clone(),
            dec_gps_time: tmp_decoders.clone(),
            // Point structure
            requested_z: false,
            requested_classification: false,
            requested_flags: false,
            requested_intensity: false,
            requested_scan_angle: false,
            requested_user_data: false,
            requested_point_source: false,
            requested_gps_time: false,
            // zero num_bytes and init booleans
            num_bytes_channel_returns_xy: 0,
            num_bytes_z: 0,
            num_bytes_classification: 0,
            num_bytes_flags: 0,
            num_bytes_intensity: 0,
            num_bytes_scan_angle: 0,
            num_bytes_user_data: 0,
            num_bytes_point_source: 0,
            num_bytes_gps_time: 0,
            changed_z: false,
            changed_classification: false,
            changed_flags: false,
            changed_intensity: false,
            changed_scan_angle: false,
            changed_user_data: false,
            changed_point_source: false,
            changed_gps_time: false,
            bytes: None,
            num_bytes_allocated: 0,
            current_context: 0,
            contexts: [
                LASContextPoint14::default(),
                LASContextPoint14::default(),
                LASContextPoint14::default(),
                LASContextPoint14::default(),
            ],
        };

        // mark the four scanner channel contexts as uninitialized
        for context in &mut point_read.contexts {
            context.m_changed_values[0] = None;
        }

        point_read.requested_z = (decompress_selective & LASZIP_DECOMPRESS_SELECTIVE_Z) != 0;
        point_read.requested_classification =
            (decompress_selective & LASZIP_DECOMPRESS_SELECTIVE_CLASSIFICATION) != 0;
        point_read.requested_flags =
            (decompress_selective & LASZIP_DECOMPRESS_SELECTIVE_FLAGS) != 0;
        point_read.requested_intensity =
            (decompress_selective & LASZIP_DECOMPRESS_SELECTIVE_INTENSITY) != 0;
        point_read.requested_scan_angle =
            (decompress_selective & LASZIP_DECOMPRESS_SELECTIVE_SCAN_ANGLE) != 0;
        point_read.requested_user_data =
            (decompress_selective & LASZIP_DECOMPRESS_SELECTIVE_USER_DATA) != 0;
        point_read.requested_point_source =
            (decompress_selective & LASZIP_DECOMPRESS_SELECTIVE_POINT_SOURCE) != 0;
        point_read.requested_gps_time =
            (decompress_selective & LASZIP_DECOMPRESS_SELECTIVE_GPS_TIME) != 0;

        point_read
    }

    /// Read the GPS Time
    fn read_gps_time(&mut self) {
        let Self { contexts, current_context, .. } = self;
        let current_context = *current_context as usize;
        let mut multi: i32;
        if contexts[current_context].last_gpstime_diff[contexts[current_context].last as usize] == 0
        {
            // if the last integer difference was zero
            multi = self
                .dec_gps_time
                .borrow_mut()
                .decode_symbol(contexts[current_context].m_gpstime0diff.as_mut().unwrap())
                as i32;
            if multi == 0 {
                // the difference can be represented with 32 bits
                contexts[current_context].last_gpstime_diff
                    [contexts[current_context].last as usize] =
                    contexts[current_context].ic_gpstime.as_mut().unwrap().decompress(0, 0);
                contexts[current_context].last_gpstime[contexts[current_context].last as usize]
                    .set_i64(
                        contexts[current_context].last_gpstime
                            [contexts[current_context].last as usize]
                            .i64()
                            + contexts[current_context].last_gpstime_diff
                                [contexts[current_context].last as usize]
                                as i64,
                    );
                contexts[current_context].multi_extreme_counter
                    [contexts[current_context].last as usize] = 0;
            } else if multi == 1 {
                // the difference is huge
                contexts[current_context].next = (contexts[current_context].next + 1) & 3;
                contexts[current_context].last_gpstime[contexts[current_context].next as usize]
                    .set_u64(
                        contexts[current_context].ic_gpstime.as_mut().unwrap().decompress(
                            (contexts[current_context].last_gpstime
                                [contexts[current_context].last as usize]
                                .u64()
                                >> 32) as i32,
                            8,
                        ) as u64,
                    );
                contexts[current_context].last_gpstime[contexts[current_context].next as usize]
                    .set_u64(
                        contexts[current_context].last_gpstime
                            [contexts[current_context].next as usize]
                            .u64()
                            << 32,
                    );
                let input_int = self.dec_gps_time.borrow_mut().read_int();
                contexts[current_context].last_gpstime[contexts[current_context].next as usize]
                    .set_u64(
                        contexts[current_context].last_gpstime
                            [contexts[current_context].next as usize]
                            .u64()
                            | input_int as u64,
                    );
                contexts[current_context].last = contexts[current_context].next;
                contexts[current_context].last_gpstime_diff
                    [contexts[current_context].last as usize] = 0;
                contexts[current_context].multi_extreme_counter
                    [contexts[current_context].last as usize] = 0;
            }
            // we switch to another sequence
            else {
                contexts[current_context].last =
                    ((contexts[current_context].last as i32 + multi - 1) & 3) as u32;
                self.read_gps_time();
            }
        } else {
            multi = self
                .dec_gps_time
                .borrow_mut()
                .decode_symbol(contexts[current_context].m_gpstime_multi.as_mut().unwrap())
                as i32;
            if multi == 1 {
                contexts[current_context].last_gpstime[contexts[current_context].last as usize]
                    .set_i64(
                        contexts[current_context].last_gpstime
                            [contexts[current_context].last as usize]
                            .i64()
                            + contexts[current_context].ic_gpstime.as_mut().unwrap().decompress(
                                contexts[current_context].last_gpstime_diff
                                    [contexts[current_context].last as usize],
                                1,
                            ) as i64,
                    );
                contexts[current_context].multi_extreme_counter
                    [contexts[current_context].last as usize] = 0;
            } else if multi < LASZIP_GPSTIME_MULTI_CODE_FULL {
                let gpstime_diff: i32; // I32
                if multi == 0 {
                    gpstime_diff =
                        contexts[current_context].ic_gpstime.as_mut().unwrap().decompress(0, 7);
                    contexts[current_context].multi_extreme_counter
                        [contexts[current_context].last as usize] += 1;
                    if contexts[current_context].multi_extreme_counter
                        [contexts[current_context].last as usize]
                        > 3
                    {
                        contexts[current_context].last_gpstime_diff
                            [contexts[current_context].last as usize] = gpstime_diff;
                        contexts[current_context].multi_extreme_counter
                            [contexts[current_context].last as usize] = 0;
                    }
                } else if multi < LASZIP_GPSTIME_MULTI {
                    if multi < 10 {
                        gpstime_diff =
                            contexts[current_context].ic_gpstime.as_mut().unwrap().decompress(
                                multi
                                    * contexts[current_context].last_gpstime_diff
                                        [contexts[current_context].last as usize],
                                2,
                            );
                    } else {
                        gpstime_diff =
                            contexts[current_context].ic_gpstime.as_mut().unwrap().decompress(
                                multi
                                    * contexts[current_context].last_gpstime_diff
                                        [contexts[current_context].last as usize],
                                3,
                            );
                    }
                } else if multi == LASZIP_GPSTIME_MULTI {
                    gpstime_diff =
                        contexts[current_context].ic_gpstime.as_mut().unwrap().decompress(
                            LASZIP_GPSTIME_MULTI
                                * contexts[current_context].last_gpstime_diff
                                    [contexts[current_context].last as usize],
                            4,
                        );
                    contexts[current_context].multi_extreme_counter
                        [contexts[current_context].last as usize] += 1;
                    if contexts[current_context].multi_extreme_counter
                        [contexts[current_context].last as usize]
                        > 3
                    {
                        contexts[current_context].last_gpstime_diff
                            [contexts[current_context].last as usize] = gpstime_diff;
                        contexts[current_context].multi_extreme_counter
                            [contexts[current_context].last as usize] = 0;
                    }
                } else {
                    multi = LASZIP_GPSTIME_MULTI - multi;
                    if multi > LASZIP_GPSTIME_MULTI_MINUS {
                        gpstime_diff =
                            contexts[current_context].ic_gpstime.as_mut().unwrap().decompress(
                                multi
                                    * contexts[current_context].last_gpstime_diff
                                        [contexts[current_context].last as usize],
                                5,
                            );
                    } else {
                        gpstime_diff =
                            contexts[current_context].ic_gpstime.as_mut().unwrap().decompress(
                                LASZIP_GPSTIME_MULTI_MINUS
                                    * contexts[current_context].last_gpstime_diff
                                        [contexts[current_context].last as usize],
                                6,
                            );
                        contexts[current_context].multi_extreme_counter
                            [contexts[current_context].last as usize] += 1;
                        if contexts[current_context].multi_extreme_counter
                            [contexts[current_context].last as usize]
                            > 3
                        {
                            contexts[current_context].last_gpstime_diff
                                [contexts[current_context].last as usize] = gpstime_diff;
                            contexts[current_context].multi_extreme_counter
                                [contexts[current_context].last as usize] = 0;
                        }
                    }
                }
                contexts[current_context].last_gpstime[contexts[current_context].last as usize]
                    .set_i64(
                        contexts[current_context].last_gpstime
                            [contexts[current_context].last as usize]
                            .i64()
                            + gpstime_diff as i64,
                    );
            } else if multi == LASZIP_GPSTIME_MULTI_CODE_FULL {
                contexts[current_context].next = (contexts[current_context].next + 1) & 3;
                contexts[current_context].last_gpstime[contexts[current_context].next as usize]
                    .set_u64(
                        contexts[current_context].ic_gpstime.as_mut().unwrap().decompress(
                            (contexts[current_context].last_gpstime
                                [contexts[current_context].last as usize]
                                .u64()
                                >> 32) as i32,
                            8,
                        ) as u64,
                    );
                contexts[current_context].last_gpstime[contexts[current_context].next as usize]
                    .set_u64(
                        contexts[current_context].last_gpstime
                            [contexts[current_context].next as usize]
                            .u64()
                            << 32,
                    );
                contexts[current_context].last_gpstime[contexts[current_context].next as usize]
                    .set_u64(
                        contexts[current_context].last_gpstime
                            [contexts[current_context].next as usize]
                            .u64()
                            | (self.dec_gps_time.borrow_mut().read_int()) as u64,
                    );
                contexts[current_context].last = contexts[current_context].next;
                contexts[current_context].last_gpstime_diff
                    [contexts[current_context].last as usize] = 0;
                contexts[current_context].multi_extreme_counter
                    [contexts[current_context].last as usize] = 0;
            } else if multi >= LASZIP_GPSTIME_MULTI_CODE_FULL {
                contexts[current_context].last = ((contexts[current_context].last as i32 + multi
                    - LASZIP_GPSTIME_MULTI_CODE_FULL)
                    & 3) as u32;
                self.read_gps_time();
            }
        }
    }

    /// @param context - the current context
    /// @param item - the current item to be read from
    fn create_and_init_models_and_decompressors(&mut self, context: usize, item: &LASPoint) {
        // should only be called when context is unused
        assert!(self.contexts[context].unused);

        // first create all entropy models and integer decompressors (if needed)
        if self.contexts[context].m_changed_values[0].is_none() {
            // for the channel_returns_XY layer
            for i in 0..8 {
                self.contexts[context].m_changed_values[i] = Some(ArithmeticModel::new(128, false));
            }
            self.contexts[context].m_scanner_channel = Some(ArithmeticModel::new(3, false));
            for i in 0..16 {
                self.contexts[context].m_number_of_returns[i] = None;
                self.contexts[context].m_return_number[i] = None;
            }
            self.contexts[context].m_return_number_gps_same = Some(ArithmeticModel::new(13, false));

            self.contexts[context].ic_dx = Some(IntegerCompressor::new(
                self.dec_channel_returns_xy.clone(),
                Some(32),
                Some(2),
                None,
                None,
            )); // 32 bits, 2 context
            self.contexts[context].ic_dy = Some(IntegerCompressor::new(
                self.dec_channel_returns_xy.clone(),
                Some(32),
                Some(22),
                None,
                None,
            )); // 32 bits, 22 contexts

            // for the Z layer
            self.contexts[context].ic_z =
                Some(IntegerCompressor::new(self.dec_z.clone(), Some(32), Some(20), None, None)); // 32 bits, 20 contexts

            // for the classification layer, flags layer, and user_data layer
            for i in 0..64 {
                self.contexts[context].m_classification[i] = None;
                self.contexts[context].m_flags[i] = None;
                self.contexts[context].m_user_data[i] = None;
            }
            // for the intensity layer
            self.contexts[context].ic_intensity = Some(IntegerCompressor::new(
                self.dec_intensity.clone(),
                Some(16),
                Some(4),
                None,
                None,
            ));
            // for the scan_angle layer
            self.contexts[context].ic_scan_angle = Some(IntegerCompressor::new(
                self.dec_scan_angle.clone(),
                Some(16),
                Some(2),
                None,
                None,
            ));
            // for the point_source_id layer
            self.contexts[context].ic_point_source_id = Some(IntegerCompressor::new(
                self.dec_point_source.clone(),
                Some(16),
                None,
                None,
                None,
            ));
            // for the gps_time layer
            self.contexts[context].m_gpstime_multi =
                Some(ArithmeticModel::new(LASZIP_GPSTIME_MULTI_TOTAL as u32, false));
            self.contexts[context].m_gpstime0diff = Some(ArithmeticModel::new(5, false));
            self.contexts[context].ic_gpstime = Some(IntegerCompressor::new(
                self.dec_gps_time.clone(),
                Some(32),
                Some(9),
                None,
                None,
            )); // 32 bits, 9 contexts
        }

        // for the channel_returns_XY layer
        for i in 0..8 {
            self.contexts[context].m_changed_values[i].as_mut().unwrap().init(None);
        }
        self.contexts[context].m_scanner_channel.as_mut().unwrap().init(None);
        for i in 0..16 {
            self.contexts[context].m_number_of_returns[i] = None;
            self.contexts[context].m_return_number[i] = None;
        }
        self.contexts[context].m_return_number_gps_same.as_mut().unwrap().init(None);
        self.contexts[context].ic_dx.as_mut().unwrap().init_decompressor();
        self.contexts[context].ic_dy.as_mut().unwrap().init_decompressor();
        for i in 0..12 {
            self.contexts[context].last_x_diff_median5[i].init();
            self.contexts[context].last_y_diff_median5[i].init();
        }
        // for the Z layer
        self.contexts[context].ic_z.as_mut().unwrap().init_decompressor();
        for i in 0..8 {
            self.contexts[context].last_z[i] = item.z;
        }
        // for the classification layer, flags layer, and user_data layer
        for i in 0..64 {
            self.contexts[context].m_classification[i] = None;
            self.contexts[context].m_flags[i] = None;
            self.contexts[context].m_user_data[i] = None;
        }

        // for the intensity layer
        self.contexts[context].ic_intensity.as_mut().unwrap().init_decompressor();
        for i in 0..8 {
            self.contexts[context].last_intensity[i] = item.intensity;
        }
        // for the scan_angle layer
        self.contexts[context].ic_scan_angle.as_mut().unwrap().init_decompressor();
        // for the point_source_id layer
        self.contexts[context].ic_point_source_id.as_mut().unwrap().init_decompressor();
        // for the gps_time layer
        self.contexts[context].m_gpstime_multi.as_mut().unwrap().init(None);
        self.contexts[context].m_gpstime0diff.as_mut().unwrap().init(None);
        self.contexts[context].ic_gpstime.as_mut().unwrap().init_decompressor();
        self.contexts[context].last = 0;
        self.contexts[context].next = 0;
        self.contexts[context].last_gpstime_diff[0] = 0;
        self.contexts[context].last_gpstime_diff[1] = 0;
        self.contexts[context].last_gpstime_diff[2] = 0;
        self.contexts[context].last_gpstime_diff[3] = 0;
        self.contexts[context].multi_extreme_counter[0] = 0;
        self.contexts[context].multi_extreme_counter[1] = 0;
        self.contexts[context].multi_extreme_counter[2] = 0;
        self.contexts[context].multi_extreme_counter[3] = 0;
        self.contexts[context].last_gpstime[0].set_f64(item.gps_time.unwrap_or_default());
        self.contexts[context].last_gpstime[1].set_f64(0.);
        self.contexts[context].last_gpstime[2].set_f64(0.);
        self.contexts[context].last_gpstime[3].set_f64(0.);
        // init current context from last item
        self.contexts[context].last_item = item.clone();
        self.contexts[context].last_item.gps_time_change = Some(0);
        self.contexts[context].unused = false;
    }
}
impl<T: Reader> ItemReader for LAZPoint14v3Reader<T> {
    fn init<R: Reader>(&mut self, item: &R, point: &mut LASPoint, context: &mut u32) {
        point.inject_point14(item, 0, true);
        {
            // for layered compression 'dec' only hands over the stream
            let dec = self.dec.borrow_mut();
            let reader = dec.reader.borrow();

            // how many bytes do we need to read
            let mut num_bytes = self.num_bytes_channel_returns_xy;
            if self.requested_z {
                num_bytes += self.num_bytes_z;
            }
            if self.requested_classification {
                num_bytes += self.num_bytes_classification;
            }
            if self.requested_flags {
                num_bytes += self.num_bytes_flags;
            }
            if self.requested_intensity {
                num_bytes += self.num_bytes_intensity;
            }
            if self.requested_scan_angle {
                num_bytes += self.num_bytes_scan_angle;
            }
            if self.requested_user_data {
                num_bytes += self.num_bytes_user_data;
            }
            if self.requested_point_source {
                num_bytes += self.num_bytes_point_source;
            }
            if self.requested_gps_time {
                num_bytes += self.num_bytes_gps_time;
            }
            // make sure the buffer is sufficiently large
            if num_bytes > self.num_bytes_allocated {
                //   self.bytes = new DataView(new ArrayBuffer(num_bytes));
                self.bytes = Some(Buffer::new(vec![0; num_bytes as usize]));
                self.num_bytes_allocated = num_bytes;
            }
            // load the requested bytes and init the corresponding instreams and decoders
            // num_bytes = 0;
            let bytes = reader.seek_slice(self.num_bytes_channel_returns_xy as usize);
            let buffer = Buffer::new(bytes.clone());
            let br = Rc::new(RefCell::new(BufferReader::new(bytes)));
            self.bytes = Some(buffer);
            self.instream_channel_returns_xy = Some(br.clone());
            let mut decoder = ArithmeticDecoder::new(br.clone());
            decoder.init(true);
            self.dec_channel_returns_xy = Rc::new(RefCell::new(decoder));
            // num_bytes += self.num_bytes_channel_returns_xy;

            if self.requested_z {
                if self.num_bytes_z != 0 {
                    let bytes = reader.seek_slice(self.num_bytes_z as usize);
                    let buffer = Buffer::new(bytes.clone());
                    let br = Rc::new(RefCell::new(BufferReader::new(bytes)));
                    self.bytes = Some(buffer);
                    self.instream_z = Some(br.clone());
                    let mut decoder = ArithmeticDecoder::new(br.clone());
                    decoder.init(true);
                    self.dec_z = Rc::new(RefCell::new(decoder));
                    // num_bytes += self.num_bytes_z;
                    self.changed_z = true;
                } else {
                    self.instream_z = None;
                    self.changed_z = false;
                }
            } else {
                if self.num_bytes_z != 0 {
                    // skip num_bytes_z
                    reader.seek(reader.tell() + self.num_bytes_z as u64);
                }
                self.changed_z = false;
            }

            if self.requested_classification {
                if self.num_bytes_classification != 0 {
                    let bytes = reader.seek_slice(self.num_bytes_classification as usize);
                    let buffer = Buffer::new(bytes.clone());
                    let br = Rc::new(RefCell::new(BufferReader::new(bytes)));
                    self.bytes = Some(buffer);
                    self.instream_classification = Some(br.clone());
                    let mut decoder = ArithmeticDecoder::new(br.clone());
                    decoder.init(true);
                    self.dec_classification = Rc::new(RefCell::new(decoder));
                    // num_bytes += self.num_bytes_classification;
                    self.changed_classification = true;
                } else {
                    self.instream_classification = None;
                    self.changed_classification = false;
                }
            } else {
                if self.num_bytes_classification != 0 {
                    reader.seek(reader.tell() + self.num_bytes_classification as u64);
                }
                self.changed_classification = false;
            }

            if self.requested_flags {
                if self.num_bytes_flags != 0 {
                    let bytes = reader.seek_slice(self.num_bytes_flags as usize);
                    let buffer = Buffer::new(bytes.clone());
                    let br = Rc::new(RefCell::new(BufferReader::new(bytes)));
                    self.bytes = Some(buffer);
                    self.instream_flags = Some(br.clone());
                    let mut decoder = ArithmeticDecoder::new(br.clone());
                    decoder.init(true);
                    self.dec_flags = Rc::new(RefCell::new(decoder));
                    // num_bytes += self.num_bytes_flags;
                    self.changed_flags = true;
                } else {
                    self.instream_flags = None;
                    self.changed_flags = false;
                }
            } else {
                if self.num_bytes_flags != 0 {
                    reader.seek(reader.tell() + self.num_bytes_flags as u64);
                }
                self.changed_flags = false;
            }

            if self.requested_intensity {
                if self.num_bytes_intensity != 0 {
                    let bytes = reader.seek_slice(self.num_bytes_intensity as usize);
                    let buffer = Buffer::new(bytes.clone());
                    let br = Rc::new(RefCell::new(BufferReader::new(bytes)));
                    self.bytes = Some(buffer);
                    self.instream_intensity = Some(br.clone());
                    let mut decoder = ArithmeticDecoder::new(br.clone());
                    decoder.init(true);
                    self.dec_intensity = Rc::new(RefCell::new(decoder));
                    // num_bytes += self.num_bytes_intensity;
                    self.changed_intensity = true;
                } else {
                    self.instream_intensity = None;
                    self.changed_intensity = false;
                }
            } else {
                if self.num_bytes_intensity != 0 {
                    reader.seek(reader.tell() + self.num_bytes_intensity as u64);
                }
                self.changed_intensity = false;
            }

            if self.requested_scan_angle {
                if self.num_bytes_scan_angle != 0 {
                    let bytes = reader.seek_slice(self.num_bytes_scan_angle as usize);
                    let buffer = Buffer::new(bytes.clone());
                    let br = Rc::new(RefCell::new(BufferReader::new(bytes)));
                    self.bytes = Some(buffer);
                    self.instream_scan_angle = Some(br.clone());
                    let mut decoder = ArithmeticDecoder::new(br.clone());
                    decoder.init(true);
                    self.dec_scan_angle = Rc::new(RefCell::new(decoder));
                    // num_bytes += self.num_bytes_scan_angle;
                    self.changed_scan_angle = true;
                } else {
                    self.instream_scan_angle = None;
                    self.changed_scan_angle = false;
                }
            } else {
                if self.num_bytes_scan_angle != 0 {
                    reader.seek(reader.tell() + self.num_bytes_scan_angle as u64);
                }
                self.changed_scan_angle = false;
            }

            if self.requested_user_data {
                if self.num_bytes_user_data != 0 {
                    let bytes = reader.seek_slice(self.num_bytes_user_data as usize);
                    let buffer = Buffer::new(bytes.clone());
                    let br = Rc::new(RefCell::new(BufferReader::new(bytes)));
                    self.bytes = Some(buffer);
                    self.instream_user_data = Some(br.clone());
                    let mut decoder = ArithmeticDecoder::new(br.clone());
                    decoder.init(true);
                    self.dec_user_data = Rc::new(RefCell::new(decoder));
                    // num_bytes += self.num_bytes_user_data;
                    self.changed_user_data = true;
                } else {
                    self.instream_user_data = None;
                    self.changed_user_data = false;
                }
            } else {
                if self.num_bytes_user_data != 0 {
                    reader.seek(reader.tell() + self.num_bytes_user_data as u64);
                }
                self.changed_user_data = false;
            }

            if self.requested_point_source {
                if self.num_bytes_point_source != 0 {
                    let bytes = reader.seek_slice(self.num_bytes_point_source as usize);
                    let buffer = Buffer::new(bytes.clone());
                    let br = Rc::new(RefCell::new(BufferReader::new(bytes)));
                    self.bytes = Some(buffer);
                    self.instream_point_source = Some(br.clone());
                    let mut decoder = ArithmeticDecoder::new(br.clone());
                    decoder.init(true);
                    self.dec_point_source = Rc::new(RefCell::new(decoder));
                    // num_bytes += self.num_bytes_point_source;
                    self.changed_point_source = true;
                } else {
                    self.instream_point_source = None;
                    self.changed_point_source = false;
                }
            } else {
                if self.num_bytes_point_source != 0 {
                    reader.seek(reader.tell() + self.num_bytes_point_source as u64);
                }
                self.changed_point_source = false;
            }

            if self.requested_gps_time {
                if self.num_bytes_gps_time != 0 {
                    let bytes = reader.seek_slice(self.num_bytes_gps_time as usize);
                    let buffer = Buffer::new(bytes.clone());
                    let br = Rc::new(RefCell::new(BufferReader::new(bytes)));
                    self.bytes = Some(buffer);
                    self.instream_gps_time = Some(br.clone());
                    let mut decoder = ArithmeticDecoder::new(br.clone());
                    decoder.init(true);
                    self.dec_gps_time = Rc::new(RefCell::new(decoder));
                    // num_bytes += self.num_bytes_gps_time;
                    self.changed_gps_time = true;
                } else {
                    self.instream_gps_time = None;
                    self.changed_gps_time = false;
                }
            } else {
                if self.num_bytes_gps_time != 0 {
                    reader.seek(reader.tell() + self.num_bytes_gps_time as u64);
                }
                self.changed_gps_time = false;
            }
        }

        // mark the four scanner channel contexts as unused
        for c in 0..4 {
            self.contexts[c].unused = true;
        }
        // set scanner channel as current context
        self.current_context = point.scanner_channel as u32;
        *context = self.current_context; // the POINT14 reader sets context for all other items

        // create and init models and decompressors
        self.create_and_init_models_and_decompressors(self.current_context as usize, point);
    }

    fn read(&mut self, item: &mut LASPoint, context: &mut u32) {
        // get last
        let changed_values: i32;
        {
            let last_item = &mut self.contexts[self.current_context as usize].last_item;
            ////////////////////////////////////////
            // decompress returns_XY layer
            ////////////////////////////////////////
            // create single (3) / first (1) / last (2) / intermediate (0) context from last point return
            let mut lpr: i32 = if last_item.return_number == 1 { 1 } else { 0 }; // (I32) first?
            lpr += if last_item.return_number >= last_item.number_of_returns { 2 } else { 0 }; // last?
            // add info whether the GPS time changed in the last return to the context
            lpr += if last_item.gps_time_change.unwrap_or_default() != 0 { 4 } else { 0 };
            // decompress which values have changed with last point return context
            changed_values = self.dec_channel_returns_xy.borrow_mut().decode_symbol(
                self.contexts[self.current_context as usize].m_changed_values[lpr as usize]
                    .as_mut()
                    .unwrap(),
            ) as i32; // I32
        }

        // if scanner channel has changed
        if (changed_values & (1 << 6)) != 0 {
            // U32
            let diff = self.dec_channel_returns_xy.borrow_mut().decode_symbol(
                self.contexts[self.current_context as usize].m_scanner_channel.as_mut().unwrap(),
            ); // curr = last + (sym + 1)
            // U32
            let scanner_channel = (self.current_context + diff + 1) % 4;
            // maybe create and init entropy models and integer compressors
            if self.contexts[scanner_channel as usize].unused {
                // create and init entropy models and integer decompressors
                let last_item = self.contexts[self.current_context as usize].last_item.clone();
                self.create_and_init_models_and_decompressors(scanner_channel as usize, &last_item);
            }
            // switch context to current scanner channel
            self.current_context = scanner_channel;
            *context = self.current_context; // the POINT14 reader sets context for all other items
            // get last for new context
            let last_item = &mut self.contexts[self.current_context as usize].last_item;
            last_item.scanner_channel = scanner_channel as u8;
        }
        // determine changed attributes
        let point_source_change = (changed_values & (1 << 5)) != 0;
        let gps_time_change = (changed_values & (1 << 4)) != 0;
        let scan_angle_change = (changed_values & (1 << 3)) != 0;
        // get last return counts
        {
            let last_item = &mut self.contexts[self.current_context as usize].last_item;
            let last_n = last_item.number_of_returns; // U32
            let last_r = last_item.return_number; // U32
            // if number of returns is different we decompress it
            let n: u32; // U32
            if (changed_values & (1 << 2)) != 0 {
                if self.contexts[self.current_context as usize].m_number_of_returns[last_n as usize]
                    .is_none()
                {
                    let mut model = ArithmeticModel::new(16, false);
                    model.init(None);
                    self.contexts[self.current_context as usize].m_number_of_returns
                        [last_n as usize] = Some(model);
                }
                n = self.dec_channel_returns_xy.borrow_mut().decode_symbol(
                    self.contexts[self.current_context as usize].m_number_of_returns
                        [last_n as usize]
                        .as_mut()
                        .unwrap(),
                );
                last_item.number_of_returns = n as u8;
            } else {
                n = last_n as u32;
            }
            // how is the return number different
            let r: u32; // U32
            if (changed_values & 3) == 0 {
                // same return number
                r = last_r as u32;
            } else if (changed_values & 3) == 1 {
                // return number plus 1 mod 16
                r = (last_r as u32 + 1) % 16;
                last_item.return_number = r as u8;
            } else if (changed_values & 3) == 2 {
                // return number minus 1 mod 16
                r = (last_r as u32 + 15) % 16;
                last_item.return_number = r as u8;
            } else {
                // the return number difference is bigger than +1 / -1 so we decompress how it is different
                if gps_time_change {
                    // if the GPS time has changed
                    if self.contexts[self.current_context as usize].m_return_number[last_r as usize]
                        .is_none()
                    {
                        let mut model = ArithmeticModel::new(16, false);
                        model.init(None);
                        self.contexts[self.current_context as usize].m_return_number
                            [last_r as usize] = Some(model);
                    }
                    r = self.dec_channel_returns_xy.borrow_mut().decode_symbol(
                        self.contexts[self.current_context as usize].m_return_number
                            [last_r as usize]
                            .as_mut()
                            .unwrap(),
                    );
                }
                // if the GPS time has not changed
                else {
                    // I32
                    let sym = self.dec_channel_returns_xy.borrow_mut().decode_symbol(
                        self.contexts[self.current_context as usize]
                            .m_return_number_gps_same
                            .as_mut()
                            .unwrap(),
                    ) as i32;
                    r = ((last_r as i32 + (sym + 2)) % 16) as u32;
                }
                last_item.return_number = r as u8;
            }
            // set legacy return counts and number of returns
            if n > 7 {
                if r > 6 {
                    if r >= n {
                        last_item.legacy_return_number = 7;
                    } else {
                        last_item.legacy_return_number = 6;
                    }
                } else {
                    last_item.legacy_return_number = r as u8;
                }
                last_item.legacy_number_of_returns = 7;
            } else {
                last_item.legacy_return_number = r as u8;
                last_item.legacy_number_of_returns = n as u8;
            }
            // get return map m and return level l context for current point
            let m = NUMBER_RETURN_MAP_6CTX[n as usize][r as usize] as u32; // U32
            let l = NUMBER_RETURN_LEVEL_8CTX[n as usize][r as usize]; // U32
            // create single (3) / first (1) / last (2) / intermediate (0) return context for current point
            let mut cpr = if r == 1 { 2 } else { 0 }; // (I32) first ?
            cpr += if r >= n { 1 } else { 0 }; // last ?
            let mut k_bits: u32; // U32
            let mut median: i32;
            let mut diff: i32; // I32
            let dec_index = (m << 1) | (if gps_time_change { 1 } else { 0 });
            // decompress X coordinate
            median = self.contexts[self.current_context as usize].last_x_diff_median5
                [dec_index as usize]
                .get();
            diff = self.contexts[self.current_context as usize]
                .ic_dx
                .as_mut()
                .unwrap()
                .decompress(median, if n == 1 { 1 } else { 0 });
            last_item.x += diff;
            self.contexts[self.current_context as usize].last_x_diff_median5[dec_index as usize]
                .add(diff);
            // decompress Y coordinate
            median = self.contexts[self.current_context as usize].last_y_diff_median5
                [dec_index as usize]
                .get();
            k_bits = self.contexts[self.current_context as usize].ic_dx.as_mut().unwrap().get_k();
            diff = self.contexts[self.current_context as usize].ic_dy.as_mut().unwrap().decompress(
                median,
                (if n == 1 { 1 } else { 0 })
                    + (if k_bits < 20 { u32_zero_bit0(k_bits) } else { 20 }),
            );
            last_item.y += diff;
            self.contexts[self.current_context as usize].last_y_diff_median5[dec_index as usize]
                .add(diff);
            ////////////////////////////////////////
            // decompress Z layer (if changed and requested)
            ////////////////////////////////////////
            if self.changed_z {
                // if the Z coordinate should be decompressed and changes within this chunk
                k_bits = (self.contexts[self.current_context as usize]
                    .ic_dx
                    .as_mut()
                    .unwrap()
                    .get_k()
                    + self.contexts[self.current_context as usize].ic_dy.as_mut().unwrap().get_k())
                    / 2;
                last_item.z =
                    self.contexts[self.current_context as usize].ic_z.as_mut().unwrap().decompress(
                        self.contexts[self.current_context as usize].last_z[l as usize],
                        (if n == 1 { 1 } else { 0 })
                            + (if k_bits < 18 { u32_zero_bit0(k_bits) } else { 18 }),
                    );
                self.contexts[self.current_context as usize].last_z[l as usize] = last_item.z;
            }
            ////////////////////////////////////////
            // decompress classifications layer (if changed and requested)
            ////////////////////////////////////////
            if self.changed_classification {
                // if the classification should be decompressed and changes within this chunk
                let last_classification = last_item.classification as u32; // U32
                let ccc =
                    (((last_classification & 0x1f) << 1) + (if cpr == 3 { 1 } else { 0 })) as i32; // I32
                if self.contexts[self.current_context as usize].m_classification[ccc as usize]
                    .is_none()
                {
                    let mut model = ArithmeticModel::new(256, false);
                    model.init(None);
                    self.contexts[self.current_context as usize].m_classification[ccc as usize] =
                        Some(model);
                }
                last_item.classification = self.dec_classification.borrow_mut().decode_symbol(
                    self.contexts[self.current_context as usize].m_classification[ccc as usize]
                        .as_mut()
                        .unwrap(),
                ) as u8;
                // update the legacy copy
                if last_item.classification < 32 {
                    last_item.legacy_classification = last_item.classification;
                } else {
                    last_item.legacy_classification = 0;
                }
            }
            ////////////////////////////////////////
            // decompress flags layer (if changed and requested)
            ////////////////////////////////////////
            if self.changed_flags {
                // if the flags should be decompressed and change within this chunk
                // U32
                let last_flags = ((last_item.edge_of_flight_line as u32) << 5)
                    | ((last_item.scan_direction_flag as u32) << 4)
                    | last_item.class_flag as u32;
                if self.contexts[self.current_context as usize].m_flags[last_flags as usize]
                    .is_none()
                {
                    let mut model = ArithmeticModel::new(64, false);
                    model.init(None);
                    self.contexts[self.current_context as usize].m_flags[last_flags as usize] =
                        Some(model);
                }
                let flags = self.dec_flags.borrow_mut().decode_symbol(
                    self.contexts[self.current_context as usize].m_flags[last_flags as usize]
                        .as_mut()
                        .unwrap(),
                ); // U32
                last_item.edge_of_flight_line = (flags & (1 << 5)) != 0;
                last_item.scan_direction_flag = (flags & (1 << 4)) != 0;
                last_item.class_flag = (flags & 0x0f) as u8;
                // legacy copies
                //   last_item.legacy_flags = flags & 0x07;
            }
            ////////////////////////////////////////
            // decompress intensity layer (if changed and requested)
            ////////////////////////////////////////
            if self.changed_intensity {
                // if the intensity should be decompressed and changes within this chunk
                // U16
                let intensity = self.contexts[self.current_context as usize]
                    .ic_intensity
                    .as_mut()
                    .unwrap()
                    .decompress(
                        self.contexts[self.current_context as usize].last_intensity
                            [(cpr << 1) | (if gps_time_change { 1 } else { 0 })]
                            as i32,
                        cpr as u32,
                    );
                self.contexts[self.current_context as usize].last_intensity
                    [(cpr << 1) | (if gps_time_change { 1 } else { 0 })] = intensity as u16;
                last_item.intensity = intensity as u16;
            }
            ////////////////////////////////////////
            // decompress scan_angle layer (if changed and requested)
            ////////////////////////////////////////
            if self.changed_scan_angle {
                // if the scan angle should be decompressed and changes within this chunk
                if scan_angle_change {
                    // if the scan angle has actually changed
                    last_item.scan_angle = self.contexts[self.current_context as usize]
                        .ic_scan_angle
                        .as_mut()
                        .unwrap()
                        .decompress(
                            last_item.scan_angle as i32,
                            if gps_time_change { 1 } else { 0 },
                        ) as i16; // if the GPS time has changed
                    last_item.legacy_scan_angle_rank =
                        i8_clamp(i16_quantize(0.006 * last_item.scan_angle as f64) as i32);
                }
            }
            ////////////////////////////////////////
            // decompress user_data layer (if changed and requested)
            ////////////////////////////////////////
            if self.changed_user_data {
                let index = (last_item.user_data / 4) as usize;
                // if the user data should be decompressed and changes within this chunk
                if self.contexts[self.current_context as usize].m_user_data[index].is_none() {
                    let mut model = ArithmeticModel::new(256, false);
                    model.init(None);
                    self.contexts[self.current_context as usize].m_user_data[index] = Some(model);
                }
                last_item.user_data = self.dec_user_data.borrow_mut().decode_symbol(
                    self.contexts[self.current_context as usize].m_user_data[index]
                        .as_mut()
                        .unwrap(),
                ) as u8;
            }
            ////////////////////////////////////////
            // decompress point_source layer (if changed and requested)
            ////////////////////////////////////////
            if self.changed_point_source {
                // if the point source ID should be decompressed and changes within this chunk
                if point_source_change {
                    // if the point source ID has actually changed
                    last_item.point_source_id = self.contexts[self.current_context as usize]
                        .ic_point_source_id
                        .as_mut()
                        .unwrap()
                        .decompress(last_item.point_source_id as i32, 0)
                        as u16;
                }
            }
        }
        ////////////////////////////////////////
        // decompress gps_time layer (if changed and requested)
        ////////////////////////////////////////
        if self.changed_gps_time {
            // if the GPS time should be decompressed and changes within this chunk
            if gps_time_change {
                // if the GPS time has actually changed
                self.read_gps_time();
                let last_item = &mut self.contexts[self.current_context as usize].last_item;
                last_item.gps_time = Some(
                    self.contexts[self.current_context as usize].last_gpstime
                        [self.contexts[self.current_context as usize].last as usize]
                        .f64(),
                );
            }
        }
        let last_item = &mut self.contexts[self.current_context as usize].last_item;
        // copy the last item
        *item = last_item.clone();
        // remember if the last point had a gps_time_change
        last_item.gps_time_change = Some(if gps_time_change { 1 } else { 0 });
    }

    fn chunk_sizes<R: Reader>(&mut self, reader: &R) {
        self.num_bytes_channel_returns_xy = reader.uint32_le(None);
        self.num_bytes_z = reader.uint32_le(None);
        self.num_bytes_classification = reader.uint32_le(None);
        self.num_bytes_flags = reader.uint32_le(None);
        self.num_bytes_intensity = reader.uint32_le(None);
        self.num_bytes_scan_angle = reader.uint32_le(None);
        self.num_bytes_user_data = reader.uint32_le(None);
        self.num_bytes_point_source = reader.uint32_le(None);
        self.num_bytes_gps_time = reader.uint32_le(None);
    }
}

/// Parse LAZ RGB 1.4v3
#[derive(Debug)]
pub struct LAZrgb14v3Reader<T: Reader> {
    dec: Rc<RefCell<ArithmeticDecoder<T>>>,
    instream_rgb: Option<Rc<RefCell<BufferReader>>>,
    dec_rgb: Option<ArithmeticDecoder<BufferReader>>,
    changed_rgb: bool,
    num_bytes_rgb: u32,
    requested_rgb: bool,
    bytes: Option<Vec<u8>>,
    num_bytes_allocated: u32,
    current_context: u32,
    contexts: [LAZContextRGB14; 4],
}
impl<T: Reader> LAZrgb14v3Reader<T> {
    /// Create a new LAZrgb14v3Reader
    pub fn new(dec: Rc<RefCell<ArithmeticDecoder<T>>>, decompress_selective: Option<u32>) -> Self {
        let decompress_selective = decompress_selective.unwrap_or(LASZIP_DECOMPRESS_SELECTIVE_ALL);
        let mut rgb_reader = Self {
            dec,
            instream_rgb: None,
            dec_rgb: None,
            changed_rgb: false,
            num_bytes_rgb: 0,
            requested_rgb: false,
            bytes: None,
            num_bytes_allocated: 0,
            current_context: 0,
            contexts: [
                LAZContextRGB14::default(),
                LAZContextRGB14::default(),
                LAZContextRGB14::default(),
                LAZContextRGB14::default(),
            ],
        };
        rgb_reader.requested_rgb = decompress_selective & LASZIP_DECOMPRESS_SELECTIVE_RGB != 0;
        // mark the four scanner channel contexts as uninitialized
        for c in 0..4 {
            rgb_reader.contexts[c].m_byte_used = None;
        }
        rgb_reader
    }

    fn create_and_init_models_and_decompressors(&mut self, context: usize, item: &[u16; 3]) {
        // first create all entropy models (if needed)
        if self.contexts[context].m_byte_used.is_none() {
            self.contexts[context].m_byte_used = Some(ArithmeticModel::new(128, false));
            self.contexts[context].m_rgb_diff0 = Some(ArithmeticModel::new(256, false));
            self.contexts[context].m_rgb_diff1 = Some(ArithmeticModel::new(256, false));
            self.contexts[context].m_rgb_diff2 = Some(ArithmeticModel::new(256, false));
            self.contexts[context].m_rgb_diff3 = Some(ArithmeticModel::new(256, false));
            self.contexts[context].m_rgb_diff4 = Some(ArithmeticModel::new(256, false));
            self.contexts[context].m_rgb_diff5 = Some(ArithmeticModel::new(256, false));
        }
        // then init entropy models
        self.contexts[context].m_byte_used.as_mut().unwrap().init(None);
        self.contexts[context].m_rgb_diff0.as_mut().unwrap().init(None);
        self.contexts[context].m_rgb_diff1.as_mut().unwrap().init(None);
        self.contexts[context].m_rgb_diff2.as_mut().unwrap().init(None);
        self.contexts[context].m_rgb_diff3.as_mut().unwrap().init(None);
        self.contexts[context].m_rgb_diff4.as_mut().unwrap().init(None);
        self.contexts[context].m_rgb_diff5.as_mut().unwrap().init(None);
        // init current context from item
        self.contexts[context].last_item = *item;
        self.contexts[context].unused = false;
    }
}
impl<T: Reader> ItemReader for LAZrgb14v3Reader<T> {
    fn init<R: Reader>(&mut self, item: &R, point: &mut LASPoint, context: &mut u32) {
        {
            let dec = &mut self.dec.borrow_mut();
            let reader = &dec.reader.borrow();

            // make sure the buffer is sufficiently large
            if self.num_bytes_rgb > self.num_bytes_allocated {
                self.num_bytes_allocated = self.num_bytes_rgb;
            }
            // load the requested bytes and init the corresponding instreams an decoders
            if self.requested_rgb {
                if self.num_bytes_rgb != 0 {
                    let bytes = reader.seek_slice(self.num_bytes_rgb as usize);
                    self.bytes = Some(bytes.clone());
                    let rgb_reader = Rc::new(RefCell::new(BufferReader::new(bytes)));
                    self.instream_rgb = Some(rgb_reader.clone());
                    let mut decoder = ArithmeticDecoder::new(rgb_reader);
                    decoder.init(true);
                    self.dec_rgb = Some(decoder);
                    self.changed_rgb = true;
                } else {
                    self.instream_rgb = None;
                    self.changed_rgb = false;
                }
            } else {
                if self.num_bytes_rgb != 0 {
                    reader.seek(reader.tell() + self.num_bytes_rgb as u64);
                }
                self.changed_rgb = false;
            }
            // mark the four scanner channel contexts as unused
            for c in 0..4 {
                self.contexts[c].unused = true;
            }
            // set scanner channel as current context
            self.current_context = *context; // all other items use context set by POINT14 reader
        }
        // create and init models and decompressors
        let mut data: [u16; 3] = [0, 0, 0];
        data[0] = item.uint16_le(None);
        data[1] = item.uint16_le(None);
        data[2] = item.uint16_le(None);
        point.rgba = Some(RGBA::from_u16s(data[0], data[1], data[2], u16::MAX));
        self.create_and_init_models_and_decompressors(self.current_context as usize, &data);
    }

    fn read(&mut self, item: &mut LASPoint, context: &mut u32) {
        let item_rgb = item.rgba.unwrap().to_u16s();
        let mut item_rgb: [u16; 3] = [item_rgb.0, item_rgb.1, item_rgb.2];
        // get last
        let mut last_item = self.contexts[self.current_context as usize].last_item;
        // check for context switch
        if self.current_context != *context {
            self.current_context = *context; // all other items use context set by POINT14 reader
            if self.contexts[self.current_context as usize].unused {
                self.create_and_init_models_and_decompressors(
                    self.current_context as usize,
                    &last_item,
                );
                last_item = self.contexts[self.current_context as usize].last_item;
            }
        }
        // decompress
        if self.changed_rgb {
            // let mut corr: u8 = 0; // U8
            // let mut diff: i32 = 0; // I32
            let sym = self.dec_rgb.as_mut().unwrap().decode_symbol(
                self.contexts[self.current_context as usize].m_byte_used.as_mut().unwrap(),
            ); // U32
            if (sym & (1 << 0)) != 0 {
                let corr = self.dec_rgb.as_mut().unwrap().decode_symbol(
                    self.contexts[self.current_context as usize].m_rgb_diff0.as_mut().unwrap(),
                ) as u8;
                item_rgb[0] = u8_fold(corr as u32 + (last_item[0] & 255) as u32) as u16;
            } else {
                item_rgb[0] = last_item[0] & 0xff;
            }
            if (sym & (1 << 1)) != 0 {
                let corr = self.dec_rgb.as_mut().unwrap().decode_symbol(
                    self.contexts[self.current_context as usize].m_rgb_diff1.as_mut().unwrap(),
                ) as u8;
                item_rgb[0] |= (u8_fold(corr as u32 + (last_item[0] >> 8) as u32) as u16) << 8;
            } else {
                item_rgb[0] |= last_item[0] & 0xff00;
            }
            if (sym & (1 << 6)) != 0 {
                let mut diff = (item_rgb[0] & 0x00ff) as i32 - (last_item[0] & 0x00ff) as i32;
                if (sym & (1 << 2)) != 0 {
                    let corr = self.dec_rgb.as_mut().unwrap().decode_symbol(
                        self.contexts[self.current_context as usize].m_rgb_diff2.as_mut().unwrap(),
                    ) as u8;
                    item_rgb[1] = u8_fold(
                        corr as u32 + u8_clamp((diff + (last_item[1] & 255) as i32) as u32) as u32,
                    ) as u16;
                } else {
                    item_rgb[1] = last_item[1] & 0xff;
                }
                if (sym & (1 << 4)) != 0 {
                    let corr = self.dec_rgb.as_mut().unwrap().decode_symbol(
                        self.contexts[self.current_context as usize].m_rgb_diff4.as_mut().unwrap(),
                    ) as u8;
                    diff = (diff
                        + ((item_rgb[1] & 0x00ff) as i32 - (last_item[1] & 0x00ff) as i32))
                        / 2;
                    item_rgb[2] = u8_fold(
                        corr as u32 + u8_clamp((diff + (last_item[2] & 255) as i32) as u32) as u32,
                    ) as u16;
                } else {
                    item_rgb[2] = last_item[2] & 0xff;
                }
                diff = (item_rgb[0] >> 8) as i32 - (last_item[0] >> 8) as i32;
                if (sym & (1 << 3)) != 0 {
                    let corr = self.dec_rgb.as_mut().unwrap().decode_symbol(
                        self.contexts[self.current_context as usize].m_rgb_diff3.as_mut().unwrap(),
                    ) as u8;
                    item_rgb[1] |= (u8_fold(
                        corr as u32 + u8_clamp((diff + (last_item[1] >> 8) as i32) as u32) as u32,
                    ) as u16)
                        << 8;
                } else {
                    item_rgb[1] |= last_item[1] & 0xff00;
                }
                if (sym & (1 << 5)) != 0 {
                    let corr = self.dec_rgb.as_mut().unwrap().decode_symbol(
                        self.contexts[self.current_context as usize].m_rgb_diff5.as_mut().unwrap(),
                    ) as u8;
                    diff = (diff + ((item_rgb[1] >> 8) as i32 - (last_item[1] >> 8) as i32)) / 2;
                    item_rgb[2] |= (u8_fold(
                        corr as u32 + u8_clamp((diff + (last_item[2] >> 8) as i32) as u32) as u32,
                    ) as u16)
                        << 8;
                } else {
                    item_rgb[2] |= last_item[2] & 0xff00;
                }
            } else {
                item_rgb[1] = item_rgb[0];
                item_rgb[2] = item_rgb[0];
            }
            self.contexts[self.current_context as usize].last_item = item_rgb;
        }
        item.rgba = Some(RGBA::from_u16s(item_rgb[0], item_rgb[1], item_rgb[2], u16::MAX));
    }

    fn chunk_sizes<R: Reader>(&mut self, reader: &R) {
        self.num_bytes_rgb = reader.uint32_le(None);
    }
}

/// Parse LAZ RGB NIR 1.4v3
#[derive(Debug)]
pub struct LAZrgbNir14v3Reader<T: Reader> {
    dec: Rc<RefCell<ArithmeticDecoder<T>>>,
    instream_rgb: Option<Rc<RefCell<BufferReader>>>,
    instream_nir: Option<Rc<RefCell<BufferReader>>>,
    dec_rgb: Option<ArithmeticDecoder<BufferReader>>,
    dec_nir: Option<ArithmeticDecoder<BufferReader>>,
    changed_rgb: bool,
    changed_nir: bool,
    num_bytes_rgb: u32,
    num_bytes_nir: u32,
    requested_rgb: bool,
    requested_nir: bool,
    bytes: Option<Buffer>,
    num_bytes_allocated: u32,
    current_context: u32,
    contexts: [LASContextRGBNir14; 4],
}
impl<T: Reader> LAZrgbNir14v3Reader<T> {
    /// Create a new LAZrgbNir14v3Reader
    pub fn new(dec: Rc<RefCell<ArithmeticDecoder<T>>>, decompress_selective: Option<u32>) -> Self {
        let decompress_selective = decompress_selective.unwrap_or(LASZIP_DECOMPRESS_SELECTIVE_ALL);
        let mut rgbn_reader = LAZrgbNir14v3Reader {
            dec,
            instream_rgb: None,
            instream_nir: None,
            dec_rgb: None,
            dec_nir: None,
            changed_rgb: false,
            changed_nir: false,
            num_bytes_rgb: 0,
            num_bytes_nir: 0,
            requested_rgb: false,
            requested_nir: false,
            bytes: None,
            num_bytes_allocated: 0,
            current_context: 0,
            contexts: [
                LASContextRGBNir14::default(),
                LASContextRGBNir14::default(),
                LASContextRGBNir14::default(),
                LASContextRGBNir14::default(),
            ],
        };
        rgbn_reader.requested_rgb = (decompress_selective & LASZIP_DECOMPRESS_SELECTIVE_RGB) != 0;
        rgbn_reader.requested_nir = (decompress_selective & LASZIP_DECOMPRESS_SELECTIVE_NIR) != 0;
        // mark the four scanner channel contexts as uninitialized
        for c in 0..4 {
            rgbn_reader.contexts[c].m_rgb_bytes_used = None;
            rgbn_reader.contexts[c].m_nir_bytes_used = None;
        }

        rgbn_reader
    }

    fn create_and_init_models_and_decompressors(&mut self, context: usize, item: &[u16; 4]) {
        // first create all entropy models (if needed)
        if self.requested_rgb {
            if self.contexts[context].m_rgb_bytes_used.is_none() {
                self.contexts[context].m_rgb_bytes_used = Some(ArithmeticModel::new(128, false));
                self.contexts[context].m_rgb_diff0 = Some(ArithmeticModel::new(256, false));
                self.contexts[context].m_rgb_diff1 = Some(ArithmeticModel::new(256, false));
                self.contexts[context].m_rgb_diff2 = Some(ArithmeticModel::new(256, false));
                self.contexts[context].m_rgb_diff3 = Some(ArithmeticModel::new(256, false));
                self.contexts[context].m_rgb_diff4 = Some(ArithmeticModel::new(256, false));
                self.contexts[context].m_rgb_diff5 = Some(ArithmeticModel::new(256, false));
            }
            // then init entropy models
            self.contexts[context].m_rgb_bytes_used.as_mut().unwrap().init(None);
            self.contexts[context].m_rgb_diff0.as_mut().unwrap().init(None);
            self.contexts[context].m_rgb_diff1.as_mut().unwrap().init(None);
            self.contexts[context].m_rgb_diff2.as_mut().unwrap().init(None);
            self.contexts[context].m_rgb_diff3.as_mut().unwrap().init(None);
            self.contexts[context].m_rgb_diff4.as_mut().unwrap().init(None);
            self.contexts[context].m_rgb_diff5.as_mut().unwrap().init(None);
        }
        if self.requested_nir {
            if self.contexts[context].m_nir_bytes_used.is_none() {
                self.contexts[context].m_nir_bytes_used = Some(ArithmeticModel::new(4, false));
                self.contexts[context].m_nir_diff0 = Some(ArithmeticModel::new(256, false));
                self.contexts[context].m_nir_diff1 = Some(ArithmeticModel::new(256, false));
            }
            // then init entropy models
            self.contexts[context].m_nir_bytes_used.as_mut().unwrap().init(None);
            self.contexts[context].m_nir_diff0.as_mut().unwrap().init(None);
            self.contexts[context].m_nir_diff1.as_mut().unwrap().init(None);
        }
        // init current context from item
        self.contexts[context].last_item = *item;
        self.contexts[context].unused = false;
    }
}
impl<T: Reader> ItemReader for LAZrgbNir14v3Reader<T> {
    fn init<R: Reader>(&mut self, item: &R, point: &mut LASPoint, context: &mut u32) {
        {
            // for layered compression 'dec' only hands over the stream
            let dec = self.dec.borrow_mut();
            let reader = dec.reader.clone();
            // on the first init create instreams and decoders
            // TODO: Do we need this?
            // if self.instream_rgb.is_none() {
            //   // create decoders
            //   self.dec_rgb = Some(ArithmeticDecoder::new(reader));
            //   self.dec_nir = Some(ArithmeticDecoder::new(reader));
            // }
            let reader = reader.borrow_mut();
            // how many bytes do we need to read
            let mut num_bytes = 0;
            if self.requested_rgb {
                num_bytes += self.num_bytes_rgb;
            }
            if self.requested_nir {
                num_bytes += self.num_bytes_nir;
            }
            // make sure the buffer is sufficiently large
            if num_bytes > self.num_bytes_allocated {
                self.num_bytes_allocated = num_bytes;
            }
            // load the requested bytes and init the corresponding instreams an decoders
            // num_bytes = 0;
            if self.requested_rgb {
                if self.num_bytes_rgb != 0 {
                    let bytes = reader.seek_slice(self.num_bytes_rgb as usize);
                    let buffer = bytes.clone().into();
                    self.bytes = Some(buffer);
                    // num_bytes += self.num_bytes_rgb;
                    let br = Rc::new(RefCell::new(BufferReader::new(bytes)));
                    self.instream_rgb = Some(br.clone());
                    let mut decoder = ArithmeticDecoder::new(br);
                    decoder.init(true);
                    self.dec_rgb = Some(decoder);
                    self.changed_rgb = true;
                } else {
                    self.instream_rgb = None;
                    self.changed_rgb = false;
                }
            } else {
                if self.num_bytes_rgb != 0 {
                    reader.seek(reader.tell() + self.num_bytes_rgb as u64);
                }
                self.changed_rgb = false;
            }
            if self.requested_nir {
                if self.num_bytes_nir != 0 {
                    let bytes = reader.seek_slice(self.num_bytes_nir as usize);
                    let buffer = bytes.clone().into();
                    self.bytes = Some(buffer);
                    // let br = BufferReader::new(bytes);
                    let br = Rc::new(RefCell::new(BufferReader::new(bytes)));
                    self.instream_nir = Some(br.clone());
                    let mut decoder = ArithmeticDecoder::new(br);
                    decoder.init(true);
                    self.dec_nir = Some(decoder);
                    self.changed_nir = true;
                } else {
                    self.instream_nir = None;
                    self.changed_nir = false;
                }
            } else {
                if self.num_bytes_nir != 0 {
                    reader.seek(reader.tell() + self.num_bytes_nir as u64);
                }
                self.changed_nir = false;
            }
        }
        // mark the four scanner channel contexts as unused
        for c in 0..4 {
            self.contexts[c].unused = true;
        }
        // set scanner channel as current context
        self.current_context = *context; // all other items use context set by POINT14 reader
        // create and init models and decompressors
        let mut data: [u16; 4] = [0; 4];
        data[0] = item.uint16_le(None);
        data[1] = item.uint16_le(None);
        data[2] = item.uint16_le(None);
        data[3] = item.uint16_le(None);
        point.rgba = Some(RGBA::from_u16s(data[0], data[1], data[2], u16::MAX));
        point.nir = Some(data[3]);
        self.create_and_init_models_and_decompressors(self.current_context as usize, &data);
    }

    fn read(&mut self, item: &mut LASPoint, context: &mut u32) {
        let item_rgb = item.rgba.unwrap().to_u16s();
        let mut item_rgb: [u16; 3] = [item_rgb.0, item_rgb.1, item_rgb.2];
        // let mut item_nir = item.nir.unwrap();
        // get last
        let mut last_item = self.contexts[self.current_context as usize].last_item;
        // check for context switch
        if self.current_context != *context {
            self.current_context = *context; // all other items use context set by POINT14 reader
            if self.contexts[self.current_context as usize].unused {
                self.create_and_init_models_and_decompressors(
                    self.current_context as usize,
                    &last_item,
                );
                last_item = self.contexts[self.current_context as usize].last_item;
            }
        }
        // decompress
        ////////////////////////////////////////
        // decompress RGB layer
        ////////////////////////////////////////
        if self.changed_rgb {
            let mut corr: u8; // U8
            // let mut diff: i32 = 0; // I32
            let sym = self.dec_rgb.as_mut().unwrap().decode_symbol(
                self.contexts[self.current_context as usize].m_rgb_bytes_used.as_mut().unwrap(),
            ); // U32
            if (sym & (1 << 0)) != 0 {
                corr = self.dec_rgb.as_mut().unwrap().decode_symbol(
                    self.contexts[self.current_context as usize].m_rgb_diff0.as_mut().unwrap(),
                ) as u8;
                item_rgb[0] = u8_fold(corr as u32 + (last_item[0] & 255) as u32) as u16;
            } else {
                item_rgb[0] = last_item[0] & 0xff;
            }
            if (sym & (1 << 1)) != 0 {
                corr = self.dec_rgb.as_mut().unwrap().decode_symbol(
                    self.contexts[self.current_context as usize].m_rgb_diff1.as_mut().unwrap(),
                ) as u8;
                item_rgb[0] |= (u8_fold(corr as u32 + (last_item[0] >> 8) as u32) as u16) << 8;
            } else {
                item_rgb[0] |= last_item[0] & 0xff00;
            }
            if (sym & (1 << 6)) != 0 {
                let diff = (item_rgb[0] & 0x00ff) as i32 - (last_item[0] & 0x00ff) as i32;
                if (sym & (1 << 2)) != 0 {
                    corr = self.dec_rgb.as_mut().unwrap().decode_symbol(
                        self.contexts[self.current_context as usize].m_rgb_diff2.as_mut().unwrap(),
                    ) as u8;
                    item_rgb[1] = u8_fold(
                        corr as u32 + (u8_clamp(diff as u32 + (last_item[1] & 255) as u32) as u32),
                    ) as u16;
                } else {
                    item_rgb[1] = last_item[1] & 0xff;
                }
                if (sym & (1 << 4)) != 0 {
                    corr = self.dec_rgb.as_mut().unwrap().decode_symbol(
                        self.contexts[self.current_context as usize].m_rgb_diff4.as_mut().unwrap(),
                    ) as u8;
                    let diff = (diff
                        + ((item_rgb[1] & 0x00ff) as i32 - (last_item[1] & 0x00ff) as i32))
                        / 2;
                    item_rgb[2] = u8_fold(
                        corr as u32 + u8_clamp(diff as u32 + (last_item[2] & 255) as u32) as u32,
                    ) as u16;
                } else {
                    item_rgb[2] = last_item[2] & 0xff;
                }
                let diff = (item_rgb[0] >> 8) as i32 - (last_item[0] >> 8) as i32;
                if (sym & (1 << 3)) != 0 {
                    corr = self.dec_rgb.as_mut().unwrap().decode_symbol(
                        self.contexts[self.current_context as usize].m_rgb_diff3.as_mut().unwrap(),
                    ) as u8;
                    item_rgb[1] |= (u8_fold(
                        corr as u32 + u8_clamp(diff as u32 + (last_item[1] >> 8) as u32) as u32,
                    ) as u16)
                        << 8;
                } else {
                    item_rgb[1] |= last_item[1] & 0xff00;
                }
                if (sym & (1 << 5)) != 0 {
                    corr = self.dec_rgb.as_mut().unwrap().decode_symbol(
                        self.contexts[self.current_context as usize].m_rgb_diff5.as_mut().unwrap(),
                    ) as u8;
                    let diff =
                        (diff + ((item_rgb[1] >> 8) as i32 - (last_item[1] >> 8) as i32)) / 2;
                    item_rgb[2] |= (u8_fold(
                        corr as u32 + u8_clamp(diff as u32 + (last_item[2] >> 8) as u32) as u32,
                    ) as u16)
                        << 8;
                } else {
                    item_rgb[2] |= last_item[2] & 0xff00;
                }
            } else {
                item_rgb[1] = item_rgb[0];
                item_rgb[2] = item_rgb[0];
            }
            last_item[0..3].copy_from_slice(&item_rgb[0..3]);
        } else {
            item_rgb.copy_from_slice(&last_item[0..3]);
        }
        item.rgba = Some(RGBA::from_u16s(item_rgb[0], item_rgb[1], item_rgb[2], u16::MAX));
        ////////////////////////////////////////
        // decompress NIR layer
        ////////////////////////////////////////
        let mut item_nir: u16;
        if self.changed_nir {
            let mut corr: u8; // U8
            let sym = self.dec_nir.as_mut().unwrap().decode_symbol(
                self.contexts[self.current_context as usize].m_nir_bytes_used.as_mut().unwrap(),
            ); // U32
            if (sym & (1 << 0)) != 0 {
                corr = self.dec_nir.as_mut().unwrap().decode_symbol(
                    self.contexts[self.current_context as usize].m_nir_diff0.as_mut().unwrap(),
                ) as u8;
                item_nir = u8_fold(corr as u32 + (last_item[3] & 255) as u32) as u16;
            } else {
                item_nir = last_item[3] & 0xff;
            }
            if (sym & (1 << 1)) != 0 {
                corr = self.dec_nir.as_mut().unwrap().decode_symbol(
                    self.contexts[self.current_context as usize].m_nir_diff1.as_mut().unwrap(),
                ) as u8;
                item_nir |= (u8_fold(corr as u32 + (last_item[3] >> 8) as u32) as u16) << 8;
            } else {
                item_nir |= last_item[3] & 0xff00;
            }
            last_item[3] = item_nir;
        } else {
            item_nir = last_item[3];
        }
        item.nir = Some(item_nir);
    }

    fn chunk_sizes<R: Reader>(&mut self, reader: &R) {
        // read bytes per layer
        self.num_bytes_rgb = reader.uint32_le(None);
        self.num_bytes_nir = reader.uint32_le(None);
    }
}

/// Parse LAZ wavepacket 1.4v3
#[derive(Debug)]
pub struct LAZwavepacket14v3Reader<T: Reader> {
    dec: Rc<RefCell<ArithmeticDecoder<T>>>,
    instream_wavepacket: Option<BufferReader>,
    dec_wavepacket: Rc<RefCell<ArithmeticDecoder<BufferReader>>>,
    changed_wavepacket: bool,
    num_bytes_wavepacket: u32,
    requested_wavepacket: bool,
    bytes: Option<Buffer>,
    num_bytes_allocated: u32,
    current_context: u32,
    contexts: [LASContextWavePacket14<BufferReader>; 4],
}
impl<T: Reader> LAZwavepacket14v3Reader<T> {
    /// Create a new LAZwavepacket14v3Reader
    pub fn new(dec: Rc<RefCell<ArithmeticDecoder<T>>>, decompress_selective: Option<u32>) -> Self {
        let decompress_selective = decompress_selective.unwrap_or(LASZIP_DECOMPRESS_SELECTIVE_ALL);
        let mut wavepacket_reader = LAZwavepacket14v3Reader {
            dec,
            instream_wavepacket: None,
            dec_wavepacket: Rc::new(RefCell::new(ArithmeticDecoder::new(
                RefCell::new(BufferReader::new(vec![0_u8; 0])).into(),
            ))),
            changed_wavepacket: false,
            num_bytes_wavepacket: 0,
            requested_wavepacket: false,
            bytes: None,
            num_bytes_allocated: 0,
            current_context: 0,
            contexts: [
                LASContextWavePacket14::<BufferReader>::default(),
                LASContextWavePacket14::<BufferReader>::default(),
                LASContextWavePacket14::<BufferReader>::default(),
                LASContextWavePacket14::<BufferReader>::default(),
            ],
        };
        wavepacket_reader.requested_wavepacket =
            (decompress_selective & LASZIP_DECOMPRESS_SELECTIVE_WAVEPACKET) != 0;
        // mark the four scanner channel contexts as uninitialized
        for c in 0..4 {
            wavepacket_reader.contexts[c].m_packet_index = None;
        }
        wavepacket_reader
    }

    fn create_and_init_models_and_decompressors(&mut self, context: usize, item: &[u8; 29]) {
        // first create all entropy models (if needed)
        if self.requested_wavepacket {
            if self.contexts[context].m_packet_index.is_none() {
                self.contexts[context].m_packet_index = Some(ArithmeticModel::new(256, false));
                self.contexts[context].m_offset_diff[0] = Some(ArithmeticModel::new(4, false));
                self.contexts[context].m_offset_diff[1] = Some(ArithmeticModel::new(4, false));
                self.contexts[context].m_offset_diff[2] = Some(ArithmeticModel::new(4, false));
                self.contexts[context].m_offset_diff[3] = Some(ArithmeticModel::new(4, false));
                self.contexts[context].ic_offset_diff = Some(IntegerCompressor::new(
                    self.dec_wavepacket.clone(),
                    Some(32),
                    None,
                    None,
                    None,
                ));
                self.contexts[context].ic_packet_size = Some(IntegerCompressor::new(
                    self.dec_wavepacket.clone(),
                    Some(32),
                    None,
                    None,
                    None,
                ));
                self.contexts[context].ic_return_point = Some(IntegerCompressor::new(
                    self.dec_wavepacket.clone(),
                    Some(32),
                    None,
                    None,
                    None,
                ));
                self.contexts[context].ic_xyz = Some(IntegerCompressor::new(
                    self.dec_wavepacket.clone(),
                    Some(32),
                    Some(3),
                    None,
                    None,
                ));
            }
            // then init entropy models
            self.contexts[context].m_packet_index.as_mut().unwrap().init(None);
            self.contexts[context].m_offset_diff[0].as_mut().unwrap().init(None);
            self.contexts[context].m_offset_diff[1].as_mut().unwrap().init(None);
            self.contexts[context].m_offset_diff[2].as_mut().unwrap().init(None);
            self.contexts[context].m_offset_diff[3].as_mut().unwrap().init(None);
            self.contexts[context].ic_offset_diff.as_mut().unwrap().init_decompressor();
            self.contexts[context].ic_packet_size.as_mut().unwrap().init_decompressor();
            self.contexts[context].ic_return_point.as_mut().unwrap().init_decompressor();
            self.contexts[context].ic_xyz.as_mut().unwrap().init_decompressor();
        }
        // init current context from item
        self.contexts[context].last_diff32 = 0;
        self.contexts[context].sym_last_offset_diff = 0;
        self.contexts[context].last_item = *item;
        self.contexts[context].unused = false;
    }
}
impl<T: Reader> ItemReader for LAZwavepacket14v3Reader<T> {
    fn init<R: Reader>(&mut self, item: &R, point: &mut LASPoint, context: &mut u32) {
        {
            let dec = &mut self.dec.borrow_mut();
            let reader = dec.reader.borrow();
            // on the first init create instreams and decoders
            if self.instream_wavepacket.is_none() {
                // create decoders
                // TODO: Do we need this?
                //   self.dec_wavepacket = Some(ArithmeticDecoder::new(reader));
            }
            // make sure the buffer is sufficiently large
            if self.num_bytes_wavepacket > self.num_bytes_allocated {
                self.num_bytes_allocated = self.num_bytes_wavepacket;
            }
            // load the requested bytes and init the corresponding instreams an decoders
            if self.requested_wavepacket {
                if self.num_bytes_wavepacket != 0 {
                    let buf = reader.seek_slice(self.num_bytes_wavepacket as usize);
                    let br = BufferReader::new(buf.clone());
                    let buf_reader = Rc::new(RefCell::new(br.clone()));
                    self.bytes = Some(buf.into());
                    self.instream_wavepacket = Some(br);
                    let mut ad = ArithmeticDecoder::new(buf_reader.clone());
                    ad.init(true);
                    self.dec_wavepacket = Rc::new(RefCell::new(ad));
                    self.changed_wavepacket = true;
                } else {
                    self.instream_wavepacket = None;
                    self.changed_wavepacket = false;
                }
            } else {
                if self.num_bytes_wavepacket != 0 {
                    reader.seek(reader.tell() + self.num_bytes_wavepacket as u64);
                }
                self.changed_wavepacket = false;
            }
            // mark the four scanner channel contexts as unused
            for c in 0..4 {
                self.contexts[c].unused = true;
            }
            // set scanner channel as current context
            self.current_context = *context; // all other items use context set by POINT14 reader
        }
        // create and init models and decompressors
        let item: [u8; 29] = item.seek_slice(29).try_into().unwrap();
        point.inject_wave_packet(&BufferReader::new(item.to_vec()), 0);
        self.create_and_init_models_and_decompressors(self.current_context as usize, &item);
    }

    fn read(&mut self, item: &mut LASPoint, context: &mut u32) {
        // get last
        if item.wave_packet.is_none() {
            item.wave_packet = Some(WavePacket::default());
        }
        let curr_item = item.wave_packet.as_mut().unwrap();
        let last_item_val = self.contexts[self.current_context as usize].last_item;
        let mut last_item = WavePacket::from_reader(&BufferReader::new(last_item_val.to_vec()), 0);
        // check for context switch
        if self.current_context != *context {
            self.current_context = *context; // all other items use context set by POINT14 reader
            if self.contexts[self.current_context as usize].unused {
                self.create_and_init_models_and_decompressors(
                    self.current_context as usize,
                    &last_item_val,
                );
                last_item = WavePacket::from_reader(
                    &BufferReader::new(
                        self.contexts[self.current_context as usize].last_item.to_vec(),
                    ),
                    0,
                );
            }
        }

        // decompress
        if self.changed_wavepacket {
            // INDEX does not matter
            // curr_item.index = self.dec_wavepacket.decode_symbol(
            //     self.contexts[self.current_context as usize].m_packet_index.unwrap(),
            // );

            self.contexts[self.current_context as usize].sym_last_offset_diff =
                self.dec_wavepacket.borrow_mut().decode_symbol(
                    self.contexts[self.current_context as usize].m_offset_diff[self.contexts
                        [self.current_context as usize]
                        .sym_last_offset_diff
                        as usize]
                        .as_mut()
                        .unwrap(),
                );

            if self.contexts[self.current_context as usize].sym_last_offset_diff == 0 {
                curr_item.offset = last_item.offset;
            } else if self.contexts[self.current_context as usize].sym_last_offset_diff == 1 {
                curr_item.offset = last_item.offset + last_item.length as u64;
            } else if self.contexts[self.current_context as usize].sym_last_offset_diff == 2 {
                self.contexts[self.current_context as usize].last_diff32 = self.contexts
                    [self.current_context as usize]
                    .ic_offset_diff
                    .as_mut()
                    .unwrap()
                    .decompress(self.contexts[self.current_context as usize].last_diff32, 0);
                curr_item.offset = last_item.offset
                    + self.contexts[self.current_context as usize].last_diff32 as u64;
            } else {
                curr_item.offset = self.dec_wavepacket.borrow_mut().read_int64();
            }

            curr_item.length = self.contexts[self.current_context as usize]
                .ic_packet_size
                .as_mut()
                .unwrap()
                .decompress(last_item.length as i32, 0) as u32;
            curr_item.return_point = self.contexts[self.current_context as usize]
                .ic_return_point
                .as_mut()
                .unwrap()
                .decompress(last_item.return_point as i32, 0)
                as f32;
            curr_item.x_t = self.contexts[self.current_context as usize]
                .ic_xyz
                .as_mut()
                .unwrap()
                .decompress(last_item.x_t as i32, 0) as f32;
            curr_item.y_t = self.contexts[self.current_context as usize]
                .ic_xyz
                .as_mut()
                .unwrap()
                .decompress(last_item.y_t as i32, 1) as f32;
            curr_item.z_t = self.contexts[self.current_context as usize]
                .ic_xyz
                .as_mut()
                .unwrap()
                .decompress(last_item.z_t as i32, 2) as f32;

            self.contexts[self.current_context as usize].last_item =
                curr_item.to_bytes().try_into().unwrap();
            // curr_item.copyTo(last_item.data, 29);
        }
    }

    fn chunk_sizes<R: Reader>(&mut self, reader: &R) {
        self.num_bytes_wavepacket = reader.uint32_le(None);
    }
}

/// Parse LAZ RGB 1.4v3
#[derive(Debug)]
pub struct LAZbyte14v3Reader<T: Reader> {
    dec: Rc<RefCell<ArithmeticDecoder<T>>>,
    size: u32,
    instream_bytes: Vec<Option<Rc<RefCell<BufferReader>>>>,
    dec_bytes: Vec<Option<ArithmeticDecoder<BufferReader>>>,
    num_bytes_bytes: Vec<u32>,
    changed_bytes: Vec<bool>,
    requested_bytes: Vec<bool>,
    bytes: Option<Vec<u8>>,
    num_bytes_allocated: u32,
    current_context: u32,
    contexts: [LAZContextByte14; 4],
}
impl<T: Reader> LAZbyte14v3Reader<T> {
    /// Create a new LAZbyte14v3Reader
    pub fn new(
        dec: Rc<RefCell<ArithmeticDecoder<T>>>,
        size: u32,
        decompress_selective: Option<u32>,
    ) -> Self {
        let decompress_selective = decompress_selective.unwrap_or(LASZIP_DECOMPRESS_SELECTIVE_ALL);
        let mut byte_reader = Self {
            dec,
            size,
            instream_bytes: vec![],
            dec_bytes: vec![],
            num_bytes_bytes: vec![],
            changed_bytes: vec![],
            requested_bytes: vec![],
            bytes: None,
            num_bytes_allocated: 0,
            current_context: 0,
            contexts: [
                LAZContextByte14::default(),
                LAZContextByte14::default(),
                LAZContextByte14::default(),
                LAZContextByte14::default(),
            ],
        };

        byte_reader.num_bytes_bytes = vec![0; size as usize];
        byte_reader.changed_bytes = vec![false; size as usize];
        byte_reader.requested_bytes = vec![false; size as usize];

        for i in 0..size as usize {
            byte_reader.num_bytes_bytes[i] = 0;
            byte_reader.changed_bytes[i] = false;

            if i > 15 {
                // currently only the first 16 extra bytes can be selectively decompressed
                byte_reader.requested_bytes[i] = true;
            } else {
                byte_reader.requested_bytes[i] =
                    (decompress_selective & (LASZIP_DECOMPRESS_SELECTIVE_BYTE0 << i)) != 0;
            }
        }

        // mark the four scanner channel contexts as uninitialized
        for c in 0..4 {
            byte_reader.contexts[c].m_bytes = vec![];
        }
        byte_reader.current_context = 0;

        byte_reader
    }

    fn create_and_init_models_and_decompressors(&mut self, context: usize, item: &[u8]) {
        // first create all entropy models and last items (if needed)
        if self.contexts[context].m_bytes.is_empty() {
            for i in 0..self.size as usize {
                let mut model = ArithmeticModel::new(256, false);
                model.init(None);
                self.contexts[context].m_bytes[i] = model;
            }
            // create last item
            self.contexts[context].last_item = Buffer::new(vec![0; self.size as usize]);
        }
        // then init entropy models
        for i in 0..self.size as usize {
            self.contexts[context].m_bytes[i].init(None);
        }
        // init current context from item
        self.contexts[context].last_item.copy_from_slice(0, item);

        self.contexts[context].unused = false;
    }
}
impl<T: Reader> ItemReader for LAZbyte14v3Reader<T> {
    fn init<R: Reader>(&mut self, item: &R, _point: &mut LASPoint, context: &mut u32) {
        {
            let dec = &mut self.dec.borrow_mut();
            let reader = dec.reader.borrow();
            // on the first init create instreams and decoders
            if self.instream_bytes.is_empty() {
                // create instream pointer array
                self.instream_bytes = vec![None; self.size as usize];
                // create decoder pointer array
                self.dec_bytes = vec![None; self.size as usize];
                // create layer decoders
                // TODO: Is this needed? It brakes the simplicity
                // for i in 0..self.size as usize {
                //     self.dec_bytes[i] = Some(ArithmeticDecoder::new(reader.clone()));
                // }
            }
            // how many bytes do we need to read
            let mut num_bytes = 0;
            for i in 0..self.size as usize {
                if self.requested_bytes[i] {
                    num_bytes += self.num_bytes_bytes[i];
                }
            }
            // make sure the buffer is sufficiently large
            if num_bytes > self.num_bytes_allocated {
                self.num_bytes_allocated = num_bytes;
            }
            // load the requested bytes and init the corresponding instreams an decoders
            num_bytes = 0;
            for i in 0..self.size as usize {
                if self.requested_bytes[i] {
                    if self.num_bytes_bytes[i] != 0 {
                        let buf = reader.seek_slice(self.num_bytes_bytes[i] as usize);
                        self.bytes = Some(buf.clone());
                        let buf_reader = Rc::new(RefCell::new(BufferReader::new(buf)));
                        self.instream_bytes[i] = Some(buf_reader.clone());
                        let mut decoder = ArithmeticDecoder::new(buf_reader);
                        decoder.init(true);
                        self.dec_bytes[i] = Some(decoder);
                        num_bytes += self.num_bytes_bytes[i];
                        self.changed_bytes[i] = true;
                    } else {
                        self.dec_bytes[i] = None;
                        self.changed_bytes[i] = false;
                    }
                } else {
                    if self.num_bytes_bytes[i] != 0 {
                        reader.seek(reader.tell() + self.num_bytes_bytes[i] as u64);
                    }
                    self.changed_bytes[i] = false;
                }
            }
            // mark the four scanner channel contexts as unused
            for c in 0..4 {
                self.contexts[c].unused = true;
            }
            // set scanner channel as current context
            self.current_context = *context; // all other items use context set by POINT14 reader
        }
        // create and init models and decompressors
        self.create_and_init_models_and_decompressors(
            self.current_context as usize,
            &item.seek_slice(self.size as usize),
        );
    }

    fn read(&mut self, _item: &mut LASPoint, context: &mut u32) {
        // get last
        let mut last_item: &mut Buffer =
            &mut self.contexts[self.current_context as usize].last_item;
        // check for context switch
        if self.current_context != *context {
            self.current_context = *context; // all other items use context set by POINT14 reader
            if self.contexts[self.current_context as usize].unused {
                let slice = last_item.buf().to_vec();
                self.create_and_init_models_and_decompressors(
                    self.current_context as usize,
                    &slice,
                );
                last_item = &mut self.contexts[self.current_context as usize].last_item;
            }
        }
        // decompress
        for i in 0..self.size as usize {
            if self.changed_bytes[i] {
                let value = last_item.get_u8_at(i) as u32
                    + self.dec_bytes[i].as_mut().unwrap().decode_symbol(
                        &mut self.contexts[self.current_context as usize].m_bytes[i],
                    );
                // item.setUint8(i, u8_fold(value));
                last_item.set_u8_at(i, u8_fold(value));
            } else {
                // item.setUint8(i, last_item.get_u8_at(i));
            }
        }
    }

    fn chunk_sizes<R: Reader>(&mut self, reader: &R) {
        for i in 0..self.size as usize {
            self.num_bytes_bytes[i] = reader.uint32_le(None);
        }
    }
}
