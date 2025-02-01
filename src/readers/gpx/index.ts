import {
  Properties,
  VectorFeature,
  VectorLineStringGeometry,
  VectorMultiLineStringGeometry,
  VectorPointGeometry,
  xmlFindTagByName,
  xmlFindTagsByName,
  xmlGetAttribute,
} from '../..';

import type { FeatureIterator, XMLTag } from '..';
import type {
  GPXBounds,
  GPXCopyright,
  GPXEmail,
  GPXFixType,
  GPXLink,
  GPXMetadata,
  GPXPerson,
  GPXRoute,
  GPXTrack,
  GPXTrackSegment,
  GPXWaypoint,
} from './types';

/** Represents a waypoint, point of interest, or named feature on a map. */
export interface GPXWaypointProperties extends Properties {
  /** Optional timestamp in ISO 8601 format */
  time?: string;
  /** Optional magnetic variation in degrees */
  magvar?: number;
  /** Height of geoid above WGS84 ellipsoid */
  geoidheight?: number;
  /** Waypoint name */
  name?: string;
  /** Waypoint comment */
  cmt?: string;
  /** Description of the waypoint */
  desc?: string;
  /** Source of data */
  src?: string;
  /** Links to additional information */
  link?: GPXLink[];
  /** Symbol name for the waypoint */
  sym?: string;
  /** Classification type of the waypoint */
  type?: string;
  /** Type of GPS fix */
  fix?: GPXFixType;
  /** Number of satellites used for the fix */
  sat?: number;
  /** Horizontal dilution of precision */
  hdop?: number;
  /** Vertical dilution of precision */
  vdop?: number;
  /** Position dilution of precision */
  pdop?: number;
  /** Time since last DGPS update in seconds */
  ageofdgpsdata?: number;
  /** ID of DGPS station used */
  dgpsid?: number;
}

/** Represents a route, which is an ordered list of waypoints leading to a destination. */
export interface GPXRouteProperties extends Properties {
  /** Route name */
  name?: string;
  /** Route comment */
  cmt?: string;
  /** Route description */
  desc?: string;
  /** Source of data */
  src?: string;
  /** Links to external information */
  link?: GPXLink[];
  /** Route number */
  number?: number;
  /** Classification type of the route */
  type?: string;
}

/** Represents a track, which is an ordered list of points describing a path. */
export interface GPXTrackProperties extends Properties {
  /** Track name */
  name?: string;
  /** Track comment */
  cmt?: string;
  /** Track description */
  desc?: string;
  /** Source of data */
  src?: string;
  /** Links to external information */
  link?: GPXLink[];
  /** Track number */
  number?: number;
  /** Classification type of the track */
  type?: string;
}

/** All the possible GPX properties */
export type GPXProperties = GPXWaypointProperties | GPXRouteProperties | GPXTrackProperties;

/**
 * The GPX vector features
 * - Waypoints
 * - Routes
 * - Tracks
 */
export type GPXVectorFeatures =
  | VectorFeature<GPXMetadata, GPXWaypointProperties, GPXProperties, VectorPointGeometry>
  | VectorFeature<
      GPXMetadata,
      GPXWaypointProperties,
      GPXRouteProperties,
      VectorLineStringGeometry<GPXWaypointProperties>
    >
  | VectorFeature<
      GPXMetadata,
      GPXWaypointProperties,
      GPXRouteProperties,
      VectorMultiLineStringGeometry<GPXWaypointProperties>
    >;

/**
 * # GPX Reader
 *
 * ## Description
 * The GPX Reader is an XML-based GPS Exchange Format (GPX) reader.
 * Implements the {@link FeatureIterator} interface
 *
 * ## Links
 * https://www.topografix.com/gpx.asp
 */
export class GPXReader
  implements FeatureIterator<GPXMetadata, GPXWaypointProperties, GPXProperties>
{
  metadata: GPXMetadata;
  wpt?: GPXWaypoint[];
  rte?: GPXRoute[];
  trk?: GPXTrack[];

  /**
   * @param input - The data as either a buffer or file reader
   */
  constructor(input: string) {
    this.metadata = this.#parseMetadata(input);
    this.wpt = this.#parseWaypoints(input);
    this.rte = this.#parseRoutes(input);
    this.trk = this.#parseTracks(input);
  }

  /**
   * Generator to iterate over each WGS84 lon-lat point
   * @yields {VectorFeature}
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<GPXVectorFeatures> {
    for (const wpt of this.wpt ?? []) {
      const { lon, lat, ele } = wpt;
      yield {
        type: 'VectorFeature',
        metadata: this.metadata,
        geometry: {
          type: 'Point',
          coordinates: { x: lon, y: lat, z: ele },
          is3D: ele !== undefined,
        },
        properties: this.#buildWaypointProperties(wpt),
      };
    }
    for (const rte of this.rte ?? []) {
      const { name, cmt, desc, src, link, number, type, rtept } = rte;
      if (rtept === undefined) return;
      yield {
        type: 'VectorFeature',
        metadata: this.metadata,
        geometry: this.#buildRouteCoordinates(rtept),
        properties: { name, cmt, desc, src, link, number, type },
      };
    }
    for (const trk of this.trk ?? []) {
      const { name, cmt, desc, src, link, number, type, trkseg } = trk;
      if (trkseg === undefined) return;
      yield {
        type: 'VectorFeature',
        metadata: this.metadata,
        geometry: this.#buildTrackCoordinates(trkseg),
        properties: { name, cmt, desc, src, link, number, type },
      };
    }
  }

  /**
   * Build a multilinestring from a list of track segments
   * @param trkseg - List of track segments
   * @returns - Multilinestring
   */
  #buildTrackCoordinates(
    trkseg: GPXTrackSegment[],
  ): VectorMultiLineStringGeometry<GPXWaypointProperties> {
    const res: VectorMultiLineStringGeometry<GPXWaypointProperties> = {
      type: 'MultiLineString',
      coordinates: [],
      is3D: trkseg.some(({ trkpt }) => (trkpt ?? []).some((wpt) => wpt.ele !== undefined)),
    };
    for (const { trkpt } of trkseg) {
      if (trkpt === undefined) continue;
      const rtept = this.#buildRouteCoordinates(trkpt);
      res.coordinates.push(rtept.coordinates);
    }
    return res;
  }

  /**
   * Build a linestring from a list of waypoints
   * @param rtept - List of waypoints
   * @returns - Linestring
   */
  #buildRouteCoordinates(rtept: GPXWaypoint[]): VectorLineStringGeometry<GPXWaypointProperties> {
    const res: VectorLineStringGeometry<GPXWaypointProperties> = {
      type: 'LineString',
      coordinates: [],
      is3D: rtept.some((wpt) => wpt.ele !== undefined),
    };
    for (const wpt of rtept) {
      const { lon, lat, ele } = wpt;
      res.coordinates.push({ x: lon, y: lat, z: ele, m: this.#buildWaypointProperties(wpt) });
    }
    return res;
  }

  /**
   * Build the properties from a waypoint
   * @param wpt - the waypoint
   * @returns - the properties
   */
  #buildWaypointProperties(wpt: GPXWaypoint): GPXWaypointProperties {
    const { time, magvar, geoidheight, name, cmt, desc, src, link, sym } = wpt;
    const { type, fix, sat, hdop, vdop, pdop, ageofdgpsdata, dgpsid } = wpt;
    return {
      time,
      magvar,
      geoidheight,
      name,
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

  /**
   * Parse the metadata
   * @param input - the xml string
   * @returns - the metadata
   */
  #parseMetadata(input: string): GPXMetadata {
    const metadata = xmlFindTagByName(input, 'metadata');
    if (metadata === undefined) return {};
    const links = xmlFindTagsByName(metadata.inner ?? '', 'link');
    const link = links.map(this.#parseLink).filter((l) => l !== undefined);
    return {
      name: xmlFindTagByName(metadata.inner ?? '', 'name')?.inner,
      desc: xmlFindTagByName(metadata.inner ?? '', 'desc')?.inner,
      author: this.#parsePerson(xmlFindTagByName(metadata.inner ?? '', 'author')),
      copyright: this.#parseCopyright(xmlFindTagByName(metadata.inner ?? '', 'copyright')),
      link: link.length > 0 ? link : undefined,
      time: xmlFindTagByName(metadata.inner ?? '', 'time')?.inner,
      keywords: xmlFindTagByName(metadata.inner ?? '', 'keywords')?.inner,
      bounds: this.#parseBounds(xmlFindTagByName(metadata.inner ?? '', 'bounds')),
    };
  }

  /**
   * Parse the metadata
   * @param input - the xml string
   * @param tagName - the tag name. Defaults to `wpt`
   * @returns - the metadata
   */
  #parseWaypoints(input: string, tagName = 'wpt'): GPXWaypoint[] {
    const res: GPXWaypoint[] = [];
    const wpts = xmlFindTagsByName(input, tagName);
    for (const wpt of wpts) res.push(this.#parseWaypoint(wpt));
    return res;
  }

  /**
   * Parse a waypoint
   * @param wpt - the waypoint tag
   * @returns - the waypoint
   */
  #parseWaypoint(wpt: XMLTag): GPXWaypoint {
    const links = xmlFindTagsByName(wpt.inner ?? '', 'link');
    const link = links.map(this.#parseLink).filter((l) => l !== undefined);
    const ele = xmlFindTagByName(wpt.inner ?? '', 'ele')?.inner;
    const magvar = xmlFindTagByName(wpt.inner ?? '', 'magvar')?.inner;
    const geoidheight = xmlFindTagByName(wpt.inner ?? '', 'geoidheight')?.inner;
    const sat = xmlFindTagByName(wpt.inner ?? '', 'sat')?.inner;
    const hdop = xmlFindTagByName(wpt.inner ?? '', 'hdop')?.inner;
    const vdop = xmlFindTagByName(wpt.inner ?? '', 'vdop')?.inner;
    const pdop = xmlFindTagByName(wpt.inner ?? '', 'pdop')?.inner;
    const ageofdgpsdata = xmlFindTagByName(wpt.inner ?? '', 'ageofdgpsdata')?.inner;
    const dgpsid = xmlFindTagByName(wpt.inner ?? '', 'dgpsid')?.inner;
    return {
      lat: Number(xmlGetAttribute(wpt.outer, 'lat') ?? 0),
      lon: Number(xmlGetAttribute(wpt.outer, 'lon') ?? 0),
      ele: ele !== undefined ? Number(ele) : undefined,
      time: xmlFindTagByName(wpt.inner ?? '', 'time')?.inner,
      magvar: magvar !== undefined ? Number(magvar) : undefined,
      geoidheight: geoidheight !== undefined ? Number(geoidheight) : undefined,
      name: xmlFindTagByName(wpt.inner ?? '', 'name')?.inner,
      cmt: xmlFindTagByName(wpt.inner ?? '', 'cmt')?.inner,
      desc: xmlFindTagByName(wpt.inner ?? '', 'desc')?.inner,
      src: xmlFindTagByName(wpt.inner ?? '', 'src')?.inner,
      link: link.length > 0 ? link : undefined,
      sym: xmlFindTagByName(wpt.inner ?? '', 'sym')?.inner,
      type: xmlFindTagByName(wpt.inner ?? '', 'type')?.inner,
      fix: xmlGetAttribute(wpt.outer, 'type') as GPXFixType,
      sat: sat !== undefined ? Number(sat) : undefined,
      hdop: hdop !== undefined ? Number(hdop) : undefined,
      vdop: vdop !== undefined ? Number(vdop) : undefined,
      pdop: pdop !== undefined ? Number(pdop) : undefined,
      ageofdgpsdata: ageofdgpsdata !== undefined ? Number(ageofdgpsdata) : undefined,
      dgpsid: dgpsid !== undefined ? Number(dgpsid) : undefined,
    };
  }

  /**
   * Parse the routes
   * @param input - the xml string
   * @returns - the routes
   */
  #parseRoutes(input: string): GPXRoute[] {
    const res: GPXRoute[] = [];
    const routes = xmlFindTagsByName(input, 'rte');
    for (const route of routes) res.push(this.#parseRoute(route));
    return res;
  }

  /**
   * Parse a route
   * @param route - the route tag
   * @returns - the route
   */
  #parseRoute(route: XMLTag): GPXRoute {
    const links = xmlFindTagsByName(route.inner ?? '', 'link');
    const link = links.map(this.#parseLink).filter((l) => l !== undefined);
    const number = xmlGetAttribute(route.outer, 'number');
    return {
      name: xmlFindTagByName(route.inner ?? '', 'name')?.inner,
      cmt: xmlFindTagByName(route.inner ?? '', 'cmt')?.inner,
      desc: xmlFindTagByName(route.inner ?? '', 'desc')?.inner,
      src: xmlFindTagByName(route.inner ?? '', 'src')?.inner,
      link: link.length > 0 ? link : undefined,
      number: number !== undefined ? Number(number) : undefined,
      type: xmlFindTagByName(route.inner ?? '', 'type')?.inner,
      rtept: this.#parseWaypoints(route.inner ?? '', 'rtept'),
    };
  }

  /**
   * Parse the tracks
   * @param input - the xml string
   * @returns - the tracks
   */
  #parseTracks(input: string): GPXTrack[] {
    const res: GPXTrack[] = [];
    const tracks = xmlFindTagsByName(input, 'trk');
    for (const track of tracks) res.push(this.#parseTrack(track));
    return res;
  }

  /**
   * Parse a track
   * @param track - the track tag
   * @returns - the track
   */
  #parseTrack(track: XMLTag): GPXTrack {
    const links = xmlFindTagsByName(track.inner ?? '', 'link');
    const link = links.map(this.#parseLink).filter((l) => l !== undefined);
    const number = xmlGetAttribute(track.outer, 'number');
    return {
      name: xmlFindTagByName(track.inner ?? '', 'name')?.inner,
      cmt: xmlFindTagByName(track.inner ?? '', 'cmt')?.inner,
      desc: xmlFindTagByName(track.inner ?? '', 'desc')?.inner,
      src: xmlFindTagByName(track.inner ?? '', 'src')?.inner,
      link: link.length > 0 ? link : undefined,
      number: number !== undefined ? Number(number) : undefined,
      type: xmlFindTagByName(track.inner ?? '', 'type')?.inner,
      trkseg: this.#parseTrackSegments(track.inner ?? '', 'trkseg'),
    };
  }

  /**
   * Parse the track segments
   * @param input - the xml string
   * @param tag - the tag to look for
   * @returns - the track segments
   */
  #parseTrackSegments(input: string, tag: string): GPXTrackSegment[] {
    const res: GPXTrackSegment[] = [];
    const segments = xmlFindTagsByName(input, tag);
    for (const segment of segments) res.push(this.#parseTrackSegment(segment));
    return res;
  }

  /**
   * Parse a track segment
   * @param segment - the track segment tag
   * @returns - the track segment
   */
  #parseTrackSegment(segment: XMLTag): GPXTrackSegment {
    return {
      trkpt: this.#parseWaypoints(segment.inner ?? '', 'trkpt'),
    };
  }

  /**
   * Parse a person
   * @param person - the person tag
   * @returns - the person
   */
  #parsePerson(person: XMLTag | undefined): GPXPerson | undefined {
    if (person === undefined) return;
    return {
      name: xmlFindTagByName(person.inner ?? '', 'name')?.inner ?? '',
      email: this.#parseEmail(xmlFindTagByName(person.inner ?? '', 'email')),
      link: this.#parseLink(xmlFindTagByName(person.inner ?? '', 'link')),
    };
  }

  /**
   * Parse an email
   * @param email - the email tag
   * @returns - the email
   */
  #parseEmail(email: XMLTag | undefined): GPXEmail | undefined {
    if (email === undefined) return;
    return {
      id: xmlGetAttribute(email.outer, 'id') ?? '',
      domain: xmlGetAttribute(email.outer, 'domain') ?? '',
    };
  }

  /**
   * Parse a link
   * @param link - the link tag
   * @returns - the link
   */
  #parseLink(link: XMLTag | undefined): GPXLink | undefined {
    if (link === undefined) return;
    return {
      href: xmlGetAttribute(link.outer, 'href') ?? '',
      text: xmlFindTagByName(link.inner ?? '', 'text')?.inner ?? '',
      type: xmlFindTagByName(link.inner ?? '', 'type')?.inner ?? '',
    };
  }

  /**
   * Parses the bounds
   * @param bounds - XML tag with the bounds attributes
   * @returns - the bounds if they exist
   */
  #parseBounds(bounds: XMLTag | undefined): GPXBounds | undefined {
    if (bounds === undefined) return;
    const minLon = xmlGetAttribute(bounds?.outer ?? '', 'minlon');
    const maxLon = xmlGetAttribute(bounds?.outer ?? '', 'maxlon');
    const minLat = xmlGetAttribute(bounds?.outer ?? '', 'minlat');
    const maxLat = xmlGetAttribute(bounds?.outer ?? '', 'maxlat');
    if (
      minLon === undefined ||
      maxLon === undefined ||
      minLat === undefined ||
      maxLat === undefined
    )
      return;
    return {
      minlat: Number(minLat),
      minlon: Number(minLon),
      maxlat: Number(maxLat),
      maxlon: Number(maxLon),
    };
  }

  /**
   * Parse the copyright
   * @param copyright - the copyright tag
   * @returns - the copyright
   */
  #parseCopyright(copyright: XMLTag | undefined): GPXCopyright | undefined {
    if (copyright === undefined) return;
    return {
      author: xmlFindTagByName(copyright.inner ?? '', 'author')?.inner ?? '',
      year: xmlFindTagByName(copyright.inner ?? '', 'year')?.inner ?? '',
      license: xmlFindTagByName(copyright.inner ?? '', 'license')?.inner ?? '',
    };
  }
}
