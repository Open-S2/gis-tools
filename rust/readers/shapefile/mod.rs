/// Database File Reader
pub mod dbf;
/// File based functions
#[cfg(feature = "std")]
pub mod file;
/// MMap based functions
#[cfg(feature = "std")]
pub mod mmap;
/// Shape File Reader
pub mod shp;

use crate::{parsers::BufferReader, proj::Transformer, util::iter_zip_folder};
use alloc::{collections::BTreeMap, string::String};
pub use dbf::*;
use s2json::MValueCompatible;
pub use shp::*;

/// A description of what relevant files exist and where
#[cfg(feature = "std")]
#[derive(Debug)]
pub struct Definition {
    /// The path to the .shp file
    shp: String,
    /// The path to the .dbf file. dbf is optional, but needed if you want attributes */
    dbf: Option<String>,
    /// The path to the .prj file. prj is optional, but needed if your file is in some
    /// projection you don't want it in
    prj: Option<String>,
    /// The path to the .cpg file. cpg is optional, but needed if your dbf is in some
    /// weird (non utf8) encoding.
    cpg: Option<String>,
}

/// # Read a Shapefile from a Gzip folder.
///
/// ## Description
/// Assumes the input is an arraybuffer that is pointing to a collection of zip shapefile data.
pub fn shapefile_from_gzip<P: MValueCompatible>(
    input: &[u8],
    epsg_codes: BTreeMap<String, String>,
) -> ShapeFileReader<BufferReader, P> {
    let mut encoding = None;
    let mut transform = None;
    let mut dbf_reader = None;
    let mut shp_data = None;
    for item in iter_zip_folder(input).unwrap() {
        if item.filename.ends_with("cpg") {
            encoding = Some(String::from_utf8_lossy(&(item.read)().unwrap_or_default()).into());
        } else if item.filename.ends_with("dbf") {
            if let Ok(data) = (item.read)() {
                dbf_reader = Some(DataBaseFile::new(data.into(), encoding.clone()));
            }
        } else if item.filename.ends_with("shp") {
            if let Ok(data) = (item.read)() {
                shp_data = Some(data);
            }
        } else if item.filename.ends_with("prj")
            && let Ok(data) = (item.read)()
        {
            let mut transformer = Transformer::new();
            for (code, value) in epsg_codes.iter() {
                transformer.insert_epsg_code(code.clone(), value.clone());
            }
            transformer.set_source(String::from_utf8_lossy(&data).into());
            transform = Some(transformer);
        }
    }
    if let Some(shp_data) = shp_data {
        ShapeFileReader::new(shp_data.into(), dbf_reader, transform)
    } else {
        panic!("Shapefile not found");
    }
}
