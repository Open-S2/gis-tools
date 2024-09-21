import { parseProjStr } from './parseCode';
import { ALL_DEFINITIONS, DEFAULT_DEFINITIONS, WGS84 } from './projections';
import { checkNotWGS, datumTransform } from './datum';

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
  wgs84: ProjectionTransform;
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
    this.source = this.destination = this.wgs84 = this.#buildTransformer(WGS84);
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
    // search
    let def: ProjectionTransformDefinition | undefined;
    for (const name of [params.projName, params.name]) {
      def = this.definitions.get(name?.toLowerCase() ?? '');
      if (def !== undefined) break;
    }
    if (def === undefined) throw Error(`${params.name} invalid, unsupported, or not loaded`);
    return new def(params);
  }

  /**
   * @param def - a class that may be instatiated with future setSource and setDestination
   * @param names - optionally add projection reference names to add lookups to the definition
   */
  insertDefinition(def: ProjectionTransformDefinition, names: string[] = []): void {
    for (const name of def.names) this.definitions.set(name.toLowerCase(), def);
    for (const name of names) this.definitions.set(name.toLowerCase(), def);
  }

  /**
   * @param p - vector point currently in the "source" projection
   * @param enforceAxis - enforce axis ensures axis consistency relative to the final projection
   * @returns - vector point in the "destination" projection
   */
  forward(p: VectorPoint, enforceAxis = false): VectorPoint {
    return this.#transformPoint(p, this.source, this.destination, enforceAxis);
  }

  /**
   * @param p - vector point currently in the "destination" projection
   * @param enforceAxis - enforce axis ensures axis consistency relative to the final projection
   * @returns - vector point in the "source" projection
   */
  inverse(p: VectorPoint, enforceAxis = false): VectorPoint {
    return this.#transformPoint(p, this.destination, this.source, enforceAxis);
  }

  /**
   * @param sourcePoint - point to start transforming
   * @param src - source projection
   * @param dest - destination projection
   * @param enforceAxis - enforce axis ensures axis consistency relative to the final projection
   * @returns - transformed point
   */
  #transformPoint(
    sourcePoint: VectorPoint,
    src: ProjectionTransform,
    dest: ProjectionTransform,
    enforceAxis: boolean,
  ): VectorPoint {
    const hasZ = sourcePoint.z !== undefined;
    // Workaround for datum shifts towgs84, if either source or destination projection is not wgs84
    let res = { ...sourcePoint };
    if (checkNotWGS(src, dest)) {
      const wgs84 = this.wgs84;
      res = this.#transformPoint(res, src, wgs84, enforceAxis);
      src = wgs84;
    }
    // STEP 1: SOURCE -> WGS84
    // if needed, adjust axis
    if (enforceAxis && src.axis !== 'enu') {
      adjustAxis(res, src, true);
    }
    // adjust for meters if necessary
    if (src.name !== 'longlat' && src.projName !== 'longlat' && src.toMeter) {
      res.x *= src.toMeter;
      res.y *= src.toMeter;
    }

    // transform forward
    src.inverse(res);
    // Adjust for the prime meridian if necessary
    res.x += src.fromGreenwich;

    // STEP 2: MID-POINT. Convert datums if needed, and if possible.
    datumTransform(res, src, dest);

    // console.log('STEP 2: MID DATUM B', res);

    // STEP 3: WGS84 -> DEST
    // Adjust for the prime meridian if necessary
    res.x -= dest.fromGreenwich;
    // transform forward
    dest.forward(res);
    // adjust for meters if necessary
    if (dest.name !== 'longlat' && dest.projName !== 'longlat' && dest.toMeter) {
      res.x /= dest.toMeter;
      res.y /= dest.toMeter;
    }
    // if needed, adjust axis
    if (enforceAxis && dest.axis !== 'enu') {
      adjustAxis(res, dest, true);
    }

    if (!hasZ) delete res.z;

    return res;
  }
}

/** @param transformer - projection transformer */
export function injectAllDefinitions(transformer: Transformer) {
  for (const proj of ALL_DEFINITIONS) transformer.insertDefinition(proj);
}

/**
 * @param point - the vector point to adjust
 * @param prj - the projection
 * @param denorm - denormalizes z if true
 */
function adjustAxis(point: VectorPoint, prj: ProjectionTransform, denorm: boolean): void {
  const xin = point.x;
  const yin = point.y;
  const zin = point.z ?? 0;
  let v, i;
  let t: 'x' | 'y' | 'z';
  for (i = 0; i < 3; i++) {
    if (denorm && i === 2 && point.z === undefined) continue;
    if (i === 0) {
      v = xin;
      if ('ew'.indexOf(prj.axis[i]) !== -1) {
        t = 'x';
      } else {
        t = 'y';
      }
    } else if (i === 1) {
      v = yin;
      if ('ns'.indexOf(prj.axis[i]) !== -1) {
        t = 'y';
      } else {
        t = 'x';
      }
    } else {
      v = zin;
      t = 'z';
    }
    switch (prj.axis[i]) {
      case 'e':
        point[t] = v;
        break;
      case 'w':
        point[t] = -v;
        break;
      case 'n':
        point[t] = v;
        break;
      case 's':
        point[t] = -v;
        break;
      case 'u':
        if (point[t] !== undefined) {
          point.z = v;
        }
        break;
      case 'd':
        if (point[t] !== undefined) {
          point.z = -v;
        }
        break;
      default:
        throw Error(`unknown axis (${prj.axis[i]}) - check definition of ${prj.name}`);
    }
  }
}
