import type { ValueArrayObject } from '../..';

/** Represents the root GPX document. */
export interface GPX {
  /** Fixed GPX version */
  version: '1.1';
  /** Name or URL of the software that created the GPX document */
  creator: string;
  /** Optional metadata about the file */
  metadata?: GPXMetadata;
  /** Array of waypoints */
  wpt?: GPXWaypoint[];
  /** Array of routes */
  rte?: GPXRoute[];
  /** Array of tracks */
  trk?: GPXTrack[];
  /** Custom extensions for additional data */
  extensions?: GPXExtensions;
}

/** Contains metadata information about the GPX file. */
export interface GPXMetadata {
  /** Name of the GPX file */
  name?: string;
  /** Description of the file's contents */
  desc?: string;
  /** Person or organization responsible for the file */
  author?: GPXPerson;
  /** Copyright and license information */
  copyright?: GPXCopyright;
  /** URLs associated with the GPX file */
  link?: GPXLink[];
  /** Creation timestamp in ISO 8601 format */
  time?: string;
  /** Keywords for classification */
  keywords?: string;
  /** Bounding box of the data */
  bounds?: GPXBounds;
  /** Custom extensions */
  extensions?: GPXExtensions;
}

/** Represents a route, which is an ordered list of waypoints leading to a destination. */
export interface GPXRoute {
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
  /** Ordered list of route waypoints */
  rtept?: GPXWaypoint[];
  /** Custom extensions */
  extensions?: GPXExtensions;
}

/** Represents a track, which is an ordered list of points describing a path. */
export interface GPXTrack {
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
  /** Ordered list of track segments */
  trkseg?: GPXTrackSegment[];
  /** Custom extensions */
  extensions?: GPXExtensions;
}

/** Represents a track segment, which holds a list of track points logically connected in order. */
export interface GPXTrackSegment {
  /** Ordered list of track points */
  trkpt?: GPXWaypoint[];
  /** Custom extensions */
  extensions?: GPXExtensions;
}

/** Represents a waypoint, point of interest, or named feature on a map. */
export interface GPXWaypoint {
  /** Latitude in decimal degrees (WGS84) */
  lat: number;
  /** Longitude in decimal degrees (WGS84) */
  lon: number;
  /** Optional elevation in meters */
  ele?: number;
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
  /** Custom extensions */
  extensions?: GPXExtensions;
}

/** Represents custom extensions for additional data fields. */
export interface GPXExtensions {
  /** Custom extension fields */
  [key: string]: unknown;
}

/** Defines copyright and license information. */
export interface GPXCopyright {
  /** Copyright holder */
  author: string;
  /** Year of copyright */
  year?: string;
  /** License URL */
  license?: string;
}

/** Represents a hyperlink with optional text and MIME type. */
export interface GPXLink extends ValueArrayObject {
  /** URL of the link */
  href: string;
  /** Optional hyperlink text */
  text?: string;
  /** MIME type of the linked content */
  type?: string;
}

/** Defines a person or organization associated with the GPX file. */
export interface GPXPerson {
  /** Name of the person or organization */
  name?: string;
  /** Email address (split into ID and domain) */
  email?: GPXEmail;
  /** Link to external information about the person */
  link?: GPXLink;
}

/** Represents an email address, split into ID and domain parts. */
export interface GPXEmail {
  /** Local part of the email address */
  id: string;
  /** Domain part of the email address */
  domain: string;
}

/** Defines the bounding box of the GPX data. */
export interface GPXBounds {
  /** Minimum latitude */
  minlat: number;
  /** Minimum longitude */
  minlon: number;
  /** Maximum latitude */
  maxlat: number;
  /** Maximum longitude */
  maxlon: number;
}

/** Enumeration of GPS fix types. */
export type GPXFixType = 'none' | '2d' | '3d' | 'dgps' | 'pps';
