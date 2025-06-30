use super::{DatumParams, DatumType, ParameterValue};
use crate::proj::{ProjValue, name_to_param_id};
use alloc::{collections::BTreeMap, string::String};

/// A generic 4-dimensional point/vector
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Coords(pub f64, pub f64, pub f64, pub f64);

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
    /// first flattening
    pub f: f64,
    /// second flattening
    pub f2: f64,
    /// third flattening
    pub n: f64,
    /// The inverse flattening (1/f)
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
    // /// Left flag for input/output coordinate types
    // pub left: IoUnits,
    // /// Right flag for input/output coordinate types
    // pub right: IoUnits,

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
    // /// Plane coordinate scaling FROM meter
    // pub fr_meter: f64,
    // /// Vertical scaling TO meter
    // pub vto_meter: f64,
    // /// Vertical scaling FROM meter
    // pub vfr_meter: f64,

    // DATUMS AND HEIGHT SYSTEMS
    /// Datum type (None, Param3, Param7, GridShift, WGS84)
    pub datum_type: DatumType,
    /// Parameters for 3PARAM and 7PARAM
    pub datum_params: DatumParams,

    /// prime meridian offset (in radians)
    pub from_greenwich: f64,
    /// Axis order, pj_transform / pj_adjust_axis
    pub axis: [char; 4],
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
            // left: IoUnits::RADIANS,
            // right: IoUnits::CLASSIC,
            lam0: 0.,
            phi0: 0.,
            x0: 0.,
            y0: 0.,
            z0: 0.,
            t0: 0.,
            k0: 1.,
            to_meter: 1.,
            // fr_meter: 0.,
            // vto_meter: 0.,
            // vfr_meter: 0.,
            datum_type: DatumType::NoDatum,
            datum_params: DatumParams::default(),
            from_greenwich: 0.,
            axis: ['x', 'y', 'z', 't'],
        }
    }
}
impl Proj {
    /// Add a parameter to the proj object
    pub fn add_param(&mut self, param: &ParameterValue) {
        if let Some(id) = &param.id {
            self.params.insert(id.code.i64(), param.into());
        }
        for id in &param.ids {
            self.params.insert(id.code.i64(), param.into());
        }
    }
    /// Set an f64 parameter
    pub fn set_f64(&mut self, id: i64, value: f64) {
        self.params.insert(id, value.into());
    }
    /// Set a variable from user input (usually used by the API / TUI)
    pub fn set_var(&mut self, name: &str, value: &str) {
        let name_id = name_to_param_id(name);
        self.params.insert(name_id, value.into());
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
