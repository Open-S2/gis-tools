import { ALL_PROJECTIONS, DEFAULT_PROJECTIONS, ProjectionTransform } from './projections';

import type { ProjectionTransformDefinition } from './projections';
import type { VectorPoint } from 's2-tools/geometry';

/**
 * A Transformer class contains all projections necessary for converting coordinates from one
 * projection to another. This is a modular class that can be extended to add new projections
 * as needed to reduce code size and improve performance.
 * Both forward and inverse projections are default set to wgs84.
 */
export class Transformer {
  projections = new Map<string, ProjectionTransformDefinition>();
  source: ProjectionTransformDefinition;
  destination: ProjectionTransformDefinition;
  /**
   * Prepares default projections
   * @param sourceDef - can be a name or a coded definition
   * @param destinationDef - can be a name or a coded definition
   */
  constructor(sourceDef?: string, destinationDef?: string) {
    for (const proj of DEFAULT_PROJECTIONS) this.insertProjection(proj);
    const [wgs84] = DEFAULT_PROJECTIONS;
    this.source = this.destination = wgs84;
    // update source and destination if defined
    if (sourceDef !== undefined) this.setsource(sourceDef);
    if (destinationDef !== undefined) this.setDestination(destinationDef);
  }

  /** @param sourceDef - can be a name or a coded definition */
  setsource(sourceDef: string) {
    const projection: ProjectionTransformDefinition =
      this.projections.get(sourceDef) ?? new ProjectionTransform(sourceDef);
    this.source = projection;
  }

  /** @param destinationDef - can be a name or a coded definition */
  setDestination(destinationDef: string) {
    const projection: ProjectionTransformDefinition =
      this.projections.get(destinationDef) ?? new ProjectionTransform(destinationDef);
    this.destination = projection;
  }

  /** @param proj - projection transform */
  insertProjection(proj: ProjectionTransformDefinition) {
    for (const name of proj.names) this.projections.set(name, proj);
  }

  /**
   * @param p - vector point currently in the "source" projection
   * @param _enforceAxis - enforce axis ensures axis consistency relative to the final projection
   * @returns - vector point in the "destination" projection
   */
  forward(p: VectorPoint, _enforceAxis?: boolean): VectorPoint {
    return this.destination.forward(this.source.inverse(p));
  }

  /**
   * @param p - vector point currently in the "destination" projection
   * @param _enforceAxis - enforce axis ensures axis consistency relative to the final projection
   * @returns - vector point in the "source" projection
   */
  inverse(p: VectorPoint, _enforceAxis?: boolean): VectorPoint {
    return this.source.forward(this.destination.inverse(p));
  }
}

/** @param transformer - projection transformer */
export function injectAllProjections(transformer: Transformer) {
  for (const proj of ALL_PROJECTIONS) transformer.insertProjection(proj);
}
