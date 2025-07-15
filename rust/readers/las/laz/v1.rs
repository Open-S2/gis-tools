use super::{
    ItemReader,
    arithmetic_decoder::{ArithmeticDecoder, ArithmeticModel},
    integer_compressor::IntegerCompressor,
};
use crate::{
    parsers::{RGBA, Reader},
    readers::{LASPoint, WavePacket, util::U64I64F64},
};
use alloc::{rc::Rc, vec, vec::Vec};
use core::cell::RefCell;

const LASZIP_GPSTIME_MULTIMAX: u32 = 512;

/// Parse LAZ Point 10.1
#[derive(Debug)]
pub struct LAZPoint10v1Reader<T: Reader> {
    last_item: LASPoint,
    dec: Rc<RefCell<ArithmeticDecoder<T>>>,
    last_x_diff: [i32; 3],
    last_y_diff: [i32; 3],
    last_incr: i32,
    ic_dx: IntegerCompressor<T>,
    ic_dy: IntegerCompressor<T>,
    ic_z: IntegerCompressor<T>,
    ic_intensity: IntegerCompressor<T>,
    ic_scan_angle_rank: IntegerCompressor<T>,
    ic_point_source_id: IntegerCompressor<T>,
    m_changed_values: ArithmeticModel,
    m_bit_byte: [Option<ArithmeticModel>; 256],
    m_classification: [Option<ArithmeticModel>; 256],
    m_user_data: [Option<ArithmeticModel>; 256],
}
impl<T: Reader> LAZPoint10v1Reader<T> {
    /// Create a new LAZPoint10v1Reader
    pub fn new(dec: Rc<RefCell<ArithmeticDecoder<T>>>) -> Self {
        Self {
            dec: dec.clone(),
            last_item: LASPoint::default(),
            last_x_diff: [0, 0, 0],
            last_y_diff: [0, 0, 0],
            last_incr: 0,
            ic_dx: IntegerCompressor::new(dec.clone(), Some(32), None, None, None),
            ic_dy: IntegerCompressor::new(dec.clone(), Some(32), Some(20), None, None),
            ic_z: IntegerCompressor::new(dec.clone(), Some(32), Some(20), None, None),
            ic_intensity: IntegerCompressor::new(dec.clone(), Some(16), None, None, None),
            ic_scan_angle_rank: IntegerCompressor::new(dec.clone(), Some(8), Some(2), None, None),
            ic_point_source_id: IntegerCompressor::new(dec.clone(), Some(16), None, None, None),
            m_changed_values: ArithmeticModel::new(64, false),
            m_bit_byte: [const { None }; 256],
            m_classification: [const { None }; 256],
            m_user_data: [const { None }; 256],
        }
    }
}
impl<T: Reader> ItemReader for LAZPoint10v1Reader<T> {
    fn init<R: Reader>(&mut self, item: &R, point: &mut LASPoint, _context: &mut u32) {
        // init state
        self.last_x_diff = [0, 0, 0];
        self.last_y_diff = [0, 0, 0];
        self.last_incr = 0;
        // init models and integer compressors
        self.ic_dx.init_decompressor();
        self.ic_dy.init_decompressor();
        self.ic_z.init_decompressor();
        self.ic_intensity.init_decompressor();
        self.ic_scan_angle_rank.init_decompressor();
        self.ic_point_source_id.init_decompressor();
        self.m_changed_values.init(None);
        // init "last item" to current item
        self.last_item.inject_point10(item, 0);
        point.inject_point10(item, 0);
    }

    fn read(&mut self, item: &mut LASPoint, context: &mut u32) {
        // find median difference for x and y from 3 preceding differences
        let median_x;
        if self.last_x_diff[0] < self.last_x_diff[1] {
            if self.last_x_diff[1] < self.last_x_diff[2] {
                median_x = self.last_x_diff[1];
            } else if self.last_x_diff[0] < self.last_x_diff[2] {
                median_x = self.last_x_diff[2];
            } else {
                median_x = self.last_x_diff[0];
            }
        } else if self.last_x_diff[0] < self.last_x_diff[2] {
            median_x = self.last_x_diff[0];
        } else if self.last_x_diff[1] < self.last_x_diff[2] {
            median_x = self.last_x_diff[2];
        } else {
            median_x = self.last_x_diff[1];
        }

        let median_y;
        if self.last_y_diff[0] < self.last_y_diff[1] {
            if self.last_y_diff[1] < self.last_y_diff[2] {
                median_y = self.last_y_diff[1];
            } else if self.last_y_diff[0] < self.last_y_diff[2] {
                median_y = self.last_y_diff[2];
            } else {
                median_y = self.last_y_diff[0];
            }
        } else if self.last_y_diff[0] < self.last_y_diff[2] {
            median_y = self.last_y_diff[0];
        } else if self.last_y_diff[1] < self.last_y_diff[2] {
            median_y = self.last_y_diff[2];
        } else {
            median_y = self.last_y_diff[1];
        }

        // decompress x y z coordinates
        let x_diff = self.ic_dx.decompress(median_x, *context);
        self.last_item.x += x_diff;
        // we use the number k of bits corrector bits to switch contexts
        let mut k_bits = self.ic_dx.get_k();
        let y_diff = self.ic_dy.decompress(median_y, if k_bits < 19 { k_bits } else { 19 });
        self.last_item.y += y_diff;
        k_bits = (k_bits + self.ic_dy.get_k()) / 2;
        self.last_item.z =
            self.ic_z.decompress(self.last_item.z, if k_bits < 19 { k_bits } else { 19 });

        // decompress which other values have changed
        let changed_values = self.dec.borrow_mut().decode_symbol(&mut self.m_changed_values);

        if changed_values != 0 {
            // decompress the intensity if it has changed
            if (changed_values & 32) != 0 {
                self.last_item.intensity =
                    self.ic_intensity.decompress(self.last_item.intensity as i32, *context) as u16;
            }

            // decompress the edge_of_flight_line, scan_direction_flag, ... if it has changed
            if (changed_values & 16) != 0 {
                if self.m_bit_byte[self.last_item.flags as usize].is_none() {
                    let mut model = ArithmeticModel::new(256, false);
                    model.init(None);
                    self.m_bit_byte[self.last_item.flags as usize] = Some(model);
                }
                self.last_item.set_flags(
                    self.dec.borrow_mut().decode_symbol(
                        self.m_bit_byte[self.last_item.flags as usize].as_mut().unwrap(),
                    ) as u8,
                    false,
                );
            }

            // decompress the classification ... if it has changed
            if (changed_values & 8) != 0 {
                if self.m_classification[self.last_item.classification as usize].is_none() {
                    let mut model = ArithmeticModel::new(256, false);
                    model.init(None);
                    self.m_classification[self.last_item.classification as usize] = Some(model);
                }
                self.last_item.set_flags2(self.dec.borrow_mut().decode_symbol(
                    self.m_classification[self.last_item.classification as usize].as_mut().unwrap(),
                ) as u8);
            }

            // decompress the scan_angle_rank ... if it has changed
            if (changed_values & 4) != 0 {
                self.last_item.scan_angle_rank = self.ic_scan_angle_rank.decompress(
                    self.last_item.scan_angle_rank as i32,
                    if k_bits < 3 { 1 } else { 0 },
                ) as i8;
            }

            // decompress the user_data ... if it has changed
            if (changed_values & 2) != 0 {
                if self.m_user_data[self.last_item.user_data as usize].is_none() {
                    let mut model = ArithmeticModel::new(256, false);
                    model.init(None);
                    self.m_user_data[self.last_item.user_data as usize] = Some(model);
                }
                self.last_item.user_data = self.dec.borrow_mut().decode_symbol(
                    self.m_user_data[self.last_item.user_data as usize].as_mut().unwrap(),
                ) as u8;
            }

            // decompress the point_source_ID ... if it has changed
            if (changed_values & 1) != 0 {
                self.last_item.point_source_id = self
                    .ic_point_source_id
                    .decompress(self.last_item.point_source_id as i32, *context)
                    as u16;
            }
        }

        // record the difference
        self.last_x_diff[self.last_incr as usize] = x_diff;
        self.last_y_diff[self.last_incr as usize] = y_diff;
        self.last_incr += 1;
        if self.last_incr > 2 {
            self.last_incr = 0;
        }

        *item = self.last_item.clone();
    }

    fn chunk_sizes<R: Reader>(&mut self, _reader: &R) {}
}

/// Parse LAZ GPS Time 1.1v1
#[derive(Debug)]
pub struct LAZGpsTime11v1Reader<T: Reader> {
    last_item: U64I64F64,
    dec: Rc<RefCell<ArithmeticDecoder<T>>>,
    m_gpstime_multi: ArithmeticModel,
    m_gpstime0_diff: ArithmeticModel,
    ic_gpstime: IntegerCompressor<T>,
    last_item_diff: i64,
    multi_extreme_counter: u8,
}
impl<T: Reader> LAZGpsTime11v1Reader<T> {
    /// Create a new LAZPoint10v1Reader
    pub fn new(dec: Rc<RefCell<ArithmeticDecoder<T>>>) -> Self {
        Self {
            dec: dec.clone(),
            // create entropy models and integer compressors
            m_gpstime_multi: ArithmeticModel::new(LASZIP_GPSTIME_MULTIMAX, false),
            m_gpstime0_diff: ArithmeticModel::new(3, false),
            ic_gpstime: IntegerCompressor::new(dec, Some(32), Some(6), None, None),
            last_item_diff: 0,
            multi_extreme_counter: 0,
            last_item: U64I64F64::default(),
        }
    }
}
impl<T: Reader> ItemReader for LAZGpsTime11v1Reader<T> {
    fn init<R: Reader>(&mut self, item: &R, point: &mut LASPoint, _context: &mut u32) {
        // init state
        self.last_item_diff = 0;
        self.multi_extreme_counter = 0;
        // init models and integer compressors
        self.m_gpstime_multi.init(None);
        self.m_gpstime0_diff.init(None);
        self.ic_gpstime.init_decompressor();
        // init last item
        self.last_item.set_u64(item.uint64_le(Some(0)));
        point.gps_time = Some(self.last_item.f64());
    }

    fn read(&mut self, item: &mut LASPoint, _context: &mut u32) {
        let multi;
        if self.last_item_diff == 0 {
            // if the last integer difference was zero
            multi = self.dec.borrow_mut().decode_symbol(&mut self.m_gpstime0_diff);
            if multi == 1 {
                // the difference can be represented with 32 bits
                self.last_item_diff = self.ic_gpstime.decompress(0, 0) as i64;
                self.last_item.set_i64(self.last_item_diff + self.last_item.i64());
            } else if multi == 2 {
                // the difference is huge
                self.last_item.set_u64(self.dec.borrow_mut().read_int64());
            }
        } else {
            multi = self.dec.borrow_mut().decode_symbol(&mut self.m_gpstime_multi);

            if multi < LASZIP_GPSTIME_MULTIMAX - 2 {
                let gpstime_diff; // I32
                if multi == 1 {
                    gpstime_diff = self.ic_gpstime.decompress(self.last_item_diff as i32, 1);
                    self.last_item_diff = gpstime_diff as i64;
                    self.multi_extreme_counter = 0;
                } else if multi == 0 {
                    gpstime_diff = self.ic_gpstime.decompress((self.last_item_diff / 4) as i32, 2);
                    self.multi_extreme_counter += 1;
                    if self.multi_extreme_counter > 3 {
                        self.last_item_diff = gpstime_diff as i64;
                        self.multi_extreme_counter = 0;
                    }
                } else if multi < 10 {
                    gpstime_diff =
                        self.ic_gpstime.decompress((multi as i64 * self.last_item_diff) as i32, 3);
                } else if multi < 50 {
                    gpstime_diff =
                        self.ic_gpstime.decompress((multi as i64 * self.last_item_diff) as i32, 4);
                } else {
                    gpstime_diff =
                        self.ic_gpstime.decompress((multi as i64 * self.last_item_diff) as i32, 5);
                    if multi == LASZIP_GPSTIME_MULTIMAX - 3 {
                        self.multi_extreme_counter += 1;
                        if self.multi_extreme_counter > 3 {
                            self.last_item_diff = gpstime_diff as i64;
                            self.multi_extreme_counter = 0;
                        }
                    }
                }
                self.last_item.set_i64(gpstime_diff as i64 + self.last_item.i64());
            } else if multi < LASZIP_GPSTIME_MULTIMAX - 1 {
                self.last_item.set_u64(self.dec.borrow_mut().read_int64());
            }
        }

        item.gps_time = Some(self.last_item.f64());
    }

    fn chunk_sizes<R: Reader>(&mut self, _reader: &R) {}
}

/// Parse LAZ RGB 1.2v1
#[derive(Debug)]
pub struct LAZrgb12v1Reader<T: Reader> {
    /// The arithmetic decoder
    last_item: [u16; 3],
    dec: Rc<RefCell<ArithmeticDecoder<T>>>,
    m_byte_used: ArithmeticModel,
    ic_rgb: IntegerCompressor<T>,
}
impl<T: Reader> LAZrgb12v1Reader<T> {
    /// Create a new LAZrgb12v1Reader
    pub fn new(dec: Rc<RefCell<ArithmeticDecoder<T>>>) -> Self {
        Self {
            dec: dec.clone(),
            // create models and integer compressors
            m_byte_used: ArithmeticModel::new(64, false),
            ic_rgb: IntegerCompressor::new(dec, Some(8), Some(6), None, None),
            last_item: [0; 3],
        }
    }
}
impl<T: Reader> ItemReader for LAZrgb12v1Reader<T> {
    fn init<R: Reader>(&mut self, item: &R, point: &mut LASPoint, _context: &mut u32) {
        // init models and integer compressors
        self.m_byte_used.init(None);
        self.ic_rgb.init_decompressor();
        let r = item.uint16_le(None);
        let g = item.uint16_le(None);
        let b = item.uint16_le(None);
        self.last_item = [r, g, b];
        point.rgba = Some(RGBA::from_u16s(r, g, b, u16::MAX));
    }

    fn read(&mut self, item: &mut LASPoint, _context: &mut u32) {
        let mut curr_item: [u16; 3] = [0; 3];
        let sym = self.dec.borrow_mut().decode_symbol(&mut self.m_byte_used);
        if (sym & (1 << 0)) != 0 {
            curr_item[0] = self.ic_rgb.decompress((self.last_item[0] & 255) as i32, 0) as u16;
        } else {
            curr_item[0] = self.last_item[0] & 0xff;
        }
        if (sym & (1 << 1)) != 0 {
            curr_item[0] |=
                (self.ic_rgb.decompress((self.last_item[0] >> 8) as i32, 1) as u16) << 8;
        } else {
            curr_item[0] |= self.last_item[0] & 0xff00;
        }
        if (sym & (1 << 2)) != 0 {
            curr_item[1] = self.ic_rgb.decompress((self.last_item[1] & 255) as i32, 2) as u16;
        } else {
            curr_item[1] = self.last_item[1] & 0xff;
        }
        if (sym & (1 << 3)) != 0 {
            curr_item[1] |=
                (self.ic_rgb.decompress((self.last_item[1] >> 8) as i32, 3) as u16) << 8;
        } else {
            curr_item[1] |= self.last_item[1] & 0xff00;
        }
        if (sym & (1 << 4)) != 0 {
            curr_item[2] = self.ic_rgb.decompress((self.last_item[2] & 255) as i32, 4) as u16;
        } else {
            curr_item[2] = self.last_item[2] & 0xff;
        }
        if (sym & (1 << 5)) != 0 {
            curr_item[2] |=
                (self.ic_rgb.decompress((self.last_item[2] >> 8) as i32, 5) as u16) << 8;
        } else {
            curr_item[2] |= self.last_item[2] & 0xff00;
        }

        self.last_item = curr_item;
        item.rgba = Some(RGBA::from_u16s(curr_item[0], curr_item[1], curr_item[2], u16::MAX));
    }

    fn chunk_sizes<R: Reader>(&mut self, _reader: &R) {}
}

/// Parse LAZ wavepacket 1.3v1
#[derive(Debug)]
pub struct LAZwavepacket13v1Reader<T: Reader> {
    dec: Rc<RefCell<ArithmeticDecoder<T>>>,
    index: u32,
    last_item: WavePacket,
    m_packet_index: ArithmeticModel,
    m_offset_diff: [ArithmeticModel; 4],
    ic_offset_diff: IntegerCompressor<T>,
    ic_packet_size: IntegerCompressor<T>,
    ic_return_point: IntegerCompressor<T>,
    ic_xyz: IntegerCompressor<T>,
    last_diff32: i32,
    sym_last_offset_diff: u32,
}
impl<T: Reader> LAZwavepacket13v1Reader<T> {
    /// Create a new LAZwavepacket13v1Reader
    pub fn new(dec: Rc<RefCell<ArithmeticDecoder<T>>>) -> Self {
        Self {
            index: 0,
            dec: dec.clone(),
            // create models and integer compressors
            m_packet_index: ArithmeticModel::new(256, false),
            m_offset_diff: [
                ArithmeticModel::new(4, false),
                ArithmeticModel::new(4, false),
                ArithmeticModel::new(4, false),
                ArithmeticModel::new(4, false),
            ],
            ic_offset_diff: IntegerCompressor::new(dec.clone(), Some(32), None, None, None),
            ic_packet_size: IntegerCompressor::new(dec.clone(), Some(32), None, None, None),
            ic_return_point: IntegerCompressor::new(dec.clone(), Some(32), None, None, None),
            ic_xyz: IntegerCompressor::new(dec, Some(32), Some(3), None, None),
            last_item: WavePacket::default(),
            last_diff32: 0,
            sym_last_offset_diff: 0,
        }
    }
}
impl<T: Reader> ItemReader for LAZwavepacket13v1Reader<T> {
    fn init<R: Reader>(&mut self, item: &R, point: &mut LASPoint, _context: &mut u32) {
        // init state
        self.index = 0;
        self.last_diff32 = 0;
        self.sym_last_offset_diff = 0;

        // init models and integer compressors
        self.m_packet_index.init(None);
        for m in self.m_offset_diff.iter_mut() {
            m.init(None);
        }
        self.ic_offset_diff.init_decompressor();
        self.ic_packet_size.init_decompressor();
        self.ic_return_point.init_decompressor();
        self.ic_xyz.init_decompressor();

        self.last_item = WavePacket::from_reader(item, 0);
        point.wave_packet = Some(self.last_item.clone());
    }

    fn read(&mut self, item: &mut LASPoint, _context: &mut u32) {
        // let this_item_m = new LASWavePacket13(item);
        let mut this_item_m = WavePacket::default();
        self.index = self.dec.borrow_mut().decode_symbol(&mut self.m_packet_index);

        self.sym_last_offset_diff = self
            .dec
            .borrow_mut()
            .decode_symbol(&mut self.m_offset_diff[self.sym_last_offset_diff as usize]);

        if self.sym_last_offset_diff == 0 {
            this_item_m.offset = self.last_item.offset;
        } else if self.sym_last_offset_diff == 1 {
            this_item_m.offset = self.last_item.offset + self.last_item.length as u64;
        } else if self.sym_last_offset_diff == 2 {
            self.last_diff32 = self.ic_offset_diff.decompress(self.last_diff32, 0);
            this_item_m.offset = (self.last_item.offset as i32 + self.last_diff32) as u64;
        } else {
            this_item_m.offset = self.dec.borrow_mut().read_int64();
        }

        this_item_m.length = self.ic_packet_size.decompress(self.last_item.length as i32, 0) as u32;
        this_item_m.return_point =
            self.ic_return_point.decompress(self.last_item.return_point as i32, 0) as f32;
        this_item_m.x_t = self.ic_xyz.decompress(self.last_item.x_t as i32, 0) as f32;
        this_item_m.y_t = self.ic_xyz.decompress(self.last_item.y_t as i32, 1) as f32;
        this_item_m.z_t = self.ic_xyz.decompress(self.last_item.z_t as i32, 2) as f32;

        // this_item_m.copyTo(self.last_item.data, 29);
        item.wave_packet = Some(this_item_m.clone());
        self.last_item = this_item_m;
    }

    fn chunk_sizes<R: Reader>(&mut self, _reader: &R) {}
}

/// Parse LAZ byte 1.0v1
#[derive(Debug)]
pub struct LAZbyte10v1Reader<T: Reader> {
    size: u32,
    last_item: Vec<u8>,
    ic_byte: IntegerCompressor<T>,
}
impl<T: Reader> LAZbyte10v1Reader<T> {
    /// Create a new LAZbyte10v1Reader
    pub fn new(dec: Rc<RefCell<ArithmeticDecoder<T>>>, size: u32) -> Self {
        Self {
            size,
            last_item: vec![0; size as usize],
            ic_byte: IntegerCompressor::new(dec, Some(8), Some(size), None, None),
        }
    }
}
impl<T: Reader> ItemReader for LAZbyte10v1Reader<T> {
    fn init<R: Reader>(&mut self, item: &R, _point: &mut LASPoint, _context: &mut u32) {
        // init models and integer compressors
        self.ic_byte.init_decompressor();
        self.last_item = item.seek_slice(self.size as usize);
    }

    fn read(&mut self, _item: &mut LASPoint, _context: &mut u32) {
        let mut this_item = vec![0; self.size as usize];
        for (i, val) in self.last_item.iter().enumerate() {
            this_item[i] = self.ic_byte.decompress(*val as i32, i as u32);
        }
        self.last_item = this_item.iter().map(|x| *x as u8).collect();
    }

    fn chunk_sizes<R: Reader>(&mut self, _reader: &R) {}
}
