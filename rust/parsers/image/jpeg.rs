use alloc::{collections::BTreeMap, rc::Rc, string::String, vec, vec::Vec};
use core::cell::RefCell;
use libm::ceil;

// TODO: Instead we should use https://gitlab.com/libtiff/libtiff/-/blob/master/libtiff/tif_jpeg.c?ref_type=heads

// /* -*- tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- /
// vim: set shiftwidth=2 tabstop=2 autoindent cindent expandtab:
// Copyright 2011 notmasteryet
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// - The JPEG specification can be found in the ITU CCITT Recommendation T.81
//   (www.w3.org/Graphics/JPEG/itu-t81.pdf)
// - The JFIF specification can be found in the JPEG File Interchange Format
//   (www.w3.org/Graphics/JPEG/jfif3.pdf)
// - The Adobe Application-Specific JPEG markers in the Supporting the DCT Filters
//   in PostScript Level 2, Technical Note #5116
//   (partners.adobe.com/public/developer/en/ps/sdk/5116.DCT_Filter.pdf)

const DCT_COS1: i32 = 4017; // cos(pi/16)
const DCT_SIN1: i32 = 799; // sin(pi/16)
const DCT_COS3: i32 = 3406; // cos(3*pi/16)
const DCT_SIN3: i32 = 2276; // sin(3*pi/16)
const DCT_COS6: i32 = 1567; // cos(6*pi/16)
const DCT_SIN6: i32 = 3784; // sin(6*pi/16)
const DCT_SQRT2: i32 = 5793; // sqrt(2)
const DCT_SQRT1D2: i32 = 2896; // sqrt(2) / 2

/// JPEG Options
#[derive(Debug, Clone)]
pub struct JPEGOptions {
    /// Skip mutation
    pub skip_mutation: bool,
    /// Color transform
    pub color_transform: Option<bool>,
    /// Format as RGBA
    pub format_as_rgba: bool,
    /// Tolerant decoding
    pub tolerant_decoding: bool,
    /// Don't decode more than 100 megapixels
    pub max_resolution_in_mp: usize,
    /// Don't decode if memory footprint is more than 512MB
    pub max_memory_usage_in_mb: usize,
}
impl Default for JPEGOptions {
    fn default() -> Self {
        Self {
            skip_mutation: false,
            color_transform: None,
            format_as_rgba: true,
            tolerant_decoding: true,
            max_resolution_in_mp: 100,
            max_memory_usage_in_mb: 512,
        }
    }
}

/// A Component of a JPEG image
#[derive(Debug, Default, Clone)]
struct JPEGComponent {
    /// h
    pub h: usize,
    /// v
    pub v: usize,
    /// quantization IDX
    pub quantization_idx: usize,
    /// blocks per line
    pub blocks_per_line: usize,
    /// blocks per column
    pub blocks_per_column: usize,
    /// blocks
    pub blocks: Vec<Vec<Vec<i32>>>,
    /// huffman table DC
    pub huffman_table_dc: Rc<RefCell<Vec<HuffmanNode>>>,
    /// huffman table AC
    pub huffman_table_ac: Rc<RefCell<Vec<HuffmanNode>>>,
    /// quantization table
    pub quantization_table: Vec<i32>,
    /// prediction value
    pub pred: i32,
    /// zig zag dictionary
    pub dct_zig_zag: Vec<i32>,
}

/// A Component of a JPEG image organized into lines
#[derive(Debug, Default, Clone)]
struct OutComponent {
    /// lines
    pub lines: Vec<Vec<u8>>,
    /// scale x
    pub scale_x: usize,
    /// scale y
    pub scale_y: usize,
}

/// A JPEG frame
#[derive(Debug, Default, Clone)]
pub struct JPEGFrame {
    /// Exposing to hide the unused field warning
    pub extended: bool,
    progressive: bool,
    /// Exposing to hide the unused field warning
    pub precision: u8,
    scan_lines: usize,
    samples_per_line: usize,
    components: BTreeMap<i32, Rc<RefCell<JPEGComponent>>>,
    components_order: Vec<i32>,
    max_h: usize,
    max_v: usize,
    mcus_per_line: usize,
    mcus_per_column: usize,
}

/// Adobe APP14 marker
#[derive(Debug, Default, Clone)]
pub struct Adobe {
    /// Version
    pub version: u8,
    /// Flags 0
    pub flags0: usize,
    /// Flags 1
    pub flags1: usize,
    /// Color transform
    pub transform_code: u8,
}

/// JFIF marker Version
#[derive(Debug, Default, Clone)]
pub struct JFIFVersion {
    /// Major version
    pub major: u8,
    /// Minor version
    pub minor: u8,
}

/// JFIF marker
#[derive(Debug, Default, Clone)]
pub struct JFIF {
    /// JFIF version
    pub version: JFIFVersion,
    /// Density units
    pub density_units: usize,
    /// X Density
    pub x_density: usize,
    /// Y Density
    pub y_density: usize,
    /// Thumbnail width
    pub thumb_width: usize,
    /// Thumbnail height
    pub thumb_height: usize,
    /// Thumbnail data
    pub thumb_data: Vec<u8>,
}

/// Represents a Huffman tree node where each node can contain either
/// a number (leaf) or nested arrays of numbers (internal nodes).
#[derive(Debug, Clone)]
pub enum HuffmanNode {
    /// A leaf node
    Leaf(u8),
    /// An internal node
    Node(Rc<RefCell<Vec<HuffmanNode>>>),
}
impl Default for HuffmanNode {
    fn default() -> Self {
        Self::Node(Rc::new(RefCell::new(vec![])))
    }
}

/// Represents a Huffman code node that can either contain child nodes
/// or be a leaf containing a numeric value.
#[derive(Debug, Default, Clone)]
pub struct Code {
    /// Internal node is Code[], leaf node is number
    pub children: Rc<RefCell<Vec<HuffmanNode>>>,
    /// Index
    pub index: Rc<RefCell<usize>>,
}

/// The result of an individual parse */
#[derive(Debug, Clone)]
pub struct ParseResult {
    data: Vec<u8>,
    out_components: Vec<OutComponent>,
    ready: bool,
}

/// An Image organized for the JPEG decoder
#[derive(Debug, Clone)]
pub struct Image {
    /// The image width
    pub width: usize,
    /// The image height
    pub height: usize,
    /// The exif buffer
    pub exif_buffer: Option<Vec<u8>>,
    /// The image data
    pub data: Vec<u8>,
    /// The image comments
    pub comments: Vec<String>,
}

#[derive(Debug)]
struct InputReader<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> InputReader<'a> {
    fn read_u16(&mut self) -> u16 {
        let v = u16::from_be_bytes([self.data[self.offset], self.data[self.offset + 1]]);
        self.offset += 2;
        v
    }

    fn read_block(&mut self) -> Vec<u8> {
        let len = self.read_u16() as usize;
        let start = self.offset;
        let end = start + len - 2;
        self.offset = end;
        self.data[start..end].to_vec()
    }
}

/// Decodes a JPEG image
///
/// ## Parameters
/// - `jpeg_data`: The JPEG data
/// - `user_opts`: The user provided options
/// - `jpeg_tables`: The JPEG tables (if provided)
///
/// ## Returns
/// The decoded image
pub fn decode_jpeg_data(
    jpeg_data: &[u8],
    user_opts: Option<JPEGOptions>,
    jpeg_tables: Option<&[u8]>,
) -> Image {
    let mut reader = JpegStreamReader::new(user_opts);
    // If this constructor ever supports async decoding this will need to be done differently.
    // Until then, treating as singleton limit is fine.
    reader.reset_max_memory_usage(reader.max_memory_usage_in_mb * 1024 * 1024);
    if let Some(jpeg_tables) = jpeg_tables {
        reader.parse(jpeg_tables);
    }
    reader.parse(jpeg_data);

    reader.get_image_data()
}

/// Decodes a JPEG image
///
/// ## Parameters
/// - `buffer`: The JPEG data
/// - `jpeg_tables`: The JPEG tables (if provided)
///
/// ## Returns
/// The decoded image as a buffer
pub fn jpeg_decoder(buffer: &[u8], jpeg_tables: Option<&[u8]>) -> Vec<u8> {
    decode_jpeg_data(
        buffer,
        Some(JPEGOptions { skip_mutation: true, ..Default::default() }),
        jpeg_tables,
    )
    .data
}

/// A JPEG stream reader
#[derive(Debug)]
pub struct JpegStreamReader {
    color_transform: Option<bool>,
    skip_mutation: bool,
    format_as_rgba: bool,
    tolerant_decoding: bool,
    // Don't decode more than 100 megapixels
    max_resolution_in_mp: usize,
    // Don't decode if memory footprint is more than 512MB
    max_memory_usage_in_mb: usize,
    quantization_tables: Vec<Vec<i32>>,
    huffman_tables_ac: Vec<HuffmanNode>,
    huffman_tables_dc: Vec<HuffmanNode>,
    total_bytes_allocated: RefCell<usize>,
    max_memory_usage_bytes: usize,
    width: usize,
    height: usize,
    reset_interval: usize,
    comments: Vec<String>,
    adobe: Option<Adobe>,
    jfif: Option<JFIF>,
    exif_buffer: Option<Vec<u8>>,
    frames: Vec<JPEGFrame>,
    dct_zig_zag: Vec<i32>,
}
impl JpegStreamReader {
    /// Given the user provided options, Create a new JpegStreamReader
    pub fn new(opts: Option<JPEGOptions>) -> Self {
        let opts = opts.unwrap_or_default();
        Self {
            color_transform: opts.color_transform,
            skip_mutation: opts.skip_mutation,
            format_as_rgba: opts.format_as_rgba,
            tolerant_decoding: opts.tolerant_decoding,
            max_resolution_in_mp: opts.max_resolution_in_mp,
            max_memory_usage_in_mb: opts.max_memory_usage_in_mb,
            quantization_tables: vec![],
            huffman_tables_ac: vec![],
            huffman_tables_dc: vec![],
            total_bytes_allocated: 0.into(),
            max_memory_usage_bytes: 0,
            width: 0,
            height: 0,
            reset_interval: 0,
            comments: vec![],
            adobe: None,
            jfif: None,
            exif_buffer: None,
            frames: vec![],
            dct_zig_zag: vec![
                0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5, 12, 19, 26, 33, 40, 48, 41,
                34, 27, 20, 13, 6, 7, 14, 21, 28, 35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23,
                30, 37, 44, 51, 58, 59, 52, 45, 38, 31, 39, 46, 53, 60, 61, 54, 47, 55, 62, 63,
            ],
        }
    }

    /// Reset the frames
    pub fn reset_frames(&mut self) {
        self.frames = vec![];
    }

    /// Reset the max memory usage
    ///
    /// ## Parameters
    /// - `max_memory_usage_bytes`: The new max memory usage
    pub fn reset_max_memory_usage(&mut self, max_memory_usage_bytes: usize) {
        self.total_bytes_allocated = 0.into();
        self.max_memory_usage_bytes = max_memory_usage_bytes;
    }

    /// Get the complete image data
    ///
    /// ## Returns
    /// The image data
    pub fn get_image_data(&mut self) -> Image {
        let channels = if self.format_as_rgba { 4 } else { 3 };
        let ParseResult { data, out_components, ready } = self.get_result();
        let bytes_needed = self.width * self.height * channels;
        self.request_memory_allocation(bytes_needed);

        if ready {
            return Image {
                width: self.width,
                height: self.height,
                exif_buffer: self.exif_buffer.clone(),
                data,
                comments: self.comments.clone(),
            };
        }
        let mut image = Image {
            width: self.width,
            height: self.height,
            exif_buffer: self.exif_buffer.clone(),
            data: vec![0; bytes_needed],
            comments: self.comments.clone(),
        };

        let image_data_array = &mut image.data;
        let mut i = 0;
        let mut j = 0;
        match out_components.len() {
            1 => {
                for _ in 0..self.height {
                    for _ in 0..self.width {
                        let _y = data[i];
                        i += 1;

                        image_data_array[j] = _y;
                        j += 1;
                        image_data_array[j] = _y;
                        j += 1;
                        image_data_array[j] = _y;
                        j += 1;
                        if self.format_as_rgba {
                            image_data_array[j] = 255;
                            j += 1;
                        }
                    }
                }
            }
            3 => {
                for _ in 0..self.height {
                    for _ in 0..self.width {
                        let _r = data[i];
                        i += 1;
                        let _g = data[i];
                        i += 1;
                        let _b = data[i];
                        i += 1;

                        image_data_array[j] = _r;
                        j += 1;
                        image_data_array[j] = _g;
                        j += 1;
                        image_data_array[j] = _b;
                        j += 1;
                        if self.format_as_rgba {
                            image_data_array[j] = 255;
                            j += 1;
                        }
                    }
                }
            }
            4 => {
                for _ in 0..self.height {
                    for _ in 0..self.width {
                        let _c = data[i] as f64;
                        i += 1;
                        let _m = data[i] as f64;
                        i += 1;
                        let _y = data[i] as f64;
                        i += 1;
                        let _k = data[i] as f64;
                        i += 1;

                        let _r = 255 - clamp_to_8bit(ceil(_c * (1. - _k / 255.) + _k));
                        let _g = 255 - clamp_to_8bit(ceil(_m * (1. - _k / 255.) + _k));
                        let _b = 255 - clamp_to_8bit(ceil(_y * (1. - _k / 255.) + _k));

                        image_data_array[j] = _r as u8;
                        j += 1;
                        image_data_array[j] = _g as u8;
                        j += 1;
                        image_data_array[j] = _b as u8;
                        j += 1;
                        if self.format_as_rgba {
                            image_data_array[j] = 255;
                            j += 1;
                        }
                    }
                }
            }
            _ => {
                panic!("Unsupported color mode");
            }
        }

        image
    }

    /// Parse the data into the frames
    ///
    /// ## Parameters
    /// - `data`: The individual block of JPEG data to parse
    pub fn parse(&mut self, data: &[u8]) {
        let max_resolution_in_pixels = self.max_resolution_in_mp * 1000 * 1000;
        let mut reader = InputReader { data, offset: 0 };

        let mut file_marker = reader.read_u16();
        let mut malformed_data_offset: isize = -1;
        if file_marker != 0xffd8 {
            // SOI (Start of Image)
            panic!("SOI not found");
        }

        file_marker = reader.read_u16();
        while file_marker != 0xffd9 {
            // EOI (End of image)
            match file_marker {
                0xff00 => {}
                0xffe0 | // APP0 (Application Specific)
                0xffe1 | // APP1
                0xffe2 | // APP2
                0xffe3 | // APP3
                0xffe4 | // APP4
                0xffe5 | // APP5
                0xffe6 | // APP6
                0xffe7 | // APP7
                0xffe8 | // APP8
                0xffe9 | // APP9
                0xffea | // APP10
                0xffeb | // APP11
                0xffec | // APP12
                0xffed | // APP13
                0xffee | // APP14
                0xffef | // APP15
                0xfffe => {
                  // COM (Comment)
                  let app_data = reader.read_block();

                  if file_marker == 0xfffe {
                    let comment = String::from_utf8_lossy(&app_data);
                    self.comments.push(comment.into());
                  }

                  if (file_marker == 0xffe0) && (
                      app_data[0] == 0x4a &&
                      app_data[1] == 0x46 &&
                      app_data[2] == 0x49 &&
                      app_data[3] == 0x46 &&
                      app_data[4] == 0
                    ) {
                    // 'JFIF\x00'
                    self.jfif = Some(JFIF {
                      version: JFIFVersion { major: app_data[5], minor: app_data[6] },
                      density_units: app_data[7] as usize,
                      x_density: ((app_data[8] as usize) << 8) | app_data[9] as usize,
                      y_density: ((app_data[10] as usize) << 8) | app_data[11] as usize,
                      thumb_width: app_data[12]as usize,
                      thumb_height: app_data[13]as usize,
                      thumb_data: app_data[14..14 + 3 * app_data[12] as usize * app_data[13] as usize].to_vec(),
                    });
                  }
                  // TODO APP1 - Exif
                  if (file_marker == 0xffe1) && (
                      app_data[0] == 0x45 &&
                      app_data[1] == 0x78 &&
                      app_data[2] == 0x69 &&
                      app_data[3] == 0x66 &&
                      app_data[4] == 0
                    ) {
                    // 'EXIF\x00'
                    self.exif_buffer = Some(app_data[5..].to_vec());
                  }

                  if (file_marker == 0xffee) && (
                      app_data[0] == 0x41 &&
                      app_data[1] == 0x64 &&
                      app_data[2] == 0x6f &&
                      app_data[3] == 0x62 &&
                      app_data[4] == 0x65 &&
                      app_data[5] == 0
                    ) {
                    // 'Adobe\x00'
                    self.adobe = Some(Adobe {
                      version: app_data[6],
                      flags0: ((app_data[7] as usize) << 8) | app_data[8] as usize,
                      flags1: ((app_data[9] as usize) << 8) | app_data[10] as usize,
                      transform_code: app_data[11],
                    });
                  }
                }

                0xffdb => {
                  // DQT (Define Quantization Tables)
                  let quantization_tables_length = reader.read_u16() as usize;
                  let quantization_tables_end = quantization_tables_length + reader.offset - 2;
                  while reader.offset < quantization_tables_end {
                    let quantization_table_spec = data[reader.offset] as usize;
                    reader.offset += 1;
                    self.request_memory_allocation(64 * 4);
                    let mut table_data: Vec<i32> = vec![0; 64];
                    if quantization_table_spec >> 4 == 0 {
                        // 8 bit values
                        for j in 0..64 {
                            let z = self.dct_zig_zag[j] as usize;
                            table_data[z] = data[reader.offset] as i32;
                            reader.offset += 1;
                        }
                    } else if quantization_table_spec >> 4 == 1 {
                      // 16 bit
                      for j in 0..64 {
                        let z = self.dct_zig_zag[j] as usize;
                        table_data[z] = reader.read_u16() as i32;
                      }
                    } else {
                      panic!("DQT: invalid table spec");
                    }
                    let quantize_table_idx = quantization_table_spec & 15;
                    ensure_len(&mut self.quantization_tables, quantize_table_idx, vec![]);
                    self.quantization_tables[quantization_table_spec & 15] = table_data;
                  }
                }

                // SOF0 (Start of Frame, Baseline DCT)
                // SOF1 (Start of Frame, Extended DCT)
                0xffc0..=0xffc2 => {
                  // SOF2 (Start of Frame, Progressive DCT)
                  reader.read_u16(); // skip data length
                  let precision = data[reader.offset];
                  reader.offset += 1;
                  let mut frame = JPEGFrame {
                    extended: file_marker == 0xffc1,
                    progressive: file_marker == 0xffc2,
                    precision,
                    scan_lines: reader.read_u16() as usize,
                    samples_per_line: reader.read_u16() as usize,
                    components: BTreeMap::default(),
                    components_order: vec![],
                    max_h: 0,
                    max_v: 0,
                    mcus_per_line: 0,
                    mcus_per_column: 0,
                  };

                  let pixels_in_frame = frame.scan_lines * frame.samples_per_line;
                  if pixels_in_frame > max_resolution_in_pixels {
                    let exceeded_amount = ceil((pixels_in_frame as f64 - max_resolution_in_pixels as f64) / 1e6);
                    panic!("max_resolution_in_mp limit exceeded by {exceeded_amount}MP");
                  }

                  let components_count = data[reader.offset];
                  reader.offset += 1;
                for _ in 0..components_count {
                    let component_id = data[reader.offset] as i32;
                    let h = data[reader.offset + 1] as usize >> 4;
                    let v = data[reader.offset + 1] as usize & 15;
                    let q_id = data[reader.offset + 2];

                    // if h <= 0 || v <= 0 {
                    //   panic!("Invalid sampling factor, expected values above 0");
                    // }

                    frame.components_order.push(component_id);
                    frame.components.insert(component_id, Rc::new(RefCell::new(JPEGComponent {
                      h,
                      v,
                      quantization_idx: q_id as usize,
                      blocks_per_line: 0,
                      blocks_per_column: 0,
                      blocks: vec![],
                      huffman_table_dc: Rc::new(RefCell::new(vec![])),
                      huffman_table_ac: Rc::new(RefCell::new(vec![])),
                      pred: 0,
                      quantization_table: vec![],
                      dct_zig_zag: self.dct_zig_zag.clone(),
                    })));
                    reader.offset += 3;
                  }
                  self.prepare_components(&mut frame);
                  self.frames.push(frame);
                }

                0xffc4 => {
                  // DHT (Define Huffman Tables)
                  let huffman_length = reader.read_u16() as usize;
                let mut i: usize = 2;
                while i < huffman_length {
                    let huffman_table_spec = data[reader.offset] as usize;
                    reader.offset += 1;
                    let mut code_lengths: Vec<u8> = vec![0; 16];
                    let mut code_length_sum: usize = 0;
                    let mut j = 0;
                    while j < 16 {
                      code_lengths[j] = data[reader.offset];
                      code_length_sum += code_lengths[j] as usize;
                      j += 1;
                      reader.offset += 1;
                    }
                    self.request_memory_allocation(16 + code_length_sum);
                    let mut huffman_values: Vec<u8> = vec![0; code_length_sum];
                    j = 0;
                    while j < code_length_sum {
                      huffman_values[j] = data[reader.offset];
                      j += 1;
                      reader.offset += 1;
                    }
                    i += 17 + code_length_sum;

                    let huffman_table = if huffman_table_spec >> 4 == 0 { &mut self.huffman_tables_dc } else { &mut self.huffman_tables_ac };
                    let index = huffman_table_spec & 15;
                    ensure_len(huffman_table, index, HuffmanNode::Leaf(0));
                    huffman_table[
                      index
                    ] = HuffmanNode::Node(Rc::new(RefCell::new(build_huffman_table(&code_lengths, &huffman_values))));
                  }
                }

                0xffdd => { // DRI (Define Restart Interval)
                  reader.read_u16(); // skip data length
                  self.reset_interval = reader.read_u16() as usize;
                }

                0xffdc => { // Number of Lines marker
                    reader.read_u16(); // skip data length
                    reader.read_u16(); // Ignore this data since it represents the image height
                }

                0xffda => {
                    // SOS (Start of Scan)
                    reader.read_u16(); // skip scan length
                    let selectors_count = data[reader.offset];
                    reader.offset += 1;
                    let mut components: Vec<Rc<RefCell<JPEGComponent>>> = vec![];
                    let frame = &mut self.frames[0];
                    for _ in 0..selectors_count {
                        let component = frame.components.get(&(data[reader.offset] as i32)).unwrap();
                        reader.offset += 1;
                        let table_spec = data[reader.offset] as usize;
                        reader.offset += 1;
                        ensure_len(&mut self.huffman_tables_dc, table_spec >> 4, HuffmanNode::Leaf(0));
                        component.borrow_mut().huffman_table_dc = match &self.huffman_tables_dc[table_spec >> 4] {
                            HuffmanNode::Node(nodes) => nodes.clone(),
                            HuffmanNode::Leaf(_) => Rc::new(RefCell::new(vec![])),
                        };
                        ensure_len(&mut self.huffman_tables_ac, table_spec & 15, HuffmanNode::Leaf(0));
                        component.borrow_mut().huffman_table_ac = match &self.huffman_tables_ac[table_spec & 15] {
                            HuffmanNode::Node(nodes) => nodes.clone(),
                            HuffmanNode::Leaf(_) => Rc::new(RefCell::new(vec![])),
                        };
                        components.push(component.clone());
                    }
                    let spectral_start = data[reader.offset] as usize;
                    reader.offset += 1;
                    let spectral_end = data[reader.offset] as usize;
                    reader.offset += 1;
                    let successive_approximation = data[reader.offset];
                    reader.offset += 1;
                    let processed = decode_scan(
                        data,
                        reader.offset,
                        frame,
                        &components,
                        self.reset_interval,
                        spectral_start,
                        spectral_end,
                        (successive_approximation >> 4) as usize,
                        (successive_approximation & 15) as i32,
                        self.tolerant_decoding,
                    );
                    reader.offset += processed;
                }

                0xffff => { // Fill bytes
                  if data[reader.offset] != 0xff {
                    // Avoid skipping a valid marker.
                    reader.offset -= 1;
                  }
                }

                _ => {
                  if data[reader.offset - 3] == 0xff && data[reader.offset - 2] >= 0xc0 && data[reader.offset - 2] <= 0xfe {
                    // could be incorrect encoding -- last 0xFF byte of the previous
                    // block was eaten by the encoder
                    reader.offset -= 3;
                  } else if file_marker == 0xe0 || file_marker == 0xe1 {
                    // Recover from malformed APP1 markers popular in some phone models.
                    // See https://github.com/eugeneware/jpeg-js/issues/82
                    if malformed_data_offset != -1 {
                      panic!(
                        "first unknown JPEG marker at offset {malformed_data_offset}, second unknown JPEG marker {file_marker} at offset {}", reader.offset - 1
                      );
                    }
                    malformed_data_offset = reader.offset as isize - 1;
                    let next_offset = reader.read_u16() as usize;
                    if data[reader.offset + next_offset - 2] == 0xff {
                      reader.offset += next_offset - 2;
                    }
                  } else {
                    panic!("unknown JPEG marker {file_marker}");
                  }
                }
              }
            file_marker = reader.read_u16();
        }
    }

    /// Increase the max memory usage
    ///
    /// ## Parameters
    /// - `increase_amount`: The amount to increase the max memory usage
    fn request_memory_allocation(&self, increase_amount: usize) {
        let total_memory_impact_bytes = *self.total_bytes_allocated.borrow() + increase_amount;
        if total_memory_impact_bytes > self.max_memory_usage_bytes {
            let exceeded_amount = (total_memory_impact_bytes - self.max_memory_usage_bytes)
                .div_ceil(1024)
                .div_ceil(1024);
            panic!("max_memory_usage_in_mb limit exceeded by at least ${exceeded_amount}MB");
        }

        *self.total_bytes_allocated.borrow_mut() = total_memory_impact_bytes;
    }

    /// Get a result of the frame decoding
    ///
    /// ## Returns
    /// The result of the frame decoding
    fn get_result(&mut self) -> ParseResult {
        if self.frames.is_empty() {
            panic!("no frames were decoded");
        } else if self.frames.len() > 1 {
            panic!("more than one frame is not supported");
        }

        // set each frame's components quantization table
        for i in 0..self.frames.len() {
            let cp = &mut self.frames[i].components;
            for comp in cp.values_mut() {
                let index = comp.borrow().quantization_idx;
                comp.borrow_mut().quantization_table = self.quantization_tables[index].clone();
            }
        }

        let frame = &self.frames[0];
        let JPEGFrame { components, components_order, .. } = frame;
        let mut out_components: Vec<OutComponent> = vec![];
        self.width = frame.samples_per_line;
        let width = self.width;
        self.height = frame.scan_lines;
        let height = self.height;
        let scale_x = self.width / width;
        let scale_y = self.height / height;

        for index in components_order.iter() {
            let component = components.get(index).unwrap().borrow();
            out_components.push(OutComponent {
                lines: self.build_component_data(&component),
                scale_x: component.h / frame.max_h,
                scale_y: component.v / frame.max_v,
            });
        }

        let mut offset = 0;
        let mut ready = false;
        let data_length = width * height * out_components.len();
        self.request_memory_allocation(data_length);
        let mut data = vec![0; data_length];

        // No mutation function for parsing the data without mutation
        let mut no_mutation = || {
            ready = true;
            let mut oi = 0;
            for y in 0..height {
                for x in 0..width {
                    for component in &out_components {
                        data[oi] = component.lines[y * component.scale_y][x * component.scale_x];
                        oi += 1;
                    }
                }
            }
        };

        if self.skip_mutation {
            no_mutation();
            return ParseResult { data, ready, out_components };
        }

        match out_components.len() {
            1 => {
                let component1 = &out_components[0];
                for y in 0..height {
                    let component1_line = &component1.lines[y * component1.scale_y * scale_y];
                    for x in 0..width {
                        let _y = component1_line[x * component1.scale_x * scale_x];

                        data[offset] = _y;
                        offset += 1;
                    }
                }
            }
            2 => {
                // PDF might compress two component data in custom colorspace
                let component1 = &out_components[0];
                let component2 = &out_components[1];
                for y in 0..height {
                    let component1_line = &component1.lines[y * component1.scale_y * scale_y];
                    let component2_line = &component2.lines[y * component2.scale_y * scale_y];
                    for x in 0..width {
                        let mut _y = component1_line[x * component1.scale_x * scale_x];
                        data[offset] = _y;
                        offset += 1;
                        _y = component2_line[x * component2.scale_x * scale_x];
                        data[offset] = _y;
                        offset += 1;
                    }
                }
            }
            3 => {
                // The default transform for three components is true
                let mut color_transform = true;
                if let Some(self_transform) = self.color_transform {
                    color_transform = self_transform
                }

                let component1 = &out_components[0];
                let component2 = &out_components[1];
                let component3 = &out_components[2];
                for y in 0..height {
                    let component1_line = &component1.lines[y * component1.scale_y * scale_y];
                    let component2_line = &component2.lines[y * component2.scale_y * scale_y];
                    let component3_line = &component3.lines[y * component3.scale_y * scale_y];
                    for x in 0..width {
                        let _r;
                        let _g;
                        let _b;
                        if !color_transform {
                            _r = component1_line[x * component1.scale_x * scale_x];
                            _g = component2_line[x * component2.scale_x * scale_x];
                            _b = component3_line[x * component3.scale_x * scale_x];
                        } else {
                            let _y = component1_line[x * component1.scale_x * scale_x] as f64;
                            let _cb = component2_line[x * component2.scale_x * scale_x] as f64;
                            let _cr: f64 = component3_line[x * component3.scale_x * scale_x] as f64;

                            _r = clamp_to_8bit(_y + 1.402 * (_cr - 128.)) as u8;
                            _g = clamp_to_8bit(
                                _y - 0.3441363 * (_cb - 128.) - 0.71413636 * (_cr - 128.),
                            ) as u8;
                            _b = clamp_to_8bit(_y + 1.772 * (_cb - 128.)) as u8;
                        }

                        data[offset] = _r;
                        offset += 1;
                        data[offset] = _g;
                        offset += 1;
                        data[offset] = _b;
                        offset += 1;
                    }
                }
            }
            4 => {
                if self.adobe.is_none() {
                    no_mutation();
                } else {
                    // The default transform for four components is false
                    let mut color_transform = false;
                    let adobe = self.adobe.clone().unwrap_or_default();

                    if adobe.transform_code != 0 {
                        color_transform = true;
                    } else if let Some(self_transform) = self.color_transform {
                        color_transform = self_transform
                    }
                    let component1 = &out_components[0];
                    let component2 = &out_components[1];
                    let component3 = &out_components[2];
                    let component4 = &out_components[3];
                    for y in 0..height {
                        let component1_line = &component1.lines[y * component1.scale_y * scale_y];
                        let component2_line = &component2.lines[y * component2.scale_y * scale_y];
                        let component3_line = &component3.lines[y * component3.scale_y * scale_y];
                        let component4_line = &component4.lines[y * component4.scale_y * scale_y];
                        for x in 0..width {
                            let _c;
                            let _m;
                            let _ye;
                            let _k;
                            if !color_transform {
                                _c = component1_line[x * component1.scale_x * scale_x];
                                _m = component2_line[x * component2.scale_x * scale_x];
                                _ye = component3_line[x * component3.scale_x * scale_x];
                                _k = component4_line[x * component4.scale_x * scale_x];
                            } else {
                                let _y = component1_line[x * component1.scale_x * scale_x] as f64;
                                let _cb = component2_line[x * component2.scale_x * scale_x] as f64;
                                let _cr = component3_line[x * component3.scale_x * scale_x] as f64;
                                _k = component4_line[x * component4.scale_x * scale_x];

                                _c = 255 - clamp_to_8bit(_y + 1.402 * (_cr - 128.)) as u8;
                                _m = 255
                                    - clamp_to_8bit(
                                        _y - 0.3441363 * (_cb - 128.) - 0.71413636 * (_cr - 128.),
                                    ) as u8;
                                _ye = 255 - clamp_to_8bit(_y + 1.772 * (_cb - 128.)) as u8;
                            }
                            data[offset] = 255 - _c;
                            offset += 1;
                            data[offset] = 255 - _m;
                            offset += 1;
                            data[offset] = 255 - _ye;
                            offset += 1;
                            data[offset] = 255 - _k;
                            offset += 1;
                        }
                    }
                }
            }
            _ => panic!("Unsupported color mode"),
        }

        ParseResult { data, out_components, ready }
    }

    /// Build the component data
    ///
    /// ## Parameters
    /// - `component`: the component
    /// - `reader`: the jpeg stream reader
    ///
    /// ## Returns
    /// The component data
    fn build_component_data(&self, component: &JPEGComponent) -> Vec<Vec<u8>> {
        let mut lines = vec![];
        let blocks_per_line = component.blocks_per_line;
        let blocks_per_column = component.blocks_per_column;
        let samples_per_line = blocks_per_line << 3;
        // Only 1 used per invocation of this function and garbage collected after invocation, so no need to account for its memory footprint.
        let mut _r: Vec<i32> = vec![0; 64];
        let mut r: Vec<u8> = vec![0; 64];

        self.request_memory_allocation(samples_per_line * blocks_per_column * 8);

        for block_row in 0..blocks_per_column {
            let scan_line = block_row << 3;
            for _ in 0..8 {
                lines.push(vec![0; samples_per_line]);
            }
            for block_col in 0..blocks_per_line {
                quantize_and_inverse(
                    component,
                    &component.blocks[block_row][block_col],
                    &mut r,
                    &mut _r,
                );

                let mut offset = 0;
                let sample = block_col << 3;
                for j in 0..8 {
                    let line = &mut lines[scan_line + j];
                    for i in 0..8 {
                        line[sample + i] = r[offset];
                        offset += 1;
                    }
                }
            }
        }

        lines
    }

    /// Prepares the components of the frame
    ///
    /// ## Parameters
    /// - `frame`: The frame to parse
    fn prepare_components(&self, frame: &mut JPEGFrame) {
        // According to the JPEG standard, the sampling factor must be between 1 and 4
        // See https://github.com/libjpeg-turbo/libjpeg-turbo/blob/9abeff46d87bd201a952e276f3e4339556a403a3/libjpeg.txt#L1138-L1146
        let mut max_h = 1;
        let mut max_v = 1;
        for component in frame.components.values() {
            let component = component.borrow();
            if max_h < component.h {
                max_h = component.h;
            }
            if max_v < component.v {
                max_v = component.v;
            }
        }
        let mcus_per_line = frame.samples_per_line.div_ceil(8).div_ceil(max_h);
        let mcus_per_column = frame.scan_lines.div_ceil(8).div_ceil(max_v);
        for component in frame.components.values_mut() {
            let component = &mut component.borrow_mut();
            let blocks_per_line =
                ((frame.samples_per_line.div_ceil(8)) * component.h).div_ceil(max_h);
            let blocks_per_column = ((frame.scan_lines.div_ceil(8)) * component.v).div_ceil(max_v);
            let blocks_per_line_for_mcu = mcus_per_line * component.h;
            let blocks_per_column_for_mcu = mcus_per_column * component.v;
            let blocks_to_allocate = blocks_per_column_for_mcu * blocks_per_line_for_mcu;
            let mut blocks = vec![];

            // Each block is a Int32Array of length 64 (4 x 64 = 256 bytes)
            self.request_memory_allocation(blocks_to_allocate * 256);

            for _ in 0..blocks_per_column_for_mcu {
                let mut row = vec![];
                for _ in 0..blocks_per_line_for_mcu {
                    row.push(vec![0; 64]);
                }
                blocks.push(row);
            }
            component.blocks_per_line = blocks_per_line;
            component.blocks_per_column = blocks_per_column;
            component.blocks = blocks;
        }
        frame.max_h = max_h;
        frame.max_v = max_v;
        frame.mcus_per_line = mcus_per_line;
        frame.mcus_per_column = mcus_per_column;
    }
}

/// Builds a Huffman table from the input data
///
/// ## Parameters
/// - `code_lengths`: array of code lengths
/// - `values`: array of values
///
/// ## Returns
/// The Huffman table
fn build_huffman_table(code_lengths: &[u8], values: &[u8]) -> Vec<HuffmanNode> {
    let mut k = 0;
    let mut code: Vec<Code> = vec![];
    let mut length = 16;
    // Find the highest non-zero code length
    while length > 0 && code_lengths[length - 1] == 0 {
        length -= 1;
    }

    let root = Code::default();
    code.push(root.clone());
    let mut p = root.clone();
    let mut q: Code;

    for (i, code_length) in code_lengths.iter().enumerate().take(length) {
        for _ in 0..(*code_length as usize) {
            p = code.pop().unwrap();
            let index = *p.index.borrow();
            ensure_len(&mut p.children.borrow_mut(), index, HuffmanNode::default());
            p.children.borrow_mut()[index] = HuffmanNode::Leaf(values[k]);
            while *p.index.borrow() > 0 {
                if code.is_empty() {
                    panic!("Could not recreate Huffman Table");
                }
                p = code.pop().unwrap();
            }
            *p.index.borrow_mut() += 1;
            code.push(p.clone());
            while code.len() <= i {
                q = Code::default();
                code.push(q.clone());
                let index = *p.index.borrow();
                ensure_len(
                    &mut p.children.borrow_mut(),
                    index,
                    HuffmanNode::Node(Rc::new(RefCell::new(vec![]))),
                );
                p.children.borrow_mut()[index] = HuffmanNode::Node(q.children.clone());
                p = q;
            }
            k += 1;
        }
        if i + 1 < length {
            // p here points to last code
            q = Code::default();
            code.push(q.clone());
            let index = *p.index.borrow();
            ensure_len(
                &mut p.children.borrow_mut(),
                index,
                HuffmanNode::Node(Rc::new(RefCell::new(vec![]))),
            );
            p.children.borrow_mut()[index] = HuffmanNode::Node(q.children.clone());
            p = q;
        }
    }

    root.children.borrow_mut().clone()
}

/// ## Returns
/// The next bit
fn read_bit(decode_scan: &mut DecodeScan) -> usize {
    if decode_scan.bits_count > 0 {
        decode_scan.bits_count -= 1;
        return (decode_scan.bits_data >> decode_scan.bits_count) & 1;
    }
    decode_scan.bits_data = decode_scan.data[decode_scan.offset] as usize;
    decode_scan.offset += 1;
    if decode_scan.bits_data == 0xff {
        let _next_byte = decode_scan.data[decode_scan.offset];
        decode_scan.offset += 1;
        // if next_byte.is_none() {
        //     panic!("unexpected marker: {}", (decode_scan.bits_data << 8));
        // }
        // unstuff 0
    }
    decode_scan.bits_count = 7;
    decode_scan.bits_data >> 7
}

/// Decodes a Huffman Node tree
///
/// ## Parameters
/// - `decode_scan`: the decoder
/// - `tree`: the tree to decode with
///
/// ## Returns
/// The decoded value
fn decode_huffman(decode_scan: &mut DecodeScan, tree: Rc<RefCell<Vec<HuffmanNode>>>) -> u8 {
    let mut node = tree;
    loop {
        let bit = read_bit(decode_scan);
        if let Some(n) = node.clone().borrow().get(bit) {
            node = match n {
                HuffmanNode::Node(children) => children.clone(),
                HuffmanNode::Leaf(value) => {
                    return *value;
                }
            };
        } else {
            break;
        }
    }
    0
}

/// Receives a number
///
/// ## Parameters
/// - `decode_scan`: the decoder
/// - `length`: the number of bits
///
/// ## Returns
/// The number
fn receive(decode_scan: &mut DecodeScan, mut length: usize) -> usize {
    let mut n: usize = 0;
    while length > 0 {
        let bit = read_bit(decode_scan);
        n = (n << 1) | bit;
        length -= 1;
    }

    n
}

/// Recieves and extends a number
///
/// ## Parameters
/// - `decode_scan`: the decoder
/// - `length`: the number of bits
///
/// ## Returns
/// The extended number
fn receive_and_extend(decode_scan: &mut DecodeScan, length: usize) -> isize {
    let n = receive(decode_scan, length) as isize;
    if n >= 1 << (length - 1) { n } else { n + (-1 << length) + 1 }
}

/// Decodes a baseline block
///
/// ## Parameters
/// - `decode_scan`: the decoder
/// - `component`: the component
/// - `block_row`: the block row
/// - `block_col`: the block column
fn decode_baseline(
    decode_scan: &mut DecodeScan,
    component: &mut JPEGComponent,
    block_row: usize,
    block_col: usize,
) {
    let t = decode_huffman(decode_scan, component.huffman_table_dc.clone());
    let diff = if t == 0 { 0 } else { receive_and_extend(decode_scan, t as usize) };
    component.pred += diff as i32;
    component.blocks[block_row][block_col][0] = component.pred;
    let mut k = 1;
    while k < 64 {
        let rs = decode_huffman(decode_scan, component.huffman_table_ac.clone());
        let s = rs & 15;
        let r = rs >> 4;
        if s == 0 {
            if r < 15 {
                break;
            }
            k += 16;
            continue;
        }
        k += r;
        let z = component.dct_zig_zag[k as usize];
        component.blocks[block_row][block_col][z as usize] =
            receive_and_extend(decode_scan, s as usize) as i32;
        k += 1;
    }
}

/// Decodes a DC coefficient first pass
///
/// ## Parameters
/// - `decode_scan`: the decoder
/// - `component`: the component
/// - `block_row`: the block row
/// - `block_col`: the block column
fn decode_dc_first(
    decode_scan: &mut DecodeScan,
    component: &mut JPEGComponent,
    block_row: usize,
    block_col: usize,
) {
    let t = decode_huffman(decode_scan, component.huffman_table_dc.clone());
    let diff = if t == 0 {
        0
    } else {
        receive_and_extend(decode_scan, t as usize) << decode_scan.successive
    };
    component.pred += diff as i32;
    component.blocks[block_row][block_col][0] = component.pred;
}

/// Decodes a successive approximation block
///
/// ## Parameters
/// - `decode_scan`: the decoder
/// - `component`: the component
/// - `block_row`: the block row
/// - `block_col`: the block column
fn decode_dc_successive(
    decode_scan: &mut DecodeScan,
    component: &mut JPEGComponent,
    block_row: usize,
    block_col: usize,
) {
    component.blocks[block_row][block_col][0] |=
        (read_bit(decode_scan) as i32) << decode_scan.successive;
}

/// Decodes an AC block first pass
///
/// ## Parameters
/// - `decode_scan`: the decoder
/// - `component`: the component
/// - `block_row`: the block row
/// - `block_col`: the block column
fn decode_ac_first(
    decode_scan: &mut DecodeScan,
    component: &mut JPEGComponent,
    block_row: usize,
    block_col: usize,
) {
    if decode_scan.eobrun > 0 {
        decode_scan.eobrun -= 1;
        return;
    }
    let mut k = decode_scan.spectral_start;
    let e = decode_scan.spectral_end;
    while k <= e {
        let rs = decode_huffman(decode_scan, component.huffman_table_ac.clone());
        let s = rs & 15;
        let r = rs >> 4;
        if s == 0 {
            if r < 15 {
                decode_scan.eobrun = receive(decode_scan, r as usize) + (1 << r) - 1;
                break;
            }
            k += 16;
            continue;
        }
        k += r as usize;
        let z = component.dct_zig_zag[k];
        component.blocks[block_row][block_col][z as usize] =
            receive_and_extend(decode_scan, s as usize) as i32 * (1 << decode_scan.successive);
        k += 1;
    }
}

/// Decodes a successive approximation block
///
/// ## Parameters
/// - `decode_scan`: the decoder
/// - `component`: the component
/// - `block_row`: the block row
/// - `block_col`: the block column
fn decode_ac_successive(
    decode_scan: &mut DecodeScan,
    component: &mut JPEGComponent,
    block_row: usize,
    block_col: usize,
) {
    let mut k = decode_scan.spectral_start;
    let e = decode_scan.spectral_end;
    let mut r = 0;
    while k <= e {
        let z = component.dct_zig_zag[k] as usize;
        let direction = if component.blocks[block_row][block_col][z] < 0 { -1 } else { 1 };
        match decode_scan.successive_ac_state {
            0 => {
                // initial state
                let rs = decode_huffman(decode_scan, component.huffman_table_ac.clone());
                let s = rs & 15;
                r = rs >> 4;
                if s == 0 {
                    if r < 15 {
                        decode_scan.eobrun = receive(decode_scan, r as usize) + (1 << r);
                        decode_scan.successive_ac_state = 4;
                    } else {
                        r = 16;
                        decode_scan.successive_ac_state = 1;
                    }
                } else {
                    if s != 1 {
                        panic!("invalid ACn encoding");
                    }
                    decode_scan.successive_ac_next_value =
                        receive_and_extend(decode_scan, s as usize) as usize;
                    decode_scan.successive_ac_state = if r != 0 { 2 } else { 3 };
                }
                continue;
            }
            // skipping r zero items
            1 | 2 => {
                if component.blocks[block_row][block_col][z] != 0 {
                    component.blocks[block_row][block_col][z] +=
                        (read_bit(decode_scan) << decode_scan.successive) as i32 * direction;
                } else {
                    r -= 1;
                    if r == 0 {
                        decode_scan.successive_ac_state =
                            if decode_scan.successive_ac_state == 2 { 3 } else { 0 };
                    }
                }
            }
            3 => {
                // set value for a zero item
                if component.blocks[block_row][block_col][z] != 0 {
                    component.blocks[block_row][block_col][z] +=
                        (read_bit(decode_scan) << decode_scan.successive) as i32 * direction;
                } else {
                    component.blocks[block_row][block_col][z] =
                        (decode_scan.successive_ac_next_value as i32) << decode_scan.successive;
                    decode_scan.successive_ac_state = 0;
                }
            }
            4 => {
                // eob
                if component.blocks[block_row][block_col][z] != 0 {
                    component.blocks[block_row][block_col][z] +=
                        (read_bit(decode_scan) << decode_scan.successive) as i32 * direction;
                }
            }
            _ => {}
        }
        k += 1;
    }
    if decode_scan.successive_ac_state == 4 {
        decode_scan.eobrun -= 1;
        if decode_scan.eobrun == 0 {
            decode_scan.successive_ac_state = 0;
        }
    }
}
/// Decodes an MCU
///
/// ## Parameters
/// - `component`: The component
/// - `decode`: The decoder
/// - `mcu`: The mcu
/// - `row`: The row
/// - `col`: The column
fn decode_mcu(
    decode_scan: &mut DecodeScan,
    component: &mut JPEGComponent,
    decode: fn(&mut DecodeScan, &mut JPEGComponent, block_row: usize, block_col: usize),
    tolerant_decoding: bool,
    mcu: usize,
    row: usize,
    col: usize,
) {
    let mcu_row = mcu / decode_scan.mcus_per_line;
    let mcu_col = mcu % decode_scan.mcus_per_line;
    let block_row = mcu_row * component.v + row;
    let block_col = mcu_col * component.h + col;
    // If the block is missing and we're in tolerant mode, just skip it.
    if component.blocks.get(block_row).is_none() && tolerant_decoding {
        return;
    }
    decode(decode_scan, component, block_row, block_col);
}

/// Decodes a block
///
/// ## Parameters
/// - `component`: The component
/// - `decode`: The decoder
/// - `mcu`: The mcu value
fn decode_block(
    decode_scan: &mut DecodeScan,
    component: &mut JPEGComponent,
    decode: fn(&mut DecodeScan, &mut JPEGComponent, block_row: usize, block_col: usize),
    tolerant_decoding: bool,
    mcu: usize,
) {
    let block_row = mcu / component.blocks_per_line;
    let block_col = mcu % component.blocks_per_line;
    // If the block is missing and we're in tolerant mode, just skip it.
    if component.blocks.get(block_row).is_none() && tolerant_decoding {
        return;
    }
    decode(decode_scan, component, block_row, block_col);
}

struct DecodeScan<'a> {
    bits_count: usize,
    bits_data: usize,
    offset: usize,
    mcus_per_line: usize,
    progressive: bool,
    data: &'a [u8],
    successive: i32,
    successive_ac_state: usize,
    successive_ac_next_value: usize,
    eobrun: usize,
    spectral_start: usize,
    spectral_end: usize,
}

/// Decodes a JPEG scan
///
/// ## Parameters
/// - `data`: the JPEG data
/// - `offset`: the offset in the JPEG data
/// - `frame`: the frame
/// - `components`: the components of the frame
/// - `reset_interval`: the reset interval
/// - `spectral_start`: the spectral start
/// - `spectral_end`: the spectral end
/// - `successive_prev`: the successive prev
/// - `successive`: the successive number
/// - `opts`: the options passed to the reader
///
/// ## Returns
/// The decoded scan size
#[allow(clippy::too_many_arguments)]
fn decode_scan(
    data: &[u8],
    offset: usize,
    frame: &mut JPEGFrame,
    components: &[Rc<RefCell<JPEGComponent>>],
    mut reset_interval: usize,
    spectral_start: usize,
    spectral_end: usize,
    successive_prev: usize,
    successive: i32,
    tolerant_decoding: bool,
) -> usize {
    let start_offset = offset;
    let mut decode_scan = DecodeScan {
        bits_count: 0,
        bits_data: 0,
        offset,
        mcus_per_line: frame.mcus_per_line,
        progressive: frame.progressive,
        data,
        successive,
        successive_ac_state: 0,
        successive_ac_next_value: 0,
        eobrun: 0,
        spectral_start,
        spectral_end,
    };

    let components_length = components.len();
    let decode_fn = if decode_scan.progressive {
        if spectral_start == 0 {
            if successive_prev == 0 { decode_dc_first } else { decode_dc_successive }
        } else if successive_prev == 0 {
            decode_ac_first
        } else {
            decode_ac_successive
        }
    } else {
        decode_baseline
    };

    let mut mcu = 0;
    let mut marker: usize;
    let mcu_expected = if components_length == 1 {
        components[0].borrow().blocks_per_line * components[0].borrow().blocks_per_column
    } else {
        decode_scan.mcus_per_line * frame.mcus_per_column
    };
    if reset_interval == 0 {
        reset_interval = mcu_expected;
    }

    let mut h;
    let mut v;
    while mcu < mcu_expected {
        // reset interval stuff
        for component in components.iter().take(components_length) {
            component.borrow_mut().pred = 0;
        }
        decode_scan.eobrun = 0;

        if components_length == 1 {
            let component = &components[0];
            for _ in 0..reset_interval {
                decode_block(
                    &mut decode_scan,
                    &mut component.borrow_mut(),
                    decode_fn,
                    tolerant_decoding,
                    mcu,
                );
                mcu += 1;
            }
        } else {
            for _ in 0..reset_interval {
                for component in components.iter().take(components_length) {
                    h = component.borrow().h;
                    v = component.borrow().v;
                    for j in 0..v {
                        for k in 0..h {
                            decode_mcu(
                                &mut decode_scan,
                                &mut component.borrow_mut(),
                                decode_fn,
                                tolerant_decoding,
                                mcu,
                                j,
                                k,
                            );
                        }
                    }
                }
                mcu += 1;

                // If we've reached our expected MCU's, stop decoding
                if mcu == mcu_expected {
                    break;
                }
            }
        }

        if mcu == mcu_expected {
            // Skip trailing bytes at the end of the scan - until we reach the next marker
            loop {
                if (data[decode_scan.offset] == 0xff) && (data[decode_scan.offset + 1] != 0x00) {
                    break;
                }
                decode_scan.offset += 1;
                if decode_scan.offset >= data.len() - 2 {
                    break;
                }
            }
        }

        // find marker
        decode_scan.bits_count = 0;
        marker = ((data[decode_scan.offset] as usize) << 8) | data[decode_scan.offset + 1] as usize;
        if marker < 0xff00 {
            panic!("marker was not found");
        }

        if (0xffd0..=0xffd7).contains(&marker) {
            // RSTx
            decode_scan.offset += 2;
        } else {
            break;
        }
    }

    decode_scan.offset - start_offset
}

/// A port of poppler's IDCT method which in turn is taken from:
/// Christoph Loeffler, Adriaan Ligtenberg, George S. Moschytz,
/// "Practical Fast 1-D DCT Algorithms with 11 Multiplications",
/// IEEE Intl. Conf. on Acoustics, Speech & Signal Processing, 1989,988-991.
///
/// ## Parameters
/// - `zz`: the 8x8 block
/// - `data_out`: the 8x8 block
/// - `data_in`: the 8x8 block
fn quantize_and_inverse(
    component: &JPEGComponent,
    zz: &[i32],
    data_out: &mut [u8],
    data_in: &mut [i32],
) {
    let qt = &component.quantization_table;
    let mut v0;
    let mut v1;
    let mut v2;
    let mut v3;
    let mut v4;
    let mut v5;
    let mut v6;
    let mut v7;
    let mut t;
    let p = data_in;

    // dequant
    for i in 0..64 {
        p[i] = zz[i] * qt[i];
    }
    // inverse DCT on rows
    for i in 0..8 {
        let row = 8 * i;

        // check for all-zero AC coefficients
        if p[1 + row] == 0
            && p[2 + row] == 0
            && p[3 + row] == 0
            && p[4 + row] == 0
            && p[5 + row] == 0
            && p[6 + row] == 0
            && p[7 + row] == 0
        {
            t = (DCT_SQRT2 * p[row] + 512) >> 10;
            p[row] = t;
            p[1 + row] = t;
            p[2 + row] = t;
            p[3 + row] = t;
            p[4 + row] = t;
            p[5 + row] = t;
            p[6 + row] = t;
            p[7 + row] = t;
            continue;
        }

        // stage 4
        v0 = (DCT_SQRT2 * p[row] + 128) >> 8;
        v1 = (DCT_SQRT2 * p[4 + row] + 128) >> 8;
        v2 = p[2 + row];
        v3 = p[6 + row];
        v4 = (DCT_SQRT1D2 * (p[1 + row] - p[7 + row]) + 128) >> 8;
        v7 = (DCT_SQRT1D2 * (p[1 + row] + p[7 + row]) + 128) >> 8;
        v5 = p[3 + row] << 4;
        v6 = p[5 + row] << 4;

        // stage 3
        t = (v0 - v1 + 1) >> 1;
        v0 = (v0 + v1 + 1) >> 1;
        v1 = t;
        t = (v2 * DCT_SIN6 + v3 * DCT_COS6 + 128) >> 8;
        v2 = (v2 * DCT_COS6 - v3 * DCT_SIN6 + 128) >> 8;
        v3 = t;
        t = (v4 - v6 + 1) >> 1;
        v4 = (v4 + v6 + 1) >> 1;
        v6 = t;
        t = (v7 + v5 + 1) >> 1;
        v5 = (v7 - v5 + 1) >> 1;
        v7 = t;

        // stage 2
        t = (v0 - v3 + 1) >> 1;
        v0 = (v0 + v3 + 1) >> 1;
        v3 = t;
        t = (v1 - v2 + 1) >> 1;
        v1 = (v1 + v2 + 1) >> 1;
        v2 = t;
        t = (v4 * DCT_SIN3 + v7 * DCT_COS3 + 2048) >> 12;
        v4 = (v4 * DCT_COS3 - v7 * DCT_SIN3 + 2048) >> 12;
        v7 = t;
        t = (v5 * DCT_SIN1 + v6 * DCT_COS1 + 2048) >> 12;
        v5 = (v5 * DCT_COS1 - v6 * DCT_SIN1 + 2048) >> 12;
        v6 = t;

        // stage 1
        p[row] = v0 + v7;
        p[7 + row] = v0 - v7;
        p[1 + row] = v1 + v6;
        p[6 + row] = v1 - v6;
        p[2 + row] = v2 + v5;
        p[5 + row] = v2 - v5;
        p[3 + row] = v3 + v4;
        p[4 + row] = v3 - v4;
    }

    // inverse DCT on columns
    for i in 0..8 {
        let col = i;

        // check for all-zero AC coefficients
        if p[8 + col] == 0
            && p[2 * 8 + col] == 0
            && p[3 * 8 + col] == 0
            && p[4 * 8 + col] == 0
            && p[5 * 8 + col] == 0
            && p[6 * 8 + col] == 0
            && p[7 * 8 + col] == 0
        {
            t = (DCT_SQRT2 * p[i] + 8192) >> 14;
            p[col] = t;
            p[8 + col] = t;
            p[2 * 8 + col] = t;
            p[3 * 8 + col] = t;
            p[4 * 8 + col] = t;
            p[5 * 8 + col] = t;
            p[6 * 8 + col] = t;
            p[7 * 8 + col] = t;
            continue;
        }

        // stage 4
        v0 = (DCT_SQRT2 * p[col] + 2048) >> 12;
        v1 = (DCT_SQRT2 * p[4 * 8 + col] + 2048) >> 12;
        v2 = p[2 * 8 + col];
        v3 = p[6 * 8 + col];
        v4 = (DCT_SQRT1D2 * (p[8 + col] - p[7 * 8 + col]) + 2048) >> 12;
        v7 = (DCT_SQRT1D2 * (p[8 + col] + p[7 * 8 + col]) + 2048) >> 12;
        v5 = p[3 * 8 + col];
        v6 = p[5 * 8 + col];

        // stage 3
        t = (v0 - v1 + 1) >> 1;
        v0 = (v0 + v1 + 1) >> 1;
        v1 = t;
        t = (v2 * DCT_SIN6 + v3 * DCT_COS6 + 2048) >> 12;
        v2 = (v2 * DCT_COS6 - v3 * DCT_SIN6 + 2048) >> 12;
        v3 = t;
        t = (v4 - v6 + 1) >> 1;
        v4 = (v4 + v6 + 1) >> 1;
        v6 = t;
        t = (v7 + v5 + 1) >> 1;
        v5 = (v7 - v5 + 1) >> 1;
        v7 = t;

        // stage 2
        t = (v0 - v3 + 1) >> 1;
        v0 = (v0 + v3 + 1) >> 1;
        v3 = t;
        t = (v1 - v2 + 1) >> 1;
        v1 = (v1 + v2 + 1) >> 1;
        v2 = t;
        t = (v4 * DCT_SIN3 + v7 * DCT_COS3 + 2048) >> 12;
        v4 = (v4 * DCT_COS3 - v7 * DCT_SIN3 + 2048) >> 12;
        v7 = t;
        t = (v5 * DCT_SIN1 + v6 * DCT_COS1 + 2048) >> 12;
        v5 = (v5 * DCT_COS1 - v6 * DCT_SIN1 + 2048) >> 12;
        v6 = t;

        // stage 1
        p[col] = v0 + v7;
        p[7 * 8 + col] = v0 - v7;
        p[8 + col] = v1 + v6;
        p[6 * 8 + col] = v1 - v6;
        p[2 * 8 + col] = v2 + v5;
        p[5 * 8 + col] = v2 - v5;
        p[3 * 8 + col] = v3 + v4;
        p[4 * 8 + col] = v3 - v4;
    }

    // convert to 8-bit integers
    for i in 0..64 {
        let sample = 128 + ((p[i] + 8) >> 4);
        data_out[i] = if sample < 0 {
            0
        } else if sample > 0xff {
            0xff
        } else {
            sample as u8
        };
    }
}

fn ensure_len<T: Clone>(vec: &mut Vec<T>, index: usize, default: T) {
    if vec.len() <= index {
        vec.resize(index + 1, default);
    }
}

/// Clamp a number to a uint8 [0-255]
///
/// ## Parameters
/// - `a`: the number
///
/// ## Returns
/// The clamped number
fn clamp_to_8bit(a: f64) -> i32 {
    a.clamp(0., 255.) as i32
}
