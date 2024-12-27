import {
  GBFSGeofencingZonesV3,
  GBFSGeofencingZonesV3Properties,
  GBFSStationInformationV3,
  GBFSStationStatusV3,
  GBFSStationV3,
  GBFSSystemAlertsV3,
  GBFSSystemInformationV3,
  GBFSSystemPricingPlansV3,
  GBFSSystemRegionsV3,
  GBFSV3,
  GBFSVehicleStatusV3,
  GBFSVehicleTypesV3,
  GBFSVehicleV3,
  GBFSVersionsV3,
} from '.';

import type {
  Feature,
  FeatureIterator,
  MultiPolygonGeometry,
  PointGeometry,
  Properties,
} from '../../..';

export * from './gbfs';
export * from './gbfsVersions';
export * from './geofencingZones';
export * from './manifest';
export * from './stationInformation';
export * from './stationStatus';
export * from './systemAlerts';
export * from './systemInformation';
export * from './systemPricingPlans';
export * from './systemRegions';
export * from './vehicleStatus';
export * from './vehicleTypes';

/** Geofencing Feature */
export type GeofencingFeature = Feature<
  undefined,
  Properties,
  GBFSGeofencingZonesV3Properties,
  MultiPolygonGeometry<Properties>
>;

/** StationPoint Feature */
export type StationPointFeature = Feature<
  undefined,
  Properties,
  GBFSStationV3,
  PointGeometry<Properties>
>;

/** StationArea Feature */
export type StationAreaFeature = Feature<
  undefined,
  Properties,
  GBFSStationV3,
  MultiPolygonGeometry
>;

/** StationPoint Feature */
export type VehiclePointFeature = Feature<
  undefined,
  Properties,
  GBFSVehicleV3,
  PointGeometry<Properties>
>;

/**
 * GBFS Version 3 Reader
 */
export class GBFSReaderV3 implements FeatureIterator {
  gbfs: GBFSV3;
  gbfsVersions?: GBFSVersionsV3;
  systemInformation!: GBFSSystemInformationV3;
  stationInformation?: GBFSStationInformationV3;
  stationStatus?: GBFSStationStatusV3;
  vehicleStatus?: GBFSVehicleStatusV3;
  systemAlerts?: GBFSSystemAlertsV3;
  systemRegions?: GBFSSystemRegionsV3;
  systemPricingPlans?: GBFSSystemPricingPlansV3;
  geofencingZones?: GBFSGeofencingZonesV3;

  /**
   * @param gbfs - the GBFS schema
   * @param feeds - the feeds for the GBFS
   */
  constructor(gbfs: GBFSV3, feeds?: FeedRes) {
    this.gbfs = gbfs;
    this.gbfsVersions = feeds?.gbfs_versions;
    this.systemInformation = feeds?.system_information as GBFSSystemInformationV3;
    this.stationInformation = feeds?.station_information;
    this.stationStatus = feeds?.station_status;
    this.vehicleStatus = feeds?.vehicle_status;
    this.systemAlerts = feeds?.system_alerts;
    this.systemRegions = feeds?.system_regions;
    this.systemPricingPlans = feeds?.system_pricing_plans;
    this.geofencingZones = feeds?.geofencing_zones;
  }

  /**
   * Yields all of the shapes
   * @yields an iterator that contains shapes, stops, location data, and routes
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<
    GeofencingFeature | StationPointFeature | VehiclePointFeature
  > {
    const { geofencingZones, stationInformation, vehicleStatus } = this;
    if (geofencingZones !== undefined) {
      const {
        data: { geofencing_zones },
      } = geofencingZones;
      for (const feature of geofencing_zones.features) yield feature as GeofencingFeature;
    }
    if (stationInformation !== undefined) {
      const {
        data: { stations },
      } = stationInformation;
      for (const station of stations) {
        const { lat, lon, station_area } = station;
        const stationPoint: StationPointFeature = {
          type: 'Feature',
          properties: { ...station },
          geometry: {
            type: 'Point',
            coordinates: [lon, lat],
          },
        };
        yield stationPoint;
        if (station_area !== undefined) {
          const stationArea: StationAreaFeature = {
            type: 'Feature',
            properties: { ...station },
            geometry: {
              type: 'MultiPolygon',
              // @ts-expect-error - bug, ignore for now
              coordinates: station_area,
            },
          };
          yield stationArea;
        }
      }
    }
    if (vehicleStatus !== undefined) {
      const {
        data: { vehicles },
      } = vehicleStatus;
      for (const vehicle of vehicles) {
        const { lat, lon } = vehicle;
        if (lat === undefined || lon === undefined) continue;
        const vehiclePoint: VehiclePointFeature = {
          type: 'Feature',
          properties: { ...vehicle },
          geometry: {
            type: 'Point',
            coordinates: [lon, lat],
          },
        };
        yield vehiclePoint;
      }
    }
  }
}

/**
 * Parsing all the available feeds
 */
interface FeedRes {
  gbfs_versions?: GBFSVersionsV3;
  system_information?: GBFSSystemInformationV3;
  vehicle_types?: GBFSVehicleTypesV3;
  station_information?: GBFSStationInformationV3;
  station_status?: GBFSStationStatusV3;
  vehicle_status?: GBFSVehicleStatusV3;
  system_alerts?: GBFSSystemAlertsV3;
  system_regions?: GBFSSystemRegionsV3;
  system_pricing_plans?: GBFSSystemPricingPlansV3;
  geofencing_zones?: GBFSGeofencingZonesV3;
}

/**
 * @param gbfs - the GBFS schema to parse
 * @returns - the GBFS reader
 */
export async function buildGBFSReaderV3(gbfs: GBFSV3): Promise<GBFSReaderV3> {
  const {
    data: { feeds },
  } = gbfs;
  const feedData: FeedRes = {};
  await Promise.allSettled(
    feeds.map(async (feed) => {
      if (feed.name === 'gbfs') return;
      const json = await fetch(feed.url).then(async (res) => await res.json());
      // @ts-expect-error - We really don't care, we know it categorizes correctly
      feedData[feed.name] = json;
    }),
  );

  return new GBFSReaderV3(gbfs, feedData);
}
