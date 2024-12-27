/**
 * # GBFS Versions Schema V3.1-RC & V3.0
 * Lists all feed endpoints published according to versions of the GBFS documentation. (added in v1.1)
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#gbfs_versionsjson)
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#gbfs_versionsjson)
 */
export type GBFSVersionsV3 = GBFSVersionsV31RC | GBFSVersionsV30;

/**
 * # GBFS Versions Schema V3.1-RC
 * Lists all feed endpoints published according to version sof the GBFS documentation. (added in v1.1)
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#gbfs_versionsjson)
 */
export const gbfsVersionsSchemaV31RC = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#gbfs_versionsjson',
  description:
    'Lists all feed endpoints published according to version sof the GBFS documentation. (added in v1.1)',
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
        'GBFS version number to which the feed conforms, according to the versioning framework.',
      type: 'string',
      const: '3.1-RC',
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
                enum: ['1.0', '1.1', '2.0', '2.1', '2.2', '2.3', '3.0', '3.1-RC'],
              },
              url: {
                description: 'URL of the corresponding gbfs.json endpoint',
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
 * # GBFS Versions Schema V3.1-RC
 * Lists all feed endpoints published according to versions of the GBFS documentation. (added in v1.1)
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#gbfs_versionsjson)
 */
export interface GBFSVersionsV31RC {
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
   * GBFS version number to which the feed conforms, according to the versioning framework.
   */
  version: '3.1-RC';

  /**
   * Response data in the form of name:value pairs.
   */
  data: {
    /**
     * Contains one object for each of the available versions of a feed.
     * The array must be sorted by increasing MAJOR and MINOR version numbers.
     */
    versions: Array<{
      /**
       * The semantic version of the feed in the form X.Y
       */
      version: '1.0' | '1.1' | '2.0' | '2.1' | '2.2' | '2.3' | '3.0' | '3.1-RC';

      /**
       * URL of the corresponding gbfs.json endpoint
       * **format** uri
       */
      url: string;
    }>;
  };
}

/**
 * # GBFS Versions Schema V3.0
 * Lists all feed endpoints published according to version sof the GBFS documentation. (added in v1.1)
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#gbfs_versionsjson)
 */
export const gbfsVersionsSchemaV30 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#gbfs_versionsjson',
  description:
    'Lists all feed endpoints published according to version sof the GBFS documentation. (added in v1.1)',
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
        'GBFS version number to which the feed conforms, according to the versioning framework.',
      type: 'string',
      const: '3.0',
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
                description: 'URL of the corresponding gbfs.json endpoint',
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
 * # GBFS Versions Schema V3.0
 * Lists all feed endpoints published according to versions of the GBFS documentation. (added in v1.1)
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#gbfs_versionsjson)
 */
export interface GBFSVersionsV30 {
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
   * GBFS version number to which the feed conforms, according to the versioning framework.
   * **Const**: 3.0
   */
  version: '3.0';

  /**
   * Response data in the form of name:value pairs.
   */
  data: {
    /**
     * Contains one object for each of the available versions of a feed. The array must be sorted by increasing MAJOR and MINOR version number.
     */
    versions: Array<{
      /**
       * The semantic version of the feed in the form X.Y.
       * **Enum**: ['1.0', '1.1', '2.0', '2.1', '2.2', '2.3', '3.0']
       */
      version: '1.0' | '1.1' | '2.0' | '2.1' | '2.2' | '2.3' | '3.0';

      /**
       * URL of the corresponding gbfs.json endpoint.
       * **Format**: uri
       */
      url: string;
    }>;
  };
}
