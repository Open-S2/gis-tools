import { convert, fromPolygon, fromVectorGeometry, mergeBBoxes, toDBF } from '../../index.js';

import type {
  BBOX,
  FeatureIterator,
  MValue,
  VectorFeatures,
  VectorGeometryType,
  VectorLineString,
  VectorMultiLineString,
  VectorMultiPoint,
  VectorPoint,
  Writer,
} from '../../index.js';

const PRJ = `GEOGCS["WGS 84",
  DATUM["WGS_1984",
    SPHEROID["WGS 84",6378137,298.257223563,AUTHORITY["EPSG","7030"]],
    AUTHORITY["EPSG","6326"]],
  PRIMEM["Greenwich",0,AUTHORITY["EPSG","8901"]],
  UNIT["degree",0.0174532925199433,AUTHORITY["EPSG","9122"]],
  AXIS["Latitude",NORTH],
  AXIS["Longitude",EAST],
  AUTHORITY["EPSG","4326"]]
`;

/**
 * # Shapefile Writer
 *
 * ## Description
 *
 * Given a writer and an array of iterators, write the input features property data into a SHP file
 *
 * NOTE: The correct way to store geometry in a shapefile is to only store one kind of geometry.
 * However, this libraries writer and reader do not enforce this.
 *
 * ## Usage
 *
 * #### Write to files
 * ```ts
 * import { toSHP, JSONReader } from 'gis-tools-ts';
 * import { FileReader, FileWriter } from 'gis-tools-ts/file';
 * // or use mmap reader if using bun
 * // import { MMapReader } from 'gis-tools-ts/mmap';
 * // or use a BufferWriter if you are using a browser
 * // import { BufferWriter } from 'gis-tools-ts';
 *
 * const fileReader = new FileReader(`${__dirname}/fixtures/points.geojson`);
 * const jsonReader = new JSONReader(fileReader);
 * const shpWriter = new FileWriter(`${__dirname}/fixtures/points.shp`);
 * const dbfWriter = new FileWriter(`${__dirname}/fixtures/points.dbf`);
 * const shxWriter = new FileWriter(`${__dirname}/fixtures/points.shx`);
 * const prjWriter = new FileWriter(`${__dirname}/fixtures/points.prj`);
 *
 * // store to outputs
 * await toSHP(shpWriter, [jsonReader], dbfWriter, shxWriter, prjWriter);
 * ```
 *
 * #### Zip the files
 * ```ts
 * import { zipFolder } from 'gis-tools-ts';
 *
 * const shpFile = await Bun.file(`${__dirname}/fixtures/points.shp`).arrayBuffer();
 * const dbfFile = await Bun.file(`${__dirname}/fixtures/points.dbf`).arrayBuffer();
 * const shxFile = await Bun.file(`${__dirname}/fixtures/points.shx`).arrayBuffer();
 * const prjFile = await Bun.file(`${__dirname}/fixtures/points.prj`).arrayBuffer();
 *
 * const zippedData = await zipFolder([
 *   { name: 'points.shp', comment: 'shapefile data', data: shpFile },
 *   { name: 'points.dbf', comment: 'properties data', data: dbfFile },
 *   { name: 'points.shx', comment: 'index data', data: shxFile },
 *   { name: 'points.prj', comment: 'projection', data: prjFile },
 * ]);
 * ```
 *
 * ## Links
 * - <https://www.esri.com/content/dam/esrisites/sitecore-archive/Files/Pdfs/library/whitepapers/pdfs/shapefile.pdf>
 *
 * @param shpWriter - the shapefile data container to write to
 * @param iterators - the collection of iterators to write
 * @param dbfWriter - the dbf data container to write to if provided (properties data)
 * @param shxWriter - the shx data container to write to if provided (index data)
 * @param prjWriter - the prj data container to write to if provided (projection)
 * @param onFeature - A function that takes a feature and returns a feature
 * @param mValue - If the data has the measurement modifier to the z value and how to find it
 */
export async function toSHP(
  shpWriter: Writer,
  iterators: FeatureIterator[],
  dbfWriter?: Writer,
  shxWriter?: Writer,
  prjWriter?: Writer,
  onFeature?: (feature: VectorFeatures) => VectorFeatures | undefined,
  mValue?: (m: MValue | undefined) => number | undefined,
): Promise<void> {
  // write to shp and shx
  await writeSHP(shpWriter, iterators, shxWriter, onFeature, mValue);
  // write to dbf
  if (dbfWriter !== undefined) await toDBF(dbfWriter, iterators);
  // write to prj
  await prjWriter?.appendString(PRJ);
}

async function writeSHP(
  shpWriter: Writer,
  iterators: FeatureIterator[],
  shxWriter?: Writer,
  _onFeature?: (feature: VectorFeatures) => VectorFeatures | undefined,
  mValue?: (m: MValue | undefined) => number | undefined,
): Promise<void> {
  const onFeature = _onFeature ?? ((feature) => feature);
  const globalBBox: BBOX = [Infinity, Infinity, -Infinity, -Infinity];
  let index = 0;
  let determinedType: number | undefined;

  await shpWriter.append(new Uint8Array(100));
  if (shxWriter !== undefined) {
    await shxWriter.append(new Uint8Array(100));
  }

  // write features
  for (const iterator of iterators) {
    for await (const feature of iterator) {
      const convertedFeatures = convert('WG', feature, true, false);
      for (const convFeature of convertedFeatures) {
        const userFeature = onFeature(convFeature);
        if (userFeature === undefined) continue;
        const { type, is3D } = userFeature.geometry;
        if (determinedType === undefined)
          determinedType = geometryType(type, is3D, mValue !== undefined);
        // handle bbox
        let bbox = convFeature.geometry.bbox;
        if (bbox === undefined) bbox = fromVectorGeometry(convFeature.geometry);
        mergeBBoxes(globalBBox, bbox);
        if (type === 'MultiPolygon') {
          for (const polygon of userFeature.geometry.coordinates) {
            const polyFeature: VectorFeatures = {
              ...userFeature,
              geometry: { coordinates: polygon, type: 'Polygon', is3D, bbox: fromPolygon(polygon) },
            };
            await writeFeature(polyFeature, shpWriter, index, shpWriter.tell(), shxWriter, mValue);
            index++;
          }
        } else {
          await writeFeature(userFeature, shpWriter, index, shpWriter.tell(), shxWriter, mValue);
          index++;
        }
      }
    }
  }

  // lastly store the file header
  await writeFileHeader(shpWriter, determinedType ?? 0, globalBBox);
  if (shxWriter !== undefined) await writeFileHeader(shxWriter, determinedType ?? 0, globalBBox);
}

async function writeFileHeader(writer: Writer, type: number, bbox: BBOX): Promise<void> {
  const header = new DataView(new ArrayBuffer(100));
  header.setInt32(0, 9994, false); // File Code
  header.setInt32(24, writer.tell(), false); // File Length
  header.setInt32(28, 1000, true); // version
  header.setInt32(32, type, true); // Shape Type
  // bbox
  header.setFloat64(36, bbox[0], true); // xmin
  header.setFloat64(44, bbox[1], true); // ymin
  header.setFloat64(52, bbox[2], true); // xmax
  header.setFloat64(60, bbox[3], true); // ymax
  if (bbox[4] !== undefined) header.setFloat64(68, bbox[4], true); // zmin
  if (bbox[5] !== undefined) header.setFloat64(76, bbox[5], true); // zmax
  // TODO:
  // Byte 84* Bounding Box Mmin Double Little
  // Byte 92* Bounding Box Mmax Double Little

  await writer.write(new Uint8Array(header.buffer), 0);
}

async function writeFeature(
  feature: VectorFeatures,
  shpWriter: Writer,
  index: number,
  shpOffset: number,
  shxWriter?: Writer,
  mValue?: (m: MValue | undefined) => number | undefined,
): Promise<void> {
  const hasM = mValue !== undefined;
  const { bbox, type, is3D, coordinates: coords } = feature.geometry;
  let data: DataView;

  if (type === 'Point') {
    data = is3D
      ? writePointZ(coords, mValue)
      : hasM
        ? writePointM(coords, mValue)
        : writePoint(coords);
  } else if (type === 'MultiPoint') {
    data = is3D
      ? writeMultiPointZ(coords, bbox!, mValue)
      : hasM
        ? writeMultiPointM(coords, bbox!, mValue)
        : writeMultiPoint(coords, bbox!);
  } else if (type === 'LineString') {
    data = is3D
      ? writeLineStringsZ([coords], bbox!, mValue)
      : hasM
        ? writeLineStringsM([coords], bbox!, mValue)
        : writeLineStrings([coords], bbox!);
  } else if (type === 'MultiLineString') {
    data = is3D
      ? writeLineStringsZ(coords, bbox!, mValue)
      : hasM
        ? writeLineStringsM(coords, bbox!, mValue)
        : writeLineStrings(coords, bbox!);
  } else if (type === 'Polygon') {
    data = is3D
      ? writeLineStringsZ(coords, bbox!, mValue, 15)
      : hasM
        ? writeLineStringsM(coords, bbox!, mValue, 25)
        : writeLineStrings(coords, bbox!, 5);
  } else {
    data = writeNull();
  }

  await writeRecordHeader(shpWriter, index, data.byteLength);
  await shpWriter.append(new Uint8Array(data.buffer, data.byteOffset, data.byteLength));
  if (shxWriter !== undefined) {
    await writeIndexRecord(shxWriter, shpOffset, data.byteLength);
  }
}

async function writeIndexRecord(
  shxWriter: Writer,
  shpOffset: number,
  contentLength: number,
): Promise<void> {
  const header = new DataView(new ArrayBuffer(8));
  header.setInt32(0, shpOffset >> 1, false);
  header.setInt32(4, contentLength >> 1, false);
  await shxWriter.append(new Uint8Array(header.buffer));
}

async function writeRecordHeader(
  shpWriter: Writer,
  recordNum: number,
  contentLength: number,
): Promise<void> {
  const header = new DataView(new ArrayBuffer(8));
  header.setInt32(0, recordNum, false);
  header.setInt32(4, contentLength >> 1, false);
  await shpWriter.append(new Uint8Array(header.buffer));
}

// Byte 0 Shape Type 0 Integer 1 Little
function writeNull(): DataView {
  const view = new DataView(new ArrayBuffer(20));
  view.setInt32(0, 0, true); // Type NULL
  return view;
}

// Byte 0 Shape Type 1 Integer 1 Little
// Byte 4 X X Double 1 Little
// Byte 12 Y Y Double 1 Little
function writePoint(point: VectorPoint): DataView {
  const view = new DataView(new ArrayBuffer(20));
  view.setInt32(0, 1, true); // Type 1 (POINT)
  view.setFloat64(4, point.x, true); // X
  view.setFloat64(12, point.y, true); // Y
  return view;
}

// Byte 0 Shape Type 8 Integer 1 Little
// Byte 4 Box Box Double 4 Little
// Byte 36 NumPoints NumPoints Integer 1 Little
// Byte 40 Points Points Point NumPoints Little
function writeMultiPoint(points: VectorMultiPoint, bbox: BBOX): DataView {
  const numPoints = points.length;

  // Header is 40 bytes. Each XY pair is exactly 16 bytes (2 * 8-byte Float64)
  const bufferSize = 40 + numPoints * 16;
  const view = new DataView(new ArrayBuffer(bufferSize));
  // 1. Write Header Metadata
  view.setInt32(0, 8, true); // Type 8 (MULTIPOINT)
  view.setFloat64(4, bbox[0], true); // xmin
  view.setFloat64(12, bbox[1], true); // ymin
  view.setFloat64(20, bbox[2], true); // xmax
  view.setFloat64(28, bbox[3], true); // ymax
  view.setInt32(36, numPoints, true); // NumPoints
  // 2. Stream XY Points sequentially
  let offset = 40;
  for (const point of points) {
    view.setFloat64(offset, point.x, true);
    view.setFloat64(offset + 8, point.y, true);
    offset += 16; // Step forward by exactly 16 bytes per point pair
  }

  return view;
}

// Byte 0 Shape Type 3 Integer 1 Little
// Byte 4 Box Box Double 4 Little
// Byte 36 NumParts NumParts Integer 1 Little
// Byte 40 NumPoints NumPoints Integer 1 Little
// Byte 44 Parts Parts Integer NumParts Little
// Byte X Points Points Point NumPoints Little
// Note: X = 44 + 4 * NumParts
function writeLineStrings(lines: VectorMultiLineString, bbox: BBOX, type = 3): DataView {
  const totalLines = lines.reduce((a, b) => a + b.length, 0);
  const totalParts = lines.length;

  // Header Metadata is exactly 44 bytes (Offsets 0 - 43)
  const bufferSize = 44 + totalParts * 4 + totalLines * 16;
  const view = new DataView(new ArrayBuffer(bufferSize));

  // 1. Write Header Metadata
  view.setInt32(0, type, true);
  view.setFloat64(4, bbox[0], true); // xmin
  view.setFloat64(12, bbox[1], true); // ymin
  view.setFloat64(20, bbox[2], true); // xmax
  view.setFloat64(28, bbox[3], true); // ymax
  view.setInt32(36, totalParts, true);
  view.setInt32(40, totalLines, true);
  // 2. Write Parts Index Array (Cumulative 0-based index offsets)
  let offset = 44;
  let partIndexAccumulator = 0;
  for (const line of lines) {
    view.setInt32(offset, partIndexAccumulator, true);
    offset += 4;
    partIndexAccumulator += line.length;
  }
  // 3. Stream all X, Y coordinates sequentially across all parts
  for (const line of lines) {
    for (const point of line) {
      view.setFloat64(offset, point.x, true);
      view.setFloat64(offset + 8, point.y, true);
      offset += 16;
    }
  }

  return view;
}

// Byte 0 Shape Type 21 Integer 1 Little
// Byte 4 X X Double 1 Little
// Byte 12 Y Y Double 1 Little
// Byte 20 M M Double 1 Little
function writePointM(
  point: VectorPoint,
  mValue: (m: MValue | undefined) => number | undefined,
): DataView {
  const M_NO_DATA = -Number.MAX_VALUE;
  const view = new DataView(new ArrayBuffer(28));

  view.setInt32(0, 21, true); // Type 21 (POINTM)
  view.setFloat64(4, point.x, true); // X
  view.setFloat64(12, point.y, true); // Y
  view.setFloat64(20, mValue(point.m) ?? M_NO_DATA, true); // M

  return view;
}

// Byte 0 Shape Type 28 Integer 1 Little
// Byte 4 Box Box Double 4 Little
// Byte 36 NumPoints NumPoints Integer 1 Little
// Byte 40 Points Points Point NumPoints Little
// Byte X* Mmin Mmin Double 1 Little
// Byte X+8* Mmax Mmax Double 1 Little
// Byte X+16* Marray Marray Double NumPoints Little
// Note: X = 40 + (16 * NumPoints)
// * optional
function writeMultiPointM(
  points: VectorMultiPoint,
  bbox: BBOX,
  mValue: (m: MValue | undefined) => number | undefined,
): DataView {
  const numPoints = points.length;
  const M_NO_DATA = -Number.MAX_VALUE;

  // Calculate explicit spec-compliant buffer size:
  // Header(40) + XY Array(numPoints * 16) + M Range(16) + M Array(numPoints * 8)
  const bufferSize = 40 + numPoints * 16 + 16 + numPoints * 8;
  const view = new DataView(new ArrayBuffer(bufferSize));

  // 1. Write Header Metadata
  view.setInt32(0, 28, true); // Type 28 (MULTIPOINTM)
  view.setFloat64(4, bbox[0], true); // xmin
  view.setFloat64(12, bbox[1], true); // ymin
  view.setFloat64(20, bbox[2], true); // xmax
  view.setFloat64(28, bbox[3], true); // ymax
  view.setInt32(36, numPoints, true); // NumPoints

  // 2. Phase A: Stream all X, Y coordinates sequentially
  let offset = 40;
  for (const point of points) {
    view.setFloat64(offset, point.x, true);
    view.setFloat64(offset + 8, point.y, true);
    offset += 16;
  }

  // 3. Phase B: Calculate M boundaries and stream the standalone M block
  let minM = Infinity;
  let maxM = -Infinity;
  const mRangeStartOffset = offset;
  offset += 16; // Skip past the 16-byte M-range placeholder for now

  for (const point of points) {
    const m = mValue(point.m) ?? M_NO_DATA;
    if (m !== M_NO_DATA) {
      minM = Math.min(minM, m);
      maxM = Math.max(maxM, m);
    }
    view.setFloat64(offset, m, true);
    offset += 8;
  }

  // Fallback if no valid measures were found
  if (minM === Infinity) {
    minM = 0.0;
    maxM = 0.0;
  }

  // Go back and populate the M Range bounding box right before the M array
  view.setFloat64(mRangeStartOffset, minM, true);
  view.setFloat64(mRangeStartOffset + 8, maxM, true);

  return view;
}

// Byte 0 Shape Type 23 Integer 1 Little
// Byte 4 Box Box Double 4 Little
// Byte 36 NumParts NumParts Integer 1 Little
// Byte 40 NumPoints NumPoints Integer 1 Little
// Byte 44 Parts Parts Integer NumParts Little
// Byte X Points Points Point NumPoints Little
// Byte Y* Mmin Mmin Double 1 Little
// Byte Y + 8* Mmax Mmax Double 1 Little
// Byte Y + 16* Marray Marray Double NumPoints Little
// Note: X = 44 + (4 * NumParts), Y = X + (16 * NumPoints)
// * optional
function writeLineStringsM(
  lines: VectorLineString[],
  bbox: BBOX,
  mValue: (m: MValue | undefined) => number | undefined,
  type = 23,
): DataView {
  const totalLines = lines.reduce((a, b) => a + b.length, 0);
  const totalParts = lines.length;
  const M_NO_DATA = -Number.MAX_VALUE;

  // Calculate explicit buffer size:
  // Header(44) + Parts Array(totalParts * 4) + XY Array(totalLines * 16) + M Range(16) + M Array(totalLines * 8)
  const bufferSize = 44 + totalParts * 4 + totalLines * 16 + 16 + totalLines * 8;
  const view = new DataView(new ArrayBuffer(bufferSize));

  // 1. Write Header Metadata
  view.setInt32(0, type, true);
  view.setFloat64(4, bbox[0], true); // xmin
  view.setFloat64(12, bbox[1], true); // ymin
  view.setFloat64(20, bbox[2], true); // xmax
  view.setFloat64(28, bbox[3], true); // ymax
  view.setInt32(36, totalParts, true);
  view.setInt32(40, totalLines, true);

  // 2. Write Parts Index Array (Cumulative offsets, not part lengths!)
  let offset = 44;
  let partIndexAccumulator = 0;
  for (const line of lines) {
    view.setInt32(offset, partIndexAccumulator, true);
    offset += 4;
    partIndexAccumulator += line.length;
  }

  // 3. Stream all X, Y coordinates sequentially
  for (const line of lines) {
    for (const point of line) {
      view.setFloat64(offset, point.x, true);
      view.setFloat64(offset + 8, point.y, true);
      offset += 16;
    }
  }

  // 4. Calculate M boundaries and stream the standalone M block
  let minM = Infinity;
  let maxM = -Infinity;
  const mRangeStartOffset = offset;
  offset += 16; // Skip past the 16-byte M-range placeholder for now

  for (const line of lines) {
    for (const point of line) {
      const m = mValue(point.m) ?? M_NO_DATA;

      if (m !== M_NO_DATA) {
        if (m < minM) minM = m;
        if (m > maxM) maxM = m;
      }

      view.setFloat64(offset, m, true);
      offset += 8;
    }
  }

  // Clean up bounds if no valid measures were encountered
  if (minM === Infinity) {
    minM = 0.0;
    maxM = 0.0;
  }

  // Go back and populate the M Range bounding box right before the M array
  view.setFloat64(mRangeStartOffset, minM, true);
  view.setFloat64(mRangeStartOffset + 8, maxM, true);

  return view;
}

// Byte 0 Shape Type 11 Integer 1 Little
// Byte 4 X X Double 1 Little
// Byte 12 Y Y Double 1 Little
// Byte 20 Z Z Double 1 Little
// Byte 28 Measure M Double 1 Little
// Byte 0 Shape Type 11 Integer 1 Little
// Byte 4 X X Double 1 Little
// Byte 12 Y Y Double 1 Little
// Byte 20 Z Z Double 1 Little
// Byte 28* M M Double 1 Little
// * optional
function writePointZ(
  point: VectorPoint,
  mValue?: (m: MValue | undefined) => number | undefined,
): DataView {
  const hasM = mValue !== undefined;
  const M_NO_DATA = -Number.MAX_VALUE;

  // Sized strictly: 28 bytes for X/Y/Z, 36 bytes if it includes M
  const bufferSize = hasM ? 36 : 28;
  const view = new DataView(new ArrayBuffer(bufferSize));
  view.setInt32(0, 11, true); // Type 11 (POINTZ)
  view.setFloat64(4, point.x, true); // X
  view.setFloat64(12, point.y, true); // Y
  view.setFloat64(20, point.z ?? 0.0, true); // Z

  if (hasM) {
    view.setFloat64(28, mValue(point.m) ?? M_NO_DATA, true); // M
  }

  return view;
}

// Byte 0 Shape Type 18 Integer 1 Little
// Byte 4 Box Box Double 4 Little
// Byte 36 NumPoints NumPoints Integer 1 Little
// Byte 40 Points Points Point NumPoints Little
// Byte X Zmin Zmin Double 1 Little
// Byte X+8 Zmax Zmax Double 1 Little
// Byte X+16 Zarray Zarray Double NumPoints Little
// Byte Y* Mmin Mmin Double 1 Little
// Byte Y+8* Mmax Mmax Double 1 Little
// Byte Y+16* Marray Marray Double NumPoints Little
// Note: X = 40 + (16 * NumPoints); Y = X + 16 + (8 * NumPoints)
// * optional
function writeMultiPointZ(
  points: VectorMultiPoint,
  bbox: BBOX,
  mValue?: (m: MValue | undefined) => number | undefined,
): DataView {
  const numPoints = points.length;
  const hasM = mValue !== undefined;
  const M_NO_DATA = -Number.MAX_VALUE;

  // 1. Calculate dynamic buffer allocation
  let bufferSize = 40 + numPoints * 16; // Header (40) + XY Array (N * 16)
  bufferSize += 16 + numPoints * 8; // Z Range (16) + Z Array (N * 8)
  if (hasM) bufferSize += 16 + numPoints * 8; // M Range (16) + M Array (N * 8)

  const view = new DataView(new ArrayBuffer(bufferSize));

  // 2. Write Main Metadata Header
  view.setInt32(0, 18, true); // Type 18 (MULTIPOINTZ)
  view.setFloat64(4, bbox[0], true); // xmin
  view.setFloat64(12, bbox[1], true); // ymin
  view.setFloat64(20, bbox[2], true); // xmax
  view.setFloat64(28, bbox[3], true); // ymax
  view.setInt32(36, numPoints, true); // NumPoints

  // 3. Phase A: Stream all X, Y coordinates sequentially
  let offset = 40;
  for (const point of points) {
    view.setFloat64(offset, point.x, true);
    view.setFloat64(offset + 8, point.y, true);
    offset += 16;
  }

  // 4. Phase B: Calculate Z boundaries and stream the standalone Z block
  let minZ = Infinity;
  let maxZ = -Infinity;
  const zRangeOffset = offset;
  offset += 16; // Skip past the 16-byte Z-range placeholder for now

  for (const point of points) {
    const z = point.z ?? 0.0;
    if (z < minZ) minZ = z;
    if (z > maxZ) maxZ = z;
    view.setFloat64(offset, z, true);
    offset += 8;
  }
  // Write the actual Z range limits back into the placeholder spot
  view.setFloat64(zRangeOffset, minZ === Infinity ? 0.0 : minZ, true);
  view.setFloat64(zRangeOffset + 8, maxZ === -Infinity ? 0.0 : maxZ, true);

  // 5. Phase C: Optional M Block serialization
  if (hasM) {
    let minM = Infinity;
    let maxM = -Infinity;
    const mRangeOffset = offset;
    offset += 16; // Skip past the 16-byte M-range placeholder

    for (const point of points) {
      const m = mValue(point.m) ?? M_NO_DATA;
      if (m !== M_NO_DATA) {
        minM = Math.min(minM, m);
        maxM = Math.max(maxM, m);
      }
      view.setFloat64(offset, m, true);
      offset += 8;
    }
    // Write the actual M range limits back into the placeholder spot
    view.setFloat64(mRangeOffset, minM === Infinity ? 0.0 : minM, true);
    view.setFloat64(mRangeOffset + 8, maxM === -Infinity ? 0.0 : maxM, true);
  }

  return view;
}

// Byte 0 Shape Type 13 Integer 1 Little
// Byte 4 Box Box Double 4 Little
// Byte 36 NumParts NumParts Integer 1 Little
// Byte 40 NumPoints NumPoints Integer 1 Little
// Byte 44 Parts Parts Integer NumParts Little
// Byte X Points Points Point NumPoints Little
// Byte Y Zmin Zmin Double 1 Little
// Byte Y + 8 Zmax Zmax Double 1 Little
// Byte Y + 16 Zarray Zarray Double NumPoints Little
// Byte Z* Mmin Mmin Double 1 Little
// Byte Z+8* Mmax Mmax Double 1 Little
// Byte Z+16* Marray Marray Double NumPoints Little
// Note: X = 44 + (4 * NumParts), Y = X + (16 * NumPoints), Z = Y + 16 + (8 * NumPoints)
// * optional
function writeLineStringsZ(
  lines: VectorLineString[],
  bbox: BBOX,
  mValue?: (m: MValue | undefined) => number | undefined,
  type = 13,
): DataView {
  const totalLines = lines.reduce((a, b) => a + b.length, 0);
  const totalParts = lines.length;
  const hasM = mValue !== undefined;
  const M_NO_DATA = -Number.MAX_VALUE;

  // Calculate explicit buffer size:
  // Header(44) + Parts(4 * totalParts) + Points(16 * totalLines) + Z Range(16) + Z Array(8 * totalLines)
  let bufferSize = 44 + totalParts * 4 + totalLines * 16; // Base line structure
  bufferSize += 16 + totalLines * 8; // Required Z dimension blocks
  if (hasM) bufferSize += 16 + totalLines * 8; // Optional M dimension blocks

  const view = new DataView(new ArrayBuffer(bufferSize));

  view.setInt32(0, type, true);
  view.setFloat64(4, bbox[0], true);
  view.setFloat64(12, bbox[1], true);
  view.setFloat64(20, bbox[2], true);
  view.setFloat64(28, bbox[3], true);
  view.setInt32(36, totalParts, true);
  view.setInt32(40, totalLines, true);

  let offset = 44;
  let partIndexAccumulator = 0;
  for (const line of lines) {
    view.setInt32(offset, partIndexAccumulator, true);
    offset += 4;
    partIndexAccumulator += line.length;
  }

  // 1. Write complete X/Y Block
  for (const line of lines) {
    for (const point of line) {
      view.setFloat64(offset, point.x, true);
      view.setFloat64(offset + 8, point.y, true);
      offset += 16;
    }
  }
  // 2. Write Z bounding limits followed immediately by flat Z array
  let minZ = Infinity,
    maxZ = -Infinity;
  const zStartOffset = offset;
  offset += 16; // Advance past placeholders to stream values
  for (const line of lines) {
    for (const point of line) {
      const z = point.z ?? 0.0;
      if (z < minZ) minZ = z;
      if (z > maxZ) maxZ = z;
      view.setFloat64(offset, z, true);
      offset += 8;
    }
  }
  view.setFloat64(zStartOffset, minZ === Infinity ? 0 : minZ, true);
  view.setFloat64(zStartOffset + 8, maxZ === -Infinity ? 0 : maxZ, true);
  // 3. Write optional M bounding limits followed immediately by flat M array
  if (hasM) {
    let minM = Infinity,
      maxM = -Infinity;
    const mStartOffset = offset;
    offset += 16;
    for (const line of lines) {
      for (const point of line) {
        const m = mValue(point.m) ?? M_NO_DATA;
        if (m !== M_NO_DATA) {
          if (m < minM) minM = m;
          if (m > maxM) maxM = m;
        }
        view.setFloat64(offset, m, true);
        offset += 8;
      }
    }
    view.setFloat64(mStartOffset, minM === Infinity ? 0 : minM, true);
    view.setFloat64(mStartOffset + 8, maxM === -Infinity ? 0 : maxM, true);
  }

  return view;
}

function geometryType(type: VectorGeometryType, hasZ: boolean, hasM: boolean): number {
  switch (type) {
    case 'Point':
      if (hasZ) return 11; // POINTZ (Contains X, Y, Z, and optional M)
      if (hasM) return 21; // POINTM (Contains X, Y, M)
      return 1; // POINT  (Contains X, Y)
    case 'LineString':
    case 'MultiLineString':
      if (hasZ) return 13; // POLYLINEZ
      if (hasM) return 23; // POLYLINEM
      return 3; // POLYLINE
    case 'Polygon':
    case 'MultiPolygon':
      if (hasZ) return 15; // POLYGONZ
      if (hasM) return 25; // POLYGONM
      return 5; // POLYGON
    case 'MultiPoint':
      if (hasZ) return 18; // MULTIPOINTZ
      if (hasM) return 28; // MULTIPOINTM
      return 8; // MULTIPOINT
    default:
      return 0; // NULL
  }
}
