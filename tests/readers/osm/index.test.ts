import FileReader from '../../../src/readers/fileReader';
import { OSMReader } from '../../../src/readers/osm';
import { expect, test } from 'bun:test';

test('parse basic case', async () => {
  const fileReader = new FileReader('tests/readers/fixtures/test.pbf');

  const reader = new OSMReader(fileReader, { removeEmptyNodes: false });
  const header = await reader.getHeader();
  expect(header).toEqual({
    bbox: [-1, -1, -1, -1],
    optional_features: [],
    osmosis_replication_base_url: undefined,
    osmosis_replication_sequence_number: -1,
    osmosis_replication_timestamp: -1,
    required_features: ['-x�S��/�\rN�H�M�\r3�3S�rI�+N'],
    source: undefined,
    writingprogram: undefined,
  });
  const features = await Array.fromAsync(reader.iterate());

  expect(features.length).toBe(8);
  expect(features).toEqual([
    {
      geometry: {
        coordinates: { x: -0.1080108, y: 51.5074089 },
        is3D: false,
        type: 'Point',
      },
      id: 319408586,
      metadata: {
        changeset: 440330,
        timestamp: 1229476722000,
        uid: 6871,
        user: 'name',
        version: -1,
        visible: true,
      },
      properties: {},
      type: 'VectorFeature',
    },
    {
      geometry: {
        coordinates: { x: -0.10812640000000001, y: 51.5074343 },
        is3D: false,
        type: 'Point',
      },
      id: 319408587,
      metadata: {
        changeset: 0,
        timestamp: 1229476722000,
        uid: 6871,
        user: '',
        version: -1,
        visible: true,
      },
      properties: {},
      type: 'VectorFeature',
    },
    {
      geometry: {
        coordinates: { x: -0.10761860000000001, y: 51.5075933 },
        is3D: false,
        type: 'Point',
      },
      id: 275452090,
      metadata: {
        changeset: 2540257,
        timestamp: 1256818475000,
        uid: 1697,
        user: 'service',
        version: -2,
        visible: true,
      },
      properties: {
        amenity: 'cafe',
        name: "Jam's Sandwich Bar",
      },
      type: 'VectorFeature',
    },
    {
      geometry: {
        coordinates: { x: -0.1075735, y: 51.507464500000005 },
        is3D: false,
        type: 'Point',
      },
      id: 304994980,
      metadata: {
        changeset: -2591627,
        timestamp: 1234485707000,
        uid: 3516,
        user: 'private',
        version: 1,
        visible: true,
      },
      properties: {
        barrier: 'gate',
      },
      type: 'VectorFeature',
    },
    {
      geometry: {
        coordinates: { x: -0.10750140000000001, y: 51.5074723 },
        is3D: false,
        type: 'Point',
      },
      id: 304994981,
      metadata: {
        changeset: -14817,
        timestamp: 1224174957000,
        uid: 70,
        version: -1,
        visible: true,
      },
      properties: {},
      type: 'VectorFeature',
    },
    {
      geometry: {
        coordinates: { x: -0.10833480000000001, y: 51.507406 },
        is3D: false,
        type: 'Point',
      },
      id: 304994979,
      metadata: {
        changeset: 1739860,
        timestamp: 1250040812000,
        uid: 38244,
        version: 2,
        visible: true,
      },
      properties: {},
      type: 'VectorFeature',
    },
    {
      geometry: {
        coordinates: [
          { x: -0.10833480000000001, y: 51.507406 },
          { x: -0.10812640000000001, y: 51.5074343 },
          { x: -0.1080108, y: 51.5074089 },
          { x: -0.1075735, y: 51.507464500000005 },
          { x: -0.10750140000000001, y: 51.5074723 },
        ],
        is3D: false,
        type: 'LineString',
      },
      id: 27776903,
      metadata: {
        user: 'Matt',
        changeset: 684276,
        timestamp: -621888578000,
        uid: 35,
        version: -2,
        visible: true,
      },
      properties: {
        access: 'private',
        highway: 'service',
        name: 'üßé€',
      },
      type: 'VectorFeature',
    },
    {
      geometry: {
        coordinates: [
          { x: -0.10833480000000001, y: 51.507406 },
          { x: -0.10812640000000001, y: 51.5074343 },
          { x: -0.1080108, y: 51.5074089 },
          { x: -0.1075735, y: 51.507464500000005 },
          { x: -0.10750140000000001, y: 51.5074723 },
        ],
        is3D: false,
        type: 'LineString',
      },
      id: 56688,
      metadata: {
        changeset: -3473819,
        timestamp: -647421115000,
        uid: 28095,
        user: 'kmvar',
        version: 14,
        visible: true,
      },
      properties: {
        network: 'VVW',
        ref: '123',
        route: 'bus',
        type: 'route',
      },
      type: 'VectorFeature',
    },
  ]);
});
