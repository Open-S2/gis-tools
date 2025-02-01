import { GPXReader } from '../../../src';
import { expect, test } from 'bun:test';

test('GPXReader - basic', async () => {
  const xml = await Bun.file(`${__dirname}/fixtures/gpx-test-short.gpx`).text();
  const reader = new GPXReader(xml);
  expect(reader.metadata).toEqual({
    author: {
      email: {
        domain: 'example.com',
        id: 'demo',
      },
      link: {
        href: 'http://example.com',
        text: 'Author website',
        type: 'Web',
      },
      name: 'Demo Author',
    },
    bounds: {
      maxlat: 45.85097922514941,
      maxlon: 4.336738935765406,
      minlat: 49.12965660728301,
      minlon: -1.5521714646550901,
    },
    copyright: undefined,
    desc: 'A full featured gpx demo file',
    link: [
      {
        href: 'http://example.com',
        text: 'Author website',
        type: 'Web',
      },
      {
        href: 'http://example.com',
        text: 'Author website',
        type: 'Web',
      },
    ],
    name: 'GPX DEMO',
    time: '2020-01-12T21:32:52',
  });

  const features = await Array.fromAsync(reader);
  expect(features.length).toEqual(4);
  // await Bun.write(`${__dirname}/fixtures/gpx-test-short.json`, JSON.stringify(features, null, 2));
  const expected = await Bun.file(`${__dirname}/fixtures/gpx-test-short.json`).json();
  expect(features).toEqual(expected);
});

test('GPXReader - long', async () => {
  const xml = await Bun.file(`${__dirname}/fixtures/gpx-test-long.gpx`).text();
  const reader = new GPXReader(xml);
  expect(reader.metadata).toEqual({
    author: undefined,
    bounds: undefined,
    copyright: undefined,
    time: '2023-01-12T16:03:11Z',
  });
  const features = await Array.fromAsync(reader);
  expect(features.length).toEqual(1);
  // await Bun.write(`${__dirname}/fixtures/gpx-test-long.json`, JSON.stringify(features, null, 2));
  const expected = await Bun.file(`${__dirname}/fixtures/gpx-test-long.json`).json();
  expect(features).toEqual(expected);
});
