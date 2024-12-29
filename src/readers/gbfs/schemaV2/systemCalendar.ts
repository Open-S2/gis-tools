/**
 * # GBFS System Calendar Schema V2.3, V2.2, V2.1, OR V2.0
 * Describes the operating calendar for a system.
 *
 * ## Links
 * - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_calendarjson)
 * - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_calendarjson)
 * - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_calendarjson)
 * - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_calendarjson)
 */
export type GBFSSystemCalendarV2 =
  | GBFSSystemCalendarV23
  | GBFSSystemCalendarV22
  | GBFSSystemCalendarV21
  | GBFSSystemCalendarV20;

/**
 * # GBFS System Calendar Schema V2.3
 * Describes the operating calendar for a system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_calendarjson)
 */
export const gbfsSystemCalendarSchemaV23 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_calendarjson',
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
      const: '2.3',
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
 * # GBFS System Calendar V2.3
 * Describes the operating calendar for a system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_calendarjson)
 */
export interface GBFSSystemCalendarV23 {
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
   * Contains the operations calendar data.
   */
  data: {
    /**
     * Array of calendars for the system operations.
     */
    calendars: Array<{
      /**
       * Starting month for the system operations.
       * **Minimum**: 1
       * **Maximum**: 12
       */
      start_month: number;

      /**
       * Starting day for the system operations.
       * **Minimum**: 1
       * **Maximum**: 31
       */
      start_day: number;

      /**
       * Starting year for the system operations.
       * **Pattern**: `^\\d{4}$`
       */
      start_year?: number;

      /**
       * End month for the system operations.
       * **Minimum**: 1
       * **Maximum**: 12
       */
      end_month: number;

      /**
       * End day for the system operations.
       * **Minimum**: 1
       * **Maximum**: 31
       */
      end_day: number;

      /**
       * End year for the system operations.
       * **Pattern**: `^\\d{4}$`
       */
      end_year?: number;
    }>;
  };
}

/**
 * # GBFS System Calendar Schema V2.2
 * Describes the operating calendar for a system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_calendarjson)
 */
export const gbfsSystemCalendarSchemaV22 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_calendarjson',
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
      const: '2.2',
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
 * # GBFS System Calendar V2.2
 * Describes the operating calendar for a system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_calendarjson)
 */
export interface GBFSSystemCalendarV22 {
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
   * Contains the operations calendar data.
   */
  data: {
    /**
     * Array of calendars for the system operations.
     */
    calendars: Array<{
      /**
       * Starting month for the system operations.
       * **Minimum**: 1
       * **Maximum**: 12
       */
      start_month: number;

      /**
       * Starting day for the system operations.
       * **Minimum**: 1
       * **Maximum**: 31
       */
      start_day: number;

      /**
       * Starting year for the system operations.
       * **Pattern**: `^\\d{4}$`
       */
      start_year?: number;

      /**
       * End month for the system operations.
       * **Minimum**: 1
       * **Maximum**: 12
       */
      end_month: number;

      /**
       * End day for the system operations.
       * **Minimum**: 1
       * **Maximum**: 31
       */
      end_day: number;

      /**
       * End year for the system operations.
       * **Pattern**: `^\\d{4}$`
       */
      end_year?: number;
    }>;
  };
}

/**
 * # GBFS System Calendar Schema V2.1
 * Describes the operating calendar for a system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_calendarjson)
 */
export const gbfsSystemCalendarSchemaV21 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_calendarjson',
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
      const: '2.1',
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
 * # GBFS System Calendar V2.1
 * Describes the operating calendar for a system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_calendarjson)
 */
export interface GBFSSystemCalendarV21 {
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
   * Contains the operations calendar data.
   */
  data: {
    /**
     * Array of calendars for the system operations.
     */
    calendars: Array<{
      /**
       * Starting month for the system operations.
       * **Minimum**: 1
       * **Maximum**: 12
       */
      start_month: number;

      /**
       * Starting day for the system operations.
       * **Minimum**: 1
       * **Maximum**: 31
       */
      start_day: number;

      /**
       * Starting year for the system operations.
       * **Pattern**: `^\\d{4}$`
       */
      start_year?: number;

      /**
       * End month for the system operations.
       * **Minimum**: 1
       * **Maximum**: 12
       */
      end_month: number;

      /**
       * End day for the system operations.
       * **Minimum**: 1
       * **Maximum**: 31
       */
      end_day: number;

      /**
       * End year for the system operations.
       * **Pattern**: `^\\d{4}$`
       */
      end_year?: number;
    }>;
  };
}

/**
 * # GBFS System Calendar Schema V2.0
 * Describes the operating calendar for a system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_calendarjson)
 */
export const gbfsSystemCalendarSchemaV20 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_calendarjson',
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
      const: '2.0',
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
 * # GBFS System Calendar V2.0
 * Describes the operating calendar for a system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_calendarjson)
 */
export interface GBFSSystemCalendarV20 {
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
   * Contains the operations calendar data.
   */
  data: {
    /**
     * Array of calendars for the system operations.
     */
    calendars: Array<{
      /**
       * Starting month for the system operations.
       * **Minimum**: 1
       * **Maximum**: 12
       */
      start_month: number;

      /**
       * Starting day for the system operations.
       * **Minimum**: 1
       * **Maximum**: 31
       */
      start_day: number;

      /**
       * Starting year for the system operations.
       * **Pattern**: `^\\d{4}$`
       */
      start_year?: number;

      /**
       * End month for the system operations.
       * **Minimum**: 1
       * **Maximum**: 12
       */
      end_month: number;

      /**
       * End day for the system operations.
       * **Minimum**: 1
       * **Maximum**: 31
       */
      end_day: number;

      /**
       * End year for the system operations.
       * **Pattern**: `^\\d{4}$`
       */
      end_year?: number;
    }>;
  };
}
