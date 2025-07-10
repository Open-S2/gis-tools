use crate::proj::{
    Proj, ProjectionTransform, SRS_WGS84_ESQUARED, SRS_WGS84_SEMIMAJOR, SRS_WGS84_SEMIMINOR,
    TransformCoordinates,
};
use alloc::{vec, vec::Vec};
use core::f64::consts::TAU;
use core::f64::consts::{FRAC_PI_2, PI};
use libm::{atan, atan2, cos, sin, sqrt};

/// Check if the source and destination datums are not WGS84
pub fn check_not_wgs84(src: &ProjectionTransform, dest: &ProjectionTransform) -> bool {
    let src_proj = src.proj.borrow();
    let dest_proj = dest.proj.borrow();
    !src_proj.datum_type.is_wgs84(&src_proj.datum_params)
        || !dest_proj.datum_type.is_wgs84(&dest_proj.datum_params)
}

/// Datum Type helps quicker assessment of a datum
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub enum DatumType {
    /// 3 Parameter Datum
    Param3 = 1,
    /// 7 Parameter Datum
    Param7 = 2,
    /// Grid Shift
    GridShift = 3,
    /// WGS84 (Base case)
    WGS84 = 4,
    /// Unknown
    #[default]
    NoDatum = 5,
}
impl DatumType {
    fn is_params(&self) -> bool {
        matches!(self, DatumType::Param3 | DatumType::Param7)
    }

    /// Check if the datum type is WGS84
    pub fn is_wgs84(&self, params: &DatumParams) -> bool {
        matches!(self, DatumType::WGS84) || params.is_wgs84()
    }
}

/// Datum Parameters can be either 3 or 7
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum DatumParams {
    /// 3 parameter datum (translate-x, translate-y, translate-z)
    Param3(f64, f64, f64),
    /// 7 parameter datum (translate-x, translate-y, translate-z, rotate-x, rotate-y, rotate-z, scale)
    Param7(f64, f64, f64, f64, f64, f64, f64),
}
impl Default for DatumParams {
    fn default() -> Self {
        DatumParams::Param3(0.0, 0.0, 0.0)
    }
}
impl DatumParams {
    /// Returns the datum parameters as a vector
    pub fn to_vec(&self) -> Vec<f64> {
        match self {
            DatumParams::Param3(x, y, z) => vec![*x, *y, *z],
            DatumParams::Param7(x, y, z, rx, ry, rz, s) => vec![*x, *y, *z, *rx, *ry, *rz, *s],
        }
    }
    /// Given a vector of datum parameters, returns a DatumParams
    pub fn from_vec(v: Vec<f64>) -> DatumParams {
        if v.len() == 3 {
            DatumParams::Param3(v[0], v[1], v[2])
        } else {
            DatumParams::Param7(v[0], v[1], v[2], v[3], v[4], v[5], v[6])
        }
    }
    /// Check if the datum is WGS84
    pub fn is_wgs84(&self) -> bool {
        match self {
            Self::Param3(p3_1, p3_2, p3_3) => *p3_1 == 0. && *p3_2 == 0. && *p3_3 == 0.,
            Self::Param7(p7_1, p7_2, p7_3, p7_4, p7_5, p7_6, p7_7) => {
                *p7_1 == 0.
                    && *p7_2 == 0.
                    && *p7_3 == 0.
                    && *p7_4 == 0.
                    && *p7_5 == 0.
                    && *p7_6 == 0.
                    && *p7_7 == 0.
            }
        }
    }
    /// Geocentric to WGS84
    /// pj_geocentic_to_wgs84( p )
    /// p = point to transform in geocentric coordinates (x,y,z)
    /// point object, nothing fancy, just allows values to be
    /// passed back and forth by reference rather than by value.
    /// Other point classes may be used as long as they have
    /// x and y properties, which will get modified in the transform method.
    pub fn to_wgs84<P: TransformCoordinates>(&self, p: &mut P) {
        match self {
            Self::Param3(p3_x, p3_y, p3_z) => {
                p.set_x(p.x() + *p3_x);
                p.set_y(p.y() + *p3_y);
                p.set_z(p.z() - *p3_z);
            }
            Self::Param7(p7_dx, p7_dy, p7_dz, p7_rx, p7_ry, p7_rz, p7_s) => {
                let z = p.z();
                p.set_x(p7_s * (p.x() - p7_rz * p.y() + p7_ry * z) + *p7_dx);
                p.set_y(p7_s * (p7_rz * p.x() + p.y() - p7_rx * z) + *p7_dy);
                p.set_z(p7_s * (-p7_ry * p.x() + p7_rx * p.y() + z) + *p7_dz);
            }
        }
    }

    /// Geocentric from WGS84
    /// pj_geocentic_from_wgs84() coordinate system definition,
    /// point to transform in geocentric coordinates (x,y,z)
    pub fn from_wgs84<P: TransformCoordinates>(&self, p: &mut P) {
        match self {
            Self::Param3(p3_x, p3_y, p3_z) => {
                p.set_x(p.x() - *p3_x);
                p.set_y(p.y() - *p3_y);
                p.set_z(p.z() - *p3_z);
            }
            Self::Param7(p7_dx, p7_dy, p7_dz, p7_rx, p7_ry, p7_rz, p7_s) => {
                let z = p.z();
                let x_tmp = (p.x() - *p7_dx) / *p7_s;
                let y_tmp = (p.y() - *p7_dy) / *p7_s;
                let z_tmp = (z - *p7_dz) / *p7_s;
                p.set_x(x_tmp + *p7_rz * y_tmp - *p7_ry * z_tmp);
                p.set_y(-p7_rz * x_tmp + y_tmp + *p7_rx * z_tmp);
                p.set_z(*p7_ry * x_tmp - *p7_rx * y_tmp + z_tmp);
            }
        }
    }
}

/**
 * Transforms a point from one datum to another
 * point - lon-lat WGS84 point to mutate
 * source - source projection
 * dest - destination projection
 */
pub fn datum_transform<P: TransformCoordinates>(point: &mut P, source: &Proj, dest: &Proj) {
    // Short cut if the datums are identical.
    if source.datum_type == dest.datum_type
        || source.datum_type == DatumType::NoDatum
        || dest.datum_type == DatumType::NoDatum
    {
        return;
    }

    // If this datum requires grid shifts, then apply it to geodetic coordinates.
    let mut source_a = source.a;
    let mut source_es = source.es;
    if source.datum_type == DatumType::GridShift {
        // source
        // TODO:
        // apply_grid_shift(source, false, point);
        source_a = SRS_WGS84_SEMIMAJOR;
        source_es = SRS_WGS84_ESQUARED;
    }

    let mut dest_a = dest.a;
    let mut dest_b = dest.b;
    let mut dest_es = dest.es;
    if dest.datum_type == DatumType::GridShift {
        dest_a = SRS_WGS84_SEMIMAJOR;
        dest_b = SRS_WGS84_SEMIMINOR;
        dest_es = SRS_WGS84_ESQUARED;
    }

    // Do we need to go through geocentric coordinates?
    if source_es == dest_es
        && source_a == dest_a
        && !source.datum_type.is_params()
        && !dest.datum_type.is_params()
    {
        return;
    }

    // Convert to geocentric coordinates.
    geodetic_to_geocentric(point, source_es, source_a);
    // Convert between datums
    if source.datum_type.is_params() {
        // geocentric_to_wgs84(point, source.datum_type, source.datum_params);
        source.datum_params.to_wgs84(point);
    }
    if dest.datum_type.is_params() {
        // geocentric_from_wgs84(point, dest.datum_type, dest.datum_params);
        dest.datum_params.from_wgs84(point);
    }
    // Convert back to geodetic coordinates.
    geocentric_to_geodetic(point, dest_es, dest_a, dest_b);

    if dest.datum_type == DatumType::GridShift {
        // TODO:
        // apply_grid_shift(dest, true, point);
    }
}

/// The function Convert_Geodetic_To_Geocentric converts geodetic coordinates
/// (latitude, longitude, and height) to geocentric coordinates (X, Y, Z),
/// according to the current ellipsoid parameters.
///
/// Latitude  : Geodetic latitude in radians                     (input)
/// Longitude : Geodetic longitude in radians                    (input)
/// Height    : Geodetic height, in meters                       (input)
/// X         : Calculated Geocentric X coordinate, in meters    (output)
/// Y         : Calculated Geocentric Y coordinate, in meters    (output)
/// Z         : Calculated Geocentric Z coordinate, in meters    (output)
/// p - lon-lat WGS84 point
/// es - eccentricity
/// a - semi-major axis
pub fn geodetic_to_geocentric<P: TransformCoordinates>(p: &mut P, es: f64, a: f64) {
    let mut longitude = p.x();
    let mut latitude = p.y();
    let height = p.z(); //Z value not always supplied
    // Don't blow up if Latitude is just a little out of the value
    // range as it may just be a rounding issue.  Also removed longitude
    // test, it should be wrapped by cos() and sin().  NFW for PROJ.4, Sep/2001.
    if latitude < -FRAC_PI_2 && latitude > -1.001 * FRAC_PI_2 {
        latitude = -FRAC_PI_2;
    } else if latitude > FRAC_PI_2 && latitude < 1.001 * FRAC_PI_2 {
        latitude = FRAC_PI_2;
    } else if !(-FRAC_PI_2..=FRAC_PI_2).contains(&latitude) {
        panic!("geocent:lat out of range: {}", latitude);
    }

    if longitude > PI {
        longitude -= TAU;
    }
    let sin_lat = sin(latitude); /*  sin(latitude)  */
    let cos_lat = cos(latitude); /*  cos(latitude)  */
    let sin2_lat = sin_lat * sin_lat; /*  Square of sin(latitude)  */
    let r_n = a / sqrt(1.0 - es * sin2_lat); /*  Earth radius at location  */

    p.set_x((r_n + height) * cos_lat * cos(longitude));
    p.set_y((r_n + height) * cos_lat * sin(longitude));
    p.set_z((r_n * (1. - es) + height) * sin_lat);
}

/// converts a geocentric point to a geodetic point
pub fn geocentric_to_geodetic<P: TransformCoordinates>(point: &mut P, es: f64, a: f64, _b: f64) {
    /* local defintions and variables */
    /* end-criterium of loop, accuracy of sin(Latitude) */
    let genau = 1e-12;
    let genau2 = genau * genau;
    let maxiter = 30;

    let mut rx;
    let mut rk;
    let mut rn; /* Earth radius at location */
    let mut cphi0; /* cos of start or old geodetic latitude in iterations */
    let mut sphi0; /* sin of start or old geodetic latitude in iterations */
    let mut cphi; /* cos of searched geodetic latitude */
    let mut sphi; /* sin of searched geodetic latitude */
    let mut sdphi; /* end-criterium: addition-theorem of sin(Latitude(iter)-Latitude(iter-1)) */
    let mut iter; /* # of continous iteration, max. 30 is always enough (s.a.) */

    let x = point.x();
    let y = point.y();
    let z = point.z(); //Z value not always supplied
    let p = sqrt(x * x + y * y); /* distance between semi-minor axis and location */
    let rr = sqrt(x * x + y * y + z * z); /* distance between center and location */
    let longitude = if p / a < genau { 0.0 } else { atan2(y, x) };
    let mut height;

    /* --------------------------------------------------------------
     * Following iterative algorithm was developped by
     * "Institut for Erdmessung", University of Hannover, July 1988.
     * Internet: www.ife.uni-hannover.de
     * Iterative computation of CPHI,SPHI and Height.
     * Iteration of CPHI and SPHI to 10**-12 radian resp.
     * 2*10**-7 arcsec.
     * --------------------------------------------------------------
     */
    let ct = z / rr; /* sin of geocentric latitude */
    let st = p / rr; /* cos of geocentric latitude */
    rx = 1.0 / sqrt(1.0 - es * (2.0 - es) * st * st);
    cphi0 = st * (1.0 - es) * rx;
    sphi0 = ct * rx;
    iter = 0;

    /* loop to find sin(Latitude) resp. Latitude
     * until |sin(Latitude(iter)-Latitude(iter-1))| < genau */
    loop {
        iter += 1;
        rn = a / sqrt(1.0 - es * sphi0 * sphi0);

        /*  ellipsoidal (geodetic) height */
        height = p * cphi0 + z * sphi0 - rn * (1.0 - es * sphi0 * sphi0);

        rk = (es * rn) / (rn + height);
        rx = 1.0 / sqrt(1.0 - rk * (2.0 - rk) * st * st);
        cphi = st * (1.0 - rk) * rx;
        sphi = ct * rx;
        sdphi = sphi * cphi0 - cphi * sphi0;
        cphi0 = cphi;
        sphi0 = sphi;
        if sdphi * sdphi <= genau2 && iter >= maxiter {
            break;
        }
    }

    /*      ellipsoidal (geodetic) latitude */
    let latitude = atan(sphi / cphi.abs());

    point.set_x(longitude);
    point.set_y(latitude);
    point.set_z(height);
}

/// Description of a WGS84 datum
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ToWGS84Datum {
    datum_params: DatumParams,
    ellipse: &'static str,
    datum_name: &'static str,
}

/// WGS84 Datum
pub const TO_WGS84: ToWGS84Datum = ToWGS84Datum {
    datum_params: DatumParams::Param7(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ellipse: "WGS84",
    datum_name: "WGS84",
};

/// Swiss Datum
pub const CH1903: ToWGS84Datum = ToWGS84Datum {
    datum_params: DatumParams::Param3(674.374, 15.056, 405.346),
    ellipse: "bessel",
    datum_name: "swiss",
};

/// Greek_Geodetic_Reference_System_1987 Datum
pub const GGRS87: ToWGS84Datum = ToWGS84Datum {
    datum_params: DatumParams::Param3(-199.87, 74.79, 246.62),
    ellipse: "GRS80",
    datum_name: "Greek_Geodetic_Reference_System_1987",
};

/// North_American_Datum_1983 Datum
pub const NAD83: ToWGS84Datum = ToWGS84Datum {
    datum_params: DatumParams::Param3(0.0, 0.0, 0.0),
    ellipse: "GRS80",
    datum_name: "North_American_Datum_1983",
};

/// Potsdam Rauenberg 1950 DHDN Datum
pub const POTSDAM: ToWGS84Datum = ToWGS84Datum {
    datum_params: DatumParams::Param7(598.1, 73.7, 418.2, 0.202, 0.045, -2.455, 6.7),
    ellipse: "bessel",
    datum_name: "Potsdam Rauenberg 1950 DHDN",
};

/// Carthage 1934 Tunisia Datum
pub const CARTHAGE: ToWGS84Datum = ToWGS84Datum {
    datum_params: DatumParams::Param3(-263.0, 6.0, 431.0),
    ellipse: "clark80",
    datum_name: "Carthage 1934 Tunisia",
};

/// Hermannskogel Datum
pub const HERMANNSKOGEL: ToWGS84Datum = ToWGS84Datum {
    datum_params: DatumParams::Param7(577.326, 90.129, 463.919, 5.137, 1.474, 5.297, 2.4232),
    ellipse: "bessel",
    datum_name: "Hermannskogel",
};

/// Militar-Geographische Institut Datum
pub const MILITARGEOGRAPHISCHE_INSTITUT: ToWGS84Datum = ToWGS84Datum {
    datum_params: DatumParams::Param7(577.326, 90.129, 463.919, 5.137, 1.474, 5.297, 2.4232),
    ellipse: "bessel",
    datum_name: "Militar-Geographische Institut",
};

/// Irish National Datum
pub const OSNI52: ToWGS84Datum = ToWGS84Datum {
    datum_params: DatumParams::Param7(482.53, -130.596, 564.557, -1.042, -0.214, -0.631, 8.15),
    ellipse: "airy",
    datum_name: "Irish National",
};

/// Ireland 1965 Datum
pub const IRE65: ToWGS84Datum = ToWGS84Datum {
    datum_params: DatumParams::Param7(482.53, -130.596, 564.557, -1.042, -0.214, -0.631, 8.15),
    ellipse: "mod_airy",
    datum_name: "Ireland 1965",
};

/// Rassadiran Datum
pub const RASSADIRAN: ToWGS84Datum = ToWGS84Datum {
    datum_params: DatumParams::Param3(-133.63, -157.5, -158.62),
    ellipse: "intl",
    datum_name: "Rassadiran",
};

/// New Zealand Geodetic Datum 1949 Datum
pub const NZGD49: ToWGS84Datum = ToWGS84Datum {
    datum_params: DatumParams::Param7(59.47, -5.04, 187.44, 0.47, -0.1, 1.024, -4.5993),
    ellipse: "intl",
    datum_name: "New Zealand Geodetic Datum 1949",
};

/// Airy 1830 Datum
pub const OSGB36: ToWGS84Datum = ToWGS84Datum {
    datum_params: DatumParams::Param7(446.448, -125.157, 542.06, 0.1502, 0.247, 0.8421, -20.4894),
    ellipse: "airy",
    datum_name: "Airy 1830",
};

/// S-JTSK (Ferro) Datum
pub const S_JTSK: ToWGS84Datum = ToWGS84Datum {
    datum_params: DatumParams::Param3(589.0, 76.0, 480.0),
    ellipse: "bessel",
    datum_name: "S-JTSK (Ferro)",
};

/// Beduaram Datum
pub const BEDUARAM: ToWGS84Datum = ToWGS84Datum {
    datum_params: DatumParams::Param3(-106.0, -87.0, 188.0),
    ellipse: "clrk80",
    datum_name: "Beduaram",
};

/// Gunung Segara Jakarta Datum
pub const GUNUNG_SEGARA: ToWGS84Datum = ToWGS84Datum {
    datum_params: DatumParams::Param3(-403.0, 684.0, 41.0),
    ellipse: "bessel",
    datum_name: "Gunung Segara Jakarta",
};

/// Reseau National Belge 1972
pub const RNB72: ToWGS84Datum = ToWGS84Datum {
    datum_params: DatumParams::Param7(
        106.869, -52.2978, 103.724, -0.33657, 0.456955, -1.84218, 1.0,
    ),
    ellipse: "intl",
    datum_name: "Reseau National Belge 1972",
};

/// Given a name, return the corresponding ellipsoid
pub fn get_datum(name: &str) -> Option<ToWGS84Datum> {
    // fix name to remove _ and convert to uppercase
    let name = name.to_uppercase().replace("_", "");
    match name.as_str() {
        "WGS84" => Some(TO_WGS84),
        "CH1903" => Some(CH1903),
        "GGRS87" => Some(GGRS87),
        "NAD83" => Some(NAD83),
        "RASSADIRAN" => Some(RASSADIRAN),
        "NZGD49" => Some(NZGD49),
        "OSGB36" => Some(OSGB36),
        "SJTSK" => Some(S_JTSK),
        "BEDUARAM" => Some(BEDUARAM),
        "POTSDAM" => Some(POTSDAM),
        "CARTHAGE" => Some(CARTHAGE),
        "HERMANNSKOGEL" => Some(HERMANNSKOGEL),
        "MILITARGEOGRAPHISCHEINSTITUT" => Some(MILITARGEOGRAPHISCHE_INSTITUT),
        "OSNI52" => Some(OSNI52),
        "IRE65" => Some(IRE65),
        "RNB72" => Some(RNB72),
        "GUNUNG_SEGARA" => Some(GUNUNG_SEGARA),
        _ => None,
    }
}
