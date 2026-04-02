import {
  convert,
  idChildren,
  idFromIJ,
  idFromST,
  idGetVertices,
  idNeighbors,
  idParent,
  idToFaceIJ,
  llToPX,
  llToTile,
  pointToLonLat,
  xyzToBBOX,
} from './index.js';

import type {
  BBox,
  Face,
  JSONCollection,
  MValue,
  Properties,
  S2CellId,
  VectorFeatures,
  VectorGeometry,
  VectorLineString,
  VectorMultiLineString,
  VectorMultiPolygon,
  VectorPoint,
  VectorPolygon,
} from './index.js';

/** Tile metadata */
export type TileID = WMTileID | S2TileID;

/** WM Tile's metadata */
export interface WMTileID {
  zoom: number;
  x: number;
  y: number;
}

/** S2 Tile's metadata */
export interface S2TileID {
  face: Face;
  zoom: number;
  x: number;
  y: number;
}

/**
 * Convert a tileID to an S2CellId
 *
 * @param tileID - the tileID to convert
 * @returns the S2CellId
 */
export function tileToID(tileID: TileID): S2CellId {
  return idFromIJ('face' in tileID ? tileID.face : 0, tileID.x, tileID.y, tileID.zoom);
}

/**
 * Convert an S2CellId to a tileID
 *
 * @param id - the S2CellId
 * @param isWM - whether the tileID is an WM tile or an S2 tile
 * @returns the appropriate tileID
 */
export function tileFromID(id: S2CellId, isWM: boolean): TileID {
  const [face, zoom, i, j] = idToFaceIJ(id);
  return isWM ? { zoom, x: i, y: j } : { face, zoom, x: i, y: j };
}

/**
 * Convert a tileID to a BBox
 *
 * @param tileID - the tileID
 * @param tmsStyle - whether the tile is TMS style. Only applicable to WM tiles. Inverts the Y axis
 * @returns the BBox of the tile
 */
export function tileToBBox(tileID: TileID, tmsStyle: boolean = false): BBox {
  if ('face' in tileID) {
    return s2Bounds(tileID);
  } else {
    return xyzToBBOX(tileID.x, tileID.y, tileID.zoom, tmsStyle, 'WGS84');
  }
}

/**
 * Convert a tileID to a center lon-lat
 * @param tileID - the tileID
 * @param tmsStyle - whether the tile is TMS style. Only applicable to WM tiles. Inverts the Y axis
 * @returns the center lon-lat of the tile
 */
export function tileToCenterLonLat(tileID: TileID, tmsStyle?: boolean): VectorPoint {
  const bbox = tileToBBox(tileID, tmsStyle);

  return { x: (bbox[0] + bbox[2]) / 2, y: (bbox[1] + bbox[3]) / 2 };
}

/**
 * Get the children tiles of a tile
 *
 * @param tileID - the tile
 * @returns the children
 */
export function tileChildren(tileID: TileID): [TileID, TileID, TileID, TileID] {
  if ('face' in tileID) {
    const id = idFromIJ(tileID.face, tileID.x, tileID.y, tileID.zoom);
    return idChildren(id).map((id) => tileFromID(id, false)) as [TileID, TileID, TileID, TileID];
  } else {
    return [
      { zoom: tileID.zoom + 1, x: tileID.x * 2, y: tileID.y * 2 },
      { zoom: tileID.zoom + 1, x: tileID.x * 2 + 1, y: tileID.y * 2 },
      { zoom: tileID.zoom + 1, x: tileID.x * 2, y: tileID.y * 2 + 1 },
      { zoom: tileID.zoom + 1, x: tileID.x * 2 + 1, y: tileID.y * 2 + 1 },
    ];
  }
}

/**
 * Get the parent tile
 *
 * @param tileID - the tile
 * @returns the parent tile
 */
export function tileParent(tileID: TileID): TileID {
  if ('face' in tileID) {
    const id = idFromIJ(tileID.face, tileID.x, tileID.y, tileID.zoom);
    const parentID = idParent(id);
    return tileFromID(parentID, false);
  } else {
    return { zoom: tileID.zoom - 1, x: tileID.x >>> 1, y: tileID.y >>> 1 };
  }
}

/**
 * Get the neighbors of a tile
 *
 * @param tileID - the tile
 * @returns the neighbors
 */
export function tileNeighbors(tileID: TileID): TileID[] {
  if ('face' in tileID) {
    const id = idFromIJ(tileID.face, tileID.x, tileID.y, tileID.zoom);
    return idNeighbors(id).map((id) => tileFromID(id, false));
  } else {
    if (tileID.zoom === 0) return [];
    // allow wrapping horizontally
    const { zoom, x, y } = tileID;
    const maxGrid = (1 << zoom) - 1;
    const neighbors = new Map<string, TileID>();
    // wrap X tiles
    if (x + 1 <= maxGrid) neighbors.set(`${zoom}_${x + 1}_${y}`, { zoom, x: x + 1, y });
    else neighbors.set(`${zoom}_0_${y}`, { zoom, x: 0, y });
    if (x - 1 >= 0) neighbors.set(`${zoom}_${x - 1}_${y}`, { zoom, x: x - 1, y });
    else neighbors.set(`${zoom}_${maxGrid}_${y}`, { zoom, x: maxGrid, y });
    // vertical Y has a max and min
    if (y + 1 <= maxGrid) neighbors.set(`${zoom}_${x}_${y + 1}`, { zoom, x, y: y + 1 });
    if (y - 1 >= 0) neighbors.set(`${zoom}_${x}_${y - 1}`, { zoom, x, y: y - 1 });
    return [...neighbors.values()];
  }
}

/**
 * Get the tile from a Lon-Lat WGS84 coordinate
 *
 * @param point - the point
 * @param zoom - the zoom level to find the tile in
 * @returns the tile at the zoom that contains the point
 */
export function wmTileFromPoint<D extends MValue = Properties>(
  point: VectorPoint<D>,
  zoom: number,
): WMTileID {
  const { x, y } = llToTile(point, zoom);
  return { zoom, x, y };
}

/**
 * Get the S2 tile from a point that is in S2's S-T coordinate space
 *
 * @param point - the point
 * @param face - the face of the S2 Cube the geometry is in
 * @param zoom - the zoom level to find the tile in
 * @returns the tile at the zoom that contains the point
 */
export function s2TileFromPoint<D extends MValue = Properties>(
  point: VectorPoint<D>,
  face: Face,
  zoom: number,
): S2TileID {
  const id = idFromST(face, point.x, point.y, zoom);
  const [_face, _zoom, x, y] = idToFaceIJ(id);
  return { face, zoom, x, y };
}

/**
 * Get the tiles from Lon-Lat WGS84 coordinates
 *
 * @param point - the point
 * @param zoom - the zoom level to find the tile in
 * @returns the tile at the zoom that contains the point
 */
export function wmTileFromMultiPoint<D extends MValue = Properties>(
  point: VectorPoint<D>[],
  zoom: number,
): WMTileID[] {
  const res = [];
  for (const p of point) res.push(wmTileFromPoint(p, zoom));
  return res;
}

/**
 * Get the S2 tiles from a collection of points that are in S2's S-T coordinate space
 *
 * @param points - the source points to generate tiles from
 * @param face - the face of the S2 Cube the geometry is in
 * @param zoom - the zoom level to find the tile in
 * @returns the tile at the zoom that contains the point
 */
export function s2TileFromMultiPoint<D extends MValue = Properties>(
  points: VectorPoint<D>[],
  face: Face,
  zoom: number,
): S2TileID[] {
  const res = [];
  for (const point of points) res.push(s2TileFromPoint(point, face, zoom));
  return res;
}

/**
 * Get the tiles that cover a linestring from Lon-Lat WGS84 coordinates
 *
 * @param points - the linestring
 * @param zoom - the zoom level
 * @returns the tiles that contain the linestring at the zoom
 */
export function wmTilesFromLineString<D extends MValue = Properties>(
  points: VectorLineString<D>,
  zoom: number,
): WMTileID[] {
  const tileMap = new Map<string, WMTileID>();
  lineCover(tileMap, points, zoom, []);
  return mergeTiles([...tileMap.values()]);
}

/**
 * Get the S2 tiles that cover a linestring from S2's S-T coordinate space
 *
 * @param points - the linestring
 * @param face - the face of the S2 Cube the geometry is in
 * @param zoom - the zoom level
 * @returns the tiles that contain the linestring at the zoom
 */
export function s2TilesFromLineString<D extends MValue = Properties>(
  points: VectorLineString<D>,
  face: Face,
  zoom: number,
): S2TileID[] {
  const tileMap = new Map<string, WMTileID>();
  lineCover(tileMap, points, zoom, [], s2ToPX);
  return mergeTiles(
    [...tileMap.values()].map(({ zoom, x, y }) => ({ face, zoom, x, y })),
  ) as S2TileID[];
}

/**
 * Get the tiles that cover a multilinestring from Lon-Lat WGS84 coordinates
 *
 * @param lines - the multilinestring
 * @param zoom - the zoom level
 * @returns the tiles that contain the multilinestring at the zoom
 */
export function wmTilesFromMultiLineString<D extends MValue = Properties>(
  lines: VectorMultiLineString<D>,
  zoom: number,
): WMTileID[] {
  const tileMap = new Map<string, WMTileID>();
  for (const line of lines) lineCover(tileMap, line, zoom, []);
  return mergeTiles([...tileMap.values()]);
}

/**
 * Get the S2 tiles that cover a multilinestring from S2's S-T coordinate space
 *
 * @param lines - the multilinestring
 * @param face - the face of the S2 Cube the geometry is in
 * @param zoom - the zoom level
 * @returns the tiles that contain the multilinestring at the zoom
 */
export function s2TilesFromMultiLineString<D extends MValue = Properties>(
  lines: VectorMultiLineString<D>,
  face: Face,
  zoom: number,
): S2TileID[] {
  const tileMap = new Map<string, WMTileID>();
  for (const line of lines) lineCover(tileMap, line, zoom, [], s2ToPX);
  return mergeTiles(
    [...tileMap.values()].map(({ zoom, x, y }) => ({ face, zoom, x, y })),
  ) as S2TileID[];
}

/**
 * Get the tiles that cover a polygon from Lon-Lat WGS84 coordinates
 *
 * @param poly - the polygon
 * @param zoom - the zoom level
 * @returns the tiles that cover the polygon
 */
export function wmTilesFromPolygon<D extends MValue = Properties>(
  poly: VectorPolygon<D>,
  zoom: number,
): WMTileID[] {
  const tileMap = new Map<string, WMTileID>();
  polyCover(tileMap, poly, zoom);
  return mergeTiles([...tileMap.values()]);
}

/**
 * Get the S2 tiles that cover a polygon from S2's S-T coordinate space
 *
 * @param poly - the polygon
 * @param face - the face of the S2 Cube the geometry is in
 * @param zoom - the zoom level
 * @returns the tiles that cover the polygon
 */
export function s2TilesFromPolygon<D extends MValue = Properties>(
  poly: VectorPolygon<D>,
  face: Face,
  zoom: number,
): S2TileID[] {
  const tileMap = new Map<string, WMTileID>();

  polyCover(tileMap, poly, zoom, s2ToPX);

  return mergeTiles(
    [...tileMap.values()].map(({ zoom, x, y }) => ({ face, zoom, x, y })),
  ) as S2TileID[];
}

/**
 * Get the tiles that cover a multipolygon from Lon-Lat WGS84 coordinates
 *
 * @param polys - the multipolygon
 * @param zoom - the zoom level
 * @returns the tiles that cover the multipolygon
 */
export function wmTilesFromMultiPolygon<D extends MValue = Properties>(
  polys: VectorMultiPolygon<D>,
  zoom: number,
): WMTileID[] {
  const tileMap = new Map<string, WMTileID>();

  for (const poly of polys) polyCover(tileMap, poly, zoom);

  return mergeTiles([...tileMap.values()]);
}

/**
 * Get the S2 tiles that cover a multipolygon from S2's S-T coordinate space
 *
 * @param polys - the multipolygon
 * @param face - the face of the S2 Cube the geometry is in
 * @param zoom - the zoom level
 * @returns the tiles that cover the multipolygon
 */
export function s2TilesFromMultiPolygon<D extends MValue = Properties>(
  polys: VectorMultiPolygon<D>,
  face: Face,
  zoom: number,
): S2TileID[] {
  const tileMap = new Map<string, WMTileID>();

  for (const poly of polys) polyCover(tileMap, poly, zoom);

  return mergeTiles(
    [...tileMap.values()].map(({ zoom, x, y }) => ({ face, zoom, x, y })),
  ) as S2TileID[];
}

/**
 * Convert a vector geometry to the WM tiles that cover it
 *
 * @param geom - the vector geometry
 * @param zoom - the zoom level
 * @returns the tiles that cover the vector geometry
 */
export function wmTileFromVectorGeometry<D extends MValue = Properties>(
  geom: VectorGeometry<D>,
  zoom: number,
): WMTileID[] {
  if (geom.type === 'Point') return [wmTileFromPoint(geom.coordinates, zoom)];
  else if (geom.type === 'MultiPoint') return wmTileFromMultiPoint(geom.coordinates, zoom);
  else if (geom.type === 'LineString') return wmTilesFromLineString(geom.coordinates, zoom);
  else if (geom.type === 'MultiLineString')
    return wmTilesFromMultiLineString(geom.coordinates, zoom);
  else if (geom.type === 'Polygon') return wmTilesFromPolygon(geom.coordinates, zoom);
  else if (geom.type === 'MultiPolygon') return wmTilesFromMultiPolygon(geom.coordinates, zoom);
  // @ts-expect-error - ignore 'never' for error report to benefit the user
  else throw new Error(`Unsupported geometry type: ${geom.type}`);
}

/**
 * Convert an S2 vector geometry to the S2 tiles that cover it
 *
 * @param geom - the vector geometry
 * @param face - the face of the S2 Cube the geometry is in
 * @param zoom - the zoom level
 * @returns the tiles that cover the vector geometry
 */
export function s2TileFromVectorGeometry<D extends MValue = Properties>(
  geom: VectorGeometry<D>,
  face: Face,
  zoom: number,
): WMTileID[] {
  if (geom.type === 'Point') return [s2TileFromPoint(geom.coordinates, face, zoom)];
  else if (geom.type === 'MultiPoint') return s2TileFromMultiPoint(geom.coordinates, face, zoom);
  else if (geom.type === 'LineString') return s2TilesFromLineString(geom.coordinates, face, zoom);
  else if (geom.type === 'MultiLineString')
    return s2TilesFromMultiLineString(geom.coordinates, face, zoom);
  else if (geom.type === 'Polygon') return s2TilesFromPolygon(geom.coordinates, face, zoom);
  else if (geom.type === 'MultiPolygon')
    return s2TilesFromMultiPolygon(geom.coordinates, face, zoom);
  // @ts-expect-error - ignore 'never' for error report to benefit the user
  else throw new Error(`Unsupported geometry type: ${geom.type}`);
}

/**
 * Convert a vector feature to the WM tiles that cover it
 *
 * @param feature - the vector feature
 * @param zoom - the zoom level
 * @returns the tiles that cover the vector feature
 */
export function wmTileFromVectorFeature<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(feature: VectorFeatures<M, D, P>, zoom: number): WMTileID[] {
  return wmTileFromVectorGeometry(feature.geometry, zoom);
}

/**
 * Convert an S2 vector feature to the S2 tiles that cover it
 *
 * @param feature - the vector feature
 * @param zoom - the zoom level
 * @returns the tiles that cover the vector feature
 */
export function s2TileFromVectorFeature<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(feature: VectorFeatures<M, D, P>, zoom: number): WMTileID[] {
  return s2TileFromVectorGeometry(feature.geometry, feature.face ?? 6, zoom);
}

/**
 * Convert input GeoJSON or S2JSON data into a list of WM Tiles if covers.
 * If you are unsure about the input data or you have a FeatureCollection/S2FeatureCollection,
 * this is your best bet.
 *
 * @param json - the vector data
 * @param zoom - the zoom level
 * @returns the tiles that cover the vector feature
 */
export function wmTileFromJSON<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(json: JSONCollection<M, D, P>, zoom: number): WMTileID[] {
  const features = convert('WG', json);
  const list = new Map<string, WMTileID>();
  for (const feature of features) {
    const featureTiles = wmTileFromVectorFeature(feature, zoom);
    for (const tile of featureTiles) list.set(`${tile.zoom}_${tile.x}_${tile.y}`, tile);
  }
  return [...list.values()];
}

/**
 * Convert input GeoJSON or S2JSON data into a list of S2 Tiles if covers.
 * If you are unsure about the input data or you have a FeatureCollection/S2FeatureCollection,
 * this is your best bet.
 *
 * @param json - the vector data
 * @param zoom - the zoom level
 * @returns the tiles that cover the vector feature
 */
export function s2TileFromJSON<
  M = Record<string, unknown>,
  D extends MValue = Properties,
  P extends Properties = Properties,
>(json: JSONCollection<M, D, P>, zoom: number): WMTileID[] {
  const features = convert('S2', json);
  const list = new Map<string, WMTileID>();
  for (const feature of features) {
    const featureTiles = s2TileFromVectorFeature(feature, zoom);
    for (const tile of featureTiles) list.set(`${tile.zoom}_${tile.x}_${tile.y}`, tile);
  }
  return [...list.values()];
}

/**
 * Given a collection of tiles at various zoom levels, merge them into a single collection at a
 * higher zoom level when possible
 *
 * @param tiles - list of tiles
 * @returns a merged/simplified list of tiles where a full collection of children are simplified into their parent
 */
export function mergeTiles(tiles: TileID[]): TileID[] {
  if (tiles.length <= 1) return tiles;
  let maxzoom = 0;
  const tileSet = new Map<number, Set<string>>();

  // 1) Get the total zoom range of all tiles while we store all in a Set
  for (const tile of tiles) {
    maxzoom = Math.max(tile.zoom, maxzoom);
    const zoomTiles = tileSet.get(tile.zoom) ?? new Set();
    zoomTiles.add(`${'face' in tile ? tile.face : '6'}_${tile.zoom}_${tile.x}_${tile.y}`);
    tileSet.set(tile.zoom, zoomTiles);
  }

  // 2) Start at lowest zoom, iterate each -> get Parent -> Get children of parent -> if all children are in Set, remove them and add parent
  for (let zoom = maxzoom; zoom > 0; zoom--) {
    const currZoomTiles = tileSet.get(zoom);
    if (currZoomTiles === undefined || currZoomTiles.size === 0) continue;
    const parentZoomTiles = tileSet.get(zoom - 1) ?? new Set<string>();
    const hasParent = new Set<string>();
    const potentialParents = new Map<string, TileID>();

    // collect all possible parent candidates
    for (const tile of currZoomTiles) {
      const [face, zoom, x, y] = tile.split('_').map(Number);
      const parent = tileParent(face <= 5 ? { face: face as Face, zoom, x, y } : { zoom, x, y });
      const parentKey = `${'face' in parent ? parent.face : '6'}_${parent.zoom}_${parent.x}_${parent.y}`;
      potentialParents.set(parentKey, parent);
    }
    // iterate candidates, if all children are in currZoomTiles, add parent and remove children
    for (const parent of potentialParents.values()) {
      const children = tileChildren(parent);
      const childrenKeys = children.map(
        (c) => `${'face' in c ? c.face : '6'}_${c.zoom}_${c.x}_${c.y}`,
      );
      if (childrenKeys.every((c) => currZoomTiles.has(c))) {
        for (const child of childrenKeys) hasParent.add(child);
        parentZoomTiles.add(
          `${'face' in parent ? parent.face : '6'}_${parent.zoom}_${parent.x}_${parent.y}`,
        );
      }
    }
    // cleanup the tiles that were upgraded to a higher zoom
    const remaining = new Set([...currZoomTiles].filter((t) => !hasParent.has(t)));
    tileSet.set(zoom, remaining);
    // store the parent tiles even if empty
    tileSet.set(zoom - 1, parentZoomTiles);
  }

  // 3) restore back to array
  const res: TileID[] = [];
  for (const tiles of tileSet.values()) {
    for (const tile of tiles) {
      const [face, zoom, x, y] = tile.split('_').map(Number);
      res.push(face <= 5 ? { face: face as Face, zoom, x, y } : { zoom, x, y });
    }
  }

  return res;
}

function s2Bounds(tileID: S2TileID): BBox {
  const id = idFromIJ(tileID.face, tileID.x, tileID.y, tileID.zoom);
  const [a, b, c, d] = idGetVertices(id).map(pointToLonLat);

  return [
    Math.min(a.x, b.x, c.x, d.x),
    Math.min(a.y, b.y, c.y, d.y),
    Math.max(a.x, b.x, c.x, d.x),
    Math.max(a.y, b.y, c.y, d.y),
  ];
}

function s2ToPX<M extends MValue = Properties>(
  pt: VectorPoint<M>,
  zoom: number,
  _antiMeridian = false, // Kept for signature compatibility
  _tileSize = 512,
): VectorPoint {
  // Total map size at this zoom: tileSize * 2^zoom
  const mapSize = 512 * Math.pow(2, zoom);

  return { x: pt.x * mapSize, y: pt.y * mapSize };
}

function polyCover(
  tileMap: Map<string, WMTileID>,
  poly: VectorPolygon,
  zoom: number,
  coordToPX = llToPX,
): void {
  const intersections: VectorPoint[] = [];

  for (const line of poly) {
    const ring: VectorPoint[] = [];
    lineCover(tileMap, line, zoom, ring, coordToPX);

    for (let j = 0, len = ring.length, k = len - 1; j < len; k = j++) {
      const m = (j + 1) % len;
      const y = ring[j].y;
      // add interesction if it's not local extremum or duplicate
      if (
        (y > ring[k].y || y > ring[m].y) && // not local minimum
        (y < ring[k].y || y < ring[m].y) && // not local maximum
        y !== ring[m].y
      )
        intersections.push(ring[j]);
    }
  }

  intersections.sort((a, b) => (a.y - b.y !== 0 ? a.y - b.y : a.x - b.x));

  // even-odd fill
  for (let i = 0; i < intersections.length - 1; i += 2) {
    const y = intersections[i].y;
    for (let x = intersections[i].x + 1; x < intersections[i + 1].x; x++) {
      tileMap.set(`${zoom}_${x}_${y}`, { zoom, x, y });
    }
  }
}

// Modified Digital Differential Analyzer algorithm
function lineCover<D extends MValue = Properties>(
  tileMap: Map<string, WMTileID>,
  coords: VectorPoint<D>[],
  zoom: number,
  ring: VectorPoint[],
  coordToPX = llToPX,
): void {
  const { floor, abs } = Math;
  let prevX: number | undefined;
  let prevY: number | undefined;

  for (let i = 0; i < coords.length - 1; i++) {
    const a = coordToPX(coords[i], zoom, false, 512);
    const b = coordToPX(coords[i + 1], zoom, false, 512);
    let x = floor(a.x / 512);
    let y = floor(a.y / 512);
    const endX = floor(b.x / 512);
    const endY = floor(b.y / 512);
    const dx = b.x - a.x;
    const dy = b.y - a.y;
    if (dx === 0 && dy === 0) continue;
    const stepX = dx > 0 ? 1 : -1;
    const stepY = dy > 0 ? 1 : -1;
    let tMaxX = dx === 0 ? Infinity : abs(((stepX > 0 ? 1 : 0) * 512 + x * 512 - a.x) / dx);
    let tMaxY = dy === 0 ? Infinity : abs(((stepY > 0 ? 1 : 0) * 512 + y * 512 - a.y) / dy);
    const tDeltaX = dx === 0 ? Infinity : abs(512 / dx);
    const tDeltaY = dy === 0 ? Infinity : abs(512 / dy);

    // Initial tile check
    if (x !== prevX || y !== prevY) {
      tileMap.set(`${zoom}_${x}_${y}`, { zoom, x, y });
      // Record intersection only if the Y row changed
      if (y !== prevY) ring.push({ x, y });
      prevX = x;
      prevY = y;
    }
    // Main loop
    while (x !== endX || y !== endY) {
      if (tMaxX < tMaxY) {
        tMaxX += tDeltaX;
        x += stepX;
      } else {
        tMaxY += tDeltaY;
        y += stepY;
      }

      tileMap.set(`${zoom}_${x}_${y}`, { zoom, x, y });
      // If the step we just took changed the row, it's an intersection
      if (y !== prevY) ring.push({ x, y });

      prevX = x;
      prevY = y;
    }
  }

  // If the last intersection recorded is on the same row as the very first
  // intersection, we pop it to maintain parity (closing the loop).
  if (ring.length > 1 && ring[ring.length - 1].y === ring[0].y) {
    ring.pop();
  }
}
