import { FileReader, FileWriter } from '../../src/file.js';
import { createTempPath, deletePath } from '../../tests/tmp.js';
import { expect, test } from 'bun:test';

test('writers - File', async () => {
  const dir = createTempPath('writers_file_test');
  const writer = new FileWriter(`${dir}/test_writers_file.txt`);
  await writer.appendString('test');
  writer.appendStringSync('test2');
  await writer.close();

  const reader = new FileReader(`${dir}/test_writers_file.txt`);
  expect(reader.parseString(0, 4)).toEqual('test');
  expect(reader.parseString(4, 5)).toEqual('test2');
  reader.close();

  deletePath(dir);
});
