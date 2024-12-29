/**
 * # GBFS Versions Schema V2.3, V2.2, V2.1, OR V2.0
 * Lists all feed endpoints published according to versions of the GBFS documentation.
 *
 * ## Links
 * - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#gbfs_versionsjson)
 * - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#gbfs_versionsjson-added-in-v11)
 * - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#gbfs_versionsjson-added-in-v11)
 * - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#gbfs_versionsjson-added-in-v11)
 */
export type GBFSVersionsV2 = GBFSVersionsV23 | GBFSVersionsV22 | GBFSVersionsV21 | GBFSVersionsV20;

/**
 * # GBFS Versions Schema V2.3
 * Lists all feed endpoints published according to versions of the GBFS documentation (added in v1.1).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#gbfs_versionsjson)
 */
export const gbfsVersionsSchemaV23 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#gbfs_versionsjson',
  description:
    'Lists all feed endpoints published according to versions of the GBFS documentation (added in v1.1).',
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
      const: '2.3',
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
                description: 'The semantic version of the feed in the form X.Y.',
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
 * # GBFS Versions V2.3
 * Lists all feed endpoints published according to versions of the GBFS documentation (added in v1.1).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#gbfs_versionsjson)
 */
export interface GBFSVersionsV23 {
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
   * Response data in the form of name:value pairs.
   */
  data: {
    /**
     * Contains one object for each of the available versions of a feed.
     * The array must be sorted by increasing MAJOR and MINOR version number.
     */
    versions: Array<{
      /**
       * The semantic version of the feed in the form X.Y.
       * **Enum**: "1.0", "1.1", "2.0", "2.1", "2.2", "2.3", "3.0"
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

/**
 * # GBFS Versions Schema V2.2
 * Lists all feed endpoints published according to versions of the GBFS documentation (added in v1.1).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#gbfs_versionsjson-added-in-v11)
 */
export const gbfsVersionsSchemaV22 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#gbfs_versionsjson-added-in-v11',
  description:
    'Lists all feed endpoints published according to versions of the GBFS documentation (added in v1.1).',
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
      const: '2.2',
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
                description: 'The semantic version of the feed in the form X.Y.',
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
 * # GBFS Versions V2.2
 * Lists all feed endpoints published according to versions of the GBFS documentation (added in v1.1).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#gbfs_versionsjson-added-in-v11)
 */
export interface GBFSVersionsV22 {
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
   * Response data in the form of name:value pairs.
   */
  data: {
    /**
     * Contains one object for each of the available versions of a feed.
     * The array must be sorted by increasing MAJOR and MINOR version number.
     */
    versions: Array<{
      /**
       * The semantic version of the feed in the form X.Y.
       * **Enum**: "1.0", "1.1", "2.0", "2.1", "2.2", "2.3", "3.0"
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

/**
 * # GBFS Versions Schema V2.1
 * Lists all feed endpoints published according to versions of the GBFS documentation (added in v1.1).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#gbfs_versionsjson-added-in-v11)
 */
export const gbfsVersionsSchemaV21 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#gbfs_versionsjson-added-in-v11',
  description:
    'Lists all feed endpoints published according to versions of the GBFS documentation. (added in v1.1).',
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
      const: '2.1',
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
                description: 'The semantic version of the feed in the form X.Y.',
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
 * # GBFS Versions V2.1
 * Lists all feed endpoints published according to versions of the GBFS documentation (added in v1.1).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#gbfs_versionsjson-added-in-v11)
 */
export interface GBFSVersionsV21 {
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
   * Response data in the form of name:value pairs.
   */
  data: {
    /**
     * Contains one object for each of the available versions of a feed.
     * The array must be sorted by increasing MAJOR and MINOR version number.
     */
    versions: Array<{
      /**
       * The semantic version of the feed in the form X.Y.
       * **Enum**: "1.0", "1.1", "2.0", "2.1", "2.2", "2.3", "3.0"
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

/**
 * # GBFS Versions Schema V2.0
 * Lists all feed endpoints published according to versions of the GBFS documentation (added in v1.1).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#gbfs_versionsjson-added-in-v11)
 */
export const gbfsVersionsSchemaV20 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#gbfs_versionsjson-added-in-v11',
  description:
    'Lists all feed endpoints published according to versions of the GBFS documentation. (added in v1.1).',
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
      const: '2.0',
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
                description: 'The semantic version of the feed in the form X.Y.',
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
 * # GBFS Versions V2.0
 * Lists all feed endpoints published according to versions of the GBFS documentation (added in v1.1).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#gbfs_versionsjson-added-in-v11)
 */
export interface GBFSVersionsV20 {
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
   * Response data in the form of name:value pairs.
   */
  data: {
    /**
     * Contains one object for each of the available versions of a feed.
     * The array must be sorted by increasing MAJOR and MINOR version number.
     */
    versions: Array<{
      /**
       * The semantic version of the feed in the form X.Y.
       * **Enum**: "1.0", "1.1", "2.0", "2.1", "2.2", "2.3", "3.0"
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
