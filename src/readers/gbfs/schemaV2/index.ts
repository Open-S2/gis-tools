import {
  GBFSGeofencingZonesV2,
  GBFSStationInformationV2,
  GBFSStationStatusV2,
  GBFSSystemAlertsV2,
  GBFSSystemInformationV2,
  GBFSSystemPricingPlansV2,
  GBFSSystemRegionsV2,
  GBFSV2,
  GBFSVehicleStatusV2,
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
 * GBFS Version 2 Reader
 */
export class GBFSReaderV2 {
  gbfs: GBFSV2;
  gbfsVersions?: GBFSVersionsV2;
  systemInformation!: GBFSSystemInformationV2;
  stationInformation?: GBFSStationInformationV2;
  stationStatus?: GBFSStationStatusV2;
  vehicleStatus?: GBFSVehicleStatusV2;
  systemAlerts?: GBFSSystemAlertsV2;
  systemRegions?: GBFSSystemRegionsV2;
  systemPricingPlans?: GBFSSystemPricingPlansV2;
  geofencingZones?: GBFSGeofencingZonesV2;

  /**
   * @param gbfs - the GBFS schema
   * @param feeds - the feeds for the GBFS
   */
  constructor(gbfs: GBFSV2, feeds?: FeedRes) {
    this.gbfs = gbfs;
    this.gbfsVersions = feeds?.gbfs_versions;
    this.systemInformation = feeds?.system_information as GBFSSystemInformationV2;
    this.stationInformation = feeds?.station_information;
    this.stationStatus = feeds?.station_status;
    this.vehicleStatus = feeds?.vehicle_status;
    this.systemAlerts = feeds?.system_alerts;
    this.systemRegions = feeds?.system_regions;
    this.systemPricingPlans = feeds?.system_pricing_plans;
    this.geofencingZones = feeds?.geofencing_zones;
  }
}

/**
 * Parsing all the available feeds
 */
interface FeedRes {
  gbfs_versions?: GBFSVersionsV2;
  system_information?: GBFSSystemInformationV2;
  vehicle_types?: GBFSVehicleTypesV2;
  station_information?: GBFSStationInformationV2;
  station_status?: GBFSStationStatusV2;
  vehicle_status?: GBFSVehicleStatusV2;
  system_alerts?: GBFSSystemAlertsV2;
  system_regions?: GBFSSystemRegionsV2;
  system_pricing_plans?: GBFSSystemPricingPlansV2;
  geofencing_zones?: GBFSGeofencingZonesV2;
}

/**
 * @param gbfs - the GBFS schema to parse
 * @returns - the GBFS reader
 */
export async function buildGBFSReaderV2(gbfs: GBFSV2): Promise<GBFSReaderV2> {
  const {
    data: { feeds },
  } = gbfs;
  const feedData: FeedRes = {};
  await Promise.allSettled(
    feeds.map(async (feed) => {
      if (feed.name === 'gbfs') return;
      const json = await fetch(feed.url).then(async (res) => await res.json());
      // @ts-expect-error - We really don't care, we categorize naturally
      feedData[feed.name] = json as keyof typeof feedData;
    }),
  );

  return new GBFSReaderV2(gbfs, feedData);
}
