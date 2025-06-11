// import {
//   CLARKE_FT_TO_M,
//   D2R,
//   FT_TO_M,
//   GRD2R,
//   INDIAN_FT_TO_M,
//   LINK_FT_TO_M,
//   MIN2R,
//   SEC2R,
//   Transformer,
//   US_FT_TO_M,
//   US_MOD_FT_TO_M,
// } from '../../proj4/index.js';
use crate::proj::{
    CLARKE_FT_TO_M, FT_TO_M, GRD2R, INDIAN_FT_TO_M, LINK_FT_TO_M, MIN2R, SEC2R, US_FT_TO_M,
    US_MOD_FT_TO_M,
};
use alloc::format;
use std::string::String;
// import type { DatumParams } from '../index.js';
// import type { GeoKeyDirectory, GridReader } from './index.js';
// import type { ProjectionParams, ProjectionTransformDefinition } from '../../proj4/index.js';

// /**
//  * Builds the projection transformer for a GeoTIFF image
//  * @param geoKeys - the geo-keys pulled from the image metadata
//  * @param definitions - an array of projection definitions for the transformer if needed
//  * @param epsgCodes - a record of EPSG codes to use for the transformer if needed
//  * @param gridStore - the grid readers
//  * @returns - the projection transformer
//  */
// export function buildTransformFromGeoKeys(
//   geoKeys?: GeoKeyDirectory,
//   definitions: ProjectionTransformDefinition[] = [],
//   epsgCodes: Record<string, string> = {},
//   gridStore: GridReader[] = [],
// ): Transformer {
//   const params = buildParamsFromGeoKeys(geoKeys);
//   if (geoKeys === undefined || params === undefined) return new Transformer();
//   return new Transformer(params, undefined, definitions, epsgCodes, gridStore);
// }

// /**
//  * NOTE: This project assumes ProjectionGeoKey and ProjectedCSTypeGeoKey will always be
//  *       set to 32767. This means the projection is user defined. Other mappings are waaay
//  *       too complicated.
//  * @param geoKeys - the geo-keys pulled from the image metadata
//  * @returns - the projection parameters. If nothing is returned a lon-lat system is already in place
//  */
// export function buildParamsFromGeoKeys(geoKeys?: GeoKeyDirectory): ProjectionParams | undefined {
//   const {
//     GTModelTypeGeoKey, // http://geotiff.maptools.org/spec/geotiff6.html#6.3.1.1
//     // GTRasterTypeGeoKey, // http://geotiff.maptools.org/spec/geotiff6.html#6.3.1.2 (used at a higher level)
//     GTCitationGeoKey,
//     // GeographicTypeGeoKey, // http://geotiff.maptools.org/spec/geotiff6.html#6.3.2.1
//     GeogCitationGeoKey,
//     // Unimplemented: a DATUM based lookup system
//     // GeogGeodeticDatumGeoKey, // http://geotiff.maptools.org/spec/geotiff6.html#6.3.2.2
//     GeogPrimeMeridianGeoKey,
//     GeogLinearUnitsGeoKey,
//     // GeogLinearUnitSizeGeoKey,
//     GeogAngularUnitsGeoKey,
//     // GeogAngularUnitSizeGeoKey,
//     GeogEllipsoidGeoKey,
//     GeogSemiMajorAxisGeoKey,
//     GeogSemiMinorAxisGeoKey,
//     GeogInvFlatteningGeoKey,
//     GeogAzimuthUnitsGeoKey,
//     GeogPrimeMeridianLongGeoKey,
//     GeogTOWGS84GeoKey,
//     ProjectedCSTypeGeoKey,
//     // PCSCitationGeoKey, -> documentation flag (unused)
//     ProjectionGeoKey,
//     ProjCoordTransGeoKey,
//     ProjLinearUnitsGeoKey,
//     // ProjLinearUnitSizeGeoKey,
//     ProjStdParallel1GeoKey,
//     ProjStdParallel2GeoKey,
//     ProjNatOriginLongGeoKey,
//     ProjNatOriginLatGeoKey,
//     ProjFalseEastingGeoKey,
//     ProjFalseNorthingGeoKey,
//     ProjFalseOriginLongGeoKey,
//     ProjFalseOriginLatGeoKey,
//     ProjFalseOriginEastingGeoKey,
//     ProjFalseOriginNorthingGeoKey,
//     ProjCenterLongGeoKey,
//     ProjCenterLatGeoKey,
//     ProjCenterEastingGeoKey,
//     ProjCenterNorthingGeoKey,
//     ProjScaleAtNatOriginGeoKey,
//     ProjScaleAtCenterGeoKey,
//     ProjAzimuthAngleGeoKey,
//     ProjStraightVertPoleLongGeoKey,
//     ProjRectifiedGridAngleGeoKey,
//     VerticalCSTypeGeoKey,
//     // VerticalCitationGeoKey, - documentation flag (unused)
//     // VerticalDatumGeoKey, - this was never implemented by geotiff
//     VerticalUnitsGeoKey,
//   } = geoKeys ?? {};

//   if (GTModelTypeGeoKey === 2) return undefined; // already in WGS84

//   const toMeter = build_to_meter(
//     ProjLinearUnitsGeoKey ?? VerticalUnitsGeoKey ?? GeogLinearUnitsGeoKey,
//   );

//   return {
//     name: GTCitationGeoKey,
//     projName:
//       from_epsg_key(ProjectedCSTypeGeoKey ?? VerticalCSTypeGeoKey ?? ProjectionGeoKey) ??
//       build_proj_name(ProjCoordTransGeoKey),
//     datumCode: GeogCitationGeoKey,
//     ellps: build_ellps(GeogEllipsoidGeoKey),
//     a: GeogSemiMajorAxisGeoKey,
//     b: GeogSemiMinorAxisGeoKey,
//     rf: GeogInvFlatteningGeoKey,
//     alpha: GeogAzimuthUnitsGeoKey ?? get_angle(ProjAzimuthAngleGeoKey, GeogAngularUnitsGeoKey),
//     x0:
//       (ProjFalseEastingGeoKey ?? ProjFalseOriginEastingGeoKey ?? ProjCenterEastingGeoKey ?? 0) *
//       (toMeter ?? 1),
//     y0:
//       (ProjFalseNorthingGeoKey ?? ProjFalseOriginNorthingGeoKey ?? ProjCenterNorthingGeoKey ?? 0) *
//       (toMeter ?? 1),
//     lat0: (ProjStdParallel1GeoKey ?? ProjNatOriginLatGeoKey ?? ProjFalseOriginLatGeoKey ?? 0) * D2R,
//     lat1: (ProjStdParallel1GeoKey ?? 0) * D2R,
//     lat2: (ProjStdParallel2GeoKey ?? 0) * D2R,
//     long0:
//       (ProjFalseOriginLongGeoKey ??
//         ProjNatOriginLongGeoKey ??
//         ProjStraightVertPoleLongGeoKey ??
//         0) * D2R,
//     longc: (ProjCenterLongGeoKey ?? 0) * D2R,
//     lamc: (ProjCenterLatGeoKey ?? 0) * D2R,
//     datumParams: GeogTOWGS84GeoKey as DatumParams | undefined,
//     fromGreenwich: (GeogPrimeMeridianLongGeoKey ?? GeogPrimeMeridianGeoKey ?? 0) * D2R,
//     // NOTE: There might be other cases that cause sphere to be true
//     sphere: GTModelTypeGeoKey === 3 || ProjCoordTransGeoKey === 6 ? true : false,
//     k0: ProjScaleAtNatOriginGeoKey ?? ProjScaleAtCenterGeoKey,
//     rectifiedGridAngle: ProjRectifiedGridAngleGeoKey,
//     toMeter,
//   };
// }

/// Given an EPSG key, return the corresponding proj4 string
/// http://geotiff.maptools.org/spec/geotiff6.html#6.3.2.1
/// http://geotiff.maptools.org/spec/geotiff6.html#6.3.3.1
///
/// @param id - the EPSG code
/// @returns - the proj4 string
fn from_epsg_key(id: Option<usize>) -> Option<String> {
    // 32767 is a user-defined code
    if let Some(id) = id {
        if id == 32767 {
            return None;
        }
        Some(format!("EPSG_{}", id))
    } else {
        None
    }
}

/// Convert angle to Radians
/// http://geotiff.maptools.org/spec/geotiff6.html#6.3.1.4
///
/// @param angle - the angle
/// @param units - the unit type of the angle
/// @returns - the angle in radians
fn get_angle(angle: Option<f64>, units: Option<usize>) -> Option<f64> {
    match units.unwrap_or_default() {
        9001 => angle,
        9002 => angle.map(|a| a.to_radians()),
        9003 => angle.map(|a| a * MIN2R),
        9004 => angle.map(|a| a * SEC2R),
        9005 | 9006 => angle.map(|a| a * GRD2R),
        _ => None,
    }
    // Note: unsupported:
    // Angular_DMS =	9107
    // Angular_DMS_Hemisphere =	9108
}

/// Convert geotiff key to proj4 name
/// http://geotiff.maptools.org/spec/geotiff6.html#6.3.3.3
///
/// @param key - the geotiff key
/// @returns - the proj4 name
fn build_proj_name(key: Option<usize>) -> Option<String> {
    match key.unwrap_or_default() {
        1 => Some("tmerc".into()),  // Transverse_Mercator, GaussBoaga; GaussKruger
        2 => Some("etmerc".into()), // TransvMercator_Modified_Alaska, AlaskaConformal
        3 => Some("omerc".into()),  // ObliqueMercator, ObliqueMercator_Hotine
        4 => Some("omerc".into()),  // ObliqueMercator_Laborde
        5 => Some("omerc".into()),  // ObliqueMercator_Rosenmund
        6 => Some("omerc".into()),  // ObliqueMercator_Spherical
        7 => Some("merc".into()),   // Mercator
        8 => Some("lcc".into()),    // LambertConfConic_2SP
        9 => Some("lcc".into()),    // LambertConfConic_Helmert
        10 => Some("laea".into()),  // LambertAzimEqualArea
        11 => Some("aea".into()),   // AlbersEqualArea
        12 => Some("aeqd".into()),  // AzimuthalEquidistant
        13 => Some("eqdc".into()),  // EquidistantConic
        14 => Some("stere".into()), // Stereographic
        15 => Some("stere".into()), // PolarStereographic
        16 => Some("sterea".into()), // ObliqueStereographic
        17 => Some("eqc".into()),   // Equirectangular
        18 => Some("cass".into()),  // CassiniSoldner, TransvEquidistCylindrical
        19 => Some("gnom".into()),  // Gnomonic
        20 => Some("mill".into()),  // MillerCylindrical
        21 => Some("ortho".into()), // Orthographic
        22 => Some("poly".into()),  // Polyconic
        23 => Some("robin".into()), // Robinson
        24 => Some("sinu".into()),  // Sinusoidal
        25 => Some("vandg".into()), // VanDerGrinten
        26 => Some("nzmg".into()),  // NewZealandMapGrid
        27 => Some("etmerc".into()), // TransvMercator_SouthOriented
        _ => None,
    }
}

/// Convert geotiff key to proj4 ellipsoid
///
/// @param key - the geotiff key
/// @returns - the proj4 ellipsoid
fn build_ellps(key: Option<usize>) -> Option<String> {
    match key.unwrap_or_default() {
        7001 => Some("airy".into()),
        7002 => Some("mod_airy".into()),
        7003 => Some("aust_SA".into()),
        7004 => Some("bessel".into()),
        7005 => Some("mod_bessel".into()),
        7006 => Some("bess_nam".into()),
        7007 => Some("clrk58".into()),
        7008 => Some("clrk66".into()),
        7009 => Some("clrk80mich".into()),
        7010 => Some("clrk80ben".into()),
        7011 => Some("clrk80ign".into()),
        7012 => Some("clrk80rgs".into()),
        7013 => Some("clrk80arc".into()),
        7014 => Some("clrk80sga".into()),
        7015 => Some("evrst30".into()),
        7016 => Some("evrstSS".into()),
        7017 => Some("evrst75".into()),
        7018 => Some("evrst30_mod".into()),
        7019 => Some("GRS80".into()),
        7020 => Some("helmert".into()),
        7021 => Some("indonesian".into()),
        7022 => Some("intl24".into()),
        7023 => Some("intl67".into()),
        7024 => Some("krass".into()),
        7025 => Some("NWL9D".into()),
        7026 => Some("NWL10D".into()),
        7027 => Some("plessis".into()),
        7028 => Some("struve".into()),
        7029 => Some("WARO".into()),
        7030 => Some("WGS84".into()),
        7031 => Some("GEM10C".into()),
        7032 => Some("OSU86F".into()),
        7033 => Some("OSU91A".into()),
        7034 => Some("clrk80".into()),
        7035 => Some("SPHERE".into()),
        _ => None,
    }
}

/// Convert geotiff meter key to proj4 toMeter format
/// http://geotiff.maptools.org/spec/geotiff6.html#6.3.1.3
///
/// @param key - the geotiff key
/// @returns - the proj4 toMeter
pub fn geotiff_to_meter(key: Option<usize>) -> Option<f64> {
    match key.unwrap_or_default() {
        9001 => Some(1.0),            // Linear_Meter
        9002 => Some(FT_TO_M),        // Linear_Foot
        9003 => Some(US_FT_TO_M),     // Linear_Foot_US_Survey
        9004 => Some(US_MOD_FT_TO_M), // Linear_Foot_Modified_American
        9005 => Some(CLARKE_FT_TO_M), // Linear_Foot_Clarke
        9006 => Some(INDIAN_FT_TO_M), // Linear_Foot_Indian
        9007 => Some(LINK_FT_TO_M),   // Linear_Foot_Link
        _ => None,
    }
    // others are unsupported
    // Linear_Link_Benoit =	9008
    // Linear_Link_Sears =	9009
    // Linear_Chain_Benoit =	9010
    // Linear_Chain_Sears =	9011
    // Linear_Yard_Sears =	9012
    // Linear_Yard_Indian =	9013
    // Linear_Fathom =	9014
    // Linear_Mile_International_Nautical =	9015
}
