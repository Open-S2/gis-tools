use super::{DatumParams, DatumType, geodesic::GeodGeodesic};
use alloc::string::String;

/// A generic 4-dimensional point/vector
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Coords(pub f64, pub f64, pub f64, pub f64);

/// Generic Projection Container
#[derive(Debug, Default)]
#[allow(non_snake_case)]
pub struct Proj {
    // GENERAL PARAMETER
    /// Short name of the current projection
    pub short_name: String,
    /// Projection description
    pub descr: String,
    /// Full textual definition (usually 0 - set by proj_pj_info)
    pub def_full: String,

    /// For geodesic computations
    pub geod: Option<GeodGeodesic>,
    // void *opaque = nullptr; /* Projection specific parameters, Defined in PJ_*.c */
    /// Tell high level API functions to swap inv/fwd
    pub inverted: i32, // 0, /* Tell high level API functions to swap inv/fwd */

    // ELLIPSOID PARAMETERS

    // The linear parameters
    /// semimajor axis (radius if eccentricity==0)
    pub a: f64,
    /// semiminor axis
    pub b: f64,
    /// 1 / a
    pub ra: f64,
    /// 1 / b
    pub rb: f64,
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
    /// proj=latlong, ... not really a projection at all
    pub is_ll: bool,
    /// proj=geocent ... not really a projection at all
    pub is_geocent: bool,
    /// 0 for operations that are purely cartesian
    pub need_ellps: bool,
    /// flag to indicate we skip fwd prepare
    pub skip_fwd_prepare: bool,
    /// flag to indicate we skip fwd finalize
    pub skip_fwd_finalize: bool,
    /// flag to indicate we skip inv prepare
    pub skip_inv_prepare: bool,
    /// flag to indicate we skip inv finalize
    pub skip_inv_finalize: bool,

    /// Left flag for input/output coordinate types
    pub left: IoUnits,
    /// Right flag for input/output coordinate types
    pub right: IoUnits,

    // These Projs are used for implementing cs2cs style coordinate handling in the 4D API
    // /// axisswap step
    // pub axisswap: Option<Rc<RefCell<Proj>>>,
    // /// cartesian step
    // pub cart: Option<Rc<RefCell<Proj>>>,
    // /// cartesian wgs84 step
    // pub cart_wgs84: Option<Rc<RefCell<Proj>>>,
    // /// helmert step
    // pub helmert: Option<Rc<RefCell<Proj>>>,
    // /// horizontal grid shift
    // pub hgridshift: Option<Rc<RefCell<Proj>>>,
    // /// vertical grid shift
    // pub vgridshift: Option<Rc<RefCell<Proj>>>,

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
    /// 0.0 for -180 to 180, actually in radians
    pub long_wrap_center: f64,
    /// 0.0 for -180 to 180
    pub is_long_wrap_set: bool,
    /// Axis order, pj_transform / pj_adjust_axis
    pub axis: [char; 4],

    // ISO-19111 interface
    /// If the operation is a coordinate operation
    pub is_coordinate_operation: bool,
    /// Coordinate epoch
    pub coordinate_epoch: Option<f64>,
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
#[derive(Debug, Default)]
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
