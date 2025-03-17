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

use core::panic;

use super::BufferReader;
use crate::{
    proj::{ProjectionTransformDefinition, Transformer},
    util::iter_zip_folder,
};
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};
pub use dbf::*;
use s2json::MValueCompatible;
pub use shp::*;

/// A description of what relevant files exist and where
#[cfg(feature = "std")]
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

/**
 * # Read a Shapefile from a Gzip folder.
 *
 * ## Description
 * Assumes the input is an arraybuffer that is pointing to a collection of zip shapefile data.
 */
pub fn shapefile_from_gzip<M: Clone, P: MValueCompatible, D: MValueCompatible>(
    input: &[u8],
    _defs: Option<Vec<ProjectionTransformDefinition>>,
    _epsg_codes: BTreeMap<String, String>,
) -> ShapeFileReader<BufferReader, M, P, D> {
    let mut encoding = None;
    let mut transform = None;
    let mut dbf_reader = None;
    let mut shp_data = None;
    for item in iter_zip_folder(input).unwrap() {
        if item.filename.ends_with("cpg") {
            encoding =
                Some(String::from_utf8_lossy(&(item.read)().unwrap_or_default()).to_string());
        } else if item.filename.ends_with("dbf") {
            if let Ok(data) = (item.read)() {
                dbf_reader = Some(DataBaseFile::new(data.into(), encoding.clone()));
            }
        } else if item.filename.ends_with("shp") {
            if let Ok(data) = (item.read)() {
                shp_data = Some(data);
            }
        } else if item.filename.ends_with("prj") {
            if let Ok(_data) = (item.read)() {
                let transformer = Transformer::new();
                // TODO:
                // if (defs != undefined) for (const def of defs) transform.insertDefinition(def);
                // if (epsgCodes != undefined)
                // for (const [key, value] of Object.entries(epsgCodes)) transform.insertEPSGCode(key, value);
                // transform.setSource(new TextDecoder("utf8").decode(data));
                transform = Some(transformer);
            }
        }
    }
    if let Some(shp_data) = shp_data {
        ShapeFileReader::new(shp_data.into(), dbf_reader, transform)
    } else {
        panic!("Shapefile not found");
    }
}
