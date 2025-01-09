import { EARTH_CIRCUMFERENCE } from '../..';
import { degToRad, radToDeg } from '../util';

import type { BBox, Point, Point3D } from '../';

/** 900913 (Web Mercator) constant */
export const A = 6_378_137.0;
/** 900913 (Web Mercator) max extent */
export const MAXEXTENT = 20_037_508.342789244;
/** 900913 (Web Mercator) maximum latitude */
export const MAXLAT = 85.0511287798;

/** The source of the coordinate inputs */
export type Sources = 'WGS84' | '900913';

/**
 * Given a zoom and tilesize, build mercator positional attributes
 * @param zoom - the zoom level
 * @param tileSize - in pixels
 * @returns - a bounding box sharing zoom size bounds
 */
function getZoomSize(zoom: number, tileSize: number): BBox {
  const size = tileSize * Math.pow(2, zoom);
  return [size / 360, size / (2 * Math.PI), size / 2, size];
}

/**
 * Convert Longitude and Latitude to a mercator pixel coordinate
 * @param ll - the longitude and latitude
 * @param zoom - the zoom level
 * @param antiMeridian - true if you want to use the antimeridian
 * @param tileSize - in pixels
 * @returns - the mercator pixel
 */
export function llToPX(ll: Point, zoom: number, antiMeridian = false, tileSize = 512): Point {
  const { min, max, sin, log } = Math;
  const [Bc, Cc, Zc, Ac] = getZoomSize(zoom, tileSize);
  const expansion = antiMeridian ? 2 : 1;
  const d = Zc;
  const f = min(max(sin(degToRad(ll[1])), -0.999999999999), 0.999999999999);
  let x = d + ll[0] * Bc;
  let y = d + 0.5 * log((1 + f) / (1 - f)) * -Cc;
  if (x > Ac * expansion) x = Ac * expansion;
  if (y > Ac) y = Ac;

  return [x, y];
}

/**
 * Convert mercator pixel coordinates to Longitude and Latitude
 * @param px - the mercator pixel
 * @param zoom - the zoom level
 * @param tileSize - in pixels
 * @returns - the longitude and latitude
 */
export function pxToLL(px: Point, zoom: number, tileSize = 512): Point {
  const { atan, exp, PI } = Math;
  const [Bc, Cc, Zc] = getZoomSize(zoom, tileSize);
  const g = (px[1] - Zc) / -Cc;
  const lon = (px[0] - Zc) / Bc;
  const lat = radToDeg(2 * atan(exp(g)) - 0.5 * PI);
  return [lon, lat];
}

/**
 * Convert Longitude and Latitude to a mercator x-y coordinates
 * @param ll - the longitude and latitude
 * @returns - the mercator pixel
 */
export function llToMerc(ll: Point): Point {
  const { tan, log, PI } = Math;
  let x = degToRad(A * ll[0]);
  let y = A * log(tan(PI * 0.25 + degToRad(0.5 * ll[1])));
  // if xy value is beyond maxextent (e.g. poles), return maxextent.
  if (x > MAXEXTENT) x = MAXEXTENT;
  if (x < -MAXEXTENT) x = -MAXEXTENT;
  if (y > MAXEXTENT) y = MAXEXTENT;
  if (y < -MAXEXTENT) y = -MAXEXTENT;

  return [x, y];
}

/**
 * Convert mercator x-y coordinates to Longitude and Latitude
 * @param merc - the mercator pixel
 * @returns - the longitude and latitude
 */
export function mercToLL(merc: Point): Point {
  const { atan, exp, PI } = Math;
  const x = radToDeg(merc[0] / A);
  const y = radToDeg(0.5 * PI - 2 * atan(exp(-merc[1] / A)));
  return [x, y];
}

/**
 * Convert a pixel coordinate to a tile x-y coordinate
 * @param px - the pixel
 * @param tileSize - in pixels
 * @returns - the tile x-y
 */
export function pxToTile(px: Point, tileSize = 512): Point {
  const { floor } = Math;
  const x = floor(px[0] / tileSize);
  const y = floor(px[1] / tileSize);
  return [x, y];
}

/**
 * Convert a tile x-y-z to a bbox of the form `[w, s, e, n]`
 * @param tile - the tile
 * @param tileSize - in pixels
 * @returns - the bbox
 */
export function tilePxBounds(tile: Point3D, tileSize = 512): BBox {
  const [, x, y] = tile;
  const minX = x * tileSize;
  const minY = y * tileSize;
  const maxX = minX + tileSize;
  const maxY = minY + tileSize;
  return [minX, minY, maxX, maxY];
}

/**
 * Convert a lat-lon and zoom to the tile's x-y coordinates
 * @param ll - the lat-lon
 * @param zoom - the zoom
 * @param tileSize - in pixels
 * @returns - the tile x-y
 */
export function llToTile(ll: Point, zoom: number, tileSize = 512): Point {
  const px = llToPX(ll, zoom, false, tileSize);
  return pxToTile(px, tileSize);
}

/**
 * given a lon-lat and tile, find the offset in pixels
 * @param ll - the lon-lat
 * @param tile - the tile
 * @param tileSize - in pixels
 * @returns - the tile x-y
 */
export function llToTilePx(ll: Point, tile: Point3D, tileSize = 512): Point {
  const [zoom, x, y] = tile;
  const px = llToPX(ll, zoom, false, tileSize);
  const tileXStart = x * tileSize;
  const tileYStart = y * tileSize;

  return [(px[0] - tileXStart) / tileSize, (px[1] - tileYStart) / tileSize];
}

/**
 * Convert a bbox of the form `[w, s, e, n]` to a bbox of the form `[w, s, e, n]`
 * The result can be in lon-lat (WGS84) or WebMercator (900913)
 * If the input is in WebMercator (900913), the outSource should be set to 'WGS84'
 * @param bbox - the bounding box to convert
 * @param outSource - the output source
 * @returns - the converted bbox
 */
export function convert(bbox: BBox, outSource: Sources): BBox {
  if (outSource === 'WGS84') {
    const low = mercToLL([bbox[0], bbox[1]]);
    const high = mercToLL([bbox[2], bbox[3]]);
    return [low[0], low[1], high[0], high[1]];
  } else {
    const low = llToMerc([bbox[0], bbox[1]]);
    const high = llToMerc([bbox[2], bbox[3]]);
    return [low[0], low[1], high[0], high[1]];
  }
}

/**
 * Convert a tile x-y-z to a bbox of the form `[w, s, e, n]`
 * The result can be in lon-lat (WGS84) or WebMercator (900913)
 * The default result is in WebMercator (900913)
 * @param x - the x tile position
 * @param y - the y tile position
 * @param zoom - the zoom level
 * @param tmsStyle - if true, the y is inverted
 * @param source - the source
 * @param tileSize - in pixels
 * @returns - the bounding box in WGS84
 */
export function xyzToBBOX(
  x: number,
  y: number,
  zoom: number,
  tmsStyle = true,
  source: Sources = '900913',
  tileSize = 512,
): BBox {
  // Convert xyz into bbox with srs WGS84
  // if tmsStyle, the y is inverted
  if (tmsStyle) y = Math.pow(2, zoom) - 1 - y;
  // Use +y to make sure it's a number to avoid inadvertent concatenation.
  const bl: Point = [x * tileSize, (y + 1) * tileSize]; // bottom left
  // Use +x to make sure it's a number to avoid inadvertent concatenation.
  const tr: Point = [(x + 1) * tileSize, y * tileSize]; // top right
  // to pixel-coordinates
  const pxBL = pxToLL(bl, zoom, tileSize);
  const pxTR = pxToLL(tr, zoom, tileSize);

  // If web mercator requested reproject to 900913.
  if (source === '900913') {
    const llBL = llToMerc(pxBL);
    const llTR = llToMerc(pxTR);
    return [llBL[0], llBL[1], llTR[0], llTR[1]];
  }
  return [pxBL[0], pxBL[1], pxTR[0], pxTR[1]];
}

/**
 * Convert a bbox of the form `[w, s, e, n]` to a tile's bounding box
 * in the form of [minX, maxX, minY, maxY]
 * The bbox can be in lon-lat (WGS84) or WebMercator (900913)
 * The default expectation is in WebMercator (900913)
 * @param bbox - the bounding box
 * @param zoom - the zoom level
 * @param tmsStyle - if true, the y is inverted
 * @param source - the source
 * @param tileSize - in pixels
 * @returns - the tile's bounding box [minX, minY, maxX, maxY]
 */
export function bboxToXYZBounds(
  bbox: BBox,
  zoom: number,
  tmsStyle = true,
  source: Sources = '900913',
  tileSize = 512,
): BBox {
  const { min, max, pow, floor } = Math;
  let bl: Point = [bbox[0], bbox[1]]; // bottom left
  let tr: Point = [bbox[2], bbox[3]]; // top right

  if (source === '900913') {
    bl = llToMerc(bl);
    tr = llToMerc(tr);
  }

  const pxBL = llToPX(bl, zoom, false, tileSize);
  const pxTR = llToPX(tr, zoom, false, tileSize);
  // Y = 0 for XYZ is the top hence minY uses pxTR[1].
  const x = [floor(pxBL[0] / tileSize), floor((pxTR[0] - 1) / tileSize)];
  const y = [floor(pxTR[1] / tileSize), floor((pxBL[1] - 1) / tileSize)];

  const bounds: BBox = [
    min(...x) < 0 ? 0 : min(...x),
    min(...y) < 0 ? 0 : min(...y),
    max(...x),
    max(...y),
  ];

  if (tmsStyle) {
    const tmsMinY = pow(2, zoom) - 1 - bounds[3];
    const tmsMaxY = pow(2, zoom) - 1 - bounds[1];
    bounds[1] = tmsMinY;
    bounds[3] = tmsMaxY;
  }

  return bounds;
}

/**
 * The circumference at a line of latitude in meters.
 * @param latitude - in degrees
 * @returns - the circumference
 */
function circumferenceAtLatitude(latitude: number): number {
  return EARTH_CIRCUMFERENCE * Math.cos((latitude * Math.PI) / 180);
}

/**
 * Convert longitude to mercator projection X-Value
 * @param lng - in degrees
 * @returns the X-Value
 */
export function mercatorXfromLng(lng: number): number {
  return (180 + lng) / 360;
}

/**
 * Convert latitude to mercator projection Y-Value
 * @param lat - in degrees
 * @returns the Y-Value
 */
export function mercatorYfromLat(lat: number): number {
  const { PI, log, tan } = Math;
  return (180 - (180 / PI) * log(tan(PI / 4 + (lat * PI) / 360))) / 360;
}

/**
 * Convert altitude to mercator projection Z-Value
 * @param altitude - in meters
 * @param lat - in degrees
 * @returns the Z-Value
 */
export function mercatorZfromAltitude(altitude: number, lat: number): number {
  return altitude / circumferenceAtLatitude(lat);
}

/**
 * Convert mercator projection's X-Value to longitude
 * @param x - in radians
 * @returns the longitude
 */
export function lngFromMercatorX(x: number): number {
  return x * 360 - 180;
}

/**
 * Convert mercator projection's Y-Value to latitude
 * @param y - in radians
 * @returns the latitude
 */
export function latFromMercatorY(y: number): number {
  const { PI, atan, exp } = Math;
  const y2 = 180 - y * 360;
  return (360 / PI) * atan(exp((y2 * PI) / 180)) - 90;
}

/**
 * Convert mercator projection's Z-Value to altitude
 * @param z - in meters
 * @param y - in radians
 * @returns the altitude
 */
export function altitudeFromMercatorZ(z: number, y: number): number {
  return z * circumferenceAtLatitude(latFromMercatorY(y));
}

/**
 * Determine the Mercator scale factor for a given latitude, see
 * https://en.wikipedia.org/wiki/Mercator_projection#Scale_factor
 *
 * At the equator the scale factor will be 1, which increases at higher latitudes.
 * @param lat - in degrees
 * @returns the scale factor
 */
export function mercatorLatScale(lat: number): number {
  const { cos, PI } = Math;
  return 1 / cos((lat * PI) / 180);
}
