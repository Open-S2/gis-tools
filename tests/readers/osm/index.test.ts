import { OSMMMapReader } from '../../../src/mmap';
import { FileReader, OSMFileReader } from '../../../src/file';
import { OSMReader, TagFilter } from '../../../src';
import { expect, test } from 'bun:test';

test('parse basic case', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/test.pbf`);

  const reader = new OSMReader(fileReader, { removeEmptyNodes: false });
  const header = reader.getHeader();
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
  const features = await Array.fromAsync(reader);

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
        info: {
          changeset: 440330,
          timestamp: 1229476722000,
          uid: 6871,
          user: 'name',
          version: -1,
          visible: true,
        },
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
        info: {
          changeset: 0,
          timestamp: 1229476722000,
          uid: 6871,
          user: '',
          version: -1,
          visible: true,
        },
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
        info: {
          changeset: 2540257,
          timestamp: 1256818475000,
          uid: 1697,
          user: 'service',
          version: -2,
          visible: true,
        },
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
        info: {
          changeset: -2591627,
          timestamp: 1234485707000,
          uid: 3516,
          user: 'private',
          version: 1,
          visible: true,
        },
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
        info: {
          changeset: -14817,
          timestamp: 1224174957000,
          uid: 70,
          version: -1,
          visible: true,
        },
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
        info: {
          changeset: 1739860,
          timestamp: 1250040812000,
          uid: 38244,
          version: 2,
          visible: true,
        },
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
        info: {
          user: 'Matt',
          changeset: 684276,
          timestamp: -621888578000,
          uid: 35,
          version: -2,
          visible: true,
        },
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
        info: {
          changeset: -3473819,
          timestamp: -647421115000,
          uid: 28095,
          user: 'kmvar',
          version: 14,
          visible: true,
        },
        nodes: [
          {
            node: 319408586,
            relationID: 56688,
            role: '',
          },
        ],
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

test('parse basic case with mmap', async () => {
  const tagFilter = new TagFilter();
  tagFilter.addFilter('All', 'amenity', 'cafe');

  const reader = new OSMMMapReader(`${__dirname}/fixtures/test.pbf`, {
    removeEmptyNodes: false,
    tagFilter,
  });
  const header = reader.getHeader();
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
  const features = await Array.fromAsync(reader);

  expect(features.length).toBe(1);
  expect(features).toEqual([
    {
      geometry: {
        coordinates: { x: -0.10761860000000001, y: 51.5075933 },
        is3D: false,
        type: 'Point',
      },
      id: 275452090,
      metadata: {
        info: {
          changeset: 2540257,
          timestamp: 1256818475000,
          uid: 1697,
          user: 'service',
          version: -2,
          visible: true,
        },
      },
      properties: {
        amenity: 'cafe',
        name: "Jam's Sandwich Bar",
      },
      type: 'VectorFeature',
    },
  ]);

  reader.close();
});

test('parse basic case with filesystem', async () => {
  const tagFilter = new TagFilter();
  tagFilter.addFilter('All', 'amenity', 'cafe');

  const reader = new OSMFileReader(`${__dirname}/fixtures/test.pbf`, {
    removeEmptyNodes: false,
    tagFilter,
  });
  const header = reader.getHeader();
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
  const features = await Array.fromAsync(reader);

  expect(features.length).toBe(1);
  expect(features).toEqual([
    {
      geometry: {
        coordinates: { x: -0.10761860000000001, y: 51.5075933 },
        is3D: false,
        type: 'Point',
      },
      id: 275452090,
      metadata: {
        info: {
          changeset: 2540257,
          timestamp: 1256818475000,
          uid: 1697,
          user: 'service',
          version: -2,
          visible: true,
        },
      },
      properties: {
        amenity: 'cafe',
        name: "Jam's Sandwich Bar",
      },
      type: 'VectorFeature',
    },
  ]);

  reader.close();
});

test('parse only nodes', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/test.pbf`);

  const reader = new OSMReader(fileReader, {
    removeEmptyNodes: false,
    skipWays: true,
    skipRelations: true,
  });
  const header = reader.getHeader();
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
  const features = await Array.fromAsync(reader);

  expect(features.length).toBe(6);
  expect(features).toEqual([
    {
      geometry: {
        coordinates: { x: -0.1080108, y: 51.5074089 },
        is3D: false,
        type: 'Point',
      },
      id: 319408586,
      metadata: {
        info: {
          changeset: 440330,
          timestamp: 1229476722000,
          uid: 6871,
          user: 'name',
          version: -1,
          visible: true,
        },
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
        info: {
          changeset: 0,
          timestamp: 1229476722000,
          uid: 6871,
          user: '',
          version: -1,
          visible: true,
        },
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
        info: {
          changeset: 2540257,
          timestamp: 1256818475000,
          uid: 1697,
          user: 'service',
          version: -2,
          visible: true,
        },
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
        info: {
          changeset: -2591627,
          timestamp: 1234485707000,
          uid: 3516,
          user: 'private',
          version: 1,
          visible: true,
        },
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
        info: {
          changeset: -14817,
          timestamp: 1224174957000,
          uid: 70,
          version: -1,
          visible: true,
        },
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
        info: {
          changeset: 1739860,
          timestamp: 1250040812000,
          uid: 38244,
          version: 2,
          visible: true,
        },
      },
      properties: {},
      type: 'VectorFeature',
    },
  ]);
});

test('parse only ways', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/test.pbf`);

  const reader = new OSMReader(fileReader, {
    removeEmptyNodes: false,
    skipNodes: true,
    skipRelations: true,
  });
  const header = reader.getHeader();
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
  const features = await Array.fromAsync(reader);

  expect(features.length).toBe(1);
  expect(features).toEqual([
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
        info: {
          user: 'Matt',
          changeset: 684276,
          timestamp: -621888578000,
          uid: 35,
          version: -2,
          visible: true,
        },
      },
      properties: {
        access: 'private',
        highway: 'service',
        name: 'üßé€',
      },
      type: 'VectorFeature',
    },
  ]);
});

test('parse bounds case', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/bounds.osm.pbf`);

  const reader = new OSMReader(fileReader, { removeEmptyNodes: false, addBBox: true });
  const features = await Array.fromAsync(reader);
  expect(features.length).toBe(7);

  expect(features).toEqual([
    {
      geometry: {
        bbox: [1, 1, 1, 1],
        coordinates: { x: 1, y: 1 },
        is3D: false,
        type: 'Point',
      },
      id: 10,
      metadata: {
        info: {
          changeset: 1,
          timestamp: 1420074000000,
          uid: 1,
          user: 'a',
          version: -1,
          visible: true,
        },
      },
      properties: {},
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [1, 2, 1, 2],
        coordinates: { x: 1, y: 2 },
        is3D: false,
        type: 'Point',
      },
      id: 11,
      metadata: {
        info: {
          changeset: 0,
          timestamp: 1420074000000,
          uid: 1,
          user: '',
          version: -1,
          visible: true,
        },
      },
      properties: {},
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [1, 1, 1, 3],
        coordinates: { x: 1, y: 3 },
        is3D: false,
        type: 'Point',
      },
      id: 12,
      metadata: {
        info: {
          changeset: 0,
          timestamp: 1420074000000,
          uid: 1,
          user: '',
          version: -1,
          visible: true,
        },
        relation: {
          properties: { '123': '890' },
          role: 'label',
        },
      },
      properties: {
        a: '2',
      },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [1, 1, 1, 4],
        coordinates: { x: 1, y: 4 },
        is3D: false,
        type: 'Point',
      },
      id: 13,
      metadata: {
        info: {
          changeset: 0,
          timestamp: 1420074000000,
          uid: 1,
          user: '',
          version: -1,
          visible: true,
        },
        relation: {
          properties: { '123': '890' },
          role: 'admin_centre',
        },
      },
      properties: {},
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [1, 1, 1, 3],
        coordinates: [
          { x: 1, y: 1 },
          { x: 1, y: 2 },
          { x: 1, y: 3 },
        ],
        is3D: false,
        type: 'LineString',
      },
      id: 20,
      metadata: {
        info: {
          changeset: -1,
          timestamp: 710037000000,
          uid: -1,
          user: 'test',
          version: -1,
          visible: true,
        },
      },
      properties: {
        foo: 'bar',
      },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [1, 3, 1, 4],
        coordinates: [
          { x: 1, y: 3 },
          { x: 1, y: 4 },
        ],
        is3D: false,
        type: 'LineString',
      },
      id: 21,
      metadata: {
        info: {
          changeset: -1,
          timestamp: 710037000000,
          uid: -1,
          user: 'test',
          version: -1,
          visible: true,
        },
      },
      properties: {
        xyz: 'abc',
      },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: [1, 1, 1, 3],
        coordinates: [
          { x: 1, y: 1 },
          { x: 1, y: 2 },
          { x: 1, y: 3 },
        ],
        is3D: false,
        type: 'LineString',
      },
      id: 30,
      metadata: {
        info: {
          changeset: -1,
          timestamp: 710037000000,
          uid: -1,
          user: 'test',
          version: -1,
          visible: true,
        },
        nodes: [
          {
            node: 12,
            relationID: 30,
            role: 'label',
          },
          {
            node: 13,
            relationID: 30,
            role: 'admin_centre',
          },
        ],
      },
      properties: {
        '123': '890',
      },
      type: 'VectorFeature',
    },
  ]);
});

test('parse with filter', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/test.pbf`);
  const tagFilter = new TagFilter();
  tagFilter.addFilter('Node', 'amenity', 'cafe');
  tagFilter.addFilter('Node', 'barrier', 'gate');
  tagFilter.addFilter('Way', 'access', 'private');
  tagFilter.addFilter('Relation', 'type', 'route');

  const reader = new OSMReader(fileReader, { removeEmptyNodes: false, tagFilter });
  const features = await Array.fromAsync(reader);

  expect(features.length).toBe(4);

  expect(features).toEqual([
    {
      geometry: {
        bbox: undefined,
        coordinates: { x: -0.10761860000000001, y: 51.5075933 },
        is3D: false,
        type: 'Point',
      },
      id: 275452090,
      metadata: {
        info: {
          changeset: 2540257,
          timestamp: 1256818475000,
          uid: 1697,
          user: 'service',
          version: -2,
          visible: true,
        },
      },
      properties: {
        amenity: 'cafe',
        name: "Jam's Sandwich Bar",
      },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: undefined,
        coordinates: { x: -0.1075735, y: 51.507464500000005 },
        is3D: false,
        type: 'Point',
      },
      id: 304994980,
      metadata: {
        info: {
          changeset: -2591627,
          timestamp: 1234485707000,
          uid: 3516,
          user: 'private',
          version: 1,
          visible: true,
        },
      },
      properties: {
        barrier: 'gate',
      },
      type: 'VectorFeature',
    },
    {
      geometry: {
        bbox: undefined,
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
        info: {
          changeset: 684276,
          timestamp: -621888578000,
          uid: 35,
          user: 'Matt',
          version: -2,
          visible: true,
        },
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
        bbox: undefined,
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
        info: {
          changeset: -3473819,
          timestamp: -647421115000,
          uid: 28095,
          user: 'kmvar',
          version: 14,
          visible: true,
        },
        nodes: [
          {
            node: 319408586,
            relationID: 56688,
            role: '',
          },
        ],
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

test('check changeset is parsed without error', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/changeset.osm.pbf`);

  const reader = new OSMReader(fileReader, { removeEmptyNodes: false, addBBox: true });
  const features = await Array.fromAsync(reader);

  expect(features.length).toBe(0);
});
