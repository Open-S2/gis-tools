use super::{
    DataBaseFile, Definition, ShapeFileReader, shapefile_from_gzip as shapefile_from_gzip_local,
};
use crate::{
    parsers::{BufferReader, FileReader},
    proj::Transformer,
};
use s2json::MValueCompatible;
use std::{
    collections::BTreeMap,
    fs::{File, exists},
    io::Read,
    path::Path,
    string::String,
};

/// # Build a Shapefile from an input path
///
/// ## Description
/// Given a path to where all the shapefile relevant files exist, build a [`ShapeFileReader`]
///
/// ## Usage
/// ```rust
/// use gistools::{parsers::{FileReader, FeatureReader}, readers::{ShapeFileReader, file::shapefile_from_path}};
/// use s2json::MValue;
/// use std::{collections::BTreeMap, path::PathBuf};
///
/// let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
/// path.push("tests/readers/shapefile/fixtures/utf.shp");
/// let path_str = path.to_str().unwrap();
///
/// #[derive(Default, Debug, Clone, MValue, PartialEq)]
/// struct Props {
///     field: String,
/// }
///
/// let shp: ShapeFileReader<FileReader, Props> =
///     shapefile_from_path(path_str, BTreeMap::from([("a".into(), "b".into())]));
///
/// let features: Vec<_> = shp.iter().collect();
/// assert_eq!(features.len(), 2);
/// ```
pub fn shapefile_from_path<I: AsRef<Path>, P: MValueCompatible>(
    input: I,
    epsg_codes: BTreeMap<String, String>,
) -> ShapeFileReader<FileReader, P> {
    let path = input.as_ref().to_path_buf();
    let stem = path.with_extension(""); // removes `.shp`
    let path_str: String = stem.to_string_lossy().into();
    let shp = path_str.clone() + ".shp";
    let dbf_str = path_str.clone() + ".dbf";
    let prj_str = path_str.clone() + ".prj";
    let cpg_str = path_str.clone() + ".cpg";

    if exists(&shp).is_err() {
        panic!("Shapefile does not exist");
    }
    let dbf: Option<String> = if exists(&dbf_str).is_ok() { Some(dbf_str) } else { None };
    let prj: Option<String> = if exists(&prj_str).is_ok() { Some(prj_str) } else { None };
    let cpg: Option<String> = if exists(&cpg_str).is_ok() { Some(cpg_str) } else { None };
    let definition = Definition { shp, dbf, prj, cpg };

    shapefile_from_definition(definition, epsg_codes)
}

/// # Build a Shapefile from a Definition
///
/// ## Description
/// Given a collection of files, build a Shapefile
pub fn shapefile_from_definition<P: MValueCompatible>(
    def: Definition,
    epsg_codes: BTreeMap<String, String>,
) -> ShapeFileReader<FileReader, P> {
    let Definition { shp, dbf, cpg, prj } = def;
    let mut database_file = None;
    let mut encoding = None;
    let mut transform = None;
    if let Some(cpg) = cpg {
        // read cpg file as string
        let mut file = File::open(cpg).unwrap();
        let mut input_str: String = String::new();
        let _ = file.read_to_string(&mut input_str);
        if !input_str.is_empty() {
            encoding = Some(input_str);
        }
    }
    // Handle projection
    if let Some(prj) = prj {
        let mut transformer = Transformer::new();
        for (code, value) in epsg_codes.iter() {
            transformer.insert_epsg_code(code.clone(), value.clone());
        }
        let pr_str = std::fs::read_to_string(prj).unwrap();
        transformer.set_source(pr_str);
        transform = Some(transformer);
    }
    // handle database data
    if let Some(dbf) = dbf {
        database_file = Some(DataBaseFile::new(FileReader::from(dbf), encoding));
    }

    ShapeFileReader::new(FileReader::from(shp), database_file, transform)
}

/// # Read a Shapefile from a Gzip folder.
///
/// ## Description
/// Assumes the input is an arraybuffer that is pointing to a collection of zip shapefile data.
pub fn shapefile_from_gzip<P: MValueCompatible>(
    input: &str,
    epsg_codes: BTreeMap<String, String>,
) -> ShapeFileReader<BufferReader, P> {
    let data = std::fs::read(input).unwrap();

    shapefile_from_gzip_local(&data, epsg_codes)
}
