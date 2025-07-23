use super::decoder::Decoder;
use crate::{
    parsers::{RGBA, Reader},
    proj::Transformer,
    readers::{
        FieldTagNames, GTiffDataType, GeoKeyDirectoryKeys, GeoTIFFVectorFeature, GeoTiePoint,
        ImageDirectory, PhotometricInterpretations, apply_predictor, build_samples,
        build_transform_from_geo_keys, convert_color_space, geotiff::decoder::get_decoder,
        get_reader_for_sample, needs_normalization, normalize_array, sample_sum,
    },
};
use alloc::{rc::Rc, vec, vec::Vec};
use core::cell::RefCell;
use half::f16;
use libm::{fmax, fmin, pow, round, sqrt};
use s2json::{BBox, Properties, VectorGeometry, VectorMultiPoint, VectorPoint};

/// Metadata for a GeoTIFF image
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GeoTIFFMetadata {
    /// The height of the image
    pub height: usize,
    /// The width of the image
    pub width: usize,
    /// True if the image has an alpha channel
    pub alpha: bool,
}

/// Raster data
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Raster {
    /// The width of the image
    pub width: usize,
    /// The height of the image
    pub height: usize,
    /// Type
    pub r#type: GTiffDataType,
    /// The type data
    pub data: Vec<f64>,
    /// True if the image has an alpha channel
    pub alpha: bool,
    /// The min value found
    pub min: f64,
    /// The max value found
    pub max: f64,
}
impl Raster {
    /// Convert the data to u8s
    pub fn to_u8s(&self) -> Vec<u8> {
        self.data.iter().map(|x| round(fmax(0., fmin(255., *x))) as u8).collect()
    }
    /// Convert the data to u16s
    pub fn to_u16s(&self) -> Vec<u16> {
        self.data.iter().map(|x| round(fmax(0., fmin(65535., *x))) as u16).collect()
    }
    /// Convert the data to u32s
    pub fn to_u32s(&self) -> Vec<u32> {
        self.data.iter().map(|x| round(fmax(0., fmin(4294967295., *x))) as u32).collect()
    }
    /// Convert the data to i8s
    pub fn to_i8s(&self) -> Vec<i8> {
        self.data.iter().map(|x| round(fmax(-128., fmin(127., *x))) as i8).collect()
    }
    /// Convert the data to i16s
    pub fn to_i16s(&self) -> Vec<i16> {
        self.data.iter().map(|x| round(fmax(-32768., fmin(32767., *x))) as i16).collect()
    }
    /// Convert the data to i32s
    pub fn to_i32s(&self) -> Vec<i32> {
        self.data.iter().map(|x| round(fmax(-2147483648., fmin(2147483647., *x))) as i32).collect()
    }
    /// Convert the data to f16s
    pub fn to_f16s(&self) -> Vec<f16> {
        self.data.iter().map(|x| f16::from_f64(*x)).collect()
    }
    /// Convert the data to f32s
    pub fn to_f32s(&self) -> Vec<f32> {
        self.data.iter().map(|x| *x as f32).collect()
    }
}

/// Internal interface for sample reader
pub type SampleReader = fn(buffer: &[u8], offset: usize, little_endian: bool) -> f64;

/// Internal interface for sample format
pub struct SampleFormat {
    /// The source sample offset collection
    pub src_sample_offsets: Vec<u32>,
    /// The sample readers
    pub sample_readers: Vec<SampleReader>,
}

/// # GeoTIFF Image Container
///
/// ## Description
/// A Container for a GeoTIFF image
///
/// ## Usage
///
/// The methods you have access to:
/// - [`GeoTIFFImage::new`]: Create a new GeoTIFFImage
/// - [`GeoTIFFImage::width`]: Get the image width
/// - [`GeoTIFFImage::height`]: Get the image height
/// - [`GeoTIFFImage::tile_width`]: Get the tile width
/// - [`GeoTIFFImage::tile_height`]: Get the tile height
/// - [`GeoTIFFImage::block_width`]: Get the block width
/// - [`GeoTIFFImage::block_height`]: Get the block height
/// - [`GeoTIFFImage::bytes_per_pixel`]: Calculates the number of bytes for each pixel across all samples.
/// - [`GeoTIFFImage::samples_per_pixel`]: The number of samples per pixel
/// - [`GeoTIFFImage::get_sample_format`]: Returns the sample format
/// - [`GeoTIFFImage::get_bits_per_sample`]: Returns the number of bits per sample
/// - [`GeoTIFFImage::raster_array_type`]: Convert the data format and bits per sample to the appropriate array type
/// - [`GeoTIFFImage::get_tie_point`]: Returns an array of tiepoints.
/// - [`GeoTIFFImage::origin`]: Returns the image origin as a XYZ-vector.
/// - [`GeoTIFFImage::origin_ll`]: Returns the image origin as a XYZ-vector in lon-lat space.
/// - [`GeoTIFFImage::resolution`]: Returns the image resolution as a XYZ-vector.
/// - [`GeoTIFFImage::resolution_ll`]: Returns the image resolution as a XYZ-vector in lon-lat space.
/// - [`GeoTIFFImage::pixel_is_area`]: Returns whether or not the pixels of the image depict an area (or point).
/// - [`GeoTIFFImage::get_bbox`]: Returns the image bounding box as an array of 4 values: min-x, min-y, max-x and max-y.
/// - [`GeoTIFFImage::raster_data`]: Returns the raster data of the image.
/// - [`GeoTIFFImage::get_rgba`]: Returns the RGBA raster data of the image.
/// - [`GeoTIFFImage::get_multi_point_vector`]: Build a vector feature from the image
///
/// The methods you probably care about are `get_bbox`, `raster_data`, `get_rgba`,
/// and `get_multi_point_vector`
#[derive(Debug, Clone, Default)]
pub struct GeoTIFFImage<T: Reader> {
    reader: Rc<RefCell<T>>,
    image_directory: ImageDirectory,
    little_endian: bool,
    is_tiled: bool,
    transformer: Rc<RefCell<Transformer>>,
    decode_fn: Option<Decoder>,
    src_sample_offsets: Vec<u32>,
    sample_readers: Vec<SampleReader>,
    planar_configuration: i16,
}
impl<T: Reader> GeoTIFFImage<T> {
    /// Create a new GeoTIFFImage
    pub fn new(
        reader: Rc<RefCell<T>>,
        image_directory: ImageDirectory,
        transformer: Rc<RefCell<Transformer>>,
        little_endian: bool,
    ) -> Self {
        build_transform_from_geo_keys(
            &mut transformer.borrow_mut(),
            &image_directory.geo_key_directory,
        );
        let compression = image_directory.variables.get_short(FieldTagNames::Compression as u16);
        let planar_configuration = image_directory
            .variables
            .get_short(FieldTagNames::PlanarConfiguration as u16)
            .unwrap_or(1);
        let is_tiled = !image_directory.variables.has(FieldTagNames::StripOffsets as u16);
        Self {
            reader,
            image_directory,
            little_endian,
            is_tiled,
            transformer,
            decode_fn: get_decoder(compression.map(|v| v as u16)),
            src_sample_offsets: Vec::new(),
            sample_readers: Vec::new(),
            planar_configuration,
        }
    }

    /// Get the image width
    pub fn width(&self) -> usize {
        self.image_directory.variables.get_short(FieldTagNames::ImageWidth as u16).unwrap_or(0)
            as usize
    }

    /// Get the image height
    pub fn height(&self) -> usize {
        self.image_directory.variables.get_short(FieldTagNames::ImageLength as u16).unwrap_or(0)
            as usize
    }

    /// Get the tile width
    pub fn tile_width(&self) -> usize {
        if self.is_tiled {
            self.image_directory.variables.get_short(FieldTagNames::TileWidth as u16).unwrap_or(0)
                as usize
        } else {
            self.width()
        }
    }

    /// Get the tile height
    pub fn tile_height(&self) -> usize {
        if self.is_tiled {
            self.image_directory.variables.get_short(FieldTagNames::TileLength as u16).unwrap_or(0)
                as usize
        } else {
            let rows_per_strip = self
                .image_directory
                .variables
                .get_short(FieldTagNames::RowsPerStrip as u16)
                .unwrap_or(0) as usize;
            self.height().min(rows_per_strip)
        }
    }

    /// Get the block width
    pub fn block_width(&self) -> usize {
        self.tile_width()
    }

    /// Get the block height
    pub fn block_height(&self, y: usize) -> usize {
        let tile_height = self.tile_height();
        let height = self.height();
        if self.is_tiled || (y + 1) * tile_height <= height {
            tile_height
        } else {
            height - y * tile_height
        }
    }

    /// Calculates the number of bytes for each pixel across all samples. Only full
    /// bytes are supported, an exception is thrown when this is not the case.
    ///
    /// ## Returns
    /// the bytes per pixel
    pub fn bytes_per_pixel(&self) -> usize {
        let bits_per_sample = self
            .image_directory
            .variables
            .get_u16s(FieldTagNames::BitsPerSample as u16)
            .unwrap_or_default();
        let mut bytes = 0;
        for bit in &bits_per_sample {
            bytes += (*bit).div_ceil(8) as usize;
        }
        bytes
    }

    /// ## Returns
    /// The number of samples per pixel
    pub fn samples_per_pixel(&self) -> usize {
        self.image_directory.variables.get_short(FieldTagNames::SamplesPerPixel as u16).unwrap_or(1)
            as usize
    }

    /// Returns the sample format
    ///
    /// ## Parameters
    /// - `sample_index`: the sample index to start at
    ///
    /// ## Returns
    /// The sample format code
    pub fn get_sample_format(&self, sample_index: Option<usize>) -> u16 {
        let sample_index = sample_index.unwrap_or(0);
        *self
            .image_directory
            .variables
            .get_u16s(FieldTagNames::SampleFormat as u16)
            .unwrap_or_default()
            .get(sample_index)
            .unwrap_or(&1)
    }

    /// Returns the number of bits per sample
    ///
    /// ## Parameters
    /// - `sample_index`: the sample index to start at
    ///
    /// ## Returns
    /// The number of bits per sample at the sample index
    pub fn get_bits_per_sample(&self, sample_index: Option<usize>) -> u16 {
        let sample_index = sample_index.unwrap_or(0);
        *self
            .image_directory
            .variables
            .get_u16s(FieldTagNames::BitsPerSample as u16)
            .unwrap_or_default()
            .get(sample_index)
            .unwrap_or(&0)
    }

    /// Convert the data format and bits per sample to the appropriate array type
    ///
    /// ## Parameters
    /// - `raster`: the data
    ///
    /// ## Returns
    /// The array
    pub fn raster_array_type(&self) -> GTiffDataType {
        let format = self.get_sample_format(None);
        let bits_per_sample = self.get_bits_per_sample(None);
        GTiffDataType::to_type(format, bits_per_sample)
    }

    /// Returns an array of tiepoints.
    pub fn get_tie_point(&self) -> GeoTiePoint {
        self.image_directory.tie_point
    }

    /// Returns the image origin as a XYZ-vector. When the image has no affine
    /// transformation, then an exception is thrown.
    ///
    /// ## Returns
    /// The origin as a vector
    pub fn origin(&self) -> VectorPoint<()> {
        // const { tiepoint, ModelTransformation: transform } = self.#imageDirectory;
        // if (Array.isArray(tiepoint) && tiepoint.length === 6) {
        //   return { x: tiepoint[3], y: tiepoint[4], z: tiepoint[5] };
        // } else if (transform !== undefined) {
        //   return { x: transform[3], y: transform[7], z: transform[11] };
        // }
        // throw new Error('The image does not have an affine transformation.');
        // TODO: Is a modeltranasformation needed here?
        let tiepoint = self.get_tie_point();
        VectorPoint::new_xyz(tiepoint.x, tiepoint.y, tiepoint.z, None)
    }

    /// Returns the image origin as a XYZ-vector in lon-lat space. When the image has no affine
    /// transformation, then an exception is thrown.
    ///
    /// ## Returns
    /// The origin as a lon-lat vector
    pub fn origin_ll(&self) -> VectorPoint<()> {
        let mut origin = self.origin();
        self.transformer.borrow_mut().forward_mut(&mut origin);
        origin
    }

    /// Returns the image resolution as a XYZ-vector. When the image has no affine
    /// transformation, then an exception is thrown. in cases when the current image does
    /// not have the required tags on its own.
    ///
    /// ## Returns
    /// The resolution as a vector
    pub fn resolution(&self) -> VectorPoint<()> {
        // try pixel scale first
        let pixel_scale = self.image_directory.pixel_scale;
        if pixel_scale.x != 0. || pixel_scale.y != 0. || pixel_scale.z != 0. {
            return VectorPoint::new_xyz(pixel_scale.x, -pixel_scale.y, pixel_scale.z, None);
        }
        // then try model transformation
        let transform = self
            .image_directory
            .variables
            .getf64s(FieldTagNames::ModelTransformation as u16)
            .unwrap_or_else(|| panic!("The image does not have an affine transformation."));
        if transform[1] == 0. && transform[4] == 0. {
            VectorPoint::new_xyz(transform[0], -transform[5], transform[10], None)
        } else {
            let x = sqrt(transform[0] * transform[0] + transform[4] * transform[4]);
            let y = -sqrt(transform[1] * transform[1] + transform[5] * transform[5]);
            let z = transform[10];
            VectorPoint::new_xyz(x, y, z, None)
        }
    }

    /// Returns the image resolution as a XYZ-vector in lon-lat space. When the image has no affine
    /// transformation, then an exception is thrown. in cases when the current image does not
    /// have the required tags on its own.
    ///
    /// ## Returns
    /// The resolution as a lon-lat vector
    pub fn resolution_ll(&self) -> VectorPoint<()> {
        let mut resolution = self.resolution();
        self.transformer.borrow_mut().forward_mut(&mut resolution);
        resolution
    }

    /// Returns whether or not the pixels of the image depict an area (or point).
    ///
    /// ## Returns
    /// Whether the pixels are a point
    pub fn pixel_is_area(&self) -> bool {
        self.image_directory
            .geo_key_directory
            .get_short(GeoKeyDirectoryKeys::GTRasterTypeGeoKey as u16)
            .unwrap_or(0)
            == 1
    }

    /// Returns the image bounding box as an array of 4 values: min-x, min-y,
    /// max-x and max-y. When the image has no affine transformation, then an
    /// exception is thrown.
    ///
    /// ## Parameters
    /// - `transform`: apply affine transformation or proj4 transformation
    ///
    /// ## Returns
    /// The bounding box
    pub fn get_bbox(&mut self, transform: bool) -> BBox {
        let height = self.height() as f64;
        let width = self.width() as f64;
        let model_transformation =
            self.image_directory.variables.getf64s(FieldTagNames::ModelTransformation as u16);

        if transform && model_transformation.is_some() {
            let model_transformation = model_transformation.unwrap();
            let [a, b, _c, d, e, f, _g, h] = model_transformation[..8].try_into().unwrap();
            let corners = [[0., 0.], [0., height], [width, 0.], [width, height]];
            let projected = corners.map(|[_i, _j]| [d + a * _i + b * _j, h + e * _i + f * _j]);
            let xs = projected.map(|pt| pt[0]);
            let ys = projected.map(|pt| pt[1]);

            BBox::new(
                xs.iter().copied().fold(f64::INFINITY, fmin),
                ys.iter().copied().fold(f64::INFINITY, fmin),
                xs.iter().copied().fold(f64::NEG_INFINITY, fmax),
                ys.iter().copied().fold(f64::NEG_INFINITY, fmax),
            )
        } else {
            let VectorPoint { x: x1, y: y1, .. } = self.origin();
            let VectorPoint { x: r1, y: r2, .. } = self.resolution();
            let x2 = x1 + r1 * width;
            let y2 = y1 + r2 * height;
            let min_x = fmin(x1, x2);
            let min_y = fmin(y1, y2);
            let max_x = fmax(x1, x2);
            let max_y = fmax(y1, y2);

            if transform {
                let mut min_vec: VectorPoint<()> = VectorPoint::new_xy(min_x, min_y, None);
                self.transformer.borrow_mut().forward_mut(&mut min_vec);
                let mut max_vec: VectorPoint<()> = VectorPoint::new_xy(max_x, max_y, None);
                self.transformer.borrow_mut().forward_mut(&mut max_vec);
                BBox::new(min_vec.x, min_vec.y, max_vec.x, max_vec.y)
            } else {
                BBox::new(min_x, min_y, max_x, max_y)
            }
        }
    }

    /// Returns the raster data of the image.
    ///
    /// ## Parameters
    /// - `samples`: Samples to read from the image
    ///
    /// ## Returns
    /// The raster data
    pub fn raster_data(&mut self, samples: Option<Vec<u16>>) -> Raster {
        let samples =
            samples.unwrap_or_else(|| (0..self.samples_per_pixel()).map(|v| v as u16).collect());
        // setup
        let tile_width = self.tile_width();
        let tile_height = self.tile_height();
        let width = self.width();
        let height = self.height();
        let sample_per_pixel = self.samples_per_pixel();
        let bits_per_sample = self
            .image_directory
            .variables
            .get_u16s(FieldTagNames::BitsPerSample as u16)
            .unwrap_or_default();
        let mut bytes_per_pixel = self.bytes_per_pixel();
        let SampleFormat { src_sample_offsets, sample_readers } =
            self.get_sample_offsets_and_readers(&bits_per_sample, &samples);

        let mut res: Vec<f64> = vec![0.0; width * height * sample_per_pixel];
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        let max_x_tile = width.div_ceil(tile_width);
        let max_y_tile = height.div_ceil(tile_height);
        for y_tile in 0..max_y_tile {
            for x_tile in 0..max_x_tile {
                let mut data: Option<Vec<u8>> = None;
                if self.planar_configuration == 1 {
                    data = Some(self.get_tile_or_strip(x_tile, y_tile, 0));
                }
                for sample_index in 0..samples.len() {
                    let si = sample_index;
                    let sample = samples[sample_index] as usize;
                    if self.planar_configuration == 2 {
                        bytes_per_pixel = bits_per_sample[sample].div_ceil(8) as usize;
                        data = Some(self.get_tile_or_strip(x_tile, y_tile, sample));
                    }
                    let data = data.as_ref().expect("data failed to load");
                    let block_height = self.block_height(y_tile);
                    let first_line = y_tile * tile_height;
                    let first_col = x_tile * tile_width;
                    let last_line = first_line + block_height;
                    let last_col = (x_tile + 1) * tile_width;
                    let reader = sample_readers[si];

                    let ymax = block_height
                        .min(
                            (block_height as isize - (last_line as isize - height as isize))
                                as usize,
                        )
                        .min(height - first_line);
                    let xmax = tile_width
                        .min((tile_width as isize - (last_col as isize - width as isize)) as usize)
                        .min(width - first_col);

                    for y in 0..ymax {
                        for x in 0..xmax {
                            let pixel_offset = (y * tile_width + x) * bytes_per_pixel;
                            let value = reader(
                                data,
                                pixel_offset + src_sample_offsets[si] as usize,
                                self.little_endian,
                            );
                            if !value.is_nan() {
                                min = min.min(value);
                                max = max.max(value);
                            }
                            let window_coordinate = (y + first_line) * width * samples.len()
                                + (x + first_col) * samples.len()
                                + si;
                            res[window_coordinate] = value;
                        }
                    }
                }
            }
        }

        Raster {
            r#type: self.raster_array_type(),
            data: res,
            width,
            height,
            alpha: false,
            min,
            max,
        }
    }

    /// Returns the RGBA raster data of the image.
    ///
    /// ## Returns
    /// The RGBA raster data
    pub fn get_rgba(&mut self) -> Raster {
        let bits_per_sample =
            self.image_directory.variables.get_u16s(FieldTagNames::BitsPerSample as u16);
        let extra_samples = self
            .image_directory
            .variables
            .get_u16s(FieldTagNames::ExtraSamples as u16)
            .unwrap_or_else(|| vec![0])[0];
        let pi: PhotometricInterpretations = self
            .image_directory
            .variables
            .get_short(FieldTagNames::PhotometricInterpretation as u16)
            .unwrap_or(0)
            .into();
        let samples = build_samples(pi, bits_per_sample, Some(extra_samples.into()));

        let mut raster_data = self.raster_data(Some(samples));
        let max = pow(2., self.get_bits_per_sample(None) as f64);
        convert_color_space(
            pi,
            &mut raster_data,
            max,
            self.image_directory.variables.get_u16s(FieldTagNames::ColorMap as u16),
        );
        raster_data.alpha = extra_samples != 0;

        raster_data
    }

    /// Build a vector feature from the image
    ///
    /// ## Returns
    /// The vector feature with rgba values incoded into the points
    pub fn get_multi_point_vector(&mut self) -> GeoTIFFVectorFeature {
        let Raster { width, height, alpha, data, .. } = self.get_rgba();
        let bbox = self.get_bbox(false);
        let BBox { left: min_x, bottom: min_y, right: max_x, top: max_y } = bbox;
        let mut coordinates: VectorMultiPoint<RGBA> = vec![];
        let rgba_stride = if alpha { 4 } else { 3 };
        let bound_x = if min_x == max_x { 1. } else { max_x - min_x };
        let bound_y = if min_y == max_y { 1. } else { max_y - min_y };
        let area_x_stride =
            if self.pixel_is_area() { (0.5 / (width as f64)) * bound_x } else { 0. };
        let area_y_stride =
            if self.pixel_is_area() { (0.5 / (height as f64)) * bound_y } else { 0. };
        let pixel_x_stride = if width == 1 { 1 } else { width - 1 };
        let pixel_y_stride = if height == 1 { 1 } else { height - 1 };

        for y in 0..height {
            for x in 0..width {
                // Adjust x_pos and y_pos relative to the bounding box
                let x_pos =
                    min_x + ((x as f64) / (pixel_x_stride as f64)) * bound_x + area_x_stride;
                let y_pos =
                    min_y + ((y as f64) / (pixel_y_stride as f64)) * bound_y + area_y_stride;
                // Extract RGBA values
                let r = data[y * width * rgba_stride + x * rgba_stride];
                let g = data[y * width * rgba_stride + x * rgba_stride + 1];
                let b = data[y * width * rgba_stride + x * rgba_stride + 2];
                let a =
                    if alpha { data[y * width * rgba_stride + x * rgba_stride + 3] } else { 255. };
                // find the lon-lat coordinates of the point
                let mut point: VectorPoint<RGBA> = VectorPoint::new_xy(
                    x_pos,
                    y_pos,
                    Some(RGBA { r: r / 255., g: g / 255., b: b / 255., a: a / 255. }),
                );
                self.transformer.borrow_mut().forward_mut(&mut point);
                // Add point to coordinates array
                coordinates.push(point);
            }
        }

        GeoTIFFVectorFeature::new_wm(
            None,
            Properties::default(),
            VectorGeometry::new_multipoint(coordinates, Some(bbox.into())),
            Some(GeoTIFFMetadata { width, height, alpha }),
        )
    }

    /// Get the data for a tile or strip
    ///
    /// ## Parameters
    /// - `x`: the tile or strip x coordinate
    /// - `y`: the tile or strip y coordinate
    /// - `sample`: the sample
    ///
    /// ## Returns
    /// The data as a buffer
    fn get_tile_or_strip(&mut self, x: usize, y: usize, sample: usize) -> Vec<u8> {
        let num_tiles_per_row = self.width().div_ceil(self.tile_width());
        let num_tiles_per_col = self.height().div_ceil(self.tile_height());
        let index = if self.planar_configuration == 1 {
            y * num_tiles_per_row + x
        } else if self.planar_configuration == 2 {
            sample * num_tiles_per_row * num_tiles_per_col + y * num_tiles_per_row + x
        } else {
            0
        };

        let tile_offsets = self
            .image_directory
            .variables
            .get_u32s(FieldTagNames::TileOffsets as u16)
            .unwrap_or_default();
        let tile_byte_counts = self
            .image_directory
            .variables
            .get_u32s(FieldTagNames::TileByteCounts as u16)
            .unwrap_or_default();
        let strip_offsets = self
            .image_directory
            .variables
            .get_u32s(FieldTagNames::StripOffsets as u16)
            .unwrap_or_default();
        let strip_byte_counts = self
            .image_directory
            .variables
            .get_u32s(FieldTagNames::StripByteCounts as u16)
            .unwrap_or_default();

        let offset = if self.is_tiled { tile_offsets.get(index) } else { strip_offsets.get(index) };
        let byte_count =
            if self.is_tiled { tile_byte_counts.get(index) } else { strip_byte_counts.get(index) };
        if offset.is_none() || byte_count.is_none() {
            panic!("Invalid offset or byte count");
        }
        let offset = *offset.unwrap() as u64;
        let byte_count = *byte_count.unwrap() as u64;
        let slice = self.reader.borrow_mut().slice(Some(offset), Some(offset + byte_count));
        let mut data = if let Some(decode_fn) = &mut self.decode_fn {
            decode_fn(
                &slice,
                self.image_directory
                    .variables
                    .get(FieldTagNames::JPEGTables as u16)
                    .map(|v| v.0)
                    .as_deref(),
            )
        } else {
            slice
        };
        data = self.maybe_apply_predictor(data);
        let sample_format = self.get_sample_format(None) as usize;
        let bits_per_sample = self.get_bits_per_sample(None) as usize;

        if needs_normalization(sample_format, bits_per_sample) {
            normalize_array(
                data,
                sample_format,
                self.planar_configuration as usize,
                self.samples_per_pixel(),
                bits_per_sample,
                self.tile_width(),
                self.block_height(y),
            )
        } else {
            // convert data into f64
            data
        }
    }

    /// Apply the predictor if necessary
    ///
    /// ## Parameters
    /// - `data`: the raw data
    ///
    /// ## Returns
    /// The data with the predictor applied
    fn maybe_apply_predictor(&mut self, data: Vec<u8>) -> Vec<u8> {
        let predictor =
            self.image_directory.variables.get_short(FieldTagNames::Predictor as u16).unwrap_or(1);
        if predictor == 1 {
            data
        } else {
            let tile_width = if self.is_tiled { self.tile_width() } else { self.width() };
            let tile_height = if self.is_tiled {
                self.tile_height()
            } else {
                let val = self
                    .image_directory
                    .variables
                    .get_short(FieldTagNames::RowsPerStrip as u16)
                    .unwrap_or_else(|| self.height() as i16);
                val as usize
            };
            let bits_per_sample = self
                .image_directory
                .variables
                .get_u16s(FieldTagNames::BitsPerSample as u16)
                .unwrap_or_default();
            apply_predictor(
                data,
                predictor,
                tile_width,
                tile_height,
                bits_per_sample,
                self.planar_configuration,
            )
        }
    }

    /// Get the sample format. If first time than build it
    ///
    /// ## Parameters
    /// - `bits_per_sample`: the bits per sample
    /// - `samples`: the samples
    ///
    /// ## Returns
    /// The sample format
    fn get_sample_offsets_and_readers(
        &mut self,
        bits_per_sample: &[u16],
        samples: &[u16],
    ) -> SampleFormat {
        if self.src_sample_offsets.is_empty()
            && self.sample_readers.is_empty()
            && samples.len() == self.sample_readers.len()
        {
            return SampleFormat {
                src_sample_offsets: self.src_sample_offsets.clone(),
                sample_readers: self.sample_readers.clone(),
            };
        }
        let mut src_sample_offsets: Vec<u32> = vec![];
        let mut sample_readers = vec![];
        for sample in samples.iter() {
            if self.planar_configuration == 1 {
                src_sample_offsets
                    .push((sample_sum(bits_per_sample, 0, *sample as usize) / 8) as u32);
            } else {
                src_sample_offsets.push(0);
            }
            let format = self.get_sample_format(Some(*sample as usize));
            sample_readers.push(get_reader_for_sample(*sample, format));
        }
        self.src_sample_offsets = src_sample_offsets.clone();
        self.sample_readers = sample_readers.clone();

        SampleFormat { src_sample_offsets, sample_readers }
    }
}

//  /// Get a value in the image
//  ///
//  /// ## Parameters
//  /// - `x`: the x coordinate
//  /// - `y`: the y coordinate
//  /// - `inv_y`: if true, the y coordinate is inverted
//  /// - `samples`: Samples to read from the image
//  ///
//  /// ## Returns
//  /// The sample
//   pub fn get_value(
//     x: number,
//     y: number,
//     inv_y = false,
//     samples: number[] = [...Array(self.samples_per_pixel()).keys()],
//   ): Promise<number[]> {
//     // setup
//     let { tile_width, tile_height, width } = this;
//     let bits_per_sample = self.#imageDirectory.BitsPerSample ?? [];
//     let bytesPerPixel = self.bytesPerPixel;
//     let { src_sample_offsets, sample_readers } = self.get_sample_offsets_and_readers(
//       bits_per_sample,
//       samples,
//     );
//     let res: number[] = new Array(samples.length);
//     // invert Y if needed
//     if (inv_y) y = self.height - 1 - y;

//     // search the right tile
//     let x_tile = floor(x / tile_width);
//     let y_tile = floor(y / tile_height);
//     let data: ArrayBufferLike | undefined;
//     if (self.planar_configuration === 1) {
//       data = await self.get_tile_or_strip(x_tile, y_tile, 0);
//     }
//     for (let sample_index = 0; sample_index < samples.len(); ++sample_index) {
//       let si = sample_index;
//       let sample = samples[sample_index];
//       if (self.planar_configuration === 2) {
//         bytesPerPixel = Math.ceil(bits_per_sample[sample] / 8);
//         data = await self.get_tile_or_strip(x_tile, y_tile, sample);
//       }
//       if (data === undefined) throw new Error('data failed to load');
//       let data_view = new DataView(data);
//       let first_line = y_tile * tile_height;
//       let first_col = x_tile * tile_width;
//       let reader = sample_readers[si];

//       let pixel_offset = (y * tile_width + x) * bytesPerPixel;
//       let value = reader.call(data_view, pixel_offset + src_sample_offsets[si], self.#little_endian);
//       let window_coordinate =
//         (y + first_line) * width * samples.len() + (x + first_col) * samples.len() + si;
//       res[floor(window_coordinate) % samples.len()] = value;
//     }

//     return res;
//   }
