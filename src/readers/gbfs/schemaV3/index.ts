import { toVector } from '../../..';
import {
  GBFSGeofencingZonesV3,
  GBFSGeofencingZonesV3Properties,
  GBFSManifestV3,
  GBFSStationInformationV3,
  GBFSStationStatusV3,
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
  FeatureIterator,
  Properties,
  VectorFeature,
  VectorMultiPolygonGeometry,
  VectorPointGeometry,
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
export type GBFSGeofencingFeatureV3 = VectorFeature<
  undefined,
  Properties,
  GBFSGeofencingZonesV3Properties,
  VectorMultiPolygonGeometry<Properties>
>;

/** Station Information feature properties */
export interface GBFSStationV3FeaturesV3Properties extends Properties {
  station_id: string;
  name: Array<{
    text: string;
    language: string;
  }>;
  short_name?: Array<{
    text: string;
    language: string;
  }>;
  address?: string;
  cross_street?: string;
  region_id?: string;
  post_code?: string;
  station_opening_hours?: string;
  rental_methods?: Array<
    | 'key'
    | 'creditcard'
    | 'paypass'
    | 'applepay'
    | 'androidpay'
    | 'transitcard'
    | 'accountnumber'
    | 'phone'
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
export type GBFSStationPointFeatureV3 = VectorFeature<
  undefined,
  Properties,
  GBFSStationV3FeaturesV3Properties,
  VectorPointGeometry<Properties>
>;

/** Station Information Area Feature */
export type GBFSStationAreaFeatureV3 = VectorFeature<
  undefined,
  Properties,
  GBFSStationV3FeaturesV3Properties,
  VectorMultiPolygonGeometry
>;

/** Vehicle Point Feature */
export type GBFSVehiclePointFeatureV3 = VectorFeature<
  undefined,
  Properties,
  GBFSVehicleV3,
  VectorPointGeometry
>;

/** Metadata Database Properties */
export interface GBFSDatasetAreaPropertiesV3 extends Properties {
  system_id: string;
  versions?: {
    version: '1.0' | '1.1' | '2.0' | '2.1' | '2.2' | '2.3' | '3.0' | '3.1-RC';
    url: string;
  }[];
  country_code?: string;
}

/** Metadata Database Feature */
export type GBFSDatasetAreaFeatureV3 = VectorFeature<
  undefined,
  Properties,
  GBFSDatasetAreaPropertiesV3,
  VectorMultiPolygonGeometry
>;

/** All potential feature types in a GBFS V3 specification */
export type GBFSFeaturesV3 =
  | GBFSGeofencingFeatureV3
  | GBFSStationPointFeatureV3
  | GBFSVehiclePointFeatureV3
  | GBFSDatasetAreaFeatureV3;

/** All potential feature property types in a GBFS V3 specification */
export type GBFSFeaturePropertiesV3 =
  | GBFSGeofencingZonesV3Properties
  | GBFSStationV3FeaturesV3Properties
  | GBFSVehicleV3
  | GBFSDatasetAreaPropertiesV3;

/**
 * GBFS Version 3 Reader
 */
export class GBFSReaderV3
  implements FeatureIterator<undefined, Properties, GBFSFeaturePropertiesV3>
{
  version = 3;
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
  manifest?: GBFSManifestV3;

  /**
   * @param gbfs - the GBFS schema
   * @param feeds - the feeds for the GBFS
   */
  constructor(gbfs: GBFSV3, feeds?: FeedResV3) {
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
    this.manifest = feeds?.manifest;
  }

  /**
   * Yields all of the shapes
   * @yields an iterator that contains shapes, stops, location data, and routes
   */
  async *[Symbol.asyncIterator](): AsyncGenerator<GBFSFeaturesV3> {
    const { geofencingZones, stationInformation, vehicleStatus, manifest } = this;
    if (geofencingZones !== undefined) {
      const {
        data: { geofencing_zones },
      } = geofencingZones;
      for (const feature of geofencing_zones.features)
        yield toVector(feature) as GBFSGeofencingFeatureV3;
    }
    if (stationInformation !== undefined) {
      const {
        data: { stations },
      } = stationInformation;
      for (const station of stations) {
        const {
          lat,
          lon,
          station_area,
          station_id,
          name,
          short_name,
          address,
          cross_street,
          region_id,
          post_code,
          station_opening_hours,
          rental_methods,
          is_virtual_station,
          parking_type,
          parking_hoop,
          contact_phone,
          capacity,
          is_valet_station,
          is_charging_station,
          rental_uris,
        } = station;
        const stationProperties = {
          station_id,
          name,
          short_name,
          address,
          cross_street,
          region_id,
          post_code,
          station_opening_hours,
          rental_methods,
          is_virtual_station,
          parking_type,
          parking_hoop,
          contact_phone,
          capacity,
          is_valet_station,
          is_charging_station,
          rental_uris,
        };
        const stationPoint: GBFSStationPointFeatureV3 = {
          type: 'VectorFeature',
          properties: stationProperties,
          geometry: {
            type: 'Point',
            is3D: false,
            coordinates: { x: lon, y: lat },
          },
        };
        yield stationPoint;
        if (station_area !== undefined) {
          const stationArea = toVector({
            type: 'Feature',
            properties: stationProperties,
            geometry: station_area,
          }) as GBFSStationAreaFeatureV3;
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
        const vehiclePoint: GBFSVehiclePointFeatureV3 = {
          type: 'VectorFeature',
          properties: { ...vehicle },
          geometry: {
            type: 'Point',
            is3D: false,
            coordinates: { x: lon, y: lat },
          },
        };
        yield vehiclePoint;
      }
    }
    if (manifest !== undefined) {
      const datasets = manifest.data.datasets;
      for (const dataset of datasets) {
        if ('area' in dataset && dataset.area !== undefined) {
          const { system_id, versions, area, country_code } = dataset;
          const areaFeature = toVector({
            type: 'Feature',
            properties: { system_id, versions, country_code },
            geometry: area,
          }) as GBFSDatasetAreaFeatureV3;
          yield areaFeature;
        }
      }
    }
  }
}

/** Parsing all the available feeds */
export interface FeedResV3 {
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
  manifest?: GBFSManifestV3;
}

/**
 * @param gbfs - the GBFS schema to parse
 * @param path - if provided, will use this path instead of the url (for testing)
 * @returns - the GBFS reader
 */
export async function buildGBFSReaderV3(gbfs: GBFSV3, path?: string): Promise<GBFSReaderV3> {
  const {
    data: { feeds },
  } = gbfs;
  const feedData: FeedResV3 = {};
  await Promise.allSettled(
    feeds.map(async (feed) => {
      if (feed.name === 'gbfs') return;
      const url = path !== undefined ? `${path}/${feed.name}.json` : feed.url;
      const json = await fetch(url).then(async (res) => await res.json());
      // @ts-expect-error - We really don't care, we know it categorizes correctly
      feedData[feed.name] = json;
    }),
  );

  // If there is a manifest.json, lets get it
  if (feedData.system_information?.data.manifest_url !== undefined) {
    const manifestURL =
      path !== undefined ? `${path}/manifest.json` : feedData.system_information.data.manifest_url;
    feedData.manifest = await fetch(manifestURL).then(
      async (res) => (await res.json()) as GBFSManifestV3,
    );
  }

  return new GBFSReaderV3(gbfs, feedData);
}
