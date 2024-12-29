/**
 * # GBFS Auto-Discovery Schema V2.x
 * Auto-discovery file that links to all of the other files published by the system.
 *
 * ## Links
 * - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#gbfsjson)
 * - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#gbfsjson)
 * - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#gbfsjson)
 * - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#gbfsjson)
 */
export type GBFSV2 = GBFSV23 | GBFSV22 | GBFSV21 | GBFSV20;

/**
 * # GBFS Auto-Discovery Schema V2.3
 * Auto-discovery file that links to all of the other files published by the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#gbfsjson)
 */
export const gbfsSchemaV23 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#gbfsjson',
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
      const: '2.3',
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
                'An array of all of the feeds that are published by the auto-discovery file. Each element in the array is an object with the keys below.',
              type: 'array',
              items: {
                type: 'object',
                properties: {
                  name: {
                    description:
                      'Key identifying the type of feed this is. The key must be the base file name defined in the spec for the corresponding feed type.',
                    type: 'string',
                    enum: [
                      'gbfs',
                      'gbfs_versions',
                      'system_information',
                      'vehicle_types',
                      'station_information',
                      'station_status',
                      'free_bike_status',
                      'system_hours',
                      'system_alerts',
                      'system_calendar',
                      'system_regions',
                      'system_pricing_plans',
                      'geofencing_zones',
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
 * # GBFS Auto-Discovery V2.3
 * Auto-discovery file that links to all of the other files published by the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#gbfsjson)
 */
export interface GBFSV23 {
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
   * Response data in the form of name:value pairs.
   */
  data: {
    /**
     * An object containing feeds keyed by language code (e.g., "en", "en-US").
     *
     * **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
     */
    [languageCode: string]: {
      /**
       * An array of all of the feeds that are published by the auto-discovery file.
       */
      feeds: Array<{
        /**
         * Key identifying the type of feed this is. The key must be the base file name defined in the spec for the corresponding feed type.
         *
         * **Enum**: "gbfs", "gbfs_versions", "system_information", "vehicle_types", "station_information", "station_status", "free_bike_status", "system_hours", "system_alerts", "system_calendar", "system_regions", "system_pricing_plans", "geofencing_zones"
         */
        name:
          | 'gbfs'
          | 'gbfs_versions'
          | 'system_information'
          | 'vehicle_types'
          | 'station_information'
          | 'station_status'
          | 'free_bike_status'
          | 'system_hours'
          | 'system_alerts'
          | 'system_calendar'
          | 'system_regions'
          | 'system_pricing_plans'
          | 'geofencing_zones';

        /**
         * URL for the feed.
         * **Format**: uri
         */
        url: string;
      }>;
    };
  };
}

/**
 * # GBFS Auto-Discovery Schema V2.2
 * Auto-discovery file that links to all of the other files published by the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#gbfsjson)
 */
export const gbfsSchemaV22 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#gbfsjson',
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
      const: '2.2',
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
                'An array of all of the feeds that are published by the auto-discovery file. Each element in the array is an object with the keys below.',
              type: 'array',
              items: {
                type: 'object',
                properties: {
                  name: {
                    description:
                      'Key identifying the type of feed this is. The key must be the base file name defined in the spec for the corresponding feed type.',
                    type: 'string',
                    enum: [
                      'gbfs',
                      'gbfs_versions',
                      'system_information',
                      'vehicle_types',
                      'station_information',
                      'station_status',
                      'free_bike_status',
                      'system_hours',
                      'system_alerts',
                      'system_calendar',
                      'system_regions',
                      'system_pricing_plans',
                      'geofencing_zones',
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
 * # GBFS Auto-Discovery V2.2
 * Auto-discovery file that links to all of the other files published by the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#gbfsjson)
 */
export interface GBFSV22 {
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
   * **Const**: 2.2
   */
  version: '2.2';

  /**
   * Response data in the form of name:value pairs.
   */
  data: {
    /**
     * An object containing feeds keyed by language code (e.g., "en", "en-US").
     *
     * **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
     */
    [languageCode: string]: {
      /**
       * An array of all of the feeds that are published by the auto-discovery file.
       */
      feeds: Array<{
        /**
         * Key identifying the type of feed this is. The key must be the base file name defined in the spec for the corresponding feed type.
         *
         * **Enum**: "gbfs", "gbfs_versions", "system_information", "vehicle_types", "station_information", "station_status", "free_bike_status", "system_hours", "system_alerts", "system_calendar", "system_regions", "system_pricing_plans", "geofencing_zones"
         */
        name:
          | 'gbfs'
          | 'gbfs_versions'
          | 'system_information'
          | 'vehicle_types'
          | 'station_information'
          | 'station_status'
          | 'free_bike_status'
          | 'system_hours'
          | 'system_alerts'
          | 'system_calendar'
          | 'system_regions'
          | 'system_pricing_plans'
          | 'geofencing_zones';

        /**
         * URL for the feed.
         * **Format**: uri
         */
        url: string;
      }>;
    };
  };
}

/**
 * # GBFS Schema V2.1
 * Auto-discovery file that links to all of the other files published by the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#gbfsjson)
 */
export const gbfsSchemaV21 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#gbfsjson',
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
      const: '2.1',
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
                'An array of all of the feeds that are published by the auto-discovery file. Each element in the array is an object with the keys below.',
              type: 'array',
              items: {
                type: 'object',
                properties: {
                  name: {
                    description:
                      'Key identifying the type of feed this is. The key must be the base file name defined in the spec for the corresponding feed type.',
                    type: 'string',
                    enum: [
                      'gbfs',
                      'gbfs_versions',
                      'system_information',
                      'vehicle_types',
                      'station_information',
                      'station_status',
                      'free_bike_status',
                      'system_hours',
                      'system_alerts',
                      'system_calendar',
                      'system_regions',
                      'system_pricing_plans',
                      'geofencing_zones',
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
 * # GBFS V2.1
 * Auto-discovery file that links to all of the other files published by the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#gbfsjson)
 */
export interface GBFSV21 {
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
   * **Const**: 2.1
   */
  version: '2.1';

  /**
   * Response data in the form of name:value pairs.
   */
  data: {
    /**
     * An object containing feeds keyed by language code (e.g., "en", "en-US").
     *
     * **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
     */
    [languageCode: string]: {
      /**
       * An array of all of the feeds that are published by the auto-discovery file.
       */
      feeds: Array<{
        /**
         * Key identifying the type of feed this is. The key must be the base file name defined in the spec for the corresponding feed type.
         *
         * **Enum**: "gbfs", "gbfs_versions", "system_information", "vehicle_types", "station_information", "station_status", "free_bike_status", "system_hours", "system_alerts", "system_calendar", "system_regions", "system_pricing_plans", "geofencing_zones"
         */
        name:
          | 'gbfs'
          | 'gbfs_versions'
          | 'system_information'
          | 'vehicle_types'
          | 'station_information'
          | 'station_status'
          | 'free_bike_status'
          | 'system_hours'
          | 'system_alerts'
          | 'system_calendar'
          | 'system_regions'
          | 'system_pricing_plans'
          | 'geofencing_zones';

        /**
         * URL for the feed.
         * **Format**: uri
         */
        url: string;
      }>;
    };
  };
}

/**
 * # GBFS Schema V2.0
 * Auto-discovery file that links to all of the other files published by the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#gbfsjson)
 */
export const gbfsSchemaV20 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#gbfsjson',
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
      const: '2.0',
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
                'An array of all of the feeds that are published by the auto-discovery file. Each element in the array is an object with the keys below.',
              type: 'array',
              items: {
                type: 'object',
                properties: {
                  name: {
                    description:
                      'Key identifying the type of feed this is. The key must be the base file name defined in the spec for the corresponding feed type.',
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
 * # GBFS V2.0
 * Auto-discovery file that links to all of the other files published by the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#gbfsjson)
 */
export interface GBFSV20 {
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
   * **Const**: 2.0
   */
  version: '2.0';

  /**
   * Response data in the form of name:value pairs.
   */
  data: {
    /**
     * An object containing feeds keyed by language code (e.g., "en", "en-US").
     *
     * **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
     */
    [languageCode: string]: {
      /**
       * An array of all of the feeds that are published by the auto-discovery file.
       */
      feeds: Array<{
        /**
         * Key identifying the type of feed this is. The key must be the base file name defined in the spec for the corresponding feed type.
         *
         * **Enum**: "gbfs", "gbfs_versions", "system_information", "station_information", "station_status", "free_bike_status", "system_hours", "system_alerts", "system_calendar", "system_regions", "system_pricing_plans"
         */
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

        /**
         * URL for the feed.
         * **Format**: uri
         */
        url: string;
      }>;
    };
  };
}
