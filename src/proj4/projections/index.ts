import { AlbersConicEqualArea } from './aea.js';
import { AzimuthalEquidistant } from './aeqd.js';
import { BonneWerner } from './bonne.js';
import { CassiniSoldner } from './cass.js';
import { CylindricalEqualArea } from './cea.js';
import { EqualEarth } from './eqearth.js';
import { EquiRectangular } from './equi.js';
import { EquidistantConic } from './eqdc.js';
import { EquidistantCylindrical } from './eqc.js';
import { ExtendedTransverseMercator } from './etmerc.js';
import { GaussKruger } from './gauss.js';
// import { GaussSchreiberTransverseMercator } from './gstmerc.js';
import { Geocentric } from './geocent.js';
import { GeostationarySatelliteView } from './geos.js';
import { Gnomonic } from './gnom.js';
import { HotineObliqueMercator } from './omerc.js';
import { Krovak } from './krovak.js';
import { LambertAzimuthalEqualArea } from './laea.js';
import { LambertConformalConic } from './lcc.js';
import { MillerCylindrical } from './mill.js';
import { Mollweide } from './moll.js';
import { NewZealandMapGrid } from './nzmg.js';
import { Orthographic } from './ortho.js';
import { Polyconic } from './poly.js';
import { QuadrilateralizedSphericalCube } from './qsc.js';
import { Robinson } from './robin.js';
import { Sinusoidal } from './sinu.js';
import { StereographicNorthPole } from './sterea.js';
import { StereographicSouthPole } from './stere.js';
import { SwissObliqueMercator } from './somerc.js';
import { TiltedPerspective } from './tpers.js';
import { TransverseMercator } from './tmerc.js';
import { UniversalTransverseMercator } from './utm.js';
import { VanDerGrinten } from './vandg.js';
import { Mercator, WebMercator } from './merc.js';

import { ProjectionBase } from './base.js';

import type { DatumParams } from '../../readers/wkt/index.js';
import type { NadGridDefinition } from '../../readers/nadgrid.js';
import type { VectorPoint } from '../../geometry/index.js';

export * from './aea.js';
export * from './aeqd.js';
export * from './base.js';
export * from './bonne.js';
export * from './cass.js';
export * from './cea.js';
export * from './eqc.js';
export * from './eqdc.js';
export * from './eqearth.js';
export * from './equi.js';
export * from './etmerc.js';
export * from './gauss.js';
export * from './geocent.js';
export * from './geos.js';
export * from './gnom.js';
// export * from './gstmerc.js';
export * from './krovak.js';
export * from './laea.js';
export * from './lcc.js';
export * from './merc.js';
export * from './mill.js';
export * from './moll.js';
export * from './nzmg.js';
export * from './omerc.js';
export * from './ortho.js';
export * from './poly.js';
export * from './qsc.js';
export * from './references.js';
export * as EPSG_CODES from './references.js';
export * from './robin.js';
export * from './sinu.js';
export * from './somerc.js';
export * from './stere.js';
export * from './sterea.js';
export * from './tmerc.js';
export * from './tpers.js';
export * from './utm.js';
export * from './vandg.js';

/** Defines a projection class that isn't instantiated yet */
export type ProjectionTransformDefinition = typeof ProjectionBase;

/** All projections need these parameters */
export interface ProjectionTransform {
  name: string;
  projName?: string;
  axis: string;
  toMeter?: number;
  fromGreenwich: number;
  datum?: string;
  grids?: NadGridDefinition[];
  datumCode: string;
  datumType: number;
  datumParams: DatumParams;
  a: number;
  b: number;
  es: number;
  forward: (p: VectorPoint) => void;
  inverse: (p: VectorPoint) => void;
}

/** Contains all projections */
export const ALL_DEFINITIONS: ProjectionTransformDefinition[] = [
  AlbersConicEqualArea,
  AzimuthalEquidistant,
  BonneWerner,
  CassiniSoldner,
  CylindricalEqualArea,
  EquidistantCylindrical,
  EquidistantConic,
  EqualEarth,
  EquiRectangular,
  ExtendedTransverseMercator,
  GaussKruger,
  // GaussSchreiberTransverseMercator,
  Geocentric,
  GeostationarySatelliteView,
  Gnomonic,
  HotineObliqueMercator,
  Krovak,
  LambertAzimuthalEqualArea,
  LambertConformalConic,
  MillerCylindrical,
  Mollweide,
  NewZealandMapGrid,
  Orthographic,
  Polyconic,
  QuadrilateralizedSphericalCube,
  Robinson,
  Sinusoidal,
  StereographicNorthPole,
  StereographicSouthPole,
  SwissObliqueMercator,
  TiltedPerspective,
  TransverseMercator,
  UniversalTransverseMercator,
  WebMercator,
  VanDerGrinten,
];

/**
 * Builds most commonly used projections
 * @returns - an array of default projections
 */
export const DEFAULT_DEFINITIONS: [
  BASE: ProjectionTransformDefinition,
  MERC: ProjectionTransformDefinition,
] = [ProjectionBase, Mercator];
