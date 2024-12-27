/**
 * # GBFS Station Status Schema V3.1-RC & V3.0
 * Describes the capacity and rental availability of the station.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#station_statusjson)
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#station_statusjson)
 */
export type GBFSStationStatusV3 = GBFSStationStatusV31RC | GBFSStationStatusV30;

/**
 * # GBFS Station Status Schema V3.1-RC
 * Describes the capacity and rental availability of the station
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#station_statusjson)
 */
export const gbfsStationStatusSchemaV31RC = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#station_statusjson',
  description: 'Describes the capacity and rental availability of the station',
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
      description: 'Array that contains one object per station as defined below.',
      type: 'object',
      properties: {
        stations: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              station_id: {
                description: 'Identifier of a station.',
                type: 'string',
              },
              num_vehicles_available: {
                description:
                  'Number of vehicles of any type physically available for rental at the station.',
                type: 'integer',
                minimum: 0,
              },
              vehicle_types_available: {
                description:
                  'Array of objects displaying the total number of each vehicle type at the station (added in v2.1-RC).',
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    vehicle_type_id: {
                      description:
                        'The vehicle_type_id of vehicle at the station (added in v2.1-RC).',
                      type: 'string',
                    },
                    count: {
                      description:
                        'A number representing the total amount of this vehicle type at the station (added in v2.1-RC).',
                      type: 'integer',
                      minimum: 0,
                    },
                  },
                  required: ['vehicle_type_id', 'count'],
                },
              },
              num_vehicles_disabled: {
                description: 'Number of disabled vehicles of any type at the station.',
                type: 'integer',
                minimum: 0,
              },
              num_docks_available: {
                description: 'Number of functional docks physically at the station.',
                type: 'integer',
                minimum: 0,
              },
              num_docks_disabled: {
                description: 'Number of empty but disabled docks at the station.',
                type: 'integer',
                minimum: 0,
              },
              is_installed: {
                description: 'Is the station currently on the street?',
                type: 'boolean',
              },
              is_renting: {
                description: 'Is the station currently renting vehicles?',
                type: 'boolean',
              },
              is_returning: {
                description: 'Is the station accepting vehicle returns?',
                type: 'boolean',
              },
              last_reported: {
                description:
                  "The last time this station reported its status to the operator's backend in RFC3339 format.",
                type: 'string',
                format: 'date-time',
              },
              vehicle_docks_available: {
                description:
                  'Object displaying available docks by vehicle type (added in v2.1-RC).',
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    vehicle_type_ids: {
                      description:
                        'An array of strings where each string represents a vehicle_type_id that is able to use a particular type of dock at the station (added in v2.1-RC).',
                      type: 'array',
                      items: {
                        type: 'string',
                      },
                    },
                    count: {
                      description:
                        'A number representing the total number of available docks for the defined vehicle type (added in v2.1-RC).',
                      type: 'integer',
                      minimum: 0,
                    },
                  },
                  required: ['vehicle_type_ids', 'count'],
                },
              },
            },
            required: [
              'station_id',
              'num_vehicles_available',
              'is_installed',
              'is_renting',
              'is_returning',
              'last_reported',
            ],
          },
        },
      },
      required: ['stations'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS Station Status Schema V3.1-RC
 * Describes the capacity and rental availability of the station.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#station_statusjson)
 */
export interface GBFSStationStatusV31RC {
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
   * Contains the status information for all stations in the system.
   */
  data: {
    /**
     * Array of stations, each containing capacity and rental availability details.
     */
    stations: Array<{
      /**
       * Identifier of a station.
       */
      station_id: string;

      /**
       * Number of vehicles of any type physically available for rental at the station.
       * **Minimum**: 0
       */
      num_vehicles_available: number;

      /**
       * Array of objects displaying the total number of each vehicle type at the station (added in v2.1-RC).
       */
      vehicle_types_available?: Array<{
        /**
         * The vehicle_type_id of vehicle at the station (added in v2.1-RC).
         */
        vehicle_type_id: string;

        /**
         * A number representing the total amount of this vehicle type at the station (added in v2.1-RC).
         * **Minimum**: 0
         */
        count: number;
      }>;

      /**
       * Number of disabled vehicles of any type at the station.
       * **Minimum**: 0
       */
      num_vehicles_disabled?: number;

      /**
       * Number of functional docks physically at the station.
       * **Minimum**: 0
       */
      num_docks_available?: number;

      /**
       * Number of empty but disabled docks at the station.
       * **Minimum**: 0
       */
      num_docks_disabled?: number;

      /**
       * Is the station currently on the street?
       */
      is_installed: boolean;

      /**
       * Is the station currently renting vehicles?
       */
      is_renting: boolean;

      /**
       * Is the station accepting vehicle returns?
       */
      is_returning: boolean;

      /**
       * The last time this station reported its status to the operator's backend in RFC3339 format.
       * **Format**: date-time
       */
      last_reported: string;

      /**
       * Object displaying available docks by vehicle type (added in v2.1-RC).
       */
      vehicle_docks_available?: Array<{
        /**
         * An array of strings where each string represents a vehicle_type_id that is able to use a particular type of dock at the station (added in v2.1-RC).
         */
        vehicle_type_ids: string[];

        /**
         * A number representing the total number of available docks for the defined vehicle type (added in v2.1-RC).
         * **Minimum**: 0
         */
        count: number;
      }>;
    }>;
  };
}

/**
 * # GBFS Station Status Schema V3.0
 * Describes the capacity and rental availability of the station
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#station_statusjson)
 */
export const gbfsStationStatusSchemaV30 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#station_statusjson',
  description: 'Describes the capacity and rental availability of the station',
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
      description: 'Array that contains one object per station as defined below.',
      type: 'object',
      properties: {
        stations: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              station_id: {
                description: 'Identifier of a station.',
                type: 'string',
              },
              num_vehicles_available: {
                description:
                  'Number of vehicles of any type physically available for rental at the station.',
                type: 'integer',
                minimum: 0,
              },
              vehicle_types_available: {
                description:
                  'Array of objects displaying the total number of each vehicle type at the station (added in v2.1-RC).',
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    vehicle_type_id: {
                      description:
                        'The vehicle_type_id of vehicle at the station (added in v2.1-RC).',
                      type: 'string',
                    },
                    count: {
                      description:
                        'A number representing the total amount of this vehicle type at the station (added in v2.1-RC).',
                      type: 'integer',
                      minimum: 0,
                    },
                  },
                  required: ['vehicle_type_id', 'count'],
                },
              },
              num_vehicles_disabled: {
                description: 'Number of disabled vehicles of any type at the station.',
                type: 'integer',
                minimum: 0,
              },
              num_docks_available: {
                description: 'Number of functional docks physically at the station.',
                type: 'integer',
                minimum: 0,
              },
              num_docks_disabled: {
                description: 'Number of empty but disabled docks at the station.',
                type: 'integer',
                minimum: 0,
              },
              is_installed: {
                description: 'Is the station currently on the street?',
                type: 'boolean',
              },
              is_renting: {
                description: 'Is the station currently renting vehicles?',
                type: 'boolean',
              },
              is_returning: {
                description: 'Is the station accepting vehicle returns?',
                type: 'boolean',
              },
              last_reported: {
                description:
                  "The last time this station reported its status to the operator's backend in RFC3339 format.",
                type: 'string',
                format: 'date-time',
              },
              vehicle_docks_available: {
                description:
                  'Object displaying available docks by vehicle type (added in v2.1-RC).',
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    vehicle_type_ids: {
                      description:
                        'An array of strings where each string represents a vehicle_type_id that is able to use a particular type of dock at the station (added in v2.1-RC).',
                      type: 'array',
                      items: {
                        type: 'string',
                      },
                    },
                    count: {
                      description:
                        'A number representing the total number of available docks for the defined vehicle type (added in v2.1-RC).',
                      type: 'integer',
                      minimum: 0,
                    },
                  },
                  required: ['vehicle_type_ids', 'count'],
                },
              },
            },
            required: [
              'station_id',
              'num_vehicles_available',
              'is_installed',
              'is_renting',
              'is_returning',
              'last_reported',
            ],
          },
        },
      },
      required: ['stations'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS Station Status Schema V3.0
 * Describes the capacity and rental availability of the station.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#station_statusjson)
 */
export interface GBFSStationStatusV30 {
  /**
   * Last time the data in the feed was updated in RFC3339 format.
   * **Format**: date-time
   */
  last_updated: string;

  /**
   * Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
   * **Minimum**: 0
   */
  ttl: number;

  /**
   * GBFS version number to which the feed conforms.
   * **Const**: '3.0'
   */
  version: '3.0';

  /**
   * Data object containing station statuses.
   */
  data: {
    /**
     * List of stations with their rental availability and capacities.
     */
    stations: Array<{
      /**
       * Identifier of the station.
       */
      station_id: string;

      /**
       * Number of vehicles physically available for rental at the station.
       * **Minimum**: 0
       */
      num_vehicles_available: number;

      /**
       * Details of vehicles available by type at the station.
       */
      vehicle_types_available?: Array<{
        vehicle_type_id: string;
        count: number;
      }>;

      /**
       * Number of disabled vehicles at the station.
       * **Minimum**: 0
       */
      num_vehicles_disabled?: number;

      /**
       * Number of functional docks physically at the station.
       * **Minimum**: 0
       */
      num_docks_available?: number;

      /**
       * Number of disabled but empty docks at the station.
       * **Minimum**: 0
       */
      num_docks_disabled?: number;

      /**
       * Indicates whether the station is installed on the street.
       */
      is_installed: boolean;

      /**
       * Indicates whether the station is currently renting vehicles.
       */
      is_renting: boolean;

      /**
       * Indicates whether the station is accepting vehicle returns.
       */
      is_returning: boolean;

      /**
       * Last reported status time in RFC3339 format.
       * **Format**: date-time
       */
      last_reported: string;

      /**
       * Details of docks available by vehicle type at the station.
       */
      vehicle_docks_available?: Array<{
        vehicle_type_ids: string[];
        count: number;
      }>;
    }>;
  };
}
