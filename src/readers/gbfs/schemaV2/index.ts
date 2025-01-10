import { toVector } from '../../..';
import {
  GBFSFreeBikeStatusV2,
  GBFSGeofencingZonesV2,
  GBFSGeofencingZonesV2Properties,
  GBFSStationInformationV2,
  GBFSStationStatusV2,
  GBFSSystemAlertsV2,
  GBFSSystemCalendarV2,
  GBFSSystemHoursV2,
  GBFSSystemInformationV2,
  GBFSSystemPricingPlansV2,
  GBFSSystemRegionsV2,
  GBFSV2,
  GBFSVehicleTypesV2,
  GBFSVersionsV2,
} from '.';

import type {
  FeatureIterator,
  Properties,
  VectorFeature,
  VectorMultiPolygonGeometry,
  VectorPointGeometry,
} from '../../..';

export * from './freeBikeStatus';
export * from './gbfs';
export * from './gbfsVersions';
export * from './geofencingZones';
export * from './stationInformation';
export * from './stationStatus';
export * from './systemAlerts';
export * from './systemCalendar';
export * from './systemHours';
export * from './systemInformation';
export * from './systemPricingPlans';
export * from './systemRegions';
export * from './vehicleTypes';

/** Geofencing Feature */
export type GBFSGeofencingFeatureV2 = VectorFeature<
  undefined,
  Properties,
  GBFSGeofencingZonesV2Properties,
  VectorMultiPolygonGeometry<Properties>
>;

/** Station Information feature properties */
export interface GBFSStationV2FeaturesV2Properties extends Properties {
  station_id: string;
  name: string;
  short_name?: string;
  address?: string;
  cross_street?: string;
  region_id?: string;
  post_code?: string;
  rental_methods?: Array<
    | 'key'
    | 'creditcard'
    | 'paypass'
    | 'applepay'
    | 'androidpay'
    | 'transitcard'
    | 'accountnumber'
    | 'phone'
    | 'KEY'
    | 'CREDITCARD'
    | 'PAYPASS'
    | 'APPLEPAY'
    | 'ANDROIDPAY'
    | 'TRANSITCARD'
    | 'ACCOUNTNUMBER'
    | 'PHONE'
  >;
  is_virtual_station?: boolean;
  parking_type?:
    | 'parking_lot'
    | 'street_parking'
    | 'underground_parking'
    | 'sidewalk_parking'
    | 'other';
  parking_hoop?: boolean;
  contact_phone?: string;
  capacity?: number;
  is_valet_station?: boolean;
  is_charging_station?: boolean;
  rental_uris?: {
    android?: string;
    ios?: string;
    web?: string;
  };
}

/** Station Information Point Feature */
export type GBFSStationPointFeatureV2 = VectorFeature<
  undefined,
  Properties,
  GBFSStationV2FeaturesV2Properties,
  VectorPointGeometry<Properties>
>;

/** Station Information Area Feature */
export type GBFSStationAreaFeatureV2 = VectorFeature<
  undefined,
  Properties,
  GBFSStationV2FeaturesV2Properties,
  VectorMultiPolygonGeometry
>;

/** All potential feature types in a GBFS V2 specification */
export type GBFSFeaturesV2 =
  | GBFSGeofencingFeatureV2
  | GBFSStationPointFeatureV2
  | GBFSStationAreaFeatureV2;

/** All potential feature property types in a GBFS V2 specification */
export type GBFSFeaturePropertiesV2 =
  | GBFSGeofencingZonesV2Properties
  | GBFSStationV2FeaturesV2Properties;

/**
 * GBFS Version 1 Reader
 */
export class GBFSReaderV2
  implements FeatureIterator<undefined, Properties, GBFSFeaturePropertiesV2>
{
  version = 2;
  freeBikeStatus?: GBFSFreeBikeStatusV2;
  gbfs: GBFSV2;
  gbfsVersions?: GBFSVersionsV2;
  geofencingZones?: GBFSGeofencingZonesV2;
  stationInformation?: GBFSStationInformationV2;
  stationStatus?: GBFSStationStatusV2;
  systemAlerts?: GBFSSystemAlertsV2;
  systemCalendar?: GBFSSystemCalendarV2;
  systemHours?: GBFSSystemHoursV2;
  systemInformation!: GBFSSystemInformationV2;
  systemPricingPlans?: GBFSSystemPricingPlansV2;
  systemRegions?: GBFSSystemRegionsV2;
  vehicleTypes?: GBFSVehicleTypesV2;

  /**
   * @param gbfs - the GBFS schema
   * @param feeds - the feeds for the GBFS
   */
  constructor(gbfs: GBFSV2, feeds?: FeedResV2) {
    this.gbfs = gbfs;
    this.gbfsVersions = feeds?.gbfs_versions;
    this.geofencingZones = feeds?.geofencing_zones;
    this.systemInformation = feeds?.system_information as GBFSSystemInformationV2;
    this.stationInformation = feeds?.station_information;
    this.stationStatus = feeds?.station_status;
    this.freeBikeStatus = feeds?.free_bike_status;
    this.systemHours = feeds?.system_hours;
    this.systemAlerts = feeds?.system_alerts;
    this.systemCalendar = feeds?.system_calendar;
    this.systemPricingPlans = feeds?.system_pricing_plans;
    this.systemRegions = feeds?.system_regions;
    this.vehicleTypes = feeds?.vehicle_types;
  }

  /**
   * Yields all of the shapes
   * @yields an iterator that contains shapes, stops, location data, and routes
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<GBFSFeaturesV2> {
    const { geofencingZones, stationInformation } = this;
    if (geofencingZones !== undefined) {
      const {
        data: { geofencing_zones },
      } = geofencingZones;
      for (const feature of geofencing_zones.features)
        yield toVector(feature) as GBFSGeofencingFeatureV2;
    }
    if (stationInformation !== undefined) {
      const {
        data: { stations },
      } = stationInformation;
      for (const station of stations) {
        const {
          lat,
          lon,
          station_id,
          name,
          short_name,
          address,
          cross_street,
          region_id,
          post_code,
          rental_methods,
          capacity,
          rental_uris,
        } = station;
        const stationProperties: GBFSStationV2FeaturesV2Properties = {
          station_id,
          name,
          short_name,
          address,
          cross_street,
          region_id,
          post_code,
          rental_methods,
          is_virtual_station: 'is_virtual_station' in station && station.is_virtual_station,
          parking_type: 'parking_type' in station ? station.parking_type : undefined,
          parking_hoop: 'parking_hoop' in station && station.parking_hoop,
          contact_phone: 'contact_phone' in station ? station.contact_phone : undefined,
          capacity,
          is_valet_station: 'is_valet_station' in station && station.is_valet_station,
          is_charging_station: 'is_charging_station' in station && station.is_charging_station,
          rental_uris,
        };
        const stationPoint: GBFSStationPointFeatureV2 = {
          type: 'VectorFeature',
          properties: stationProperties,
          geometry: {
            type: 'Point',
            is3D: false,
            coordinates: { x: lon, y: lat },
          },
        };
        yield stationPoint;
        if ('station_area' in station && station.station_area !== undefined) {
          const stationArea = toVector({
            type: 'Feature',
            properties: stationProperties,
            geometry: station.station_area,
          }) as GBFSStationAreaFeatureV2;
          yield stationArea;
        }
      }
    }
  }
}

/** Parsing all the available feeds */
export interface FeedResV2 {
  gbfs_versions?: GBFSVersionsV2;
  system_information?: GBFSSystemInformationV2;
  vehicle_types?: GBFSVehicleTypesV2;
  station_information?: GBFSStationInformationV2;
  station_status?: GBFSStationStatusV2;
  free_bike_status?: GBFSFreeBikeStatusV2;
  system_hours?: GBFSSystemHoursV2;
  system_alerts?: GBFSSystemAlertsV2;
  system_calendar?: GBFSSystemCalendarV2;
  system_regions?: GBFSSystemRegionsV2;
  system_pricing_plans?: GBFSSystemPricingPlansV2;
  geofencing_zones?: GBFSGeofencingZonesV2;
}

/**
 * @param gbfs - the GBFS schema to parse
 * @param locale - the locale to use if provided, otherwise default to en
 * @param path - if provided, will use this path instead of the url (for testing)
 * @returns - the GBFS reader
 */
export async function buildGBFSReaderV2(
  gbfs: GBFSV2,
  locale = 'en',
  path?: string,
): Promise<GBFSReaderV2> {
  const { data } = gbfs;
  const firstLocale = Object.keys(data)[0];
  const { feeds } = data[locale] ?? data[firstLocale];
  const feedData: FeedResV2 = {};
  await Promise.allSettled(
    feeds.map(async (feed) => {
      if (feed.name === 'gbfs') return;
      const url = path !== undefined ? `${path}/${feed.name}.json` : feed.url;
      const json = await fetch(url).then(async (res) => await res.json());
      // @ts-expect-error - We really don't care, we categorize naturally
      feedData[feed.name] = json;
    }),
  );

  return new GBFSReaderV2(gbfs, feedData);
}
