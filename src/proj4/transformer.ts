import { parseProjStr } from './parseCode';
import { ALL_DEFINITIONS, DEFAULT_DEFINITIONS, WGS84 } from './projections';

import type { VectorPoint } from 's2-tools/geometry';
import type { ProjectionTransform, ProjectionTransformDefinition } from './projections';

/**
 * A Transformer class contains all projections necessary for converting coordinates from one
 * projection to another. This is a modular class that can be extended to add new projections
 * as needed to reduce code size and improve performance.
 * Both forward and inverse projections are default set to wgs84.
 */
export class Transformer {
  // Definitions are descriptions of projections
  definitions = new Map<string, ProjectionTransformDefinition>();
  // source and destination projections
  source: ProjectionTransform;
  destination: ProjectionTransform;

  /**
   * Prepares default definitions, source transform, and destination transform
   * @param sourceCode - convenience: if provided, we run `this.setSource(sourceCode)` immediately
   * @param destCode - convenience: if provided, we run `this.setDestination(destCode)` immediately
   */
  constructor(sourceCode?: string, destCode?: string) {
    for (const def of DEFAULT_DEFINITIONS) this.insertDefinition(def);
    // defaults to a standard WGS84 lon-lat projection transform
    this.source = this.destination = this.#buildTransformer(WGS84);
    if (sourceCode) this.setSource(sourceCode);
    if (destCode) this.setDestination(destCode);
  }

  /** @param sourceCode - can be a name or a coded definition */
  setSource(sourceCode: string): void {
    this.source = this.#buildTransformer(sourceCode);
  }

  /** @param destCode - can be a name or a coded definition */
  setDestination(destCode: string): void {
    this.destination = this.#buildTransformer(destCode);
  }

  /**
   * @param code - can be a WKT object or proj4 encoded string
   * @returns - A ready to use ProjectionTransform
   */
  #buildTransformer(code: string): ProjectionTransform {
    const params = parseProjStr(code);
    const def = this.definitions.get(params.name ?? '');
    if (def === undefined) throw Error(`${params.name} invalid, unsupported, or not loaded`);
    return new def(params);
  }

  /**
   * @param def - a class that may be instatiated with future setSource and setDestination
   * @param names - optionally add projection reference names to add lookups to the definition
   */
  insertDefinition(def: ProjectionTransformDefinition, names: string[] = []): void {
    for (const name of def.names) this.definitions.set(name, def);
    for (const name of names) this.definitions.set(name, def);
  }

  /**
   * @param p - vector point currently in the "source" projection
   * @param _enforceAxis - enforce axis ensures axis consistency relative to the final projection
   * @returns - vector point in the "destination" projection
   */
  forward(p: VectorPoint, _enforceAxis?: boolean): VectorPoint {
    // TODO: apply enforceAxis if true
    if (this.source.name === this.destination.name) return p;
    return this.destination.forward(this.source.inverse(p));
  }

  /**
   * @param p - vector point currently in the "destination" projection
   * @param _enforceAxis - enforce axis ensures axis consistency relative to the final projection
   * @returns - vector point in the "source" projection
   */
  inverse(p: VectorPoint, _enforceAxis?: boolean): VectorPoint {
    // TODO: apply enforceAxis if true
    if (this.source.name === this.destination.name) return p;
    return this.source.forward(this.destination.inverse(p));
  }
}

/** @param transformer - projection transformer */
export function injectAllDefinitions(transformer: Transformer) {
  for (const proj of ALL_DEFINITIONS) transformer.insertDefinition(proj);
}
