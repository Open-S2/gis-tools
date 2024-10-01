// dist/geometry/util.js
function radToDeg(radians) {
  return radians * 180 / Math.PI;
}
function degToRad(deg) {
  return deg * Math.PI / 180;
}

// dist/geometry/s2/coords.js
function quadraticSTtoUV(s) {
  if (s >= 0.5)
    return 1 / 3 * (4 * s * s - 1);
  return 1 / 3 * (1 - 4 * (1 - s) * (1 - s));
}
function quadraticUVtoST(u) {
  const { sqrt } = Math;
  if (u >= 0)
    return 0.5 * sqrt(1 + 3 * u);
  return 1 - 0.5 * sqrt(1 - 3 * u);
}
function STtoIJ(s) {
  const { max, min, floor } = Math;
  return max(0, min(K_LIMIT_IJ - 1, floor(K_LIMIT_IJ * s)));
}
function IJtoST(i) {
  return i / K_LIMIT_IJ;
}
function faceUVtoXYZ(face, u, v) {
  switch (face) {
    case 0:
      return [1, u, v];
    case 1:
      return [-u, 1, v];
    case 2:
      return [-u, -v, 1];
    case 3:
      return [-1, -v, -u];
    case 4:
      return [v, -1, -u];
    default:
      return [v, u, -1];
  }
}
function faceXYZtoUV(face, xyz) {
  const [x, y, z] = xyz;
  switch (face) {
    case 0:
      return [y / x, z / x];
    case 1:
      return [-x / y, z / y];
    case 2:
      return [-x / z, -y / z];
    case 3:
      return [z / x, y / x];
    case 4:
      return [z / y, -x / y];
    default:
      return [-y / z, -x / z];
  }
}
function XYZtoFace(xyz) {
  const temp = xyz.map((n) => Math.abs(n));
  let face = temp[0] > temp[1] ? temp[0] > temp[2] ? 0 : 2 : temp[1] > temp[2] ? 1 : 2;
  if (xyz[face] < 0)
    face += 3;
  return face;
}
function XYZtoFaceUV(xyz) {
  const face = XYZtoFace(xyz);
  return [face, ...faceXYZtoUV(face, xyz)];
}
function xyzToLonLat(xyz) {
  const { atan2, sqrt } = Math;
  const [x, y, z] = xyz;
  return [radToDeg(atan2(y, x)), radToDeg(atan2(z, sqrt(x * x + y * y)))];
}
function lonLatToXYZ(lon, lat) {
  const { sin, cos } = Math;
  lon = degToRad(lon);
  lat = degToRad(lat);
  return [
    cos(lat) * cos(lon),
    cos(lat) * sin(lon),
    sin(lat)
  ];
}
var K_LIMIT_IJ = 1 << 30;

// dist/geometry/id.js
function initLookupCell(level, i, j, origOrientation, pos, orientation) {
  const kPosToOriengation = [1, 0, 0, 3];
  const kPosToIJ = [
    [0, 1, 3, 2],
    [0, 2, 3, 1],
    [3, 2, 0, 1],
    [3, 1, 0, 2]
  ];
  if (level === 4) {
    const ij = (i << 4) + j;
    LOOKUP_POS[(ij << 2) + origOrientation] = BigInt((pos << 2) + orientation);
    LOOKUP_IJ[(pos << 2) + origOrientation] = BigInt((ij << 2) + orientation);
  } else {
    level++;
    i <<= 1;
    j <<= 1;
    pos <<= 2;
    const r = kPosToIJ[orientation];
    initLookupCell(level, i + (r[0] >> 1), j + (r[0] & 1), origOrientation, pos, orientation ^ kPosToOriengation[0]);
    initLookupCell(level, i + (r[1] >> 1), j + (r[1] & 1), origOrientation, pos + 1, orientation ^ kPosToOriengation[1]);
    initLookupCell(level, i + (r[2] >> 1), j + (r[2] & 1), origOrientation, pos + 2, orientation ^ kPosToOriengation[2]);
    initLookupCell(level, i + (r[3] >> 1), j + (r[3] & 1), origOrientation, pos + 3, orientation ^ kPosToOriengation[3]);
  }
}
function fromFace(face) {
  return (BigInt(face) << POS_BITS) + (1n << 60n);
}
function fromS2Point(xyz) {
  const [face, i, j] = toIJ(xyz);
  return fromIJ(face, i, j);
}
function toFaceIJ(id) {
  const zoom = level(id);
  const [face, i, j] = toIJ2(id, zoom);
  return [face, zoom, i, j];
}
function fromIJ(face, i, j, level) {
  const bigFace = BigInt(face);
  let bigI = BigInt(i);
  let bigJ = BigInt(j);
  if (level !== undefined) {
    const levelB = BigInt(level);
    bigI = bigI << MAX_LEVEL - levelB;
    bigJ = bigJ << MAX_LEVEL - levelB;
  }
  let n = bigFace << 60n;
  let bits = bigFace & 1n;
  for (let k = 7n;k >= 0n; k--) {
    const kk = k * 4n;
    bits += (bigI >> kk & 15n) << NUM_FACES;
    bits += (bigJ >> kk & 15n) << 2n;
    bits = LOOKUP_POS[Number(bits)];
    n |= bits >> 2n << k * 8n;
    bits &= FACE_BITS;
  }
  const id = n * 2n + 1n;
  if (level !== undefined)
    return parent(id, level);
  return id;
}
function toIJ2(id, level) {
  let i = 0n;
  let j = 0n;
  const face = Number(id >> POS_BITS);
  let bits = BigInt(face) & 1n;
  for (let k = 7n;k >= 0n; k--) {
    const nbits = k === 7n ? 2n : 4n;
    bits += (id >> k * 8n + 1n & (1n << 2n * nbits) - 1n) << 2n;
    bits = LOOKUP_IJ[Number(bits)];
    i += bits >> NUM_FACES << k * 4n;
    j += (bits >> 2n & 15n) << k * 4n;
    bits &= FACE_BITS;
  }
  const lsb = id & ~id + 1n;
  if ((lsb & 1229782938247303424n) !== 0n)
    bits ^= 1n;
  if (level !== undefined) {
    level = BigInt(level);
    i = i >> MAX_LEVEL - level;
    j = j >> MAX_LEVEL - level;
  }
  return [face, Number(i), Number(j), Number(bits)];
}
function face(id) {
  const face2 = Number(id >> POS_BITS);
  return face2;
}
function isFace(id) {
  return (id & (1n << 60n) - 1n) === 0n;
}
function level(id) {
  let count = 0;
  let i = 0n;
  while ((id & 1n << i) === 0n && i < 60n) {
    i += 2n;
    count++;
  }
  return 30 - count;
}
function child(id, pos) {
  const newLSB = (id & ~id + 1n) >> 2n;
  return id + (2n * pos - FACE_BITS) * newLSB;
}
function children(id, orientation = 0) {
  const childs = [
    child(id, 0n),
    child(id, 3n),
    child(id, 2n),
    child(id, 1n)
  ];
  if (orientation === 0) {
    const tmp = childs[1];
    childs[1] = childs[3];
    childs[3] = tmp;
  }
  return childs;
}
function childrenIJ(face2, level2, i, j) {
  i = i << 1;
  j = j << 1;
  return [
    fromIJ(face2, i, j, level2 + 1),
    fromIJ(face2, i + 1, j, level2 + 1),
    fromIJ(face2, i, j + 1, level2 + 1),
    fromIJ(face2, i + 1, j + 1, level2 + 1)
  ];
}
function parent(id, level2) {
  const newLSB = level2 !== undefined ? 1n << 2n * (MAX_LEVEL - BigInt(level2)) : (id & ~id + 1n) << 2n;
  return id & ~newLSB + 1n | newLSB;
}
function range(id) {
  const lsb = id & ~id + 1n;
  return [id - (lsb - 1n), id + (lsb - 1n)];
}
function contains(a, b) {
  const [min, max] = range(a);
  return b >= min && b <= max;
}
function getVertices(id) {
  return getVerticesRaw(id).map(normalize);
}
function getVerticesRaw(id) {
  const f = face(id);
  const [uLow, uHigh, vLow, vHigh] = getBoundUV(id);
  return [
    faceUVtoXYZ(f, uLow, vLow),
    faceUVtoXYZ(f, uHigh, vLow),
    faceUVtoXYZ(f, uHigh, vHigh),
    faceUVtoXYZ(f, uLow, vHigh)
  ];
}
function getBoundUV(id) {
  const [, i, j] = toIJ2(id);
  const cellSize = getSizeIJ(id);
  const iLow = i & -cellSize;
  const jLow = j & -cellSize;
  const ijBounds = [iLow, iLow + cellSize, jLow, jLow + cellSize];
  return ijBounds.map((n) => quadraticSTtoUV(IJtoST(n)));
}
function getSizeIJ(id) {
  return 1 << K_MAX_LEVEL - level(id);
}
var LOOKUP_POS = [];
var LOOKUP_IJ = [];
var FACE_BITS = 3n;
var NUM_FACES = 6n;
var K_MAX_LEVEL = 30;
var MAX_LEVEL = 30n;
var POS_BITS = 61n;
for (let i = 0;i < 4; i++)
  initLookupCell(0, 0, 0, i, 0, i);

// dist/geometry/planets/earth.js
var EARTH_RADIUS = 6371008.8;
var EARTH_CIRCUMFERENCE = 2 * Math.PI * EARTH_RADIUS;

// dist/geometry/s2/point.js
function fromLonLat(lon, lat) {
  return lonLatToXYZ(lon, lat);
}
function fromUV(face2, u, v) {
  return faceUVtoXYZ(face2, u, v);
}
function fromST(face2, s, t) {
  const u = quadraticSTtoUV(s);
  const v = quadraticSTtoUV(t);
  return fromUV(face2, u, v);
}
function toUV2(xyz) {
  return XYZtoFaceUV(xyz);
}
function toST(xyz) {
  const [face2, u, v] = toUV2(xyz);
  return [face2, quadraticUVtoST(u), quadraticUVtoST(v)];
}
function toIJ(xyz, level2) {
  const [face2, s, t] = toST(xyz);
  let i = STtoIJ(s);
  let j = STtoIJ(t);
  if (level2 !== undefined) {
    i = i >> 30 - level2;
    j = j >> 30 - level2;
  }
  return [face2, i, j];
}
function toLonLat(xyz) {
  return xyzToLonLat(xyz);
}
function sub(a, b) {
  return [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
}
function normalize(xyz) {
  const len = length(xyz);
  xyz[0] /= len;
  xyz[1] /= len;
  xyz[2] /= len;
  return xyz;
}
function length(xyz) {
  return Math.sqrt(norm2(xyz));
}
function norm2(xyz) {
  return dot(xyz, xyz);
}
function dot(a, b) {
  return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

// dist/geometry/s2/convert.js
function toWM(data) {
  const { id, face: face2, properties, metadata, geometry } = data;
  convertGeometry(face2, geometry);
  return {
    id,
    type: "VectorFeature",
    properties,
    metadata,
    geometry
  };
}
function convertGeometry(face2, geometry) {
  const { type, coordinates } = geometry;
  if (type === "Point")
    convertGeometryPoint(face2, coordinates);
  else if (type === "MultiPoint")
    coordinates.forEach((point) => convertGeometryPoint(face2, point));
  else if (type === "LineString")
    coordinates.forEach((point) => convertGeometryPoint(face2, point));
  else if (type === "MultiLineString")
    coordinates.forEach((line) => line.forEach((point) => convertGeometryPoint(face2, point)));
  else if (type === "Polygon")
    coordinates.forEach((line) => line.forEach((point) => convertGeometryPoint(face2, point)));
  else if (type === "MultiPolygon")
    coordinates.forEach((polygon) => polygon.forEach((line) => line.forEach((point) => convertGeometryPoint(face2, point))));
  else {
    throw new Error("Invalid S2Geometry type");
  }
}
function convertGeometryPoint(face2, point) {
  const { x: s, y: t } = point;
  const [lon, lat] = toLonLat(fromST(face2, s, t));
  point.x = lon;
  point.y = lat;
}
// dist/geometry/bbox.js
function fromPoint(point) {
  const { x, y, z } = point;
  if (z !== undefined)
    return [x, y, x, y, z, z];
  return [x, y, x, y];
}
function extendBBox(bbox, point) {
  bbox = bbox ?? fromPoint(point);
  bbox = mergeBBoxes(bbox, fromPoint(point));
  return bbox;
}
function mergeBBoxes(b1, b2) {
  const { min, max } = Math;
  if (b1 === undefined)
    b1 = [...b2];
  b1[0] = min(b1[0] ?? b2[0], b2[0]);
  b1[1] = min(b1[1] ?? b2[1], b2[1]);
  b1[2] = max(b1[2] ?? b2[2], b2[2]);
  b1[3] = max(b1[3] ?? b2[3], b2[3]);
  if (b1.length > 4 || b2.length > 4) {
    b1[4] = min(b1[4] ?? 0, b2[4] ?? 0);
    b1[5] = max(b1[5] ?? 0, b2[5] ?? 0);
  }
  return b1;
}
function clipBBox(bb, axis, k1, k2) {
  const { min, max } = Math;
  const newBox = bb !== undefined ? [...bb] : [0, 0, 0, 0];
  if (axis === 0) {
    newBox[0] = max(newBox[0], k1);
    newBox[2] = min(newBox[2], k2);
  } else {
    newBox[1] = max(newBox[1], k1);
    newBox[3] = min(newBox[3], k2);
  }
  return newBox;
}

// dist/geometry/clip.js
function splitTile(tile, buffer = 0.0625) {
  const { id } = tile;
  const [face2, zoom, i, j] = toFaceIJ(id);
  const [blID, brID, tlID, trID] = childrenIJ(face2, zoom, i, j);
  const children2 = [new Tile(blID), new Tile(brID), new Tile(tlID), new Tile(trID)];
  const scale = 1 << zoom;
  const k1 = 0;
  const k2 = 0.5;
  const k3 = 0.5;
  const k4 = 1;
  let tl = null;
  let bl = null;
  let tr = null;
  let br = null;
  for (const [name, { features }] of Object.entries(tile.layers)) {
    const left = _clip(features, scale, i - k1, i + k3, 0, buffer);
    const right = _clip(features, scale, i + k2, i + k4, 0, buffer);
    if (left !== null) {
      bl = _clip(left, scale, j - k1, j + k3, 1, buffer);
      tl = _clip(left, scale, j + k2, j + k4, 1, buffer);
      if (bl !== null)
        for (const d of bl)
          children2[0].addFeature(d, name);
      if (tl !== null)
        for (const d of tl)
          children2[2].addFeature(d, name);
    }
    if (right !== null) {
      br = _clip(right, scale, j - k1, j + k3, 1, buffer);
      tr = _clip(right, scale, j + k2, j + k4, 1, buffer);
      if (br !== null)
        for (const d of br)
          children2[1].addFeature(d, name);
      if (tr !== null)
        for (const d of tr)
          children2[3].addFeature(d, name);
    }
  }
  return children2;
}
function _clip(features, scale, k1, k2, axis, baseBuffer) {
  k1 /= scale;
  k2 /= scale;
  const buffer = baseBuffer / scale;
  const k1b = k1 - buffer;
  const k2b = k2 + buffer;
  const clipped = [];
  for (const feature of features) {
    const { geometry } = feature;
    const { type } = geometry;
    let newGeometry = undefined;
    if (type === "Point")
      newGeometry = clipPoint(geometry, axis, k1, k2);
    else if (type === "MultiPoint")
      newGeometry = clipMultiPoint(geometry, axis, k1, k2);
    else if (type === "LineString")
      newGeometry = clipLineString(geometry, axis, k1b, k2b);
    else if (type === "MultiLineString")
      newGeometry = clipMultiLineString(geometry, axis, k1b, k2b);
    else if (type === "Polygon")
      newGeometry = clipPolygon(geometry, axis, k1b, k2b);
    else if (type === "MultiPolygon")
      newGeometry = clipMultiPolygon(geometry, axis, k1b, k2b);
    if (newGeometry !== undefined) {
      newGeometry.vecBBox = clipBBox(newGeometry.vecBBox, axis, k1b, k2b);
      clipped.push({ ...feature, geometry: newGeometry });
    }
  }
  return clipped.length > 0 ? clipped : null;
}
function clipPoint(geometry, axis, k1, k2) {
  const { type, is3D, coordinates, bbox, vecBBox } = geometry;
  const value = axis === 0 ? coordinates.x : coordinates.y;
  if (value >= k1 && value < k2)
    return { type, is3D, coordinates: { ...coordinates }, bbox, vecBBox };
}
function clipMultiPoint(geometry, axis, k1, k2) {
  const { type, is3D, coordinates, bbox } = geometry;
  let vecBBox = undefined;
  const points = coordinates.filter((point) => {
    const value = axis === 0 ? point.x : point.y;
    return value >= k1 && value < k2;
  }).map((p) => ({ ...p }));
  points.forEach((p) => vecBBox = extendBBox(vecBBox, p));
  if (points.length > 0)
    return { type, is3D, coordinates: points, bbox, vecBBox };
}
function clipLineString(geometry, axis, k1, k2) {
  const { is3D, coordinates: line, bbox, vecBBox } = geometry;
  const initO = geometry.offset ?? 0;
  const newOffsets = [];
  const newLines = [];
  for (const clip of _clipLine({ line, offset: initO }, k1, k2, axis, false)) {
    newOffsets.push(clip.offset);
    newLines.push(clip.line);
  }
  if (newLines.length === 0)
    return;
  return {
    type: "MultiLineString",
    is3D,
    coordinates: newLines,
    bbox,
    offset: newOffsets,
    vecBBox
  };
}
function clipMultiLineString(geometry, axis, k1, k2, isPolygon = false) {
  const { is3D, coordinates, bbox, vecBBox } = geometry;
  const initO = geometry.offset ?? coordinates.map((_) => 0);
  const newOffsets = [];
  const newLines = [];
  coordinates.forEach((line, i) => {
    for (const clip of _clipLine({ line, offset: initO[i] }, k1, k2, axis, isPolygon)) {
      newOffsets.push(clip.offset);
      newLines.push(clip.line);
    }
  });
  if (newLines.length === 0 || isPolygon && newLines[0].length === 0)
    return;
  return {
    type: isPolygon ? "Polygon" : "MultiLineString",
    is3D,
    coordinates: newLines,
    bbox,
    offset: newOffsets,
    vecBBox
  };
}
function clipPolygon(geometry, axis, k1, k2) {
  return clipMultiLineString(geometry, axis, k1, k2, true);
}
function clipMultiPolygon(geometry, axis, k1, k2) {
  const { is3D, coordinates, bbox, vecBBox } = geometry;
  const initO = geometry.offset ?? coordinates.map((l) => l.map(() => 0));
  const newCoordinates = [];
  const newOffsets = [];
  coordinates.forEach((polygon, p) => {
    const newPolygon = clipPolygon({ type: "Polygon", is3D, coordinates: polygon, bbox, offset: initO[p] }, axis, k1, k2);
    if (newPolygon !== undefined) {
      newCoordinates.push(newPolygon.coordinates);
      if (newPolygon.offset !== undefined)
        newOffsets.push(newPolygon.offset);
    }
  });
  if (newCoordinates.length === 0)
    return;
  return {
    type: "MultiPolygon",
    is3D,
    coordinates: newCoordinates,
    bbox,
    vecBBox,
    offset: newOffsets
  };
}
function clipLine(geom, bbox, isPolygon, offset = 0, buffer = 0.0625) {
  const res = [];
  const [left, bottom, right, top] = bbox;
  const horizontalClips = _clipLine({ line: geom, offset, vecBBox: [0, 0, 0, 0] }, left - buffer, right + buffer, 0, isPolygon);
  for (const clip of horizontalClips) {
    res.push(..._clipLine(clip, bottom - buffer, top + buffer, 1, isPolygon));
  }
  return res.map((clip) => {
    let vecBBox;
    for (const p of clip.line)
      vecBBox = extendBBox(vecBBox, p);
    clip.vecBBox = vecBBox;
    return clip;
  });
}
function _clipLine(input, k1, k2, axis, isPolygon) {
  const { line: geom, offset: startOffset } = input;
  const newGeom = [];
  let slice = [];
  let last = geom.length - 1;
  const intersect = axis === 0 ? intersectX : intersectY;
  let curOffset = startOffset;
  let accOffset = startOffset;
  let prevP = geom[0];
  let firstEnter = false;
  for (let i = 0;i < last; i++) {
    const { x: ax, y: ay, z: az, m: am } = geom[i];
    const { x: bx, y: by, z: bz, m: bm } = geom[i + 1];
    const a2 = axis === 0 ? ax : ay;
    const b = axis === 0 ? bx : by;
    const azNU = az !== undefined;
    const bzNU = bz !== undefined;
    const z = azNU && bzNU ? (az + bz) / 2 : azNU ? az : bzNU ? bz : undefined;
    let entered = false;
    let exited = false;
    let intP;
    if (a2 < k1) {
      if (b > k1) {
        intP = intersect(ax, ay, bx, by, k1, z, bm);
        slice.push(intP);
        entered = true;
      }
    } else if (a2 > k2) {
      if (b < k2) {
        intP = intersect(ax, ay, bx, by, k2, z, bm);
        slice.push(intP);
        entered = true;
      }
    } else {
      intP = { x: ax, y: ay, z: az, m: am };
      slice.push(intP);
    }
    if (intP !== undefined) {
      if (entered && !firstEnter) {
        curOffset = accOffset + distance(prevP, intP);
        firstEnter = true;
      }
    }
    if (b < k1 && a2 >= k1) {
      intP = intersect(ax, ay, bx, by, k1, z, bm ?? am);
      slice.push(intP);
      exited = true;
    }
    if (b > k2 && a2 <= k2) {
      intP = intersect(ax, ay, bx, by, k2, z, bm ?? am);
      slice.push(intP);
      exited = true;
    }
    accOffset += distance(prevP, geom[i + 1]);
    prevP = geom[i + 1];
    if (!isPolygon && exited) {
      newGeom.push({ line: slice, offset: curOffset });
      slice = [];
      firstEnter = false;
    }
  }
  const lastPoint = geom[last];
  const a = axis === 0 ? lastPoint.x : lastPoint.y;
  if (a >= k1 && a <= k2)
    slice.push({ ...lastPoint });
  if (slice.length > 0 && isPolygon) {
    last = slice.length - 1;
    const firstP = slice[0];
    if (last >= 1 && (slice[last].x !== firstP.x || slice[last].y !== firstP.y)) {
      slice.push({ ...firstP });
    }
  }
  if (slice.length > 0)
    newGeom.push({ line: slice, offset: curOffset });
  return newGeom;
}
function intersectX(ax, ay, bx, by, x, z, m) {
  const t = (x - ax) / (bx - ax);
  return { x, y: ay + (by - ay) * t, z, m, t: 1 };
}
function intersectY(ax, ay, bx, by, y, z, m) {
  const t = (y - ay) / (by - ay);
  return { x: ax + (bx - ax) * t, y, z, m, t: 1 };
}
function distance(p1, p2) {
  const { sqrt, pow } = Math;
  return sqrt(pow(p2.x - p1.x, 2) + pow(p2.y - p1.y, 2));
}

// dist/geometry/wm/convert.js
function toS2(data, tolerance, maxzoom, buildBBox) {
  const { id, properties, metadata } = data;
  const res = [];
  const vectorGeo = data.type === "VectorFeature" ? data.geometry : convertGeometry2(data.geometry, buildBBox);
  for (const { geometry, face: face2 } of convertVectorGeometry(vectorGeo, tolerance, maxzoom)) {
    res.push({
      id,
      type: "S2Feature",
      face: face2,
      properties,
      metadata,
      geometry
    });
  }
  return res;
}
function toVector(data, buildBBox) {
  const { id, properties, metadata } = data;
  const vectorGeo = convertGeometry2(data.geometry, buildBBox);
  return {
    id,
    type: "VectorFeature",
    properties,
    metadata,
    geometry: vectorGeo
  };
}
function convertPoint(point, m, bbox) {
  const newPoint = { x: point[0], y: point[1], z: point[2], m };
  if (bbox !== undefined) {
    const newBBox = extendBBox(bbox, newPoint);
    for (let i = 0;i < newBBox.length; i++)
      bbox[i] = newBBox[i];
  }
  return newPoint;
}
function convertGeometry2(geometry, buildBBox) {
  const { type, coordinates: coords, mValues, bbox } = geometry;
  const newBBox = buildBBox === true && bbox === undefined ? [] : undefined;
  let coordinates;
  if (type === "Point" || type === "Point3D")
    coordinates = convertPoint(coords, mValues, newBBox);
  else if (type === "MultiPoint" || type === "MultiPoint3D")
    coordinates = coords.map((point, i) => convertPoint(point, mValues?.[i], newBBox));
  else if (type === "LineString" || type === "LineString3D")
    coordinates = coords.map((point, i) => convertPoint(point, mValues?.[i], newBBox));
  else if (type === "MultiLineString" || type === "MultiLineString3D")
    coordinates = coords.map((line, i) => line.map((point, j) => convertPoint(point, mValues?.[i]?.[j], newBBox)));
  else if (type === "Polygon" || type === "Polygon3D")
    coordinates = coords.map((line, i) => line.map((point, j) => convertPoint(point, mValues?.[i]?.[j], newBBox)));
  else if (type === "MultiPolygon" || type === "MultiPolygon3D")
    coordinates = coords.map((polygon, i) => polygon.map((line, j) => line.map((point, k) => convertPoint(point, mValues?.[i]?.[j]?.[k], newBBox))));
  else {
    throw new Error("Invalid GeoJSON type");
  }
  const is3D = type.slice(-2) === "3D";
  return { type: type.replace("3D", ""), is3D, coordinates, bbox: newBBox ?? bbox };
}
function convertVectorGeometry(geometry, tolerance, maxzoom) {
  const { type } = geometry;
  let cGeo;
  if (type === "Point")
    cGeo = convertGeometryPoint2(geometry);
  else if (type === "MultiPoint")
    cGeo = convertGeometryMultiPoint(geometry);
  else if (type === "LineString")
    cGeo = convertGeometryLineString(geometry);
  else if (type === "MultiLineString")
    cGeo = convertGeometryMultiLineString(geometry);
  else if (type === "Polygon")
    cGeo = convertGeometryPolygon(geometry);
  else if (type === "MultiPolygon")
    cGeo = convertGeometryMultiPolygon(geometry);
  else {
    throw new Error("Either the conversion is not yet supported or Invalid S2Geometry type.");
  }
  if (tolerance !== undefined)
    for (const { geometry: geometry2 } of cGeo)
      buildSqDists(geometry2, tolerance, maxzoom);
  return cGeo;
}
function convertGeometryPoint2(geometry) {
  const { type, is3D, coordinates, bbox } = geometry;
  const { x: lon, y: lat, z, m } = coordinates;
  const [face2, s, t] = toST(fromLonLat(lon, lat));
  const vecBBox = fromPoint({ x: s, y: t, z });
  return [{ face: face2, geometry: { type, is3D, coordinates: { x: s, y: t, z, m }, bbox, vecBBox } }];
}
function convertGeometryMultiPoint(geometry) {
  const { is3D, coordinates, bbox } = geometry;
  return coordinates.flatMap((coordinates2) => convertGeometryPoint2({ type: "Point", is3D, coordinates: coordinates2, bbox }));
}
function convertGeometryLineString(geometry) {
  const { type, is3D, coordinates, bbox } = geometry;
  return convertLineString(coordinates, false).map(({ face: face2, line, offset, vecBBox }) => {
    return { face: face2, geometry: { type, is3D, coordinates: line, bbox, offset, vecBBox } };
  });
}
function convertGeometryMultiLineString(geometry) {
  const { coordinates, is3D, bbox } = geometry;
  return coordinates.flatMap((line) => convertLineString(line, false)).map(({ face: face2, line, offset, vecBBox }) => ({
    face: face2,
    geometry: { type: "LineString", is3D, coordinates: line, bbox, offset, vecBBox }
  }));
}
function convertGeometryPolygon(geometry) {
  const { type, is3D, coordinates, bbox } = geometry;
  const res = [];
  const outerRing = convertLineString(coordinates[0], true);
  const innerRings = coordinates.slice(1).flatMap((line) => convertLineString(line, true));
  for (const { face: face2, line, offset, vecBBox: polyBBox } of outerRing) {
    const polygon = [line];
    const polygonOffsets = [offset];
    for (const { face: innerFace, line: innerLine, offset: innerOffset, vecBBox } of innerRings) {
      if (innerFace === face2) {
        polygon.push(innerLine);
        polygonOffsets.push(innerOffset);
        mergeBBoxes(polyBBox, vecBBox);
      }
    }
    res.push({
      face: face2,
      geometry: {
        type,
        coordinates: polygon,
        is3D,
        bbox,
        offset: polygonOffsets,
        vecBBox: polyBBox
      }
    });
  }
  return res;
}
function convertGeometryMultiPolygon(geometry) {
  const { is3D, coordinates, bbox, offset } = geometry;
  return coordinates.flatMap((polygon, i) => convertGeometryPolygon({
    type: "Polygon",
    is3D,
    coordinates: polygon,
    bbox,
    offset: offset?.[i]
  }));
}
function convertLineString(line, isPolygon) {
  const res = [];
  const newGeometry = [];
  for (const { x: lon, y: lat, z, m } of line) {
    const [face2, s, t] = toST(fromLonLat(lon, lat));
    newGeometry.push({ face: face2, s, t, z, m });
  }
  const faces = new Set;
  newGeometry.forEach(({ face: face2 }) => faces.add(face2));
  for (const face2 of faces) {
    const line2 = [];
    for (const stPoint of newGeometry)
      line2.push(stPointToFace(face2, stPoint));
    const clippedLines = clipLine(line2, [0, 0, 1, 1], isPolygon);
    for (const { line: line3, offset, vecBBox } of clippedLines)
      res.push({ face: face2, line: line3, offset, vecBBox });
  }
  return res;
}
function toUnitScale(feature, tolerance, maxzoom) {
  const { geometry } = feature;
  const { type, coordinates } = geometry;
  if (type === "Point")
    projectPoint(coordinates, geometry);
  else if (type === "MultiPoint")
    coordinates.map((p) => projectPoint(p, geometry));
  else if (type === "LineString")
    coordinates.map((p) => projectPoint(p, geometry));
  else if (type === "MultiLineString")
    coordinates.map((l) => l.map((p) => projectPoint(p, geometry)));
  else if (type === "Polygon")
    coordinates.map((l) => l.map((p) => projectPoint(p, geometry)));
  else if (type === "MultiPolygon")
    coordinates.map((p) => p.map((l) => l.map((p2) => projectPoint(p2, geometry))));
  else {
    throw new Error("Either the conversion is not yet supported or Invalid S2Geometry type.");
  }
  if (tolerance !== undefined)
    buildSqDists(geometry, tolerance, maxzoom);
}
function projectPoint(input, geo) {
  const { x, y } = input;
  const sin = Math.sin(y * Math.PI / 180);
  const y2 = 0.5 - 0.25 * Math.log((1 + sin) / (1 - sin)) / Math.PI;
  input.x = x / 360 + 0.5;
  input.y = y2 < 0 ? 0 : y2 > 1 ? 1 : y2;
  geo.vecBBox = extendBBox(geo.vecBBox, input);
}
function stPointToFace(targetFace, stPoint) {
  const { face: curFace, s, t, z, m } = stPoint;
  if (targetFace === curFace)
    return { x: s, y: t, z, m };
  const [rot, x, y] = FACE_RULE_SET[targetFace][curFace];
  const [newS, newT] = rotate(rot, s, t);
  return { x: newS + x, y: newT + y, z, m };
}
function rotate(rot, s, t) {
  if (rot === 90)
    return [t, 1 - s];
  else if (rot === -90)
    return [1 - t, s];
  else
    return [s, t];
}
var FACE_RULE_SET = [
  [
    [0, 0, 0],
    [0, 1, 0],
    [90, 0, 1],
    [-90, 2, 0],
    [-90, -1, 0],
    [0, 0, -1]
  ],
  [
    [0, -1, 0],
    [0, 0, 0],
    [0, 0, 1],
    [-90, 1, 0],
    [-90, 2, 0],
    [90, 0, -1]
  ],
  [
    [-90, -1, 0],
    [0, 0, -1],
    [0, 0, 0],
    [0, 1, 0],
    [90, 0, 1],
    [-90, 2, 0]
  ],
  [
    [-90, 2, 0],
    [90, 0, -1],
    [0, -1, 0],
    [0, 0, 0],
    [0, 0, 1],
    [-90, 1, 0]
  ],
  [
    [90, 0, 1],
    [-90, 2, 0],
    [-90, -1, 0],
    [0, 0, -1],
    [0, 0, 0],
    [0, 1, 0]
  ],
  [
    [0, 0, 1],
    [-90, 1, 0],
    [-90, 2, 0],
    [90, 0, -1],
    [0, -1, 0],
    [0, 0, 0]
  ]
];
// dist/geometry/simplify.js
function buildSqDists(geometry, tolerance, maxzoom = 16) {
  const tol = Math.pow(tolerance / ((1 << maxzoom) * 4096), 2);
  const { type, coordinates: coords } = geometry;
  if (type === "LineString")
    buildSqDist(coords, 0, coords.length - 1, tol);
  else if (type === "MultiLineString")
    coords.forEach((line) => buildSqDist(line, 0, line.length - 1, tol));
  else if (type === "Polygon")
    coords.forEach((line) => buildSqDist(line, 0, line.length - 1, tol));
  else if (type === "MultiPolygon")
    coords.forEach((polygon) => polygon.forEach((line) => buildSqDist(line, 0, line.length - 1, tol)));
}
function buildSqDist(coords, first, last, sqTolerance) {
  coords[first].t = 1;
  _buildSqDist(coords, first, last, sqTolerance);
  coords[last].t = 1;
}
function _buildSqDist(coords, first, last, sqTolerance) {
  let maxSqDist = sqTolerance;
  const mid = last - first >> 1;
  let minPosToMid = last - first;
  let index;
  const { x: as, y: at } = coords[first];
  const { x: bs, y: bt } = coords[last];
  for (let i = first;i < last; i++) {
    const { x, y } = coords[i];
    const d = getSqSegDist(x, y, as, at, bs, bt);
    if (d > maxSqDist) {
      index = i;
      maxSqDist = d;
    } else if (d === maxSqDist) {
      const posToMid = Math.abs(i - mid);
      if (posToMid < minPosToMid) {
        index = i;
        minPosToMid = posToMid;
      }
    }
  }
  if (index !== undefined && maxSqDist > sqTolerance) {
    if (index - first > 1)
      _buildSqDist(coords, first, index, sqTolerance);
    coords[index].t = maxSqDist;
    if (last - index > 1)
      _buildSqDist(coords, index, last, sqTolerance);
  }
}
function getSqSegDist(ps, pt, s, t, bs, bt) {
  let ds = bs - s;
  let dt = bt - t;
  if (ds !== 0 || dt !== 0) {
    const m = ((ps - s) * ds + (pt - t) * dt) / (ds * ds + dt * dt);
    if (m > 1) {
      s = bs;
      t = bt;
    } else if (m > 0) {
      s += ds * m;
      t += dt * m;
    }
  }
  ds = ps - s;
  dt = pt - t;
  return ds * ds + dt * dt;
}
function simplify(geometry, tolerance, zoom, maxzoom = 16) {
  const zoomTol = zoom >= maxzoom ? 0 : tolerance / ((1 << zoom) * 4096);
  const { type, coordinates: coords } = geometry;
  if (type === "LineString")
    geometry.coordinates = simplifyLine(coords, zoomTol, false, false);
  else if (type === "MultiLineString")
    geometry.coordinates = coords.map((line) => simplifyLine(line, zoomTol, false, false));
  else if (type === "Polygon")
    geometry.coordinates = coords.map((line, i) => simplifyLine(line, zoomTol, true, i === 0));
  else if (type === "MultiPolygon")
    geometry.coordinates = coords.map((polygon) => polygon.map((line, i) => simplifyLine(line, zoomTol, true, i === 0)));
}
function simplifyLine(line, tolerance, isPolygon, isOuter) {
  const sqTolerance = tolerance * tolerance;
  const size = line.length;
  if (tolerance > 0 && size < (isPolygon ? sqTolerance : tolerance))
    return line;
  const ring = [];
  for (const point of line) {
    if (tolerance === 0 || (point.t ?? 0) > sqTolerance)
      ring.push({ ...point });
  }
  if (isPolygon)
    rewind(ring, isOuter);
  return ring;
}
function rewind(ring, clockwise) {
  let area = 0;
  for (let i = 0, len = ring.length, j = len - 2;i < len; j = i, i += 2) {
    area += (ring[i].x - ring[j].x) * (ring[i].y + ring[j].y);
  }
  if (area > 0 === clockwise) {
    for (let i = 0, len = ring.length;i < len / 2; i += 2) {
      swapPoints(ring, i, len - i - 1);
    }
  }
}
function swapPoints(ring, i, j) {
  const tmp = ring[i];
  ring[i] = ring[j];
  ring[j] = tmp;
}
// dist/geometry/convert.js
function convert3(projection, data, tolerance, maxzoom, buildBBox, toUnitScale2 = true) {
  const res = [];
  if (data.type === "Feature") {
    res.push(...convertFeature(projection, data, toUnitScale2, tolerance, maxzoom, buildBBox));
  } else if (data.type === "VectorFeature") {
    res.push(...convertVectorFeature(projection, data, toUnitScale2, tolerance, maxzoom));
  } else if (data.type === "FeatureCollection") {
    for (const feature of data.features) {
      if (feature.type === "Feature")
        res.push(...convertFeature(projection, feature, toUnitScale2, tolerance, maxzoom, buildBBox));
      else
        res.push(...convertVectorFeature(projection, feature, toUnitScale2, tolerance, maxzoom));
    }
  } else if (data.type === "S2Feature") {
    res.push(convertS2Feature(projection, data, toUnitScale2, tolerance, maxzoom));
  } else if (data.type === "S2FeatureCollection") {
    for (const feature of data.features) {
      res.push(convertS2Feature(projection, feature, toUnitScale2, tolerance, maxzoom));
    }
  }
  return res;
}
function convertFeature(projection, data, toUS, tolerance, maxzoom, buildBBox) {
  if (projection === "WM") {
    const vf = toVector(data, buildBBox);
    if (toUS)
      toUnitScale(vf, tolerance, maxzoom);
    return [vf];
  } else {
    return toS2(data, tolerance, maxzoom, buildBBox);
  }
}
function convertVectorFeature(projection, data, toUS, tolerance, maxzoom) {
  if (projection === "WM") {
    if (toUS)
      toUnitScale(data, tolerance, maxzoom);
    return [data];
  } else {
    return toS2(data, tolerance, maxzoom);
  }
}
function convertS2Feature(projection, data, toUS, tolerance, maxzoom) {
  if (projection === "WM") {
    const vf = toWM(data);
    if (toUS)
      toUnitScale(vf, tolerance, maxzoom);
    return vf;
  } else {
    return data;
  }
}

// dist/dataStructures/tile.js
function _transform(geometry, zoom, ti, tj) {
  const { type, coordinates } = geometry;
  zoom = 1 << zoom;
  if (type === "Point")
    transformPoint(coordinates, zoom, ti, tj);
  else if (type === "MultiPoint" || type === "LineString")
    coordinates.forEach((p) => transformPoint(p, zoom, ti, tj));
  else if (type === "MultiLineString" || type === "Polygon")
    coordinates.forEach((l) => l.forEach((p) => transformPoint(p, zoom, ti, tj)));
  else if (type === "MultiPolygon")
    coordinates.forEach((p) => p.forEach((l) => l.forEach((p2) => transformPoint(p2, zoom, ti, tj))));
}
function transformPoint(vp, zoom, ti, tj) {
  vp.x = vp.x * zoom - ti;
  vp.y = vp.y * zoom - tj;
}

class Tile {
  id;
  layers;
  transformed;
  constructor(id, layers = {}, transformed = false) {
    this.id = id;
    this.layers = layers;
    this.transformed = transformed;
  }
  isEmpty() {
    for (const layer of Object.values(this.layers)) {
      if (layer.features.length > 0)
        return false;
    }
    return true;
  }
  addFeature(feature, layer) {
    const { metadata = {} } = feature;
    const layerName = metadata.layer ?? layer ?? "default";
    if (this.layers[layerName] === undefined) {
      this.layers[layerName] = new Layer(layerName, []);
    }
    this.layers[layerName].features.push(feature);
  }
  transform(tolerance, maxzoom) {
    const { transformed, id, layers } = this;
    if (transformed)
      return;
    const [, zoom, i, j] = toFaceIJ(id);
    for (const layer of Object.values(layers)) {
      for (const feature of layer.features) {
        if (tolerance > 0)
          simplify(feature.geometry, tolerance, zoom, maxzoom);
        _transform(feature.geometry, zoom, i, j);
      }
    }
    this.transformed = true;
  }
}

class Layer {
  name;
  features;
  constructor(name, features = []) {
    this.name = name;
    this.features = features;
  }
}

class TileStore {
  minzoom = 0;
  maxzoom = 18;
  faces = new Set;
  indexMaxzoom = 4;
  tolerance = 3;
  buffer = 0.0625;
  tiles = new Map;
  projection;
  constructor(data, options) {
    this.minzoom = options?.minzoom ?? 0;
    this.maxzoom = options?.maxzoom ?? 20;
    this.indexMaxzoom = options?.indexMaxzoom ?? 4;
    this.tolerance = options?.tolerance ?? 3;
    this.buffer = options?.buffer ?? 64;
    if (options?.projection !== undefined)
      this.projection = options.projection;
    else if (data.type === "Feature" || data.type === "FeatureCollection")
      this.projection = "WM";
    else
      this.projection = "S2";
    if (this.maxzoom < 0 || this.maxzoom > 20)
      throw new Error("maxzoom should be in the 0-20 range");
    const features = convert3(this.projection, data);
    for (const feature of features)
      this.addFeature(feature);
    for (let face2 = 0;face2 < 6; face2++) {
      const id = fromFace(face2);
      this.splitTile(id);
    }
  }
  addFeature(feature) {
    const { faces, tiles } = this;
    const face2 = feature.face ?? 0;
    const id = fromFace(face2);
    let tile = tiles.get(id);
    if (tile === undefined) {
      faces.add(face2);
      tile = new Tile(id);
      tiles.set(id, tile);
    }
    tile?.addFeature(feature);
  }
  splitTile(startID, endID, endZoom = this.maxzoom) {
    const { buffer, tiles, tolerance, maxzoom, indexMaxzoom } = this;
    const stack = [startID];
    while (stack.length > 0) {
      const stackID = stack.pop();
      if (stackID === undefined)
        break;
      const tile = tiles.get(stackID);
      if (tile === undefined || tile.isEmpty() || tile.transformed)
        continue;
      const tileZoom = level(tile.id);
      if (tileZoom >= maxzoom || endID === undefined && tileZoom >= indexMaxzoom || endID !== undefined && (tileZoom > endZoom || !contains(tile.id, endID)))
        continue;
      const [blID, brID, tlID, trID] = splitTile(tile, buffer);
      tiles.set(blID.id, blID);
      tiles.set(brID.id, brID);
      tiles.set(tlID.id, tlID);
      tiles.set(trID.id, trID);
      tile.transform(tolerance, maxzoom);
      stack.push(blID.id, brID.id, tlID.id, trID.id);
    }
  }
  getTile(id) {
    const { tiles, faces } = this;
    const zoom = level(id);
    const face2 = face(id);
    if (zoom < 0 || zoom > 20 || !faces.has(face2))
      return;
    let pID = id;
    while (!tiles.has(pID) && !isFace(pID)) {
      pID = parent(pID);
    }
    this.splitTile(pID, id, zoom);
    return tiles.get(id);
  }
}
// dist/dataStructures/uint64.js
function toCell(id2) {
  if (typeof id2 === "object")
    return id2;
  const bigint = BigInt(id2);
  return {
    low: Number(bigint & 0xffffffffn),
    high: Number(bigint >> 32n) & 4294967295
  };
}
function fromS2Point2(point) {
  const id2 = fromS2Point(point);
  return toCell(id2);
}
function compare(a, b) {
  if (a.high < b.high)
    return -1;
  if (a.high > b.high)
    return 1;
  if (a.low < b.low)
    return -1;
  if (a.low > b.low)
    return 1;
  return 0;
}

// dist/dataStore/vector/index.js
class Vector {
  #store = [];
  push(value) {
    this.#store.push(value);
  }
  async get(index) {
    return await this.#store[index];
  }
  get length() {
    return this.#store.length;
  }
  *values() {
    for (const value of this.#store)
      yield value;
  }
  sort() {
    this.#store.sort((a, b) => {
      return compare(a.cell, b.cell);
    });
  }
  [Symbol.iterator]() {
    return this.values();
  }
}

// node_modules/.deno/s2json-spec@1.5.5/node_modules/s2json-spec/dist/util.js
function radToDeg2(radians) {
  return radians * 180 / Math.PI;
}
function degToRad2(deg) {
  return deg * Math.PI / 180;
}
var EARTH_RADIUS2 = 6371008.8;
var EARTH_CIRCUMFERENCE2 = 2 * Math.PI * EARTH_RADIUS2;

// node_modules/.deno/s2json-spec@1.5.5/node_modules/s2json-spec/dist/s2/s2Coords.js
function quadraticSTtoUV2(s) {
  if (s >= 0.5)
    return 1 / 3 * (4 * s * s - 1);
  return 1 / 3 * (1 - 4 * (1 - s) * (1 - s));
}
function quadraticUVtoST2(u) {
  const { sqrt } = Math;
  if (u >= 0)
    return 0.5 * sqrt(1 + 3 * u);
  return 1 - 0.5 * sqrt(1 - 3 * u);
}
function faceUVtoXYZ2(face2, u, v) {
  switch (face2) {
    case 0:
      return [1, u, v];
    case 1:
      return [-u, 1, v];
    case 2:
      return [-u, -v, 1];
    case 3:
      return [-1, -v, -u];
    case 4:
      return [v, -1, -u];
    default:
      return [v, u, -1];
  }
}
function faceXYZtoUV2(face2, xyz) {
  const [x, y, z] = xyz;
  switch (face2) {
    case 0:
      return [y / x, z / x];
    case 1:
      return [-x / y, z / y];
    case 2:
      return [-x / z, -y / z];
    case 3:
      return [z / x, y / x];
    case 4:
      return [z / y, -x / y];
    default:
      return [-y / z, -x / z];
  }
}
function XYZtoFace2(xyz) {
  const temp = xyz.map((n) => Math.abs(n));
  let face2 = temp[0] > temp[1] ? temp[0] > temp[2] ? 0 : 2 : temp[1] > temp[2] ? 1 : 2;
  if (xyz[face2] < 0)
    face2 += 3;
  return face2;
}
function XYZtoFaceUV2(xyz) {
  const face2 = XYZtoFace2(xyz);
  return [face2, ...faceXYZtoUV2(face2, xyz)];
}
function xyzToLonLat2(xyz) {
  const { atan2, sqrt } = Math;
  const [x, y, z] = xyz;
  return [radToDeg2(atan2(y, x)), radToDeg2(atan2(z, sqrt(x * x + y * y)))];
}
function lonLatToXYZ2(lon, lat) {
  const { sin, cos } = Math;
  lon = degToRad2(lon);
  lat = degToRad2(lat);
  return [
    cos(lat) * cos(lon),
    cos(lat) * sin(lon),
    sin(lat)
  ];
}
var K_LIMIT_IJ2 = 1 << 30;

// node_modules/.deno/s2json-spec@1.5.5/node_modules/s2json-spec/dist/id.js
function initLookupCell2(level2, i, j, origOrientation, pos, orientation) {
  const kPosToOriengation = [1, 0, 0, 3];
  const kPosToIJ = [
    [0, 1, 3, 2],
    [0, 2, 3, 1],
    [3, 2, 0, 1],
    [3, 1, 0, 2]
  ];
  if (level2 === 4) {
    const ij = (i << 4) + j;
    LOOKUP_POS2[(ij << 2) + origOrientation] = BigInt((pos << 2) + orientation);
    LOOKUP_IJ2[(pos << 2) + origOrientation] = BigInt((ij << 2) + orientation);
  } else {
    level2++;
    i <<= 1;
    j <<= 1;
    pos <<= 2;
    const r = kPosToIJ[orientation];
    initLookupCell2(level2, i + (r[0] >> 1), j + (r[0] & 1), origOrientation, pos, orientation ^ kPosToOriengation[0]);
    initLookupCell2(level2, i + (r[1] >> 1), j + (r[1] & 1), origOrientation, pos + 1, orientation ^ kPosToOriengation[1]);
    initLookupCell2(level2, i + (r[2] >> 1), j + (r[2] & 1), origOrientation, pos + 2, orientation ^ kPosToOriengation[2]);
    initLookupCell2(level2, i + (r[3] >> 1), j + (r[3] & 1), origOrientation, pos + 3, orientation ^ kPosToOriengation[3]);
  }
}
function fromFace2(face2) {
  return (BigInt(face2) << POS_BITS2) + (1n << 60n);
}
function toFaceIJ2(id2) {
  const zoom = level2(id2);
  const [face2, i, j] = toIJ4(id2, zoom);
  return [face2, zoom, i, j];
}
function fromIJ2(face2, i, j, level2) {
  const bigFace = BigInt(face2);
  let bigI = BigInt(i);
  let bigJ = BigInt(j);
  if (level2 !== undefined) {
    const levelB = BigInt(level2);
    bigI = bigI << MAX_LEVEL2 - levelB;
    bigJ = bigJ << MAX_LEVEL2 - levelB;
  }
  let n = bigFace << 60n;
  let bits = bigFace & 1n;
  for (let k = 7n;k >= 0n; k--) {
    const kk = k * 4n;
    bits += (bigI >> kk & 15n) << NUM_FACES2;
    bits += (bigJ >> kk & 15n) << 2n;
    bits = LOOKUP_POS2[Number(bits)];
    n |= bits >> 2n << k * 8n;
    bits &= FACE_BITS2;
  }
  const id2 = n * 2n + 1n;
  if (level2 !== undefined)
    return parent2(id2, level2);
  return id2;
}
function toIJ4(id2, level2) {
  let i = 0n;
  let j = 0n;
  const face2 = Number(id2 >> POS_BITS2);
  let bits = BigInt(face2) & 1n;
  for (let k = 7n;k >= 0n; k--) {
    const nbits = k === 7n ? 2n : 4n;
    bits += (id2 >> k * 8n + 1n & (1n << 2n * nbits) - 1n) << 2n;
    bits = LOOKUP_IJ2[Number(bits)];
    i += bits >> NUM_FACES2 << k * 4n;
    j += (bits >> 2n & 15n) << k * 4n;
    bits &= FACE_BITS2;
  }
  const lsb = id2 & ~id2 + 1n;
  if ((lsb & 1229782938247303424n) !== 0n)
    bits ^= 1n;
  if (level2 !== undefined) {
    level2 = BigInt(level2);
    i = i >> MAX_LEVEL2 - level2;
    j = j >> MAX_LEVEL2 - level2;
  }
  return [face2, Number(i), Number(j), Number(bits)];
}
function face2(id2) {
  const face3 = Number(id2 >> POS_BITS2);
  return face3;
}
function isFace2(id2) {
  return (id2 & (1n << 60n) - 1n) === 0n;
}
function level2(id2) {
  let count = 0;
  let i = 0n;
  while ((id2 & 1n << i) === 0n && i < 60n) {
    i += 2n;
    count++;
  }
  return 30 - count;
}
function childrenIJ2(face3, level3, i, j) {
  i = i << 1;
  j = j << 1;
  return [
    fromIJ2(face3, i, j, level3 + 1),
    fromIJ2(face3, i + 1, j, level3 + 1),
    fromIJ2(face3, i, j + 1, level3 + 1),
    fromIJ2(face3, i + 1, j + 1, level3 + 1)
  ];
}
function parent2(id2, level3) {
  const newLSB = level3 !== undefined ? 1n << 2n * (MAX_LEVEL2 - BigInt(level3)) : (id2 & ~id2 + 1n) << 2n;
  return id2 & ~newLSB + 1n | newLSB;
}
function range2(id2) {
  const lsb = id2 & ~id2 + 1n;
  return [id2 - (lsb - 1n), id2 + (lsb - 1n)];
}
function contains2(a, b) {
  const [min, max] = range2(a);
  return b >= min && b <= max;
}
var LOOKUP_POS2 = [];
var LOOKUP_IJ2 = [];
var FACE_BITS2 = 3n;
var NUM_FACES2 = 6n;
var MAX_LEVEL2 = 30n;
var POS_BITS2 = 61n;
for (let i = 0;i < 4; i++)
  initLookupCell2(0, 0, 0, i, 0, i);

// node_modules/.deno/s2json-spec@1.5.5/node_modules/s2json-spec/dist/s2/s2Point.js
function fromLonLat2(lon, lat) {
  return lonLatToXYZ2(lon, lat);
}
function fromUV2(face3, u, v) {
  return faceUVtoXYZ2(face3, u, v);
}
function fromST3(face3, s, t) {
  const u = quadraticSTtoUV2(s);
  const v = quadraticSTtoUV2(t);
  return fromUV2(face3, u, v);
}
function toUV4(xyz) {
  return XYZtoFaceUV2(xyz);
}
function toST2(xyz) {
  const [face3, u, v] = toUV4(xyz);
  return [face3, quadraticUVtoST2(u), quadraticUVtoST2(v)];
}
function toLonLat2(xyz) {
  return xyzToLonLat2(xyz);
}

// node_modules/.deno/s2json-spec@1.5.5/node_modules/s2json-spec/dist/s2/convert.js
function toWM2(data) {
  const { id: id2, face: face3, properties, metadata, geometry: geometry2 } = data;
  convertGeometry3(face3, geometry2);
  return {
    id: id2,
    type: "VectorFeature",
    properties,
    metadata,
    geometry: geometry2
  };
}
function convertGeometry3(face3, geometry2) {
  const { type, coordinates } = geometry2;
  if (type === "Point")
    convertGeometryPoint3(face3, coordinates);
  else if (type === "MultiPoint")
    coordinates.forEach((point) => convertGeometryPoint3(face3, point));
  else if (type === "LineString")
    coordinates.forEach((point) => convertGeometryPoint3(face3, point));
  else if (type === "MultiLineString")
    coordinates.forEach((line) => line.forEach((point) => convertGeometryPoint3(face3, point)));
  else if (type === "Polygon")
    coordinates.forEach((line) => line.forEach((point) => convertGeometryPoint3(face3, point)));
  else if (type === "MultiPolygon")
    coordinates.forEach((polygon) => polygon.forEach((line) => line.forEach((point) => convertGeometryPoint3(face3, point))));
  else {
    throw new Error("Invalid S2Geometry type");
  }
}
function convertGeometryPoint3(face3, point) {
  const { x: s, y: t } = point;
  const [lon, lat] = toLonLat2(fromST3(face3, s, t));
  point.x = lon;
  point.y = lat;
}
// node_modules/.deno/s2json-spec@1.5.5/node_modules/s2json-spec/dist/bbox.js
function fromPoint2(point) {
  const { x, y, z } = point;
  if (z !== undefined)
    return [x, y, x, y, z, z];
  return [x, y, x, y];
}
function extendBBox2(bbox2, point) {
  bbox2 = bbox2 ?? fromPoint2(point);
  bbox2 = mergeBBoxes2(bbox2, fromPoint2(point));
  return bbox2;
}
function mergeBBoxes2(b1, b2) {
  const { min, max } = Math;
  b1[0] = min(b1[0] ?? b2[0], b2[0]);
  b1[1] = min(b1[1] ?? b2[1], b2[1]);
  b1[2] = max(b1[2] ?? b2[2], b2[2]);
  b1[3] = max(b1[3] ?? b2[3], b2[3]);
  if (b1.length > 4 || b2.length > 4) {
    b1[4] = min(b1[4] ?? 0, b2[4] ?? 0);
    b1[5] = max(b1[5] ?? 0, b2[5] ?? 0);
  }
  return b1;
}
function clipBBox2(bb, axis, k1, k2) {
  const { min, max } = Math;
  const newBox = bb !== undefined ? [...bb] : [0, 0, 0, 0];
  if (axis === 0) {
    newBox[0] = max(newBox[0], k1);
    newBox[2] = min(newBox[2], k2);
  } else {
    newBox[1] = max(newBox[1], k1);
    newBox[3] = min(newBox[3], k2);
  }
  return newBox;
}

// node_modules/.deno/s2json-spec@1.5.5/node_modules/s2json-spec/dist/clip.js
function splitTile2(tile2, buffer = 0.0625) {
  const { id: id2 } = tile2;
  const [face3, zoom, i, j] = toFaceIJ2(id2);
  const [blID, brID, tlID, trID] = childrenIJ2(face3, zoom, i, j);
  const children2 = [new Tile2(blID), new Tile2(brID), new Tile2(tlID), new Tile2(trID)];
  const scale = 1 << zoom;
  const k1 = 0;
  const k2 = 0.5;
  const k3 = 0.5;
  const k4 = 1;
  let tl = null;
  let bl = null;
  let tr = null;
  let br = null;
  for (const [name, { features }] of Object.entries(tile2.layers)) {
    const left = _clip2(features, scale, i - k1, i + k3, 0, buffer);
    const right = _clip2(features, scale, i + k2, i + k4, 0, buffer);
    if (left !== null) {
      bl = _clip2(left, scale, j - k1, j + k3, 1, buffer);
      tl = _clip2(left, scale, j + k2, j + k4, 1, buffer);
      if (bl !== null)
        for (const d of bl)
          children2[0].addFeature(d, name);
      if (tl !== null)
        for (const d of tl)
          children2[2].addFeature(d, name);
    }
    if (right !== null) {
      br = _clip2(right, scale, j - k1, j + k3, 1, buffer);
      tr = _clip2(right, scale, j + k2, j + k4, 1, buffer);
      if (br !== null)
        for (const d of br)
          children2[1].addFeature(d, name);
      if (tr !== null)
        for (const d of tr)
          children2[3].addFeature(d, name);
    }
  }
  return children2;
}
function _clip2(features, scale, k1, k2, axis, baseBuffer) {
  k1 /= scale;
  k2 /= scale;
  const buffer = baseBuffer / scale;
  const k1b = k1 - buffer;
  const k2b = k2 + buffer;
  const clipped = [];
  for (const feature of features) {
    const { geometry: geometry2 } = feature;
    const { type } = geometry2;
    let newGeometry = undefined;
    if (type === "Point")
      newGeometry = clipPoint2(geometry2, axis, k1, k2);
    else if (type === "MultiPoint")
      newGeometry = clipMultiPoint2(geometry2, axis, k1, k2);
    else if (type === "LineString")
      newGeometry = clipLineString2(geometry2, axis, k1b, k2b);
    else if (type === "MultiLineString")
      newGeometry = clipMultiLineString2(geometry2, axis, k1b, k2b);
    else if (type === "Polygon")
      newGeometry = clipPolygon2(geometry2, axis, k1b, k2b);
    else if (type === "MultiPolygon")
      newGeometry = clipMultiPolygon2(geometry2, axis, k1b, k2b);
    if (newGeometry !== undefined) {
      newGeometry.vecBBox = clipBBox2(newGeometry.vecBBox, axis, k1b, k2b);
      clipped.push({ ...feature, geometry: newGeometry });
    }
  }
  return clipped.length > 0 ? clipped : null;
}
function clipPoint2(geometry2, axis, k1, k2) {
  const { type, is3D, coordinates, bbox: bbox2, vecBBox } = geometry2;
  const value = axis === 0 ? coordinates.x : coordinates.y;
  if (value >= k1 && value < k2)
    return { type, is3D, coordinates: { ...coordinates }, bbox: bbox2, vecBBox };
}
function clipMultiPoint2(geometry2, axis, k1, k2) {
  const { type, is3D, coordinates, bbox: bbox2 } = geometry2;
  let vecBBox = undefined;
  const points = coordinates.filter((point) => {
    const value = axis === 0 ? point.x : point.y;
    return value >= k1 && value < k2;
  }).map((p) => ({ ...p }));
  points.forEach((p) => vecBBox = extendBBox2(vecBBox, p));
  if (points.length > 0)
    return { type, is3D, coordinates: points, bbox: bbox2, vecBBox };
}
function clipLineString2(geometry2, axis, k1, k2) {
  const { is3D, coordinates: line, bbox: bbox2, vecBBox } = geometry2;
  const initO = geometry2.offset ?? 0;
  const newOffsets = [];
  const newLines = [];
  for (const clip of _clipLine2({ line, offset: initO }, k1, k2, axis, false)) {
    newOffsets.push(clip.offset);
    newLines.push(clip.line);
  }
  if (newLines.length === 0)
    return;
  return {
    type: "MultiLineString",
    is3D,
    coordinates: newLines,
    bbox: bbox2,
    offset: newOffsets,
    vecBBox
  };
}
function clipMultiLineString2(geometry2, axis, k1, k2, isPolygon = false) {
  const { is3D, coordinates, bbox: bbox2, vecBBox } = geometry2;
  const initO = geometry2.offset ?? coordinates.map((_) => 0);
  const newOffsets = [];
  const newLines = [];
  coordinates.forEach((line, i) => {
    for (const clip of _clipLine2({ line, offset: initO[i] }, k1, k2, axis, isPolygon)) {
      newOffsets.push(clip.offset);
      newLines.push(clip.line);
    }
  });
  if (newLines.length === 0 || isPolygon && newLines[0].length === 0)
    return;
  return {
    type: isPolygon ? "Polygon" : "MultiLineString",
    is3D,
    coordinates: newLines,
    bbox: bbox2,
    offset: newOffsets,
    vecBBox
  };
}
function clipPolygon2(geometry2, axis, k1, k2) {
  return clipMultiLineString2(geometry2, axis, k1, k2, true);
}
function clipMultiPolygon2(geometry2, axis, k1, k2) {
  const { is3D, coordinates, bbox: bbox2, vecBBox } = geometry2;
  const initO = geometry2.offset ?? coordinates.map((l) => l.map(() => 0));
  const newCoordinates = [];
  const newOffsets = [];
  coordinates.forEach((polygon, p) => {
    const newPolygon = clipPolygon2({ type: "Polygon", is3D, coordinates: polygon, bbox: bbox2, offset: initO[p] }, axis, k1, k2);
    if (newPolygon !== undefined) {
      newCoordinates.push(newPolygon.coordinates);
      if (newPolygon.offset !== undefined)
        newOffsets.push(newPolygon.offset);
    }
  });
  if (newCoordinates.length === 0)
    return;
  return {
    type: "MultiPolygon",
    is3D,
    coordinates: newCoordinates,
    bbox: bbox2,
    vecBBox,
    offset: newOffsets
  };
}
function clipLine2(geom, bbox2, isPolygon, offset = 0, buffer = 0.0625) {
  const res = [];
  const [left, bottom, right, top] = bbox2;
  const horizontalClips = _clipLine2({ line: geom, offset, vecBBox: [0, 0, 0, 0] }, left - buffer, right + buffer, 0, isPolygon);
  for (const clip of horizontalClips) {
    res.push(..._clipLine2(clip, bottom - buffer, top + buffer, 1, isPolygon));
  }
  return res.map((clip) => {
    let vecBBox;
    for (const p of clip.line)
      vecBBox = extendBBox2(vecBBox, p);
    clip.vecBBox = vecBBox;
    return clip;
  });
}
function _clipLine2(input, k1, k2, axis, isPolygon) {
  const { line: geom, offset: startOffset } = input;
  const newGeom = [];
  let slice = [];
  let last = geom.length - 1;
  const intersect = axis === 0 ? intersectX2 : intersectY2;
  let curOffset = startOffset;
  let accOffset = startOffset;
  let prevP = geom[0];
  let firstEnter = false;
  for (let i = 0;i < last; i++) {
    const { x: ax, y: ay, z: az, m: am } = geom[i];
    const { x: bx, y: by, z: bz, m: bm } = geom[i + 1];
    const a2 = axis === 0 ? ax : ay;
    const b = axis === 0 ? bx : by;
    const z = az && bz ? (az + bz) / 2 : az ? az : bz ? bz : undefined;
    let entered = false;
    let exited = false;
    let intP;
    if (a2 < k1) {
      if (b > k1) {
        intP = intersect(ax, ay, bx, by, k1, z, bm);
        slice.push(intP);
        entered = true;
      }
    } else if (a2 > k2) {
      if (b < k2) {
        intP = intersect(ax, ay, bx, by, k2, z, bm);
        slice.push(intP);
        entered = true;
      }
    } else {
      intP = { x: ax, y: ay, z: az, m: am };
      slice.push(intP);
    }
    if (intP) {
      if (entered && !firstEnter) {
        curOffset = accOffset + distance2(prevP, intP);
        firstEnter = true;
      }
    }
    if (b < k1 && a2 >= k1) {
      intP = intersect(ax, ay, bx, by, k1, z, bm ?? am);
      slice.push(intP);
      exited = true;
    }
    if (b > k2 && a2 <= k2) {
      intP = intersect(ax, ay, bx, by, k2, z, bm ?? am);
      slice.push(intP);
      exited = true;
    }
    accOffset += distance2(prevP, geom[i + 1]);
    prevP = geom[i + 1];
    if (!isPolygon && exited) {
      newGeom.push({ line: slice, offset: curOffset });
      slice = [];
      firstEnter = false;
    }
  }
  const lastPoint = geom[last];
  const a = axis === 0 ? lastPoint.x : lastPoint.y;
  if (a >= k1 && a <= k2)
    slice.push({ ...lastPoint });
  if (slice.length > 0 && isPolygon) {
    last = slice.length - 1;
    const firstP = slice[0];
    if (last >= 1 && (slice[last].x !== firstP.x || slice[last].y !== firstP.y)) {
      slice.push({ ...firstP });
    }
  }
  if (slice.length > 0)
    newGeom.push({ line: slice, offset: curOffset });
  return newGeom;
}
function intersectX2(ax, ay, bx, by, x, z, m) {
  const t = (x - ax) / (bx - ax);
  return { x, y: ay + (by - ay) * t, z, m, t: 1 };
}
function intersectY2(ax, ay, bx, by, y, z, m) {
  const t = (y - ay) / (by - ay);
  return { x: ax + (bx - ax) * t, y, z, m, t: 1 };
}
function distance2(p1, p2) {
  const { sqrt, pow } = Math;
  return sqrt(pow(p2.x - p1.x, 2) + pow(p2.y - p1.y, 2));
}

// node_modules/.deno/s2json-spec@1.5.5/node_modules/s2json-spec/dist/wm/convert.js
function toS22(data, tolerance, maxzoom, buildBBox) {
  const { id: id2, properties, metadata } = data;
  const res = [];
  const vectorGeo = data.type === "VectorFeature" ? data.geometry : convertGeometry4(data.geometry, buildBBox);
  for (const { geometry: geometry2, face: face3 } of convertVectorGeometry2(vectorGeo, tolerance, maxzoom)) {
    res.push({
      id: id2,
      type: "S2Feature",
      face: face3,
      properties,
      metadata,
      geometry: geometry2
    });
  }
  return res;
}
function toVector2(data, buildBBox) {
  const { id: id2, properties, metadata } = data;
  const vectorGeo = convertGeometry4(data.geometry, buildBBox);
  return {
    id: id2,
    type: "VectorFeature",
    properties,
    metadata,
    geometry: vectorGeo
  };
}
function convertPoint2(point, m, bbox2) {
  const newPoint = { x: point[0], y: point[1], z: point[2], m };
  if (bbox2 !== undefined) {
    const newBBox = extendBBox2(bbox2, newPoint);
    for (let i = 0;i < newBBox.length; i++)
      bbox2[i] = newBBox[i];
  }
  return newPoint;
}
function convertGeometry4(geometry2, buildBBox) {
  const { type, coordinates: coords, mValues, bbox: bbox2 } = geometry2;
  const newBBox = buildBBox && bbox2 === undefined ? [] : undefined;
  let coordinates;
  if (type === "Point" || type === "Point3D")
    coordinates = convertPoint2(coords, mValues, newBBox);
  else if (type === "MultiPoint" || type === "MultiPoint3D")
    coordinates = coords.map((point, i) => convertPoint2(point, mValues?.[i], newBBox));
  else if (type === "LineString" || type === "LineString3D")
    coordinates = coords.map((point, i) => convertPoint2(point, mValues?.[i], newBBox));
  else if (type === "MultiLineString" || type === "MultiLineString3D")
    coordinates = coords.map((line, i) => line.map((point, j) => convertPoint2(point, mValues?.[i]?.[j], newBBox)));
  else if (type === "Polygon" || type === "Polygon3D")
    coordinates = coords.map((line, i) => line.map((point, j) => convertPoint2(point, mValues?.[i]?.[j], newBBox)));
  else if (type === "MultiPolygon" || type === "MultiPolygon3D")
    coordinates = coords.map((polygon, i) => polygon.map((line, j) => line.map((point, k) => convertPoint2(point, mValues?.[i]?.[j]?.[k], newBBox))));
  else {
    throw new Error("Invalid GeoJSON type");
  }
  const is3D = type.slice(-2) === "3D";
  return { type: type.replace("3D", ""), is3D, coordinates, bbox: newBBox ?? bbox2 };
}
function convertVectorGeometry2(geometry2, tolerance, maxzoom) {
  const { type } = geometry2;
  let cGeo;
  if (type === "Point")
    cGeo = convertGeometryPoint4(geometry2);
  else if (type === "MultiPoint")
    cGeo = convertGeometryMultiPoint2(geometry2);
  else if (type === "LineString")
    cGeo = convertGeometryLineString2(geometry2);
  else if (type === "MultiLineString")
    cGeo = convertGeometryMultiLineString2(geometry2);
  else if (type === "Polygon")
    cGeo = convertGeometryPolygon2(geometry2);
  else if (type === "MultiPolygon")
    cGeo = convertGeometryMultiPolygon2(geometry2);
  else {
    throw new Error("Either the conversion is not yet supported or Invalid S2Geometry type.");
  }
  if (tolerance !== undefined)
    for (const { geometry: geometry3 } of cGeo)
      buildSqDists2(geometry3, tolerance, maxzoom);
  return cGeo;
}
function convertGeometryPoint4(geometry2) {
  const { type, is3D, coordinates, bbox: bbox2 } = geometry2;
  const { x: lon, y: lat, z, m } = coordinates;
  const [face3, s, t] = toST2(fromLonLat2(lon, lat));
  const vecBBox = fromPoint2({ x: s, y: t, z });
  return [{ face: face3, geometry: { type, is3D, coordinates: { x: s, y: t, z, m }, bbox: bbox2, vecBBox } }];
}
function convertGeometryMultiPoint2(geometry2) {
  const { is3D, coordinates, bbox: bbox2 } = geometry2;
  return coordinates.flatMap((coordinates2) => convertGeometryPoint4({ type: "Point", is3D, coordinates: coordinates2, bbox: bbox2 }));
}
function convertGeometryLineString2(geometry2) {
  const { type, is3D, coordinates, bbox: bbox2 } = geometry2;
  return convertLineString2(coordinates, false).map(({ face: face3, line, offset, vecBBox }) => {
    return { face: face3, geometry: { type, is3D, coordinates: line, bbox: bbox2, offset, vecBBox } };
  });
}
function convertGeometryMultiLineString2(geometry2) {
  const { coordinates, is3D, bbox: bbox2 } = geometry2;
  return coordinates.flatMap((line) => convertLineString2(line, false)).map(({ face: face3, line, offset, vecBBox }) => ({
    face: face3,
    geometry: { type: "LineString", is3D, coordinates: line, bbox: bbox2, offset, vecBBox }
  }));
}
function convertGeometryPolygon2(geometry2) {
  const { type, is3D, coordinates, bbox: bbox2 } = geometry2;
  const res = [];
  const outerRing = convertLineString2(coordinates[0], true);
  const innerRings = coordinates.slice(1).flatMap((line) => convertLineString2(line, true));
  for (const { face: face3, line, offset, vecBBox: polyBBox } of outerRing) {
    const polygon = [line];
    const polygonOffsets = [offset];
    for (const { face: innerFace, line: innerLine, offset: innerOffset, vecBBox } of innerRings) {
      if (innerFace === face3) {
        polygon.push(innerLine);
        polygonOffsets.push(innerOffset);
        mergeBBoxes2(polyBBox, vecBBox);
      }
    }
    res.push({
      face: face3,
      geometry: {
        type,
        coordinates: polygon,
        is3D,
        bbox: bbox2,
        offset: polygonOffsets,
        vecBBox: polyBBox
      }
    });
  }
  return res;
}
function convertGeometryMultiPolygon2(geometry2) {
  const { is3D, coordinates, bbox: bbox2, offset } = geometry2;
  return coordinates.flatMap((polygon, i) => convertGeometryPolygon2({
    type: "Polygon",
    is3D,
    coordinates: polygon,
    bbox: bbox2,
    offset: offset?.[i]
  }));
}
function convertLineString2(line, isPolygon) {
  const res = [];
  const newGeometry = [];
  for (const { x: lon, y: lat, z, m } of line) {
    const [face3, s, t] = toST2(fromLonLat2(lon, lat));
    newGeometry.push({ face: face3, s, t, z, m });
  }
  const faces = new Set;
  newGeometry.forEach(({ face: face3 }) => faces.add(face3));
  for (const face3 of faces) {
    const line2 = [];
    for (const stPoint of newGeometry)
      line2.push(stPointToFace2(face3, stPoint));
    const clippedLines = clipLine2(line2, [0, 0, 1, 1], isPolygon);
    for (const { line: line3, offset, vecBBox } of clippedLines)
      res.push({ face: face3, line: line3, offset, vecBBox });
  }
  return res;
}
function toUnitScale2(feature, tolerance, maxzoom) {
  const { geometry: geometry2 } = feature;
  const { type, coordinates } = geometry2;
  if (type === "Point")
    projectPoint2(coordinates, geometry2);
  else if (type === "MultiPoint")
    coordinates.map((p) => projectPoint2(p, geometry2));
  else if (type === "LineString")
    coordinates.map((p) => projectPoint2(p, geometry2));
  else if (type === "MultiLineString")
    coordinates.map((l) => l.map((p) => projectPoint2(p, geometry2)));
  else if (type === "Polygon")
    coordinates.map((l) => l.map((p) => projectPoint2(p, geometry2)));
  else if (type === "MultiPolygon")
    coordinates.map((p) => p.map((l) => l.map((p2) => projectPoint2(p2, geometry2))));
  else {
    throw new Error("Either the conversion is not yet supported or Invalid S2Geometry type.");
  }
  if (tolerance !== undefined)
    buildSqDists2(geometry2, tolerance, maxzoom);
}
function projectPoint2(input, geo) {
  const { x, y } = input;
  const sin = Math.sin(y * Math.PI / 180);
  const y2 = 0.5 - 0.25 * Math.log((1 + sin) / (1 - sin)) / Math.PI;
  input.x = x / 360 + 0.5;
  input.y = y2 < 0 ? 0 : y2 > 1 ? 1 : y2;
  geo.vecBBox = extendBBox2(geo.vecBBox, input);
}
function stPointToFace2(targetFace, stPoint) {
  const { face: curFace, s, t, z, m } = stPoint;
  if (targetFace === curFace)
    return { x: s, y: t, z, m };
  const [rot, x, y] = FACE_RULE_SET2[targetFace][curFace];
  const [newS, newT] = rotate2(rot, s, t);
  return { x: newS + x, y: newT + y, z, m };
}
function rotate2(rot, s, t) {
  if (rot === 90)
    return [t, 1 - s];
  else if (rot === -90)
    return [1 - t, s];
  else
    return [s, t];
}
var FACE_RULE_SET2 = [
  [
    [0, 0, 0],
    [0, 1, 0],
    [90, 0, 1],
    [-90, 2, 0],
    [-90, -1, 0],
    [0, 0, -1]
  ],
  [
    [0, -1, 0],
    [0, 0, 0],
    [0, 0, 1],
    [-90, 1, 0],
    [-90, 2, 0],
    [90, 0, -1]
  ],
  [
    [-90, -1, 0],
    [0, 0, -1],
    [0, 0, 0],
    [0, 1, 0],
    [90, 0, 1],
    [-90, 2, 0]
  ],
  [
    [-90, 2, 0],
    [90, 0, -1],
    [0, -1, 0],
    [0, 0, 0],
    [0, 0, 1],
    [-90, 1, 0]
  ],
  [
    [90, 0, 1],
    [-90, 2, 0],
    [-90, -1, 0],
    [0, 0, -1],
    [0, 0, 0],
    [0, 1, 0]
  ],
  [
    [0, 0, 1],
    [-90, 1, 0],
    [-90, 2, 0],
    [90, 0, -1],
    [0, -1, 0],
    [0, 0, 0]
  ]
];
// node_modules/.deno/s2json-spec@1.5.5/node_modules/s2json-spec/dist/simplify.js
function buildSqDists2(geometry2, tolerance, maxzoom = 16) {
  const tol = Math.pow(tolerance / ((1 << maxzoom) * 4096), 2);
  const { type, coordinates: coords } = geometry2;
  if (type === "LineString")
    buildSqDist2(coords, 0, coords.length - 1, tol);
  else if (type === "MultiLineString")
    coords.forEach((line) => buildSqDist2(line, 0, line.length - 1, tol));
  else if (type === "Polygon")
    coords.forEach((line) => buildSqDist2(line, 0, line.length - 1, tol));
  else if (type === "MultiPolygon")
    coords.forEach((polygon) => polygon.forEach((line) => buildSqDist2(line, 0, line.length - 1, tol)));
}
function buildSqDist2(coords, first, last, sqTolerance) {
  coords[first].t = 1;
  _buildSqDist2(coords, first, last, sqTolerance);
  coords[last].t = 1;
}
function _buildSqDist2(coords, first, last, sqTolerance) {
  let maxSqDist = sqTolerance;
  const mid = last - first >> 1;
  let minPosToMid = last - first;
  let index;
  const { x: as, y: at } = coords[first];
  const { x: bs, y: bt } = coords[last];
  for (let i = first;i < last; i++) {
    const { x, y } = coords[i];
    const d = getSqSegDist2(x, y, as, at, bs, bt);
    if (d > maxSqDist) {
      index = i;
      maxSqDist = d;
    } else if (d === maxSqDist) {
      const posToMid = Math.abs(i - mid);
      if (posToMid < minPosToMid) {
        index = i;
        minPosToMid = posToMid;
      }
    }
  }
  if (index !== undefined && maxSqDist > sqTolerance) {
    if (index - first > 1)
      _buildSqDist2(coords, first, index, sqTolerance);
    coords[index].t = maxSqDist;
    if (last - index > 1)
      _buildSqDist2(coords, index, last, sqTolerance);
  }
}
function getSqSegDist2(ps, pt, s, t, bs, bt) {
  let ds = bs - s;
  let dt = bt - t;
  if (ds !== 0 || dt !== 0) {
    const m = ((ps - s) * ds + (pt - t) * dt) / (ds * ds + dt * dt);
    if (m > 1) {
      s = bs;
      t = bt;
    } else if (m > 0) {
      s += ds * m;
      t += dt * m;
    }
  }
  ds = ps - s;
  dt = pt - t;
  return ds * ds + dt * dt;
}
function simplify3(geometry2, tolerance, zoom, maxzoom = 16) {
  const zoomTol = zoom >= maxzoom ? 0 : tolerance / ((1 << zoom) * 4096);
  const { type, coordinates: coords } = geometry2;
  if (type === "LineString")
    geometry2.coordinates = simplifyLine2(coords, zoomTol, false, false);
  else if (type === "MultiLineString")
    geometry2.coordinates = coords.map((line) => simplifyLine2(line, zoomTol, false, false));
  else if (type === "Polygon")
    geometry2.coordinates = coords.map((line, i) => simplifyLine2(line, zoomTol, true, i === 0));
  else if (type === "MultiPolygon")
    geometry2.coordinates = coords.map((polygon) => polygon.map((line, i) => simplifyLine2(line, zoomTol, true, i === 0)));
}
function simplifyLine2(line, tolerance, isPolygon, isOuter) {
  const sqTolerance = tolerance * tolerance;
  const size = line.length;
  if (tolerance > 0 && size < (isPolygon ? sqTolerance : tolerance))
    return line;
  const ring = [];
  for (const point of line) {
    if (tolerance === 0 || (point.t ?? 0) > sqTolerance)
      ring.push({ ...point });
  }
  if (isPolygon)
    rewind2(ring, isOuter);
  return ring;
}
function rewind2(ring, clockwise) {
  let area = 0;
  for (let i = 0, len = ring.length, j = len - 2;i < len; j = i, i += 2) {
    area += (ring[i].x - ring[j].x) * (ring[i].y + ring[j].y);
  }
  if (area > 0 === clockwise) {
    for (let i = 0, len = ring.length;i < len / 2; i += 2) {
      swapPoints2(ring, i, len - i - 1);
    }
  }
}
function swapPoints2(ring, i, j) {
  const tmp = ring[i];
  ring[i] = ring[j];
  ring[j] = tmp;
}
// node_modules/.deno/s2json-spec@1.5.5/node_modules/s2json-spec/dist/convert.js
function convert6(projection, data, tolerance, maxzoom, buildBBox) {
  const res = [];
  if (data.type === "Feature") {
    res.push(...convertFeature2(projection, data, tolerance, maxzoom, buildBBox));
  } else if (data.type === "VectorFeature") {
    res.push(...convertVectorFeature2(projection, data, tolerance, maxzoom));
  } else if (data.type === "FeatureCollection") {
    for (const feature of data.features) {
      if (feature.type === "Feature")
        res.push(...convertFeature2(projection, feature, tolerance, maxzoom, buildBBox));
      else
        res.push(...convertVectorFeature2(projection, feature, tolerance, maxzoom));
    }
  } else if (data.type === "S2Feature") {
    res.push(convertS2Feature2(projection, data, tolerance, maxzoom));
  } else if (data.type === "S2FeatureCollection") {
    for (const feature of data.features) {
      res.push(convertS2Feature2(projection, feature, tolerance, maxzoom));
    }
  }
  return res;
}
function convertFeature2(projection, data, tolerance, maxzoom, buildBBox) {
  if (projection === "WM") {
    const vf = toVector2(data, buildBBox);
    toUnitScale2(vf, tolerance, maxzoom);
    return [vf];
  } else {
    return toS22(data, tolerance, maxzoom, buildBBox);
  }
}
function convertVectorFeature2(projection, data, tolerance, maxzoom) {
  if (projection === "WM") {
    toUnitScale2(data, tolerance, maxzoom);
    return [data];
  } else {
    return toS22(data, tolerance, maxzoom);
  }
}
function convertS2Feature2(projection, data, tolerance, maxzoom) {
  if (projection === "WM") {
    const vf = toWM2(data);
    toUnitScale2(vf, tolerance, maxzoom);
    return vf;
  } else {
    return data;
  }
}

// node_modules/.deno/s2json-spec@1.5.5/node_modules/s2json-spec/dist/tile.js
function _transform2(geometry2, zoom, ti, tj) {
  const { type, coordinates } = geometry2;
  zoom = 1 << zoom;
  if (type === "Point")
    transformPoint2(coordinates, zoom, ti, tj);
  else if (type === "MultiPoint" || type === "LineString")
    coordinates.forEach((p) => transformPoint2(p, zoom, ti, tj));
  else if (type === "MultiLineString" || type === "Polygon")
    coordinates.forEach((l) => l.forEach((p) => transformPoint2(p, zoom, ti, tj)));
  else if (type === "MultiPolygon")
    coordinates.forEach((p) => p.forEach((l) => l.forEach((p2) => transformPoint2(p2, zoom, ti, tj))));
}
function transformPoint2(vp, zoom, ti, tj) {
  vp.x = vp.x * zoom - ti;
  vp.y = vp.y * zoom - tj;
}

class Tile2 {
  id;
  layers;
  transformed;
  constructor(id2, layers = {}, transformed = false) {
    this.id = id2;
    this.layers = layers;
    this.transformed = transformed;
  }
  isEmpty() {
    for (const layer of Object.values(this.layers)) {
      if (layer.features.length > 0)
        return false;
    }
    return true;
  }
  addFeature(feature, layer) {
    const { metadata = {} } = feature;
    const layerName = metadata.layer ?? layer ?? "default";
    if (!this.layers[layerName]) {
      this.layers[layerName] = new Layer2(layerName, []);
    }
    this.layers[layerName].features.push(feature);
  }
  transform(tolerance, maxzoom) {
    const { transformed, id: id2, layers } = this;
    if (transformed)
      return;
    const [, zoom, i, j] = toFaceIJ2(id2);
    for (const layer of Object.values(layers)) {
      for (const feature of layer.features) {
        simplify3(feature.geometry, tolerance, zoom, maxzoom);
        _transform2(feature.geometry, zoom, i, j);
      }
    }
    this.transformed = true;
  }
}

class Layer2 {
  name;
  features;
  constructor(name, features = []) {
    this.name = name;
    this.features = features;
  }
}

class TileStore2 {
  minzoom = 0;
  maxzoom = 18;
  faces = new Set;
  indexMaxzoom = 4;
  tolerance = 3;
  buffer = 0.0625;
  tiles = new Map;
  projection;
  constructor(data, options) {
    this.minzoom = options?.minzoom ?? 0;
    this.maxzoom = options?.maxzoom ?? 20;
    this.indexMaxzoom = options?.indexMaxzoom ?? 4;
    this.tolerance = options?.tolerance ?? 3;
    this.buffer = options?.buffer ?? 64;
    if (options?.projection !== undefined)
      this.projection = options.projection;
    else if (data.type === "Feature" || data.type === "FeatureCollection")
      this.projection = "WM";
    else
      this.projection = "S2";
    if (this.maxzoom < 0 || this.maxzoom > 20)
      throw new Error("maxzoom should be in the 0-20 range");
    const features = convert6(this.projection, data);
    for (const feature of features)
      this.addFeature(feature);
    for (let face3 = 0;face3 < 6; face3++) {
      const id2 = fromFace2(face3);
      this.splitTile(id2);
    }
  }
  addFeature(feature) {
    const { faces, tiles } = this;
    const face3 = feature.face ?? 0;
    const id2 = fromFace2(face3);
    let tile2 = tiles.get(id2);
    if (tile2 === undefined) {
      faces.add(face3);
      tile2 = new Tile2(id2);
      tiles.set(id2, tile2);
    }
    tile2?.addFeature(feature);
  }
  splitTile(startID, endID, endZoom = this.maxzoom) {
    const { buffer, tiles, tolerance, maxzoom, indexMaxzoom } = this;
    const stack = [startID];
    while (stack.length > 0) {
      const stackID = stack.pop();
      if (stackID === undefined)
        break;
      const tile2 = tiles.get(stackID);
      if (tile2 === undefined || tile2.isEmpty() || tile2.transformed)
        continue;
      const tileZoom = level2(tile2.id);
      if (tileZoom >= maxzoom || endID === undefined && tileZoom >= indexMaxzoom || endID !== undefined && (tileZoom > endZoom || !contains2(tile2.id, endID)))
        continue;
      const [blID, brID, tlID, trID] = splitTile2(tile2, buffer);
      tiles.set(blID.id, blID);
      tiles.set(brID.id, brID);
      tiles.set(tlID.id, tlID);
      tiles.set(trID.id, trID);
      tile2.transform(tolerance, maxzoom);
      stack.push(blID.id, brID.id, tlID.id, trID.id);
    }
  }
  getTile(id2) {
    const { tiles, faces } = this;
    const zoom = level2(id2);
    const face3 = face2(id2);
    if (zoom < 0 || zoom > 20 || !faces.has(face3))
      return;
    let pID = id2;
    while (!tiles.has(pID) && !isFace2(pID)) {
      pID = parent2(pID);
    }
    this.splitTile(pID, id2, zoom);
    return tiles.get(id2);
  }
}
// dist/geometry/s1/chordAngle.js
function fromS2Points(a, b) {
  return Math.min(K_MAX_LENGTH_2, norm2(sub(a, b)));
}
var K_MAX_LENGTH_2 = 4;

// dist/geometry/s2/metrics.js
class Metric {
  dim;
  deriv;
  constructor(dim, deriv) {
    this.dim = dim;
    this.deriv = deriv;
  }
  getValue(level3) {
    return this.deriv * Math.pow(2, -this.dim * level3);
  }
  getClosestLevel(value) {
    return this.getLevelForMaxValue((this.dim === 1 ? Math.sqrt(2) : 2) * value);
  }
  getLevelForMaxValue(value) {
    if (value <= 0)
      return K_MAX_LEVEL;
    const { min, max, log2, floor } = Math;
    let level3 = floor(log2(value / this.deriv));
    level3 = max(0, min(K_MAX_LEVEL, -(level3 >> this.dim - 1)));
    return level3;
  }
  getLevelForMinValue(value) {
    if (value <= 0)
      return K_MAX_LEVEL;
    const { min, max, log2, floor } = Math;
    let level3 = floor(log2(this.deriv / value));
    level3 = max(0, min(K_MAX_LEVEL, level3 >> this.dim - 1));
    return level3;
  }
}

class LengthMetric extends Metric {
  constructor(deriv) {
    super(1, deriv);
  }
}

class AreaMetric extends Metric {
  constructor(deriv) {
    super(2, deriv);
  }
}
var K_MIN_ANGLE_SPAN = new LengthMetric(4 / 3);
var K_MAX_ANGLE_SPAN = new LengthMetric(1.704897179199218);
var K_AVG_ANGLE_SPAN = new LengthMetric(Math.PI / 2);
var K_MIN_WIDTH = new LengthMetric(2 * Math.sqrt(2) / 3);
var K_MAX_WIDTH = new LengthMetric(K_MAX_ANGLE_SPAN.deriv);
var K_AVG_WIDTH = new LengthMetric(1.4345236728860993);
var K_MIN_EDGE = new LengthMetric(2 * Math.sqrt(2) / 3);
var K_MAX_EDGE = new LengthMetric(K_MAX_ANGLE_SPAN.deriv);
var K_AVG_EDGE = new LengthMetric(1.459213746386106);
var K_MIN_DIAG = new LengthMetric(8 * Math.sqrt(2) / 9);
var K_MAX_DIAG = new LengthMetric(2.438654594434021);
var K_AVG_DIAG = new LengthMetric(2.060422738998471);
var K_MIN_AREA = new AreaMetric(8 * Math.sqrt(2) / 9);
var K_MAX_AREA = new AreaMetric(2.6357992569631614);
var K_AVG_AREA = new AreaMetric(4 * Math.PI / 6);
var K_MAX_EDGE_ASPECT = Math.sqrt(1.442615274452682);
var K_MAX_DIAG_ASPECT = Math.sqrt(3);

// dist/geometry/s2/cap.js
function isEmpty(cap) {
  return cap.radius < 0;
}
function isFull(cap) {
  return cap.radius === 4;
}
function fromS1ChordAngle(center, radius, data) {
  return { center, radius, data };
}
function containsS2Point2(cap, p) {
  return fromS2Points(cap.center, p) <= cap.radius;
}
function containsS2CellVertexCount(cap, cell) {
  let count = 0;
  const vertices = getVertices(cell);
  for (const vertex of vertices) {
    if (containsS2Point2(cap, vertex))
      count++;
  }
  return count;
}
function getIntersectingCells(cap) {
  const res = [];
  if (isEmpty(cap))
    return res;
  const queue = [0, 1, 2, 3, 4, 5].map(fromFace);
  if (isFull(cap))
    return queue;
  const maxDepth = K_AVG_EDGE.getLevelForMaxValue(cap.radius);
  while (true) {
    const cell = queue.pop();
    if (cell === undefined)
      break;
    const vertexCount = containsS2CellVertexCount(cap, cell);
    if (vertexCount === 4) {
      res.push(cell);
    } else if (vertexCount === 0) {
      continue;
    } else {
      if (level(cell) >= maxDepth)
        res.push(cell);
      else
        queue.push(...children(cell));
    }
  }
  return res;
}

// dist/dataStructures/pointIndex.js
class PointIndex {
  #store;
  #unsorted = false;
  constructor(store = Vector) {
    this.#store = new store;
  }
  insert(point, data) {
    this.#store.push({ cell: fromS2Point2(point), point, data });
    this.#unsorted = true;
  }
  insertLonLat(lon, lat, data) {
    this.insert(fromLonLat(lon, lat), data);
  }
  insertFaceST(face3, s, t, data) {
    this.insert(fromST(face3, s, t), data);
  }
  async* [Symbol.asyncIterator]() {
    await this.sort();
    for (const value of this.#store)
      yield value;
  }
  insertPoints(points) {
    for (const point of points)
      this.#store.push(point);
    this.#unsorted = true;
  }
  async sort() {
    if (!this.#unsorted)
      return;
    await this.#store.sort();
    this.#unsorted = false;
  }
  async lowerBound(id3) {
    const cellID = toCell(id3);
    await this.sort();
    let lo = 0;
    let hi = this.#store.length;
    let mid;
    while (lo < hi) {
      mid = Math.floor(lo + (hi - lo) / 2);
      const { cell: midCell } = await this.#store.get(mid);
      if (compare(midCell, cellID) === -1) {
        lo = mid + 1;
      } else {
        hi = mid;
      }
    }
    return lo;
  }
  async searchRange(low, high) {
    await this.sort();
    const res = [];
    let lo = await this.lowerBound(low);
    const hiID = toCell(high);
    while (true) {
      const currLo = await this.#store.get(lo);
      if (lo < this.#store.length && compare(currLo.cell, hiID) <= 0)
        break;
      res.push(currLo);
      lo++;
    }
    return res;
  }
  async searchRadius(target, radius) {
    await this.sort();
    const res = [];
    if (radius < 0)
      return res;
    const cap = fromS1ChordAngle(target, radius, undefined);
    for (const cell of getIntersectingCells(cap)) {
      const [min, max] = range(cell);
      for (const point of await this.searchRange(min, max)) {
        if (fromS2Points(target, point.point) < radius)
          res.push(point);
      }
    }
    return res;
  }
}

// benchmarks/pointIndex.ts
function getRandomInt(a, b) {
  if (a > b) {
    throw new Error("The first argument must be less than or equal to the second argument.");
  }
  const min = Math.ceil(a);
  const max = Math.floor(b);
  return Math.floor(Math.random() * (max - min + 1)) + min;
}
var TOTAL_SIZE = 1e6;
var lls = [];
for (let i = 0;i < TOTAL_SIZE; i++) {
  const lon = getRandomInt(-180, 180);
  const lat = getRandomInt(-90, 90);
  lls.push({ lon, lat, data: { a: i } });
}
console.info("\n\n");
var index = new PointIndex;
for (let i = 0;i < TOTAL_SIZE; i++) {
  index.insertLonLat(lls[i].lon, lls[i].lat, { a: i });
}
await index.sort();
var _withinSearch2 = index.searchRadius(fromLonLat(lls[0].lon, lls[0].lat), 1);
