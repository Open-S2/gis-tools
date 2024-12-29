/**
 * # GBFS System Regions Schema V1.1 OR GBFS System Regions Schema V1.0
 * Describes regions for a system that is broken up by geographic or political region.
 *
 * ## Links
 * - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_regionsjson)
 * - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_regionsjson)
 */
export type GBFSSystemRegionsV1 = GBFSSystemRegionsV11 | GBFSSystemRegionsV10;

/**
 * # GBFS System Regions Schema V1.1
 * Describes regions for a system that is broken up by geographic or political region.
 *
 * ## Links
 * - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_regionsjson)
 */
export const gbfsSystemRegionsSchemaV11 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_regionsjson',
  description:
    'Describes regions for a system that is broken up by geographic or political region.',
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
                description: 'Identifier of the region.',
                type: 'string',
              },
              name: {
                description: 'Public name for this region.',
                type: 'string',
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
 * GBFS System Regions Schema V1.1 Interface
 */
export interface GBFSSystemRegionsV11 {
  /** Last time the data in the feed was updated in POSIX time. */
  last_updated: number;

  /** Number of seconds before the data in the feed will be updated again. */
  ttl: number;

  /** GBFS version number (1.1). */
  version: '1.1';

  /** Data describing regions for a system. */
  data: {
    regions: Array<{
      region_id: string;
      name: string;
    }>;
  };
}

/**
 * # GBFS System Regions Schema V1.0
 * Describes regions for a system that is broken up by geographic or political region.
 *
 * ## Links
 * - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_regionsjson)
 */
export const gbfsSystemRegionsSchemaV10 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_regionsjson',
  description:
    'Describes regions for a system that is broken up by geographic or political region.',
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
                description: 'Identifier of the region.',
                type: 'string',
              },
              name: {
                description: 'Public name for this region.',
                type: 'string',
              },
            },
            required: ['region_id', 'name'],
          },
        },
      },
      required: ['regions'],
    },
  },
  required: ['last_updated', 'ttl', 'data'],
};

/**
 * GBFS System Regions Schema V1.0 Interface
 */
export interface GBFSSystemRegionsV10 {
  /** Last time the data in the feed was updated in POSIX time. */
  last_updated: number;

  /** Number of seconds before the data in the feed will be updated again. */
  ttl: number;

  /** Data describing regions for a system. */
  data: {
    regions: Array<{
      region_id: string;
      name: string;
    }>;
  };
}
