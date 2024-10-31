import { Satellite } from '../../src';
import { expect, test } from 'bun:test';

test('Test "bad" TLE data', async () => {
  const badTLE = await Bun.file(`${__dirname}/fixtures/badTLE.json`).json();
  for (const bTLE of badTLE) {
    const { tleLine1, tleLine2, description, error, errorMessage } = bTLE;
    const sat = new Satellite(`${tleLine1}\n${tleLine2}`);
    const sgp4Result = sat.sgp4(0);
    if (!('type' in sgp4Result)) throw new Error('No type in sgp4Result');
    expect('type' in sgp4Result, description).toBeTruthy();
    expect(sgp4Result.type).toEqual(error);
    expect(sgp4Result.error).toEqual(errorMessage);
  }
});
