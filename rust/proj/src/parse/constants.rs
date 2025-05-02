// Map projection methods

/// Albers Equal Area
/// - EPSG Code: 9822
/// - EPSG Codes Used by AEA: 8821, 8822, 8823, 8824, 8826, 8827
/// - Aliases: "Albers", "aea"
pub const ALBERS_EQUAL_AREA: u32 = 9822;

/// Lambert Equal Area Conic
/// - EPSG Code: 9823
/// - EPSG Codes Used by LEAC: 8831, 8832, 8833, 8834, 8836, 8837
/// - Aliases: "Lambert Equal Area Conic", "leac"
pub const LAMBERT_EQUAL_AREA_CONIC: u32 = 9823;

/// Mercator
/// - EPSG Code: 3395
/// - Aliases: "Mercator", "merc"
pub const MERCATOR: u32 = 3395;

/// Web Mercator / Pseudo Mercator Projection
/// - EPSG Code: 3857
/// - Aliases: "Web Mercator", "Pseudo Mercator", "webmerc"
pub const WEB_MERCATOR: u32 = 3857;

// F.3  Map projection parameters

/// Latitude of natural origin - ANGLE UNIT
/// geodetic latitude of the point from which the values of both the geographical coordinates on the ellipsoid and the grid coordinates on the projection are deemed to increment or decrement for computational purposes
/// Alternatively: geodetic latitude of the point which in the absence of application of false coordinates has grid coordinates of (0,0).
/// - EPSG Code: `8801`
/// - Aliases: "latitude of origin", "latitude of natural origin"
pub const LATITUDE_OF_NATURAL_ORIGIN: u32 = 8801;
/// Longitude of natural origin - ANGLE UNIT
/// geodetic longitude of the point from which the values of both the geographical coordinates on the ellipsoid and the grid coordinates on the projection are deemed to increment or decrement for computational purposes
/// Alternatively: geodetic longitude of the point which in the absence of application of false coordinates has grid coordinates of (0,0).
/// - EPSG Code: `8802`
/// - Aliases: "longitude of origin", "longitude of natural origin", "central meridian"
pub const LONGITUDE_OF_NATURAL_ORIGIN: u32 = 8802;
/// Scale factor at natural origin - SCALE UNIT
/// factor by which the map grid is reduced or enlarged during the projection process, defined by its value at the natural origin
/// - EPSG Code: `8805`
/// - Aliases: "scale factor at natural origin", "scale factor", "k0"
pub const SCALE_FACTOR_AT_NATURAL_ORIGIN: u32 = 8805;
/// False easting - LENGTH UNIT
/// value assigned to the abscissa (east or west) axis of the projection grid at the natural origin
/// - EPSG Code: `8806`
/// - Aliases: "false easting", "x0"
pub const FALSE_EASTING: u32 = 8806;
/// False northing - LENGTH UNIT
/// value assigned to the ordinate (north or south) axis of the projection grid at the natural origin
/// - EPSG Code: `8807`
/// - Aliases: "false northing", "y0"
pub const FALSE_NORTHING: u32 = 8807;
/// Latitude of False Origin - ANGLE UNIT
/// geodetic latitude of the point which is not the natural origin and at which grid coordinate values false easting and false northing are defined
/// - EPSG Code: `8821`
/// - Aliases: "latitude of false origin", "latitude of origin", "latitude of natural origin", "central_parallel", "lat0"
pub const LATITUDE_OF_FALSE_ORIGIN: u32 = 8821;
/// Longitude of False Origin - ANGLE UNIT
/// geodetic longitude of the point which is not the natural origin and at which grid coordinate values false easting and false northing are defined
/// - EPSG Code: `8822`
/// - Aliases: "longitude of false origin", "longitude of natural origin", "longitude of origin", "central_meridian", "long0"
pub const LONGITUDE_OF_FALSE_ORIGIN: u32 = 8822;

// #define EPSG_NAME_PARAMETER_COLATITUDE_CONE_AXIS "Co-latitude of cone axis"
// #define EPSG_CODE_PARAMETER_COLATITUDE_CONE_AXIS 1036

// #define EPSG_NAME_PARAMETER_LATITUDE_PROJECTION_CENTRE                         \
//     "Latitude of projection centre"
// #define EPSG_CODE_PARAMETER_LATITUDE_PROJECTION_CENTRE 8811

// #define EPSG_NAME_PARAMETER_LONGITUDE_PROJECTION_CENTRE                        \
//     "Longitude of projection centre"
// #define EPSG_CODE_PARAMETER_LONGITUDE_PROJECTION_CENTRE 8812

// // Before EPSG 11.015
// #define EPSG_NAME_PARAMETER_AZIMUTH_INITIAL_LINE "Azimuth of initial line"
// #define EPSG_CODE_PARAMETER_AZIMUTH_INITIAL_LINE 8813

// // Since EPSG 11.015
// #define EPSG_NAME_PARAMETER_AZIMUTH_PROJECTION_CENTRE                          \
//     "Azimuth at projection centre"
// #define EPSG_CODE_PARAMETER_AZIMUTH_PROJECTION_CENTRE 8813

// #define EPSG_NAME_PARAMETER_ANGLE_RECTIFIED_TO_SKEW_GRID                       \
//     "Angle from Rectified to Skew Grid"
// #define EPSG_CODE_PARAMETER_ANGLE_RECTIFIED_TO_SKEW_GRID 8814

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

// #define EPSG_NAME_PARAMETER_LATITUDE_1ST_STD_PARALLEL                          \
//     "Latitude of 1st standard parallel"
// #define EPSG_CODE_PARAMETER_LATITUDE_1ST_STD_PARALLEL 8823

// #define EPSG_NAME_PARAMETER_LATITUDE_2ND_STD_PARALLEL                          \
//     "Latitude of 2nd standard parallel"
// #define EPSG_CODE_PARAMETER_LATITUDE_2ND_STD_PARALLEL 8824

// #define EPSG_NAME_PARAMETER_EASTING_FALSE_ORIGIN "Easting at false origin"
// #define EPSG_CODE_PARAMETER_EASTING_FALSE_ORIGIN 8826

// #define EPSG_NAME_PARAMETER_NORTHING_FALSE_ORIGIN "Northing at false origin"
// #define EPSG_CODE_PARAMETER_NORTHING_FALSE_ORIGIN 8827

// #define EPSG_NAME_PARAMETER_LATITUDE_STD_PARALLEL                              \
//     "Latitude of standard parallel"
// #define EPSG_CODE_PARAMETER_LATITUDE_STD_PARALLEL 8832

// #define EPSG_NAME_PARAMETER_LONGITUDE_OF_ORIGIN "Longitude of origin"
// #define EPSG_CODE_PARAMETER_LONGITUDE_OF_ORIGIN 8833

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
