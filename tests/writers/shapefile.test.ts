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
  test('toSHP base case', async () => {
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
});
