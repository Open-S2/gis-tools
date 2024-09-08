function createDataView(): DataView {
  // Calculate total byte size based on the sizes of each data type
  const bufferSize = 1  // Uint8 = 1 byte
                   + 2  // Uint16 = 2 bytes
                   + 4  // Uint32 = 4 bytes
                   + 1  // Int8 = 1 byte
                   + 2  // Int16 = 2 bytes
                   + 4  // Int32 = 4 bytes
                   + 4  // Float32 = 4 bytes
                   + 8  // Float64 = 8 bytes
                   + 8  // BigUint64 = 8 bytes
                   + 8; // BigInt64 = 8 bytes

  // Create the buffer and wrap it in a DataView
  const buffer = new ArrayBuffer(bufferSize);
  const dv = new DataView(buffer);

  let offset = 0;

  // Set data in the DataView using the specified methods

  // Set Uint8 (1 byte)
  dv.setUint8(offset, 255);
  offset += 1;

  // Set Uint16 (2 bytes)
  dv.setUint16(offset, 65535, true); // littleEndian
  offset += 2;

  // Set Uint32 (4 bytes)
  dv.setUint32(offset, 4294967295, true); // littleEndian
  offset += 4;

  // Set Int8 (1 byte)
  dv.setInt8(offset, -128);
  offset += 1;

  // Set Int16 (2 bytes)
  dv.setInt16(offset, -32768, true); // littleEndian
  offset += 2;

  // Set Int32 (4 bytes)
  dv.setInt32(offset, -2147483648, true); // littleEndian
  offset += 4;

  // Set Float32 (4 bytes)
  dv.setFloat32(offset, 3.14, true); // littleEndian
  offset += 4;

  // Set Float64 (8 bytes)
  dv.setFloat64(offset, 3.14159265359, true); // littleEndian
  offset += 8;

  // Set BigUint64 (8 bytes)
  dv.setBigUint64(offset, 12345678901234567890n, true); // littleEndian
  offset += 8;

  // Set BigInt64 (8 bytes)
  dv.setBigInt64(offset, -1234567890123456789n, true); // littleEndian
  offset += 8;

  return dv;
}

// Usage
const dataView = createDataView();
console.log(dataView);
Bun.write('./tests/readers/fixtures/dv.bin', dataView.buffer);
