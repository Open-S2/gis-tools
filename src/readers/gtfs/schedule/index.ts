// https://gtfs.org/documentation/schedule/reference/#agencytxt
import { BufferJSONReader, iterItems } from '../../..';

import { type GTFSAgency, parseGTFSAgencies } from './agency';
import { type GTFSArea, parseGTFSAreas } from './areas';
import { type GTFSAttribution, parseGTFSAttributions } from './attributions';
import { type GTFSBookingRule, parseGTFSBookingRules } from './bookingRules';
import { type GTFSCalendar, parseGTFSCalendars } from './calendar';
import { type GTFSCalendarDate, parseGTFSCalendarDates } from './calendarDates';
import { type GTFSFareAttribute, parseGTFSFareAttributes } from './fareAttributes';
import { type GTFSFareLegJoinRule, parseGTFSFareLegJoinRules } from './fareLegJoinRules';
import { type GTFSFareLegRule, parseGTFSFareLegRules } from './fareLegRules';
import { type GTFSFareMedia, parseGTFSFareMedias } from './fareMedia';
import { type GTFSFareProduct, parseGTFSFareProducts } from './fareProducts';
import { type GTFSFareRule, parseGTFSFareRules } from './fareRules';
import { type GTFSFareTransferRule, parseGTFSFareTransferRules } from './fareTransferRules';
import { type GTFSFeedInfo, parseGTFSFeedInfos } from './feedInfo';
import { type GTFSFrequency, parseGTFSFrequencies } from './frequencies';
import { type GTFSLevel, parseGTFSLevels } from './levels';
import { type GTFSLocationGroup, parseGTFSLocationGroups } from './locationGroups';
import { type GTFSLocationGroupStop, parseGTFSLocationGroupStops } from './locationGroupStops';
import { type GTFSNetwork, parseGTFSNetworks } from './networks';
import { type GTFSPathway, parseGTFSPathways } from './pathways';
import { type GTFSRoute, parseGTFSRoutes } from './routes';
import { type GTFSRouteNetwork, parseGTFSRouteNetworks } from './routeNetworks';
import { GTFSShapeProperties, parseGTFSShapes } from './shapes';
import { type GTFSStop, type GTFSStopProperties, parseGTFSStops } from './stops';
import { type GTFSStopArea, parseGTFSStopAreas } from './stopAreas';
import { type GTFSStopTime, parseGTFSStopTimes } from './stopTimes';
import { type GTFSTimeframe, parseGTFSTimeframes } from './timeframes';
import { type GTFSTransfer, parseGTFSTransfers } from './transfers';
import { type GTFSTranslation, parseGTFSTranslations } from './translations';
import { type GTFSTrip, parseGTFSTrips } from './trips';

import type { FeatureIterator } from '../..';
import type {
  MValue,
  Properties,
  VectorFeature,
  VectorLineStringGeometry,
  VectorMultiPolygonGeometry,
  VectorPointGeometry,
  VectorPolygonGeometry,
} from '../../../geometry';

export * from './agency';
export * from './areas';
export * from './attributions';
export * from './bookingRules';
export * from './calendar';
export * from './calendarDates';
export * from './fareAttributes';
export * from './fareLegJoinRules';
export * from './fareLegRules';
export * from './fareMedia';
export * from './fareProducts';
export * from './fareRules';
export * from './fareTransferRules';
export * from './feedInfo';
export * from './frequencies';
export * from './levels';
export * from './locationGroups';
export * from './locationGroupStops';
export * from './networks';
export * from './pathways';
export * from './routeNetworks';
export * from './routes';
export * from './shapes';
export * from './stopAreas';
export * from './stops';
export * from './stopTimes';
export * from './timeframes';
export * from './transfers';
export * from './translations';
export * from './trips';

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
 * import { buildGTFSSchedule } from 'gis-tools';
 *
 * const schedule = await buildGTFSSchedule(gzipData);
 * ```
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

  for (const item of iterItems(new Uint8Array(gzipData))) {
    const { filename } = item;
    const chunk = new TextDecoder('utf8').decode(await item.read());
    pieces.push({ filename, data: chunk });
  }

  return new GTFSScheduleReader(pieces);
}
