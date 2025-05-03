use alloc::{vec, vec::Vec};

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
