import { AlbersConicEqualArea } from './aea';
import { AzimuthalEquidistant } from './aeqd';
import { BonneWerner } from './bonne';
import { CassiniSoldner } from './cass';
import { CylindricalEqualArea } from './cea';
import { EquidistantCylindrical } from './eqc';
import { Mercator } from './merc';

import { ProjectionBase } from './base';

import type { VectorPoint } from 's2json-spec';

export * from './aea';
export * from './aeqd';
export * from './base';
export * from './bonne';
export * from './cass';
export * from './cea';
export * from './eqc';
export * from './merc';
export * from './references';

/** Defines a projection class that isn't instantiated yet */
export type ProjectionTransformDefinition = typeof ProjectionBase;

/** All projections need these parameters */
export interface ProjectionTransform {
  name: string;
  forward: (p: VectorPoint) => VectorPoint;
  inverse: (p: VectorPoint) => VectorPoint;
}

/** Contains all projections */
export const ALL_DEFINITIONS: ProjectionTransformDefinition[] = [
  AlbersConicEqualArea,
  AzimuthalEquidistant,
  BonneWerner,
  CassiniSoldner,
  CylindricalEqualArea,
  EquidistantCylindrical,
];

/**
 * Builds most commonly used projections
 * @returns - an array of default projections
 */
export const DEFAULT_DEFINITIONS: [
  BASE: ProjectionTransformDefinition,
  MERC: ProjectionTransformDefinition,
] = [ProjectionBase, Mercator];
