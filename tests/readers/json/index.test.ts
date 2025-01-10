import { FileReader } from '../../../src/file';
import {
  BufferJSONReader,
  BufferReader,
  JSONReader,
  NewLineDelimitedJSONReader,
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
      id: undefined,
      metadata: undefined,
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
      id: undefined,
      metadata: undefined,
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
      id: undefined,
      metadata: undefined,
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
      id: undefined,
      metadata: undefined,
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
      id: undefined,
      metadata: undefined,
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
      id: undefined,
      metadata: undefined,
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
      id: undefined,
      metadata: undefined,
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
      id: undefined,
      metadata: undefined,
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
      id: undefined,
      metadata: undefined,
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
      id: undefined,
      metadata: undefined,
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
      id: undefined,
      metadata: undefined,
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
      id: undefined,
      metadata: undefined,
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
      id: undefined,
      metadata: undefined,
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
      id: undefined,
      metadata: undefined,
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
      id: undefined,
      metadata: undefined,
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
      id: undefined,
      metadata: undefined,
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
      id: undefined,
      metadata: undefined,
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
      id: undefined,
      metadata: undefined,
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
      id: undefined,
      metadata: undefined,
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
      id: undefined,
      metadata: undefined,
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
      id: undefined,
      metadata: undefined,
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
