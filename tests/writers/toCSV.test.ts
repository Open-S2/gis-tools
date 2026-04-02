import { FileReader } from '../../src/file.js';
import { toCSV } from '../../src/writers/index.js';
import { BufferWriter, JSONReader } from '../../src/index.js';
import { expect, test } from 'bun:test';

test('toCSV', async () => {
  const fileReader = new FileReader(`${__dirname}/fixtures/points.geojson`);
  const jsonReader = new JSONReader(fileReader);
  const bufWriter = new BufferWriter();
  await toCSV(bufWriter, [jsonReader], { properties: ['name'] });

  const string = new TextDecoder().decode(bufWriter.commit());
  expect(string).toEqual(
    'lon,lat,name\n144.9584,-37.8173,Melbourne\n149.1009,-35.3039,Canberra\n151.2144,-33.8766,Sydney\n',
  );
});
