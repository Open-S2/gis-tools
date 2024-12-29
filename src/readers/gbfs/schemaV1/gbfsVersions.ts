/**
 * # GBFS Versions Schema V1.1
 * Lists all feed endpoints published according to versions of the GBFS documentation.
 *
 * ## Links
 * - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#gbfs_versionsjson-added-in-v11)
 */
export type GBFSVersionsV1 = GBFSVersionsV11;

/**
 * # GBFS Versions Schema V1.1
 * Lists all feed endpoints published according to versions of the GBFS documentation. (added in v1.1)
 *
 * ## Links
 * - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#gbfs_versionsjson-added-in-v11)
 */
export const gbfsVersionsSchemaV11 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#gbfs_versionsjson-added-in-v11',
  description:
    'Lists all feed endpoints published according to versions of the GBFS documentation. (added in v1.1)',
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
        'GBFS version number to which the feed conforms, according to the versioning framework.',
      type: 'string',
      const: '1.1',
    },
    data: {
      description: 'Response data in the form of name:value pairs.',
      type: 'object',
      properties: {
        versions: {
          description:
            'Contains one object, as defined below, for each of the available versions of a feed. The array must be sorted by increasing MAJOR and MINOR version number.',
          type: 'array',
          items: {
            type: 'object',
            properties: {
              version: {
                description: 'The semantic version of the feed in the form X.Y',
                type: 'string',
                enum: ['1.0', '1.1', '2.0', '2.1', '2.2', '2.3', '3.0'],
              },
              url: {
                description: 'URL of the corresponding gbfs.json endpoint.',
                type: 'string',
                format: 'uri',
              },
            },
            required: ['version', 'url'],
          },
        },
      },
      required: ['versions'],
      additionalProperties: false,
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * GBFS Versions Schema V1.1 Interface
 */
export interface GBFSVersionsV11 {
  /** Last time the data in the feed was updated in POSIX time. */
  last_updated: number;

  /** Number of seconds before the data in the feed will be updated again. */
  ttl: number;

  /** GBFS version number (1.1). */
  version: '1.1';

  /** Data containing available feed versions. */
  data: {
    versions: Array<{
      version: '1.0' | '1.1' | '2.0' | '2.1' | '2.2' | '2.3' | '3.0';
      url: string;
    }>;
  };
}
