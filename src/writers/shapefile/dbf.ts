import type { DBFRow, FeatureIterator, Writer } from '../../index.js';

export type DBFFileVersion =
  | 0x03 // dBase III without memo file
  | 0x83 // dBase III with memo file
  | 0x8b // dBase IV with memo file
  | 0x30 // Visual FoxPro 9 (may have memo file)
  | 0xf5; // FoxPro 2.x (may have memo file)

/**
 * Sweeps over all feature property collections to assemble a unified DBF column schema definition.
 * @param iterators - Multiple array of records containing the GeoJSON feature property values
 * @returns An array of field schema objects tailored to conform with DBF specifications
 */
export async function toDBFMeta(
  iterators: FeatureIterator[],
): Promise<[DBFRow[], featureCount: number]> {
  // Use a map to accumulate the most permissive size constraints across all rows
  const schemaMap = new Map<string, DBFRow>();
  let featureCount = 0;

  for (const iterator of iterators) {
    for await (const feature of iterator) {
      featureCount++;
      for (const [key, value] of Object.entries(feature.properties)) {
        if (value === undefined || value === null) continue;
        const normalizedKey = key.slice(0, 10);
        const dataType = getType(value);
        // Initialize configuration parameters for this column if missing
        if (!schemaMap.has(normalizedKey)) {
          schemaMap.set(normalizedKey, { name: normalizedKey, dataType, len: 0, decimal: 0 });
        }

        const currentMeta = schemaMap.get(normalizedKey)!;
        // Handle type widening if a column shifts from Boolean/Numeric up to String
        if (currentMeta.dataType !== dataType && currentMeta.dataType !== 'C') {
          currentMeta.dataType = 'C';
        }
        // Calculate formatting geometry constraints for the current specific value
        if (currentMeta.dataType === 'N') {
          const { totalLength, decimalPlaces } = getNumericConstraints(value);
          currentMeta.decimal = Math.max(currentMeta.decimal, decimalPlaces);
          currentMeta.len = Math.max(currentMeta.len, totalLength);
        } else {
          // Handle length constraints for strings or fallback characters
          const stringLen = String(value).length;
          currentMeta.len = Math.max(currentMeta.len, stringLen);
        }
      }
    }
  }

  // Cap fallback sizes and enforce maximum specification boundaries
  return [
    Array.from(schemaMap.values()).map((row) => {
      if (row.dataType === 'N') {
        row.len = Math.min(Math.max(row.len, 1), 18); // dBase III bounds: 1 to 18 characters
      } else {
        row.len = Math.min(Math.max(row.len, getSize(row.dataType)), 254);
      }
      return row;
    }),
    featureCount,
  ];
}

/**
 * # DBF Writer
 *
 * ## Description
 *
 * Given a writer and an array of iterators, write the input features property data into a DBF file
 *
 * ## Usage
 * ```ts
 * import { toDBF, JSONReader } from 'gis-tools-ts';
 * import { FileReader, FileWriter } from 'gis-tools-ts/file';
 * // or use mmap reader if using bun
 * // import { MMapReader } from 'gis-tools-ts/mmap';
 * // or use a BufferWriter if you are using a browser
 * // import { BufferWriter } from 'gis-tools-ts';
 *
 * const fileReader = new FileReader(`${__dirname}/fixtures/points.geojson`);
 * const jsonReader = new JSONReader(fileReader);
 * const bufWriter = new FileWriter(`${__dirname}/fixtures/points.dbf`);
 *
 * // store to singular output
 * await toDBF(bufWriter, [jsonReader]);
 * ```
 * @param writer - the writer to append strings to
 * @param iterators - the collection of iterators to write
 */
export async function toDBF(writer: Writer, iterators: FeatureIterator[]): Promise<void> {
  const textEncoder = new TextEncoder();
  const [meta, featureCount] = await toDBFMeta(iterators);

  // 1. Compute foundational structural offsets
  const fieldDescLength = 32 * meta.length + 1; // 32 bytes per descriptor + 0x0D terminator
  const headerLength = 32 + fieldDescLength;
  // Calculate raw bytes per row payload: 1 byte for the deletion flag + sum of field widths
  const bytesPerRecord = 1 + meta.reduce((acc, row) => acc + row.len, 0);

  // 2. Build and emit the primary 32-byte DBF File Header
  const headerBuffer = new ArrayBuffer(32);
  const headerView = new DataView(headerBuffer);
  const now = new Date();

  headerView.setUint8(0, 0x03); // dBase III signature version code
  headerView.setUint8(1, now.getFullYear() - 1900); // Year offset tracking
  headerView.setUint8(2, now.getMonth() + 1); // Month index (1-based)
  headerView.setUint8(3, now.getDate()); // Day of the month
  headerView.setUint32(4, featureCount, true); // Total record inventory size (Little Endian)
  headerView.setUint16(8, headerLength, true); // Total header block footprint
  headerView.setUint16(10, bytesPerRecord, true); // Total record byte sequence distance
  await writer.append(new Uint8Array(headerBuffer));

  // 3. Build and emit the Field Descriptor Array (32 bytes per field)
  const descriptorBuffer = new ArrayBuffer(32 * meta.length);
  const descriptorView = new DataView(descriptorBuffer);
  meta.forEach((field, i) => {
    const baseOffset = i * 32;
    // Encode field name directly to bytes safely bounded at 10 characters max
    const nameBytes = textEncoder.encode(field.name.slice(0, 10));
    for (let x = 0; x < 11; x++) {
      const byteValue = x < nameBytes.length ? nameBytes[x] : 0x00; // Null padded trailing buffer spaces
      descriptorView.setUint8(baseOffset + x, byteValue);
    }
    // Set field DataType, Field Width length, and decimal sub-precision parameters
    descriptorView.setUint8(baseOffset + 11, field.dataType.charCodeAt(0));
    descriptorView.setUint8(baseOffset + 16, field.len);
    descriptorView.setUint8(baseOffset + 17, field.decimal);
  });
  await writer.append(new Uint8Array(descriptorBuffer));
  // Write out the required field description array terminator block
  await writer.append(new Uint8Array([0x0d]));

  // 4. Transform and stream specific entity attribute rows
  // Allocate a reusable record buffer space matching our schema layout calculation
  const recordBuffer = new Uint8Array(bytesPerRecord);
  for (const iterator of iterators) {
    for await (const feature of iterator) {
      recordBuffer.fill(0x20);

      const row = feature.properties;
      let offset = 0;

      // Write deletion indicator token: 0x20 represents an active valid entity row
      recordBuffer[offset++] = 0x20;

      for (const field of meta) {
        const rawValue = row[field.name];
        let stringPayload = rawValue === null || rawValue === undefined ? '' : String(rawValue);
        if (field.dataType === 'N') {
          // Numbers must align exactly with your custom precision metrics
          const numValue = Number(rawValue);
          if (!isNaN(numValue)) {
            stringPayload = numValue.toFixed(field.decimal);
          }
          // Left-pad with empty space characters matching dBase standard formatting expectations
          stringPayload = stringPayload.padStart(field.len, ' ').slice(0, field.len);
        } else if (field.dataType === 'L') {
          // Enforce valid Boolean indicators: 'T' or 'F'
          const lower = stringPayload.toLowerCase();
          const isTrue = rawValue === true || ['true', 't', 'y'].includes(lower);
          stringPayload = isTrue ? 'T' : 'F';
        } else if (field.dataType === 'D') {
          // Dates require strict YYYYMMDD string format representations
          if (rawValue instanceof Date) {
            const y = rawValue.getFullYear();
            const m = String(rawValue.getMonth() + 1).padStart(2, '0');
            const d = String(rawValue.getDate()).padStart(2, '0');
            stringPayload = `${y}${m}${d}`;
          }
          stringPayload = stringPayload.padStart(field.len, ' ').slice(0, field.len);
        } else {
          // Standard alphanumeric string processing (Right-padded with spacing characters)
          stringPayload = stringPayload.padEnd(field.len, ' ').slice(0, field.len);
        }

        // Safe continuous allocation block layout injection via TextEncoder
        const encodedFieldBytes = textEncoder.encode(stringPayload);
        for (let b = 0; b < field.len; b++) {
          recordBuffer[offset++] = b < encodedFieldBytes.length ? encodedFieldBytes[b] : 0x20;
        }
      }
      // Stream out the record buffer sequence directly to disk or memory stream instantly
      await writer.append(recordBuffer);
    }
  }

  // 5. Emit structural standard EOF byte sequence completion flag
  await writer.append(new Uint8Array([0x1a]));
}

/**
 * Parses numeric properties safely to establish full string geometry and decimal requirements.
 *
 * @param value - the value to parse
 * @returns numeric constraints
 */
function getNumericConstraints(value: unknown): { totalLength: number; decimalPlaces: number } {
  const num = Number(value);
  if (isNaN(num)) return { totalLength: 18, decimalPlaces: 0 };

  // Convert to fixed notation to sidestep scientific notation snags (like 1e-7)
  // 15 is the IEEE-754 double precision ceiling for precise decimal representations
  let str = num.toFixed(15);

  // Strip floating point binary trailing zeros
  if (str.includes('.')) {
    str = str.replace(/0+$/, '');
    if (str.endsWith('.')) str = str.slice(0, -1);
  }

  const parts = str.split('.');
  const integerPart = parts[0].replace(/^-/, ''); // Exclude negative sign from index tracking
  const decimalPart = parts[1] ?? '';

  const decimalPlaces = Math.min(decimalPart.length, 15);

  // DBF Numeric width includes: integer digits, the negative sign (if any),
  // the '.' separator character, and the decimal spaces.
  const isNegative = num < 0 ? 1 : 0;
  const hasDot = decimalPlaces > 0 ? 1 : 0;
  const totalLength = integerPart.length + isNegative + hasDot + decimalPlaces;

  return { totalLength, decimalPlaces };
}

function getType(value: unknown): string {
  if (typeof value === 'number') return 'N';
  if (value instanceof Date) return 'D';
  if (typeof value === 'boolean') return 'L';

  if (typeof value === 'string') {
    const lower = value.toLowerCase();
    if (['true', 'false', 't', 'f', 'y', 'n'].includes(lower)) {
      return 'L';
    }
    return 'C';
  }

  return 'C';
}

function getSize(value: string): number {
  const defaults: Record<string, number> = { C: 254, L: 1, D: 8, B: 8 };
  return defaults[value] ?? 18;
}
