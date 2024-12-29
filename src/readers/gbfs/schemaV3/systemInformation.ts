import { LICENSES, TIMEZONE_IDENTIFIER_LIST } from '../../..';

import type { Licenses, TimeZone } from '../../..';

/**
 * # GBFS System Information Schema V3.1-RC & V3.0
 * Details including system operator, system location, year implemented, URL, contact info, time zone.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_informationjson)
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_informationjson)
 */
export type GBFSSystemInformationV3 = GBFSSystemInformationV31RC | GBFSSystemInformationV30;

/**
 * # GBFS System Information Schema V3.1-RC
 * Details including system operator, system location, year implemented, URL, contact info, time zone.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_informationjson)
 */
export const gbfsSystemInformationSchemaV31RC = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_informationjson',
  description:
    'Details including system operator, system location, year implemented, URL, contact info, time zone.',
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
      description: 'Response data in the form of name:value pairs.',
      type: 'object',
      properties: {
        system_id: {
          description:
            'Identifier for this vehicle share system. This should be globally unique (even between different systems).',
          type: 'string',
        },
        languages: {
          description:
            'List of languages used in translated strings. Each element in the list must be of type Language.',
          type: 'array',
          items: {
            description: 'IETF BCP 47 language code.',
            type: 'string',
            pattern: '^[a-z]{2,3}(-[A-Z]{2})?$',
          },
        },
        name: {
          description:
            'Name of the system to be displayed to customers. An array with one object per supported language with the following keys:',
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
        opening_hours: {
          description:
            'Hours and dates of operation for the system in OSM opening_hours format. (added in v3.0)',
          type: 'string',
        },
        short_name: {
          description:
            'Abbreviation for a system. An array with one object per supported language with the following keys:',
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
        operator: {
          description:
            'Name of the system operator. An array with one object per supported language with the following keys:',
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
        url: {
          description: 'The URL of the vehicle share system.',
          type: 'string',
          format: 'uri',
        },
        purchase_url: {
          description: 'URL where a customer can purchase a membership.',
          type: 'string',
          format: 'uri',
        },
        start_date: {
          description: 'Date that the system began operations.',
          type: 'string',
          format: 'date',
        },
        termination_date: {
          description:
            'Date after which this data source will no longer be available to consuming applications.',
          type: 'string',
          format: 'date',
        },
        phone_number: {
          description:
            'This OPTIONAL field SHOULD contain a single voice telephone number for the specified system’s customer service department. MUST be in E.164 format as defined in Field Types.',
          type: 'string',
          pattern: '^\\+[1-9]\\d{1,14}$',
        },
        email: {
          description:
            "Email address actively monitored by the operator's customer service department.",
          type: 'string',
          format: 'email',
        },
        feed_contact_email: {
          description:
            'A single contact email address for consumers of this feed to report technical issues (added in v1.1).',
          type: 'string',
          format: 'email',
        },
        manifest_url: {
          description:
            'REQUIRED if the producer publishes datasets for more than one system geography, for example Berlin and Paris. A fully qualified URL pointing to the manifest.json file for the publisher.',
          type: 'string',
          format: 'uri',
        },
        timezone: {
          description: 'The time zone where the system is located.',
          type: 'string',
          enum: TIMEZONE_IDENTIFIER_LIST,
        },
        license_id: {
          description:
            'REQUIRED if the dataset is provided under a standard license. An identifier for a standard license from the SPDX License List. Provide license_id rather than license_url if the license is included in the SPDX License List. See the GBFS wiki for a comparison of a subset of standard licenses. If the license_id and license_url fields are blank or omitted, this indicates that the feed is provided under the Creative Commons Universal Public Domain Dedication.',
          type: 'string',
          enum: LICENSES,
        },
        license_url: {
          description:
            'A fully qualified URL of a page that defines the license terms for the GBFS data for this system.',
          type: 'string',
          format: 'uri',
        },
        attribution_organization_name: {
          description:
            'If the feed license requires attribution, name of the organization to which attribution should be provided. An array with one object per supported language with the following keys:',
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
        attribution_url: {
          description: 'URL of the organization to which attribution should be provided.',
          type: 'string',
          format: 'uri',
        },
        brand_assets: {
          description:
            'An object where each key defines one of the items listed below (added in v2.3-RC).',
          type: 'object',
          properties: {
            brand_last_modified: {
              description:
                'Date that indicates the last time any included brand assets were updated (added in v2.3-RC).',
              type: 'string',
              format: 'date',
            },
            brand_terms_url: {
              description:
                'A fully qualified URL pointing to the location of a page that defines the license terms of brand icons, colors or other trademark information (added in v2.3-RC).',
              type: 'string',
              format: 'uri',
            },
            brand_image_url: {
              description:
                'A fully qualified URL pointing to the location of a graphic file representing the brand for the service (added in v2.3-RC). ',
              type: 'string',
              format: 'uri',
            },
            brand_image_url_dark: {
              description:
                'A fully qualified URL pointing to the location of a graphic file representing the brand for the service for use in dark mode (added in v2.3-RC).',
              type: 'string',
              format: 'uri',
            },
            color: {
              description: 'Color used to represent the brand for the service (added in v2.3-RC)',
              type: 'string',
              pattern: '^#([a-fA-F0-9]{6})$',
            },
          },
          required: ['brand_last_modified', 'brand_image_url'],
        },
        terms_url: {
          description:
            'A fully qualified URL pointing to the terms of service (also often called "terms of use" or "terms and conditions") for the service. An array with one object per supported language with the following keys:',
          type: 'array',
          items: {
            type: 'object',
            properties: {
              text: {
                description: 'The translated text.',
                type: 'string',
                format: 'uri',
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
        terms_last_updated: {
          description:
            'The date that the terms of service provided at terms_url were last updated (added in v2.3-RC)',
          type: 'string',
          format: 'date',
        },
        privacy_url: {
          description:
            'A fully qualified URL pointing to the privacy policy for the service. An array with one object per supported language with the following keys:',
          type: 'array',
          items: {
            type: 'object',
            properties: {
              text: {
                description: 'The translated text.',
                type: 'string',
                format: 'uri',
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
        privacy_last_updated: {
          description:
            'The date that the privacy policy provided at privacy_url was last updated (added in v2.3-RC).',
          type: 'string',
          format: 'date',
        },
        rental_apps: {
          description:
            'Contains rental app information in the android and ios JSON objects (added in v1.1).',
          type: 'object',
          properties: {
            android: {
              description:
                'Contains rental app download and app discovery information for the Android platform. (added in v1.1)',
              type: 'object',
              properties: {
                store_uri: {
                  description:
                    'URI where the rental Android app can be downloaded from (added in v1.1).',
                  type: 'string',
                  format: 'uri',
                },
                discovery_uri: {
                  description:
                    'URI that can be used to discover if the rental Android app is installed on the device (added in v1.1).',
                  type: 'string',
                  format: 'uri',
                },
              },
              required: ['store_uri', 'discovery_uri'],
            },
            ios: {
              description: 'Contains rental information for the iOS platform (added in v1.1).',
              type: 'object',
              properties: {
                store_uri: {
                  description:
                    'URI where the rental iOS app can be downloaded from (added in v1.1).',
                  type: 'string',
                  format: 'uri',
                },
                discovery_uri: {
                  description:
                    'URI that can be used to discover if the rental iOS app is installed on the device (added in v1.1).',
                  type: 'string',
                  format: 'uri',
                },
              },
              required: ['store_uri', 'discovery_uri'],
            },
          },
        },
      },
      oneOf: [
        {
          not: { required: ['license_url', 'license_id'] },
        },
        {
          required: ['license_id'],
          not: { required: ['license_id'] },
        },
        {
          required: ['license_url'],
          not: { required: ['license_url'] },
        },
      ],
      required: [
        'system_id',
        'languages',
        'name',
        'opening_hours',
        'feed_contact_email',
        'timezone',
      ],
      dependentRequired: {
        terms_url: ['terms_last_updated'],
        privacy_url: ['privacy_last_updated'],
      },
      additionalProperties: false,
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS System Information Schema V3.1-RC
 * Details including system operator, system location, year implemented, URL, contact info, time zone.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_informationjson)
 */
export interface GBFSSystemInformationV31RC {
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
   * Response data in the form of name:value pairs.
   */
  data: {
    /**
     * Identifier for this vehicle share system. This should be globally unique.
     */
    system_id: string;

    /**
     * List of languages used in translated strings.
     * Each element must be an IETF BCP 47 language code.
     * **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
     */
    languages: string[];

    /**
     * Name of the system to be displayed to customers.
     * An array with one object per supported language.
     */
    name: Array<{
      text: string;
      language: string;
    }>;

    /**
     * Hours and dates of operation for the system in OSM opening_hours format. (added in v3.0)
     */
    opening_hours: string;

    /**
     * Abbreviation for the system. An array with one object per supported language.
     */
    short_name?: Array<{
      text: string;
      language: string;
    }>;

    /**
     * Name of the system operator. An array with one object per supported language.
     */
    operator?: Array<{
      text: string;
      language: string;
    }>;

    /**
     * The URL of the vehicle share system.
     * **Format**: uri
     */
    url?: string;

    /**
     * URL where a customer can purchase a membership.
     * **Format**: uri
     */
    purchase_url?: string;

    /**
     * Date that the system began operations.
     * **Format**: date
     */
    start_date?: string;

    /**
     * Date after which this data source will no longer be available.
     * **Format**: date
     */
    termination_date?: string;

    /**
     * Contact phone number for the customer service department.
     * **Pattern**: `^\+[1-9]\d{1,14}$`
     */
    phone_number?: string;

    /**
     * Email address for customer service.
     * **Format**: email
     */
    email?: string;

    /**
     * Contact email for consumers of this feed to report technical issues.
     * **Format**: email
     */
    feed_contact_email: string;

    /**
     * A fully qualified URL pointing to the manifest.json file for the publisher.
     * **Format**: uri
     */
    manifest_url?: string;

    /**
     * The time zone where the system is located.
     * **Enum**: [... full list of timezones ...]
     */
    timezone: TimeZone;

    /**
     * License information, either `license_id` or `license_url` must be provided.
     */
    license_id?: Licenses;
    license_url?: string; // **Format**: uri

    /**
     * Brand assets for the service.
     */
    brand_assets?: {
      brand_last_modified: string; // **Format**: date
      brand_terms_url?: string; // **Format**: uri
      brand_image_url: string; // **Format**: uri
      brand_image_url_dark?: string; // **Format**: uri
      color?: string; // **Pattern**: `^#([a-fA-F0-9]{6})$`
    };

    /**
     * Terms of service.
     */
    terms_url?: Array<{
      text: string; // **Format**: uri
      language: string; // **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
    }>;
    terms_last_updated?: string; // **Format**: date

    /**
     * Privacy policy.
     */
    privacy_url?: Array<{
      text: string; // **Format**: uri
      language: string; // **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
    }>;
    privacy_last_updated?: string; // **Format**: date

    /**
     * Rental app information.
     */
    rental_apps?: {
      android?: {
        store_uri: string; // **Format**: uri
        discovery_uri: string; // **Format**: uri
      };
      ios?: {
        store_uri: string; // **Format**: uri
        discovery_uri: string; // **Format**: uri
      };
    };
  };
}

/**
 * # GBFS System Information Schema V3.0
 * Details including system operator, system location, year implemented, URL, contact info, time zone.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_informationjson)
 */
export const gbfsSystemInformationSchemaV30 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_informationjson',
  description:
    'Details including system operator, system location, year implemented, URL, contact info, time zone.',
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
      description: 'Response data in the form of name:value pairs.',
      type: 'object',
      properties: {
        system_id: {
          description:
            'Identifier for this vehicle share system. This should be globally unique (even between different systems).',
          type: 'string',
        },
        languages: {
          description:
            'List of languages used in translated strings. Each element in the list must be of type Language.',
          type: 'array',
          items: {
            description: 'IETF BCP 47 language code.',
            type: 'string',
            pattern: '^[a-z]{2,3}(-[A-Z]{2})?$',
          },
        },
        name: {
          description:
            'Name of the system to be displayed to customers. An array with one object per supported language with the following keys:',
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
        opening_hours: {
          description:
            'Hours and dates of operation for the system in OSM opening_hours format. (added in v3.0)',
          type: 'string',
        },
        short_name: {
          description:
            'Abbreviation for a system. An array with one object per supported language with the following keys:',
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
        operator: {
          description:
            'Name of the system operator. An array with one object per supported language with the following keys:',
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
        url: {
          description: 'The URL of the vehicle share system.',
          type: 'string',
          format: 'uri',
        },
        purchase_url: {
          description: 'URL where a customer can purchase a membership.',
          type: 'string',
          format: 'uri',
        },
        start_date: {
          description: 'Date that the system began operations.',
          type: 'string',
          format: 'date',
        },
        termination_date: {
          description:
            'Date after which this data source will no longer be available to consuming applications.',
          type: 'string',
          format: 'date',
        },
        phone_number: {
          description:
            'This OPTIONAL field SHOULD contain a single voice telephone number for the specified system’s customer service department. MUST be in E.164 format as defined in Field Types.',
          type: 'string',
          pattern: '^\\+[1-9]\\d{1,14}$',
        },
        email: {
          description:
            "Email address actively monitored by the operator's customer service department.",
          type: 'string',
          format: 'email',
        },
        feed_contact_email: {
          description:
            'A single contact email address for consumers of this feed to report technical issues (added in v1.1).',
          type: 'string',
          format: 'email',
        },
        manifest_url: {
          description:
            'REQUIRED if the producer publishes datasets for more than one system geography, for example Berlin and Paris. A fully qualified URL pointing to the manifest.json file for the publisher.',
          type: 'string',
          format: 'uri',
        },
        timezone: {
          description: 'The time zone where the system is located.',
          type: 'string',
          enum: TIMEZONE_IDENTIFIER_LIST,
        },
        license_id: {
          description:
            'REQUIRED if the dataset is provided under a standard license. An identifier for a standard license from the SPDX License List. Provide license_id rather than license_url if the license is included in the SPDX License List. See the GBFS wiki for a comparison of a subset of standard licenses. If the license_id and license_url fields are blank or omitted, this indicates that the feed is provided under the Creative Commons Universal Public Domain Dedication.',
          type: 'string',
          enum: LICENSES,
        },
        license_url: {
          description:
            'A fully qualified URL of a page that defines the license terms for the GBFS data for this system.',
          type: 'string',
          format: 'uri',
        },
        attribution_organization_name: {
          description:
            'If the feed license requires attribution, name of the organization to which attribution should be provided. An array with one object per supported language with the following keys:',
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
        attribution_url: {
          description: 'URL of the organization to which attribution should be provided.',
          type: 'string',
          format: 'uri',
        },
        brand_assets: {
          description:
            'An object where each key defines one of the items listed below (added in v2.3-RC).',
          type: 'object',
          properties: {
            brand_last_modified: {
              description:
                'Date that indicates the last time any included brand assets were updated (added in v2.3-RC).',
              type: 'string',
              format: 'date',
            },
            brand_terms_url: {
              description:
                'A fully qualified URL pointing to the location of a page that defines the license terms of brand icons, colors or other trademark information (added in v2.3-RC).',
              type: 'string',
              format: 'uri',
            },
            brand_image_url: {
              description:
                'A fully qualified URL pointing to the location of a graphic file representing the brand for the service (added in v2.3-RC). ',
              type: 'string',
              format: 'uri',
            },
            brand_image_url_dark: {
              description:
                'A fully qualified URL pointing to the location of a graphic file representing the brand for the service for use in dark mode (added in v2.3-RC).',
              type: 'string',
              format: 'uri',
            },
            color: {
              description: 'Color used to represent the brand for the service (added in v2.3-RC)',
              type: 'string',
              pattern: '^#([a-fA-F0-9]{6})$',
            },
          },
          required: ['brand_last_modified', 'brand_image_url'],
        },
        terms_url: {
          description:
            'A fully qualified URL pointing to the terms of service (also often called "terms of use" or "terms and conditions") for the service. An array with one object per supported language with the following keys:',
          type: 'array',
          items: {
            type: 'object',
            properties: {
              text: {
                description: 'The translated text.',
                type: 'string',
                format: 'uri',
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
        terms_last_updated: {
          description:
            'The date that the terms of service provided at terms_url were last updated (added in v2.3-RC)',
          type: 'string',
          format: 'date',
        },
        privacy_url: {
          description:
            'A fully qualified URL pointing to the privacy policy for the service. An array with one object per supported language with the following keys:',
          type: 'array',
          items: {
            type: 'object',
            properties: {
              text: {
                description: 'The translated text.',
                type: 'string',
                format: 'uri',
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
        privacy_last_updated: {
          description:
            'The date that the privacy policy provided at privacy_url was last updated (added in v2.3-RC).',
          type: 'string',
          format: 'date',
        },
        rental_apps: {
          description:
            'Contains rental app information in the android and ios JSON objects (added in v1.1).',
          type: 'object',
          properties: {
            android: {
              description:
                'Contains rental app download and app discovery information for the Android platform. (added in v1.1)',
              type: 'object',
              properties: {
                store_uri: {
                  description:
                    'URI where the rental Android app can be downloaded from (added in v1.1).',
                  type: 'string',
                  format: 'uri',
                },
                discovery_uri: {
                  description:
                    'URI that can be used to discover if the rental Android app is installed on the device (added in v1.1).',
                  type: 'string',
                  format: 'uri',
                },
              },
              required: ['store_uri', 'discovery_uri'],
            },
            ios: {
              description: 'Contains rental information for the iOS platform (added in v1.1).',
              type: 'object',
              properties: {
                store_uri: {
                  description:
                    'URI where the rental iOS app can be downloaded from (added in v1.1).',
                  type: 'string',
                  format: 'uri',
                },
                discovery_uri: {
                  description:
                    'URI that can be used to discover if the rental iOS app is installed on the device (added in v1.1).',
                  type: 'string',
                  format: 'uri',
                },
              },
              required: ['store_uri', 'discovery_uri'],
            },
          },
        },
      },
      oneOf: [
        {
          not: { required: ['license_url', 'license_id'] },
        },
        {
          required: ['license_id'],
          not: { required: ['license_id'] },
        },
        {
          required: ['license_url'],
          not: { required: ['license_url'] },
        },
      ],
      required: [
        'system_id',
        'languages',
        'name',
        'opening_hours',
        'feed_contact_email',
        'timezone',
      ],
      dependentRequired: {
        terms_url: ['terms_last_updated'],
        privacy_url: ['privacy_last_updated'],
      },
      additionalProperties: false,
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS System Information Schema V3.0
 * Details including system operator, system location, year implemented, URL, contact info, time zone.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_informationjson)
 */
export interface GBFSSystemInformationV30 {
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
   * System information data object.
   */
  data: {
    /**
     * Globally unique identifier for the system.
     */
    system_id: string;

    /**
     * List of languages used in translated strings.
     */
    languages: string[];

    /**
     * Name of the system to be displayed to customers.
     */
    name: Array<{
      text: string;
      language: string;
    }>;

    /**
     * Hours and dates of operation in OSM opening_hours format.
     */
    opening_hours: string;

    /**
     * Abbreviation for the system.
     */
    short_name?: Array<{
      text: string;
      language: string;
    }>;

    /**
     * Name of the system operator.
     */
    operator?: Array<{
      text: string;
      language: string;
    }>;

    /**
     * URL of the vehicle share system.
     */
    url?: string;

    /**
     * URL to purchase a membership.
     */
    purchase_url?: string;

    /**
     * Date the system began operations.
     */
    start_date?: string;

    /**
     * Date after which the data source will no longer be available.
     */
    termination_date?: string;

    /**
     * Customer service phone number in E.164 format.
     */
    phone_number?: string;

    /**
     * Email address actively monitored by customer service.
     */
    email?: string;

    /**
     * Contact email for feed consumers to report technical issues.
     */
    feed_contact_email: string;

    /**
     * URL to the manifest.json file for the publisher.
     */
    manifest_url?: string;

    /**
     * Time zone of the system.
     */
    timezone: TimeZone;

    /**
     * Standard license identifier for the dataset.
     */
    license_id?: Licenses;

    /**
     * URL defining the license terms.
     */
    license_url?: string;

    /**
     * Name of the organization to which attribution should be provided.
     */
    attribution_organization_name?: Array<{
      text: string;
      language: string;
    }>;

    /**
     * URL of the organization for attribution.
     */
    attribution_url?: string;

    /**
     * Brand assets and related information.
     */
    brand_assets?: {
      brand_last_modified: string;
      brand_terms_url?: string;
      brand_image_url: string;
      brand_image_url_dark?: string;
      color?: string;
    };

    /**
     * Terms of service URL.
     */
    terms_url?: Array<{
      text: string;
      language: string;
    }>;

    /**
     * Date terms of service were last updated.
     */
    terms_last_updated?: string;

    /**
     * Privacy policy URL.
     */
    privacy_url?: Array<{
      text: string;
      language: string;
    }>;

    /**
     * Date the privacy policy was last updated.
     */
    privacy_last_updated?: string;

    /**
     * Rental app information for Android and iOS platforms.
     */
    rental_apps?: {
      android?: {
        store_uri: string;
        discovery_uri: string;
      };
      ios?: {
        store_uri: string;
        discovery_uri: string;
      };
    };
  };
}
