import { Properties } from '../../../';

/**
 * # GBFS Vehicle Status Schema V3.1-RC & V3.0
 * Describes the vehicles that are available for rent (as of v3.0, formerly free_bike_status).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#vehicle_statusjson)
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#vehicle_statusjson)
 */
export type GBFSVehicleStatusV3 = GBFSVehicleStatusV31RC | GBFSVehicleStatusV30;

/**
 * # GBFS Vehicle Status Schema V3.1-RC
 * Describes the vehicles that are available for rent (as of v3.0, formerly free_bike_status).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#vehicle_statusjson)
 */
export const gbfsVehicleStatusSchemaV31RC = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#vehicle_statusjson',
  description:
    'Describes the vehicles that are available for rent (as of v3.0, formerly free_bike_status).',
  type: 'object',
  properties: {
    last_updated: {
      description: 'Last time the data in the feed was updated in RFC3339 format.',
      type: 'string',
      format: 'date-time',
    },
    ttl: {
      description:
        'Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).',
      type: 'integer',
      minimum: 0,
    },
    version: {
      description:
        'GBFS version number to which the feed conforms, according to the versioning framework (added in v1.1).',
      type: 'string',
      const: '3.1-RC',
    },
    data: {
      description: 'Array that contains one object per vehicle as defined below.',
      type: 'object',
      properties: {
        vehicles: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              vehicle_id: {
                description: 'Rotating (as of v2.0) identifier of a vehicle.',
                type: 'string',
              },
              lat: {
                description: 'The latitude of the vehicle.',
                type: 'number',
                minimum: -90,
                maximum: 90,
              },
              lon: {
                description: 'The longitude of the vehicle.',
                type: 'number',
                minimum: -180,
                maximum: 180,
              },
              is_reserved: {
                description: 'Is the vehicle currently reserved?',
                type: 'boolean',
              },
              is_disabled: {
                description: 'Is the vehicle currently disabled (broken)?',
                type: 'boolean',
              },
              rental_uris: {
                description:
                  'Contains rental uris for Android, iOS, and web in the android, ios, and web fields (added in v1.1).',
                type: 'object',
                properties: {
                  android: {
                    description:
                      'URI that can be passed to an Android app with an intent (added in v1.1).',
                    type: 'string',
                    format: 'uri',
                  },
                  ios: {
                    description:
                      'URI that can be used on iOS to launch the rental app for this vehicle (added in v1.1).',
                    type: 'string',
                    format: 'uri',
                  },
                  web: {
                    description:
                      'URL that can be used by a web browser to show more information about renting this vehicle (added in v1.1).',
                    type: 'string',
                    format: 'uri',
                  },
                },
              },
              vehicle_type_id: {
                description: 'The vehicle_type_id of this vehicle (added in v2.1-RC).',
                type: 'string',
              },
              last_reported: {
                description:
                  "The last time this vehicle reported its status to the operator's backend in RFC3339 format (added in v2.1-RC).",
                type: 'string',
                format: 'date-time',
              },
              current_range_meters: {
                description:
                  "The furthest distance in meters that the vehicle can travel without recharging or refueling with the vehicle's current charge or fuel (added in v2.1-RC).",
                type: 'number',
                minimum: 0,
              },
              current_fuel_percent: {
                description:
                  'This value represents the current percentage, expressed from 0 to 1, of fuel or battery power remaining in the vehicle. Added in v2.3-RC.',
                type: 'number',
                minimum: 0,
                maximum: 1,
              },
              station_id: {
                description:
                  'Identifier referencing the station_id if the vehicle is currently at a station (added in v2.1-RC2).',
                type: 'string',
              },
              home_station_id: {
                description:
                  'The station_id of the station this vehicle must be returned to (added in v2.3-RC).',
                type: 'string',
              },
              pricing_plan_id: {
                description:
                  'The plan_id of the pricing plan this vehicle is eligible for (added in v2.2).',
                type: 'string',
              },
              vehicle_equipment: {
                description:
                  'List of vehicle equipment provided by the operator in addition to the accessories already provided in the vehicle. Added in v2.3.',
                type: 'array',
                items: {
                  enum: [
                    'child_seat_a',
                    'child_seat_b',
                    'child_seat_c',
                    'winter_tires',
                    'snow_chains',
                  ],
                },
              },
              available_until: {
                description:
                  'The date and time when any rental of the vehicle must be completed. Added in v2.3.',
                type: 'string',
                pattern:
                  '^([0-9]{4})-([0-9]{2})-([0-9]{2})T([0-9]{2}):([0-9]{2}):([0-9]{2})(([+-]([0-9]{2}):([0-9]{2}))|Z)$',
              },
            },
            anyOf: [
              {
                required: ['lat', 'lon'],
                errorMessage: "Both 'lat' and 'lon' are required.",
              },
              {
                required: ['station_id'],
                properties: {
                  lat: {
                    not: {},
                  },
                  lon: {
                    not: {},
                  },
                },
                errorMessage: "'station_id' is required if 'lat' and 'lon' are not present.",
              },
            ],
            required: ['vehicle_id', 'is_reserved', 'is_disabled'],
          },
        },
      },
      required: ['vehicles'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * GBFS Vehicle V3
 */
export interface GBFSVehicleV3 extends Properties {
  /**
   * Rotating (as of v2.0) identifier of a vehicle.
   */
  vehicle_id: string;

  /**
   * The latitude of the vehicle.
   * **Range**: [-90, 90]
   */
  // @ts-expect-error - This is ok for now
  lat?: number;

  /**
   * The longitude of the vehicle.
   * **Range**: [-180, 180]
   */
  // @ts-expect-error - This is ok for now
  lon?: number;

  /**
   * Is the vehicle currently reserved?
   */
  is_reserved: boolean;

  /**
   * Is the vehicle currently disabled (broken)?
   */
  is_disabled: boolean;

  /**
   * Contains rental URIs for Android, iOS, and web.
   */
  // @ts-expect-error - This is ok for now
  rental_uris?: {
    android?: string; // **Format**: uri
    ios?: string; // **Format**: uri
    web?: string; // **Format**: uri
  };

  /**
   * The vehicle_type_id of this vehicle (added in v2.1-RC).
   */
  // @ts-expect-error - This is ok for now
  vehicle_type_id?: string;

  /**
   * The last time this vehicle reported its status to the operator's backend.
   * **Format**: date-time
   */
  // @ts-expect-error - This is ok for now
  last_reported?: string;

  /**
   * The furthest distance in meters the vehicle can travel without recharging or refueling.
   * **Minimum**: 0
   */
  // @ts-expect-error - This is ok for now
  current_range_meters?: number;

  /**
   * Current percentage of fuel or battery power remaining in the vehicle.
   * **Range**: [0, 1]
   */
  // @ts-expect-error - This is ok for now
  current_fuel_percent?: number;

  /**
   * Identifier referencing the station_id if the vehicle is currently at a station.
   */
  // @ts-expect-error - This is ok for now
  station_id?: string;

  /**
   * The station_id of the station this vehicle must be returned to.
   */
  // @ts-expect-error - This is ok for now
  home_station_id?: string;

  /**
   * The plan_id of the pricing plan this vehicle is eligible for.
   */
  // @ts-expect-error - This is ok for now
  pricing_plan_id?: string;

  /**
   * List of vehicle equipment provided by the operator.
   * **Enum**: ['child_seat_a', 'child_seat_b', 'child_seat_c', 'winter_tires', 'snow_chains']
   */
  // @ts-expect-error - This is ok for now
  vehicle_equipment?: Array<
    'child_seat_a' | 'child_seat_b' | 'child_seat_c' | 'winter_tires' | 'snow_chains'
  >;

  /**
   * The date and time when any rental of the vehicle must be completed.
   * **Pattern**: ^([0-9]{4})-([0-9]{2})-([0-9]{2})T([0-9]{2}):([0-9]{2}):([0-9]{2})(([+-]([0-9]{2}):([0-9]{2}))|Z)$
   */
  // @ts-expect-error - This is ok for now
  available_until?: string;
}

/**
 * # GBFS Vehicle Status Schema V3.1-RC
 * Describes the vehicles that are available for rent (as of v3.0, formerly free_bike_status).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#vehicle_statusjson)
 */
export interface GBFSVehicleStatusV31RC {
  /**
   * Last time the data in the feed was updated in RFC3339 format.
   * **Format**: date-time
   */
  last_updated: string;

  /**
   * Number of seconds before the data in the feed will be updated again
   * (0 if the data should always be refreshed).
   * **Minimum**: 0
   */
  ttl: number;

  /**
   * GBFS version number to which the feed conforms, according to the versioning framework (added in v1.1).
   * **Const**: 3.1-RC
   */
  version: '3.1-RC';

  /**
   * Contains the vehicle data.
   */
  data: {
    /**
     * Array of vehicles available for rent.
     */
    vehicles: GBFSVehicleV3[];
  };
}

/**
 * # GBFS Vehicle Status Schema V3.0
 * Describes the vehicles that are available for rent (as of v3.0, formerly free_bike_status).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#vehicle_statusjson)
 */
export const gbfsVehicleStatusSchemaV30 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#vehicle_statusjson',
  description:
    'Describes the vehicles that are available for rent (as of v3.0, formerly free_bike_status).',
  type: 'object',
  properties: {
    last_updated: {
      description: 'Last time the data in the feed was updated in RFC3339 format.',
      type: 'string',
      format: 'date-time',
    },
    ttl: {
      description:
        'Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).',
      type: 'integer',
      minimum: 0,
    },
    version: {
      description:
        'GBFS version number to which the feed conforms, according to the versioning framework (added in v1.1).',
      type: 'string',
      const: '3.0',
    },
    data: {
      description: 'Array that contains one object per vehicle as defined below.',
      type: 'object',
      properties: {
        vehicles: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              vehicle_id: {
                description: 'Rotating (as of v2.0) identifier of a vehicle.',
                type: 'string',
              },
              lat: {
                description: 'The latitude of the vehicle.',
                type: 'number',
                minimum: -90,
                maximum: 90,
              },
              lon: {
                description: 'The longitude of the vehicle.',
                type: 'number',
                minimum: -180,
                maximum: 180,
              },
              is_reserved: {
                description: 'Is the vehicle currently reserved?',
                type: 'boolean',
              },
              is_disabled: {
                description: 'Is the vehicle currently disabled (broken)?',
                type: 'boolean',
              },
              rental_uris: {
                description:
                  'Contains rental uris for Android, iOS, and web in the android, ios, and web fields (added in v1.1).',
                type: 'object',
                properties: {
                  android: {
                    description:
                      'URI that can be passed to an Android app with an intent (added in v1.1).',
                    type: 'string',
                    format: 'uri',
                  },
                  ios: {
                    description:
                      'URI that can be used on iOS to launch the rental app for this vehicle (added in v1.1).',
                    type: 'string',
                    format: 'uri',
                  },
                  web: {
                    description:
                      'URL that can be used by a web browser to show more information about renting this vehicle (added in v1.1).',
                    type: 'string',
                    format: 'uri',
                  },
                },
              },
              vehicle_type_id: {
                description: 'The vehicle_type_id of this vehicle (added in v2.1-RC).',
                type: 'string',
              },
              last_reported: {
                description:
                  "The last time this vehicle reported its status to the operator's backend in RFC3339 format (added in v2.1-RC).",
                type: 'string',
                format: 'date-time',
              },
              current_range_meters: {
                description:
                  "The furthest distance in meters that the vehicle can travel without recharging or refueling with the vehicle's current charge or fuel (added in v2.1-RC).",
                type: 'number',
                minimum: 0,
              },
              current_fuel_percent: {
                description:
                  'This value represents the current percentage, expressed from 0 to 1, of fuel or battery power remaining in the vehicle. Added in v2.3-RC.',
                type: 'number',
                minimum: 0,
                maximum: 1,
              },
              station_id: {
                description:
                  'Identifier referencing the station_id if the vehicle is currently at a station (added in v2.1-RC2).',
                type: 'string',
              },
              home_station_id: {
                description:
                  'The station_id of the station this vehicle must be returned to (added in v2.3-RC).',
                type: 'string',
              },
              pricing_plan_id: {
                description:
                  'The plan_id of the pricing plan this vehicle is eligible for (added in v2.2).',
                type: 'string',
              },
              vehicle_equipment: {
                description:
                  'List of vehicle equipment provided by the operator in addition to the accessories already provided in the vehicle. Added in v2.3.',
                type: 'array',
                items: {
                  enum: [
                    'child_seat_a',
                    'child_seat_b',
                    'child_seat_c',
                    'winter_tires',
                    'snow_chains',
                  ],
                },
              },
              available_until: {
                description:
                  'The date and time when any rental of the vehicle must be completed. Added in v2.3.',
                type: 'string',
                pattern:
                  '^([0-9]{4})-([0-9]{2})-([0-9]{2})T([0-9]{2}):([0-9]{2}):([0-9]{2})(([+-]([0-9]{2}):([0-9]{2}))|Z)$',
              },
            },
            anyOf: [
              {
                required: ['lat', 'lon'],
                errorMessage: "Both 'lat' and 'lon' are required.",
              },
              {
                required: ['station_id'],
                properties: {
                  lat: {
                    not: {},
                  },
                  lon: {
                    not: {},
                  },
                },
                errorMessage: "'station_id' is required if 'lat' and 'lon' are not present.",
              },
            ],
            required: ['vehicle_id', 'is_reserved', 'is_disabled'],
          },
        },
      },
      required: ['vehicles'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS Vehicle Status Schema V3.0
 * Describes the vehicles that are available for rent (as of v3.0, formerly free_bike_status).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#vehicle_statusjson)
 */
export interface GBFSVehicleStatusV30 {
  /**
   * Last time the data in the feed was updated in RFC3339 format.
   */
  last_updated: string;

  /**
   * Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
   */
  ttl: number;

  /**
   * GBFS version number to which the feed conforms.
   */
  version: '3.0';

  /**
   * Vehicle data containing information on available vehicles for rent.
   */
  data: {
    vehicles: GBFSVehicleV3[];
  };
}
