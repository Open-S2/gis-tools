use super::{
    ItemReader,
    arithmetic_decoder::{ArithmeticDecoder, ArithmeticModel},
    constants::{NUMBER_RETURN_LEVEL, NUMBER_RETURN_MAP},
    integer_compressor::IntegerCompressor,
};
use crate::{
    parsers::{RGBA, Reader},
    readers::{
        LASPoint,
        util::{U64I64F64, ValueType64, u8_clamp, u8_fold, u32_zero_bit0},
    },
};
use alloc::{rc::Rc, vec, vec::Vec};
use core::cell::RefCell;

const LASZIP_GPSTIME_MULTI: i32 = 500;
const LASZIP_GPSTIME_MULTI_MINUS: i32 = -10;
const LASZIP_GPSTIME_MULTI_UNCHANGED: i32 = LASZIP_GPSTIME_MULTI - LASZIP_GPSTIME_MULTI_MINUS + 1;
const LASZIP_GPSTIME_MULTI_CODE_FULL: i32 = LASZIP_GPSTIME_MULTI - LASZIP_GPSTIME_MULTI_MINUS + 2;
const LASZIP_GPSTIME_MULTI_TOTAL: i32 = LASZIP_GPSTIME_MULTI - LASZIP_GPSTIME_MULTI_MINUS + 6;

/// Streaming Median 5
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct StreamingMedian5 {
    /// contains the last 5 values
    pub values: [i32; 5],
    /// true if the last value was the highest
    pub high: bool,
}
impl StreamingMedian5 {
    /// Creates a new StreamingMedian5
    pub fn new() -> Self {
        let mut sm5 = StreamingMedian5::default();
        sm5.init();
        sm5
    }
    /// Initializes the StreamingMedian5
    pub fn init(&mut self) {
        self.values = [0; 5];
        self.high = true;
    }

    /// add a new value
    /// @param v - the new value to add
    pub fn add(&mut self, v: i32) {
        let Self { high, values } = self;
        if *high {
            if v < values[2] {
                values[4] = values[3];
                values[3] = values[2];
                if v < values[0] {
                    values[2] = values[1];
                    values[1] = values[0];
                    values[0] = v;
                } else if v < values[1] {
                    values[2] = values[1];
                    values[1] = v;
                } else {
                    values[2] = v;
                }
            } else {
                if v < values[3] {
                    values[4] = values[3];
                    values[3] = v;
                } else {
                    values[4] = v;
                }
                self.high = false;
            }
        } else if values[2] < v {
            values[0] = values[1];
            values[1] = values[2];
            if values[4] < v {
                values[2] = values[3];
                values[3] = values[4];
                values[4] = v;
            } else if values[3] < v {
                values[2] = values[3];
                values[3] = v;
            } else {
                values[2] = v;
            }
        } else {
            if values[1] < v {
                values[0] = values[1];
                values[1] = v;
            } else {
                values[0] = v;
            }
            self.high = true;
        }
    }

    /// @returns the median value
    pub fn get(&self) -> i32 {
        self.values[2]
    }
}

/// LAZ Point10 2.0 Reader
#[derive(Debug)]
pub struct LAZPoint10v2Reader<T: Reader> {
    /// The arithmetic decoder
    dec: Rc<RefCell<ArithmeticDecoder<T>>>,
    /// The last point
    last_item: LASPoint,
    /// The last increment
    /// TODO: Can we drop this
    pub last_incr: i32,
    ic_dx: IntegerCompressor<T>,
    ic_dy: IntegerCompressor<T>,
    ic_z: IntegerCompressor<T>,
    ic_intensity: IntegerCompressor<T>,
    last_intensity: [u16; 16],
    last_x_diff_median5: [StreamingMedian5; 16],
    last_y_diff_median5: [StreamingMedian5; 16],
    last_height: [i32; 8],
    ic_point_source_id: IntegerCompressor<T>,
    m_changed_values: ArithmeticModel,
    m_bit_byte: [Option<ArithmeticModel>; 256],
    m_classification: [Option<ArithmeticModel>; 256],
    m_user_data: [Option<ArithmeticModel>; 256],
    m_scan_angle_rank: [ArithmeticModel; 2],
}
impl<T: Reader> LAZPoint10v2Reader<T> {
    /// Create a new LAZPoint10v2Reader
    pub fn new(dec: Rc<RefCell<ArithmeticDecoder<T>>>) -> Self {
        Self {
            dec: dec.clone(),
            last_item: LASPoint::default(),
            last_incr: 0,
            ic_dx: IntegerCompressor::new(dec.clone(), Some(32), Some(2), None, None),
            ic_dy: IntegerCompressor::new(dec.clone(), Some(32), Some(22), None, None),
            ic_z: IntegerCompressor::new(dec.clone(), Some(32), Some(20), None, None),
            ic_intensity: IntegerCompressor::new(dec.clone(), Some(16), Some(4), None, None),
            last_intensity: [0; 16],
            last_x_diff_median5: [StreamingMedian5::new(); 16],
            last_y_diff_median5: [StreamingMedian5::new(); 16],
            last_height: [0; 8],
            ic_point_source_id: IntegerCompressor::new(dec.clone(), Some(16), None, None, None),
            m_changed_values: ArithmeticModel::new(64, false),
            m_bit_byte: [const { None }; 256],
            m_classification: [const { None }; 256],
            m_user_data: [const { None }; 256],
            m_scan_angle_rank: [ArithmeticModel::new(256, false), ArithmeticModel::new(256, false)],
        }
    }
}
impl<T: Reader> ItemReader for LAZPoint10v2Reader<T> {
    fn init<R: Reader>(&mut self, item: &R, point: &mut LASPoint, _context: &mut u32) {
        // init state
        for i in 0..16 {
            self.last_x_diff_median5[i].init();
            self.last_y_diff_median5[i].init();
            self.last_intensity[i] = 0;
            self.last_height[i / 2] = 0;
        }
        // init models and integer compressors
        self.m_changed_values.init(None);
        self.ic_intensity.init_decompressor();
        self.m_scan_angle_rank[0].init(None);
        self.m_scan_angle_rank[1].init(None);
        self.ic_point_source_id.init_decompressor();
        for i in 0..256 {
            if let Some(m) = &mut self.m_bit_byte[i] {
                m.init(None);
            }
            if let Some(m) = &mut self.m_classification[i] {
                m.init(None);
            }
            if let Some(m) = &mut self.m_user_data[i] {
                m.init(None);
            }
        }
        self.ic_dx.init_decompressor();
        self.ic_dy.init_decompressor();
        self.ic_z.init_decompressor();

        // init last item
        self.last_item.inject_point10(item, 0);
        point.inject_point10(item, 0);
        // but set intensity to zero
        self.last_item.intensity = 0;
    }

    fn read(&mut self, item: &mut LASPoint, _context: &mut u32) {
        let r: usize;
        let n: usize;
        let m: usize;
        let l: usize;
        let mut k_bits;
        let mut median;
        let mut diff;
        // decompress which other values have changed
        let changed_values = self.dec.borrow_mut().decode_symbol(&mut self.m_changed_values) as i32;
        if changed_values != 0 {
            // decompress the edge_of_flight_line, scan_direction_flag, ... if it has changed
            if (changed_values & 32) != 0 {
                if self.m_bit_byte[self.last_item.flags as usize].is_none() {
                    let mut m = ArithmeticModel::new(256, false);
                    m.init(None);
                    self.m_bit_byte[self.last_item.flags as usize] = Some(m);
                }
                self.last_item.set_flags(
                    self.dec.borrow_mut().decode_symbol(
                        self.m_bit_byte[self.last_item.flags as usize].as_mut().unwrap(),
                    ) as u8,
                    false,
                );
            }
            r = self.last_item.return_number as usize;
            n = self.last_item.number_of_returns as usize;
            m = NUMBER_RETURN_MAP[n][r] as usize;
            l = NUMBER_RETURN_LEVEL[n][r] as usize;
            // decompress the intensity if it has changed
            if (changed_values & 16) != 0 {
                self.last_item.intensity = self
                    .ic_intensity
                    .decompress(self.last_intensity[m] as i32, if m < 3 { m as u32 } else { 3 })
                    as u16;
                self.last_intensity[m] = self.last_item.intensity;
            } else {
                self.last_item.intensity = self.last_intensity[m];
            }
            // decompress the classification ... if it has changed
            if (changed_values & 8) != 0 {
                if self.m_classification[self.last_item.classification as usize].is_none() {
                    let mut m = ArithmeticModel::new(256, false);
                    m.init(None);
                    self.m_classification[self.last_item.classification as usize] = Some(m);
                }
                self.last_item.set_flags2(self.dec.borrow_mut().decode_symbol(
                    self.m_classification[self.last_item.classification as usize].as_mut().unwrap(),
                ) as u8);
            }
            // decompress the scan_angle_rank ... if it has changed
            if (changed_values & 4) != 0 {
                let mut val = self.dec.borrow_mut().decode_symbol(
                    &mut self.m_scan_angle_rank[self.last_item.scan_direction_flag as usize],
                );
                let sar = self.last_item.scan_angle_rank;
                if sar < 0 {
                    val = (val as i32 + sar as i32) as u32;
                } else {
                    val += sar as u32;
                }
                self.last_item.scan_angle_rank = u8_fold(val) as i8;
            }
            // decompress the user_data ... if it has changed
            if (changed_values & 2) != 0 {
                if self.m_user_data[self.last_item.user_data as usize].is_none() {
                    let mut m = ArithmeticModel::new(256, false);
                    m.init(None);
                    self.m_user_data[self.last_item.user_data as usize] = Some(m);
                }
                self.last_item.user_data = self.dec.borrow_mut().decode_symbol(
                    self.m_user_data[self.last_item.user_data as usize].as_mut().unwrap(),
                ) as u8;
            }
            // decompress the point_source_id ... if it has changed
            if (changed_values & 1) != 0 {
                self.last_item.point_source_id =
                    self.ic_point_source_id.decompress(self.last_item.point_source_id as i32, 0)
                        as u16;
            }
        } else {
            r = self.last_item.return_number as usize;
            n = self.last_item.number_of_returns as usize;
            m = NUMBER_RETURN_MAP[n][r] as usize;
            l = NUMBER_RETURN_LEVEL[n][r] as usize;
        }
        // decompress x coordinate
        median = self.last_x_diff_median5[m].get();
        diff = self.ic_dx.decompress(median, if n == 1 { 1 } else { 0 });
        self.last_item.x += diff;
        self.last_x_diff_median5[m].add(diff);
        // decompress y coordinate
        median = self.last_y_diff_median5[m].get();
        k_bits = self.ic_dx.get_k();
        diff = self.ic_dy.decompress(
            median,
            (n == 1) as u32 + if k_bits < 20 { u32_zero_bit0(k_bits) } else { 20 },
        );
        self.last_item.y += diff;
        self.last_y_diff_median5[m].add(diff);
        // decompress z coordinate
        k_bits = (self.ic_dx.get_k() + self.ic_dy.get_k()) / 2;
        self.last_item.z = self.ic_z.decompress(
            self.last_height[l],
            (n == 1) as u32 + if k_bits < 18 { u32_zero_bit0(k_bits) } else { 18 },
        );
        self.last_height[l] = self.last_item.z;

        // copy in the last point
        *item = self.last_item.clone();
    }

    fn chunk_sizes<R: Reader>(&mut self, _reader: &R) {}
}

/// Parse LAZ GPS Time 1.1v2
#[derive(Debug)]
pub struct LAZGpsTime11v2Reader<T: Reader> {
    dec: Rc<RefCell<ArithmeticDecoder<T>>>,
    m_gpstime_multi: ArithmeticModel,
    m_gpstime0diff: ArithmeticModel,
    ic_gpstime: IntegerCompressor<T>,
    last: u32,
    next: u32,
    last_gpstime: [U64I64F64; 4],
    last_gpstime_diff: [i32; 4],
    multi_extreme_counter: [i32; 4],
}
impl<T: Reader> LAZGpsTime11v2Reader<T> {
    /// Create a new LAZGpsTime11v2Reader
    pub fn new(dec: Rc<RefCell<ArithmeticDecoder<T>>>) -> Self {
        LAZGpsTime11v2Reader {
            dec: dec.clone(),
            m_gpstime_multi: ArithmeticModel::new(LASZIP_GPSTIME_MULTI_TOTAL as u32, false),
            m_gpstime0diff: ArithmeticModel::new(6, false),
            ic_gpstime: IntegerCompressor::new(dec, Some(32), Some(9), None, None),
            last: 0,
            next: 0,
            last_gpstime: [
                U64I64F64::new(0_u64, ValueType64::U64),
                U64I64F64::new(0_u64, ValueType64::U64),
                U64I64F64::new(0_u64, ValueType64::U64),
                U64I64F64::new(0_u64, ValueType64::U64),
            ],
            last_gpstime_diff: [0, 0, 0, 0],
            multi_extreme_counter: [0, 0, 0, 0],
        }
    }
}
impl<T: Reader> ItemReader for LAZGpsTime11v2Reader<T> {
    fn init<R: Reader>(&mut self, item: &R, point: &mut LASPoint, _context: &mut u32) {
        // init state
        self.last = 0;
        self.next = 0;
        self.last_gpstime_diff = [0, 0, 0, 0];
        self.multi_extreme_counter = [0, 0, 0, 0];
        // init models and integer compressors
        self.m_gpstime_multi.init(None);
        self.m_gpstime0diff.init(None);
        self.ic_gpstime.init_decompressor();
        // init last item
        self.last_gpstime[0].set_u64(item.uint64_le(Some(0)));
        self.last_gpstime[1].set_u64(0);
        self.last_gpstime[2].set_u64(0);
        self.last_gpstime[3].set_u64(0);

        point.gps_time = Some(self.last_gpstime[0].f64());
    }

    fn read(&mut self, item: &mut LASPoint, _context: &mut u32) {
        let mut multi: i32; // I32
        if self.last_gpstime_diff[self.last as usize] == 0 {
            // if the last integer difference was zero
            multi = self.dec.borrow_mut().decode_symbol(&mut self.m_gpstime0diff) as i32;
            if multi == 1 {
                // the difference can be represented with 32 bits
                self.last_gpstime_diff[self.last as usize] = self.ic_gpstime.decompress(0, 0);
                self.last_gpstime[self.last as usize].set_i64(
                    self.last_gpstime[self.last as usize].i64()
                        + self.last_gpstime_diff[self.last as usize] as i64,
                );
                self.multi_extreme_counter[self.last as usize] = 0;
            } else if multi == 2 {
                // the difference is huge
                self.next = (self.next + 1) & 3;
                self.last_gpstime[self.next as usize].set_u64(
                    self.ic_gpstime
                        .decompress((self.last_gpstime[self.last as usize].u64() >> 32) as i32, 8)
                        as u64,
                );
                self.last_gpstime[self.next as usize]
                    .set_u64(self.last_gpstime[self.next as usize].u64() << 32);
                self.last_gpstime[self.next as usize].set_u64(
                    self.last_gpstime[self.next as usize].u64()
                        | self.dec.borrow_mut().read_int() as u64,
                );
                self.last = self.next;
                self.last_gpstime_diff[self.last as usize] = 0;
                self.multi_extreme_counter[self.last as usize] = 0;
            } else if multi > 2 {
                // we switch to another sequence
                self.last = ((self.last as i32 + multi - 2) & 3) as u32;
                self.read(item, _context);
            }
        } else {
            multi = self.dec.borrow_mut().decode_symbol(&mut self.m_gpstime_multi) as i32;
            if multi == 1 {
                self.last_gpstime[self.last as usize].set_i64(
                    self.last_gpstime[self.last as usize].i64()
                        + self.ic_gpstime.decompress(self.last_gpstime_diff[self.last as usize], 1)
                            as i64,
                );
                self.multi_extreme_counter[self.last as usize] = 0;
            } else if multi < LASZIP_GPSTIME_MULTI_UNCHANGED {
                let gpstime_diff;
                if multi == 0 {
                    gpstime_diff = self.ic_gpstime.decompress(0, 7);
                    self.multi_extreme_counter[self.last as usize] += 1;
                    if self.multi_extreme_counter[self.last as usize] > 3 {
                        self.last_gpstime_diff[self.last as usize] = gpstime_diff;
                        self.multi_extreme_counter[self.last as usize] = 0;
                    }
                } else if multi < LASZIP_GPSTIME_MULTI {
                    if multi < 10 {
                        gpstime_diff = self
                            .ic_gpstime
                            .decompress(multi * self.last_gpstime_diff[self.last as usize], 2);
                    } else {
                        gpstime_diff = self
                            .ic_gpstime
                            .decompress(multi * self.last_gpstime_diff[self.last as usize], 3);
                    }
                } else if multi == LASZIP_GPSTIME_MULTI {
                    gpstime_diff = self.ic_gpstime.decompress(
                        LASZIP_GPSTIME_MULTI * self.last_gpstime_diff[self.last as usize],
                        4,
                    );
                    self.multi_extreme_counter[self.last as usize] += 1;
                    if self.multi_extreme_counter[self.last as usize] > 3 {
                        self.last_gpstime_diff[self.last as usize] = gpstime_diff;
                        self.multi_extreme_counter[self.last as usize] = 0;
                    }
                } else {
                    multi = LASZIP_GPSTIME_MULTI - multi;
                    if multi > LASZIP_GPSTIME_MULTI_MINUS {
                        gpstime_diff = self
                            .ic_gpstime
                            .decompress(multi * self.last_gpstime_diff[self.last as usize], 5);
                    } else {
                        gpstime_diff = self.ic_gpstime.decompress(
                            LASZIP_GPSTIME_MULTI_MINUS * self.last_gpstime_diff[self.last as usize],
                            6,
                        );
                        self.multi_extreme_counter[self.last as usize] += 1;
                        if self.multi_extreme_counter[self.last as usize] > 3 {
                            self.last_gpstime_diff[self.last as usize] = gpstime_diff;
                            self.multi_extreme_counter[self.last as usize] = 0;
                        }
                    }
                }
                self.last_gpstime[self.last as usize]
                    .set_i64(self.last_gpstime[self.last as usize].i64() + gpstime_diff as i64);
            } else if multi == LASZIP_GPSTIME_MULTI_CODE_FULL {
                self.next = (self.next + 1) & 3;
                self.last_gpstime[self.next as usize].set_u64(
                    self.ic_gpstime
                        .decompress((self.last_gpstime[self.last as usize].u64() >> 32) as i32, 8)
                        as u64,
                );
                self.last_gpstime[self.next as usize]
                    .set_u64(self.last_gpstime[self.next as usize].u64() << 32);
                self.last_gpstime[self.next as usize].set_u64(
                    self.last_gpstime[self.next as usize].u64()
                        | self.dec.borrow_mut().read_int() as u64,
                );
                self.last = self.next;
                self.last_gpstime_diff[self.last as usize] = 0;
                self.multi_extreme_counter[self.last as usize] = 0;
            } else if multi >= LASZIP_GPSTIME_MULTI_CODE_FULL {
                self.last =
                    ((self.last as i32 + multi - LASZIP_GPSTIME_MULTI_CODE_FULL) & 3) as u32;
                self.read(item, _context);
            }
        }

        item.gps_time = Some(self.last_gpstime[self.last as usize].f64());
    }

    fn chunk_sizes<R: Reader>(&mut self, _reader: &R) {}
}

/// Parse LAZ RGB 1.2v2
#[derive(Debug)]
pub struct LAZrgb12v2Reader<T: Reader> {
    dec: Rc<RefCell<ArithmeticDecoder<T>>>,
    last_item: [u16; 3],
    m_byte_used: ArithmeticModel,
    m_rgb_diff0: ArithmeticModel,
    m_rgb_diff1: ArithmeticModel,
    m_rgb_diff2: ArithmeticModel,
    m_rgb_diff3: ArithmeticModel,
    m_rgb_diff4: ArithmeticModel,
    m_rgb_diff5: ArithmeticModel,
}
impl<T: Reader> LAZrgb12v2Reader<T> {
    /// Create a new LAZrgb12v2Reader
    pub fn new(dec: Rc<RefCell<ArithmeticDecoder<T>>>) -> Self {
        Self {
            dec,
            last_item: [0; 3],
            m_byte_used: ArithmeticModel::new(128, false),
            m_rgb_diff0: ArithmeticModel::new(256, false),
            m_rgb_diff1: ArithmeticModel::new(256, false),
            m_rgb_diff2: ArithmeticModel::new(256, false),
            m_rgb_diff3: ArithmeticModel::new(256, false),
            m_rgb_diff4: ArithmeticModel::new(256, false),
            m_rgb_diff5: ArithmeticModel::new(256, false),
        }
    }
}
impl<T: Reader> ItemReader for LAZrgb12v2Reader<T> {
    fn init<R: Reader>(&mut self, item: &R, point: &mut LASPoint, _context: &mut u32) {
        // init models and integer compressors
        self.m_byte_used.init(None);
        self.m_rgb_diff0.init(None);
        self.m_rgb_diff1.init(None);
        self.m_rgb_diff2.init(None);
        self.m_rgb_diff3.init(None);
        self.m_rgb_diff4.init(None);
        self.m_rgb_diff5.init(None);
        // init last item
        let r = item.uint16_le(None);
        let g = item.uint16_le(None);
        let b = item.uint16_le(None);
        self.last_item = [r, g, b];
        point.rgba = Some(RGBA::from_u16s(r, g, b, u16::MAX));
    }

    fn read(&mut self, item: &mut LASPoint, _context: &mut u32) {
        let dec = &mut self.dec.borrow_mut();
        let mut curr_item = self.last_item;

        let mut corr: u8;
        let mut diff: i32;
        let sym = dec.decode_symbol(&mut self.m_byte_used);
        if (sym & 1) != 0 {
            corr = dec.decode_symbol(&mut self.m_rgb_diff0) as u8;
            curr_item[0] = u8_fold(corr as u32 + (self.last_item[0] & 255) as u32) as u16;
        } else {
            curr_item[0] = self.last_item[0] & 0xff;
        }
        if (sym & (1 << 1)) != 0 {
            corr = dec.decode_symbol(&mut self.m_rgb_diff1) as u8;
            curr_item[0] |= (u8_fold(corr as u32 + (self.last_item[0] >> 8) as u32) as u16) << 8;
        } else {
            curr_item[0] |= self.last_item[0] & 0xff00;
        }
        if (sym & (1 << 6)) != 0 {
            diff = (curr_item[0] & 0x00ff) as i32 - (self.last_item[0] & 0x00ff) as i32;
            if (sym & (1 << 2)) != 0 {
                corr = dec.decode_symbol(&mut self.m_rgb_diff2) as u8;
                curr_item[1] = u8_fold(
                    corr as u32 + u8_clamp((diff + (self.last_item[1] & 255) as i32) as u32) as u32,
                ) as u16;
            } else {
                curr_item[1] = self.last_item[1] & 0xff;
            }
            if (sym & (1 << 4)) != 0 {
                corr = dec.decode_symbol(&mut self.m_rgb_diff4) as u8;
                diff = (diff
                    + ((curr_item[1] & 0x00ff) as i32 - (self.last_item[1] & 0x00ff) as i32))
                    / 2;
                curr_item[2] = u8_fold(
                    corr as u32 + u8_clamp((diff + (self.last_item[2] & 255) as i32) as u32) as u32,
                ) as u16;
            } else {
                curr_item[2] = self.last_item[2] & 0xff;
            }
            diff = (curr_item[0] >> 8) as i32 - (self.last_item[0] >> 8) as i32;
            if (sym & (1 << 3)) != 0 {
                corr = dec.decode_symbol(&mut self.m_rgb_diff3) as u8;
                curr_item[1] |= (u8_fold(
                    corr as u32 + u8_clamp((diff + (self.last_item[1] >> 8) as i32) as u32) as u32,
                ) as u16)
                    << 8;
            } else {
                curr_item[1] |= self.last_item[1] & 0xff00;
            }
            if (sym & (1 << 5)) != 0 {
                corr = dec.decode_symbol(&mut self.m_rgb_diff5) as u8;
                diff = (diff + ((curr_item[1] >> 8) as i32 - (self.last_item[1] >> 8) as i32)) / 2;
                curr_item[2] |= (u8_fold(
                    corr as u32 + u8_clamp((diff + (self.last_item[2] >> 8) as i32) as u32) as u32,
                ) as u16)
                    << 8;
            } else {
                curr_item[2] |= self.last_item[2] & 0xff00;
            }
        } else {
            curr_item[1] = curr_item[0];
            curr_item[2] = curr_item[0];
        }

        self.last_item = curr_item;
        item.rgba = Some(RGBA::from_u16s(curr_item[0], curr_item[1], curr_item[2], u16::MAX));
    }

    fn chunk_sizes<R: Reader>(&mut self, _reader: &R) {}
}

/// LAZ byte reader V2
#[derive(Debug)]
pub struct LAZbyte10v2Reader<T: Reader> {
    dec: Rc<RefCell<ArithmeticDecoder<T>>>,
    size: u32,
    m_byte: Vec<ArithmeticModel>,
    last_item: Vec<u8>,
}
impl<T: Reader> LAZbyte10v2Reader<T> {
    /// Create a new LAZbyte10v2Reader
    pub fn new(dec: Rc<RefCell<ArithmeticDecoder<T>>>, size: u32) -> Self {
        Self {
            size,
            last_item: vec![0; size as usize],
            m_byte: vec![ArithmeticModel::new(256, false); size as usize],
            dec,
        }
    }
}
impl<T: Reader> ItemReader for LAZbyte10v2Reader<T> {
    fn init<R: Reader>(&mut self, item: &R, _point: &mut LASPoint, _context: &mut u32) {
        // init models and integer compressors
        for m in self.m_byte.iter_mut() {
            m.init(None);
        }
        // init last item
        self.last_item = item.seek_slice(self.size as usize);
    }

    fn read(&mut self, _item: &mut LASPoint, _context: &mut u32) {
        let mut res = vec![];
        for i in 0..self.size as usize {
            let value =
                self.last_item[i] as u32 + self.dec.borrow_mut().decode_symbol(&mut self.m_byte[i]);
            res.push(u8_fold(value));
        }
        self.last_item = res;
    }

    fn chunk_sizes<R: Reader>(&mut self, _reader: &R) {}
}
