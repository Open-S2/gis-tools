import { MMapReader } from '../../src/mmap';
import { expect, test } from 'bun:test';

test('MMapReader', () => {
  const reader = new MMapReader(`${__dirname}/fixtures/dv.bin`);
  reader.setStringEncoding('utf-8');

  let offset = 0;
  expect(reader.getUint8(offset)).toBe(255);
  offset += 1;
  expect(reader.getUint16(offset, true)).toBe(65535);
  offset += 2;
  expect(reader.getUint32(offset, true)).toBe(4294967295);
  offset += 4;
  expect(reader.getInt8(offset)).toBe(-128);
  offset += 1;
  expect(reader.getInt16(offset, true)).toBe(-32768);
  offset += 2;
  expect(reader.getInt32(offset, true)).toBe(-2147483648);
  offset += 4;
  expect(reader.getFloat32(offset, true)).toBe(3.140000104904175);
  offset += 4;
  expect(reader.getFloat64(offset, true)).toBe(3.14159265359);
  offset += 8;
  expect(reader.getBigUint64(offset, true)).toBe(12345678901234567890n);
  offset += 8;
  expect(reader.getBigInt64(offset, true)).toBe(-1234567890123456789n);

  expect(() => {
    reader.slice(1_000, 2_000);
  }).toThrowError('Invalid slice range');

  expect(new Uint8Array(reader.slice(4, 8).buffer)).toEqual(new Uint8Array([255, 255, 255, 128]));
  reader.seek(4);
  expect(new Uint8Array(reader.seekSlice(4).buffer)).toEqual(new Uint8Array([255, 255, 255, 128]));
  expect(reader.tell()).toEqual(8);
});
