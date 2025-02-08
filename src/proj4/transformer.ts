import { NadGridStore } from '../readers/nadgrid';
import { parseProj } from './parseCode';
import { ALL_DEFINITIONS, DEFAULT_DEFINITIONS, EPSG_CODES, WGS84 } from './projections';
import { checkNotWGS, datumTransform } from './datum';

import type { MValue, VectorPoint, VectorPointM } from '../geometry';
import type {
  ProjectionParams,
  ProjectionTransform,
  ProjectionTransformDefinition,
} from './projections';

/**
 * # PROJ4 Transformer
 *
 * ## Description
 *
 * A Transformer class contains all projections necessary for converting coordinates from one
 * projection to another. This is a modular class that can be extended to add new projections
 * as needed to reduce code size and improve performance.
 * Both forward and inverse projections are default set to wgs84.
 *
 * Extends the {@link NadGridStore} class to support grid lookups
 *
 * ## Usage
 *
 * ### Full Example
 *
 * ```ts
 * import { Transformer, injectAllDefinitions, injectAllEPSGCodes } from 'gis-tools-ts';
 *
 * // Create a transform using a source and destination projection
 * const transform = new Transformer();
 * // inject all default definition projections. This is not memory efficient but ensures all
 * // projections are available
 * injectAllDefinitions(transform);
 * // inject all common EPSG codes. This is not memory efficient but ensures all EPSG codes are available
 * injectAllEPSGCodes(transform);
 * // If the transform requires a grid, this is how you add it.
 * transform.addGridFromReader(
 *   'BETA2007.gsb',
 *   new MMapReader(`${__dirname}/fixtures/BETA2007.gsb`),
 * );
 * // Set the source and destination projections
 * transform.setSource('EPSG_31466');
 * transform.setDestination('EPSG_25832');
 * // example forward projection
 * const forward = transform.forward({ x: 2559552, y: 5670982 });
 * // example inverse projection
 * const inverse = transform.inverse({ x: 349757.381712518, y: 5671004.06504954 });
 * ```
 *
 * ### Minimal Example only adding the Oblique Mercator
 *
 * ```ts
 * import { Transformer, HotineObliqueMercator, EPSG_8803 } from 'gis-tools-ts';
 *
 * const transform = new Transformer();
 * transform.insertDefinition(HotineObliqueMercator);
 * transform.insertEPSGCode('EPSG_8803', EPSG_8803);
 *
 * transform.setDestination('EPSG_8803');
 *
 * const forward = transform.forward({ x: 60.8, y: -132.2 });
 * ```
 */
export class Transformer extends NadGridStore {
  // EPSG code definitions
  epsgs = new Map<string, string>();
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
  constructor(sourceCode?: string | ProjectionParams, destCode?: string | ProjectionParams) {
    super();
    // by default supports the mercator and base (lon-lat) projection
    for (const def of DEFAULT_DEFINITIONS) this.insertDefinition(def);
    // defaults to a standard WGS84 lon-lat projection transform
    this.source = this.destination = this.wgs84 = this.#buildTransformer(WGS84);
    if (sourceCode !== undefined) this.setSource(sourceCode);
    if (destCode !== undefined) this.setDestination(destCode);
  }

  /**
   * Set the source projection
   * @param sourceCode - can be a name or a coded definition
   */
  setSource(sourceCode: string | ProjectionParams): void {
    this.source = this.#buildTransformer(sourceCode);
  }

  /**
   * Set the destination projection
   * @param destCode - can be a name or a coded definition
   */
  setDestination(destCode: string | ProjectionParams): void {
    this.destination = this.#buildTransformer(destCode);
  }

  /**
   * Build a ProjectionTransform
   * @param code - can be a WKT object or proj4 encoded string
   * @returns - A ready to use ProjectionTransform
   */
  #buildTransformer(code: string | ProjectionParams): ProjectionTransform {
    code = this.#parseEPSGCode(code);
    const params = parseProj(code, this);
    // search
    let def: ProjectionTransformDefinition | undefined;
    for (const name of [params.projName, params.name]) {
      def = this.definitions.get(name?.toLowerCase() ?? '');
      if (def !== undefined) break;
    }
    if (def === undefined)
      throw Error(`${params.projName ?? params.name} invalid, unsupported, or not loaded`);
    return new def(params);
  }

  /**
   * Attempt to parse the EPSG code if epsg codes are loaded and return the definition
   * @param code - a string or params with the EPSG code set in projName
   * @returns - if no EPSG code is found, returns the original code. Otherwise returns the EPSG definition
   */
  #parseEPSGCode(code: string | ProjectionParams): string | ProjectionParams {
    const codeName = (typeof code === 'string' ? code : code.projName)?.replaceAll(':', '_');
    const epsg = this.epsgs.get(codeName ?? '');
    if (epsg !== undefined) return epsg;
    return code;
  }

  /**
   * Insert a projection definition
   * ```ts
   * import { Transformer, HotineObliqueMercator } from 'gis-tools-ts';
   * const transformer = new Transformer();
   * transformer.insertDefinition(HotineObliqueMercator);
   * ```
   * @param def - a class that may be instatiated with future setSource and setDestination
   * @param names - optionally add projection reference names to add lookups to the definition
   */
  insertDefinition(def: ProjectionTransformDefinition, names: string[] = []): void {
    for (const name of def.names) this.definitions.set(name.toLowerCase(), def);
    for (const name of names) this.definitions.set(name.toLowerCase(), def);
  }

  /**
   * Insert an EPSG code definition
   * ```ts
   * import { Transformer, EPSG_4326 } from 'gis-tools-ts';
   * const transformer = new Transformer();
   * transformer.insertEPSGCode('EPSG_4326', EPSG_4326);
   * ```
   * @param code - EPSG code to insert e.g. "EPSG_4326" (uses underscore instead of colon)
   * @param value - the EPSG definition which is either a WKT string object or proj4 encoded string
   */
  insertEPSGCode(code: string, value: string): void {
    this.epsgs.set(code, value);
  }

  /**
   * Forward projection from src projection to dest projection
   * ```ts
   * const transformer = new Transformer();
   * transformer.setSource('EPSG_4326');
   * const point = transformer.forward({ x: 0, y: 0 });
   * ```
   * @param p - vector point currently in the "source" projection
   * @param enforceAxis - enforce axis ensures axis consistency relative to the final projection
   * @returns - vector point in the "destination" projection
   */
  forward<D extends MValue>(p: VectorPoint<D>, enforceAxis?: false): VectorPoint<D>;
  /**
   * Forward projection from src projection to dest projection
   * ```ts
   * const transformer = new Transformer();
   * transformer.setSource('EPSG_4326');
   * const point = transformer.forward({ x: 0, y: 0 });
   * ```
   * @param p - vector point currently in the "source" projection
   * @param enforceAxis - enforce axis ensures axis consistency relative to the final projection
   * @returns - vector point in the "destination" projection
   */
  forward<D extends MValue>(p: VectorPointM<D>, enforceAxis?: false): VectorPointM<D>;

  /**
   * Forward projection from src projection to dest projection
   * ```ts
   * const transformer = new Transformer();
   * transformer.setSource('EPSG_4326');
   * const point = transformer.forward({ x: 0, y: 0 });
   * ```
   * @param p - vector point currently in the "source" projection
   * @param enforceAxis - enforce axis ensures axis consistency relative to the final projection
   * @returns - vector point in the "destination" projection
   */
  forward<D extends MValue>(
    p: VectorPoint<D> | VectorPointM<D>,
    enforceAxis = false,
  ): VectorPoint<D> | VectorPointM<D> {
    return this.#transformPoint(p, this.source, this.destination, enforceAxis);
  }

  /**
   * Inverse projection from dest projection to src projection
   * ```ts
   * const transformer = new Transformer();
   * transformer.setSource('EPSG_4326');
   * const point = transformer.inverse({ x: 0, y: 0 });
   * ```
   * @param p - vector point currently in the "destination" projection
   * @param enforceAxis - enforce axis ensures axis consistency relative to the final projection
   * @returns - vector point in the "source" projection
   */
  inverse<D extends MValue>(p: VectorPoint<D>, enforceAxis = false): VectorPoint<D> {
    return this.#transformPoint(p, this.destination, this.source, enforceAxis);
  }

  /**
   * Transforms a point from one projection to another
   * @param sourcePoint - point to start transforming
   * @param src - source projection
   * @param dest - destination projection
   * @param enforceAxis - enforce axis ensures axis consistency relative to the final projection
   * @returns - transformed point
   */
  #transformPoint<M extends MValue>(
    sourcePoint: VectorPoint<M>,
    src: ProjectionTransform,
    dest: ProjectionTransform,
    enforceAxis: boolean,
  ): VectorPoint<M> {
    // Workaround for datum shifts towgs84, if either source or destination projection is not wgs84
    let res: VectorPoint<M> = { ...sourcePoint };
    if (src === dest) return res;
    const hasZ = sourcePoint.z !== undefined;
    if (checkNotWGS(src, dest)) {
      const wgs84 = this.wgs84;
      res = this.#transformPoint(res, src, wgs84, enforceAxis);
      src = wgs84;
    }
    // STEP 1: SOURCE -> WGS84
    // if needed, adjust axis
    if (enforceAxis && src.axis !== 'enu') adjustAxis(res, src, true);
    // adjust for meters if necessary
    if (src.name !== 'longlat' && src.projName !== 'longlat' && src.toMeter !== undefined) {
      res.x *= src.toMeter;
      res.y *= src.toMeter;
    }

    // transform forward
    src.inverse(res);
    // Adjust for the prime meridian if necessary
    res.x += src.fromGreenwich;

    // STEP 2: MID-POINT. Convert datums if needed, and if possible.
    datumTransform(res, src, dest);

    // STEP 3: WGS84 -> DEST
    // Adjust for the prime meridian if necessary
    res.x -= dest.fromGreenwich;
    // transform forward
    dest.forward(res);
    // adjust for meters if necessary
    if (dest.name !== 'longlat' && dest.projName !== 'longlat' && dest.toMeter !== undefined) {
      res.x /= dest.toMeter;
      res.y /= dest.toMeter;
    }
    // if needed, adjust axis
    if (enforceAxis && dest.axis !== 'enu') adjustAxis(res, dest, true);

    if (!hasZ) delete res.z;

    return res;
  }
}

/**
 * Inject all default definitions into the transformer
 * @param transformer - projection transformer
 */
export function injectAllDefinitions(transformer: Transformer): void {
  for (const proj of ALL_DEFINITIONS) transformer.insertDefinition(proj);
}

/**
 * Inject all EPSG codes into the transformer
 * @param transformer - the transformer to inject EPSG codes to
 */
export function injectAllEPSGCodes(transformer: Transformer): void {
  for (const [key, value] of Object.entries(EPSG_CODES)) transformer.insertEPSGCode(key, value);
}

/**
 * Adjusts an axis
 * @param point - the vector point to adjust
 * @param prj - the projection
 * @param denorm - denormalizes z if true
 */
function adjustAxis<M extends MValue>(
  point: VectorPoint<M>,
  prj: ProjectionTransform,
  denorm: boolean,
): void {
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
