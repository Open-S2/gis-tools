// import {
//   WKTGeometryReader,
//   parseWKTGeometry,
//   parseWKTObject,
//   parseWKTProjection,
//   splitWKTGeometry,
// } from '../../../src/readers/wkt';
import { SVGReader } from '../../../src';
import { expect, test } from 'bun:test';

test('SVG Reader - paths', async () => {
  const paths = await Bun.file(`${__dirname}/fixtures/paths.svg`).text();
  const reader = new SVGReader(paths);
  const features = await Array.fromAsync(reader);
  expect(features.length).toEqual(2);
  const [poly, line] = features;
  expect(poly.geometry.type).toEqual('Polygon');
  expect(poly.properties).toEqual({
    fill: 'red',
    stroke: 'black',
    strokeWidth: '2',
  });
  expect(poly.geometry.coordinates).toEqual([
    [
      { x: 0.1, y: 0.9 },
      { x: 0.4, y: 0.9 },
      { x: 0.4, y: 0.6 },
      { x: 0.1, y: 0.6 },
      { x: 0.1, y: 0.6 },
      { x: 0.1, y: 0.9 },
    ],
  ]);
  expect(line.geometry.type).toEqual('LineString');
  expect(line.properties).toEqual({
    fill: undefined,
    stroke: 'blue',
    strokeWidth: '4',
  });
  expect(line.geometry.coordinates).toEqual([
    { x: 0.6, y: 0.4 },
    { x: 0.6, y: 0.19999999999999996 },
    { x: 0.8, y: 0.19999999999999996 },
  ]);
});
