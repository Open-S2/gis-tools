import { BufferReader } from '../../../src/readers';
import FileReader from '../../../src/readers/fileReader';
import {
  BufferJSONReader,
  JSONReader,
  NewLineDelimitedJSONReader,
} from '../../../src/readers/json';
import { expect, test } from 'bun:test';

test('BufferJSONReader - string', async () => {
  const file = await Bun.file('tests/readers/fixtures/points.geojson').arrayBuffer();
  const buffer = Buffer.from(file);
  const reader = new BufferJSONReader(buffer.toString('utf-8'));
  const data = [...reader.iterate()];
  expect(data).toEqual([
    {
      geometry: { coordinates: [144.9584, -37.8173], type: 'Point' },
      properties: { name: 'Melbourne' },
      type: 'Feature',
    },
    {
      geometry: { coordinates: [149.1009, -35.3039], type: 'Point' },
      properties: { name: 'Canberra' },
      type: 'Feature',
    },
    {
      geometry: { coordinates: [151.2144, -33.8766], type: 'Point' },
      properties: { name: 'Sydney' },
      type: 'Feature',
    },
  ]);
});

test('BufferJSONReader - object', async () => {
  const json = await Bun.file('tests/readers/fixtures/points.geojson').json();
  const reader = new BufferJSONReader(json);
  const data = [...reader.iterate()];
  expect(data).toEqual([
    {
      geometry: { coordinates: [144.9584, -37.8173], type: 'Point' },
      properties: { name: 'Melbourne' },
      type: 'Feature',
    },
    {
      geometry: { coordinates: [149.1009, -35.3039], type: 'Point' },
      properties: { name: 'Canberra' },
      type: 'Feature',
    },
    {
      geometry: { coordinates: [151.2144, -33.8766], type: 'Point' },
      properties: { name: 'Sydney' },
      type: 'Feature',
    },
  ]);
});

test('NewLineDelimitedJSONReader - BufferReader', async () => {
  const fileBuf = await Bun.file('tests/readers/fixtures/points.geojsonld').arrayBuffer();
  const bufReader = new BufferReader(fileBuf);
  const ldReader = new NewLineDelimitedJSONReader(bufReader);
  const data = [...ldReader.iterate()];
  expect(data).toEqual([
    {
      geometry: { coordinates: [144.9584, -37.8173], type: 'Point' },
      properties: { name: 'Melbourne' },
      type: 'Feature',
    },
    {
      geometry: { coordinates: [149.1009, -35.3039], type: 'Point' },
      properties: { name: 'Canberra' },
      type: 'Feature',
    },
    {
      geometry: { coordinates: [151.2144, -33.8766], type: 'Point' },
      properties: { name: 'Sydney' },
      type: 'Feature',
    },
  ]);
});

test('NewLineDelimitedJSONReader - FileReader', async () => {
  const fileReader = new FileReader('tests/readers/fixtures/points.geojsonld');
  const ldReader = new NewLineDelimitedJSONReader(fileReader);
  const data = [...ldReader.iterate()];
  fileReader.close();
  expect(data).toEqual([
    {
      geometry: { coordinates: [144.9584, -37.8173], type: 'Point' },
      properties: { name: 'Melbourne' },
      type: 'Feature',
    },
    {
      geometry: { coordinates: [149.1009, -35.3039], type: 'Point' },
      properties: { name: 'Canberra' },
      type: 'Feature',
    },
    {
      geometry: { coordinates: [151.2144, -33.8766], type: 'Point' },
      properties: { name: 'Sydney' },
      type: 'Feature',
    },
  ]);
});

test('JSONReader - BufferReader', async () => {
  const fileBuf = await Bun.file('tests/readers/fixtures/points.geojson').arrayBuffer();
  const bufReader = new BufferReader(fileBuf);
  const reader = new JSONReader(bufReader);
  const data = [...reader.iterate()];
  expect(data).toEqual([
    {
      geometry: { coordinates: [144.9584, -37.8173], type: 'Point' },
      properties: { name: 'Melbourne' },
      type: 'Feature',
    },
    {
      geometry: { coordinates: [149.1009, -35.3039], type: 'Point' },
      properties: { name: 'Canberra' },
      type: 'Feature',
    },
    {
      geometry: { coordinates: [151.2144, -33.8766], type: 'Point' },
      properties: { name: 'Sydney' },
      type: 'Feature',
    },
  ]);
});

test('JSONReader - BufferReader (forced "large" read)', async () => {
  const fileBuf = await Bun.file('tests/readers/fixtures/points.geojson').arrayBuffer();
  const bufReader = new BufferReader(fileBuf);
  const reader = new JSONReader(bufReader, 1);
  const data = [...reader.iterate()];
  expect(data).toEqual([
    {
      geometry: { coordinates: [144.9584, -37.8173], type: 'Point' },
      properties: { name: 'Melbourne' },
      type: 'Feature',
    },
    {
      geometry: { coordinates: [149.1009, -35.3039], type: 'Point' },
      properties: { name: 'Canberra' },
      type: 'Feature',
    },
    {
      geometry: { coordinates: [151.2144, -33.8766], type: 'Point' },
      properties: { name: 'Sydney' },
      type: 'Feature',
    },
  ]);
});

test('JSONReader - FileReader', async () => {
  const fileReader = new FileReader('tests/readers/fixtures/points.geojson');
  const reader = new JSONReader(fileReader);
  const data = [...reader.iterate()];
  fileReader.close();
  expect(data).toEqual([
    {
      geometry: { coordinates: [144.9584, -37.8173], type: 'Point' },
      properties: { name: 'Melbourne' },
      type: 'Feature',
    },
    {
      geometry: { coordinates: [149.1009, -35.3039], type: 'Point' },
      properties: { name: 'Canberra' },
      type: 'Feature',
    },
    {
      geometry: { coordinates: [151.2144, -33.8766], type: 'Point' },
      properties: { name: 'Sydney' },
      type: 'Feature',
    },
  ]);
});

test('JSONReader - BufferReader', async () => {
  const fileBuf = await Bun.file('tests/readers/fixtures/point-feature.geojson').arrayBuffer();
  const bufReader = new BufferReader(fileBuf);
  const reader = new JSONReader(bufReader);
  const data = [...reader.iterate()];
  expect(data).toEqual([
    {
      geometry: { coordinates: [144.9584, -37.8173], type: 'Point' },
      properties: { name: 'Melbourne' },
      type: 'Feature',
    },
  ]);
});
