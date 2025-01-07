import { FileReader } from '../../../src/file';
import { BufferReader, CSVReader } from '../../../src';
import { expect, test } from 'bun:test';

test('CSVReader - basic', async () => {
  const file = await Bun.file(`${__dirname}/fixtures/basic.csv`).arrayBuffer();
  const buffer = Buffer.from(file) as Buffer;
  const reader = new BufferReader(buffer.buffer, 0, buffer.byteLength);
  const csvReader = new CSVReader(reader);
  const data = await Array.fromAsync(csvReader);
  expect(data).toEqual([
    {
      geometry: {
        coordinates: { x: 2, y: 1 },
        is3D: false,
        type: 'Point',
      },
      properties: {
        name: '3',
      },
      type: 'VectorFeature',
    },
    {
      geometry: {
        coordinates: { x: 1.1, y: 3.2 },
        is3D: false,
        type: 'Point',
      },
      properties: {
        name: 'a',
      },
      type: 'VectorFeature',
    },
  ]);
});

test('CSVReader - 3D', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/basic3D.csv`);
  const csvReader = new CSVReader(fileReader, {
    delimiter: ',',
    lineDelimiter: '\n',
    lonKey: 'Longitude',
    latKey: 'Latitude',
    heightKey: 'height',
  });
  const data = await Array.fromAsync(csvReader);
  expect(data).toEqual([
    {
      geometry: {
        coordinates: { x: 2, y: 1, z: 55 },
        is3D: true,
        type: 'Point',
      },
      properties: {
        name: '3',
      },
      type: 'VectorFeature',
    },
    {
      geometry: {
        coordinates: { x: 1.1, y: 3.2, z: -2.2 },
        is3D: true,
        type: 'Point',
      },
      properties: {
        name: 'a',
      },
      type: 'VectorFeature',
    },
  ]);
});
