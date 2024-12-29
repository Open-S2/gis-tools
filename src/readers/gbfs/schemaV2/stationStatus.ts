/**
 * # GBFS Station Status Schema V2.3, V2.2, V2.1, OR V2.0
 * List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
 *
 * ## Links
 * - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#station_statusjson)
 * - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#station_statusjson)
 * - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#station_statusjson)
 * - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#station_statusjson)
 */
export type GBFSStationStatusV2 =
  | GBFSStationStatusV23
  | GBFSStationStatusV22
  | GBFSStationStatusV21
  | GBFSStationStatusV20;

/**
 * # GBFS Station Status Schema V2.3
 * Describes the capacity and rental availability of the station.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#station_statusjson)
 */
export const gbfsStationStatusSchemaV23 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#station_statusjson',
  description: 'Describes the capacity and rental availability of the station.',
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
              num_bikes_available: {
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
              num_bikes_disabled: {
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
                  "The last time this station reported its status to the operator's backend in POSIX time.",
                type: 'integer',
                minimum: 1450155600,
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
              'num_bikes_available',
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
 * # GBFS Station Status V2.3
 * Describes the capacity and rental availability of the station.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#station_statusjson)
 */
export interface GBFSStationStatusV23 {
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
   * GBFS version number to which the feed conforms, according to the versioning framework.
   * **Const**: 2.3
   */
  version: '2.3';

  /**
   * Contains station status information.
   */
  data: {
    /**
     * Array of station status objects.
     */
    stations: Array<{
      station_id: string;
      num_bikes_available: number;
      vehicle_types_available?: Array<{
        vehicle_type_id: string;
        count: number;
      }>;
      num_bikes_disabled?: number;
      num_docks_available?: number;
      num_docks_disabled?: number;
      is_installed: boolean;
      is_renting: boolean;
      is_returning: boolean;
      last_reported: number;
      vehicle_docks_available?: Array<{
        vehicle_type_ids: string[];
        count: number;
      }>;
    }>;
  };
}

/**
 * # GBFS Station Status Schema V2.2
 * Describes the capacity and rental availability of the station.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#station_statusjson)
 */
export const gbfsStationStatusSchemaV22 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#station_statusjson',
  description: 'Describes the capacity and rental availability of the station.',
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
      const: '2.2',
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
              num_bikes_available: {
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
              num_bikes_disabled: {
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
                  "The last time this station reported its status to the operator's backend in POSIX time.",
                type: 'number',
                minimum: 1450155600,
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
              'num_bikes_available',
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
 * # GBFS Station Status V2.2
 * Describes the capacity and rental availability of the station.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#station_statusjson)
 */
export interface GBFSStationStatusV22 {
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
   * GBFS version number to which the feed conforms, according to the versioning framework.
   * **Const**: 2.2
   */
  version: '2.2';

  /**
   * Contains station status information.
   */
  data: {
    /**
     * Array of station status objects.
     */
    stations: Array<{
      station_id: string;
      num_bikes_available: number;
      vehicle_types_available?: Array<{
        vehicle_type_id: string;
        count: number;
      }>;
      num_bikes_disabled?: number;
      num_docks_available?: number;
      num_docks_disabled?: number;
      is_installed: boolean;
      is_renting: boolean;
      is_returning: boolean;
      last_reported: number;
      vehicle_docks_available?: Array<{
        vehicle_type_ids: string[];
        count: number;
      }>;
    }>;
  };
}

/**
 * # GBFS Station Status Schema V2.1
 * Describes the capacity and rental availability of the station.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#station_statusjson)
 */
export const gbfsStationStatusSchemaV21 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#station_statusjson',
  description: 'Describes the capacity and rental availability of the station.',
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
      const: '2.1',
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
              num_bikes_available: {
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
              num_bikes_disabled: {
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
                  "The last time this station reported its status to the operator's backend in POSIX time.",
                type: 'number',
                minimum: 1450155600,
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
              'num_bikes_available',
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
 * # GBFS Station Status V2.1
 * Describes the capacity and rental availability of the station.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#station_statusjson)
 */
export interface GBFSStationStatusV21 {
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
   * GBFS version number to which the feed conforms, according to the versioning framework.
   * **Const**: 2.1
   */
  version: '2.1';

  /**
   * Contains station status information.
   */
  data: {
    /**
     * Array of station status objects.
     */
    stations: Array<{
      station_id: string;
      num_bikes_available: number;
      vehicle_types_available?: Array<{
        vehicle_type_id: string;
        count: number;
      }>;
      num_bikes_disabled?: number;
      num_docks_available?: number;
      num_docks_disabled?: number;
      is_installed: boolean;
      is_renting: boolean;
      is_returning: boolean;
      last_reported: number;
      vehicle_docks_available?: Array<{
        vehicle_type_ids: string[];
        count: number;
      }>;
    }>;
  };
}

/**
 * # GBFS Station Status Schema V2.0
 * Describes the capacity and rental availability of the station.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#station_statusjson)
 */
export const gbfsStationStatusSchemaV20 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#station_statusjson',
  description: 'Describes the capacity and rental availability of the station.',
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
      const: '2.0',
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
              num_bikes_available: {
                description:
                  'Number of vehicles of any type physically available for rental at the station.',
                type: 'integer',
                minimum: 0,
              },
              num_bikes_disabled: {
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
                  "The last time this station reported its status to the operator's backend.",
                type: 'number',
                minimum: 1450155600,
              },
            },
            required: [
              'station_id',
              'num_bikes_available',
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
 * # GBFS Station Status V2.0
 * Describes the capacity and rental availability of the station.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#station_statusjson)
 */
export interface GBFSStationStatusV20 {
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
   * GBFS version number to which the feed conforms, according to the versioning framework.
   * **Const**: 2.0
   */
  version: '2.0';

  /**
   * Contains station status information.
   */
  data: {
    /**
     * Array of station status objects.
     */
    stations: Array<{
      station_id: string;
      num_bikes_available: number;
      num_bikes_disabled?: number;
      num_docks_available?: number;
      num_docks_disabled?: number;
      is_installed: boolean;
      is_renting: boolean;
      is_returning: boolean;
      last_reported: number;
    }>;
  };
}
