// Primitives

/// Common Tolerance
pub const EPS10: f64 = 1e-10;
/// 3 parameter transform
pub const PJD_3PARAM: u8 = 1;
/// 7 parameter transform
pub const PJD_7PARAM: u8 = 2;
/// Grid shift transform
pub const PJD_GRIDSHIFT: u8 = 3;
/// WGS84 or equivalent
pub const PJD_WGS84: u8 = 4; // WGS84 or equivalent
/// No datum applied
pub const PJD_NODATUM: u8 = 5; // WGS84 or equivalent
/// SRS WGS84 semimajor only used in grid shift transforms
pub const SRS_WGS84_SEMIMAJOR: f64 = 6378137.0; // only used in grid shift transforms
/// SRS WGS84 semiminor only used in grid shift transforms
pub const SRS_WGS84_SEMIMINOR: f64 = 6356752.314; // only used in grid shift transforms
/// SRS WGS84 esquared only used in grid shift transforms
pub const SRS_WGS84_ESQUARED: f64 = 0.0066943799901413165; // only used in grid shift transforms
/// 1/4 π - Sec to Radians
pub const SEC_TO_RAD: f64 = 4.848_136_811_095_36e-6;
/// ellipoid pj_set_ell.c
pub const SIXTH: f64 = 0.166_666_666_666_666_66;
/// 1/6
pub const RA4: f64 = 0.047_222_222_222_222_22;
/// 17/360
pub const RA6: f64 = 0.022_156_084_656_084_655;
/// An arc minute is 1/60th of a degree [(π/180) / 60 radians]
pub const MIN2R: f64 = 0.0002908882086657216;
/// An arc second is 1/60th of an arc minute, or 1/3600th of a degree [(π/180) / 3600 radians]
pub const SEC2R: f64 = 0.00000484813681109536;
/// The grad, or gradian, is a unit of angular measure where a right angle is 100 grads,
/// and a full circle is 400 grads.
/// [1 grad = π/200 radians]
pub const GRD2R: f64 = 0.015707963267948967;
/// The gon is equivalent to the grad and is primarily used in surveying.
/// A full circle is 400 gons.
/// [1 gon = π/200 radians]
pub const GON2R: f64 = GRD2R;
/// SPI is slightly greater than Math.PI, so values that exceed the -180..180
/// degree range by a tiny amount don't get wrapped. This prevents points that
/// have drifted from their original location along the 180th meridian (due to
/// floating point error) from changing their sign.
#[allow(clippy::approx_constant)]
pub const SPI: f64 = 3.14159265359;

/// feet to meters
pub const FT_TO_M: f64 = 0.3048;
/// US feet to meters (1200 / 3937)
pub const US_FT_TO_M: f64 = 0.3048006096012192; // 1200 / 3937
/// US (modified) feet to meters (1200 / 3937)
pub const US_MOD_FT_TO_M: f64 = 0.304_812_252_984_506; // 1200 / 3937
/// Linear foot Clarke to meters
pub const CLARKE_FT_TO_M: f64 = 0.3047972654; // Linear_Foot_Clarke
/// Linear foot Indian to meters
pub const INDIAN_FT_TO_M: f64 = 0.3047995; // Linear_Foot_Indian
/// Linear Yard Indian to meters
pub const INDIAN_YD_TO_M: f64 = 0.91439523;
/// Linear foot Link to meters
pub const LINK_FT_TO_M: f64 = 0.201168; // Linear_Foot_Link
/// International Fathom to meters
pub const INTERNATIONAL_FATHOM_TO_M: f64 = 1.8288; // Linear_Fathom
/// Linear Mile International Nautical
pub const INTERNATIONAL_NAUTICAL_MILE_TO_M: f64 = 1852.0; // Linear_Mile_International_Nautical
// /// Linear Mile International Statute
// pub const INTERNATIONAL_STATUTE_MILE: f64 = 1609.34; // Linear_Mile_International_Statute

// PROJ Specific Constants

/// Used by AEA. Input is boolean
pub const SOUTH: i64 = 1;
/// Used by Airy. Input is in degrees
pub const LAT_B: i64 = 2;
/// Used by Airy. Input is boolean
pub const NO_CUT: i64 = 3;
/// Used by Transverse Mercator. Input is boolean
pub const APPROX: i64 = 4;
/// Used by Transverse Mercator. Input is a string
pub const ALGO: i64 = 5;
// /// Latitude of true scale. Defines the latitude where scale is not distorted.
// /// Takes precedence over ``+k_0`` if both options are used together.
// /// The default convention is to interpret this value as decimal degrees. To
// /// specify radians instead, follow the value with the "r" character.
// pub const LAT_TS: i64 = 6;
/// hyperbolic
pub const HYPERBOLIC: i64 = 7;
/// UTM Zone (Used by Universal Transverse Mercator). Value is "real" in range 1-60
pub const ZONE: i64 = 8;
/// Czech Republic flag (Used by Krovak). Value is a "bool"
pub const CZECH: i64 = 9;
/// User defined theta file
pub const THETA: i64 = 10;
/// User defined n value file
pub const N_VAL: i64 = 11;
/// User defined m value file
pub const M_VAL: i64 = 12;
/// LAT1 or Latitude_Of_1st_Point or Latitude of 1st point
pub const LATITUDE_OF_FIRST_POINT: i64 = 13;
/// LAT2 or Latitude_Of_2nd_Point or Latitude of 2nd point
pub const LATITUDE_OF_SECOND_POINT: i64 = 14;
/// LON1 or Longitude_Of_1st_Point or Longitude of 1st point
pub const LONGITUDE_OF_FIRST_POINT: i64 = 15;
/// LON2 or Longitude_Of_2nd_Point or Longitude of 2nd point
pub const LONGITUDE_OF_SECOND_POINT: i64 = 16;
/// NO ROTATION Flag. Used by OMERC projections
pub const NO_ROTATION: i64 = 17;
/// No Off. An OMERC projection variable
pub const NO_OFF: i64 = 18;
/// No Unsigned Off. An OMERC projection variable
pub const NO_UOFF: i64 = 19;
/// GUAM flag. Used by Azimuthal Equidistant projection
pub const GUAM: i64 = 20;

/// Converts a name to a parameter id
pub fn name_to_param_id(name: &str) -> i64 {
    match name.to_lowercase().as_str() {
        "south" => SOUTH,
        "lat_b" => LAT_B,
        "lat_ts" => LATITUDE_STD_PARALLEL,
        "no_cut" => NO_CUT,
        "approx" => APPROX,
        "algo" => ALGO,
        "hyperbolic" => HYPERBOLIC,
        "zone" => ZONE,
        "czech" => CZECH,
        "theta" => THETA,
        "n" => N_VAL,
        "m" => M_VAL,
        "lat_1" | "Latitude_Of_1st_Point" | "Latitude of 1st point" => LATITUDE_OF_FIRST_POINT,
        "lat_2" | "Latitude_Of_2nd_Point" | "Latitude of 2nd point" => LATITUDE_OF_SECOND_POINT,
        "lon_1" | "Longitude_Of_1st_Point" | "Longitude of 1st point" => LONGITUDE_OF_FIRST_POINT,
        "lon_2" | "Longitude_Of_2nd_Point" | "Longitude of 2nd point" => LONGITUDE_OF_SECOND_POINT,
        "no_rot" => NO_ROTATION,
        "no_off" => NO_OFF,
        "no_unoff" => NO_UOFF,
        "guam" => GUAM,
        _ => -1,
    }
}

// Map projection methods

/// Web Mercator / Pseudo Mercator Projection
/// - [EPSG Link](https://epsg.org/coord-operation-method_1024/Popular-Visualisation-Pseudo-Mercator.html)
/// - EPSG Code: 1024
/// - EPSG Codes Used by Web Mercator: 8801, 8802, 8806, 8807
/// - Aliases: "Web Mercator", "Pseudo Mercator", "webmerc"
pub const WEB_MERCATOR: i64 = 1024;
/// Azimuthal Equidistant
/// - [EPSG Link](https://epsg.org/coord-operation-method_1125/Azimuthal-Equidistant.html)
/// - EPSG Code: 9821
/// - EPSG Codes Used by AEQD: 8801, 8802, 8806, 8807
/// - Aliases: "aeqd"
pub const AZIMUTHAL_EQUIDISTANT: i64 = 1125;
/// Mercator (Spherical)
/// - [EPSG Link](https://epsg.org/coord-operation-method_1026/Mercator-Spherical.html)
/// - EPSG Code: 1026
/// - EPSG Codes Used by Mercator: 8801, 8802, 8806, 8807
/// - Aliases: "Mercator", "merc"
pub const MERCATOR: i64 = 1026;
/// Mercator Variant A
/// - [EPSG Link](https://epsg.org/coord-operation-method_9804/Mercator-variant-A.html)
/// - EPSG Code: 9804
/// - EPSG Codes Used by Mercator: 8801, 8802, 8805, 8806, 8807
/// - Aliases: "Mercator", "merc"
pub const MERCATOR_VARIANT_A: i64 = 9804;
/// Mercator Variant B
/// - [EPSG Link](https://epsg.org/coord-operation-method_9805/Mercator-variant-B.html)
/// - EPSG Code: 9805
/// - EPSG Codes Used by Mercator: 8823, 8802, 8806, 8807
/// - Aliases: "Mercator", "merc"
pub const MERCATOR_VARIANT_B: i64 = 9805;
/// Albers Equal Area
/// - [EPSG Link](https://epsg.org/coord-operation-method_9822/Albers-Equal-Area.html)
/// - EPSG Code: 9822
/// - EPSG Codes Used by AEA: 8821, 8822, 8823, 8824, 8826, 8827
/// - Aliases: "Albers", "aea"
pub const ALBERS_EQUAL_AREA: i64 = 9822;
/// Bonne
/// - [EPSG Link](https://epsg.org/coord-operation-method_9827/Bonne.html)
/// - EPSG Code: 9827
/// - EPSG Codes Used by Bonne: 8801, 8802, 8806, 8807
/// - Aliases: "bonne"
pub const BONNE: i64 = 9827;
/// Equal Earth
/// - [EPSG Link](https://epsg.org/coord-operation-method_1078/Equal-Earth.html)
/// - EPSG Code: 1078
/// - EPSG Codes Used by Equal Earth: 8802, 8806, 8807
/// - Aliases: "eqearth"
pub const EQUAL_EARTH: i64 = 1078;
/// Equidistant Cylindrical
/// - [EPSG Link](https://epsg.org/coord-operation-method_1028/Equidistant-Cylindrical.html)
/// - EPSG Code: 1028
/// - EPSG Codes Used by Equidistant Cylindrical: 8823, 8802, 8806, 8807
/// - Aliases: "eqc", "Equidistant Cylindrical (Plate Carree)"
pub const EQUIDISTANT_CYLINDRICAL: i64 = 1028;
/// Lambert Conic Conformal (1SP)
/// - [EPSG Link](https://epsg.org/coord-operation-method_9801/Lambert-Conic-Conformal-1SP.html)
/// - EPSG Code: 9801
/// - EPSG Codes Used by Lambert Conic Conformal: 8801, 8802, 8805, 8806, 8807
/// - Aliases: "lcc", "Lambert Conic Conformal"
pub const LAMBERT_CONFORMAL_CONIC_1SP: i64 = 9801;
/// Lambert Conic Conformal (2SP)
/// - [EPSG Link](https://epsg.org/coord-operation-method_9802/Lambert-Conic-Conformal-2SP.html)
/// - EPSG Code: 9802
/// - EPSG Codes Used by Lambert Conic Conformal: 8821, 8822, 8823. 8824, 8826, 8827
/// - Aliases: "lcc", "Lambert Conic Conformal"
pub const LAMBERT_CONFORMAL_CONIC_2SP: i64 = 9802;
/// Cassini-Soldner
/// - [EPSG Link](https://epsg.org/coord-operation-method_9806/Cassini-Soldner.html)
/// - EPSG Code: 9806
/// - EPSG Codes Used by Cassini-Soldner: 8801, 8802, 8806, 8807
/// - Aliases: "cass", "Cassini"
pub const CASSINI: i64 = 9806;
/// Orthographic
/// - [EPSG Link](https://epsg.org/coord-operation-method_9840/Orthographic.html)
/// - EPSG Code: 9840
/// - EPSG Codes Used by Orthographic: 8801, 8802, 8806, 8807
pub const ORTHOGRAPHIC: i64 = 9840;
/// Swiss Oblique Cylindrical
/// - [EPSG Link](https://epsg.org/coord-operation-method_9814/Swiss-Oblique-Cylindrical.html)
/// - EPSG Code: 9814
/// - EPSG Codes Used by SOMERC: 8811, 8812, 8816, 8817
pub const SOMERC: i64 = 9814;
/// Transverse Mercator
/// - [EPSG Link](https://epsg.org/coord-operation-method_9807/Transverse-Mercator.html)
/// - EPSG Code: 9807
/// - EPSG Codes Used by Tmerc: 8801, 8802, 8805, 8806, 8807
pub const TRANSVERSE_MERCATOR: i64 = 9807;
/// Transverse Mercator (South Orientated)
/// - [EPSG Link](https://epsg.org/coord-operation-method_9808/Transverse-Mercator-South-Orientated.html)
/// - EPSG Code: 9808
/// - EPSG Codes Used by Tmerc: 8801, 8802, 8805, 8806, 8807
pub const TRANSVERSE_MERCATOR_SOUTH_ORIENTATED: i64 = 9808;
/// Oblique Stereographic
/// - [EPSG Link](https://epsg.org/coord-operation-method_9809/Oblique-Stereographic.html)
/// - EPSG Code: 9809
/// - EPSG Codes Used by Stereographic: 8801, 8802, 8805, 8806, 8807
pub const OBLIQUE_STEREOGRAPHIC: i64 = 9809;
/// Polar Stereographic (variant A)
/// - [EPSG Link](https://epsg.org/coord-operation-method_9810/Polar-Stereographic-variant-A.html)
/// - EPSG Code: 9810
/// - EPSG Codes Used by Stereographic: 8801, 8802, 8805, 8806, 8807
pub const POLAR_STEREOGRAPHIC_VARIANT_A: i64 = 9810;
/// Polar Stereographic (variant B)
/// - [EPSG Link](https://epsg.org/coord-operation-method_9829/Polar-Stereographic-variant-B.html)
/// - EPSG Code: 9829
/// - EPSG Codes Used by Stereographic: 8832, 8833, 8806, 8807
pub const POLAR_STEREOGRAPHIC_VARIANT_B: i64 = 9829;
/// Polar Stereographic (variant C)
/// - [EPSG Link](https://epsg.org/coord-operation-method_9830/Polar-Stereographic-variant-C.html)
/// - EPSG Code: 9830
/// - EPSG Codes Used by Stereographic: 8832, 8833, 8826, 8827
pub const POLAR_STEREOGRAPHIC_VARIANT_C: i64 = 9830;
/// Krovak
/// - [EPSG Link](https://epsg.org/coord-operation-method_9819/Krovak.html)
/// - EPSG Code: 9819
/// - EPSG Codes Used by Krovak: 8811, 8833, 1036, 8818, 8819, 8806, 8807
pub const KROVAK: i64 = 9819;
/// Krovak (North Orientated)
/// - [EPSG Link](https://epsg.org/coord-operation-method_1041/Krovak-North-Orientated.html)
/// - EPSG Code: 1041
/// - EPSG Codes Used by Krovak: 8811, 8833, 1036, 8818, 8819, 8806, 8807
pub const KROVAK_NORTH_ORIENTED: i64 = 1041;
/// Krovak Modified
/// - [EPSG Link](https://epsg.org/coord-operation-method_1042/Krovak-Modified.html)
/// - EPSG Code: 1042
/// - EPSG Codes Used by Krovak: 8811, 8833, 1036, 8818, 8819, 8806, 8807, 8617, 8618
pub const KROVAK_MODIFIED: i64 = 1042;
/// Krovak Modified (North Orientated)
/// - [EPSG Link](https://epsg.org/coord-operation-method_1043/Krovak-Modified-North-Orientated.html)
/// - EPSG Code: 1043
/// - EPSG Codes Used by Krovak: 8811, 8833, 1036, 8818, 8819, 8806, 8807, 8617, 8618
pub const KROVAK_MODIFIED_NORTH_ORIENTED: i64 = 1043;
/// Equidistant Conic
/// - [EPSG Link](https://epsg.org/coord-operation-method_1119/Equidistant-Conic.html)
/// - EPSG Code: 1119
/// - EPSG Codes Used by Equidistant Conic: 8821, 8822, 8823, 8824, 8826, 8827
pub const EQUIDISTANT_CONIC: i64 = 1119;
/// Lambert Azimuthal Equal Area
/// - [EPSG Link](https://epsg.org/coord-operation-method_9820/Lambert-Azimuthal-Equal-Area.html)
/// - EPSG Code: 9820
/// - EPSG Codes Used by LEA: 8801, 8802, 8806, 8807
/// - Aliases: "Lambert Equal Area", "laea"
pub const LAMBERT_AZIMUTHAL_EQUAL_AREA: i64 = 9820;
/// Lambert Azimuthal Equal Area (Spherical)
/// - [EPSG Link](https://epsg.org/coord-operation-method_1027/Lambert-Azimuthal-Equal-Area-Spherical.html)
/// - EPSG Code: 1027
/// - EPSG Codes Used by Lambert Azimuthal Equal Area (Spherical): 8801, 8802, 8806, 8807
/// - Aliases: "laea"
pub const LAMBERT_AZIMUTHAL_EQUAL_AREA_SPHERICAL: i64 = 1027;
/// Polyconic
/// - [EPSG Link](https://epsg.org/coord-operation-method_9818/Polyconic.html)
/// - EPSG Code: 9818
/// - EPSG Codes Used by Polyconic: 8801, 8802, 8806, 8807
/// - Aliases: "Polyconic (American)", "poly"
pub const POLYCONIC: i64 = 9818;
/// Hotine Oblique Mercator (variant A)
/// - [EPSG Link](https://epsg.org/coord-operation-method_9812/Hotine-Oblique-Mercator-variant-A.html)
/// - EPSG Code: 9812
/// - EPSG Codes Used by Hotine Oblique Mercator (variant A): 8811, 8812, 8813, 8814, 8815, 8806, 8807
pub const HOTINE_OBLIQUE_MERCATOR_VARIANT_A: i64 = 9812;
/// Hotine Oblique Mercator (variant B)
/// - [EPSG Link](https://epsg.org/coord-operation-method_9815/Hotine-Oblique-Mercator-variant-B.html)
/// - EPSG Code: 9815
/// - EPSG Codes Used by Hotine Oblique Mercator (variant B): 8811, 8812, 8813, 8814, 8815, 8816, 8817
pub const HOTINE_OBLIQUE_MERCATOR_VARIANT_B: i64 = 9815;

// Map projection parameters

/// Latitude of natural origin - ANGLE UNIT
///
/// geodetic latitude of the point from which the values of both the geographical coordinates on the
/// ellipsoid and the grid coordinates on the projection are deemed to increment or decrement for
/// computational purposes
/// Alternatively: geodetic latitude of the point which in the absence of application of false
/// coordinates has grid coordinates of (0,0).
/// - EPSG Code: `8801`
/// - Aliases: "latitude of origin", "latitude of natural origin"
pub const LATITUDE_OF_NATURAL_ORIGIN: i64 = 8801;
/// Longitude of natural origin - ANGLE UNIT
///
/// geodetic longitude of the point from which the values of both the geographical coordinates on
/// the ellipsoid and the grid coordinates on the projection are deemed to increment or decrement
/// for computational purposes
/// Alternatively: geodetic longitude of the point which in the absence of application of false
/// coordinates has grid coordinates of (0,0).
/// - EPSG Code: `8802`
/// - Aliases: "longitude of origin", "longitude of natural origin", "central meridian"
pub const LONGITUDE_OF_NATURAL_ORIGIN: i64 = 8802;
/// Scale factor at natural origin - SCALE UNIT
///
/// factor by which the map grid is reduced or enlarged during the projection process, defined by its value at the natural origin
/// - EPSG Code: `8805`
/// - Aliases: "scale factor at natural origin", "scale factor", "k", "k0", "k_0", "lat_ts" (lat_ts takes precidence over k0)
pub const SCALE_FACTOR_AT_NATURAL_ORIGIN: i64 = 8805;
/// False easting - LENGTH UNIT
///
/// value assigned to the abscissa (east or west) axis of the projection grid at the natural origin
/// - EPSG Code: `8806`
/// - Aliases: "false easting", "x0"
pub const FALSE_EASTING: i64 = 8806;
/// False northing - LENGTH UNIT
///
/// value assigned to the ordinate (north or south) axis of the projection grid at the natural origin
/// - EPSG Code: `8807`
/// - Aliases: "false northing", "y0"
pub const FALSE_NORTHING: i64 = 8807;
/// Latitude of projection centre - ANGLE UNIT
///
/// For an oblique projection, this is the latitude of the point at which the azimuth of the central
/// line is defined.
/// - [EPSG Link](https://epsg.org/coord-op-param_8811/Latitude-of-projection-centre.html)
/// - EPSG Code: `8811`
/// - Aliases: "latitude of projection centre", "lat0", "lat_0"
pub const LATITUDE_OF_PROJECTION_CENTRE: i64 = 8811;
/// Longitude of projection centre - ANGLE UNIT
///
/// For an oblique projection, this is the longitude of the point at which the azimuth of the central
/// line is defined.
/// - [EPSG Link](https://epsg.org/coord-op-param_8812/Longitude-of-projection-centre.html)
/// - EPSG Code: `8812`
/// - Aliases: "longitude of projection centre", "lon0", "lon_0" "lonc"
pub const LONGITUDE_OF_PROJECTION_CENTRE: i64 = 8812;
/// Longitude of origin - ANGLE UNIT
///
/// For polar aspect azimuthal projections, the meridian along which the northing axis increments
/// and also across which parallels of latitude increment towards the north pole.
/// - [EPSG Link](https://epsg.org/coord-op-param_8833/Longitude-of-origin.html)
/// - EPSG Code: `8833`
/// - Aliases: "longitude of origin", "lon0"
pub const LONGITUDE_OF_ORIGIN: i64 = 8833;
/// Latitude of False Origin - ANGLE UNIT
///
/// geodetic latitude of the point which is not the natural origin and at which grid coordinate
/// values false easting and false northing are defined
/// - EPSG Code: `8821`
/// - Aliases: "latitude of false origin", "latitude of origin", "latitude of natural origin",
///   "central_parallel", "lat0"
pub const LATITUDE_OF_FALSE_ORIGIN: i64 = 8821;
/// Longitude of False Origin - ANGLE UNIT
///
/// geodetic longitude of the point which is not the natural origin and at which grid coordinate
/// values false easting and false northing are defined
/// - EPSG Code: `8822`
/// - Aliases: "longitude of false origin", "longitude of natural origin", "longitude of origin",
///   "central_meridian", "lon0"
pub const LONGITUDE_OF_FALSE_ORIGIN: i64 = 8822;
/// Latitude of 1st standard parallel - ANGLE UNIT
///
/// For a conic projection with two standard parallels, this is the latitude of one of the
/// parallels of intersection of the cone with the ellipsoid. It is normally but not necessarily
/// that nearest to the pole. Scale is true along this parallel.
/// - [EPSG Link](https://epsg.org/coord-op-param_8823/Latitude-of-1st-standard-parallel.html)
/// - EPSG Code: `8823`
/// - Aliases: "latitude of 1st standard parallel", "lat1"/"lat_1"
pub const LATITUDE_OF_FIRST_STANDARD_PARALLEL: i64 = 8823;
/// Latitude of 2nd standard parallel - ANGLE UNIT
///
/// For a conic projection with two standard parallels, this is the latitude of one of the
/// parallels at which the cone intersects with the ellipsoid. It is normally but not necessarily
/// that nearest to the equator. Scale is true along this parallel.
/// - [EPSG Link](https://epsg.org/coord-op-param_8824/Latitude-of-2nd-standard-parallel.html)
/// - EPSG Code: `8824`
/// - Aliases: "latitude of 2nd standard parallel", "lat2"/"lat_2"
pub const LATITUDE_OF_SECOND_STANDARD_PARALLEL: i64 = 8824;
/// Easting at false origin - LENGTH UNIT
///
/// The easting value assigned to the false origin.
/// - [EPSG Link](https://epsg.org/coord-op-param_8826/Easting-at-false-origin.html)
/// - EPSG Code: `8826`
/// - Aliases: "easting at false origin", "x0"
pub const EASTING_AT_FALSE_ORIGIN: i64 = 8826;
/// Northing at false origin - LENGTH UNIT
///
/// The northing value assigned to the false origin.
/// - [EPSG Link](https://epsg.org/coord-op-param_8827/Northing-at-false-origin.html)
/// - EPSG Code: `8827`
/// - Aliases: "northing at false origin", "y0"
pub const NORTHING_AT_FALSE_ORIGIN: i64 = 8827;
/// Latitude of standard parallel - UNIT
///
/// For polar aspect azimuthal projections, the parallel on which the scale factor is defined to be unity.
/// - [EPSG Link](https://epsg.org/coord-op-param_8832/Latitude-of-standard-parallel.html)
/// - EPSG Code: `8832`
/// - Aliases: "standard_parallel_1", "lat_ts"
pub const LATITUDE_STD_PARALLEL: i64 = 8832;
/// Azimuth at projection centre - UNIT
///
/// The azimuthal direction (north zero, east of north being positive) of the great circle which is
/// the centre line of an oblique projection. The azimuth is given at the projection centre.
/// - [EPSG Link](https://epsg.org/coord-op-param_8832/Latitude-of-standard-parallel.html)
/// - EPSG Code: `8832`
/// - Aliases: "Azimuth of initial line", "alpha", "azimuth"
pub const AZIMUTH_PROJECTION_CENTRE: i64 = 8813;
/// Angle from Rectified to Skew Grid - ANGLE UNIT
///
/// The angle at the natural origin of an oblique projection through which the natural coordinate
/// reference system is rotated to make the projection north axis parallel with true north.
/// - [EPSG Link](https://epsg.org/coord-op-param_8814/Angle-from-Rectified-to-Skew-Grid.html)
/// - EPSG Code: `8814`
/// - Aliases: "Angle from Rectified to Skew Grid", "angle_rectified_to_skew_grid", "gamma"
pub const ANGLE_RECTIFIED_TO_SKEW_GRID: i64 = 8814;

// #define EPSG_NAME_PARAMETER_COLATITUDE_CONE_AXIS "Co-latitude of cone axis"
// #define EPSG_CODE_PARAMETER_COLATITUDE_CONE_AXIS 1036

// #define EPSG_NAME_PARAMETER_LATITUDE_PROJECTION_CENTRE                         \
//     "Latitude of projection centre"
// #define EPSG_CODE_PARAMETER_LATITUDE_PROJECTION_CENTRE 8811

// #define EPSG_NAME_PARAMETER_LONGITUDE_PROJECTION_CENTRE                        \
//     "Longitude of projection centre"
// #define EPSG_CODE_PARAMETER_LONGITUDE_PROJECTION_CENTRE 8812

// // Before EPSG 11.015
// #define EPSG_NAME_PARAMETER_SCALE_FACTOR_INITIAL_LINE                          \
//     "Scale factor on initial line"
// #define EPSG_CODE_PARAMETER_SCALE_FACTOR_INITIAL_LINE 8815

// // Since EPSG 11.015
// #define EPSG_NAME_PARAMETER_SCALE_FACTOR_PROJECTION_CENTRE                     \
//     "Scale factor at projection centre"
// #define EPSG_CODE_PARAMETER_SCALE_FACTOR_PROJECTION_CENTRE 8815

// #define EPSG_NAME_PARAMETER_EASTING_PROJECTION_CENTRE                          \
//     "Easting at projection centre"
// #define EPSG_CODE_PARAMETER_EASTING_PROJECTION_CENTRE 8816

// #define EPSG_NAME_PARAMETER_NORTHING_PROJECTION_CENTRE                         \
//     "Northing at projection centre"
// #define EPSG_CODE_PARAMETER_NORTHING_PROJECTION_CENTRE 8817

// #define EPSG_NAME_PARAMETER_LATITUDE_PSEUDO_STANDARD_PARALLEL                  \
//     "Latitude of pseudo standard parallel"
// #define EPSG_CODE_PARAMETER_LATITUDE_PSEUDO_STANDARD_PARALLEL 8818

// #define EPSG_NAME_PARAMETER_SCALE_FACTOR_PSEUDO_STANDARD_PARALLEL              \
//     "Scale factor on pseudo standard parallel"
// #define EPSG_CODE_PARAMETER_SCALE_FACTOR_PSEUDO_STANDARD_PARALLEL 8819

// #define EPSG_NAME_PARAMETER_LATITUDE_FALSE_ORIGIN "Latitude of false origin"
// #define EPSG_CODE_PARAMETER_LATITUDE_FALSE_ORIGIN 8821

// #define EPSG_NAME_PARAMETER_LONGITUDE_FALSE_ORIGIN "Longitude of false origin"
// #define EPSG_CODE_PARAMETER_LONGITUDE_FALSE_ORIGIN 8822

// #define EPSG_NAME_PARAMETER_EASTING_FALSE_ORIGIN "Easting at false origin"
// #define EPSG_CODE_PARAMETER_EASTING_FALSE_ORIGIN 8826

// #define EPSG_NAME_PARAMETER_NORTHING_FALSE_ORIGIN "Northing at false origin"
// #define EPSG_CODE_PARAMETER_NORTHING_FALSE_ORIGIN 8827

// #define EPSG_NAME_PARAMETER_ELLIPSOID_SCALE_FACTOR "Ellipsoid scaling factor"
// #define EPSG_CODE_PARAMETER_ELLIPSOID_SCALE_FACTOR 1038

// #define EPSG_NAME_PARAMETER_LATITUDE_TOPOGRAPHIC_ORIGIN                        \
//     "Latitude of topocentric origin"
// #define EPSG_CODE_PARAMETER_LATITUDE_TOPOGRAPHIC_ORIGIN 8834

// #define EPSG_NAME_PARAMETER_LONGITUDE_TOPOGRAPHIC_ORIGIN                       \
//     "Longitude of topocentric origin"
// #define EPSG_CODE_PARAMETER_LONGITUDE_TOPOGRAPHIC_ORIGIN 8835

// #define EPSG_NAME_PARAMETER_ELLIPSOIDAL_HEIGHT_TOPOCENTRIC_ORIGIN              \
//     "Ellipsoidal height of topocentric origin"
// #define EPSG_CODE_PARAMETER_ELLIPSOIDAL_HEIGHT_TOPOCENTRIC_ORIGIN 8836

// #define EPSG_NAME_PARAMETER_VIEWPOINT_HEIGHT "Viewpoint height"
// #define EPSG_CODE_PARAMETER_VIEWPOINT_HEIGHT 8840

// #define EPSG_NAME_PARAMETER_PROJECTION_PLANE_ORIGIN_HEIGHT                     \
//     "Projection plane origin height"
// #define EPSG_CODE_PARAMETER_PROJECTION_PLANE_ORIGIN_HEIGHT 1039
