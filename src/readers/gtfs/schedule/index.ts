// https://gtfs.org/documentation/schedule/reference/#agencytxt
import { BufferJSONReader, iterZipFolder } from '../../../index.js';

import { type GTFSAgency, parseGTFSAgencies } from './agency.js';
import { type GTFSArea, parseGTFSAreas } from './areas.js';
import { type GTFSAttribution, parseGTFSAttributions } from './attributions.js';
import { type GTFSBookingRule, parseGTFSBookingRules } from './bookingRules.js';
import { type GTFSCalendar, parseGTFSCalendars } from './calendar.js';
import { type GTFSCalendarDate, parseGTFSCalendarDates } from './calendarDates.js';
import { type GTFSFareAttribute, parseGTFSFareAttributes } from './fareAttributes.js';
import { type GTFSFareLegJoinRule, parseGTFSFareLegJoinRules } from './fareLegJoinRules.js';
import { type GTFSFareLegRule, parseGTFSFareLegRules } from './fareLegRules.js';
import { type GTFSFareMedia, parseGTFSFareMedias } from './fareMedia.js';
import { type GTFSFareProduct, parseGTFSFareProducts } from './fareProducts.js';
import { type GTFSFareRule, parseGTFSFareRules } from './fareRules.js';
import { type GTFSFareTransferRule, parseGTFSFareTransferRules } from './fareTransferRules.js';
import { type GTFSFeedInfo, parseGTFSFeedInfos } from './feedInfo.js';
import { type GTFSFrequency, parseGTFSFrequencies } from './frequencies.js';
import { type GTFSLevel, parseGTFSLevels } from './levels.js';
import { type GTFSLocationGroup, parseGTFSLocationGroups } from './locationGroups.js';
import { type GTFSLocationGroupStop, parseGTFSLocationGroupStops } from './locationGroupStops.js';
import { type GTFSNetwork, parseGTFSNetworks } from './networks.js';
import { type GTFSPathway, parseGTFSPathways } from './pathways.js';
import { type GTFSRoute, parseGTFSRoutes } from './routes.js';
import { type GTFSRouteNetwork, parseGTFSRouteNetworks } from './routeNetworks.js';
import { type GTFSShapeProperties, parseGTFSShapes } from './shapes.js';
import { type GTFSStop, type GTFSStopProperties, parseGTFSStops } from './stops.js';
import { type GTFSStopArea, parseGTFSStopAreas } from './stopAreas.js';
import { type GTFSStopTime, parseGTFSStopTimes } from './stopTimes.js';
import { type GTFSTimeframe, parseGTFSTimeframes } from './timeframes.js';
import { type GTFSTransfer, parseGTFSTransfers } from './transfers.js';
import { type GTFSTranslation, parseGTFSTranslations } from './translations.js';
import { type GTFSTrip, parseGTFSTrips } from './trips.js';

import type { FeatureIterator } from '../../index.js';
import type {
  MValue,
  Properties,
  VectorFeature,
  VectorLineStringGeometry,
  VectorMultiPolygonGeometry,
  VectorPointGeometry,
  VectorPolygonGeometry,
} from '../../../geometry/index.js';

export * from './agency.js';
export * from './areas.js';
export * from './attributions.js';
export * from './bookingRules.js';
export * from './calendar.js';
export * from './calendarDates.js';
export * from './fareAttributes.js';
export * from './fareLegJoinRules.js';
export * from './fareLegRules.js';
export * from './fareMedia.js';
export * from './fareProducts.js';
export * from './fareRules.js';
export * from './fareTransferRules.js';
export * from './feedInfo.js';
export * from './frequencies.js';
export * from './levels.js';
export * from './locationGroups.js';
export * from './locationGroupStops.js';
export * from './networks.js';
export * from './pathways.js';
export * from './routeNetworks.js';
export * from './routes.js';
export * from './shapes.js';
export * from './stopAreas.js';
export * from './stops.js';
export * from './stopTimes.js';
export * from './timeframes.js';
export * from './transfers.js';
export * from './translations.js';
export * from './trips.js';

// TODO: postprocess all interactions like `Trips -> shape_id [Link]` & `StopTime -> On-demand Service Routing Behavior [Link]`

/** A piece of the GTFS schedule */
export interface Piece {
  filename: string;
  data: string;
}

/**
 * `locations.geojson` data properties
 * Defines zones where riders can request either pickup or drop off by on-demand services.
 * These zones are represented as GeoJSON polygons.
 */
export interface GTFSLocationsProperties extends Properties {
  stop_name: string;
  stop_desc: string;
}

/**
 * # GTFS Schedule Reader
 *
 * ## Description
 * Schedule class that pulls in all of the GTFS schedule files and parses them into a single object
 * implements the {@link FeatureIterator} interface.
 *
 * ## Usage
 * ```ts
 * import { buildGTFSSchedule } from 'gis-tools-ts';
 *
 * const schedule = await buildGTFSSchedule(gzipData);
 *
 * for await (const feature of schedule) {
 *   console.log(feature);
 * }
 * ```
 *
 * ## Links
 * - https://mobilitydatabase.org
 * - https://developers.google.com/transit/gtfs/examples/overview
 * - https://gtfs.org/documentation/schedule/reference/#tripstxt
 * - https://mobilitydata.github.io/
 * - https://www.transit.land
 */
export class GTFSScheduleReader implements FeatureIterator {
  agencies!: Record<string, GTFSAgency>;
  areas?: GTFSArea[];
  attributions?: Record<string, GTFSAttribution>;
  bookingRules?: Record<string, GTFSBookingRule>;
  calendar?: GTFSCalendar[];
  calendarDates?: GTFSCalendarDate[];
  fareAttributes?: Record<string, GTFSFareAttribute>;
  fareLegJoinRules?: GTFSFareLegJoinRule[];
  fareLegRules?: GTFSFareLegRule[];
  fareMedia?: Record<string, GTFSFareMedia>;
  fareProducts?: Record<string, GTFSFareProduct>;
  fareRules?: GTFSFareRule[];
  fareTransferRules?: GTFSFareTransferRule[];
  feedInfo?: Record<string, GTFSFeedInfo>;
  frequencies?: GTFSFrequency[];
  levels?: Record<string, GTFSLevel>;
  locationGroups?: Record<string, GTFSLocationGroup>;
  locationGroupStops?: GTFSLocationGroupStop[];
  networks?: Record<string, GTFSNetwork>;
  pathways?: Record<string, GTFSPathway>;
  routeNetworks?: GTFSRouteNetwork[];
  routes!: Record<string, GTFSRoute>;
  shapes?: Record<
    string,
    VectorFeature<Record<string, unknown>, MValue, GTFSShapeProperties, VectorLineStringGeometry>
  >;
  stopAreas?: GTFSStopArea[];
  stops?: Record<string, GTFSStop>;
  stopTimes!: GTFSStopTime[];
  timeframes?: Record<string, GTFSTimeframe>;
  transfers?: GTFSTransfer[];
  translations?: GTFSTranslation[];
  trips!: GTFSTrip[];

  geojson?: BufferJSONReader<Record<string, unknown>, MValue, GTFSLocationsProperties>;

  /** @param pieces - all files */
  constructor(pieces: Piece[]) {
    for (const { filename, data } of pieces) {
      if (filename === 'agency.txt') this.agencies = parseGTFSAgencies(data);
      else if (filename === 'areas.txt') this.areas = parseGTFSAreas(data);
      else if (filename === 'attributions.txt') this.attributions = parseGTFSAttributions(data);
      else if (filename === 'booking_rules.txt') this.bookingRules = parseGTFSBookingRules(data);
      else if (filename === 'calendar.txt') this.calendar = parseGTFSCalendars(data);
      else if (filename === 'calendar_dates.txt') this.calendarDates = parseGTFSCalendarDates(data);
      else if (filename === 'fare_attributes.txt')
        this.fareAttributes = parseGTFSFareAttributes(data);
      else if (filename === 'fare_leg_join_rules.txt')
        this.fareLegJoinRules = parseGTFSFareLegJoinRules(data);
      else if (filename === 'fare_leg_rules.txt') this.fareLegRules = parseGTFSFareLegRules(data);
      else if (filename === 'fare_media.txt') this.fareMedia = parseGTFSFareMedias(data);
      else if (filename === 'fare_products.txt') this.fareProducts = parseGTFSFareProducts(data);
      else if (filename === 'fare_rules.txt') this.fareRules = parseGTFSFareRules(data);
      else if (filename === 'fare_transfer_rules.txt')
        this.fareTransferRules = parseGTFSFareTransferRules(data);
      else if (filename === 'feed_info.txt') this.feedInfo = parseGTFSFeedInfos(data);
      else if (filename === 'frequencies.txt') this.frequencies = parseGTFSFrequencies(data);
      else if (filename === 'levels.txt') this.levels = parseGTFSLevels(data);
      else if (filename === 'location_groups.txt')
        this.locationGroups = parseGTFSLocationGroups(data);
      else if (filename === 'location_group_stops.txt')
        this.locationGroupStops = parseGTFSLocationGroupStops(data);
      else if (filename === 'networks.txt') this.networks = parseGTFSNetworks(data);
      else if (filename === 'pathways.txt') this.pathways = parseGTFSPathways(data);
      else if (filename === 'route_networks.txt') this.routeNetworks = parseGTFSRouteNetworks(data);
      else if (filename === 'routes.txt') this.routes = parseGTFSRoutes(data);
      else if (filename === 'shapes.txt') this.shapes = parseGTFSShapes(data);
      else if (filename === 'stop_areas.txt') this.stopAreas = parseGTFSStopAreas(data);
      else if (filename === 'stops.txt') this.stops = parseGTFSStops(data);
      else if (filename === 'stop_times.txt') this.stopTimes = parseGTFSStopTimes(data);
      else if (filename === 'timeframes.txt') this.timeframes = parseGTFSTimeframes(data);
      else if (filename === 'transfers.txt') this.transfers = parseGTFSTransfers(data);
      else if (filename === 'translations.txt') this.translations = parseGTFSTranslations(data);
      else if (filename === 'trips.txt') this.trips = parseGTFSTrips(data);
      else if (filename === 'locations.geojson') {
        // Defines zones where riders can request either pickup or drop off by on-demand services.
        // These zones are represented as GeoJSON polygons.
        this.geojson = new BufferJSONReader<
          Record<string, unknown>,
          MValue,
          GTFSLocationsProperties
        >(data);
      }
    }
  }

  /**
   * TODO: Add proeprties from other files like "color"
   * TODO: All features should be parsed as VectorGeometry
   * Yields all of the shapes
   * @yields an iterator that contains shapes, stops, location data, and routes
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<
    | VectorFeature<Record<string, unknown>, MValue, GTFSShapeProperties, VectorLineStringGeometry>
    | VectorFeature<
        Record<string, unknown>,
        MValue,
        GTFSLocationsProperties,
        VectorMultiPolygonGeometry | VectorPolygonGeometry
      >
    | VectorFeature<undefined, MValue, GTFSStopProperties, VectorPointGeometry>
  > {
    if (this.geojson !== undefined) {
      for await (const feature of this.geojson)
        yield feature as VectorFeature<
          Record<string, unknown>,
          MValue,
          GTFSLocationsProperties,
          VectorMultiPolygonGeometry | VectorPolygonGeometry
        >;
    }
    if (this.shapes !== undefined) {
      for (const shape of Object.values(this.shapes)) yield shape;
    }
    if (this.stops !== undefined) {
      for (const stop of Object.values(this.stops)) {
        const { lon, lat } = stop;
        if (lon !== undefined && lat !== undefined) {
          const stopFeature: VectorFeature<
            undefined,
            MValue,
            GTFSStopProperties,
            VectorPointGeometry
          > = {
            type: 'VectorFeature',
            properties: stop.properties(),
            geometry: { type: 'Point', is3D: false, coordinates: { x: lon, y: lat } },
          };
          yield stopFeature;
        }
      }
    }
  }
}

/**
 * Builds a GTFSScheduleReader from a gzip folder
 * @param gzipData - the gzip folder to parse
 * @returns - a Schedule class
 */
export async function buildGTFSSchedule(gzipData: ArrayBufferLike): Promise<GTFSScheduleReader> {
  const pieces: Piece[] = [];

  for (const item of iterZipFolder(new Uint8Array(gzipData))) {
    const { filename } = item;
    const chunk = new TextDecoder('utf8').decode(await item.read());
    pieces.push({ filename, data: chunk });
  }

  return new GTFSScheduleReader(pieces);
}
