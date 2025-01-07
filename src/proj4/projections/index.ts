import { AlbersConicEqualArea } from './aea';
import { AzimuthalEquidistant } from './aeqd';
import { BonneWerner } from './bonne';
import { CassiniSoldner } from './cass';
import { CylindricalEqualArea } from './cea';
import { EqualEarth } from './eqearth';
import { EquiRectangular } from './equi';
import { EquidistantConic } from './eqdc';
import { EquidistantCylindrical } from './eqc';
import { ExtendedTransverseMercator } from './etmerc';
import { GaussKruger } from './gauss';
// import { GaussSchreiberTransverseMercator } from './gstmerc';
import { Geocentric } from './geocent';
import { GeostationarySatelliteView } from './geos';
import { Gnomonic } from './gnom';
import { HotineObliqueMercator } from './omerc';
import { Krovak } from './krovak';
import { LambertAzimuthalEqualArea } from './laea';
import { LambertConformalConic } from './lcc';
import { Mercator } from './merc';
import { MillerCylindrical } from './mill';
import { Mollweide } from './moll';
import { NewZealandMapGrid } from './nzmg';
import { Orthographic } from './ortho';
import { Polyconic } from './poly';
import { QuadrilateralizedSphericalCube } from './qsc';
import { Robinson } from './robin';
import { Sinusoidal } from './sinu';
import { StereographicNorthPole } from './sterea';
import { StereographicSouthPole } from './stere';
import { SwissObliqueMercator } from './somerc';
import { TiltedPerspective } from './tpers';
import { TransverseMercator } from './tmerc';
import { UniversalTransverseMercator } from './utm';
import { VanDerGrinten } from './vandg';

import { ProjectionBase } from './base';

import type { DatumParams } from '../../readers/wkt';
import type { NadGridDefinition } from '../../readers/nadgrid';
import type { VectorPoint } from '../../geometry';

export * from './aea';
export * from './aeqd';
export * from './base';
export * from './bonne';
export * from './cass';
export * from './cea';
export * from './eqc';
export * from './eqdc';
export * from './eqearth';
export * from './equi';
export * from './etmerc';
export * from './gauss';
export * from './geocent';
export * from './geos';
export * from './gnom';
// export * from './gstmerc';
export * from './krovak';
export * from './laea';
export * from './lcc';
export * from './merc';
export * from './mill';
export * from './moll';
export * from './nzmg';
export * from './omerc';
export * from './ortho';
export * from './poly';
export * from './qsc';
export * from './references';
export * as EPSG_CODES from './references';
export * from './robin';
export * from './sinu';
export * from './somerc';
export * from './stere';
export * from './sterea';
export * from './tmerc';
export * from './tpers';
export * from './utm';
export * from './vandg';

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
