import {
  GBFSFreeBikeStatusV1,
  GBFSStationInformationV1,
  GBFSStationStatusV1,
  GBFSSystemAlertsV1,
  GBFSSystemCalendarV1,
  GBFSSystemHoursV1,
  GBFSSystemInformationV1,
  GBFSSystemPricingPlansV1,
  GBFSSystemRegionsV1,
  GBFSV1,
  GBFSVersionsV1,
} from '.';

import type { Feature, FeatureIterator, PointGeometry, Properties } from '../../..';

export * from './freeBikeStatus';
export * from './gbfs';
export * from './gbfsVersions';
export * from './stationInformation';
export * from './stationStatus';
export * from './systemAlerts';
export * from './systemCalendar';
export * from './systemHours';
export * from './systemInformation';
export * from './systemPricingPlans';
export * from './systemRegions';

/** Station Information feature properties */
export interface GBFSStationV1FeaturesV1Properties extends Properties {
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
  capacity?: number;
}

/** Station Information Point Feature */
export type GBFSStationPointFeatureV1 = Feature<
  undefined,
  Properties,
  GBFSStationV1FeaturesV1Properties,
  PointGeometry<Properties>
>;

/** All potential feature types in a GBFS V1 specification */
export type GBFSFeaturesV1 = GBFSStationPointFeatureV1;

/** All potential feature property types in a GBFS V1 specification */
export type GBFSFeaturePropertiesV1 = GBFSStationV1FeaturesV1Properties;

/**
 * GBFS Version 1 Reader
 */
export class GBFSReaderV1
  implements FeatureIterator<undefined, Properties, GBFSFeaturePropertiesV1>
{
  version = 1;
  freeBikeStatus?: GBFSFreeBikeStatusV1;
  gbfs: GBFSV1;
  gbfsVersions?: GBFSVersionsV1;
  stationInformation?: GBFSStationInformationV1;
  stationStatus?: GBFSStationStatusV1;
  systemAlerts?: GBFSSystemAlertsV1;
  systemCalendar?: GBFSSystemCalendarV1;
  systemHours?: GBFSSystemHoursV1;
  systemInformation!: GBFSSystemInformationV1;
  systemPricingPlans?: GBFSSystemPricingPlansV1;
  systemRegions?: GBFSSystemRegionsV1;

  /**
   * @param gbfs - the GBFS schema
   * @param feeds - the feeds for the GBFS
   */
  constructor(gbfs: GBFSV1, feeds?: FeedResV1) {
    this.gbfs = gbfs;
    this.gbfsVersions = feeds?.gbfs_versions;
    this.systemInformation = feeds?.system_information as GBFSSystemInformationV1;
    this.stationInformation = feeds?.station_information;
    this.stationStatus = feeds?.station_status;
    this.freeBikeStatus = feeds?.free_bike_status;
    this.systemHours = feeds?.system_hours;
    this.systemAlerts = feeds?.system_alerts;
    this.systemCalendar = feeds?.system_calendar;
    this.systemPricingPlans = feeds?.system_pricing_plans;
    this.systemRegions = feeds?.system_regions;
  }

  /**
   * Yields all of the shapes
   * @yields an iterator that contains shapes, stops, location data, and routes
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<GBFSFeaturesV1> {
    const { stationInformation } = this;
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
        } = station;
        const stationProperties: GBFSStationV1FeaturesV1Properties = {
          station_id,
          name,
          short_name,
          address,
          cross_street,
          region_id,
          post_code,
          rental_methods,
          capacity,
        };
        const stationPoint: GBFSStationPointFeatureV1 = {
          type: 'Feature',
          properties: stationProperties,
          geometry: {
            type: 'Point',
            coordinates: [lon, lat],
          },
        };
        yield stationPoint;
      }
    }
  }
}

/** Parsing all the available feeds */
export interface FeedResV1 {
  gbfs_versions?: GBFSVersionsV1;
  system_information?: GBFSSystemInformationV1;
  station_information?: GBFSStationInformationV1;
  station_status?: GBFSStationStatusV1;
  free_bike_status?: GBFSFreeBikeStatusV1;
  system_hours?: GBFSSystemHoursV1;
  system_alerts?: GBFSSystemAlertsV1;
  system_calendar?: GBFSSystemCalendarV1;
  system_regions?: GBFSSystemRegionsV1;
  system_pricing_plans?: GBFSSystemPricingPlansV1;
}

/**
 * @param gbfs - the GBFS schema to parse
 * @param locale - the locale to use if provided, otherwise default to en
 * @param path - if provided, will use this path instead of the url (for testing)
 * @returns - the GBFS reader
 */
export async function buildGBFSReaderV1(
  gbfs: GBFSV1,
  locale = 'en',
  path?: string,
): Promise<GBFSReaderV1> {
  const { data } = gbfs;
  const firstLocale = Object.keys(data)[0];
  const { feeds } = data[locale] ?? data[firstLocale];
  const feedData: FeedResV1 = {};
  await Promise.allSettled(
    feeds.map(async (feed) => {
      if (feed.name === 'gbfs') return;
      const url = path !== undefined ? `${path}/${feed.name}.json` : feed.url;
      const json = await fetch(url).then(async (res) => await res.json());
      // @ts-expect-error - We really don't care, we know it categorizes correctly
      feedData[feed.name] = json;
    }),
  );

  return new GBFSReaderV1(gbfs, feedData);
}
