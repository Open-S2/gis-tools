import {
  bboxToFeature,
  geoToVector,
  tileChildren,
  tileFromID,
  tileNeighbors,
  tileParent,
  tileToBBox,
  tileToCenterLonLat,
  tileToID,
  toFlatGeometry,
  toVector,
  wmTileFromPoint,
  wmTileFromVectorFeature,
  wmTileFromVectorGeometry,
} from '../../../src/index.js';
import { describe, expect, it } from 'bun:test';

import type { Geometry } from '../../../src/index.js';

describe('tileToID', () => {
  it('WM', () => {
    expect(tileToID({ zoom: 0, x: 0, y: 0 })).toEqual(1152921504606846976n);
    expect(tileToID({ zoom: 1, x: 1, y: 0 })).toEqual(2017612633061982208n);
  });
  it('S2', () => {
    expect(tileToID({ face: 0, zoom: 0, x: 0, y: 0 })).toEqual(1152921504606846976n);
    expect(tileToID({ face: 0, zoom: 1, x: 1, y: 0 })).toEqual(2017612633061982208n);
    expect(tileToID({ face: 1, zoom: 1, x: 1, y: 0 })).toEqual(3170534137668829184n);
  });
});

describe('tileFromID', () => {
  it('WM', () => {
    expect(tileFromID(1152921504606846976n, true)).toEqual({ zoom: 0, x: 0, y: 0 });
    expect(tileFromID(2017612633061982208n, true)).toEqual({ zoom: 1, x: 1, y: 0 });
  });
  it('S2', () => {
    expect(tileFromID(1152921504606846976n, false)).toEqual({ face: 0, zoom: 0, x: 0, y: 0 });
    expect(tileFromID(2017612633061982208n, false)).toEqual({ face: 0, zoom: 1, x: 1, y: 0 });
    expect(tileFromID(3170534137668829184n, false)).toEqual({ face: 1, zoom: 1, x: 1, y: 0 });
  });
});

describe('tileToBBox', () => {
  it('WM', () => {
    expect(tileToBBox({ zoom: 0, x: 0, y: 0 })).toEqual([
      -180, -85.05112877980659, 180, 85.05112877980659,
    ]);
    expect(tileToBBox({ zoom: 1, x: 1, y: 0 })).toEqual([0, 0, 180, 85.05112877980659]);
    expect(tileToBBox({ zoom: 1, x: 1, y: 0 }, true)).toEqual([0, -85.05112877980659, 180, 0]);
  });
  it('S2', () => {
    expect(tileToBBox({ face: 0, zoom: 0, x: 0, y: 0 })).toEqual([
      -45, -35.264389682754654, 45, 35.264389682754654,
    ]);
    expect(tileToBBox({ face: 0, zoom: 1, x: 1, y: 0 })).toEqual([0, -45, 45, 0]);
    expect(tileToBBox({ face: 1, zoom: 1, x: 1, y: 0 })).toEqual([90, -45, 135, 0]);
  });
});

describe('tileToCenterLonLat', () => {
  it('WM', () => {
    expect(tileToCenterLonLat({ zoom: 0, x: 0, y: 0 })).toEqual({ x: 0, y: 0 });
    expect(tileToCenterLonLat({ zoom: 1, x: 1, y: 0 })).toEqual({ x: 90, y: 42.525564389903295 });
    expect(tileToCenterLonLat({ zoom: 1, x: 1, y: 0 }, true)).toEqual({
      x: 90,
      y: -42.525564389903295,
    });
  });
  it('S2', () => {
    expect(tileToCenterLonLat({ face: 0, zoom: 0, x: 0, y: 0 })).toEqual({ x: 0, y: 0 });
    expect(tileToCenterLonLat({ face: 0, zoom: 1, x: 1, y: 0 })).toEqual({
      x: 22.5,
      y: -22.5,
    });
    expect(tileToCenterLonLat({ face: 1, zoom: 1, x: 1, y: 0 })).toEqual({
      x: 112.5,
      y: -22.5,
    });
  });
});

describe('tileChildren', () => {
  it('WM', () => {
    expect(tileChildren({ zoom: 0, x: 0, y: 0 })).toEqual([
      { zoom: 1, x: 0, y: 0 },
      { zoom: 1, x: 1, y: 0 },
      { zoom: 1, x: 0, y: 1 },
      { zoom: 1, x: 1, y: 1 },
    ]);
  });
  it('S2', () => {
    expect(tileChildren({ face: 0, zoom: 0, x: 0, y: 0 })).toEqual([
      { face: 0, x: 0, y: 0, zoom: 1 },
      { face: 0, x: 0, y: 1, zoom: 1 },
      { face: 0, x: 1, y: 1, zoom: 1 },
      { face: 0, x: 1, y: 0, zoom: 1 },
    ]);
    expect(tileChildren({ face: 2, zoom: 0, x: 0, y: 0 })).toEqual([
      { face: 2, x: 0, y: 0, zoom: 1 },
      { face: 2, x: 0, y: 1, zoom: 1 },
      { face: 2, x: 1, y: 1, zoom: 1 },
      { face: 2, x: 1, y: 0, zoom: 1 },
    ]);
  });
});

describe('tileParent', () => {
  it('WM', () => {
    expect(tileParent({ zoom: 1, x: 1, y: 0 })).toEqual({ zoom: 0, x: 0, y: 0 });
  });
  it('S2', () => {
    expect(tileParent({ face: 0, zoom: 1, x: 1, y: 0 })).toEqual({ face: 0, zoom: 0, x: 0, y: 0 });
  });
});

describe('tileNeighbors', () => {
  it('WM', () => {
    expect(tileNeighbors({ zoom: 1, x: 1, y: 0 })).toEqual([
      { zoom: 1, x: 0, y: 0 },
      { zoom: 1, x: 1, y: 1 },
    ]);
  });
  it('S2', () => {
    expect(tileNeighbors({ face: 0, zoom: 1, x: 1, y: 0 })).toEqual([
      { face: 5, x: 1, y: 1, zoom: 1 },
      { face: 1, x: 0, y: 0, zoom: 1 },
      { face: 0, x: 1, y: 1, zoom: 1 },
      { face: 0, x: 0, y: 0, zoom: 1 },
    ]);
  });
});

describe('WM Tile From Vectors', () => {
  it('point', () => {
    const point = { x: 79.0809631347656, y: 21.135184856708992 };
    const tile = wmTileFromPoint(point, 1);
    expect(tile).toEqual({ zoom: 1, x: 1, y: 0 });

    const tiles = wmTileFromVectorFeature(
      {
        type: 'VectorFeature',
        geometry: { type: 'Point', is3D: false, coordinates: point },
        properties: {},
      },
      1,
    );
    expect(tiles).toEqual([{ zoom: 1, x: 1, y: 0 }]);
  });

  it('line', async () => {
    const lineFeture = await Bun.file(`${__dirname}/fixtures/line.geojson`).json();
    const lineVectorFeature = toVector(lineFeture);
    const tiles = wmTileFromVectorFeature(lineVectorFeature, 12);
    expect(tiles).toEqual([
      { x: 839, y: 1708, zoom: 12 },
      { x: 839, y: 1707, zoom: 12 },
      { x: 840, y: 1707, zoom: 12 },
      { x: 840, y: 1706, zoom: 12 },
      { x: 840, y: 1705, zoom: 12 },
      { x: 841, y: 1705, zoom: 12 },
      { x: 843, y: 1706, zoom: 12 },
      { x: 843, y: 1707, zoom: 12 },
      { x: 843, y: 1708, zoom: 12 },
      { x: 421, y: 852, zoom: 11 },
    ]);
    const resFeatures = tiles.map((tile) => {
      const bbox = tileToBBox(tile);
      const vf = bboxToFeature(bbox);
      return toFlatGeometry(vf);
    });

    // comment out when written
    // await Bun.write(
    //   `${__dirname}/fixtures/line_out_local.geojson`,
    //   JSON.stringify({ type: 'FeatureCollection', features: resFeatures }, null, 2),
    // );

    const expectedFeatures = await Bun.file(`${__dirname}/fixtures/line_out_local.geojson`).json();
    expect(resFeatures).toEqual(expectedFeatures.features);
  });

  it('edgeline', async () => {
    const lineFeture = await Bun.file(`${__dirname}/fixtures/edgeline.geojson`).json();
    const lineVectorFeature = toVector(lineFeture);
    const tiles = wmTileFromVectorFeature(lineVectorFeature, 14);
    expect(tiles).toEqual([
      { x: 4543, y: 6612, zoom: 14 },
      { x: 4544, y: 6612, zoom: 14 },
    ]);
    const resFeatures = tiles.map((tile) => {
      const bbox = tileToBBox(tile);
      const vf = bboxToFeature(bbox);
      return toFlatGeometry(vf);
    });

    // comment out when written
    // await Bun.write(
    //   `${__dirname}/fixtures/edgeline_out_local.geojson`,
    //   JSON.stringify({ type: 'FeatureCollection', features: resFeatures }, null, 2),
    // );

    const expectedFeatures = await Bun.file(
      `${__dirname}/fixtures/edgeline_out_local.geojson`,
    ).json();
    expect(resFeatures).toEqual(expectedFeatures.features);
  });

  it('polygon', async () => {
    const polygonFeture = await Bun.file(`${__dirname}/fixtures/polygon.geojson`).json();
    const polygonVectorFeature = toVector({
      type: 'Feature',
      properties: {},
      geometry: polygonFeture,
    });
    const tiles = wmTileFromVectorFeature(polygonVectorFeature, 15);
    expect(tiles).toEqual([
      { x: 16850, y: 14480, zoom: 15 },
      { x: 16850, y: 14451, zoom: 15 },
      { x: 16851, y: 14451, zoom: 15 },
      { x: 16852, y: 14451, zoom: 15 },
      { x: 16853, y: 14451, zoom: 15 },
      { x: 16854, y: 14451, zoom: 15 },
      { x: 16855, y: 14451, zoom: 15 },
      { x: 16856, y: 14451, zoom: 15 },
      { x: 16857, y: 14451, zoom: 15 },
      { x: 16858, y: 14451, zoom: 15 },
      { x: 16859, y: 14451, zoom: 15 },
      { x: 16860, y: 14451, zoom: 15 },
      { x: 16861, y: 14451, zoom: 15 },
      { x: 16862, y: 14451, zoom: 15 },
      { x: 16863, y: 14451, zoom: 15 },
      { x: 16864, y: 14451, zoom: 15 },
      { x: 16865, y: 14451, zoom: 15 },
      { x: 16866, y: 14451, zoom: 15 },
      { x: 16867, y: 14451, zoom: 15 },
      { x: 16868, y: 14451, zoom: 15 },
      { x: 16869, y: 14451, zoom: 15 },
      { x: 16870, y: 14451, zoom: 15 },
      { x: 16871, y: 14451, zoom: 15 },
      { x: 16872, y: 14451, zoom: 15 },
      { x: 16873, y: 14451, zoom: 15 },
      { x: 16874, y: 14451, zoom: 15 },
      { x: 16875, y: 14451, zoom: 15 },
      { x: 16876, y: 14451, zoom: 15 },
      { x: 16877, y: 14451, zoom: 15 },
      { x: 16878, y: 14451, zoom: 15 },
      { x: 16879, y: 14451, zoom: 15 },
      { x: 16880, y: 14451, zoom: 15 },
      { x: 16881, y: 14451, zoom: 15 },
      { x: 16882, y: 14451, zoom: 15 },
      { x: 16883, y: 14451, zoom: 15 },
      { x: 16884, y: 14451, zoom: 15 },
      { x: 16885, y: 14451, zoom: 15 },
      { x: 16885, y: 14480, zoom: 15 },
      { x: 16884, y: 14480, zoom: 15 },
      { x: 16883, y: 14480, zoom: 15 },
      { x: 16882, y: 14480, zoom: 15 },
      { x: 16881, y: 14480, zoom: 15 },
      { x: 16880, y: 14480, zoom: 15 },
      { x: 16879, y: 14480, zoom: 15 },
      { x: 16878, y: 14480, zoom: 15 },
      { x: 16877, y: 14480, zoom: 15 },
      { x: 16876, y: 14480, zoom: 15 },
      { x: 16875, y: 14480, zoom: 15 },
      { x: 16874, y: 14480, zoom: 15 },
      { x: 16873, y: 14480, zoom: 15 },
      { x: 16872, y: 14480, zoom: 15 },
      { x: 16871, y: 14480, zoom: 15 },
      { x: 16870, y: 14480, zoom: 15 },
      { x: 16869, y: 14480, zoom: 15 },
      { x: 16868, y: 14480, zoom: 15 },
      { x: 16867, y: 14480, zoom: 15 },
      { x: 16866, y: 14480, zoom: 15 },
      { x: 16865, y: 14480, zoom: 15 },
      { x: 16864, y: 14480, zoom: 15 },
      { x: 16863, y: 14480, zoom: 15 },
      { x: 16862, y: 14480, zoom: 15 },
      { x: 16861, y: 14480, zoom: 15 },
      { x: 16860, y: 14480, zoom: 15 },
      { x: 16859, y: 14480, zoom: 15 },
      { x: 16858, y: 14480, zoom: 15 },
      { x: 16857, y: 14480, zoom: 15 },
      { x: 16856, y: 14480, zoom: 15 },
      { x: 16855, y: 14480, zoom: 15 },
      { x: 16854, y: 14480, zoom: 15 },
      { x: 16853, y: 14480, zoom: 15 },
      { x: 16852, y: 14480, zoom: 15 },
      { x: 16851, y: 14480, zoom: 15 },
      { x: 8425, y: 7239, zoom: 14 },
      { x: 8425, y: 7238, zoom: 14 },
      { x: 8425, y: 7237, zoom: 14 },
      { x: 8425, y: 7236, zoom: 14 },
      { x: 8425, y: 7235, zoom: 14 },
      { x: 8425, y: 7234, zoom: 14 },
      { x: 8425, y: 7233, zoom: 14 },
      { x: 8425, y: 7232, zoom: 14 },
      { x: 8425, y: 7231, zoom: 14 },
      { x: 8425, y: 7230, zoom: 14 },
      { x: 8425, y: 7229, zoom: 14 },
      { x: 8425, y: 7228, zoom: 14 },
      { x: 8425, y: 7227, zoom: 14 },
      { x: 8425, y: 7226, zoom: 14 },
      { x: 8442, y: 7226, zoom: 14 },
      { x: 8442, y: 7227, zoom: 14 },
      { x: 8442, y: 7228, zoom: 14 },
      { x: 8442, y: 7229, zoom: 14 },
      { x: 8442, y: 7230, zoom: 14 },
      { x: 8442, y: 7231, zoom: 14 },
      { x: 8442, y: 7232, zoom: 14 },
      { x: 8442, y: 7233, zoom: 14 },
      { x: 8442, y: 7234, zoom: 14 },
      { x: 8442, y: 7235, zoom: 14 },
      { x: 8442, y: 7236, zoom: 14 },
      { x: 8442, y: 7237, zoom: 14 },
      { x: 8442, y: 7238, zoom: 14 },
      { x: 8442, y: 7239, zoom: 14 },
      { x: 4213, y: 3613, zoom: 13 },
      { x: 4214, y: 3613, zoom: 13 },
      { x: 4215, y: 3613, zoom: 13 },
      { x: 4216, y: 3613, zoom: 13 },
      { x: 4217, y: 3613, zoom: 13 },
      { x: 4218, y: 3613, zoom: 13 },
      { x: 4219, y: 3613, zoom: 13 },
      { x: 4220, y: 3613, zoom: 13 },
      { x: 4213, y: 3614, zoom: 13 },
      { x: 4220, y: 3614, zoom: 13 },
      { x: 4213, y: 3615, zoom: 13 },
      { x: 4220, y: 3615, zoom: 13 },
      { x: 4213, y: 3616, zoom: 13 },
      { x: 4220, y: 3616, zoom: 13 },
      { x: 4213, y: 3617, zoom: 13 },
      { x: 4220, y: 3617, zoom: 13 },
      { x: 4213, y: 3618, zoom: 13 },
      { x: 4220, y: 3618, zoom: 13 },
      { x: 4213, y: 3619, zoom: 13 },
      { x: 4220, y: 3619, zoom: 13 },
      { x: 2107, y: 1807, zoom: 12 },
      { x: 2108, y: 1807, zoom: 12 },
      { x: 2109, y: 1807, zoom: 12 },
      { x: 2107, y: 1808, zoom: 12 },
      { x: 2107, y: 1809, zoom: 12 },
      { x: 1054, y: 904, zoom: 11 },
    ]);
    const resFeatures = tiles.map((tile) => {
      const bbox = tileToBBox(tile);
      const vf = bboxToFeature(bbox);
      return toFlatGeometry(vf);
    });

    // comment out when written
    // await Bun.write(
    //   `${__dirname}/fixtures/polygon_out_local.geojson`,
    //   JSON.stringify({ type: 'FeatureCollection', features: resFeatures }, null, 2),
    // );

    const expectedFeatures = await Bun.file(
      `${__dirname}/fixtures/polygon_out_local.geojson`,
    ).json();
    expect(resFeatures).toEqual(expectedFeatures.features);
  });

  it('multipoint', async () => {
    const multipointFeture = await Bun.file(`${__dirname}/fixtures/multipoint.geojson`).json();
    const multipointVectorFeature = toVector(multipointFeture);
    const tiles = wmTileFromVectorFeature(multipointVectorFeature, 12);
    expect(tiles).toEqual([
      { x: 1086, y: 1498, zoom: 12 },
      { x: 1014, y: 1552, zoom: 12 },
      { x: 1086, y: 1497, zoom: 12 },
      { x: 1014, y: 1551, zoom: 12 },
    ]);
    const resFeatures = tiles.map((tile) => {
      const bbox = tileToBBox(tile);
      const vf = bboxToFeature(bbox);
      return toFlatGeometry(vf);
    });

    // comment out when written
    // await Bun.write(
    //   `${__dirname}/fixtures/multipoint_out_local.geojson`,
    //   JSON.stringify({ type: 'FeatureCollection', features: resFeatures }, null, 2),
    // );

    const expectedFeatures = await Bun.file(
      `${__dirname}/fixtures/multipoint_out_local.geojson`,
    ).json();
    expect(resFeatures).toEqual(expectedFeatures.features);
  });

  it('multiline', async () => {
    const multilineFeture = await Bun.file(`${__dirname}/fixtures/multiline.geojson`).json();
    const multilineVectorFeature = toVector(multilineFeture);
    const tiles = wmTileFromVectorFeature(multilineVectorFeature, 8);
    expect(tiles).toEqual([
      { x: 136, y: 85, zoom: 8 },
      { x: 135, y: 85, zoom: 8 },
      { x: 133, y: 86, zoom: 8 },
      { x: 135, y: 88, zoom: 8 },
      { x: 134, y: 88, zoom: 8 },
      { x: 132, y: 90, zoom: 8 },
      { x: 131, y: 90, zoom: 8 },
      { x: 128, y: 87, zoom: 8 },
      { x: 129, y: 87, zoom: 8 },
      { x: 129, y: 86, zoom: 8 },
      { x: 130, y: 86, zoom: 8 },
      { x: 130, y: 87, zoom: 8 },
      { x: 130, y: 88, zoom: 8 },
      { x: 131, y: 88, zoom: 8 },
      { x: 131, y: 87, zoom: 8 },
      { x: 135, y: 89, zoom: 8 },
      { x: 136, y: 89, zoom: 8 },
      { x: 136, y: 90, zoom: 8 },
      { x: 67, y: 43, zoom: 7 },
      { x: 66, y: 44, zoom: 7 },
    ]);
    const resFeatures = tiles.map((tile) => {
      const bbox = tileToBBox(tile);
      const vf = bboxToFeature(bbox);
      return toFlatGeometry(vf);
    });

    // comment out when written
    // await Bun.write(
    //   `${__dirname}/fixtures/multiline_out_local.geojson`,
    //   JSON.stringify({ type: 'FeatureCollection', features: resFeatures }, null, 2),
    // );

    const expectedFeatures = await Bun.file(
      `${__dirname}/fixtures/multiline_out_local.geojson`,
    ).json();
    expect(resFeatures).toEqual(expectedFeatures.features);
  });

  it('uk', async () => {
    const ukFeture = await Bun.file(`${__dirname}/fixtures/uk.geojson`).json();
    const ukVectorFeature = toVector(ukFeture);
    const tiles = wmTileFromVectorFeature(ukVectorFeature, 9);
    expect(tiles).toEqual([
      { x: 247, y: 164, zoom: 9 },
      { x: 246, y: 164, zoom: 9 },
      { x: 245, y: 164, zoom: 9 },
      { x: 245, y: 163, zoom: 9 },
      { x: 245, y: 162, zoom: 9 },
      { x: 245, y: 161, zoom: 9 },
      { x: 246, y: 161, zoom: 9 },
      { x: 250, y: 154, zoom: 9 },
      { x: 250, y: 155, zoom: 9 },
      { x: 251, y: 155, zoom: 9 },
      { x: 252, y: 155, zoom: 9 },
      { x: 253, y: 155, zoom: 9 },
      { x: 253, y: 156, zoom: 9 },
      { x: 252, y: 156, zoom: 9 },
      { x: 252, y: 157, zoom: 9 },
      { x: 252, y: 158, zoom: 9 },
      { x: 252, y: 159, zoom: 9 },
      { x: 253, y: 159, zoom: 9 },
      { x: 254, y: 161, zoom: 9 },
      { x: 254, y: 162, zoom: 9 },
      { x: 254, y: 163, zoom: 9 },
      { x: 255, y: 163, zoom: 9 },
      { x: 256, y: 165, zoom: 9 },
      { x: 256, y: 166, zoom: 9 },
      { x: 256, y: 167, zoom: 9 },
      { x: 257, y: 167, zoom: 9 },
      { x: 258, y: 167, zoom: 9 },
      { x: 258, y: 168, zoom: 9 },
      { x: 258, y: 169, zoom: 9 },
      { x: 258, y: 170, zoom: 9 },
      { x: 254, y: 172, zoom: 9 },
      { x: 253, y: 172, zoom: 9 },
      { x: 252, y: 172, zoom: 9 },
      { x: 251, y: 172, zoom: 9 },
      { x: 250, y: 172, zoom: 9 },
      { x: 250, y: 173, zoom: 9 },
      { x: 247, y: 173, zoom: 9 },
      { x: 249, y: 167, zoom: 9 },
      { x: 249, y: 166, zoom: 9 },
      { x: 249, y: 165, zoom: 9 },
      { x: 250, y: 165, zoom: 9 },
      { x: 251, y: 165, zoom: 9 },
      { x: 251, y: 164, zoom: 9 },
      { x: 249, y: 162, zoom: 9 },
      { x: 248, y: 162, zoom: 9 },
      { x: 247, y: 159, zoom: 9 },
      { x: 247, y: 158, zoom: 9 },
      { x: 247, y: 157, zoom: 9 },
      { x: 247, y: 156, zoom: 9 },
      { x: 247, y: 155, zoom: 9 },
      { x: 247, y: 154, zoom: 9 },
      { x: 123, y: 81, zoom: 8 },
      { x: 125, y: 76, zoom: 8 },
      { x: 126, y: 80, zoom: 8 },
      { x: 128, y: 84, zoom: 8 },
      { x: 128, y: 85, zoom: 8 },
      { x: 124, y: 86, zoom: 8 },
      { x: 125, y: 81, zoom: 8 },
      { x: 124, y: 80, zoom: 8 },
      { x: 124, y: 77, zoom: 8 },
      { x: 124, y: 76, zoom: 8 },
      { x: 125, y: 80, zoom: 8 },
      { x: 126, y: 81, zoom: 8 },
      { x: 125, y: 83, zoom: 8 },
      { x: 62, y: 39, zoom: 7 },
      { x: 63, y: 41, zoom: 7 },
      { x: 63, y: 42, zoom: 7 },
      { x: 62, y: 42, zoom: 7 },
    ]);
    const resFeatures = tiles.map((tile) => {
      const bbox = tileToBBox(tile);
      const vf = bboxToFeature(bbox);
      return toFlatGeometry(vf);
    });

    // comment out when written
    // await Bun.write(
    //   `${__dirname}/fixtures/uk_out_local.geojson`,
    //   JSON.stringify({ type: 'FeatureCollection', features: resFeatures }, null, 2),
    // );

    const expectedFeatures = await Bun.file(`${__dirname}/fixtures/uk_out_local.geojson`).json();
    expect(resFeatures).toEqual(expectedFeatures.features);
  });

  it('degenring', async () => {
    const degenringFeture = await Bun.file(`${__dirname}/fixtures/degenring.geojson`).json();
    const degenringVectorFeature = geoToVector(degenringFeture);
    const tiles = wmTileFromVectorGeometry(degenringVectorFeature, 15);
    // expect(tiles).toEqual([]);
    const resFeatures = tiles.map((tile) => {
      const bbox = tileToBBox(tile);
      const vf = bboxToFeature(bbox);
      return toFlatGeometry(vf);
    });

    // comment out when written
    // await Bun.write(
    //   `${__dirname}/fixtures/degenring_out_local.geojson`,
    //   JSON.stringify({ type: 'FeatureCollection', features: resFeatures }, null, 2),
    // );

    const expectedFeatures = await Bun.file(
      `${__dirname}/fixtures/degenring_out_local.geojson`,
    ).json();
    expect(resFeatures).toEqual(expectedFeatures.features);
  });

  it('hourglass', async () => {
    const hourglassFeature: Geometry = {
      type: 'Polygon',
      coordinates: [
        [
          [-12.034835815429688, 8.901183448260598],
          [-12.060413360595701, 8.899826693726117],
          [-12.036380767822266, 8.873199368734273],
          [-12.059383392333983, 8.871418491385919],
          [-12.034835815429688, 8.901183448260598],
        ],
      ],
    };
    const hourglassVectorFeature = geoToVector(hourglassFeature);
    const tiles = wmTileFromVectorGeometry(hourglassVectorFeature, 15);
    expect(tiles).toEqual([
      { x: 15288, y: 15570, zoom: 15 },
      { x: 15288, y: 15572, zoom: 15 },
      { x: 15288, y: 15573, zoom: 15 },
      { x: 15288, y: 15571, zoom: 15 },
      { x: 7643, y: 7785, zoom: 14 },
      { x: 7643, y: 7786, zoom: 14 },
    ]);
    const resFeatures = tiles.map((tile) => {
      const bbox = tileToBBox(tile);
      const vf = bboxToFeature(bbox);
      return toFlatGeometry(vf);
    });

    // comment out when written
    // await Bun.write(
    //   `${__dirname}/fixtures/hourglass_out_local.geojson`,
    //   JSON.stringify({ type: 'FeatureCollection', features: resFeatures }, null, 2),
    // );

    const expectedFeatures = await Bun.file(
      `${__dirname}/fixtures/hourglass_out_local.geojson`,
    ).json();
    expect(resFeatures).toEqual(expectedFeatures.features);
  });

  it('spiked', async () => {
    const spikedFeture = await Bun.file(`${__dirname}/fixtures/spiked.geojson`).json();
    const spikedVectorFeature = geoToVector(spikedFeture);
    const tiles = wmTileFromVectorGeometry(spikedVectorFeature, 10);
    // expect(tiles).toEqual([]);
    const resFeatures = tiles.map((tile) => {
      const bbox = tileToBBox(tile);
      const vf = bboxToFeature(bbox);
      return toFlatGeometry(vf);
    });

    // comment out when written
    // await Bun.write(
    //   `${__dirname}/fixtures/spiked_out_local.geojson`,
    //   JSON.stringify({ type: 'FeatureCollection', features: resFeatures }, null, 2),
    // );

    const expectedFeatures = await Bun.file(
      `${__dirname}/fixtures/spiked_out_local.geojson`,
    ).json();
    expect(resFeatures).toEqual(expectedFeatures.features);
  });

  it('blocky', async () => {
    const blockyFeture = await Bun.file(`${__dirname}/fixtures/blocky.geojson`).json();
    const blockyVectorFeature = geoToVector(blockyFeture);
    const tiles = wmTileFromVectorGeometry(blockyVectorFeature, 6);
    expect(tiles).toEqual([
      { x: 10, y: 26, zoom: 6 },
      { x: 11, y: 26, zoom: 6 },
      { x: 11, y: 25, zoom: 6 },
      { x: 12, y: 25, zoom: 6 },
      { x: 13, y: 25, zoom: 6 },
      { x: 14, y: 25, zoom: 6 },
      { x: 15, y: 25, zoom: 6 },
      { x: 16, y: 25, zoom: 6 },
      { x: 16, y: 26, zoom: 6 },
      { x: 16, y: 27, zoom: 6 },
      { x: 16, y: 28, zoom: 6 },
      { x: 16, y: 29, zoom: 6 },
      { x: 11, y: 29, zoom: 6 },
      { x: 11, y: 28, zoom: 6 },
      { x: 11, y: 27, zoom: 6 },
      { x: 7, y: 14, zoom: 5 },
      { x: 6, y: 14, zoom: 5 },
      { x: 6, y: 13, zoom: 5 },
      { x: 7, y: 13, zoom: 5 },
    ]);
    const resFeatures = tiles.map((tile) => {
      const bbox = tileToBBox(tile);
      const vf = bboxToFeature(bbox);
      return toFlatGeometry(vf);
    });

    // comment out when written
    // await Bun.write(
    //   `${__dirname}/fixtures/blocky_out_local.geojson`,
    //   JSON.stringify({ type: 'FeatureCollection', features: resFeatures }, null, 2),
    // );

    const expectedFeatures = await Bun.file(
      `${__dirname}/fixtures/blocky_out_local.geojson`,
    ).json();
    expect(resFeatures).toEqual(expectedFeatures.features);
  });

  it('donut', async () => {
    const donutFeture = await Bun.file(`${__dirname}/fixtures/donut.geojson`).json();
    const donutVectorFeature = geoToVector(donutFeture);
    const tiles = wmTileFromVectorGeometry(donutVectorFeature, 16);
    // expect(tiles).toEqual([]);
    const resFeatures = tiles.map((tile) => {
      const bbox = tileToBBox(tile);
      const vf = bboxToFeature(bbox);
      return toFlatGeometry(vf);
    });

    // comment out when written
    // await Bun.write(
    //   `${__dirname}/fixtures/donut_out_local.geojson`,
    //   JSON.stringify({ type: 'FeatureCollection', features: resFeatures }, null, 2),
    // );

    const expectedFeatures = await Bun.file(`${__dirname}/fixtures/donut_out_local.geojson`).json();
    expect(resFeatures).toEqual(expectedFeatures.features);
  });
});
