/**
 * # GBFS Free Bike Status V2
 * Describes the vehicles that are available for rent (as of v2.1-RC2)
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#free_bike_statusjson)
 */
export const gbfsFreeBikeStatusSchemaV2 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#free_bike_statusjson',
  description: 'Describes the vehicles that are available for rent (as of v2.1-RC2).',
  type: 'object',
  properties: {
    last_updated: {
      description: 'Last time the data in the feed was updated in POSIX time.',
      type: 'integer',
      minimum: 1450155600,
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
      const: '2.3',
    },
    data: {
      description: 'Array that contains one object per bike as defined below.',
      type: 'object',
      properties: {
        bikes: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              bike_id: {
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
                  "The last time this vehicle reported its status to the operator's backend in POSIX time (added in v2.1-RC).",
                type: 'number',
                minimum: 1450155600,
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
            required: ['bike_id', 'is_reserved', 'is_disabled'],
          },
        },
      },
      required: ['bikes'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS Free Bike Status V2
 * Describes the vehicles that are available for rent (as of v2.1-RC2).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#free_bike_statusjson)
 */
export interface GBFSFreeBikeStatusV2 {
  /**
   * Last time the data in the feed was updated in POSIX time.
   * **Minimum**: 1450155600
   */
  last_updated: number;

  /**
   * Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
   * **Minimum**: 0
   */
  ttl: number;

  /**
   * GBFS version number to which the feed conforms, according to the versioning framework (added in v1.1).
   * **Const**: 2.3
   */
  version: '2.3';

  /**
   * Contains the bike data.
   */
  data: {
    /**
     * Array of bikes available for rent.
     */
    bikes: Array<{
      /**
       * Rotating (as of v2.0) identifier of a vehicle.
       */
      bike_id: string;

      /**
       * The latitude of the vehicle.
       * **Minimum**: -90
       * **Maximum**: 90
       */
      lat?: number;

      /**
       * The longitude of the vehicle.
       * **Minimum**: -180
       * **Maximum**: 180
       */
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
      rental_uris?: {
        /**
         * URI that can be passed to an Android app with an intent (added in v1.1).
         * **Format**: uri
         */
        android?: string;

        /**
         * URI that can be used on iOS to launch the rental app for this vehicle (added in v1.1).
         * **Format**: uri
         */
        ios?: string;

        /**
         * URL that can be used by a web browser to show more information about renting this vehicle (added in v1.1).
         * **Format**: uri
         */
        web?: string;
      };

      /**
       * The vehicle_type_id of this vehicle (added in v2.1-RC).
       */
      vehicle_type_id?: string;

      /**
       * The last time this vehicle reported its status to the operator's backend in POSIX time (added in v2.1-RC).
       * **Minimum**: 1450155600
       */
      last_reported?: number;

      /**
       * The furthest distance in meters that the vehicle can travel without recharging or refueling with the vehicle's current charge or fuel (added in v2.1-RC).
       * **Minimum**: 0
       */
      current_range_meters?: number;

      /**
       * The current percentage of fuel or battery power remaining in the vehicle (added in v2.3-RC).
       * **Minimum**: 0
       * **Maximum**: 1
       */
      current_fuel_percent?: number;

      /**
       * Identifier referencing the station_id if the vehicle is currently at a station (added in v2.1-RC2).
       */
      station_id?: string;

      /**
       * The station_id of the station this vehicle must be returned to (added in v2.3-RC).
       */
      home_station_id?: string;

      /**
       * The plan_id of the pricing plan this vehicle is eligible for (added in v2.2).
       */
      pricing_plan_id?: string;

      /**
       * List of vehicle equipment provided by the operator in addition to the accessories already provided in the vehicle (added in v2.3).
       * **Enum**: ['child_seat_a', 'child_seat_b', 'child_seat_c', 'winter_tires', 'snow_chains']
       */
      vehicle_equipment?: Array<
        'child_seat_a' | 'child_seat_b' | 'child_seat_c' | 'winter_tires' | 'snow_chains'
      >;

      /**
       * The date and time when any rental of the vehicle must be completed (added in v2.3).
       * **Pattern**: ^([0-9]{4})-([0-9]{2})-([0-9]{2})T([0-9]{2}):([0-9]{2}):([0-9]{2})(([+-]([0-9]{2}):([0-9]{2}))|Z)$
       */
      available_until?: string;
    }>;
  };
}
