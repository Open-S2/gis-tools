import { buildServer } from '../../server';
import { fetchGTFSSystems } from '../../../src';
import { expect, test } from 'bun:test';

test('fetchGTFSSystems', async () => {
  const server = buildServer();

  const systems = await fetchGTFSSystems(
    `http://localhost:${server.port}/readers/gbfs/fixtures/systems.csv`,
  );
  expect(systems.length).toEqual(960);
  expect(systems.slice(0, 5)).toEqual([
    {
      authInfo: undefined,
      autoDiscoveryUrl: 'https://dubai.publicbikesystem.net/customer/gbfs/v2/gbfs.json',
      countryCode: 'AE',
      location: 'Dubai',
      name: 'Careem BIKE',
      supportedVersions: ['1.1', '2.3'],
      systemId: 'careem_bike',
      url: 'https://www.careem.com/en-ae/careem-bike/',
    },
    {
      authInfo: undefined,
      autoDiscoveryUrl: 'https://nordelta.publicbikesystem.net/customer/gbfs/v2/gbfs.json',
      countryCode: 'AR',
      location: 'Buenos Aires',
      name: 'Bike Nordelta',
      supportedVersions: ['1.1', '2.3'],
      systemId: 'bike_nordelta',
      url: 'https://bikeitau.com.br/nordelta/',
    },
    {
      authInfo: undefined,
      autoDiscoveryUrl: 'https://buenosaires.publicbikesystem.net/customer/gbfs/v2/gbfs.json',
      countryCode: 'AR',
      location: 'Buenos Aires',
      name: 'Ecobici',
      supportedVersions: ['1.1', '2.3'],
      systemId: 'bike_buenosaires',
      url: 'https://www.buenosaires.gob.ar/ecobici',
    },
    {
      authInfo: undefined,
      autoDiscoveryUrl: 'https://www.mibicitubici.gob.ar/opendata/gbfs.json',
      countryCode: 'AR',
      location: 'Rosario',
      name: 'MiBiciTuBici',
      supportedVersions: ['1.0'],
      systemId: 'biketobike',
      url: 'https://www.mibicitubici.gob.ar/',
    },
    {
      authInfo: undefined,
      autoDiscoveryUrl: 'https://gbfs.nextbike.net/maps/gbfs/v2/nextbike_al/gbfs.json',
      countryCode: 'AT',
      location: 'Linz',
      name: 'city bike Linz',
      supportedVersions: ['2.3'],
      systemId: 'nextbike_al',
      url: 'https://citybikelinz.at/',
    },
  ]);

  await server.stop();
});
