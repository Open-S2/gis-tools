/**
 * # GBFS System Hours Schema V1.1 OR GBFS System Hours Schema V1.0
 * Describes the system hours of operation.
 *
 * ## Links
 * - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_hoursjson)
 * - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_hoursjson)
 */
export type GBFSSystemHoursV1 = GBFSSystemHoursV11 | GBFSSystemHoursV10;

/**
 * # GBFS System Hours Schema V1.1
 * Describes the system hours of operation.
 *
 * ## Links
 * - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_hoursjson)
 */
export const gbfsSystemHoursSchemaV11 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_hoursjson',
  description: 'Describes the system hours of operation.',
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
      description: 'Array that contains system hours of operations.',
      type: 'object',
      properties: {
        rental_hours: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              user_types: {
                description:
                  'Array of member and nonmember value(s) indicating that this set of rental hours applies to either members or non-members only.',
                type: 'array',
                items: {
                  type: 'string',
                  enum: ['member', 'nonmember'],
                },
                minItems: 1,
                maxItems: 2,
              },
              days: {
                description:
                  'An array of abbreviations (first 3 letters) of English names of the days of the week for which this object applies.',
                type: 'array',
                items: {
                  type: 'string',
                  enum: ['sun', 'mon', 'tue', 'wed', 'thu', 'fri', 'sat'],
                },
                minItems: 1,
                maxItems: 7,
              },
              start_time: {
                description: 'Start time for the hours of operation of the system.',
                type: 'string',
                pattern: '^([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$',
              },
              end_time: {
                description: 'End time for the hours of operation of the system.',
                type: 'string',
                pattern: '^([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$',
              },
            },
            required: ['user_types', 'days', 'start_time', 'end_time'],
          },
        },
      },
      required: ['rental_hours'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * GBFS System Hours Schema V1.1 Interface
 */
export interface GBFSSystemHoursV11 {
  /** Last time the data in the feed was updated in POSIX time. */
  last_updated: number;

  /** Number of seconds before the data in the feed will be updated again. */
  ttl: number;

  /** GBFS version number (1.1). */
  version: '1.1';

  /** Data containing system hours of operations. */
  data: {
    rental_hours: Array<{
      user_types: ('member' | 'nonmember')[];
      days: ('sun' | 'mon' | 'tue' | 'wed' | 'thu' | 'fri' | 'sat')[];
      start_time: string;
      end_time: string;
    }>;
  };
}

/**
 * # GBFS System Hours Schema V1.0
 * Describes the system hours of operation.
 *
 * ## Links
 * - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_hoursjson)
 */
export const gbfsSystemHoursSchemaV10 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_hoursjson',
  description: 'Describes the system hours of operation.',
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
      description: 'Array that contains system hours of operations.',
      type: 'object',
      properties: {
        rental_hours: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              user_types: {
                description:
                  'Array of member and nonmember value(s) indicating that this set of rental hours applies to either members or non-members only.',
                type: 'array',
                items: {
                  type: 'string',
                  enum: ['member', 'nonmember'],
                },
              },
              days: {
                description:
                  'An array of abbreviations (first 3 letters) of English names of the days of the week for which this object applies.',
                type: 'array',
                items: {
                  type: 'string',
                  enum: ['mon', 'tue', 'wed', 'thu', 'fri', 'sat', 'sun'],
                },
              },
              start_time: {
                description: 'Start time for the hours of operation of the system.',
                type: 'string',
                pattern: '^[0-9]{2}:[0-9]{2}:[0-9]{2}$',
              },
              end_time: {
                description: 'End time for the hours of operation of the system.',
                type: 'string',
                pattern: '^[0-9]{2}:[0-9]{2}:[0-9]{2}$',
              },
            },
            required: ['user_types', 'days', 'start_time', 'end_time'],
          },
        },
      },
      required: ['rental_hours'],
    },
  },
  required: ['last_updated', 'ttl', 'data'],
};

/**
 * GBFS System Hours Schema V1.0 Interface
 */
export interface GBFSSystemHoursV10 {
  /** Last time the data in the feed was updated in POSIX time. */
  last_updated: number;

  /** Number of seconds before the data in the feed will be updated again. */
  ttl: number;

  /** Data containing system hours of operations. */
  data: {
    rental_hours: Array<{
      user_types: ('member' | 'nonmember')[];
      days: ('mon' | 'tue' | 'wed' | 'thu' | 'fri' | 'sat' | 'sun')[];
      start_time: string;
      end_time: string;
    }>;
  };
}
