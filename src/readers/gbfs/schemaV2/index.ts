import {
  GBFSFreeBikeStatusV2,
  GBFSGeofencingZonesV2,
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

/**
 * GBFS Version 1 Reader
 */
export class GBFSReaderV2 {
  freeBikeStatus?: GBFSFreeBikeStatusV2;
  gbfs: GBFSV2;
  versions?: GBFSVersionsV2;
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
    this.versions = feeds?.gbfs_versions;
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
