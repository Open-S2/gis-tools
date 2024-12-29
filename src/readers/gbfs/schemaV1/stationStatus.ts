/**
 * # GBFS Station Status Schema V1.1 OR GBFS Station Status Schema V1.0
 * Describes the capacity and rental availability of the station.
 *
 * ## Links
 * - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#station_statusjson)
 * - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#station_statusjson)
 */
export type GBFSStationStatusV1 = GBFSStationStatusV11 | GBFSStationStatusV10;

/**
 * # GBFS Station Status Schema V1.1
 * Describes the capacity and rental availability of the station.
 *
 * ## Links
 * - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#station_statusjson)
 */
export const gbfsStationStatusSchemaV11 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#station_statusjson',
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
      const: '1.1',
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
              station_id: { description: 'Identifier of a station.', type: 'string' },
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
                type: 'number',
                minimum: 0,
                maximum: 1,
              },
              is_renting: {
                description: 'Is the station currently renting vehicles?',
                type: 'number',
                minimum: 0,
                maximum: 1,
              },
              is_returning: {
                description: 'Is the station accepting vehicle returns?',
                type: 'number',
                minimum: 0,
                maximum: 1,
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
              'num_docks_available',
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
 * GBFS Station Status Schema V1.1 Interface
 */
export interface GBFSStationStatusV11 {
  /** Last time the data in the feed was updated in POSIX time. */
  last_updated: number;

  /** Number of seconds before the data in the feed will be updated again. */
  ttl: number;

  /** GBFS version number (1.1). */
  version: '1.1';

  /** Data containing an array of station statuses. */
  data: {
    stations: Array<{
      station_id: string;
      num_bikes_available: number;
      num_bikes_disabled?: number;
      num_docks_available: number;
      num_docks_disabled?: number;
      is_installed: 0 | 1;
      is_renting: 0 | 1;
      is_returning: 0 | 1;
      last_reported: number;
    }>;
  };
}

/**
 * # GBFS Station Status Schema V1.0
 * Describes the capacity and rental availability of the station.
 *
 * ## Links
 * - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#station_statusjson)
 */
export const gbfsStationStatusSchemaV10 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#station_statusjson',
  description: 'Describes the capacity and rental availability of the station.',
  type: 'object',
  properties: {
    last_updated: {
      description: 'Last time the data in the feed was updated in POSIX time.',
      type: 'integer',
      minimum: 0,
      maximum: 1924988399,
    },
    ttl: {
      description:
        'Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).',
      type: 'integer',
      minimum: 0,
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
              station_id: { description: 'Identifier of a station.', type: 'string' },
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
                oneOf: [{ type: 'boolean' }, { type: 'number' }],
              },
              is_renting: {
                description: 'Is the station currently renting vehicles?',
                oneOf: [{ type: 'boolean' }, { type: 'number' }],
              },
              is_returning: {
                description: 'Is the station accepting vehicle returns?',
                oneOf: [{ type: 'boolean' }, { type: 'number' }],
              },
              last_reported: {
                description:
                  "The last time this station reported its status to the operator's backend in POSIX time.",
                type: 'number',
              },
            },
            required: [
              'station_id',
              'num_bikes_available',
              'num_docks_available',
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
  required: ['last_updated', 'ttl', 'data'],
};

/**
 * GBFS Station Status Schema V1.0 Interface
 */
export interface GBFSStationStatusV10 {
  /** Last time the data in the feed was updated in POSIX time. */
  last_updated: number;

  /** Number of seconds before the data in the feed will be updated again. */
  ttl: number;

  /** Data containing an array of station statuses. */
  data: {
    stations: Array<{
      station_id: string;
      num_bikes_available: number;
      num_bikes_disabled?: number;
      num_docks_available: number;
      num_docks_disabled?: number;
      is_installed: boolean | number;
      is_renting: boolean | number;
      is_returning: boolean | number;
      last_reported: number;
    }>;
  };
}
