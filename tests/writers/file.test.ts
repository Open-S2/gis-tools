import { FileReader, FileWriter } from '../../src/file';
import { expect, test } from 'bun:test';

import tmp from 'tmp';

test('writers - File', async () => {
  const dir = tmp.dirSync({
    prefix: 'writers_file_test',
    template: 'test-XXXXXX',
    unsafeCleanup: true,
  });
  const writer = new FileWriter(`${dir.name}.txt`);
  await writer.appendString('test');
  writer.appendStringSync('test2');
  writer.close();

  const reader = new FileReader(`${dir.name}.txt`);
  expect(reader.parseString(0, 4)).toEqual('test');
  expect(reader.parseString(4, 5)).toEqual('test2');
  reader.close();
  dir.removeCallback();
});
