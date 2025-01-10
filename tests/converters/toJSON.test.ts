import { FileReader } from '../../src/file';
import { BufferReader, BufferWriter, JSONReader, NewLineDelimitedJSONReader } from '../../src';
import { expect, test } from 'bun:test';
import { toJSON, toJSONLD } from '../../src/converters';

import type { VectorFeatures } from '../../src/geometry';

test('toJSON', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/points.geojson`);
  const jsonReader = new JSONReader(fileReader);
  const bufWriter = new BufferWriter();
  await toJSON(bufWriter, [jsonReader]);
  const string = new TextDecoder().decode(bufWriter.commit());
  expect(string).toEqual(
    '{\n\t"type": "S2FeatureCollection",\n\t"features": [\n\t\t{"type":"S2Feature","face":3,"properties":{"name":"Melbourne"},"geometry":{"type":"Point","is3D":false,"coordinates":{"x":0.980307055282927,"y":0.1191097721694171},"bbox":[144.9584,-37.8173,144.9584,-37.8173],"vecBBox":[0.980307055282927,0.1191097721694171,0.980307055282927,0.1191097721694171]}},\n\t\t{"type":"S2Feature","face":3,"properties":{"name":"Canberra"},"geometry":{"type":"Point","is3D":false,"coordinates":{"x":0.9321761149504832,"y":0.16402766817497416},"bbox":[149.1009,-35.3039,149.1009,-35.3039],"vecBBox":[0.9321761149504832,0.16402766817497416,0.9321761149504832,0.16402766817497416]}},\n\t\t{"type":"S2Feature","face":3,"properties":{"name":"Sydney"},"geometry":{"type":"Point","is3D":false,"coordinates":{"x":0.908036698755368,"y":0.18632281680962381},"bbox":[151.2144,-33.8766,151.2144,-33.8766],"vecBBox":[0.908036698755368,0.18632281680962381,0.908036698755368,0.18632281680962381]}}\n\t],\n\t"faces": [3]\n}',
  );
  expect(JSON.parse(string)).toEqual({
    faces: [3],
    features: [
      {
        face: 3,
        geometry: {
          bbox: [144.9584, -37.8173, 144.9584, -37.8173],
          coordinates: { x: 0.980307055282927, y: 0.1191097721694171 },
          is3D: false,
          type: 'Point',
          vecBBox: [0.980307055282927, 0.1191097721694171, 0.980307055282927, 0.1191097721694171],
        },
        properties: {
          name: 'Melbourne',
        },
        type: 'S2Feature',
      },
      {
        face: 3,
        geometry: {
          bbox: [149.1009, -35.3039, 149.1009, -35.3039],
          coordinates: { x: 0.9321761149504832, y: 0.16402766817497416 },
          is3D: false,
          type: 'Point',
          vecBBox: [
            0.9321761149504832, 0.16402766817497416, 0.9321761149504832, 0.16402766817497416,
          ],
        },
        properties: {
          name: 'Canberra',
        },
        type: 'S2Feature',
      },
      {
        face: 3,
        geometry: {
          bbox: [151.2144, -33.8766, 151.2144, -33.8766],
          coordinates: { x: 0.908036698755368, y: 0.18632281680962381 },
          is3D: false,
          type: 'Point',
          vecBBox: [0.908036698755368, 0.18632281680962381, 0.908036698755368, 0.18632281680962381],
        },
        properties: {
          name: 'Sydney',
        },
        type: 'S2Feature',
      },
    ],
    type: 'S2FeatureCollection',
  });
});

test('toJSON - WM & bbox & onFeature', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/points.geojson`);
  const jsonReader = new JSONReader(fileReader);
  const bufWriter = new BufferWriter();
  /**
   * @param feature - the feature to modify
   * @returns the modified feature
   */
  const onFeature = (feature: VectorFeatures): VectorFeatures | undefined => {
    if (feature.properties.name === 'Canberra') return;
    feature.properties.name = 'Redacted';
    return feature;
  };
  await toJSON(bufWriter, [jsonReader], { projection: 'WM', buildBBox: true, onFeature });
  const string = new TextDecoder().decode(bufWriter.commit());
  expect(string).toEqual(
    '{\n\t"type": "FeatureCollection",\n\t"features": [\n\t\t{"type":"VectorFeature","properties":{"name":"Redacted"},"geometry":{"type":"Point","is3D":false,"coordinates":{"x":144.9584,"y":-37.8173},"bbox":[144.9584,-37.8173,144.9584,-37.8173]}},\n\t\t{"type":"VectorFeature","properties":{"name":"Redacted"},"geometry":{"type":"Point","is3D":false,"coordinates":{"x":151.2144,"y":-33.8766},"bbox":[151.2144,-33.8766,151.2144,-33.8766]}}\n\t],\n\t"faces": [0],\n\t"bbox": [144.9584,-37.8173,151.2144,-33.8766]\n}',
  );
  expect(JSON.parse(string)).toEqual({
    bbox: [144.9584, -37.8173, 151.2144, -33.8766],
    faces: [0],
    features: [
      {
        geometry: {
          bbox: [144.9584, -37.8173, 144.9584, -37.8173],
          coordinates: {
            x: 144.9584,
            y: -37.8173,
          },
          is3D: false,
          type: 'Point',
        },
        properties: {
          name: 'Redacted',
        },
        type: 'VectorFeature',
      },
      {
        geometry: {
          bbox: [151.2144, -33.8766, 151.2144, -33.8766],
          coordinates: {
            x: 151.2144,
            y: -33.8766,
          },
          is3D: false,
          type: 'Point',
        },
        properties: {
          name: 'Redacted',
        },
        type: 'VectorFeature',
      },
    ],
    type: 'FeatureCollection',
  });
});

test('toJSONLD', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/points.geojson`);
  const jsonReader = new JSONReader(fileReader);
  const bufWriter = new BufferWriter();
  await toJSONLD(bufWriter, [jsonReader]);
  const string = new TextDecoder().decode(bufWriter.commit());
  expect(string).toEqual(
    '{"type":"S2Feature","face":3,"properties":{"name":"Melbourne"},"geometry":{"type":"Point","is3D":false,"coordinates":{"x":0.980307055282927,"y":0.1191097721694171},"bbox":[144.9584,-37.8173,144.9584,-37.8173],"vecBBox":[0.980307055282927,0.1191097721694171,0.980307055282927,0.1191097721694171]}}\n{"type":"S2Feature","face":3,"properties":{"name":"Canberra"},"geometry":{"type":"Point","is3D":false,"coordinates":{"x":0.9321761149504832,"y":0.16402766817497416},"bbox":[149.1009,-35.3039,149.1009,-35.3039],"vecBBox":[0.9321761149504832,0.16402766817497416,0.9321761149504832,0.16402766817497416]}}\n{"type":"S2Feature","face":3,"properties":{"name":"Sydney"},"geometry":{"type":"Point","is3D":false,"coordinates":{"x":0.908036698755368,"y":0.18632281680962381},"bbox":[151.2144,-33.8766,151.2144,-33.8766],"vecBBox":[0.908036698755368,0.18632281680962381,0.908036698755368,0.18632281680962381]}}\n',
  );

  const bufReader = new BufferReader(bufWriter.commit().buffer);
  const nlReader = new NewLineDelimitedJSONReader(bufReader);
  const data = await Array.fromAsync(nlReader);
  expect(data).toEqual([
    {
      face: 3,
      geometry: {
        bbox: [144.9584, -37.8173, 144.9584, -37.8173],
        coordinates: {
          x: 0.980307055282927,
          y: 0.1191097721694171,
        },
        is3D: false,
        type: 'Point',
        vecBBox: [0.980307055282927, 0.1191097721694171, 0.980307055282927, 0.1191097721694171],
      },
      properties: {
        name: 'Melbourne',
      },
      type: 'S2Feature',
    },
    {
      face: 3,
      geometry: {
        bbox: [149.1009, -35.3039, 149.1009, -35.3039],
        coordinates: {
          x: 0.9321761149504832,
          y: 0.16402766817497416,
        },
        is3D: false,
        type: 'Point',
        vecBBox: [0.9321761149504832, 0.16402766817497416, 0.9321761149504832, 0.16402766817497416],
      },
      properties: {
        name: 'Canberra',
      },
      type: 'S2Feature',
    },
    {
      face: 3,
      geometry: {
        bbox: [151.2144, -33.8766, 151.2144, -33.8766],
        coordinates: {
          x: 0.908036698755368,
          y: 0.18632281680962381,
        },
        is3D: false,
        type: 'Point',
        vecBBox: [0.908036698755368, 0.18632281680962381, 0.908036698755368, 0.18632281680962381],
      },
      properties: {
        name: 'Sydney',
      },
      type: 'S2Feature',
    },
  ]);
});

test('toJSONLD - WM & bbox & onFeature', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/points.geojson`);
  const jsonReader = new JSONReader(fileReader);
  const bufWriter = new BufferWriter();
  /**
   * @param feature - the feature to modify
   * @returns the modified feature
   */
  const onFeature = (feature: VectorFeatures): VectorFeatures | undefined => {
    if (feature.properties.name === 'Canberra') return;
    feature.properties.name = 'Redacted';
    return feature;
  };
  await toJSONLD(bufWriter, [jsonReader], { projection: 'WM', onFeature, buildBBox: true });
  const string = new TextDecoder().decode(bufWriter.commit());
  expect(string).toEqual(
    '{"type":"VectorFeature","properties":{"name":"Redacted"},"geometry":{"type":"Point","is3D":false,"coordinates":{"x":144.9584,"y":-37.8173},"bbox":[144.9584,-37.8173,144.9584,-37.8173]}}\n{"type":"VectorFeature","properties":{"name":"Redacted"},"geometry":{"type":"Point","is3D":false,"coordinates":{"x":151.2144,"y":-33.8766},"bbox":[151.2144,-33.8766,151.2144,-33.8766]}}\n',
  );

  const bufReader = new BufferReader(bufWriter.commit().buffer);
  const nlReader = new NewLineDelimitedJSONReader(bufReader);
  const data = await Array.fromAsync(nlReader);
  expect(data).toEqual([
    {
      geometry: {
        bbox: [144.9584, -37.8173, 144.9584, -37.8173],
        coordinates: {
          x: 144.9584,
          y: -37.8173,
        },
        is3D: false,
        type: 'Point',
      },
      properties: {
        name: 'Redacted',
      },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [151.2144, -33.8766, 151.2144, -33.8766],
        coordinates: {
          x: 151.2144,
          y: -33.8766,
        },
        is3D: false,
        type: 'Point',
      },
      properties: {
        name: 'Redacted',
      },
      type: 'VectorFeature',
    },
  ]);
});
