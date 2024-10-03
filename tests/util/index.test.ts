import { base64ToArrayBuffer } from '../../src/util';
import { expect, test } from 'bun:test';

test('base64ToArrayBuffer', () => {
  expect(base64ToArrayBuffer('')).toEqual(new ArrayBuffer(0));
  expect(new Uint8Array(base64ToArrayBuffer('aGVsbG8='))).toEqual(
    new Uint8Array([104, 101, 108, 108, 111]),
  );
  const base64String = 'SGVsbG8sIHdvcmxkIQ=='; // Base64 for "Hello, world!"
  const uint8Array = new Uint8Array(base64ToArrayBuffer(base64String));
  expect(uint8Array).toEqual(
    new Uint8Array([0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x21]),
  );
});
