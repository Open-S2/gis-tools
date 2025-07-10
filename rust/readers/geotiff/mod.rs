/// GeoTIFF Color tools
mod color;
/// GeoTIFF Constants
mod constants;
/// Decoding tools
mod decoder;
/// Header tools
mod header;
/// GeoTIFF Image tools
mod image;
/// GeoTIFF Image utility tools
mod image_util;
/// Predictors
mod predictor;
/// Projection Params
mod proj_params;

use crate::{
    parsers::{FeatureReader, RGBA, Reader},
    proj::Transformer,
};
use alloc::{collections::BTreeMap, rc::Rc, string::String};
pub use color::*;
pub use constants::*;
use core::cell::RefCell;
pub use header::*;
pub use image::*;
pub use image_util::*;
pub use predictor::*;
pub use proj_params::*;
use s2json::{Properties, VectorFeature};

/// An GeoTIFF Shaped Vector Feature
pub type GeoTIFFVectorFeature = VectorFeature<GeoTIFFMetadata, Properties, RGBA>;

/// Options for the GeoTIFF Reader
#[derive(Debug, Default, Clone)]
pub struct GeoTIFFOptions {
    /// List of EPSG codes to utilize e.g. `{ "4326": "WKT_STRING" }``
    pub epsg_codes: BTreeMap<String, String>,
}

/// # GeoTIFF Reader
///
/// ## Description
/// This class reads a GeoTIFF file and returns a list of GeoTIFF images.
///
/// ## Usage
/// ```rust
/// // TODO
/// ```
///
/// ## Links
/// - https://www.ogc.org/publications/standard/geotiff/
/// - https://download.osgeo.org/geotiff/spec/tiff6.pdf
/// - https://geospatialworld.net/article/geotiff-a-standard-image-file-format-for-gis-applications/
/// - https://docs.ogc.org/is/19-008r4/19-008r4.html
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GeoTIFFReader<T: Reader> {
    reader: Rc<RefCell<T>>,
    /// The GeoTIFF header
    pub header: GeoTIFFHeaderReader,
    transform: Rc<RefCell<Transformer>>,
}
impl<T: Reader> GeoTIFFReader<T> {
    /// Create a new GeoTIFFReader
    pub fn new(reader: T, options: Option<GeoTIFFOptions>) -> GeoTIFFReader<T> {
        let header = GeoTIFFHeaderReader::new(&reader);
        let options = options.unwrap_or_default();
        let mut transform = Transformer::new();
        for (epsg_code, wkt) in options.epsg_codes.iter() {
            transform.insert_epsg_code(epsg_code.clone(), wkt.clone());
        }
        GeoTIFFReader {
            reader: Rc::new(RefCell::new(reader)),
            header,
            transform: Rc::new(RefCell::new(transform)),
        }
    }

    /// Check if the reader is empty
    pub fn is_empty(&self) -> bool {
        self.header.image_directories.is_empty()
    }

    /// Get the length of internal subfiles
    pub fn len(&self) -> usize {
        self.header.image_directories.len()
    }

    /// Get the nth internal subfile of an image. By default, the first is returned.
    ///
    /// @param index - the index of the image to get [Default=0]
    /// @returns - the image at the given index
    pub fn get_image(&self, index: Option<usize>) -> Option<GeoTIFFImage<T>> {
        let index = index.unwrap_or(0);
        if index >= self.len() {
            None
        } else {
            Some(GeoTIFFImage::new(
                self.reader.clone(),
                self.header.image_directories[index].clone(),
                self.transform.clone(),
                self.header.little_endian,
            ))
        }
    }
}

/// The GeoTIFF Iterator tool
#[derive(Debug)]
pub struct GeoTIFFIterator<'a, T: Reader> {
    reader: &'a GeoTIFFReader<T>,
    index: usize,
}
impl<T: Reader> Iterator for GeoTIFFIterator<'_, T> {
    type Item = GeoTIFFVectorFeature;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut point) = self.reader.get_image(Some(self.index)) {
            self.index += 1;
            Some(point.get_multi_point_vector())
        } else {
            None
        }
    }
}
/// A feature reader trait with a callback-based approach
impl<T: Reader> FeatureReader<GeoTIFFMetadata, Properties, RGBA> for GeoTIFFReader<T> {
    type FeatureIterator<'a>
        = GeoTIFFIterator<'a, T>
    where
        T: 'a;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        GeoTIFFIterator { reader: self, index: 0 }
    }

    #[cfg(feature = "std")]
    fn par_iter(&self, _pool_size: usize, _thread_id: usize) -> Self::FeatureIterator<'_> {
        self.iter()
    }
}
