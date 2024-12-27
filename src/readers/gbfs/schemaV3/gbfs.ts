/**
 * # GBFS Schema V3.1-RC OR GBFS Schema V3.0
 * Auto-discovery file that links to all of the other files published by the system.
 *
 * ## Links
 * - [GBFS Specification V3.1-RC](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#gbfsjson)
 * - [GBFS Specification V3.0](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#gbfsjson)
 */
export type GBFSV3 = GBFSV31RC | GBFSV30;

/**
 * # GBFS Schema V3.1-RC
 * Auto-discovery file that links to all of the other files published by the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#gbfsjson)
 */
export const gbfsSchemaV31RC = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#gbfsjson',
  description: 'Auto-discovery file that links to all of the other files published by the system.',
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
                  'vehicle_status',
                  'system_alerts',
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
          minItems: 1,
        },
      },
      required: ['feeds'],
    },
  },
  additionalProperties: false,
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS Schema V3.1-RC
 * Auto-discovery file that links to all of the other files published by the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#gbfsjson)
 */
export interface GBFSV31RC {
  /**
   * Last time the data in the feed was updated in RFC3339 format.
   * **format** date-time
   */
  last_updated: string;

  /**
   * Number of seconds before the data in the feed will be updated again
   * (0 if the data should always be refreshed).
   * **minimum** 0
   */
  ttl: number;

  /**
   * GBFS version number to which the feed conforms,
   * according to the versioning framework (added in v1.1).
   */
  version: '3.1-RC';

  /**
   * Contains the list of feeds published by the auto-discovery file.
   */
  data: {
    /**
     * An array of all of the feeds that are published by the auto-discovery file.
     * Each element in the array is an object with the keys below.
     * **minItems** 1
     */
    feeds: Array<{
      /**
       * Key identifying the type of feed this is.
       * The key must be the base file name defined in the spec for the corresponding feed type.
       */
      name:
        | 'gbfs'
        | 'gbfs_versions'
        | 'system_information'
        | 'vehicle_types'
        | 'station_information'
        | 'station_status'
        | 'vehicle_status'
        | 'system_alerts'
        | 'system_regions'
        | 'system_pricing_plans'
        | 'geofencing_zones';

      /**
       * URL for the feed.
       * **format** uri
       */
      url: string;
    }>;
  };
}

/**
 * # GBFS Schema V3.0
 * Auto-discovery file that links to all of the other files published by the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#gbfsjson)
 */
export const gbfsSchemaV30 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#gbfsjson',
  description: 'Auto-discovery file that links to all of the other files published by the system.',
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
                  'vehicle_status',
                  'system_alerts',
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
          minItems: 1,
        },
      },
      required: ['feeds'],
    },
  },
  additionalProperties: false,
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS Schema V3.0
 * Auto-discovery file that links to all of the other files published by the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#gbfsjson)
 */
export interface GBFSV30 {
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
   * GBFS version number to which the feed conforms, according to the versioning framework (added in v1.1).
   * **Const**: 3.0
   */
  version: '3.0';

  /**
   * Contains the data for feeds published by the auto-discovery file.
   */
  data: {
    /**
     * An array of all feeds published by the auto-discovery file. Each element is an object with the following keys:
     */
    feeds: Array<{
      /**
       * Key identifying the type of feed this is. The key must be the base file name defined in the spec for the corresponding feed type.
       * **Enum**: ['gbfs', 'gbfs_versions', 'system_information', 'vehicle_types', 'station_information', 'station_status', 'vehicle_status', 'system_alerts', 'system_regions', 'system_pricing_plans', 'geofencing_zones']
       */
      name:
        | 'gbfs'
        | 'gbfs_versions'
        | 'system_information'
        | 'vehicle_types'
        | 'station_information'
        | 'station_status'
        | 'vehicle_status'
        | 'system_alerts'
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
}
