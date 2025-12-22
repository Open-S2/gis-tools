use crate::{
    space::{
        propagation::{SGP4ErrorOutput, SGP4Output, sgp4, sgp4init},
        util::{
            constants::MINUTES_PER_DAY,
            time::{TimeStamp, days2mdhms, jday},
        },
    },
    util::Date,
};
use alloc::{format, string::String, vec, vec::Vec};
use core::f64::consts::PI;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Classification of TLE
/// - U: unclassified
/// - C: classified
/// - S: secret
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Classification {
    /// Unclassified
    #[default]
    U,
    /// Classified
    C,
    /// Secret
    S,
}
impl From<&str> for Classification {
    fn from(s: &str) -> Self {
        match s {
            "U" | "u" => Classification::U,
            "C" | "c" => Classification::C,
            "S" | "s" => Classification::S,
            _ => Classification::U,
        }
    }
}

/// Mode of operation AFSPC or Improved
/// - a: afspc
/// - i: improved
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum OperationMode {
    /// AFSPC
    A,
    /// Improved
    #[default]
    I,
}
/// Method of orbit determination
/// - d: deep space
/// - n: near earth
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Method {
    /// Deep Space
    D,
    /// Near Earth
    #[default]
    N,
}

/// TLE Data Interface
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct TLEData {
    /// Name
    pub name: String,
    /// Number
    pub number: f64,
    /// Classification
    pub class: Classification,
    /// Identifier
    pub id: String,
    /// Date
    pub date: Date,
    /// Epoch Days
    pub epochdays: f64,
    /// fd mm
    pub fdmm: f64,
    /// sd mm
    pub sdmm: f64,
    /// Drag coefficient
    pub drag: f64,
    /// Mean motion
    pub ephemeris: f64,
    /// Eccentricity
    pub esn: f64,
    /// Inclination
    pub inclination: f64,
    /// Right ascension
    pub ascension: f64,
    /// Eccentricity
    pub eccentricity: f64,
    /// Perigee
    pub perigee: f64,
    /// Mean anomaly
    pub anomaly: f64,
    /// Mean motion
    pub motion: f64,
    /// Revolution
    pub revolution: f64,
    /// Bstar
    pub rms: Option<f64>,
}
impl From<&str> for TLEData {
    /// Parse TLE string
    /// https://en.wikipedia.org/wiki/Two-line_element_set
    fn from(value: &str) -> Self {
        let mut lines: Vec<&str> = trim(value).lines().collect();
        let mut tle = TLEData::default();

        // Line 0: optional name
        if lines.len() >= 3 {
            let mut name = trim(lines.remove(0));
            if name.starts_with("0 ") {
                name = &name[2..];
            }
            tle.name = name.into();
        }

        // Line 1
        let line = lines.remove(0);
        let checksum = check(line);
        if checksum != line[68..69].parse::<u32>().unwrap() {
            panic!("Line 1 checksum mismatch: {} != {}: {}", checksum, &line[68..69], line);
        }

        tle.number = parse_float(&alpha5_converter(&line[2..7]));
        tle.class = trim(&line[7..9]).into();
        tle.id = trim(&line[9..18]).into();
        (tle.date, tle.epochdays) = parse_epoch(&line[18..33]);
        tle.fdmm = parse_float(&line[33..44]);
        tle.sdmm = parse_float(&line[44..53]);
        tle.drag = parse_drag(&line[53..62]);
        tle.ephemeris = parse_float(&line[62..64]);
        tle.esn = parse_float(&line[64..68]);

        // Line 2
        let line = lines.remove(0);
        let checksum = check(line);
        if checksum != line[68..69].parse::<u32>().unwrap() {
            panic!("Line 2 checksum mismatch: {} != {}: {}", checksum, &line[68..69], line);
        }

        tle.inclination = parse_float(&line[8..17]);
        tle.ascension = parse_float(&line[17..26]);
        tle.eccentricity = parse_float(&format!("0.{}", &line[26..34]));
        tle.perigee = parse_float(&line[34..43]);
        tle.anomaly = parse_float(&line[43..52]);
        tle.motion = parse_float(&line[52..63]);
        tle.revolution = parse_float(&line[63..68]);

        tle
    }
}

/// Celestrak TLE Data Interface
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct TLEDataCelestrak {
    /// Object name
    #[serde(rename = "OBJECT_NAME")]
    pub object_name: String,
    /// Object ID
    #[serde(rename = "OBJECT_ID")]
    pub object_id: String,
    /// Epoch
    #[serde(rename = "EPOCH")]
    pub epoch: String,
    /// Mean Motion
    #[serde(rename = "MEAN_MOTION")]
    pub mean_motion: f64,
    /// Eccentricity
    #[serde(rename = "ECCENTRICITY")]
    pub eccentricity: f64,
    /// Inclination
    #[serde(rename = "INCLINATION")]
    pub inclination: f64,
    /// Right Ascension
    #[serde(rename = "RA_OF_ASC_NODE")]
    pub ra_of_asc_node: f64,
    /// Argument of Peri-center
    #[serde(rename = "ARG_OF_PERICENTER")]
    pub arg_of_pericenter: f64,
    /// Mean Anomaly
    #[serde(rename = "MEAN_ANOMALY")]
    pub mean_anomaly: f64,
    /// Ephemeris Type
    #[serde(rename = "EPHEMERIS_TYPE")]
    pub ephemeris_type: f64,
    /// Classification Type
    #[serde(rename = "CLASSIFICATION_TYPE")]
    pub classification_type: String,
    /// Norad Cat ID
    #[serde(rename = "NORAD_CAT_ID")]
    pub norad_cat_id: f64,
    /// Element Set Number
    #[serde(rename = "ELEMENT_SET_NO")]
    pub element_set_no: f64,
    /// Rev at Epoch
    #[serde(rename = "REV_AT_EPOCH")]
    pub rev_at_epoch: f64,
    /// Bstar
    #[serde(rename = "BSTAR")]
    pub bstar: f64,
    /// Mean Motion Dot
    #[serde(rename = "MEAN_MOTION_DOT")]
    pub mean_motion_dot: f64,
    /// Mean Motion Ddot
    #[serde(rename = "MEAN_MOTION_DDOT")]
    pub mean_motion_ddot: f64,
    /// RMS
    #[serde(rename = "RMS")]
    pub rms: String,
    /// Data Source
    #[serde(rename = "DATA_SOURCE")]
    pub data_source: String,
}
/// Convert Celestrak TLE data to a standard TLE data object
/// [JSON example](https://celestrak.org/NORAD/elements/supplemental/index.php?FORMAT=json)
impl From<&TLEDataCelestrak> for TLEData {
    fn from(data: &TLEDataCelestrak) -> Self {
        // convert date to UTC to avoid javascripts localization issues
        let date: Date = (&*data.epoch).into();
        let start = Date::new(date.year, 0, 0);
        TLEData {
            name: data.object_name.clone(),
            number: data.norad_cat_id,
            class: (&*data.classification_type).into(),
            id: data.object_id.clone(),
            date,
            epochdays: jday(&date) - jday(&start),
            fdmm: data.mean_motion_dot,
            sdmm: data.mean_motion_ddot,
            drag: data.bstar,
            ephemeris: data.ephemeris_type,
            esn: data.element_set_no,
            inclination: data.inclination,
            ascension: data.ra_of_asc_node,
            eccentricity: data.eccentricity,
            perigee: data.arg_of_pericenter,
            anomaly: data.mean_anomaly,
            motion: data.mean_motion,
            revolution: data.rev_at_epoch,
            rms: data.rms.parse().ok(),
        }
    }
}

/// # Satellite Orbit Class
///
/// ## Description
/// A class representing a satellite orbit.
///
/// ## Examples
///
/// ### Input TLE example
/// ```txt
/// STARLINK-1007
/// 1 44713C 19074A   23048.53451389 -.00009219  00000+0 -61811-3 0   482
/// 2 44713  53.0512 157.2379 0001140  81.3827  74.7980 15.06382459    15
/// ```
///
/// ### Run example
/// ```ts
/// import { Satellite } from 'gis-tools-ts';
///
/// const sat = new Satellite(tleString);
/// // get propagation at time
/// const { position, velocity } = sat.propagate(new Date());
/// ```
///
/// ## Usage
/// - [`Satellite::new`]: Create a new Satellite object from a TLE string
/// - [`Satellite::gpu`]: Convert the satellite state to an array that is readable by the GPU
/// - [`Satellite::propagate`]: Propagate the orbit of the satellite to a given time
/// - [`Satellite::sgp4`]: Propagate the orbit of the satellite to a given time
///
/// ## Links
/// - https://en.wikipedia.org/wiki/Two-line_element_set
/// - https://celestrak.org/NORAD/documentation/tle-fmt.php
/// - https://www.space-track.org/documentation#tle-basic
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Satellite {
    /// If the satellite is initialized
    pub init: bool, // = false;
    // Line 0
    /// Name of the satellite
    pub name: String, // = 'default',
    // Line 1
    /// (satnum) Satellite catalog number or NORAD (North American Aerospace Defense) Catalog Number
    pub number: f64,
    /// Classification (U: unclassified, C: classified, S: secret)
    pub class: Classification,
    /// International Designator
    pub id: String, // = 'null';
    /// (epochyr + epochdays)
    pub date: Date, // = new Date();
    /// Epoch year
    pub epochyr: f64,
    /// Epoch days
    pub epochdays: f64,
    /// full Sat epoch
    pub jdsatepoch: f64,
    /// (ndot) First derivative of mean motion; the ballistic coefficient
    pub fdmm: f64,
    /// (nddot) Second derivative of mean motion (decimal point assumed)
    pub sdmm: f64,
    /// (bstar) B*, the drag term, or radiation pressure coefficient (decimal point assumed)
    pub drag: f64,
    /// Ephemeris type (always zero; only used in undistributed TLE data)
    pub ephemeris: f64,
    /// Element set number. Incremented when a new TLE is generated for this object.
    pub esn: f64,
    // Line 2
    /// Inclination (degrees)
    pub inclination: f64,
    /// Right ascension of the ascending node (degrees)
    pub ascension: f64,
    /// Eccentricity (decimal point assumed)
    pub eccentricity: f64,
    /// Argument of perigee (degrees)
    pub perigee: f64,
    /// Mean anomaly (degrees)    
    pub anomaly: f64,
    /// Mean motion (revolutions per day)
    pub motion: f64,
    /// Revolution number at epoch (revolutions)
    pub revolution: f64,
    // extra
    /// Operation Mode
    pub opsmode: OperationMode,
    /// RMS
    pub rms: Option<f64>,
    // ------------ all near earth variables ------------
    /// Is IMP
    pub isimp: f64,
    /// method
    pub method: Method,
    /// ay coefficient
    pub aycof: f64,
    /// con 41
    pub con41: f64,
    /// cc1
    pub cc1: f64,
    /// cc4
    pub cc4: f64,
    /// cc5
    pub cc5: f64,
    /// d2
    pub d2: f64,
    /// d3
    pub d3: f64,
    /// d4
    pub d4: f64,
    /// delmo
    pub delmo: f64,
    /// eta
    pub eta: f64,
    /// argpdot
    pub argpdot: f64,
    /// omgcof
    pub omgcof: f64,
    /// sinmao
    pub sinmao: f64,
    /// t2cof
    pub t2cof: f64,
    /// t3cof
    pub t3cof: f64,
    /// t4cof
    pub t4cof: f64,
    /// t5cof
    pub t5cof: f64,
    /// x1mth2
    pub x1mth2: f64,
    /// x7thm1
    pub x7thm1: f64,
    /// mdot
    pub mdot: f64,
    /// nodedot
    pub nodedot: f64,
    /// xlcof
    pub xlcof: f64,
    /// xmcof
    pub xmcof: f64,
    /// nodecf
    pub nodecf: f64,
    // ------------ all deep space variables ------------
    /// irez
    pub irez: f64,
    /// d2201
    pub d2201: f64,
    /// d2211
    pub d2211: f64,
    /// d3210
    pub d3210: f64,
    /// d3222
    pub d3222: f64,
    /// d4410
    pub d4410: f64,
    /// d4422
    pub d4422: f64,
    /// d5220
    pub d5220: f64,
    /// d5232
    pub d5232: f64,
    /// d5421
    pub d5421: f64,
    /// d5433
    pub d5433: f64,
    /// dedt
    pub dedt: f64,
    /// del1
    pub del1: f64,
    /// del2
    pub del2: f64,
    /// del3
    pub del3: f64,
    /// didt
    pub didt: f64,
    /// dmdt
    pub dmdt: f64,
    /// dnodt
    pub dnodt: f64,
    /// domdt
    pub domdt: f64,
    /// e3
    pub e3: f64,
    /// ee2
    pub ee2: f64,
    /// peo
    pub peo: f64,
    /// pgho
    pub pgho: f64,
    /// pho
    pub pho: f64,
    /// pinco
    pub pinco: f64,
    /// plo
    pub plo: f64,
    /// se2
    pub se2: f64,
    /// se3
    pub se3: f64,
    /// sgh2
    pub sgh2: f64,
    /// sgh3
    pub sgh3: f64,
    /// sgh4
    pub sgh4: f64,
    /// sh2
    pub sh2: f64,
    /// sh3
    pub sh3: f64,
    /// si2
    pub si2: f64,
    /// si3
    pub si3: f64,
    /// sl2
    pub sl2: f64,
    /// sl3
    pub sl3: f64,
    /// sl4
    pub sl4: f64,
    /// gsto
    pub gsto: f64,
    /// xfact
    pub xfact: f64,
    /// xgh2
    pub xgh2: f64,
    /// xgh3
    pub xgh3: f64,
    /// xgh4
    pub xgh4: f64,
    /// xh2
    pub xh2: f64,
    /// xh3
    pub xh3: f64,
    /// xi2
    pub xi2: f64,
    /// xi3
    pub xi3: f64,
    /// xl2
    pub xl2: f64,
    /// xl3
    pub xl3: f64,
    /// xl4
    pub xl4: f64,
    /// xlamo
    pub xlamo: f64,
    /// zmol
    pub zmol: f64,
    /// zmos
    pub zmos: f64,
    /// atime
    pub atime: f64,
    /// xli
    pub xli: f64,
    /// xni
    pub xni: f64,
}
impl Satellite {
    /// Create a new Satellite using TLE data
    ///
    /// ## Parameters
    /// - `data`: TLE data or TLE string
    /// - `initialize`: initialize the object on creation
    pub fn new(data: &TLEData, initialize: Option<bool>) -> Self {
        let mut this = Self::default();
        let initialize = initialize.unwrap_or(true);
        this.rms = data.rms;
        this.name = data.name.clone();
        this.number = data.number;
        this.class = data.class;
        this.id = data.id.clone();
        this.date = data.date;
        this.fdmm = data.fdmm;
        this.sdmm = data.sdmm;
        this.drag = data.drag;
        this.ephemeris = data.ephemeris;
        this.esn = data.esn;
        this.inclination = data.inclination.to_radians();
        this.ascension = data.ascension.to_radians();
        this.eccentricity = data.eccentricity;
        this.perigee = data.perigee.to_radians();
        this.anomaly = data.anomaly.to_radians();
        // convert revolution from deg/day to rad/minute
        this.motion = data.motion;
        this.revolution = data.revolution;
        this.epochdays = data.epochdays;

        this.epochyr = (this.date.year % 100) as f64;
        // convert revolution from deg/day to rad/minute
        this.motion /= 1440. / (2. * PI); // rad/min (229.1831180523293)
        // find sgp4epoch time of element set
        // remember that sgp4 uses units of days from 0 jan 1950 (sgp4epoch)
        // and minutes from the epoch (time)
        let year = if this.epochyr < 57. { this.epochyr + 2000. } else { this.epochyr + 1900. };
        let mdhms_result = days2mdhms(year as u16, this.epochdays);

        let TimeStamp { mon, day, hr, min, sec } = mdhms_result;
        this.jdsatepoch = jday(&Date::new_full(
            year as u16,
            mon as u8,
            day as u8,
            hr as u8,
            min as u8,
            sec as u8,
        ));

        if initialize {
            sgp4init(&mut this);
        }

        this
    }

    /// Converts satellite state to an array that is readable by the GPU
    ///
    /// ## Returns
    /// Satellite state in an array
    pub fn gpu(&self) -> Vec<f64> {
        vec![
            self.anomaly,
            self.motion,
            self.eccentricity,
            self.inclination,
            if self.method == Method::D { 0. } else { 1. }, // 0 -> 'd', 1 -> 'n'
            if self.opsmode == OperationMode::A { 0. } else { 1. }, // 0 -> 'a'; 1 -> 'i'
            self.drag,
            self.mdot,
            self.perigee,
            self.argpdot,
            self.ascension,
            self.nodedot,
            self.nodecf,
            self.cc1,
            self.cc4,
            self.cc5,
            self.t2cof,
            self.isimp,
            self.omgcof,
            self.eta,
            self.xmcof,
            self.delmo,
            self.d2,
            self.d3,
            self.d4,
            self.sinmao,
            self.t3cof,
            self.t4cof,
            self.t5cof,
            self.irez,
            self.d2201,
            self.d2211,
            self.d3210,
            self.d3222,
            self.d4410,
            self.d4422,
            self.d5220,
            self.d5232,
            self.d5421,
            self.d5433,
            self.dedt,
            self.del1,
            self.del2,
            self.del3,
            self.didt,
            self.dmdt,
            self.dnodt,
            self.domdt,
            self.gsto,
            self.xfact,
            self.xlamo,
            self.atime,
            self.xli,
            self.xni,
            self.aycof,
            self.xlcof,
            self.con41,
            self.x1mth2,
            self.x7thm1,
            self.zmos,
            self.zmol,
            self.se2,
            self.se3,
            self.si2,
            self.si3,
            self.sl2,
            self.sl3,
            self.sl4,
            self.sgh2,
            self.sgh3,
            self.sgh4,
            self.sh2,
            self.sh3,
            self.ee2,
            self.e3,
            self.xi2,
            self.xi3,
            self.xl2,
            self.xl3,
            self.xl4,
            self.xgh2,
            self.xgh3,
            self.xgh4,
            self.xh2,
            self.xh3,
            self.peo,
            self.pinco,
            self.plo,
            self.pgho,
            self.pho,
        ]
    }

    /// propagate the satellite's position and velocity given a Date input
    ///
    /// ## Parameters
    /// - `time`: Date object
    ///
    /// ## Returns
    /// satellite state at that time given
    pub fn propagate(&self, time: &Date) -> Result<SGP4Output, SGP4ErrorOutput> {
        let j = jday(time);
        sgp4(self, (j - self.jdsatepoch) * MINUTES_PER_DAY)
    }

    /// time in minutes since epoch
    ///
    /// ## Parameters
    /// - `time`: time in minutes from satellite epoch
    ///
    /// ## Returns
    /// satellite state at that time given
    pub fn sgp4(&self, time: f64) -> Result<SGP4Output, SGP4ErrorOutput> {
        sgp4(self, time)
    }
}

/// Parse a float number from a string (TLE-style scientific notation)
fn parse_float(value: &str) -> f64 {
    let re = Regex::new(r"([-])?([.\d]+)([+-]\d+)?").unwrap();

    if let Some(caps) = re.captures(value) {
        let sign = if caps.get(1).map_or("", |m| m.as_str()) == "-" { -1.0 } else { 1.0 };
        let base = caps.get(2).map_or("0", |m| m.as_str());
        let power = caps.get(3).map_or_else(|| "e0".into(), |m| format!("e{}", m.as_str()));
        let combined = format!("{}{}", base, power);
        return sign * combined.parse::<f64>().unwrap();
    }

    0.0
}

/// Parse a drag coefficient from TLE-style string
fn parse_drag(value: &str) -> f64 {
    let re = Regex::new(r"([-])?([.\d]+)([+-]\d+)?").unwrap();
    if let Some(caps) = re.captures(value) {
        let sign = if caps.get(1).map_or("", |m| m.as_str()) == "-" { -1.0 } else { 1.0 };
        let base = caps.get(2).map_or("0", |m| m.as_str());
        let base = if base.contains('.') { base.into() } else { format!("0.{}", base) };
        let power = caps.get(3).map_or_else(|| "e0".into(), |m| format!("e{}", m.as_str()));
        return sign * format!("{}{}", base, power).parse::<f64>().unwrap();
    }

    0.0
}

/// Parse TLE epoch string (e.g., "22345.6789") into your Date struct
fn parse_epoch(value: &str) -> (Date, f64) {
    // Trim whitespace
    let re = Regex::new(r"^\s+|\s+$").unwrap();
    let value: String = re.replace_all(value, "").into();

    // Parse epoch year and day-of-year
    let epoch = value[0..2].parse::<u16>().unwrap();
    let days = value[2..].parse::<f64>().unwrap();

    // Compute full year
    let now_year = 2025;
    let current_epoch = (now_year % 100) as u16;
    let century = now_year - current_epoch as i32;
    let year = if epoch > current_epoch + 1 {
        (century - 100 + epoch as i32) as u16
    } else {
        (century + epoch as i32) as u16
    };

    // Convert day-of-year to month/day/hour/minute/second
    let ts = days2mdhms(year, days);

    // Return Date
    (
        Date::new_full(year, ts.mon as u8, ts.day as u8, ts.hr as u8, ts.min as u8, ts.sec as u8),
        days,
    )
}

/// Check TLE checksum
fn check(line: &str) -> u32 {
    let mut sum = 0;

    for c in line.chars().take(68) {
        if c.is_ascii_digit() {
            sum += c.to_digit(10).unwrap();
        } else if c == '-' {
            sum += 1;
        }
    }

    sum % 10
}

/// Converts alpha5 to alpha2
/// NOTE: Alpha5 skips I and O
fn alpha5_converter(s: &str) -> String {
    if let Some(first_char) = s.chars().next() {
        // if first char is numeric, return unchanged
        if first_char.is_ascii_digit() {
            return s.into();
        }

        let alphabet = "ABCDEFGHJKLMNPQRSTUVWXYZ";
        if let Some(idx) = alphabet.find(first_char) {
            // prepend converted index+10, keep rest
            let rest: String = s.chars().skip(1).collect();
            return format!("{}{}", idx + 10, rest);
        }
    }

    // fallback: return unchanged
    s.into()
}

/// Trim leading and trailing whitespace, BOM, and non-breaking spaces
fn trim(s: &str) -> &str {
    fn is_trim_char(c: char) -> bool {
        c.is_whitespace() || c == '\u{FEFF}' || c == '\u{00A0}'
    }
    s.trim_matches(is_trim_char)
}
