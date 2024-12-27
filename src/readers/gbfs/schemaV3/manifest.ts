/**
 * # GBFS Manifest Schema V3.1-RC & V3.0
 * An index of gbfs.json URLs for each GBFS data set produced by a publisher. A single instance of
 * this file should be published at a single stable URL, for example: https://example.com/gbfs/manifest.json.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#manifestjson)
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#manifestjson)
 */
export type GBFSManifestV3 = GBFSManifestV31RC | GBFSManifestV30;

/**
 * # GBFS Manifest Schema V3.1-RC
 * An index of gbfs.json URLs for each GBFS data set produced by a publisher. A single instance of
 * this file should be published at a single stable URL, for example: https://example.com/gbfs/manifest.json.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#manifestjson)
 */
export const gbfsManifestSchemaV31RC = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#manifestjson',
  description:
    'An index of gbfs.json URLs for each GBFS data set produced by a publisher. A single instance of this file should be published at a single stable URL, for example: https://example.com/gbfs/manifest.json.',
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
        datasets: {
          description: 'An array of objects, each containing the keys below.',
          type: 'array',
          items: {
            type: 'object',
            properties: {
              system_id: {
                description:
                  'The system_id from system_information.json for the corresponding data set(s).',
                type: 'string',
              },
              versions: {
                description:
                  'Contains one object, as defined below, for each of the available versions of a feed. The array MUST be sorted by increasing MAJOR and MINOR version number.',
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
              area: {
                description: 'A GeoJSON MultiPolygon that describes the operating area.',
                type: 'object',
                required: ['type', 'coordinates'],
                properties: {
                  type: {
                    type: 'string',
                    enum: ['MultiPolygon'],
                  },
                  coordinates: {
                    type: 'array',
                    items: {
                      type: 'array',
                      items: {
                        type: 'array',
                        minItems: 4,
                        items: {
                          type: 'array',
                          minItems: 2,
                          items: {
                            type: 'number',
                          },
                        },
                      },
                    },
                  },
                },
              },
              country_code: {
                description: 'The ISO 3166-1 alpha-2 country code of the operating area.',
                type: 'string',
                pattern: '^[A-Z]{2}',
              },
            },
            required: ['system_id', 'versions'],
          },
        },
      },
      required: ['datasets'],
      additionalProperties: false,
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS Manifest Schema V3.1-RC
 * An index of gbfs.json URLs for each GBFS data set produced by a publisher. A single instance of
 * this file should be published at a single stable URL, for example: https://example.com/gbfs/manifest.json.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#manifestjson)
 */
export interface GBFSManifestV31RC {
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
   * Contains the dataset information for the publisher.
   */
  data: {
    /**
     * An array of objects, each containing dataset information.
     */
    datasets: Array<{
      /**
       * The system_id from system_information.json for the corresponding data set(s).
       */
      system_id: string;

      /**
       * An array of available versions of a feed, sorted by increasing MAJOR and MINOR version numbers.
       */
      versions: Array<{
        /**
         * The semantic version of the feed in the form X.Y
         * **Enum**: ['1.0', '1.1', '2.0', '2.1', '2.2', '2.3', '3.0', '3.1-RC']
         */
        version: '1.0' | '1.1' | '2.0' | '2.1' | '2.2' | '2.3' | '3.0' | '3.1-RC';

        /**
         * URL of the corresponding gbfs.json endpoint
         * **Format**: uri
         */
        url: string;
      }>;

      /**
       * A GeoJSON MultiPolygon that describes the operating area.
       */
      area?: {
        /**
         * GeoJSON MultiPolygon type.
         * **Enum**: ['MultiPolygon']
         */
        type: 'MultiPolygon';

        /**
         * Coordinates of the MultiPolygon.
         */
        coordinates: Array<Array<Array<[number, number]>>>;
      };

      /**
       * The ISO 3166-1 alpha-2 country code of the operating area.
       * **Pattern**: ^[A-Z]{2}
       */
      country_code?: string;
    }>;
  };
}

/**
 * # GBFS Manifest Schema V3.0
 * An index of gbfs.json URLs for each GBFS data set produced by a publisher. A single instance of
 * this file should be published at a single stable URL, for example: https://example.com/gbfs/manifest.json.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#manifestjson)
 */
export const gbfsManifestSchemaV30 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#manifestjson',
  description:
    'An index of gbfs.json URLs for each GBFS data set produced by a publisher. A single instance of this file should be published at a single stable URL, for example: https://example.com/gbfs/manifest.json.',
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
        datasets: {
          description: 'An array of objects, each containing the keys below.',
          type: 'array',
          items: {
            type: 'object',
            properties: {
              system_id: {
                description:
                  'The system_id from system_information.json for the corresponding data set(s).',
                type: 'string',
              },
              versions: {
                description:
                  'Contains one object, as defined below, for each of the available versions of a feed. The array MUST be sorted by increasing MAJOR and MINOR version number.',
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
            required: ['system_id', 'versions'],
          },
        },
      },
      required: ['datasets'],
      additionalProperties: false,
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS Manifest Schema V3.0
 * An index of gbfs.json URLs for each GBFS data set produced by a publisher. A single instance of
 * this file should be published at a single stable URL, for example: https://example.com/gbfs/manifest.json.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#manifestjson)
 */
export interface GBFSManifestV30 {
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
   * **Const**: '3.0'
   */
  version: '3.0';

  /**
   * Data object containing the list of datasets.
   */
  data: {
    /**
     * Array of datasets containing system IDs and versions.
     */
    datasets: Array<{
      /**
       * The `system_id` from system_information.json for the corresponding data set(s).
       */
      system_id: string;

      /**
       * Array of available versions of the feed, sorted by increasing MAJOR and MINOR version number.
       */
      versions: Array<{
        /**
         * Semantic version of the feed in the form X.Y.
         */
        version: '1.0' | '1.1' | '2.0' | '2.1' | '2.2' | '2.3' | '3.0';

        /**
         * URL of the corresponding gbfs.json endpoint.
         * **Format**: uri
         */
        url: string;
      }>;
    }>;
  };
}
