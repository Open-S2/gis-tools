import {
  D2R,
  FT_TO_M,
  GRD2R,
  MIN2R,
  SEC2R,
  Transformer,
  US_FT_TO_M,
  US_MOD_FT_TO_M,
} from '../../proj4';

import type { DatumParams } from '..';
import type { GeoKeyDirectory, GridReader } from '.';
import type { ProjectionParams, ProjectionTransformDefinition } from '../../proj4';

/**
 * Builds the projection transformer for a GeoTIFF image
 * @param geoKeys - the geo-keys pulled from the image metadata
 * @param gridStore - the grid readers
 * @param definitions - an array of projection definitions for the transformer if needed
 * @param epsgCodes - a record of EPSG codes to use for the transformer if needed
 * @returns - the projection transformer
 */
export function buildTransform(
  geoKeys?: GeoKeyDirectory,
  gridStore: GridReader[] = [],
  definitions: ProjectionTransformDefinition[] = [],
  epsgCodes: Record<string, string> = {},
): Transformer {
  const params = buildParams(geoKeys);
  const transformer = new Transformer();
  for (const proj of definitions) transformer.insertDefinition(proj);
  for (const [key, value] of Object.entries(epsgCodes)) transformer.insertEPSGCode(key, value);
  for (const { key, reader } of gridStore) transformer.addGridFromReader(key, reader);
  if (geoKeys === undefined) return transformer;
  if (params !== undefined) transformer.setSource(params);
  return transformer;
}

/**
 * NOTE: This project assumes ProjectionGeoKey and ProjectedCSTypeGeoKey will always be
 *       set to 32767. This means the projection is user defined. Other mappings are waaay
 *       too complicated.
 * @param geoKeys - the geo-keys pulled from the image metadata
 * @returns - the projection parameters. If nothing is returned a lon-lat system is already in place
 */
function buildParams(geoKeys?: GeoKeyDirectory): ProjectionParams | undefined {
  const {
    GTModelTypeGeoKey, // http://geotiff.maptools.org/spec/geotiff6.html#6.3.1.1
    // GTRasterTypeGeoKey, // http://geotiff.maptools.org/spec/geotiff6.html#6.3.1.2 (used at a higher level)
    GTCitationGeoKey,
    // GeographicTypeGeoKey, // http://geotiff.maptools.org/spec/geotiff6.html#6.3.2.1
    GeogCitationGeoKey,
    // Unimplemented: a DATUM based lookup system
    // GeogGeodeticDatumGeoKey, // http://geotiff.maptools.org/spec/geotiff6.html#6.3.2.2
    GeogPrimeMeridianGeoKey,
    GeogLinearUnitsGeoKey,
    // GeogLinearUnitSizeGeoKey,
    GeogAngularUnitsGeoKey,
    // GeogAngularUnitSizeGeoKey,
    GeogEllipsoidGeoKey,
    GeogSemiMajorAxisGeoKey,
    GeogSemiMinorAxisGeoKey,
    GeogInvFlatteningGeoKey,
    GeogAzimuthUnitsGeoKey,
    GeogPrimeMeridianLongGeoKey,
    GeogTOWGS84GeoKey,
    ProjectedCSTypeGeoKey,
    // PCSCitationGeoKey, -> documentation flag (unused)
    ProjectionGeoKey,
    ProjCoordTransGeoKey,
    ProjLinearUnitsGeoKey,
    // ProjLinearUnitSizeGeoKey,
    ProjStdParallel1GeoKey,
    ProjStdParallel2GeoKey,
    ProjNatOriginLongGeoKey,
    ProjNatOriginLatGeoKey,
    ProjFalseEastingGeoKey,
    ProjFalseNorthingGeoKey,
    ProjFalseOriginLongGeoKey,
    ProjFalseOriginLatGeoKey,
    ProjFalseOriginEastingGeoKey,
    ProjFalseOriginNorthingGeoKey,
    ProjCenterLongGeoKey,
    ProjCenterLatGeoKey,
    ProjCenterEastingGeoKey,
    ProjCenterNorthingGeoKey,
    ProjScaleAtNatOriginGeoKey,
    ProjScaleAtCenterGeoKey,
    ProjAzimuthAngleGeoKey,
    ProjStraightVertPoleLongGeoKey,
    ProjRectifiedGridAngleGeoKey,
    VerticalCSTypeGeoKey,
    // VerticalCitationGeoKey, - documentation flag (unused)
    // VerticalDatumGeoKey, - this was never implemented by geotiff
    VerticalUnitsGeoKey,
  } = geoKeys ?? {};

  if (GTModelTypeGeoKey === 2) return undefined; // already in WGS84

  const toMeter = buildToMeter(
    ProjLinearUnitsGeoKey ?? VerticalUnitsGeoKey ?? GeogLinearUnitsGeoKey,
  );

  return {
    name: GTCitationGeoKey,
    projName:
      fromEPSGKey(ProjectedCSTypeGeoKey ?? VerticalCSTypeGeoKey ?? ProjectionGeoKey) ??
      buildProjName(ProjCoordTransGeoKey),
    datumCode: GeogCitationGeoKey,
    ellps: buildEllps(GeogEllipsoidGeoKey),
    a: GeogSemiMajorAxisGeoKey,
    b: GeogSemiMinorAxisGeoKey,
    rf: GeogInvFlatteningGeoKey,
    alpha: GeogAzimuthUnitsGeoKey ?? getAngle(ProjAzimuthAngleGeoKey, GeogAngularUnitsGeoKey),
    x0:
      (ProjFalseEastingGeoKey ?? ProjFalseOriginEastingGeoKey ?? ProjCenterEastingGeoKey ?? 0) *
      (toMeter ?? 1),
    y0:
      (ProjFalseNorthingGeoKey ?? ProjFalseOriginNorthingGeoKey ?? ProjCenterNorthingGeoKey ?? 0) *
      (toMeter ?? 1),
    lat0: (ProjStdParallel1GeoKey ?? ProjNatOriginLatGeoKey ?? ProjFalseOriginLatGeoKey ?? 0) * D2R,
    lat1: (ProjStdParallel1GeoKey ?? 0) * D2R,
    lat2: (ProjStdParallel2GeoKey ?? 0) * D2R,
    long0:
      (ProjFalseOriginLongGeoKey ??
        ProjNatOriginLongGeoKey ??
        ProjStraightVertPoleLongGeoKey ??
        0) * D2R,
    longc: (ProjCenterLongGeoKey ?? 0) * D2R,
    lamc: (ProjCenterLatGeoKey ?? 0) * D2R,
    datumParams: GeogTOWGS84GeoKey as DatumParams | undefined,
    fromGreenwich: (GeogPrimeMeridianLongGeoKey ?? GeogPrimeMeridianGeoKey ?? 0) * D2R,
    // NOTE: There might be other cases that cause sphere to be true
    sphere: GTModelTypeGeoKey === 3 || ProjCoordTransGeoKey === 6 ? true : false,
    k0: ProjScaleAtNatOriginGeoKey ?? ProjScaleAtCenterGeoKey,
    rectifiedGridAngle: ProjRectifiedGridAngleGeoKey,
    toMeter,
  };
}

/**
 * Given an EPSG key, return the corresponding proj4 string
 * http://geotiff.maptools.org/spec/geotiff6.html#6.3.2.1
 * http://geotiff.maptools.org/spec/geotiff6.html#6.3.3.1
 * @param id - the EPSG code
 * @returns - the proj4 string
 */
function fromEPSGKey(id?: number): string | undefined {
  // 32767 is a user-defined code
  if (id === undefined || id === 32767) return undefined;
  return `EPSG_${id}`;
}

/**
 * Convert angle to Radians
 * http://geotiff.maptools.org/spec/geotiff6.html#6.3.1.4
 * @param angle - the angle
 * @param units - the unit type of the angle
 * @returns - the angle in radians
 */
function getAngle(angle?: number, units?: number): undefined | number {
  if (angle === undefined) return undefined;
  if (units === 9001) {
    return angle;
  } else if (units === 9002) {
    return angle * D2R;
  } else if (units === 9003) {
    return angle * MIN2R;
  } else if (units === 9004) {
    return angle * SEC2R;
  } else if (units === 9005 || units === 9006) {
    // GRD2R and GON2R are the same
    return angle * GRD2R;
  }
  // Note: unsupported:
  // Angular_DMS =	9107
  // Angular_DMS_Hemisphere =	9108
  return undefined;
}

/**
 * Convert geotiff key to proj4 name
 * http://geotiff.maptools.org/spec/geotiff6.html#6.3.3.3
 * @param key - the geotiff key
 * @returns - the proj4 name
 */
function buildProjName(key?: number): string | undefined {
  if (key === 1)
    return 'tmerc'; // Transverse_Mercator, GaussBoaga; GaussKruger
  else if (key === 2)
    return 'etmerc'; // TransvMercator_Modified_Alaska, AlaskaConformal
  else if (key === 3)
    return 'omerc'; // ObliqueMercator, ObliqueMercator_Hotine
  else if (key === 4)
    return 'omerc'; // ObliqueMercator_Laborde
  else if (key === 5)
    return 'omerc'; // ObliqueMercator_Rosenmund
  else if (key === 6)
    return 'omerc'; // ObliqueMercator_Spherical
  else if (key === 7)
    return 'merc'; // Mercator
  else if (key === 8)
    return 'lcc'; // LambertConfConic_2SP
  else if (key === 9)
    return 'lcc'; // LambertConfConic_Helmert
  else if (key === 10)
    return 'laea'; // LambertAzimEqualArea
  else if (key === 11)
    return 'aea'; // AlbersEqualArea
  else if (key === 12)
    return 'aeqd'; // AzimuthalEquidistant
  else if (key === 13)
    return 'eqdc'; // EquidistantConic
  else if (key === 14)
    return 'stere'; // Stereographic
  else if (key === 15)
    return 'stere'; // PolarStereographic
  else if (key === 16)
    return 'sterea'; // ObliqueStereographic
  else if (key === 17)
    return 'eqc'; // Equirectangular
  else if (key === 18)
    return 'cass'; // CassiniSoldner, TransvEquidistCylindrical
  else if (key === 19)
    return 'gnom'; // Gnomonic
  else if (key === 20)
    return 'mill'; // MillerCylindrical
  else if (key === 21)
    return 'ortho'; // Orthographic
  else if (key === 22)
    return 'poly'; // Polyconic
  else if (key === 23)
    return 'robin'; // Robinson
  else if (key === 24)
    return 'sinu'; // Sinusoidal
  else if (key === 25)
    return 'vandg'; // VanDerGrinten
  else if (key === 26)
    return 'nzmg'; // NewZealandMapGrid
  else if (key === 27) return 'etmerc'; // TransvMercator_SouthOriented
  return undefined;
}

/**
 * Convert geotiff key to proj4 ellipsoid
 * @param key - the geotiff key
 * @returns - the proj4 ellipsoid
 */
function buildEllps(key?: number): string | undefined {
  if (key === 7001) return 'airy';
  else if (key === 7002) return 'mod_airy';
  else if (key === 7003) return 'aust_SA';
  else if (key === 7004) return 'bessel';
  else if (key === 7005) return 'mod_bessel';
  else if (key === 7006) return 'bess_nam';
  else if (key === 7007) return 'clrk58';
  else if (key === 7008) return 'clrk66';
  else if (key === 7009) return 'clrk80mich';
  else if (key === 7010) return 'clrk80ben';
  else if (key === 7011) return 'clrk80ign';
  else if (key === 7012) return 'clrk80rgs';
  else if (key === 7013) return 'clrk80arc';
  else if (key === 7014) return 'clrk80sga';
  else if (key === 7015) return 'evrst30';
  else if (key === 7016) return 'evrstSS';
  else if (key === 7017) return 'evrst75';
  else if (key === 7018) return 'evrst30_mod';
  else if (key === 7019) return 'GRS80';
  else if (key === 7020) return 'helmert';
  else if (key === 7021) return 'indonesian';
  else if (key === 7022) return 'intl24';
  else if (key === 7023) return 'intl67';
  else if (key === 7024) return 'krass';
  else if (key === 7025) return 'NWL9D';
  else if (key === 7026) return 'NWL10D';
  else if (key === 7027) return 'plessis';
  else if (key === 7028) return 'struve';
  else if (key === 7029) return 'WARO';
  else if (key === 7030) return 'WGS84';
  else if (key === 7031) return 'GEM10C';
  else if (key === 7032) return 'OSU86F';
  else if (key === 7033) return 'OSU91A';
  else if (key === 7034) return 'clrk80';
  else if (key === 7035) return 'SPHERE';
  else return undefined;
}

/**
 * Convert geotiff meter key to proj4 toMeter format
 * http://geotiff.maptools.org/spec/geotiff6.html#6.3.1.3
 * @param key - the geotiff key
 * @returns - the proj4 toMeter
 */
function buildToMeter(key?: number): number | undefined {
  if (key === 9001)
    return 1; // already in meters
  else if (key === 9002) return FT_TO_M;
  else if (key === 9003) return US_FT_TO_M;
  else if (key === 9004) return US_MOD_FT_TO_M;
  // others are unsupported
}
