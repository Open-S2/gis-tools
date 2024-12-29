/**
 * # GBFS System Regions Schema V3.1-RC & V3.0
 * Describes regions for a system that is broken up by geographic or political region.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_regionsjson)
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_regionsjson)
 */
export type GBFSSystemRegionsV3 = GBFSSystemRegionsV31RC | GBFSSystemRegionsV30;

/**
 * # GBFS System Regions Schema V3.1-RC
 * Describes regions for a system that is broken up by geographic or political region.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_regionsjson)
 */
export const gbfsSystemRegionsSchemaV31RC = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_regionsjson',
  description:
    'Describes regions for a system that is broken up by geographic or political region.',
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
      description:
        'Describe regions for a system that is broken up by geographic or political region.',
      type: 'object',
      properties: {
        regions: {
          description: 'Array of regions.',
          type: 'array',
          items: {
            type: 'object',
            properties: {
              region_id: {
                description: 'identifier of the region.',
                type: 'string',
              },
              name: {
                description: 'Public name for this region.',
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    text: {
                      description: 'The translated text.',
                      type: 'string',
                    },
                    language: {
                      description: 'IETF BCP 47 language code.',
                      type: 'string',
                      pattern: '^[a-z]{2,3}(-[A-Z]{2})?$',
                    },
                  },
                  required: ['text', 'language'],
                },
              },
            },
            required: ['region_id', 'name'],
          },
        },
      },
      required: ['regions'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS System Regions Schema V3.1-RC
 * Describes regions for a system that is broken up by geographic or political region.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_regionsjson)
 */
export interface GBFSSystemRegionsV31RC {
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
   * Contains the region data for the system.
   */
  data: {
    /**
     * Array of regions for the system.
     */
    regions: Array<{
      /**
       * Identifier of the region.
       */
      region_id: string;

      /**
       * Public name for this region.
       */
      name: Array<{
        /**
         * The translated text of the region name.
         */
        text: string;

        /**
         * IETF BCP 47 language code.
         * **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
         */
        language: string;
      }>;
    }>;
  };
}

/**
 * # GBFS System Regions Schema V3.0
 * Describes regions for a system that is broken up by geographic or political region.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_regionsjson)
 */
export const gbfsSystemRegionsSchemaV30 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_regionsjson',
  description:
    'Describes regions for a system that is broken up by geographic or political region.',
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
      description:
        'Describe regions for a system that is broken up by geographic or political region.',
      type: 'object',
      properties: {
        regions: {
          description: 'Array of regions.',
          type: 'array',
          items: {
            type: 'object',
            properties: {
              region_id: {
                description: 'identifier of the region.',
                type: 'string',
              },
              name: {
                description: 'Public name for this region.',
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    text: {
                      description: 'The translated text.',
                      type: 'string',
                    },
                    language: {
                      description: 'IETF BCP 47 language code.',
                      type: 'string',
                      pattern: '^[a-z]{2,3}(-[A-Z]{2})?$',
                    },
                  },
                  required: ['text', 'language'],
                },
              },
            },
            required: ['region_id', 'name'],
          },
        },
      },
      required: ['regions'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS System Regions Schema V3.0
 * Describes regions for a system that is broken up by geographic or political region.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_regionsjson)
 */
export interface GBFSSystemRegionsV30 {
  /**
   * Last time the data in the feed was updated in RFC3339 format.
   */
  last_updated: string;

  /**
   * Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
   */
  ttl: number;

  /**
   * GBFS version number to which the feed conforms.
   */
  version: '3.0';

  /**
   * Data describing regions for a system.
   */
  data: {
    /**
     * Array of regions.
     */
    regions: Array<{
      /**
       * Identifier of the region.
       */
      region_id: string;

      /**
       * Public name for this region.
       */
      name: Array<{
        text: string;
        language: string;
      }>;
    }>;
  };
}
