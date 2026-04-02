import { buildXmlElement, convert, fromVectorGeometry, mergeBBoxes } from '../../index.js';

import type {
  BBOX,
  FeatureIterator,
  GPXBounds,
  GPXCopyright,
  GPXFixType,
  GPXLink,
  GPXMetadata,
  GPXPerson,
  GPXRoute,
  GPXTrack,
  GPXWaypoint,
  Properties,
  VectorFeatures,
  VectorPoint,
  Writer,
} from '../../index.js';

/** User defined options on how to store the features */
export interface ToGPXOptions {
  /** Name of the GPX file */
  name?: string;
  /** User defined metadata */
  metadata?: GPXMetadata;
  /** On a feature, determine if it should be written as a {@link GPXWaypoint}. */
  onFeatureWaypoint?: (feature: VectorFeatures) => GPXWaypoint | undefined;
  /** On a feature, determine if it should be written as a {@link GPXRoute}. */
  onFeatureRoute?: (feature: VectorFeatures) => GPXRoute | undefined;
  /** On a feature, determine if it should be written as a {@link GPXTrack}. */
  onFeatureTrack?: (feature: VectorFeatures) => GPXTrack | undefined;
}

/**
 * # GPX Writer
 *
 * ## Description
 * Given a writer and an array of iterators, write the input features to the writer as a GPX data
 *
 * ## Usage
 *
 * ### Data Prep
 * Be sure to prep the various data. GPX Assumes 3 types: Waypoints (Points), Routes (LineStrings),
 * and Tracks (MultiLineStrings).
 *
 * #### Waypoints
 * Check {@link GPXWaypoint}. The variables ported are `ele`, `time`, `name`, `magvar`, `geoidheight`,
 * `name`, `cmt`, `desc`, `src`, `link`, `sym`, `type`, `fix`, `sat`, `hdop`, `vdop`, `pdop`,
 * `ageofdgpsdata`, `dgpsid`. These variables are optional and should be stored in the m-values of
 * the points
 *
 * #### Routes
 * Check {@link GPXRoute}. The variables ported are `name`, `cmt`, `desc`, `src`, `link`, `number`,
 * `type`, `rtept`. These variables are optional and should be stored in the properties of the feature.
 *
 * #### Tracks
 * Check {@link GPXTrack}. The variables ported are `name`, `cmt`, `desc`, `src`, `link`, `number`,
 * `type`, `trkseg`. These variables are optional and should be stored in the properties of the feature.
 *
 * ```ts
 * import { toGPX, JSONReader } from 'gis-tools-ts';
 * import { FileReader, FileWriter } from 'gis-tools-ts/file';
 * // or use mmap reader if using bun
 * // import { MMapReader } from 'gis-tools-ts/mmap';
 *
 * const fileReader = new FileReader(`${__dirname}/fixtures/points.geojson`);
 * const jsonReader = new JSONReader(fileReader);
 * const bufWriter = new FileWriter(`${__dirname}/fixtures/points.gpx`);
 *
 * // store to singular output
 * await toGPX(bufWriter, [jsonReader]);
 * ```
 *
 * ## Links
 * https://www.topografix.com/gpx.asp
 *
 * @param writer - the writer to append strings to
 * @param iterators - the collection of iterators to write
 * @param opts - user defined options [optional]
 */
export async function toGPX(
  writer: Writer,
  iterators: FeatureIterator[],
  opts?: ToGPXOptions,
): Promise<void> {
  let bbox: BBOX | undefined;
  const name = opts?.name ?? 'GIS Tools Output Data';
  const featureWaypoint = opts?.onFeatureWaypoint ?? onFeatureWaypoint;
  const featureRoute = opts?.onFeatureRoute ?? onFeatureRoute;
  const featureTrack = opts?.onFeatureTrack ?? onFeatureTrack;

  const waypoints: string[] = [];
  const routes: string[] = [];
  const tracks: string[] = [];

  for (const iterator of iterators) {
    for await (const feature of iterator) {
      const convertedFeatures = convert('WG', feature, true, false);
      for (const convFeature of convertedFeatures) {
        // handle bbox
        let gBbox = convFeature.geometry.bbox;
        if (gBbox === undefined) gBbox = fromVectorGeometry(convFeature.geometry);
        bbox = mergeBBoxes(bbox, gBbox);
        // handle waypoints
        const waypoint = featureWaypoint(convFeature);
        if (waypoint !== undefined) waypoints.push(writeWaypoint(waypoint));
        // handle routes
        const route = featureRoute(convFeature);
        if (route !== undefined) routes.push(writeRoute(route));
        // handle tracks
        const track = featureTrack(convFeature);
        if (track !== undefined) tracks.push(writeTrack(track));
      }
    }
  }

  const metadata = writeMetadata(opts?.metadata ?? setupMetadata(name, bbox ?? [0, 0, 0, 0]));
  const output =
    '<?xml version="1.0" encoding="UTF-8" ?>\n' +
    buildXmlElement('gpx', { version: 1.1, creator: 'GIS_TOOLS' }, [
      metadata,
      ...waypoints,
      ...routes,
      ...tracks,
    ]);

  await writer.appendString(output);
}

function onFeatureWaypoint(feature: VectorFeatures): GPXWaypoint | undefined {
  const { geometry, properties } = feature;
  const { type: gType, coordinates } = geometry;
  if (gType !== 'Point') return undefined;
  return encodePoint(coordinates, properties);
}

function encodePoint(point: VectorPoint, properties: Properties = {}): GPXWaypoint {
  const { x: lon, y: lat, z: ele, m = {} } = point;
  const name = (properties.name ?? m.name) as string | undefined;
  const type = (properties.type ?? m.type) as string | undefined;
  const time = (properties.time ?? m.time) as string | undefined;
  const magvar = (properties.magvar ?? m.magvar) as number | undefined;
  const geoidheight = (properties.geoidheight ?? m.geoidheight) as number | undefined;
  const cmt = (properties.cmt ?? m.cmt) as string | undefined;
  const desc = (properties.desc ?? m.desc) as string | undefined;
  const src = (properties.src ?? m.src) as string | undefined;
  const link = (properties.link ?? m.link) as GPXLink[] | undefined;
  const sym = (properties.sym ?? m.sym) as string | undefined;
  const fix = (properties.fix ?? m.fix) as GPXFixType | undefined;
  const sat = (properties.sat ?? m.sat) as number | undefined;
  const hdop = (properties.hdop ?? m.hdop) as number | undefined;
  const vdop = (properties.vdop ?? m.vdop) as number | undefined;
  const pdop = (properties.pdop ?? m.pdop) as number | undefined;
  const ageofdgpsdata = (properties.ageofdgpsdata ?? m.ageofdgpsdata) as number | undefined;
  const dgpsid = (properties.dgpsid ?? m.dgpsid) as number | undefined;

  return {
    lon,
    lat,
    ele,
    name,
    time,
    magvar,
    geoidheight,
    cmt,
    desc,
    src,
    link,
    sym,
    type,
    fix,
    sat,
    hdop,
    vdop,
    pdop,
    ageofdgpsdata,
    dgpsid,
  };
}

function onFeatureRoute(feature: VectorFeatures): GPXRoute | undefined {
  const { geometry, properties } = feature;
  const { type: gType, coordinates } = geometry;
  if (gType !== 'LineString') return undefined;

  const name = properties.name as string | undefined;
  const number = properties.number as number | undefined;
  const cmt = properties.cmt as string | undefined;
  const desc = properties.desc as string | undefined;
  const src = properties.src as string | undefined;
  const link = properties.link as GPXLink[] | undefined;
  const type = properties.type as string | undefined;
  const rtept = coordinates.map((point) => encodePoint(point));

  return { name, number, cmt, desc, src, link, type, rtept };
}

function onFeatureTrack(feature: VectorFeatures): GPXTrack | undefined {
  const { geometry, properties } = feature;
  const { type: gType, coordinates } = geometry;
  if (gType !== 'MultiLineString') return undefined;

  const name = properties.name as string | undefined;
  const number = properties.number as number | undefined;
  const cmt = properties.cmt as string | undefined;
  const desc = properties.desc as string | undefined;
  const src = properties.src as string | undefined;
  const link = properties.link as GPXLink[] | undefined;
  const type = properties.type as string | undefined;

  const trkseg = [];
  for (const line of coordinates) {
    const trkpt = line.map((point) => encodePoint(point));
    trkseg.push({ trkpt });
  }

  return { name, number, cmt, desc, src, link, type, trkseg };
}

function setupMetadata(name: string, bounds: BBOX): GPXMetadata {
  const [minlat, minlon, maxlat, maxlon] = bounds;
  return { name, bounds: { minlat, minlon, maxlat, maxlon } };
}

function writeWaypoint(waypoint: GPXWaypoint, tagName = 'wpt'): string {
  const { lon, lat, name, ele, time, magvar, geoidheight, cmt, desc, src, link, sym } = waypoint;
  const { type, fix, sat, hdop, vdop, pdop, ageofdgpsdata, dgpsid } = waypoint;
  const inner: string[] = [];
  if (name !== undefined) inner.push(buildXmlElement('name', {}, [name]));
  if (ele !== undefined) inner.push(buildXmlElement('ele', {}, [String(ele)]));
  if (time !== undefined) inner.push(buildXmlElement('time', {}, [time]));
  if (magvar !== undefined) inner.push(buildXmlElement('magvar', {}, [String(magvar)]));
  if (geoidheight !== undefined)
    inner.push(buildXmlElement('geoidheight', {}, [String(geoidheight)]));
  if (cmt !== undefined) inner.push(buildXmlElement('cmt', {}, [cmt]));
  if (desc !== undefined) inner.push(buildXmlElement('desc', {}, [desc]));
  if (src !== undefined) inner.push(buildXmlElement('src', {}, [src]));
  if (link !== undefined) for (const l of link) inner.push(writeLink(l));
  if (sym !== undefined) inner.push(buildXmlElement('sym', {}, [sym]));
  if (type !== undefined) inner.push(buildXmlElement('type', {}, [type]));
  if (fix !== undefined) inner.push(buildXmlElement('fix', {}, [fix]));
  if (sat !== undefined) inner.push(buildXmlElement('sat', {}, [String(sat)]));
  if (hdop !== undefined) inner.push(buildXmlElement('hdop', {}, [String(hdop)]));
  if (vdop !== undefined) inner.push(buildXmlElement('vdop', {}, [String(vdop)]));
  if (pdop !== undefined) inner.push(buildXmlElement('pdop', {}, [String(pdop)]));
  if (ageofdgpsdata !== undefined)
    inner.push(buildXmlElement('ageofdgpsdata', {}, [String(ageofdgpsdata)]));
  if (dgpsid !== undefined) inner.push(buildXmlElement('dgpsid', {}, [String(dgpsid)]));
  return buildXmlElement(tagName, { lat, lon }, inner);
}

function writeRoute(route: GPXRoute): string {
  const { name, cmt, desc, src, link, number, type, rtept } = route;
  const inner: string[] = [];
  if (name !== undefined) inner.push(buildXmlElement('name', {}, [name]));
  if (cmt !== undefined) inner.push(buildXmlElement('cmt', {}, [cmt]));
  if (desc !== undefined) inner.push(buildXmlElement('desc', {}, [desc]));
  if (src !== undefined) inner.push(buildXmlElement('src', {}, [src]));
  if (link !== undefined) for (const l of link) inner.push(writeLink(l));
  if (number !== undefined) inner.push(buildXmlElement('number', {}, [String(number)]));
  if (type !== undefined) inner.push(buildXmlElement('type', {}, [type]));
  if (rtept !== undefined) {
    for (const point of rtept) inner.push(writeWaypoint(point, 'rtept'));
  }
  return buildXmlElement('rte', {}, inner);
}

function writeTrack(track: GPXTrack): string {
  const { name, cmt, desc, src, link, number, type, trkseg } = track;
  const inner: string[] = [];

  if (name !== undefined) inner.push(buildXmlElement('name', {}, [name]));
  if (cmt !== undefined) inner.push(buildXmlElement('cmt', {}, [cmt]));
  if (desc !== undefined) inner.push(buildXmlElement('desc', {}, [desc]));
  if (src !== undefined) inner.push(buildXmlElement('src', {}, [src]));
  if (link !== undefined) for (const l of link) inner.push(writeLink(l));
  if (number !== undefined) inner.push(buildXmlElement('number', {}, [String(number)]));
  if (type !== undefined) inner.push(buildXmlElement('type', {}, [type]));
  if (trkseg !== undefined) {
    const trkpts = [];
    for (const { trkpt } of trkseg) {
      for (const point of trkpt ?? []) trkpts.push(writeWaypoint(point, 'trkpt'));
    }
    inner.push(buildXmlElement('trkseg', {}, trkpts));
  }

  return buildXmlElement('trk', {}, inner);
}

function writeMetadata(metadata: GPXMetadata): string {
  const { name, desc, author, copyright, link, time, keywords, bounds } = metadata;
  const inner: string[] = [];
  if (name !== undefined) inner.push(buildXmlElement('name', {}, [name]));
  if (desc !== undefined) inner.push(buildXmlElement('desc', {}, [desc]));
  if (link !== undefined) for (const l of link) inner.push(writeLink(l));
  if (time !== undefined) inner.push(buildXmlElement('time', {}, [time]));
  if (keywords !== undefined) inner.push(buildXmlElement('keywords', {}, [keywords]));
  if (author !== undefined) inner.push(writePerson(author));
  if (copyright !== undefined) inner.push(writeCopyright(copyright));
  if (bounds !== undefined) inner.push(writeBounds(bounds));
  return buildXmlElement('metadata', {}, inner);
}

function writeCopyright(copyright: GPXCopyright): string {
  const { author, year, license } = copyright;
  const inner: string[] = [];
  if (author !== undefined) inner.push(buildXmlElement('author', {}, [author]));
  if (year !== undefined) inner.push(buildXmlElement('year', {}, [year]));
  if (license !== undefined) inner.push(buildXmlElement('license', {}, [license]));
  return buildXmlElement('copyright', {}, inner);
}

function writePerson(person: GPXPerson): string {
  const { name, email, link } = person;
  const inner: string[] = [];
  if (name !== undefined) inner.push(buildXmlElement('name', {}, [name]));
  if (email !== undefined)
    inner.push(buildXmlElement('email', { id: email.id, domain: email.domain }, []));
  if (link !== undefined) inner.push(writeLink(link));
  return buildXmlElement('author', {}, inner);
}

function writeBounds(bounds: GPXBounds): string {
  const { minlon, minlat, maxlon, maxlat } = bounds;
  return buildXmlElement('bounds', { minlon, minlat, maxlon, maxlat }, []);
}

function writeLink(link: GPXLink): string {
  const { href, text, type } = link;
  const inner: string[] = [];
  if (text !== undefined) inner.push(buildXmlElement('text', {}, [text]));
  if (type !== undefined) inner.push(buildXmlElement('type', {}, [type]));
  return buildXmlElement('link', { href }, inner);
}
