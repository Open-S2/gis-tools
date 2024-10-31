import { dsinit } from '../../../src';
import { expect, test } from 'bun:test';

test('Geopotential Resonance for 12 Hour Orbits', async () => {
  const dsOptionSets = await Bun.file(`${__dirname}/../fixtures/dsinit.json`).json();
  for (const testSet of dsOptionSets) {
    const results = dsinit(testSet.options, testSet.options.t);
    expect(testSet.results).toEqual(results);
  }
});
