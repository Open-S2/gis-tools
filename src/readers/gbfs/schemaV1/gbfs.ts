/**
 * # GBFS Schema V1.1 OR GBFS Schema V1.0
 * Auto-discovery file that links to all of the other files published by the system.
 *
 * ## Links
 * - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#gbfsjson)
 * - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#gbfsjson)
 */
export type GBFSV1 = GBFSV11 | GBFSV10;

/**
 * # GBFS Schema V1.1
 * Auto-discovery file that links to all of the other files published by the system.
 *
 * ## Links
 * - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#gbfsjson)
 */
export const gbfsSchemaV11 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#gbfsjson',
  description: 'Auto-discovery file that links to all of the other files published by the system.',
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
      description: 'Response data in the form of name:value pairs.',
      type: 'object',
      patternProperties: {
        '^[a-z]{2,3}(-[A-Z]{2})?$': {
          type: 'object',
          properties: {
            feeds: {
              description:
                'An array of all of the feeds that are published by the auto-discovery file.',
              type: 'array',
              items: {
                type: 'object',
                properties: {
                  name: {
                    description: 'Key identifying the type of feed.',
                    type: 'string',
                    enum: [
                      'gbfs',
                      'gbfs_versions',
                      'system_information',
                      'station_information',
                      'station_status',
                      'free_bike_status',
                      'system_hours',
                      'system_alerts',
                      'system_calendar',
                      'system_regions',
                      'system_pricing_plans',
                    ],
                  },
                  url: {
                    description: 'URL for the feed.',
                    type: 'string',
                    format: 'uri',
                  },
                },
                required: ['name', 'url'],
              },
            },
          },
          required: ['feeds'],
        },
      },
      minProperties: 1,
      additionalProperties: false,
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * GBFS Schema V1.1 Interface
 */
export interface GBFSV11 {
  /** Last time the data in the feed was updated in POSIX time. */
  last_updated: number;

  /** Number of seconds before the data in the feed will be updated again. */
  ttl: number;

  /** GBFS version number (1.1). */
  version: '1.1';

  /** Response data in the form of name:value pairs. */
  data: {
    [locale: string]: {
      feeds: Array<{
        name:
          | 'gbfs'
          | 'gbfs_versions'
          | 'system_information'
          | 'station_information'
          | 'station_status'
          | 'free_bike_status'
          | 'system_hours'
          | 'system_alerts'
          | 'system_calendar'
          | 'system_regions'
          | 'system_pricing_plans';
        url: string;
      }>;
    };
  };
}

/**
 * # GBFS Schema V1.0
 * Auto-discovery file that links to all of the other files published by the system.
 *
 * ## Links
 * - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#gbfsjson)
 */
export const gbfsSchemaV10 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#gbfsjson',
  description: 'Auto-discovery file that links to all of the other files published by the system.',
  type: 'object',
  properties: {
    last_updated: {
      description: 'Last time the data in the feed was updated in POSIX time.',
      type: 'integer',
      minimum: 0,
      maximum: 1924988399,
    },
    ttl: {
      description: 'Number of seconds before the data in the feed will be updated again.',
      type: 'integer',
      minimum: 0,
    },
    data: {
      description: 'Response data in the form of name:value pairs.',
      type: 'object',
      patternProperties: {
        '^[a-zA-Z]{2}$': {
          type: 'object',
          properties: {
            feeds: {
              description:
                'An array of all of the feeds that are published by the auto-discovery file.',
              type: 'array',
              items: {
                type: 'object',
                properties: {
                  name: { type: 'string' },
                  url: { type: 'string' },
                },
                required: ['name', 'url'],
              },
            },
          },
          required: ['feeds'],
        },
      },
      minProperties: 1,
      additionalProperties: false,
    },
  },
  required: ['last_updated', 'ttl', 'data'],
};

/**
 * GBFS Schema V1.0 Interface
 */
export interface GBFSV10 {
  /** Last time the data in the feed was updated in POSIX time. */
  last_updated: number;

  /** Number of seconds before the data in the feed will be updated again. */
  ttl: number;

  /** Response data in the form of name:value pairs. */
  data: {
    [locale: string]: {
      feeds: Array<{
        name: string;
        url: string;
      }>;
    };
  };
}
