/**
 * # GBFS System Hours Schema V2.3, V2.2, V2.1, OR V2.0
 * Describes the operating calendar for a system.
 *
 * ## Links
 * - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_hoursjson)
 * - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_hoursjson)
 * - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_hoursjson)
 * - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_hoursjson)
 */
export type GBFSSystemHoursV2 =
  | GBFSSystemHoursV23
  | GBFSSystemHoursV22
  | GBFSSystemHoursV21
  | GBFSSystemHoursV20;

/**
 * # GBFS System Hours Schema V2.3
 * Describes the system hours of operation.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_hoursjson)
 */
export const gbfsSystemHoursSchemaV23 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_hoursjson',
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
      const: '2.3',
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
 * # GBFS System Hours V2.3
 * Describes the system hours of operation.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_hoursjson)
 */
export interface GBFSSystemHoursV23 {
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
   * Contains system hours data.
   */
  data: {
    /**
     * Rental hours for the system.
     */
    rental_hours: Array<{
      /**
       * Array of member and nonmember values indicating that this set of rental hours applies to either members or non-members only.
       * **Enum**: ["member", "nonmember"]
       * **Min Items**: 1
       * **Max Items**: 2
       */
      user_types: ('member' | 'nonmember')[];

      /**
       * Abbreviations of English names of the days of the week.
       * **Enum**: ["sun", "mon", "tue", "wed", "thu", "fri", "sat"]
       * **Min Items**: 1
       * **Max Items**: 7
       */
      days: ('sun' | 'mon' | 'tue' | 'wed' | 'thu' | 'fri' | 'sat')[];

      /**
       * Start time for the hours of operation of the system.
       * **Pattern**: `^([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$`
       */
      start_time: string;

      /**
       * End time for the hours of operation of the system.
       * **Pattern**: `^([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$`
       */
      end_time: string;
    }>;
  };
}

/**
 * # GBFS System Hours Schema V2.2
 * Describes the system hours of operation.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_hoursjson)
 */
export const gbfsSystemHoursSchemaV22 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_hoursjson',
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
      const: '2.2',
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
 * # GBFS System Hours V2.2
 * Describes the system hours of operation.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_hoursjson)
 */
export interface GBFSSystemHoursV22 {
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
   * Contains system hours data.
   */
  data: {
    /**
     * Rental hours for the system.
     */
    rental_hours: Array<{
      /**
       * Array of member and nonmember values indicating that this set of rental hours applies to either members or non-members only.
       * **Enum**: ["member", "nonmember"]
       * **Min Items**: 1
       * **Max Items**: 2
       */
      user_types: ('member' | 'nonmember')[];

      /**
       * Abbreviations of English names of the days of the week.
       * **Enum**: ["sun", "mon", "tue", "wed", "thu", "fri", "sat"]
       * **Min Items**: 1
       * **Max Items**: 7
       */
      days: ('sun' | 'mon' | 'tue' | 'wed' | 'thu' | 'fri' | 'sat')[];

      /**
       * Start time for the hours of operation of the system.
       * **Pattern**: `^([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$`
       */
      start_time: string;

      /**
       * End time for the hours of operation of the system.
       * **Pattern**: `^([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$`
       */
      end_time: string;
    }>;
  };
}

/**
 * # GBFS System Hours Schema V2.1
 * Describes the system hours of operation.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_hoursjson)
 */
export const gbfsSystemHoursSchemaV21 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_hoursjson',
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
      const: '2.1',
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
 * # GBFS System Hours V2.1
 * Describes the system hours of operation.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_hoursjson)
 */
export interface GBFSSystemHoursV21 {
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
   * Contains system hours data.
   */
  data: {
    /**
     * Rental hours for the system.
     */
    rental_hours: Array<{
      /**
       * Array of member and nonmember values indicating that this set of rental hours applies to either members or non-members only.
       * **Enum**: ["member", "nonmember"]
       * **Min Items**: 1
       * **Max Items**: 2
       */
      user_types: ('member' | 'nonmember')[];

      /**
       * Abbreviations of English names of the days of the week.
       * **Enum**: ["sun", "mon", "tue", "wed", "thu", "fri", "sat"]
       * **Min Items**: 1
       * **Max Items**: 7
       */
      days: ('sun' | 'mon' | 'tue' | 'wed' | 'thu' | 'fri' | 'sat')[];

      /**
       * Start time for the hours of operation of the system.
       * **Pattern**: `^([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$`
       */
      start_time: string;

      /**
       * End time for the hours of operation of the system.
       * **Pattern**: `^([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$`
       */
      end_time: string;
    }>;
  };
}

/**
 * # GBFS System Hours Schema V2.0
 * Describes the system hours of operation.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_hoursjson)
 */
export const gbfsSystemHoursSchemaV20 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_hoursjson',
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
      const: '2.0',
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
 * # GBFS System Hours V2.0
 * Describes the system hours of operation.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_hoursjson)
 */
export interface GBFSSystemHoursV20 {
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
   * Contains system hours data.
   */
  data: {
    /**
     * Rental hours for the system.
     */
    rental_hours: Array<{
      /**
       * Array of member and nonmember values indicating that this set of rental hours applies to either members or non-members only.
       * **Enum**: ["member", "nonmember"]
       * **Min Items**: 1
       * **Max Items**: 2
       */
      user_types: ('member' | 'nonmember')[];

      /**
       * Abbreviations of English names of the days of the week.
       * **Enum**: ["sun", "mon", "tue", "wed", "thu", "fri", "sat"]
       * **Min Items**: 1
       * **Max Items**: 7
       */
      days: ('sun' | 'mon' | 'tue' | 'wed' | 'thu' | 'fri' | 'sat')[];

      /**
       * Start time for the hours of operation of the system.
       * **Pattern**: `^([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$`
       */
      start_time: string;

      /**
       * End time for the hours of operation of the system.
       * **Pattern**: `^([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$`
       */
      end_time: string;
    }>;
  };
}
