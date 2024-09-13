/**
 * pollyfill for string to array buffer
 * @param base64 - base64 encoded string
 * @returns converted ArrayBuffer of the string data
 */
export function base64ToArrayBuffer(base64: string): ArrayBuffer {
  const binaryString = atob(base64);
  const len = binaryString.length;
  const bytes = new Uint8Array(len);
  for (let i = 0; i < len; i++) bytes[i] = binaryString.charCodeAt(i);

  return bytes.buffer as ArrayBuffer;
}
