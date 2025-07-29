use super::{DatumParams, DatumType, ParameterValue};
use crate::proj::{
    AZIMUTH_PROJECTION_CENTRE, FALSE_EASTING, FALSE_NORTHING, LATITUDE_OF_FALSE_ORIGIN,
    LATITUDE_OF_NATURAL_ORIGIN, LATITUDE_OF_PROJECTION_CENTRE, LONGITUDE_OF_FALSE_ORIGIN,
    LONGITUDE_OF_NATURAL_ORIGIN, LONGITUDE_OF_PROJECTION_CENTRE, ProjValue,
    SCALE_FACTOR_AT_NATURAL_ORIGIN, name_to_param_id,
};
use alloc::{collections::BTreeMap, string::String};
use s2json::{GetXY, GetZ};

/// A generic 4-dimensional point/vector
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Coords(pub f64, pub f64, pub f64, pub f64);
impl Coords {
    /// Create a new Coords
    pub fn new(x: f64, y: f64, z: f64, t: f64) -> Coords {
        Coords(x, y, z, t)
    }
    /// Create a new Coords from xy
    pub fn new_xy(x: f64, y: f64) -> Coords {
        Coords(x, y, 0.0, 0.0)
    }
}
impl GetXY for Coords {
    fn x(&self) -> f64 {
        self.0
    }
    fn y(&self) -> f64 {
        self.1
    }
}
impl GetZ for Coords {
    fn z(&self) -> Option<f64> {
        Some(self.2)
    }
}

/// A complex number container
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Complex {
    /// Real part
    pub r: f64,
    /// Imaginary part
    pub i: f64,
}

/// Projection datum methods
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum ProjMethod {
    /// Ellipsoidal
    #[default]
    Ellipsoidal = 0,
    /// Spheroidal
    Spheroidal = 1,
}

/// Airy projection modes
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum ProjMode {
    /// North Pole
    #[default]
    NPole = 0,
    /// South Pole
    SPole = 1,
    /// Equatorial
    Equit = 2,
    /// Oblique
    Obliq = 3,
}

/// Generic Projection Container
#[derive(Debug, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct Proj {
    // PARAMETERS
    /// The name of the projection
    pub name: String,
    /// Projection conversion params
    pub params: BTreeMap<i64, ProjValue>,

    // ELLIPSOID PARAMETERS

    // The linear parameters
    /// The name of the ellipsoid
    pub ellps: String,
    /// semimajor axis (radius if eccentricity==0)
    pub a: f64,
    /// semiminor axis
    pub b: f64,
    /// 1 / a
    pub ra: f64,
    /// 1 / b
    pub rb: f64,
    /// If the ellipsoid is a sphere
    pub sphere: bool,
    // The eccentricities
    /// angular eccentricity
    pub alpha: f64,
    /// first eccentricity
    pub e: f64,
    /// first eccentricity squared
    pub es: f64,
    /// second eccentricity
    pub e2: f64,
    /// second eccentricity squared
    pub e2s: f64,
    /// third eccentricity
    pub e3: f64,
    /// third eccentricity squared
    pub e3s: f64,
    /// 1 - e^2
    pub one_es: f64,
    /// 1 / one_es
    pub rone_es: f64,
    // The flattenings
    /// first flattening [the flattening of the ellipsoid]
    pub f: f64,
    /// second flattening
    pub f2: f64,
    /// third flattening
    pub n: f64,
    /// The inverse flattening (1/f) [the reverse flattening of the ellipsoid]
    pub rf: f64,
    /// 1/f2
    pub rf2: f64,
    /// 1/n
    pub rn: f64,
    /// This one's for GRS80 Datum (Dynamic form factor)
    pub J: f64,

    /// es and a before any +proj related adjustment
    pub es_orig: f64,
    /// a before any +proj related adjustment
    pub a_orig: f64,

    // COORDINATE HANDLING
    /// Over-range flag
    pub over: bool,
    /// Geocentric latitude flag
    pub geoc: bool,
    /// proj=latlong ... not really a projection at all
    pub is_ll: bool,
    /// proj=geocent ... not really a projection at all
    pub is_geocent: bool,
    /// Left flag for input/output coordinate types
    pub left: IoUnits,
    /// Right flag for input/output coordinate types
    pub right: IoUnits,

    // CARTOGRAPHIC OFFSETS
    /// central meridian
    pub lam0: f64,
    /// central parallel
    pub phi0: f64,
    /// false easting
    pub x0: f64,
    /// false northing
    pub y0: f64,
    /// height origin
    pub z0: f64,
    /// time origin
    pub t0: f64,

    // SCALING
    /// General scaling factor - e.g. the 0.9996 of UTM
    pub k0: f64,
    /// Plane coordinate scaling TO meter
    pub to_meter: f64,
    /// Plane coordinate scaling FROM meter
    pub fr_meter: f64,
    /// Vertical scaling TO meter
    pub vto_meter: f64,
    /// Vertical scaling FROM meter
    pub vfr_meter: f64,

    // DATUMS AND HEIGHT SYSTEMS
    /// Datum type (None, Param3, Param7, GridShift, WGS84)
    pub datum_type: DatumType,
    /// Parameters for 3PARAM and 7PARAM
    pub datum_params: DatumParams,

    /// prime meridian offset (in radians)
    pub from_greenwich: f64,
}
impl Default for Proj {
    fn default() -> Self {
        Self {
            name: "".into(),
            params: BTreeMap::new(),
            ellps: "".into(),
            a: 0.,
            b: 0.,
            ra: 0.,
            rb: 0.,
            sphere: false,
            alpha: 0.,
            e: 0.,
            es: 0.,
            e2: 0.,
            e2s: 0.,
            e3: 0.,
            e3s: 0.,
            one_es: 0.,
            rone_es: 0.,
            f: 0.,
            f2: 0.,
            n: 0.,
            rf: 0.,
            rf2: 0.,
            rn: 0.,
            J: 0.,
            es_orig: 0.,
            a_orig: 0.,
            over: false,
            geoc: false,
            is_ll: false,
            is_geocent: false,
            left: IoUnits::RADIANS,
            right: IoUnits::CLASSIC,
            lam0: 0.,
            phi0: 0.,
            x0: 0.,
            y0: 0.,
            z0: 0.,
            t0: 0.,
            k0: 1.,
            to_meter: 1.,
            fr_meter: 1.,
            vto_meter: 1.,
            vfr_meter: 1.,
            datum_type: DatumType::NoDatum,
            datum_params: DatumParams::default(),
            from_greenwich: 0.,
        }
    }
}
impl Proj {
    /// Add a parameter to the proj object
    pub fn add_param(&mut self, param: &ParameterValue) {
        // add via id
        if let Some(id) = &param.id {
            self.insert_param(id.code.i64(), param.into());
        } else if !param.ids.is_empty() {
            for id in &param.ids {
                self.insert_param(id.code.i64(), param.into());
            }
        } else {
            // add via name
            self.insert_param(name_to_param_id(&param.name), param.into());
        }
    }
    /// Set an f64 parameter
    pub fn set_f64(&mut self, id: i64, value: f64) {
        self.insert_param(id, value.into());
    }
    // /// Set a variable from user input (usually used by the API / TUI)
    // pub fn set_var(&mut self, name: &str, value: &str) {
    //     let name_id = name_to_param_id(name);
    //     self.insert_param(name_id, value.into());
    // }
    /// Insert a param given code and value
    fn insert_param(&mut self, id: i64, value: ProjValue) {
        self.add_to_params(id, &value);
        self.params.insert(id, value);
    }
    /// Apply directly to easy access params:
    fn add_to_params(&mut self, id: i64, value: &ProjValue) {
        match id {
            LONGITUDE_OF_FALSE_ORIGIN
            | LONGITUDE_OF_NATURAL_ORIGIN
            | LONGITUDE_OF_PROJECTION_CENTRE => self.lam0 = value.f64(),
            LATITUDE_OF_FALSE_ORIGIN
            | LATITUDE_OF_NATURAL_ORIGIN
            | LATITUDE_OF_PROJECTION_CENTRE => self.phi0 = value.f64(),
            SCALE_FACTOR_AT_NATURAL_ORIGIN => self.k0 = value.f64(),
            AZIMUTH_PROJECTION_CENTRE => self.alpha = value.f64(),
            FALSE_EASTING => self.x0 = value.f64(),
            FALSE_NORTHING => self.y0 = value.f64(),
            _ => {}
        }
    }
}

/// Apply transformation to observation - in forward or inverse direction
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum Direction {
    /// Forward
    FWD = 1,
    /// Do Nothing
    #[default]
    IDENT = 0,
    /// Inverse
    INV = -1,
}

/// IO Units Type
#[derive(Debug, Default, Clone, PartialEq)]
pub enum IoUnits {
    /// Doesn't matter (or depends on pipeline neighbours)
    #[default]
    WHATEVER = 0,
    /// Scaled meters (right), projected system
    CLASSIC = 1,
    /// Meters, projected system
    PROJECTED = 2,
    /// Meters, 3D cartesian system
    CARTESIAN = 3,
    /// Radians
    RADIANS = 4,
    /// Degrees
    DEGREES = 5,
}
