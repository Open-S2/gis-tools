import { toWM } from '../s2';
import { toS2, toUnitScale, toVector } from '../wm';

import type {
  Feature,
  JSONCollection,
  Projection,
  S2Feature,
  VectorFeature,
  VectorFeatures,
} from '..';

/**
 * @param projection - output either S2 or WM
 * @param data - the data to convert
 * @param buildBBox - optional - build a bbox for the feature if desired
 * @param tolerance - optionally specify a tolerance to prepare for future simplification
 * @param maxzoom - optionally specify a maxzoom to prepare for future simplification
 * @param toUnitScale - optional - convert to unit scale. Assumed to be true if not specified
 * @returns - the converted data
 */
export function convert(
  projection: Projection,
  data: JSONCollection,
  buildBBox?: boolean,
  tolerance?: number,
  maxzoom?: number,
  toUnitScale = false,
): VectorFeatures[] {
  const res: VectorFeatures[] = [];

  if (data.type === 'Feature') {
    res.push(...convertFeature(projection, data, toUnitScale, tolerance, maxzoom, buildBBox));
  } else if (data.type === 'VectorFeature') {
    res.push(...convertVectorFeature(projection, data, toUnitScale, tolerance, maxzoom));
  } else if (data.type === 'FeatureCollection') {
    for (const feature of data.features) {
      if (feature.type === 'Feature')
        res.push(
          ...convertFeature(projection, feature, toUnitScale, tolerance, maxzoom, buildBBox),
        );
      else res.push(...convertVectorFeature(projection, feature, toUnitScale, tolerance, maxzoom));
    }
  } else if (data.type === 'S2Feature') {
    res.push(convertS2Feature(projection, data, toUnitScale, tolerance, maxzoom));
  } else if (data.type === 'S2FeatureCollection') {
    for (const feature of data.features) {
      res.push(convertS2Feature(projection, feature, toUnitScale, tolerance, maxzoom));
    }
  }

  return res;
}

/**
 * @param projection - either S2 or WM is the end goal feature
 * @param data - input feature data
 * @param toUS - convert to unit scale if true
 * @param tolerance - optionally specify a tolerance to prepare for future simplification
 * @param maxzoom - optionally specify a maxzoom to prepare for future simplification
 * @param buildBBox - optional - build a bbox for the feature if desired
 * @returns - converted feature
 */
function convertFeature(
  projection: Projection,
  data: Feature,
  toUS: boolean,
  tolerance?: number,
  maxzoom?: number,
  buildBBox?: boolean,
): VectorFeatures[] {
  if (projection === 'WM') {
    const vf = toVector(data, buildBBox);
    if (toUS) toUnitScale(vf, tolerance, maxzoom);
    return [vf];
  } else {
    return toS2(data, tolerance, maxzoom, buildBBox);
  }
}

/**
 * @param projection - either S2 or WM is the end goal feature
 * @param data - input feature data
 * @param toUS - convert to unit scale if true
 * @param tolerance - optionally specify a tolerance to prepare for future simplification
 * @param maxzoom - optionally specify a maxzoom to prepare for future simplification
 * @returns - converted feature(s)
 */
function convertVectorFeature(
  projection: Projection,
  data: VectorFeature,
  toUS: boolean,
  tolerance?: number,
  maxzoom?: number,
): VectorFeatures[] {
  if (projection === 'WM') {
    if (toUS) toUnitScale(data, tolerance, maxzoom);
    return [data];
  } else {
    return toS2(data, tolerance, maxzoom);
  }
}

/**
 * @param projection - either S2 or WM is the end goal feature
 * @param data - input feature data
 * @param toUS - convert to unit scale if true
 * @param tolerance - optionally specify a tolerance to prepare for future simplification
 * @param maxzoom - optionally specify a maxzoom to prepare for future simplification
 * @returns - converted feature
 */
function convertS2Feature(
  projection: Projection,
  data: S2Feature,
  toUS: boolean,
  tolerance?: number,
  maxzoom?: number,
): VectorFeatures {
  if (projection === 'WM') {
    const vf = toWM(data);
    if (toUS) toUnitScale(vf, tolerance, maxzoom);
    return vf;
  } else {
    return data;
  }
}
