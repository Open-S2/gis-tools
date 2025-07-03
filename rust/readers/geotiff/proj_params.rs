use crate::{
    proj::{
        ANGLE_RECTIFIED_TO_SKEW_GRID, AZIMUTH_PROJECTION_CENTRE, CLARKE_FT_TO_M, FALSE_EASTING,
        FALSE_NORTHING, FT_TO_M, GRD2R, INDIAN_FT_TO_M, INDIAN_YD_TO_M, INTERNATIONAL_FATHOM_TO_M,
        INTERNATIONAL_NAUTICAL_MILE_TO_M, LATITUDE_OF_FALSE_ORIGIN,
        LATITUDE_OF_FIRST_STANDARD_PARALLEL, LATITUDE_OF_NATURAL_ORIGIN,
        LATITUDE_OF_PROJECTION_CENTRE, LATITUDE_OF_SECOND_STANDARD_PARALLEL, LINK_FT_TO_M,
        LONGITUDE_OF_FALSE_ORIGIN, LONGITUDE_OF_NATURAL_ORIGIN, LONGITUDE_OF_ORIGIN,
        LONGITUDE_OF_PROJECTION_CENTRE, MIN2R, Proj, ProjJSON, ProjectionTransform,
        SCALE_FACTOR_AT_NATURAL_ORIGIN, SEC2R, Step, Transformer, US_FT_TO_M, US_MOD_FT_TO_M,
        derive_eccentricity, derive_sphere,
    },
    readers::{GeoKeyDirectoryKeys as GKD, GeoStore},
};
use alloc::string::{String, ToString};

// UNUSED KEYS:
// GTRasterTypeGeoKey, // http://geotiff.maptools.org/spec/geotiff6.html#6.3.1.2 (used at a higher level)
// GeographicTypeGeoKey, // http://geotiff.maptools.org/spec/geotiff6.html#6.3.2.1
// Unimplemented: a DATUM based lookup system
// GeogGeodeticDatumGeoKey, // http://geotiff.maptools.org/spec/geotiff6.html#6.3.2.2
// GeogLinearUnitSizeGeoKey,
// GeogAngularUnitSizeGeoKey,
// PCSCitationGeoKey, -> documentation flag (unused)
// ProjLinearUnitSizeGeoKey,
// VerticalCitationGeoKey, - documentation flag (unused)
// VerticalDatumGeoKey, - this was never implemented by geotiff
// GeogTOWGS84GeoKey
// GeogCitationGeoKey

/// Builds the projection transformer for a GeoTIFF image
///
/// @param transformer - the transformer to update on the source projection data
/// @param geo_keys - the geo-keys pulled from the image metadata
/// @returns - the projection parameters. If nothing is returned a lon-lat system is already in place
pub fn build_transform_from_geo_keys(transformer: &mut Transformer, store: &GeoStore) {
    // http://geotiff.maptools.org/spec/geotiff6.html#6.3.1.1
    let gtmodel_type_geo_key = store.get_short(GKD::GTModelTypeGeoKey as u16).unwrap_or_default();
    if gtmodel_type_geo_key == 2 {
        return; // already in WGS84
    }

    let proj_coord_trans_geo_key =
        store.get_short(GKD::ProjCoordTransGeoKey as u16).unwrap_or_default();
    let name = store.get_string(GKD::GTCitationGeoKey as u16);
    let epsg_key = from_epsg_key(
        store
            .get_short(GKD::ProjectedCSTypeGeoKey as u16)
            .or(store.get_short(GKD::VerticalCSTypeGeoKey as u16))
            .or(store.get_short(GKD::ProjectionGeoKey as u16)),
    );
    let proj_name = build_proj_name(store.get_short(GKD::ProjCoordTransGeoKey as u16));

    let mut proj_definition: ProjectionTransform;
    if let Some(epsg_key) = epsg_key {
        if let Some(epsg_wkt) = transformer.get_epsg_code(epsg_key.clone()) {
            proj_definition = ProjJSON::parse_wkt(&epsg_wkt).to_projection_transform();
        } else {
            panic!("Unable to find EPSG code for {}", epsg_key);
        }
    } else if proj_name.is_none() {
        // There is no projection to be made
        return;
    } else {
        proj_definition = ProjectionTransform::default();
    }

    // Handle params
    {
        let proj = &mut proj_definition.proj.borrow_mut();
        if let Some(name) = name.or(proj_name.clone()) {
            proj.name = name;
        }

        // from_greenwich
        if let Some(val) = store
            .get_double(GKD::GeogPrimeMeridianLongGeoKey as u16)
            .or_else(|| store.get_short(GKD::GeogPrimeMeridianGeoKey as u16).map(|v| v as f64))
        {
            proj.from_greenwich = val.to_radians();
        }
        // sphere - NOTE: There might be other cases that cause sphere to be true
        proj.sphere = gtmodel_type_geo_key == 3 || proj_coord_trans_geo_key == 6;
        // TODO: Datum - It's a string map -_-
        // proj.datum_type = store
        //     .get_short(GKD::GeogCitationGeoKey as u16)
        //     .unwrap_or_default()
        //     .into();
        // Ellipse
        if let Some(code) = store.get_short(GKD::GeogEllipsoidGeoKey as u16) {
            proj.ellps = build_ellps(Some(code));
        }
        if let Some(a) = store.get_double(GKD::GeogSemiMajorAxisGeoKey as u16) {
            proj.a = a;
        }
        if let Some(b) = store.get_double(GKD::GeogSemiMinorAxisGeoKey as u16) {
            proj.b = b;
        }
        if let Some(rf) = store.get_double(GKD::GeogInvFlatteningGeoKey as u16) {
            proj.rf = rf;
        }
        derive_sphere(proj);
        derive_eccentricity(proj);
        // to meter
        let proj_linear_units_geo_key = store.get_short(GKD::ProjLinearUnitsGeoKey as u16);
        let vertical_units_geo_key = store.get_short(GKD::VerticalUnitsGeoKey as u16);
        let geog_linear_units_geo_key = store.get_short(GKD::GeogLinearUnitsGeoKey as u16);
        if proj.to_meter == 1. {
            let to_meter = geotiff_to_meter(
                proj_linear_units_geo_key.or(vertical_units_geo_key).or(geog_linear_units_geo_key),
            );
            proj.to_meter = to_meter;
        }
        // alpha
        let alpha_angle = store.get_double(GKD::ProjAzimuthAngleGeoKey as u16);
        let alpha_units = store
            .get_short(GKD::GeogAzimuthUnitsGeoKey as u16)
            .or(store.get_short(GKD::GeogAngularUnitsGeoKey as u16));
        if let Some(alpha) = get_angle(alpha_angle, alpha_units) {
            proj.alpha = alpha;
            proj.set_f64(AZIMUTH_PROJECTION_CENTRE, alpha);
        }
        if let Some(false_east) = store.get_double(GKD::ProjFalseOriginEastingGeoKey as u16) {
            proj.set_f64(LONGITUDE_OF_FALSE_ORIGIN, false_east.to_radians());
            proj.lam0 = false_east.to_radians();
        }
        if let Some(false_east) = store.get_double(GKD::ProjCenterEastingGeoKey as u16) {
            proj.set_f64(LONGITUDE_OF_FALSE_ORIGIN, false_east.to_radians());
            proj.lam0 = false_east.to_radians();
        }
        // long0
        let long0 = store
            .get_double(GKD::ProjFalseOriginLongGeoKey as u16)
            .or(store.get_double(GKD::ProjNatOriginLongGeoKey as u16))
            .or(store.get_double(GKD::ProjStraightVertPoleLongGeoKey as u16));
        if let Some(long0) = long0 {
            proj.lam0 = long0.to_radians();
        }
        // x0
        let x0 = store
            .get_double(GKD::ProjFalseEastingGeoKey as u16)
            .or(store.get_double(GKD::ProjFalseOriginEastingGeoKey as u16))
            .or(store.get_double(GKD::ProjCenterEastingGeoKey as u16));
        if let Some(x0) = x0 {
            proj.x0 = x0 * proj.to_meter;
        }
        // LATITUDE_OF_FALSE_ORIGIN
        if let Some(false_north) = store.get_double(GKD::ProjFalseOriginNorthingGeoKey as u16) {
            proj.set_f64(LATITUDE_OF_FALSE_ORIGIN, false_north.to_radians());
            proj.phi0 = false_north.to_radians();
        }
        if let Some(false_north) = store.get_double(GKD::ProjFalseOriginLatGeoKey as u16) {
            proj.set_f64(LATITUDE_OF_FALSE_ORIGIN, false_north.to_radians());
            proj.phi0 = false_north.to_radians();
        }
        // y0
        let y0 = store
            .get_double(GKD::ProjFalseNorthingGeoKey as u16)
            .or(store.get_double(GKD::ProjFalseOriginNorthingGeoKey as u16))
            .or(store.get_double(GKD::ProjCenterNorthingGeoKey as u16));
        if let Some(y0) = y0 {
            proj.y0 = y0 * proj.to_meter;
        }
        // k0
        let k0 = store
            .get_double(GKD::ProjScaleAtNatOriginGeoKey as u16)
            .or(store.get_double(GKD::ProjScaleAtCenterGeoKey as u16));
        if let Some(k0) = k0 {
            proj.k0 = k0;
            proj.set_f64(SCALE_FACTOR_AT_NATURAL_ORIGIN, k0);
        }
        // LAT
        let lat0 = store
            .get_double(GKD::ProjStdParallel1GeoKey as u16)
            .or(store.get_double(GKD::ProjNatOriginLatGeoKey as u16))
            .or(store.get_double(GKD::ProjFalseOriginLatGeoKey as u16));
        if let Some(lat0) = lat0 {
            proj.phi0 = lat0.to_radians();
        }
        // set params
        set_angle(proj, store, GKD::ProjFalseEastingGeoKey as u16, FALSE_EASTING);
        set_angle(proj, store, GKD::ProjFalseNorthingGeoKey as u16, FALSE_NORTHING);
        set_angle(proj, store, GKD::ProjNatOriginLatGeoKey as u16, LATITUDE_OF_NATURAL_ORIGIN);
        set_angle(
            proj,
            store,
            GKD::ProjStdParallel1GeoKey as u16,
            LATITUDE_OF_FIRST_STANDARD_PARALLEL,
        );
        set_angle(
            proj,
            store,
            GKD::ProjStdParallel2GeoKey as u16,
            LATITUDE_OF_SECOND_STANDARD_PARALLEL,
        );
        set_angle(
            proj,
            store,
            GKD::ProjRectifiedGridAngleGeoKey as u16,
            ANGLE_RECTIFIED_TO_SKEW_GRID,
        );
        set_angle(proj, store, GKD::ProjCenterLongGeoKey as u16, LONGITUDE_OF_PROJECTION_CENTRE);
        set_angle(proj, store, GKD::ProjCenterLatGeoKey as u16, LATITUDE_OF_PROJECTION_CENTRE);
        set_angle(proj, store, GKD::ProjFalseOriginLongGeoKey as u16, LONGITUDE_OF_FALSE_ORIGIN);
        set_angle(proj, store, GKD::ProjNatOriginLongGeoKey as u16, LONGITUDE_OF_NATURAL_ORIGIN);
        set_angle(proj, store, GKD::ProjStraightVertPoleLongGeoKey as u16, LONGITUDE_OF_ORIGIN);
    }

    // lastly handle injecting projection if it exists
    if let Some(proj_name) = &proj_name {
        if let Some(step) = Step::from_name(proj_name, proj_definition.proj.clone()) {
            proj_definition.method = step;
        }
    }

    transformer.set_source_def(proj_definition);
}

fn set_angle(proj: &mut Proj, store: &GeoStore, geo_key: u16, proj_key: i64) {
    if let Some(val) = store.get_double(geo_key) {
        proj.set_f64(proj_key, val.to_radians());
    }
}

/// Given an EPSG key, return the corresponding proj4 string
/// http://geotiff.maptools.org/spec/geotiff6.html#6.3.2.1
/// http://geotiff.maptools.org/spec/geotiff6.html#6.3.3.1
///
/// @param id - the EPSG code
/// @returns - the proj4 string
fn from_epsg_key(id: Option<i16>) -> Option<String> {
    // 32767 is a user-defined code
    if let Some(id) = id {
        if id == 32767 {
            return None;
        }
        Some(id.to_string())
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
fn get_angle(angle: Option<f64>, units: Option<i16>) -> Option<f64> {
    match units.unwrap_or_default() {
        9001 => angle,
        9002 => angle.map(|a| a.to_radians()),
        9003 => angle.map(|a| a * MIN2R),
        9004 => angle.map(|a| a * SEC2R),
        9005 | 9006 => angle.map(|a| a * GRD2R),
        _ => None,
    }
    // Note: unsupported:
    // Angular_Radian =	9101
    // Angular_Degree =	9102
    // Angular_Arc_Minute =	9103
    // Angular_Arc_Second =	9104
    // Angular_Grad =	9105
    // Angular_Gon =	9106
    // Angular_DMS =	9107
    // Angular_DMS_Hemisphere =	9108
}

/// Convert geotiff key to proj4 name
/// http://geotiff.maptools.org/spec/geotiff6.html#6.3.3.3
///
/// @param key - the geotiff key
/// @returns - the proj4 name
fn build_proj_name(key: Option<i16>) -> Option<String> {
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
fn build_ellps(key: Option<i16>) -> String {
    match key.unwrap_or_default() {
        7001 => "airy".into(),
        7002 => "mod_airy".into(),
        7003 => "aust_SA".into(),
        7004 => "bessel".into(),
        7005 => "mod_bessel".into(),
        7006 => "bess_nam".into(),
        7007 => "clrk58".into(),
        7008 => "clrk66".into(),
        7009 => "clrk80mich".into(),
        7010 => "clrk80ben".into(),
        7011 => "clrk80ign".into(),
        7012 => "clrk80rgs".into(),
        7013 => "clrk80arc".into(),
        7014 => "clrk80sga".into(),
        7015 => "evrst30".into(),
        7016 => "evrstSS".into(),
        7017 => "evrst75".into(),
        7018 => "evrst30_mod".into(),
        7019 => "GRS80".into(),
        7020 => "helmert".into(),
        7021 => "indonesian".into(),
        7022 => "intl24".into(),
        7023 => "intl67".into(),
        7024 => "krass".into(),
        7025 => "NWL9D".into(),
        7026 => "NWL10D".into(),
        7027 => "plessis".into(),
        7028 => "struve".into(),
        7029 => "WARO".into(),
        7030 => "WGS84".into(),
        7031 => "GEM10C".into(),
        7032 => "OSU86F".into(),
        7033 => "OSU91A".into(),
        7034 => "clrk80".into(),
        7035 => "SPHERE".into(),
        _ => "".into(),
    }
}

/// Convert geotiff meter key to proj4 to_meter format
/// http://geotiff.maptools.org/spec/geotiff6.html#6.3.1.3
///
/// @param key - the geotiff key
/// @returns - the proj4 to_meter
pub fn geotiff_to_meter(key: Option<i16>) -> f64 {
    match key.unwrap_or_default() {
        9001 => 1.0,                              // Linear_Meter
        9002 => FT_TO_M,                          // Linear_Foot
        9003 => US_FT_TO_M,                       // Linear_Foot_US_Survey
        9004 => US_MOD_FT_TO_M,                   // Linear_Foot_Modified_American
        9005 => CLARKE_FT_TO_M,                   // Linear_Foot_Clarke
        9006 => INDIAN_FT_TO_M,                   // Linear_Foot_Indian
        9007 => LINK_FT_TO_M,                     // Linear_Foot_Link
        9013 => INDIAN_YD_TO_M,                   // Linear_Yard_Indian
        9014 => INTERNATIONAL_FATHOM_TO_M,        // Linear_Fathom
        9015 => INTERNATIONAL_NAUTICAL_MILE_TO_M, // Linear_Mile_International_Nautical
        _ => 1.0,
    }
    // others are unsupported
    // Linear_Link_Benoit =	9008
    // Linear_Link_Sears = 9009
    // Linear_Chain_Benoit = 9010
    // Linear_Chain_Sears =	9011
    // Linear_Yard_Sears = 9012
}
