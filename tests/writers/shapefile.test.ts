import {
  BufferWriter,
  shapefileFromGzip,
  toDBF,
  toDBFMeta,
  toSHP,
  zipFolder,
} from '../../src/index.js';
import { describe, expect, test } from 'bun:test';

import type { FeatureIterator } from '../../src/index.js';

function createMockIteratorPointSHP(propertiesArray: Record<string, unknown>[]): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'Point',
            is3D: false,
            bbox: [index, index, index, index],
            coordinates: { x: index, y: index },
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorPointZSHP(propertiesArray: Record<string, unknown>[]): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'Point',
            is3D: true,
            bbox: [index, index, index, index],
            coordinates: { x: index, y: index, z: index },
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorPointZMSHP(propertiesArray: Record<string, unknown>[]): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'Point',
            is3D: true,
            bbox: [index, index, index, index],
            coordinates: { x: index, y: index, z: index, m: { value: index } },
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorPointMSHP(propertiesArray: Record<string, unknown>[]): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'Point',
            is3D: false,
            bbox: [index, index, index, index],
            coordinates: { x: index, y: index, m: { value: index } },
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorMultiPointSHP(
  propertiesArray: Record<string, unknown>[],
): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'MultiPoint',
            is3D: false,
            bbox: [index, index, index, index],
            coordinates: [
              { x: index, y: index },
              { x: index + 1, y: index + 1 },
            ],
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorMultiPointZSHP(
  propertiesArray: Record<string, unknown>[],
): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'MultiPoint',
            is3D: true,
            bbox: [index, index, index, index],
            coordinates: [
              { x: index, y: index, z: index },
              { x: index + 1, y: index + 1, z: index + 1 },
            ],
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorMultiPointZMSHP(
  propertiesArray: Record<string, unknown>[],
): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'MultiPoint',
            is3D: true,
            bbox: [index, index, index, index],
            coordinates: [
              { x: index, y: index, z: index, m: { value: index } },
              { x: index + 1, y: index + 1, z: index + 1, m: { value: index + 1 } },
            ],
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorMultiPointMSHP(
  propertiesArray: Record<string, unknown>[],
): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'MultiPoint',
            is3D: false,
            bbox: [index, index, index, index],
            coordinates: [
              { x: index, y: index, m: { value: index } },
              { x: index + 1, y: index + 1, m: { value: index + 1 } },
            ],
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorLineStringSHP(
  propertiesArray: Record<string, unknown>[],
): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'LineString',
            is3D: false,
            bbox: [index, index, index, index],
            coordinates: [
              { x: index, y: index },
              { x: index + 1, y: index + 1 },
            ],
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorLineStringZSHP(
  propertiesArray: Record<string, unknown>[],
): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'LineString',
            is3D: true,
            bbox: [index, index, index, index],
            coordinates: [
              { x: index, y: index, z: index },
              { x: index + 1, y: index + 1, z: index + 1 },
            ],
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorLineStringZMSHP(
  propertiesArray: Record<string, unknown>[],
): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'LineString',
            is3D: true,
            bbox: [index, index, index, index],
            coordinates: [
              { x: index, y: index, z: index, m: { value: index } },
              { x: index + 1, y: index + 1, z: index + 1, m: { value: index + 1 } },
            ],
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorLineStringMSHP(
  propertiesArray: Record<string, unknown>[],
): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'LineString',
            is3D: false,
            bbox: [index, index, index, index],
            coordinates: [
              { x: index, y: index, m: { value: index } },
              { x: index + 1, y: index + 1, m: { value: index + 1 } },
            ],
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorMultiLineStringSHP(
  propertiesArray: Record<string, unknown>[],
): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'MultiLineString',
            is3D: false,
            bbox: [index, index, index, index],
            coordinates: [
              [
                { x: index, y: index },
                { x: index + 1, y: index + 1 },
              ],
              [
                { x: index + 2, y: index + 2 },
                { x: index + 3, y: index + 3 },
              ],
            ],
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorMultiLineStringZSHP(
  propertiesArray: Record<string, unknown>[],
): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'MultiLineString',
            is3D: true,
            bbox: [index, index, index, index],
            coordinates: [
              [
                { x: index, y: index, z: index },
                { x: index + 1, y: index + 1, z: index + 1 },
              ],
              [
                { x: index + 2, y: index + 2, z: index + 2 },
                { x: index + 3, y: index + 3, z: index + 3 },
              ],
            ],
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorMultiLineStringZMSHP(
  propertiesArray: Record<string, unknown>[],
): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'MultiLineString',
            is3D: true,
            bbox: [index, index, index, index],
            coordinates: [
              [
                { x: index, y: index, z: index, m: { value: index } },
                { x: index + 1, y: index + 1, z: index + 1, m: { value: index + 1 } },
              ],
              [
                { x: index + 2, y: index + 2, z: index + 2, m: { value: index + 2 } },
                { x: index + 3, y: index + 3, z: index + 3, m: { value: index + 3 } },
              ],
            ],
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorMultiLineStringMSHP(
  propertiesArray: Record<string, unknown>[],
): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'MultiLineString',
            is3D: false,
            bbox: [index, index, index, index],
            coordinates: [
              [
                { x: index, y: index, m: { value: index } },
                { x: index + 1, y: index + 1, m: { value: index + 1 } },
              ],
              [
                { x: index + 2, y: index + 2, m: { value: index + 2 } },
                { x: index + 3, y: index + 3, m: { value: index + 3 } },
              ],
            ],
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorPolygonSHP(propertiesArray: Record<string, unknown>[]): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'Polygon',
            is3D: false,
            bbox: [index, index, index, index],
            coordinates: [
              [
                { x: index, y: index },
                { x: index + 1, y: index + 1 },
              ],
              [
                { x: index + 2, y: index + 2 },
                { x: index + 3, y: index + 3 },
              ],
            ],
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorPolygonZSHP(
  propertiesArray: Record<string, unknown>[],
): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'Polygon',
            is3D: true,
            bbox: [index, index, index, index],
            coordinates: [
              [
                { x: index, y: index, z: index },
                { x: index + 1, y: index + 1, z: index + 1 },
              ],
              [
                { x: index + 2, y: index + 2, z: index + 2 },
                { x: index + 3, y: index + 3, z: index + 3 },
              ],
            ],
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorPolygonZMSHP(
  propertiesArray: Record<string, unknown>[],
): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'Polygon',
            is3D: true,
            bbox: [index, index, index, index],
            coordinates: [
              [
                { x: index, y: index, z: index, m: { value: index } },
                { x: index + 1, y: index + 1, z: index + 1, m: { value: index + 1 } },
              ],
              [
                { x: index + 2, y: index + 2, z: index + 2, m: { value: index + 2 } },
                { x: index + 3, y: index + 3, z: index + 3, m: { value: index + 3 } },
              ],
            ],
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorPolygonMSHP(
  propertiesArray: Record<string, unknown>[],
): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'Polygon',
            is3D: false,
            bbox: [index, index, index, index],
            coordinates: [
              [
                { x: index, y: index, m: { value: index } },
                { x: index + 1, y: index + 1, m: { value: index + 1 } },
              ],
              [
                { x: index + 2, y: index + 2, m: { value: index + 2 } },
                { x: index + 3, y: index + 3, m: { value: index + 3 } },
              ],
            ],
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorMultiPolygonSHP(
  propertiesArray: Record<string, unknown>[],
): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'MultiPolygon',
            is3D: false,
            bbox: [index, index, index, index],
            coordinates: [
              [
                [
                  { x: index, y: index },
                  { x: index + 1, y: index + 1 },
                ],
                [
                  { x: index + 2, y: index + 2 },
                  { x: index + 3, y: index + 3 },
                ],
              ],
              [
                [
                  { x: index + 4, y: index + 4 },
                  { x: index + 5, y: index + 5 },
                ],
                [
                  { x: index + 6, y: index + 6 },
                  { x: index + 7, y: index + 7 },
                ],
              ],
            ],
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorMultiPolygonZSHP(
  propertiesArray: Record<string, unknown>[],
): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'MultiPolygon',
            is3D: true,
            bbox: [index, index, index, index],
            coordinates: [
              [
                [
                  { x: index, y: index, z: index },
                  { x: index + 1, y: index + 1, z: index + 1 },
                ],
                [
                  { x: index + 2, y: index + 2, z: index + 2 },
                  { x: index + 3, y: index + 3, z: index + 3 },
                ],
              ],
              [
                [
                  { x: index + 4, y: index + 4, z: index + 4 },
                  { x: index + 5, y: index + 5, z: index + 5 },
                ],
                [
                  { x: index + 6, y: index + 6, z: index + 6 },
                  { x: index + 7, y: index + 7, z: index + 7 },
                ],
              ],
            ],
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorMultiPolygonZMSHP(
  propertiesArray: Record<string, unknown>[],
): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'MultiPolygon',
            is3D: true,
            bbox: [index, index, index, index],
            coordinates: [
              [
                [
                  { x: index, y: index, z: index, m: { value: index } },
                  { x: index + 1, y: index + 1, z: index + 1, m: { value: index + 1 } },
                ],
                [
                  { x: index + 2, y: index + 2, z: index + 2, m: { value: index + 2 } },
                  { x: index + 3, y: index + 3, z: index + 3, m: { value: index + 3 } },
                ],
              ],
              [
                [
                  { x: index + 4, y: index + 4, z: index + 4, m: { value: index + 4 } },
                  { x: index + 5, y: index + 5, z: index + 5, m: { value: index + 5 } },
                ],
                [
                  { x: index + 6, y: index + 6, z: index + 6, m: { value: index + 6 } },
                  { x: index + 7, y: index + 7, z: index + 7, m: { value: index + 7 } },
                ],
              ],
            ],
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

function createMockIteratorMultiPolygonMSHP(
  propertiesArray: Record<string, unknown>[],
): FeatureIterator {
  return {
    async *[Symbol.asyncIterator]() {
      let index = 0;
      for (const props of propertiesArray) {
        yield {
          type: 'VectorFeature',
          properties: props,
          geometry: {
            type: 'MultiPolygon',
            is3D: false,
            bbox: [index, index, index, index],
            coordinates: [
              [
                [
                  { x: index, y: index, m: { value: index } },
                  { x: index + 1, y: index + 1, m: { value: index + 1 } },
                ],
                [
                  { x: index + 2, y: index + 2, m: { value: index + 2 } },
                  { x: index + 3, y: index + 3, m: { value: index + 3 } },
                ],
              ],
              [
                [
                  { x: index + 4, y: index + 4, m: { value: index + 4 } },
                  { x: index + 5, y: index + 5, m: { value: index + 5 } },
                ],
                [
                  { x: index + 6, y: index + 6, m: { value: index + 6 } },
                  { x: index + 7, y: index + 7, m: { value: index + 7 } },
                ],
              ],
            ],
          },
        };
        index++;
      }
    },
  } as FeatureIterator;
}

// --- Test Assertions ---

describe('DBF Writer', () => {
  test('toDBFMeta tracks exact feature counts and schema fields', async () => {
    const iter1 = createMockIteratorPointSHP([
      { name: 'Raleigh', elevation: 110.5 },
      { name: 'Durham', elevation: 121 },
    ]);
    const iter2 = createMockIteratorPointSHP([{ name: 'Charlotte', population: 870000 }]);

    const [meta, featureCount] = await toDBFMeta([iter1, iter2]);

    expect(featureCount).toBe(3); // Confirms fix for the nested loop property counting bug
    expect(meta).toHaveLength(3); // fields: name, elevation, population

    const nameField = meta.find((f) => f.name === 'name');
    expect(nameField?.dataType).toBe('C');
  });

  test('toDBFMeta computes numeric decimal precision up to 15 decimals maximum', async () => {
    const iter = createMockIteratorPointSHP([
      { coords: 35.7796 }, // 4 decimals
      // oxlint-disable-next-line no-loss-of-precision
      { coords: -78.638212345678912 }, // 15+ decimals (Double precision floor)
      { coords: 12.1 }, // 1 decimal
    ]);

    const [meta] = await toDBFMeta([iter]);
    const coordsField = meta.find((f) => f.name === 'coords');

    expect(coordsField).toBeDefined();
    expect(coordsField?.dataType).toBe('N');
    expect(coordsField?.decimal).toBe(15); // Confirms it caps right at your max ceiling
    // len: 2 (integer '35') + 1 (dot) + 15 (decimals) + 1 (negative sign room safety)
    expect(coordsField?.len).toBeLessThanOrEqual(18);
  });

  test('toDBFMeta forces type widening up to String characters if data clashes', async () => {
    const iter = createMockIteratorPointSHP([
      { mixed: true },
      { mixed: 42.12 },
      { mixed: 'Fallback String Content' },
    ]);

    const [meta] = await toDBFMeta([iter]);
    const mixedField = meta.find((f) => f.name === 'mixed');

    expect(mixedField?.dataType).toBe('C'); // Widened safely out of Number/Boolean to prevent data drop
  });

  test('toDBF streams complete structural byte data without leaking previous row buffers', async () => {
    // Row 1 has a long name payload. Row 2 has a short name payload.
    // If memory flushing isn't running, 'Durham' will leak trailing chars from 'Asheville'
    const writer = new BufferWriter();

    await toDBF(writer, [createMockIteratorPointSHP([{ name: 'Asheville' }, { name: 'Durham' }])]);
    const finalBytes = await writer.commit();

    // The entire file must end explicitly with the standard EOF flag
    expect(finalBytes[finalBytes.length - 1]).toBe(0x1a);

    // Turn bytes back to string format to check text alignment
    const textDecoder = new TextDecoder();
    const rawStringOutput = textDecoder.decode(finalBytes);

    expect(rawStringOutput).toContain('Asheville ');
    expect(rawStringOutput).toContain('Durham    ');
    expect(rawStringOutput).not.toContain('Durhamville'); // Absolute buffer pollution safety confirmation
  });

  test('toDBF serializes logical boolean switches and strict ISO date strings', async () => {
    const writer = new BufferWriter();

    await toDBF(writer, [
      createMockIteratorPointSHP([
        { active: true, updated: new Date('2026-06-05') },
        { active: false, updated: new Date('2026-12-25') },
      ]),
    ]);

    const outputString = new TextDecoder().decode(writer.commit());

    // Assert boolean tokens conform to DBF spec
    expect(outputString).toContain('T');
    expect(outputString).toContain('F');
    // Assert dates fall back neatly into continuous standard string geometry structures
    expect(outputString).toContain('20260605');
    expect(outputString).toContain('20261225');
  });
});

describe('toSHP', () => {
  test('toSHP base case point', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorPointSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(156);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(2);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('Point');
    expect(features[0].geometry.coordinates).toEqual({ x: 0, y: 0 });
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('Point');
    expect(features[1].geometry.coordinates).toEqual({ x: 1, y: 1 });
  });

  test('toSHP base case point3D', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();
    const shxWriter = new BufferWriter();
    const prjWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorPointZSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
      shxWriter,
      prjWriter,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(172);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(2);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('Point');
    expect(features[0].geometry.coordinates).toEqual({ x: 0, y: 0, z: 0 });
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('Point');
    expect(features[1].geometry.coordinates).toEqual({ x: 1, y: 1, z: 1 });

    const shxData = shxWriter.commit();
    expect(shxData.length).toBe(116);
    const prjData = prjWriter.commit();
    expect(prjData.length).toBe(331);

    // convert prjData to string
    const textDecoder = new TextDecoder();
    const prjString = textDecoder.decode(prjData);
    expect(prjString).toEqual(`GEOGCS["WGS 84",
  DATUM["WGS_1984",
    SPHEROID["WGS 84",6378137,298.257223563,AUTHORITY["EPSG","7030"]],
    AUTHORITY["EPSG","6326"]],
  PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],
  UNIT["degree",0.0174532925199433,AUTHORITY["EPSG","9122"]],
  AXIS["Latitude",NORTH],
  AXIS["Longitude",EAST],
  AUTHORITY["EPSG","4326"]]
`);
  });

  test('toSHP base case point3DM', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorPointZMSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
      undefined,
      undefined,
      undefined,
      (m) => m?.value as number,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(188);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(2);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('Point');
    expect(features[0].geometry.coordinates).toEqual({ x: 0, y: 0, z: 0, m: { value: 0 } });
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('Point');
    expect(features[1].geometry.coordinates).toEqual({ x: 1, y: 1, z: 1, m: { value: 1 } });
  });

  test('toSHP base case pointM', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorPointMSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
      undefined,
      undefined,
      undefined,
      (m) => m?.value as number,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(172);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(2);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('Point');
    expect(features[0].geometry.coordinates).toEqual({ x: 0, y: 0, z: undefined, m: { value: 0 } });
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('Point');
    expect(features[1].geometry.coordinates).toEqual({ x: 1, y: 1, z: undefined, m: { value: 1 } });
  });

  test('toSHP base case larger', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [
        createMockIteratorPointSHP([
          { name: 'Asheville' },
          { name: 'Durham' },
          { name: 'Durham' },
          { name: 'Durham' },
          { name: 'Test5' },
        ]),
      ],
      dbfWriter,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(240);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(5);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('Point');
    expect(features[0].geometry.coordinates).toEqual({ x: 0, y: 0 });
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('Point');
    expect(features[1].geometry.coordinates).toEqual({ x: 1, y: 1 });
    expect(features[2].properties.name).toBe('Durham');
    expect(features[2].geometry.type).toBe('Point');
    expect(features[2].geometry.coordinates).toEqual({ x: 2, y: 2 });
    expect(features[3].properties.name).toBe('Durham');
    expect(features[3].geometry.type).toBe('Point');
    expect(features[3].geometry.coordinates).toEqual({ x: 3, y: 3 });
    expect(features[4].properties.name).toBe('Test5');
    expect(features[4].geometry.type).toBe('Point');
    expect(features[4].geometry.coordinates).toEqual({ x: 4, y: 4 });
  });

  test('toSHP base case multipoint', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorMultiPointSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(260);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(2);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('MultiPoint');
    expect(features[0].geometry.coordinates).toEqual([
      { x: 0, y: 0 },
      { x: 1, y: 1 },
    ]);
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('MultiPoint');
    expect(features[1].geometry.coordinates).toEqual([
      { x: 1, y: 1 },
      { x: 2, y: 2 },
    ]);
  });

  test('toSHP base case multipoint3D', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorMultiPointZSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(324);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(2);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('MultiPoint');
    expect(features[0].geometry.coordinates).toEqual([
      { x: 0, y: 0, z: 0 },
      { x: 1, y: 1, z: 1 },
    ]);
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('MultiPoint');
    expect(features[1].geometry.coordinates).toEqual([
      { x: 1, y: 1, z: 1 },
      { x: 2, y: 2, z: 2 },
    ]);
  });

  test('toSHP base case multipoint3DM', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorMultiPointZMSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
      undefined,
      undefined,
      undefined,
      (m) => m?.value as number,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(388);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(2);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('MultiPoint');
    expect(features[0].geometry.coordinates).toEqual([
      { x: 0, y: 0, z: 0, m: { value: 0 } },
      { x: 1, y: 1, z: 1, m: { value: 1 } },
    ]);
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('MultiPoint');
    expect(features[1].geometry.coordinates).toEqual([
      { x: 1, y: 1, z: 1, m: { value: 1 } },
      { x: 2, y: 2, z: 2, m: { value: 2 } },
    ]);
  });

  test('toSHP base case multipointM', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorMultiPointMSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
      undefined,
      undefined,
      undefined,
      (m) => m?.value as number,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(324);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(2);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('MultiPoint');
    expect(features[0].geometry.coordinates).toEqual([
      { x: 0, y: 0, z: undefined, m: { value: 0 } },
      { x: 1, y: 1, z: undefined, m: { value: 1 } },
    ]);
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('MultiPoint');
    expect(features[1].geometry.coordinates).toEqual([
      { x: 1, y: 1, z: undefined, m: { value: 1 } },
      { x: 2, y: 2, z: undefined, m: { value: 2 } },
    ]);
  });

  test('toSHP base case linestring', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorLineStringSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(276);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(2);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('LineString');
    expect(features[0].geometry.coordinates).toEqual([
      { x: 0, y: 0 },
      { x: 1, y: 1 },
    ]);
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('LineString');
    expect(features[1].geometry.coordinates).toEqual([
      { x: 1, y: 1 },
      { x: 2, y: 2 },
    ]);
  });

  test('toSHP base case linestring3D', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorLineStringZSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(340);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(2);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('LineString');
    expect(features[0].geometry.coordinates).toEqual([
      { x: 0, y: 0, z: 0 },
      { x: 1, y: 1, z: 1 },
    ]);
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('LineString');
    expect(features[1].geometry.coordinates).toEqual([
      { x: 1, y: 1, z: 1 },
      { x: 2, y: 2, z: 2 },
    ]);
  });

  test('toSHP base case linestring3DM', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorLineStringZMSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
      undefined,
      undefined,
      undefined,
      (m) => m?.value as number,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(404);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(2);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('LineString');
    expect(features[0].geometry.coordinates).toEqual([
      { x: 0, y: 0, z: 0, m: { value: 0 } },
      { x: 1, y: 1, z: 1, m: { value: 1 } },
    ]);
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('LineString');
    expect(features[1].geometry.coordinates).toEqual([
      { x: 1, y: 1, z: 1, m: { value: 1 } },
      { x: 2, y: 2, z: 2, m: { value: 2 } },
    ]);
  });

  test('toSHP base case linestringM', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorLineStringMSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
      undefined,
      undefined,
      undefined,
      (m) => m?.value as number,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(340);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(2);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('LineString');
    expect(features[0].geometry.coordinates).toEqual([
      { x: 0, y: 0, z: undefined, m: { value: 0 } },
      { x: 1, y: 1, z: undefined, m: { value: 1 } },
    ]);
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('LineString');
    expect(features[1].geometry.coordinates).toEqual([
      { x: 1, y: 1, z: undefined, m: { value: 1 } },
      { x: 2, y: 2, z: undefined, m: { value: 2 } },
    ]);
  });

  test('toSHP base case multilinestring', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorMultiLineStringSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(348);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(2);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('MultiLineString');
    expect(features[0].geometry.coordinates).toEqual([
      [
        { x: 0, y: 0 },
        { x: 1, y: 1 },
      ],
      [
        { x: 2, y: 2 },
        { x: 3, y: 3 },
      ],
    ]);
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('MultiLineString');
    expect(features[1].geometry.coordinates).toEqual([
      [
        { x: 1, y: 1 },
        { x: 2, y: 2 },
      ],
      [
        { x: 3, y: 3 },
        { x: 4, y: 4 },
      ],
    ]);
  });

  test('toSHP base case multilinestring3D', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorMultiLineStringZSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(444);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(2);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('MultiLineString');
    expect(features[0].geometry.coordinates).toEqual([
      [
        { x: 0, y: 0, z: 0 },
        { x: 1, y: 1, z: 1 },
      ],
      [
        { x: 2, y: 2, z: 2 },
        { x: 3, y: 3, z: 3 },
      ],
    ]);
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('MultiLineString');
    expect(features[1].geometry.coordinates).toEqual([
      [
        { x: 1, y: 1, z: 1 },
        { x: 2, y: 2, z: 2 },
      ],
      [
        { x: 3, y: 3, z: 3 },
        { x: 4, y: 4, z: 4 },
      ],
    ]);
  });

  test('toSHP base case multilinestring3DM', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorMultiLineStringZMSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
      undefined,
      undefined,
      undefined,
      (m) => m?.value as number,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(540);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(2);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('MultiLineString');
    expect(features[0].geometry.coordinates).toEqual([
      [
        { x: 0, y: 0, z: 0, m: { value: 0 } },
        { x: 1, y: 1, z: 1, m: { value: 1 } },
      ],
      [
        { x: 2, y: 2, z: 2, m: { value: 2 } },
        { x: 3, y: 3, z: 3, m: { value: 3 } },
      ],
    ]);
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('MultiLineString');
    expect(features[1].geometry.coordinates).toEqual([
      [
        { x: 1, y: 1, z: 1, m: { value: 1 } },
        { x: 2, y: 2, z: 2, m: { value: 2 } },
      ],
      [
        { x: 3, y: 3, z: 3, m: { value: 3 } },
        { x: 4, y: 4, z: 4, m: { value: 4 } },
      ],
    ]);
  });

  test('toSHP base case multilinestringM', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorMultiLineStringMSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
      undefined,
      undefined,
      undefined,
      (m) => m?.value as number,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(444);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(2);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('MultiLineString');
    expect(features[0].geometry.coordinates).toEqual([
      [
        { x: 0, y: 0, z: undefined, m: { value: 0 } },
        { x: 1, y: 1, z: undefined, m: { value: 1 } },
      ],
      [
        { x: 2, y: 2, z: undefined, m: { value: 2 } },
        { x: 3, y: 3, z: undefined, m: { value: 3 } },
      ],
    ]);
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('MultiLineString');
    expect(features[1].geometry.coordinates).toEqual([
      [
        { x: 1, y: 1, z: undefined, m: { value: 1 } },
        { x: 2, y: 2, z: undefined, m: { value: 2 } },
      ],
      [
        { x: 3, y: 3, z: undefined, m: { value: 3 } },
        { x: 4, y: 4, z: undefined, m: { value: 4 } },
      ],
    ]);
  });

  test('toSHP base case polygon', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorPolygonSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(348);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(2);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('Polygon');
    expect(features[0].geometry.coordinates).toEqual([
      [
        { x: 0, y: 0 },
        { x: 1, y: 1 },
      ],
      [
        { x: 2, y: 2 },
        { x: 3, y: 3 },
      ],
    ]);
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('Polygon');
    expect(features[1].geometry.coordinates).toEqual([
      [
        { x: 1, y: 1 },
        { x: 2, y: 2 },
      ],
      [
        { x: 3, y: 3 },
        { x: 4, y: 4 },
      ],
    ]);
  });

  test('toSHP base case polygon3D', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorPolygonZSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(444);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(2);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('Polygon');
    expect(features[0].geometry.coordinates).toEqual([
      [
        { x: 0, y: 0, z: 0 },
        { x: 1, y: 1, z: 1 },
      ],
      [
        { x: 2, y: 2, z: 2 },
        { x: 3, y: 3, z: 3 },
      ],
    ]);
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('Polygon');
    expect(features[1].geometry.coordinates).toEqual([
      [
        { x: 1, y: 1, z: 1 },
        { x: 2, y: 2, z: 2 },
      ],
      [
        { x: 3, y: 3, z: 3 },
        { x: 4, y: 4, z: 4 },
      ],
    ]);
  });

  test('toSHP base case polygon3DM', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorPolygonZMSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
      undefined,
      undefined,
      undefined,
      (m) => m?.value as number,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(540);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(2);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('Polygon');
    expect(features[0].geometry.coordinates).toEqual([
      [
        { x: 0, y: 0, z: 0, m: { value: 0 } },
        { x: 1, y: 1, z: 1, m: { value: 1 } },
      ],
      [
        { x: 2, y: 2, z: 2, m: { value: 2 } },
        { x: 3, y: 3, z: 3, m: { value: 3 } },
      ],
    ]);
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('Polygon');
    expect(features[1].geometry.coordinates).toEqual([
      [
        { x: 1, y: 1, z: 1, m: { value: 1 } },
        { x: 2, y: 2, z: 2, m: { value: 2 } },
      ],
      [
        { x: 3, y: 3, z: 3, m: { value: 3 } },
        { x: 4, y: 4, z: 4, m: { value: 4 } },
      ],
    ]);
  });

  test('toSHP base case polygonM', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorPolygonMSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
      undefined,
      undefined,
      undefined,
      (m) => m?.value as number,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(444);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(2);
    expect(features[0].properties.name).toBe('Asheville');
    expect(features[0].geometry.type).toBe('Polygon');
    expect(features[0].geometry.coordinates).toEqual([
      [
        { x: 0, y: 0, z: undefined, m: { value: 0 } },
        { x: 1, y: 1, z: undefined, m: { value: 1 } },
      ],
      [
        { x: 2, y: 2, z: undefined, m: { value: 2 } },
        { x: 3, y: 3, z: undefined, m: { value: 3 } },
      ],
    ]);
    expect(features[1].properties.name).toBe('Durham');
    expect(features[1].geometry.type).toBe('Polygon');
    expect(features[1].geometry.coordinates).toEqual([
      [
        { x: 1, y: 1, z: undefined, m: { value: 1 } },
        { x: 2, y: 2, z: undefined, m: { value: 2 } },
      ],
      [
        { x: 3, y: 3, z: undefined, m: { value: 3 } },
        { x: 4, y: 4, z: undefined, m: { value: 4 } },
      ],
    ]);
  });

  test('toSHP base case multipolygon', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorMultiPolygonSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(596);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(4);
    expect(features[0].properties.name).toEqual('Asheville');
    expect(features[0].geometry.type).toEqual('Polygon');
    expect(features[0].geometry.coordinates).toEqual([
      [
        { x: 0, y: 0 },
        { x: 1, y: 1 },
      ],
      [
        { x: 2, y: 2 },
        { x: 3, y: 3 },
      ],
    ]);
    expect(features[1].properties.name).toEqual('Asheville');
    expect(features[1].geometry.type).toEqual('Polygon');
    expect(features[1].geometry.coordinates).toEqual([
      [
        { x: 4, y: 4 },
        { x: 5, y: 5 },
      ],
      [
        { x: 6, y: 6 },
        { x: 7, y: 7 },
      ],
    ]);
    expect(features[2].properties.name).toEqual('Durham');
    expect(features[2].geometry.type).toEqual('Polygon');
    expect(features[2].geometry.coordinates).toEqual([
      [
        { x: 1, y: 1 },
        { x: 2, y: 2 },
      ],
      [
        { x: 3, y: 3 },
        { x: 4, y: 4 },
      ],
    ]);
    expect(features[3].properties.name).toEqual('Durham');
    expect(features[3].geometry.type).toEqual('Polygon');
    expect(features[3].geometry.coordinates).toEqual([
      [
        { x: 5, y: 5 },
        { x: 6, y: 6 },
      ],
      [
        { x: 7, y: 7 },
        { x: 8, y: 8 },
      ],
    ]);
  });

  test('toSHP base case multipolygon3D', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorMultiPolygonZSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(788);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(4);
    expect(features[0].properties.name).toEqual('Asheville');
    expect(features[0].geometry.type).toEqual('Polygon');
    expect(features[0].geometry.coordinates).toEqual([
      [
        { x: 0, y: 0, z: 0 },
        { x: 1, y: 1, z: 1 },
      ],
      [
        { x: 2, y: 2, z: 2 },
        { x: 3, y: 3, z: 3 },
      ],
    ]);
    expect(features[1].properties.name).toEqual('Asheville');
    expect(features[1].geometry.type).toEqual('Polygon');
    expect(features[1].geometry.coordinates).toEqual([
      [
        { x: 4, y: 4, z: 4 },
        { x: 5, y: 5, z: 5 },
      ],
      [
        { x: 6, y: 6, z: 6 },
        { x: 7, y: 7, z: 7 },
      ],
    ]);
    expect(features[2].properties.name).toEqual('Durham');
    expect(features[2].geometry.type).toEqual('Polygon');
    expect(features[2].geometry.coordinates).toEqual([
      [
        { x: 1, y: 1, z: 1 },
        { x: 2, y: 2, z: 2 },
      ],
      [
        { x: 3, y: 3, z: 3 },
        { x: 4, y: 4, z: 4 },
      ],
    ]);
    expect(features[3].properties.name).toEqual('Durham');
    expect(features[3].geometry.type).toEqual('Polygon');
    expect(features[3].geometry.coordinates).toEqual([
      [
        { x: 5, y: 5, z: 5 },
        { x: 6, y: 6, z: 6 },
      ],
      [
        { x: 7, y: 7, z: 7 },
        { x: 8, y: 8, z: 8 },
      ],
    ]);
  });

  test('toSHP base case multipolygon3DM', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorMultiPolygonZMSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
      undefined,
      undefined,
      undefined,
      (m) => m?.value as number,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(980);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(4);
    expect(features[0].properties.name).toEqual('Asheville');
    expect(features[0].geometry.type).toEqual('Polygon');
    expect(features[0].geometry.coordinates).toEqual([
      [
        { x: 0, y: 0, z: 0, m: { value: 0 } },
        { x: 1, y: 1, z: 1, m: { value: 1 } },
      ],
      [
        { x: 2, y: 2, z: 2, m: { value: 2 } },
        { x: 3, y: 3, z: 3, m: { value: 3 } },
      ],
    ]);
    expect(features[1].properties.name).toEqual('Asheville');
    expect(features[1].geometry.type).toEqual('Polygon');
    expect(features[1].geometry.coordinates).toEqual([
      [
        { x: 4, y: 4, z: 4, m: { value: 4 } },
        { x: 5, y: 5, z: 5, m: { value: 5 } },
      ],
      [
        { x: 6, y: 6, z: 6, m: { value: 6 } },
        { x: 7, y: 7, z: 7, m: { value: 7 } },
      ],
    ]);
    expect(features[2].properties.name).toEqual('Durham');
    expect(features[2].geometry.type).toEqual('Polygon');
    expect(features[2].geometry.coordinates).toEqual([
      [
        { x: 1, y: 1, z: 1, m: { value: 1 } },
        { x: 2, y: 2, z: 2, m: { value: 2 } },
      ],
      [
        { x: 3, y: 3, z: 3, m: { value: 3 } },
        { x: 4, y: 4, z: 4, m: { value: 4 } },
      ],
    ]);
    expect(features[3].properties.name).toEqual('Durham');
    expect(features[3].geometry.type).toEqual('Polygon');
    expect(features[3].geometry.coordinates).toEqual([
      [
        { x: 5, y: 5, z: 5, m: { value: 5 } },
        { x: 6, y: 6, z: 6, m: { value: 6 } },
      ],
      [
        { x: 7, y: 7, z: 7, m: { value: 7 } },
        { x: 8, y: 8, z: 8, m: { value: 8 } },
      ],
    ]);
  });

  test('toSHP base case multipolygonM', async () => {
    const shpWriter = new BufferWriter();
    const dbfWriter = new BufferWriter();

    await toSHP(
      shpWriter,
      [createMockIteratorMultiPolygonMSHP([{ name: 'Asheville' }, { name: 'Durham' }])],
      dbfWriter,
      undefined,
      undefined,
      undefined,
      (m) => m?.value as number,
    );
    const shpData = shpWriter.commit();
    expect(shpData.length).toBe(788);

    const zippedData = await zipFolder([
      { name: 'points.shp', comment: 'shapefile data', data: shpWriter.commit() },
      { name: 'points.dbf', comment: 'properties data', data: dbfWriter.commit() },
    ]);

    const reader = await shapefileFromGzip(zippedData.buffer);
    const features = await Array.fromAsync(reader);

    expect(features).toHaveLength(4);
    expect(features[0].properties.name).toEqual('Asheville');
    expect(features[0].geometry.type).toEqual('Polygon');
    expect(features[0].geometry.coordinates).toEqual([
      [
        { x: 0, y: 0, z: undefined, m: { value: 0 } },
        { x: 1, y: 1, z: undefined, m: { value: 1 } },
      ],
      [
        { x: 2, y: 2, z: undefined, m: { value: 2 } },
        { x: 3, y: 3, z: undefined, m: { value: 3 } },
      ],
    ]);
    expect(features[1].properties.name).toEqual('Asheville');
    expect(features[1].geometry.type).toEqual('Polygon');
    expect(features[1].geometry.coordinates).toEqual([
      [
        { x: 4, y: 4, z: undefined, m: { value: 4 } },
        { x: 5, y: 5, z: undefined, m: { value: 5 } },
      ],
      [
        { x: 6, y: 6, z: undefined, m: { value: 6 } },
        { x: 7, y: 7, z: undefined, m: { value: 7 } },
      ],
    ]);
    expect(features[2].properties.name).toEqual('Durham');
    expect(features[2].geometry.type).toEqual('Polygon');
    expect(features[2].geometry.coordinates).toEqual([
      [
        { x: 1, y: 1, z: undefined, m: { value: 1 } },
        { x: 2, y: 2, z: undefined, m: { value: 2 } },
      ],
      [
        { x: 3, y: 3, z: undefined, m: { value: 3 } },
        { x: 4, y: 4, z: undefined, m: { value: 4 } },
      ],
    ]);
    expect(features[3].properties.name).toEqual('Durham');
    expect(features[3].geometry.type).toEqual('Polygon');
    expect(features[3].geometry.coordinates).toEqual([
      [
        { x: 5, y: 5, z: undefined, m: { value: 5 } },
        { x: 6, y: 6, z: undefined, m: { value: 6 } },
      ],
      [
        { x: 7, y: 7, z: undefined, m: { value: 7 } },
        { x: 8, y: 8, z: undefined, m: { value: 8 } },
      ],
    ]);
  });
});
