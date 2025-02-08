import { FileReader } from '../../../src/file';
import {
  BufferJSONReader,
  BufferReader,
  JSONReader,
  NewLineDelimitedJSONReader,
  SequenceJSONReader,
} from '../../../src';
import { expect, test } from 'bun:test';

test('BufferJSONReader - string', async () => {
  const file = await Bun.file(`${__dirname}/fixtures/points.geojson`).arrayBuffer();
  const buffer = Buffer.from(file);
  const reader = new BufferJSONReader(buffer.toString('utf-8'));
  const data = await Array.fromAsync(reader);
  expect(data).toEqual([
    {
      geometry: {
        bbox: [144.9584, -37.8173, 144.9584, -37.8173],
        coordinates: { x: 144.9584, y: -37.8173 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Melbourne' },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [149.1009, -35.3039, 149.1009, -35.3039],
        coordinates: { x: 149.1009, y: -35.3039 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Canberra' },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [151.2144, -33.8766, 151.2144, -33.8766],
        coordinates: { x: 151.2144, y: -33.8766 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Sydney' },
      type: 'VectorFeature',
    },
  ]);
});

test('BufferJSONReader - object', async () => {
  const json = await Bun.file(`${__dirname}/fixtures/points.geojson`).json();
  const reader = new BufferJSONReader(json);
  const data = await Array.fromAsync(reader);
  expect(data).toEqual([
    {
      geometry: {
        bbox: [144.9584, -37.8173, 144.9584, -37.8173],
        coordinates: { x: 144.9584, y: -37.8173 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Melbourne' },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [149.1009, -35.3039, 149.1009, -35.3039],
        coordinates: { x: 149.1009, y: -35.3039 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Canberra' },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [151.2144, -33.8766, 151.2144, -33.8766],
        coordinates: { x: 151.2144, y: -33.8766 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Sydney' },
      type: 'VectorFeature',
    },
  ]);
});

test('NewLineDelimitedJSONReader - BufferReader', async () => {
  const fileBuf = await Bun.file(`${__dirname}/fixtures/points.geojsonld`).arrayBuffer();
  const bufReader = new BufferReader(fileBuf);
  const ldReader = new NewLineDelimitedJSONReader(bufReader);
  const data = await Array.fromAsync(ldReader);
  expect(data).toEqual([
    {
      geometry: {
        bbox: [144.9584, -37.8173, 144.9584, -37.8173],
        coordinates: { x: 144.9584, y: -37.8173 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Melbourne' },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [149.1009, -35.3039, 149.1009, -35.3039],
        coordinates: { x: 149.1009, y: -35.3039 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Canberra' },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [151.2144, -33.8766, 151.2144, -33.8766],
        coordinates: { x: 151.2144, y: -33.8766 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Sydney' },
      type: 'VectorFeature',
    },
  ]);
});

test('NewLineDelimitedJSONReader - FileReader', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/points.geojsonld`);
  const ldReader = new NewLineDelimitedJSONReader(fileReader);
  const data = await Array.fromAsync(ldReader);
  fileReader.close();
  expect(data).toEqual([
    {
      geometry: {
        bbox: [144.9584, -37.8173, 144.9584, -37.8173],
        coordinates: { x: 144.9584, y: -37.8173 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Melbourne' },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [149.1009, -35.3039, 149.1009, -35.3039],
        coordinates: { x: 149.1009, y: -35.3039 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Canberra' },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [151.2144, -33.8766, 151.2144, -33.8766],
        coordinates: { x: 151.2144, y: -33.8766 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Sydney' },
      type: 'VectorFeature',
    },
  ]);
});

test('JSONReader - BufferReader', async () => {
  const fileBuf = await Bun.file(`${__dirname}/fixtures/points.geojson`).arrayBuffer();
  const bufReader = new BufferReader(fileBuf);
  const reader = new JSONReader(bufReader);
  const data = await Array.fromAsync(reader);
  expect(data).toEqual([
    {
      geometry: {
        bbox: [144.9584, -37.8173, 144.9584, -37.8173],
        coordinates: { x: 144.9584, y: -37.8173 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Melbourne' },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [149.1009, -35.3039, 149.1009, -35.3039],
        coordinates: { x: 149.1009, y: -35.3039 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Canberra' },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [151.2144, -33.8766, 151.2144, -33.8766],
        coordinates: { x: 151.2144, y: -33.8766 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Sydney' },
      type: 'VectorFeature',
    },
  ]);
});

test('JSONReader - BufferReader (forced "large" read)', async () => {
  const fileBuf = await Bun.file(`${__dirname}/fixtures/points.geojson`).arrayBuffer();
  const bufReader = new BufferReader(fileBuf);
  const reader = new JSONReader(bufReader, 1);
  const data = await Array.fromAsync(reader);
  expect(data).toEqual([
    {
      geometry: {
        bbox: [144.9584, -37.8173, 144.9584, -37.8173],
        coordinates: { x: 144.9584, y: -37.8173 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Melbourne' },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [149.1009, -35.3039, 149.1009, -35.3039],
        coordinates: { x: 149.1009, y: -35.3039 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Canberra' },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [151.2144, -33.8766, 151.2144, -33.8766],
        coordinates: { x: 151.2144, y: -33.8766 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Sydney' },
      type: 'VectorFeature',
    },
  ]);
});

test('JSONReader - FileReader', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/points.geojson`);
  const reader = new JSONReader(fileReader);
  const data = await Array.fromAsync(reader);
  fileReader.close();
  expect(data).toEqual([
    {
      geometry: {
        bbox: [144.9584, -37.8173, 144.9584, -37.8173],
        coordinates: { x: 144.9584, y: -37.8173 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Melbourne' },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [149.1009, -35.3039, 149.1009, -35.3039],
        coordinates: { x: 149.1009, y: -35.3039 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Canberra' },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [151.2144, -33.8766, 151.2144, -33.8766],
        coordinates: { x: 151.2144, y: -33.8766 },
        is3D: false,
        type: 'Point',
      },
      properties: { name: 'Sydney' },
      type: 'VectorFeature',
    },
  ]);
});

test('JSONReader - BufferReader', async () => {
  const fileBuf = await Bun.file(`${__dirname}/fixtures/point-feature.geojson`).arrayBuffer();
  const bufReader = new BufferReader(fileBuf);
  const reader = new JSONReader(bufReader);
  const data = await Array.fromAsync(reader);
  expect(data).toEqual([
    {
      geometry: {
        bbox: [144.9584, -37.8173, 144.9584, -37.8173],
        coordinates: { x: 144.9584, y: -37.8173 },
        type: 'Point',
        is3D: false,
      },
      properties: { name: 'Melbourne' },
      type: 'VectorFeature',
    },
  ]);
});

test('SequenceJSONReader - BufferReader', async () => {
  const reader = new FileReader(`${__dirname}/fixtures/features.geojsonseq`);
  const seqReader = new SequenceJSONReader(reader);
  const data = await Array.fromAsync(seqReader);
  expect(data).toEqual([
    {
      geometry: {
        bbox: [102, 0.5, 102, 0.5],
        coordinates: {
          x: 102,
          y: 0.5,
        },
        is3D: false,
        type: 'Point',
      },
      properties: {
        prop0: 'value0',
      },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [102, 0, 105, 1],
        coordinates: [
          { x: 102, y: 0 },
          { x: 103, y: 1 },
          { x: 104, y: 0 },
          { x: 105, y: 1 },
        ],
        is3D: false,
        type: 'LineString',
      },
      properties: {
        prop0: 'value0',
        prop1: 0,
      },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [100, 0, 101, 1],
        coordinates: [
          [
            { x: 100, y: 0 },
            { x: 101, y: 0 },
            { x: 101, y: 1 },
            { x: 100, y: 1 },
            { x: 100, y: 0 },
          ],
        ],
        is3D: false,
        type: 'Polygon',
      },
      properties: {
        prop0: 'value0',
        prop1: {
          this: 'that',
        },
      },
      type: 'VectorFeature',
    },
  ]);
});
