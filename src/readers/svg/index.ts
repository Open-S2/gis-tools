import { equalPoints, xmlFindTagsByName, xmlGetAttribute } from 'gis-tools-ts';

import type { BBox, MValue, Properties, VectorFeature, VectorPoint } from 's2json-spec';
import type { FeatureIterator, XMLTag } from 'gis-tools-ts';

export interface SVGProperties extends Properties {
  stroke?: string;
  strokeWidth?: string;
  fill?: string;
}

/**
 * # SVG Reader
 *
 * ## Description
 *
 * Reads data from an SVG string
 *
 * Implements the {@link FeatureIterator} interface
 *
 *
 * ## Usage
 *
 * ```ts
 * import { SVGReader } from 'gis-tools-ts';
 *
 * const reader = new SVGReader('svg-text');
 *
 * // read all the features
 * for await (const feature of reader) {
 *   console.log(feature);
 * }
 * ```
 *
 * ## Links
 * - <https://en.wikipedia.org/wiki/SVG>
 */
export class SVGReader implements FeatureIterator<Record<string, unknown>, MValue, SVGProperties> {
  extent: BBox;
  features: VectorFeature<Record<string, unknown>, MValue, SVGProperties>[] = [];
  constructor(public input: string) {
    // build extent
    const viewBox = xmlGetAttribute(input, 'viewBox');
    if (viewBox !== undefined) {
      const [minx, miny, width, height] = viewBox.split(' ').map(Number);
      this.extent = [minx, miny, minx + width, miny + height];
    } else {
      throw new Error('Could not find viewBox in SVG.');
    }
    // build features
    this.#buildFeatures();
  }

  #buildFeatures() {
    this.#buildPaths();
    // TODO: circle - https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/circle
    // TODO: ellipse - https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/ellipse
    // TODO: rect - https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/rect
    // TODO: line - https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/line
    // TODO: polyline - https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/polyline
    // TODO: polygon - https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/polygon
    // TODO: text - https://developer.mozilla.org/en-US/docs/Web/SVG/Element/text
  }

  #buildPaths() {
    for (const { stroke, strokeWidth, fill, d } of pathsFromSVG(this.input)) {
      const points = getPointsFromSVGPath(d, this.extent);
      const properties: SVGProperties = { stroke, strokeWidth, fill };
      if (fill !== undefined) {
        // Ensure it self closes
        if (!equalPoints(points[0], points[points.length - 1])) {
          points.push(points[0]);
        }
        this.features.push({
          type: 'VectorFeature',
          geometry: { type: 'Polygon', coordinates: [points], is3D: false },
          properties,
        });
      } else {
        this.features.push({
          type: 'VectorFeature',
          geometry: { type: 'LineString', coordinates: points, is3D: false },
          properties,
        });
      }
    }
  }

  /**
   * Iterate over all features in the shapefile
   * @yields {VectorFeature}
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<
    VectorFeature<Record<string, unknown>, MValue, SVGProperties>
  > {
    for (let i = 0; i < this.features.length; i++) {
      const feature = this.features[i];
      if (feature !== undefined) yield feature;
    }
  }
}

export interface SVGPath {
  d: string;
  stroke?: string;
  strokeWidth?: string;
  fill?: string;
}

export function pathsFromSVG(svg: string): SVGPath[] {
  const pathTags = xmlFindTagsByName(svg, 'path');
  return pathTags.map(pathFromSVG);
}

export function pathFromSVG(path: string | XMLTag): SVGPath {
  return {
    // The raw "d" attribute string (e.g., "M10,10 L20,20...")
    d: xmlGetAttribute(path, 'd') ?? '',
    stroke: xmlGetAttribute(path, 'stroke'),
    strokeWidth: xmlGetAttribute(path, 'stroke-width'),
    fill: xmlGetAttribute(path, 'fill'),
  };
}

export function getPointsFromSVGPath(d: string, extent: BBox): VectorPoint[] {
  const [minX, minY, maxX, maxY] = extent;
  const width = maxX - minX;
  const height = maxY - minY;
  const commands = d
    .split(' ')
    .map((s) => s.trim())
    .filter(Boolean);

  const points: VectorPoint[] = [];
  const cursor: VectorPoint = { x: 0, y: 0 };
  let lastPoint = { x: 0, y: 0 };
  let x = 0;
  let y = 0;

  for (let i = 0; i < commands.length; i++) {
    const cmd = commands[i];
    const isRelative = cmd === cmd.toLowerCase();
    const type = cmd.toUpperCase();

    // The next element in the array contains the numbers for this command

    switch (type) {
      case 'M': // MoveTo
        x = Number(commands[i + 1]);
        y = Number(commands[i + 2]);
        cursor.x = isRelative ? cursor.x + x : x;
        cursor.y = isRelative ? cursor.y + y : y;
        points.push({ ...cursor });
        i += 2; // skip the args
        break;
      case 'L': // LineTo
        x = Number(commands[i + 1]);
        y = Number(commands[i + 2]);
        cursor.x = isRelative ? cursor.x + x : x;
        cursor.y = isRelative ? cursor.y + y : y;
        points.push({ ...cursor });
        i += 2; // skip the args
        break;
      case 'H': // Horizontal
        x = Number(commands[i + 1]);
        cursor.x = isRelative ? cursor.x + x : x;
        points.push({ ...cursor });
        i++;
        break;
      case 'V': // Vertical
        y = Number(commands[i + 1]);
        cursor.y = isRelative ? cursor.y + y : y;
        points.push({ ...cursor });
        i++;
        break;
      case 'Z': // Close
        points.push({ ...lastPoint });
        break;
      // TODO: C, S, Q, T, A: interpolations
    }
    lastPoint = { ...cursor };
  }

  // remap to extent
  for (const p of points) {
    // Normalize X: (Value - Offset) / Total Range
    p.x = (p.x - minX) / width;
    // Normalize Y and Flip: 1 - ((Value - Offset) / Total Range)
    p.y = 1 - (p.y - minY) / height;
  }

  return points;
}
