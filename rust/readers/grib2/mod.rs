/// Section content
pub mod sections;

use crate::{
    parsers::{BufferReader, Reader},
    util::fetch_url,
};
use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use s2json::{BBox3D, MValue, Properties, VectorFeature, VectorGeometry, VectorMultiPoint};
pub use sections::*;

/// An GRIB2 Shaped Vector Feature
pub type GRIB2VectorFeature = VectorFeature<Vec<Grib2ProductDefinition>>;

/// GFS sources available for download
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Grib2GFSSource {
    /// AWS
    Aws,
    /// FTPPRD
    Ftpprd,
    /// NOMADS
    Nomads,
    /// Google
    Google,
    /// Azure
    Azure,
    /// User defined server
    Other(String),
}
impl From<&str> for Grib2GFSSource {
    fn from(value: &str) -> Self {
        match value {
            "aws" => Grib2GFSSource::Aws,
            "ftpprd" => Grib2GFSSource::Ftpprd,
            "nomads" => Grib2GFSSource::Nomads,
            "google" => Grib2GFSSource::Google,
            "azure" => Grib2GFSSource::Azure,
            _ => Grib2GFSSource::Other(value.into()),
        }
    }
}
impl Grib2GFSSource {
    /// Convert the source to a URL
    pub fn to_url(&self) -> String {
        match self {
            Grib2GFSSource::Aws => "https://noaa-gfs-bdp-pds.s3.amazonaws.com/".into(),
            Grib2GFSSource::Ftpprd => "https://ftpprd.ncep.noaa.gov/data/nccf/com/gfs/prod/".into(),
            Grib2GFSSource::Nomads => {
                "https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/".into()
            }
            Grib2GFSSource::Google => {
                "https://storage.googleapis.com/global-forecast-system/".into()
            }
            Grib2GFSSource::Azure => "https://noaagfs.blob.core.windows.net/gfs/".into(),
            Grib2GFSSource::Other(s) => s.into(),
        }
    }
}

/// GFS domains available for download
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grib2GFSDomain {
    /// Atmospheric
    Atmos,
    /// Ocean
    Wave,
}

/// GFS ATMOS products available for download
/// - `pgrb2.0p25` - common fields, 0.25 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2.0p25.f000.shtml)
/// - `pgrb2.0p50` - common fields, 0.50 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2.0p50.f000.shtml)
/// - `pgrb2.1p00` - common fields, 1.00 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2.1p00.f000.shtml)
/// - `pgrb2b.0p25` - uncommon fields, 0.25 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2b.0p25.f000.shtml)
/// - `pgrb2b.0p50` - uncommon fields, 0.50 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2b.0p50.f000.shtml)
/// - `pgrb2b.1p00` - uncommon fields, 1.00 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2b.1p00.f000.shtml)
/// - `pgrb2full.0p50` - combined grids of 0.50 resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t12z.pgrb2full.0p50.f000.shtml)
/// - `sfluxgrb` - surface flux fields, T1534 Semi-Lagrangian grid [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.sfluxgrbf000.grib2.shtml)
/// - `goesimpgrb2.0p25` - 0.50 degree resolution for GOES-IMP [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.goessimpgrb2.0p25.f000.shtml)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Grib2AtmosGFSProduct {
    /// `pgrb2.0p25` - common fields, 0.25 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2.0p25.f000.shtml)
    Pgrb20p25,
    /// - `pgrb2.0p50` - common fields, 0.50 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2.0p50.f000.shtml)
    Pgrb20p50,
    /// - `pgrb2.1p00` - common fields, 1.00 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2.1p00.f000.shtml)
    Pgrb21p00,
    /// - `pgrb2b.0p25` - uncommon fields, 0.25 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2b.0p25.f000.shtml)
    Pgrb2b0p25,
    /// - `pgrb2b.0p50` - uncommon fields, 0.50 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2b.0p50.f000.shtml)
    Pgrb2b0p50,
    /// - `pgrb2b.1p00` - uncommon fields, 1.00 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2b.1p00.f000.shtml)
    Pgrb2b1p00,
    /// - `pgrb2full.0p50` - combined grids of 0.50 resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t12z.pgrb2full.0p50.f000.shtml)
    Pgrb2full0p50,
    /// - `sfluxgrb` - surface flux fields, T1534 Semi-Lagrangian grid [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.sfluxgrbf000.grib2.shtml)
    Sfluxgrb,
    /// - `goesimpgrb2.0p25` - 0.50 degree resolution for GOES-IMP [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.goessimpgrb2.0p25.f000.shtml)
    Goesimpgrb20p25,
    /// - User defined product
    Other(String),
}
impl From<&str> for Grib2AtmosGFSProduct {
    fn from(value: &str) -> Self {
        match value {
            "pgrb2.0p25" => Self::Pgrb20p25,
            "pgrb2.0p50" => Self::Pgrb20p50,
            "pgrb2.1p00" => Self::Pgrb21p00,
            "pgrb2b.0p25" => Self::Pgrb2b0p25,
            "pgrb2b.0p50" => Self::Pgrb2b0p50,
            "pgrb2b.1p00" => Self::Pgrb2b1p00,
            "pgrb2full.0p50" => Self::Pgrb2full0p50,
            "sfluxgrb" => Self::Sfluxgrb,
            "goesimpgrb2.0p25" => Self::Goesimpgrb20p25,
            _ => Self::Other(value.into()),
        }
    }
}
impl From<Grib2AtmosGFSProduct> for String {
    fn from(value: Grib2AtmosGFSProduct) -> Self {
        match value {
            Grib2AtmosGFSProduct::Pgrb20p25 => "pgrb2.0p25".into(),
            Grib2AtmosGFSProduct::Pgrb20p50 => "pgrb2.0p50".into(),
            Grib2AtmosGFSProduct::Pgrb21p00 => "pgrb2.1p00".into(),
            Grib2AtmosGFSProduct::Pgrb2b0p25 => "pgrb2b.0p25".into(),
            Grib2AtmosGFSProduct::Pgrb2b0p50 => "pgrb2b.0p50".into(),
            Grib2AtmosGFSProduct::Pgrb2b1p00 => "pgrb2b.1p00".into(),
            Grib2AtmosGFSProduct::Pgrb2full0p50 => "pgrb2full.0p50".into(),
            Grib2AtmosGFSProduct::Sfluxgrb => "sfluxgrb".into(),
            Grib2AtmosGFSProduct::Goesimpgrb20p25 => "goesimpgrb2.0p25".into(),
            Grib2AtmosGFSProduct::Other(v) => v,
        }
    }
}

/// GFS WAVE products available for download
/// - `arctic.9km` - Arctic, 9km resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.arctic.9km.f003.grib2.shtml)
/// - `atlocn.0p16` - Atlantic, 0.16 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.atlocn.0p16.f003.grib2.shtml)
/// - `epacif.0p16` - Eastern Pacific, 0.16 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.epacif.0p16.f003.grib2.shtml)
/// - `global.0p16` - Global, 0.16 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.global.0p16.f003.grib2.shtml)
/// - `global.0p25` - Global, 0.25 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.global.0p25.f003.grib2.shtml)
/// - `gsouth.0p25` - Gulf of South America, 0.25 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.gsouth.0p25.f003.grib2.shtml)
/// - `wcoast.0p16` - West Coast, 0.16 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.wcoast.0p16.f003.grib2.shtml)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Grib2WaveGFSProduct {
    /// - `arctic.9km` - Arctic, 9km resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.arctic.9km.f003.grib2.shtml)
    Arctic9km,
    /// - `atlocn.0p16` - Atlantic, 0.16 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.atlocn.0p16.f003.grib2.shtml)
    Atlocn0p16,
    /// - `epacif.0p16` - Eastern Pacific, 0.16 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.epacif.0p16.f003.grib2.shtml)
    Epacif0p16,
    /// - `global.0p16` - Global, 0.16 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.global.0p16.f003.grib2.shtml)
    Global0p16,
    /// - `global.0p25` - Global, 0.25 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.global.0p25.f003.grib2.shtml)
    Global0p25,
    /// - `gsouth.0p25` - Gulf of South America, 0.25 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.gsouth.0p25.f003.grib2.shtml)
    Gsouth0p25,
    /// - `wcoast.0p16` - West Coast, 0.16 degree resolution [Study Variables here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.wcoast.0p16.f003.grib2.shtml)
    Wcoast0p16,
    /// User defined product
    Other(String),
}
impl From<&str> for Grib2WaveGFSProduct {
    fn from(value: &str) -> Self {
        match value {
            "arctic.9km" => Grib2WaveGFSProduct::Arctic9km,
            "atlocn.0p16" => Grib2WaveGFSProduct::Atlocn0p16,
            "epacif.0p16" => Grib2WaveGFSProduct::Epacif0p16,
            "global.0p16" => Grib2WaveGFSProduct::Global0p16,
            "global.0p25" => Grib2WaveGFSProduct::Global0p25,
            "gsouth.0p25" => Grib2WaveGFSProduct::Gsouth0p25,
            "wcoast.0p16" => Grib2WaveGFSProduct::Wcoast0p16,
            _ => Grib2WaveGFSProduct::Other(value.into()),
        }
    }
}
impl From<Grib2WaveGFSProduct> for String {
    fn from(value: Grib2WaveGFSProduct) -> Self {
        match value {
            Grib2WaveGFSProduct::Arctic9km => "arctic.9km".into(),
            Grib2WaveGFSProduct::Atlocn0p16 => "atlocn.0p16".into(),
            Grib2WaveGFSProduct::Epacif0p16 => "epacif.0p16".into(),
            Grib2WaveGFSProduct::Global0p16 => "global.0p16".into(),
            Grib2WaveGFSProduct::Global0p25 => "global.0p25".into(),
            Grib2WaveGFSProduct::Gsouth0p25 => "gsouth.0p25".into(),
            Grib2WaveGFSProduct::Wcoast0p16 => "wcoast.0p16".into(),
            Grib2WaveGFSProduct::Other(value) => value,
        }
    }
}

/// GFS Hour
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Grib2GFSHour {
    /// "00"
    Hour0,
    /// "06"
    Hour6,
    /// "12"
    Hour12,
    /// "18"
    Hour18,
}
impl From<&str> for Grib2GFSHour {
    fn from(value: &str) -> Self {
        match value {
            "00" => Grib2GFSHour::Hour0,
            "06" => Grib2GFSHour::Hour6,
            "12" => Grib2GFSHour::Hour12,
            "18" => Grib2GFSHour::Hour18,
            _ => panic!("Invalid hour"),
        }
    }
}
impl From<Grib2GFSHour> for String {
    fn from(value: Grib2GFSHour) -> Self {
        match value {
            Grib2GFSHour::Hour0 => "00".into(),
            Grib2GFSHour::Hour6 => "06".into(),
            Grib2GFSHour::Hour12 => "12".into(),
            Grib2GFSHour::Hour18 => "18".into(),
        }
    }
}

/// Description of a section in the GRIB2 file
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grib2SectionLocations {
    /// Start/offset of section
    pub start: u64,
    /// If missing, assume the end is the end of the file
    pub end: Option<u64>,
    /// The entire line detailing the section
    pub line: String,
    /// The name of the filter
    pub name: String,
}

/// # Fetch ATMOS or WAVE GFS data.
///
/// ## ATMOS
/// You can find some data to reference what's available [here](https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/).
///
/// An example of what variable data means can be found [here](https://www.nco.ncep.noaa.gov/pmb/products/gfs/gfs.t00z.pgrb2.0p50.f000.shtml).
///
/// ## WAVE
/// You can find some data to reference what's available [here](https://nomads.ncep.noaa.gov/pub/data/nccf/com/gfs/prod/).
///
/// An example of what variable data means can be found [here](https://www.nco.ncep.noaa.gov/pmb/products/wave/gfswave.t12z.arctic.9km.f003.grib2.shtml).
///
/// @param source - The source of the data, `aws` | `ftpprd` | `nomads` | `google` | `azure` | or a user provided url
/// @param product - which product to fetch. Use `Grib2AtmosGFSProduct` or `Grib2WaveGFSProduct`
/// @param domain - The domain of the data, `atmos` or `wave`
/// @param year - The year to fetch given a 4 digit year
/// @param month - The month to fetch given a 2 digit month 01 is January and 12 is December
/// @param day - The day to fetch given a 2 digit day, e.g. '01' or '31'
/// @param hour - The forecast hour with 2 digits often in increments of 6 up to 18, e.g. '00' or '12'
/// @param forecast - The forecast hour with 3 digits often in increments of 3 up to 384, e.g. '000' or '003'
/// @param filters - The filters to apply by filtering lines in the .idx file
/// @returns - A GRIB2Reader of the specific sections
pub fn fetch_gfs_data<T: Reader, P: Into<String>>(
    source: Grib2GFSSource,
    product: P,
    domain: Grib2GFSDomain,
    year: String,
    month: String,
    day: String,
    hour: Grib2GFSHour,
    forecast: Option<String>,
    filters: Option<Vec<String>>,
) -> GRIB2Reader {
    // If year is not 4 chars, month not 2, day not 2, or forecast is not 3 chars, return error
    let forecast = forecast.unwrap_or("000".into());
    if year.len() != 4 || month.len() != 2 || day.len() != 2 || forecast.len() != 3 {
        panic!("Year, month, day, and forecast must be 4, 2, 2, and 3 characters, respectively.",);
    }
    let link = get_gfs_link(source, product, domain, year, month, day, hour, forecast);
    // pull .idx file FIRST
    let idxs = parsed_idx_from_url(format!("{}.idx", link), filters.unwrap_or_default(), None);
    let source_data = link_to_chunks(link, &idxs);

    GRIB2Reader::new::<T>(source_data.into(), idxs)
}

/// Get the link to download GFS Atmos data relative to IDXs
fn link_to_chunks(link: String, idxs: &[Grib2SectionLocations]) -> Vec<BufferReader> {
    let mut readers: Vec<BufferReader> = vec![];
    for Grib2SectionLocations { start, end, .. } in idxs {
        let end = end.map_or(String::new(), |e| e.to_string());
        let chunk = fetch_url(&link, &[("Range", &format!("bytes={}-{}", start, end))]).unwrap();
        readers.push(BufferReader::new(chunk));
    }

    readers
}

/// Get the link to download GFS Atmos data
///
/// @param source - The source of the data, `aws` | `ftpprd` | `nomads` | `google` | `azure` | or a user provided url
/// @param product - which product to fetch
/// @param domain - The domain of the data, either 'atmos' for atmospheric data or 'wave' for ocean wave data
/// @param year - The year to fetch given a 4 digit year
/// @param month - The month to fetch given a 2 digit month 01 is January and 12 is December
/// @param day - The day to fetch given a 2 digit day, e.g. '01' or '31'
/// @param hour - The forecast hour with 2 digits often in increments of 6 up to 18, e.g. '00' or '12'
/// @param forecast - The forecast hour with 3 digits often in increments of 3 up to 384, e.g. '000' or '003'
/// @returns - A GRIB2Reader of the specific sections
pub fn get_gfs_link<P: Into<String>>(
    source: Grib2GFSSource,
    product: P,
    domain: Grib2GFSDomain,
    year: String,
    month: String,
    day: String,
    hour: Grib2GFSHour,
    forecast: String,
) -> String {
    let mut link = source.to_url();
    let domain_str = if domain == Grib2GFSDomain::Atmos { "atmos" } else { "wave/gridded" };
    let start_name = if domain == Grib2GFSDomain::Atmos { "gfs" } else { "gfswave" };
    let end_name = if domain == Grib2GFSDomain::Atmos { "" } else { ".grib2" };
    let hour: String = hour.into();
    let product: String = product.into();
    link = format!(
        "{link}gfs.{year}{month}{day}/{hour}/{domain_str}/{start_name}.t{hour}z.{product}.f{forecast}{end_name}",
    );

    link
}

/**
 * Parse the .idx file for GRIB2 section details using a URL
 * @param url - The URL of the .idx file
 * @param filters - The filters to apply
 * @param offset_position - The position of the offset in the ":" sequence
 * @returns - An array of Grib2SectionLocations
 */
pub fn parsed_idx_from_url(
    url: String,
    filters: Vec<String>,
    offset_position: Option<usize>,
) -> Vec<Grib2SectionLocations> {
    let data = fetch_url(&url, &[]).unwrap();
    parse_idx(String::from_utf8_lossy(&data).into(), filters, offset_position)
}

/// Parse the .idx file for GRIB2 section details
///
/// @param data - The contents of the .idx file
/// @param filters - The filters to apply
/// @param offset_position - The position of the offset in the ":" sequence
/// @returns - An array of Grib2SectionLocations
pub fn parse_idx(
    data: String,
    filters: Vec<String>,
    offset_position: Option<usize>,
) -> Vec<Grib2SectionLocations> {
    let offset_position = offset_position.unwrap_or(1);
    let mut res = vec![];
    // split lines, parse information, and add to array
    for line in data.split('\n') {
        if line.len() == 0 {
            continue;
        }
        let offset = line
            .split(':')
            .nth(offset_position)
            .and_then(|s| s.trim().parse::<u64>().ok())
            .unwrap_or(0);
        res.push(Grib2SectionLocations {
            start: offset,
            end: None,
            line: line.into(),
            name: line.into(),
        });
    }
    // now add the "end"s
    for i in 0..res.len() - 1 {
        res[i].end = Some(res[i + 1].start);
    }
    // lastly add the filters
    if filters.len() > 0 {
        res = res
            .iter()
            .filter(|s_l| filters.iter().any(|f| s_l.line.contains(f)))
            .cloned()
            .collect();
    }
    // set names to filter names
    for i in 0..res.len() {
        res[i].name = filters[i].clone();
    }

    res
}

/// GRIB2 Reader inputs
#[derive(Debug)]
pub enum GRIB2ReaderInput<T: Reader> {
    /// A single input reader (completely unparsed)
    Reader(T),
    /// A list of input readers, parsed into section chunks
    SectionChunks(Vec<BufferReader>),
}
impl<T: Reader> From<T> for GRIB2ReaderInput<T> {
    fn from(reader: T) -> Self {
        GRIB2ReaderInput::Reader(reader)
    }
}
impl<T: Reader> From<Vec<BufferReader>> for GRIB2ReaderInput<T> {
    fn from(readers: Vec<BufferReader>) -> Self {
        GRIB2ReaderInput::SectionChunks(readers)
    }
}

/// # GRIB2 Reader
///
/// ## Description
///
/// This class reads a GRIB2 file and returns a list of GRIB2 products.
/// Implements the {@link FeatureIterator} trait.
///
/// ## Usage
///
/// ### The recommended way to parse grib files is to filter out what you want:
/// ```ts
/// // pull .idx file FIRST and filter the ones you want
/// const filters = [':DZDT:0.01 mb:', ':TMP:0.4 mb:', ':ABSV:0.4 mb:anl:'];
/// const idxs = await parsed_idx_from_url(`{link}.idx`, filters);
/// // now bulid the reader
/// const gribReader =  await GRIB2Reader.from_idx(link, idxs);
///
/// for await (const feature of gribReader) {
///   console.log(feature);
/// }
/// ```
///
/// ### Parsing the entire grib file:
/// ```ts
/// const gribReader = new GRIB2Reader(link);
/// ```
///
/// ## Links
/// - https://en.wikipedia.org/wiki/GRIB
/// - https://www.nco.ncep.noaa.gov/pmb/docs/grib2/grib2_doc/
#[derive(Debug)]
pub struct GRIB2Reader {
    /// The GRIB2 packets
    pub packets: Vec<Grib2Sections>,
    /// The list of section locations
    pub idxs: Vec<Grib2SectionLocations>,
}
impl GRIB2Reader {
    /// Create a GRIB2Reader
    ///
    /// @param readers - Reader(s) for entire GRIB file. If array, its grib chunks, otherwise it will be the entire file
    /// @param idxs - The list of section locations we will be parsing
    pub fn new<T: Reader>(readers: GRIB2ReaderInput<T>, idxs: Vec<Grib2SectionLocations>) -> Self {
        let mut this = GRIB2Reader { packets: vec![], idxs };
        let grib_chunks = match readers {
            GRIB2ReaderInput::Reader(reader) => split_grib_chunks(&reader),
            GRIB2ReaderInput::SectionChunks(chunks) => chunks,
        };
        for grib_chunk in grib_chunks {
            this.packets.push(split_section_chunks(grib_chunk));
        }

        this
    }

    /// Create a GRIB2Reader from a .idx file
    ///
    /// @param source - Either the http path to the .idx file or the entire GRIB file
    /// @param idxs - The parsed .idx file with the locations of each section
    /// @returns A GRIB2Reader of the specific sections
    pub fn from_idx<T: Reader>(source: &T, idxs: Vec<Grib2SectionLocations>) -> GRIB2Reader {
        let mut readers: Vec<BufferReader> = vec![];
        for idx in &idxs {
            readers.push(BufferReader::new(source.slice(Some(idx.start), idx.end)));
        }
        GRIB2Reader::new::<T>(readers.into(), idxs)
    }

    /// Get the Vector Point feature data
    pub fn get_data(&mut self) -> Option<VectorMultiPoint> {
        // setup geometry
        if let Some(mut geometry) = self
            .packets
            .get_mut(0)
            .and_then(|p| Some(p.grid_definition.as_mut()?.values.build_grid()))
        {
            // add M-Values from each packet
            for i in 0..self.packets.len() {
                let packet = &self.packets[i];
                let name =
                    self.idxs.get(i).and_then(|i| Some(i.name.clone())).unwrap_or(i.to_string());
                if let Some(data) = packet.data.as_ref().and_then(|d| Some(d.data(packet))) {
                    for i in 0..data.len() {
                        if let Some(m_value) = data.get(i) {
                            let geo = &mut geometry[i];
                            if geo.m == None {
                                geo.m = Some(MValue::new());
                            }
                            geo.m.as_mut().unwrap().insert((&name).into(), (*m_value).into());
                        }
                    }
                }
            }
            Some(geometry)
            // build feature
            // let bbox = BBox3D::from_linestring(&geometry);
            // return Some(GRIB2VectorFeature::new_wm(
            //     None,
            //     Properties::default(),
            //     VectorGeometry::new_multipoint(geometry, Some(bbox)),
            //     Some(product_metadata),
            // ));
        } else {
            None
        }
    }

    /// Get the Vector Point feature
    pub fn get_feature(&mut self) -> Option<GRIB2VectorFeature> {
        if let Some(geometry) = self.get_data() {
            // setup metadata
            let product_metadata: Vec<Grib2ProductDefinition> = self
                .packets
                .iter()
                .filter_map(|packet| Some(packet.product_definition.as_ref()?.values.clone()))
                .collect();
            // setup bbox
            let bbox = BBox3D::from_linestring(&geometry);
            Some(GRIB2VectorFeature::new_wm(
                None,
                Properties::default(),
                VectorGeometry::new_multipoint(geometry, Some(bbox)),
                Some(product_metadata),
            ))
        } else {
            None
        }
    }
}

/// Split the bytes of the GRIB file into individual GRIB chunks that represent sections
///
/// @param reader - Reader for entire GRIB file
/// @returns Array of GRIB Chunk Buffers containing individual GRIB definitions in file
fn split_grib_chunks<T: Reader>(reader: &T) -> Vec<BufferReader> {
    if reader.len() == 0 {
        return vec![];
    }
    let length = reader.uint64_be(Some(8));
    let grib_data = BufferReader::new(reader.slice(Some(0), Some(length)));

    let mut chunks: Vec<BufferReader> = vec![grib_data];
    if length == reader.len() {
        return chunks;
    }
    let rest = BufferReader::new(reader.slice(Some(length), None));
    chunks.append(&mut split_grib_chunks(&rest));

    chunks
}
