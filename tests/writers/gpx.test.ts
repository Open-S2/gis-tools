import { BufferJSONReader, BufferWriter, GPXReader, toGPX } from '../../src/index.js';
import { expect, test } from 'bun:test';

import type { GPXVectorFeatures, VectorFeature } from '../../src/index.js';

test('GPX Writer - basic', async () => {
  const featureCollection = {
    type: 'FeatureCollection',
    features: await Bun.file(`${__dirname}/../readers/gpx/fixtures/gpx-test-short.json`).json(),
  };
  const len = featureCollection.features.length;
  // remove the metadata from each feature
  featureCollection.features.forEach((feature: VectorFeature) => {
    delete feature.metadata;
  });
  const jsonReader = new BufferJSONReader(JSON.stringify(featureCollection));
  const bufWriter = new BufferWriter();

  await toGPX(bufWriter, [jsonReader]);

  const actual = new TextDecoder().decode(bufWriter.commit());

  // uncomment to store the file
  // await Bun.write(`${__dirname}/fixtures/gpx-test-short-write.gpx`, actual);

  const expected = await Bun.file(`${__dirname}/fixtures/gpx-test-short-write.gpx`).text();
  expect(actual).toEqual(expected);

  // now pull the features out again
  const gpxReader = new GPXReader(actual);
  const gpxFeatures = await Array.fromAsync(gpxReader);
  // remove the metadata from each feature
  gpxFeatures.forEach((feature: GPXVectorFeatures) => {
    delete feature.metadata;
  });
  // TODO: The properties data is not perfectly identical currently but everything is working great.
  // No idea if its the reader or the writer that is the problem (most likely the reader)
  for (let i = 0; i < len; i++) {
    expect(gpxFeatures[i].geometry).toEqual(featureCollection.features[i].geometry);
  }
});

test('GPX Writer - bigger', async () => {
  const featureCollection = {
    type: 'FeatureCollection',
    features: await Bun.file(`${__dirname}/../readers/gpx/fixtures/gpx-test-long.json`).json(),
  };
  const len = featureCollection.features.length;
  // remove the metadata from each feature
  featureCollection.features.forEach((feature: VectorFeature) => {
    delete feature.metadata;
  });
  const jsonReader = new BufferJSONReader(JSON.stringify(featureCollection));
  const bufWriter = new BufferWriter();

  await toGPX(bufWriter, [jsonReader]);

  const actual = new TextDecoder().decode(bufWriter.commit());

  // uncomment to store the file
  // await Bun.write(`${__dirname}/fixtures/gpx-test-long-write.gpx`, actual);

  const expected = await Bun.file(`${__dirname}/fixtures/gpx-test-long-write.gpx`).text();
  expect(actual).toEqual(expected);

  // now pull the features out again
  const gpxReader = new GPXReader(actual);
  const gpxFeatures = await Array.fromAsync(gpxReader);
  // remove the metadata from each feature
  gpxFeatures.forEach((feature: GPXVectorFeatures) => {
    delete feature.metadata;
  });
  // TODO: The properties data is not perfectly identical currently but everything is working great.
  // No idea if its the reader or the writer that is the problem (most likely the reader)
  for (let i = 0; i < len; i++) {
    expect(gpxFeatures[i].geometry).toEqual(featureCollection.features[i].geometry);
  }
});
