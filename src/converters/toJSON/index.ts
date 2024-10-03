import { convert } from '../../geometry/convert';
import { mergeBBoxes } from '../../geometry';

import type { FeatureIterator } from '../../readers';
import type { Writer } from '../../writers';
import type { BBOX, Projection, VectorFeatures } from '../../geometry';

/** User defined options on how to store the features */
export interface ToJSONOptions {
  projection?: Projection;
  buildBBox?: boolean;
  onFeature?: (feature: VectorFeatures) => VectorFeatures | undefined;
}

/**
 * @param writer - the writer to append strings to
 * @param iterators - the collection of iterators to write
 * @param opts - user defined options [optional]
 */
export async function toJSON(
  writer: Writer,
  iterators: FeatureIterator[],
  opts?: ToJSONOptions,
): Promise<void> {
  const projection = opts?.projection ?? 'S2';
  const type = projection === 'S2' ? 'S2FeatureCollection' : 'FeatureCollection';
  const faces = new Set<number>();
  const buildBBox = opts?.buildBBox ?? false;
  let bbox: BBOX | undefined;
  const onFeature = opts?.onFeature ?? ((feature) => feature);

  await writer.appendString(`{\n\t"type": "${type}",\n`);
  await writer.appendString('\t"features": [\n');

  let first = true;
  for (const iterator of iterators) {
    for await (const feature of iterator) {
      const convertedFeatures = convert(projection, feature, buildBBox);
      for (const convertedFeature of convertedFeatures) {
        const userFeature = onFeature(convertedFeature);
        if (userFeature === undefined) continue;
        faces.add(userFeature.face ?? 0);
        if (buildBBox && userFeature.geometry.bbox !== undefined)
          bbox = mergeBBoxes(bbox, userFeature.geometry.bbox);
        if (!first) await writer.appendString(',\n');
        first = false;
        await writer.appendString(`\t\t${JSON.stringify(userFeature)}`);
      }
    }
  }

  await writer.appendString('\n\t],');
  await writer.appendString(`\n\t"faces": ${JSON.stringify([...faces])}`);
  if (bbox !== undefined) await writer.appendString(`,\n\t"bbox": ${JSON.stringify(bbox)}`);
  await writer.appendString('\n}');
}

/**
 * @param writer - the writer to apppend strings to
 * @param iterators - the collection of iterators to write
 * @param opts - user defined options [optional]
 */
export async function toJSONLD(
  writer: Writer,
  iterators: FeatureIterator[],
  opts?: ToJSONOptions,
): Promise<void> {
  const projection = opts?.projection ?? 'S2';
  const onFeature = opts?.onFeature ?? ((feature) => feature);
  const buildBBox = opts?.buildBBox ?? false;

  for (const iterator of iterators) {
    for await (const feature of iterator) {
      const convertedFeatures = convert(projection, feature, buildBBox);
      for (const convertedFeature of convertedFeatures) {
        const userFeature = onFeature(convertedFeature);
        if (userFeature === undefined) continue;
        await writer.appendString(JSON.stringify(userFeature) + '\n');
      }
    }
  }
}
