/**
 * # GBFS System Calendar Schema V1.1 OR GBFS System Calendar Schema V1.0
 * Describes the operating calendar for a system.
 *
 * ## Links
 * - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_calendarjson)
 * - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_calendarjson)
 */
export type GBFSSystemCalendarV1 = GBFSSystemCalendarV11 | GBFSSystemCalendarV10;

/**
 * # GBFS System Calendar Schema V1.1
 * Describes the operating calendar for a system.
 *
 * ## Links
 * - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_calendarjson)
 */
export const gbfsSystemCalendarSchemaV11 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_calendarjson',
  description: 'Describes the operating calendar for a system.',
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
      description: 'Array that contains operations calendar for the system.',
      type: 'object',
      properties: {
        calendars: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              start_month: {
                description: 'Starting month for the system operations.',
                type: 'integer',
                minimum: 1,
                maximum: 12,
              },
              start_day: {
                description: 'Starting day for the system operations.',
                type: 'integer',
                minimum: 1,
                maximum: 31,
              },
              start_year: {
                description: 'Starting year for the system operations.',
                type: 'integer',
                pattern: '^\\d{4}$',
              },
              end_month: {
                description: 'End month for the system operations.',
                type: 'integer',
                minimum: 1,
                maximum: 12,
              },
              end_day: {
                description: 'End day for the system operations.',
                type: 'integer',
                minimum: 1,
                maximum: 31,
              },
              end_year: {
                description: 'End year for the system operations.',
                type: 'integer',
                pattern: '^\\d{4}$',
              },
            },
            required: ['start_month', 'start_day', 'end_month', 'end_day'],
          },
        },
      },
      required: ['calendars'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * GBFS System Calendar Schema V1.1 Interface
 */
export interface GBFSSystemCalendarV11 {
  /** Last time the data in the feed was updated in POSIX time. */
  last_updated: number;

  /** Number of seconds before the data in the feed will be updated again. */
  ttl: number;

  /** GBFS version number (1.1). */
  version: '1.1';

  /** Data containing the system's operations calendar. */
  data: {
    calendars: Array<{
      start_month: number;
      start_day: number;
      start_year?: number;
      end_month: number;
      end_day: number;
      end_year?: number;
    }>;
  };
}

/**
 * # GBFS System Calendar Schema V1.0
 * Describes the operating calendar for a system.
 *
 * ## Links
 * - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_calendarjson)
 */
export const gbfsSystemCalendarSchemaV10 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_calendarjson',
  description: 'Describes the operating calendar for a system.',
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
      description: 'Array that contains operations calendar for the system.',
      type: 'object',
      properties: {
        calendars: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              start_month: {
                description: 'Starting month for the system operations.',
                type: 'integer',
                minimum: 1,
                maximum: 12,
              },
              start_day: {
                description: 'Starting day for the system operations.',
                type: 'integer',
                minimum: 1,
                maximum: 31,
              },
              start_year: {
                description: 'Starting year for the system operations.',
                type: 'integer',
              },
              end_month: {
                description: 'End month for the system operations.',
                type: 'integer',
                minimum: 1,
                maximum: 12,
              },
              end_day: {
                description: 'End day for the system operations.',
                type: 'integer',
                minimum: 1,
                maximum: 31,
              },
              end_year: {
                description: 'End year for the system operations.',
                type: 'integer',
              },
            },
            required: ['start_month', 'start_day', 'end_month', 'end_day'],
          },
          minItems: 1,
        },
      },
      required: ['calendars'],
    },
  },
  required: ['last_updated', 'ttl', 'data'],
};

/**
 * GBFS System Calendar Schema V1.0 Interface
 */
export interface GBFSSystemCalendarV10 {
  /** Last time the data in the feed was updated in POSIX time. */
  last_updated: number;

  /** Number of seconds before the data in the feed will be updated again. */
  ttl: number;

  /** Data containing the system's operations calendar. */
  data: {
    calendars: Array<{
      start_month: number;
      start_day: number;
      start_year?: number;
      end_month: number;
      end_day: number;
      end_year?: number;
    }>;
  };
}
