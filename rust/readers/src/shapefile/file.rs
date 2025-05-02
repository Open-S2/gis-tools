use super::{
    DataBaseFile, Definition, ShapeFileReader, shapefile_from_gzip as shapefile_from_gzip_local,
};
use parsers::{BufferReader, FileReader};
use proj::ProjectionTransformDefinition;
use s2json::MValueCompatible;
use std::{
    collections::BTreeMap,
    fs::{File, exists},
    io::Read,
    path::Path,
    string::{String, ToString},
    vec::Vec,
};

/// # Build a Shapefile from an input path
///
/// ## Description
/// Given a path to where all the shapefile relevant files exist, build a Shapefile
pub fn shapefile_from_path<I: AsRef<Path> + ToString, P: MValueCompatible>(
    input: I,
    defs: Option<Vec<ProjectionTransformDefinition>>,
    epsg_codes: BTreeMap<String, String>,
) -> ShapeFileReader<FileReader, P> {
    let path = input.to_string().replace(".shp", "");
    let shp = path.clone() + ".shp";
    let dbf_str = path.clone() + ".dbf";
    let prj_str = path.clone() + ".prj";
    let cpg_str = path.clone() + ".cpg";

    if exists(&shp).is_err() {
        panic!("Shapefile does not exist");
    }
    let dbf: Option<String> = if exists(&dbf_str).is_ok() { Some(dbf_str) } else { None };
    let prj: Option<String> = if exists(&prj_str).is_ok() { Some(prj_str) } else { None };
    let cpg: Option<String> = if exists(&cpg_str).is_ok() { Some(cpg_str) } else { None };
    let definition = Definition { shp, dbf, prj, cpg };

    shapefile_from_definition(definition, defs, epsg_codes)
}

/// # Build a Shapefile from a Definition
///
/// ## Description
/// Given a collection of files, build a Shapefile
pub fn shapefile_from_definition<P: MValueCompatible>(
    def: Definition,
    _defs: Option<Vec<ProjectionTransformDefinition>>,
    _epsg_codes: BTreeMap<String, String>,
) -> ShapeFileReader<FileReader, P> {
    let Definition { shp, dbf, cpg, .. } = def;
    let mut database_file = None;
    let mut encoding = None;
    let transform = None;
    if let Some(cpg) = cpg {
        // read cpg file as string
        let mut file = File::open(cpg).unwrap();
        let mut input_str: String = String::new();
        let _ = file.read_to_string(&mut input_str);
        if !input_str.is_empty() {
            encoding = Some(input_str);
        }
    }
    // TODO: Handle projection
    // let transform: Transformer | undefined = undefined;
    // let projection: string | undefined = undefined;
    // if (prj != undefined) {
    //     projection = await readFile(prj, { encoding: 'utf8' });
    //     transform = new Transformer(projection);
    //     for (let def of defs) transform.insertDefinition(def);
    //     for (let [key, value] of Object.entries(epsgCodes)) transform.insertEPSGCode(key, value);
    //     transform.setSource(projection);
    // }
    if let Some(dbf) = dbf {
        database_file = Some(DataBaseFile::new(FileReader::from(dbf), encoding));
    }

    ShapeFileReader::new(FileReader::from(shp), database_file, transform)
}

/// # Read a Shapefile from a Gzip folder.
///
/// ## Description
/// Assumes the input is an arraybuffer that is pointing to a collection of zip shapefile data.
pub fn shapefile_from_gzip<M: Clone, P: MValueCompatible>(
    input: &str,
    _defs: Option<Vec<ProjectionTransformDefinition>>,
    _epsg_codes: BTreeMap<String, String>,
) -> ShapeFileReader<BufferReader, P> {
    let data = std::fs::read(input).unwrap();

    shapefile_from_gzip_local(&data, None, BTreeMap::new())
}
