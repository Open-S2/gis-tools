/**
 * # GBFS System Regions V2.3, V2.2, V2.1, OR V2.0
 * Describes regions for a system that is broken up by geographic or political region.
 *
 * ## Links
 * - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_regionsjson)
 * - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_regionsjson)
 * - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_regionsjson)
 * - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_regionsjson)
 */
export type GBFSSystemRegionsV2 =
  | GBFSSystemRegionsV23
  | GBFSSystemRegionsV22
  | GBFSSystemRegionsV21
  | GBFSSystemRegionsV20;

/**
 * # GBFS System Regions Schema V2.3
 *
 * Describes regions for a system that is broken up by geographic or political region.
 *
 * **Links**:
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_regionsjson)
 */
export const gbfsSystemRegionsSchemaV23 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_regionsjson',
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
      const: '2.3',
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
 * # GBFS System Regions Schema V2.3
 *
 * Describes regions for a system that is broken up by geographic or political region.
 *
 * **Links**:
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_regionsjson)
 */
export interface GBFSSystemRegionsV23 {
  last_updated: number;
  ttl: number;
  version: '2.3';
  data: {
    regions: Array<{
      region_id: string; // Identifier of the region
      name: string; // Public name for the region
    }>;
  };
}

/**
 * # GBFS System Regions Schema V2.2
 *
 * Describes regions for a system that is broken up by geographic or political region.
 *
 * **Links**:
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_regionsjson)
 */
export const gbfsSystemRegionsSchemaV22 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_regionsjson',
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
      const: '2.2',
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
 * # GBFS System Regions Schema V2.2
 *
 * Describes regions for a system that is broken up by geographic or political region.
 *
 * **Links**:
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_regionsjson)
 */
export interface GBFSSystemRegionsV22 {
  last_updated: number; // Last time the data in the feed was updated in POSIX time
  ttl: number; // Number of seconds before the data in the feed will be updated again
  version: '2.2'; // GBFS version number
  data: {
    regions: Array<{
      region_id: string; // Identifier of the region
      name: string; // Public name for the region
    }>;
  };
}

/**
 * # GBFS System Regions Schema V2.1
 *
 * Describes regions for a system that is broken up by geographic or political region.
 *
 * **Links**:
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_regionsjson)
 */
export const gbfsSystemRegionsSchemaV21 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_regionsjson',
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
      const: '2.1',
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
 * # GBFS System Regions Schema V2.1
 *
 * Describes regions for a system that is broken up by geographic or political region.
 *
 * **Links**:
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_regionsjson)
 */
export interface GBFSSystemRegionsV21 {
  last_updated: number; // Last time the data in the feed was updated in POSIX time
  ttl: number; // Number of seconds before the data in the feed will be updated again
  version: '2.1'; // GBFS version number
  data: {
    regions: Array<{
      region_id: string; // Identifier of the region
      name: string; // Public name for the region
    }>;
  };
}

/**
 * # GBFS System Regions Schema V2.0
 *
 * Describes regions for a system that is broken up by geographic or political region.
 *
 * **Links**:
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_regionsjson)
 */
export const gbfsSystemRegionsSchemaV20 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_regionsjson',
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
      const: '2.0',
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
 * # GBFS System Regions Schema V2.0
 *
 * Describes regions for a system that is broken up by geographic or political region.
 *
 * **Links**:
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_regionsjson)
 */
export interface GBFSSystemRegionsV20 {
  last_updated: number; // Last time the data in the feed was updated in POSIX time
  ttl: number; // Number of seconds before the data in the feed will be updated again
  version: '2.0'; // GBFS version number
  data: {
    regions: Array<{
      region_id: string; // Identifier of the region
      name: string; // Public name for the region
    }>;
  };
}
