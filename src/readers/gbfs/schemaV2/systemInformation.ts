import { TIMEZONE_IDENTIFIER_LIST } from '../../..';

import type { TimeZone } from '../../..';

/**
 * # GBFS System Information Schema V2.3, V2.2, V2.1, OR V2.0
 * Details including system operator, system location, year implemented, URL, contact info, time zone.
 *
 * ## Links
 * - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_informationjson)
 * - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_informationjson)
 * - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_informationjson)
 * - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_informationjson)
 */
export type GBFSSystemInformationV2 =
  | GBFSSystemInformationV23
  | GBFSSystemInformationV22
  | GBFSSystemInformationV21
  | GBFSSystemInformationV20;

/**
 * # GBFS System Information Schema V2.3
 * Details including system operator, system location, year implemented, URL, contact info, and time zone.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_informationjson)
 */
export const gbfsSystemInformationSchemaV2 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_informationjson',
  description:
    'Details including system operator, system location, year implemented, URL, contact info, time zone.',
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
      properties: {
        system_id: {
          description:
            'Identifier for this vehicle share system. This should be globally unique (even between different systems).',
          type: 'string',
        },
        language: {
          description:
            'The language that will be used throughout the rest of the files. It must match the value in the gbfs.json file.',
          type: 'string',
          pattern: '^[a-z]{2,3}(-[A-Z]{2})?$',
        },
        name: {
          description: 'Name of the system to be displayed to customers.',
          type: 'string',
        },
        short_name: {
          description: 'Optional abbreviation for a system.',
          type: 'string',
        },
        operator: {
          description: 'Name of the operator',
          type: 'string',
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
        phone_number: {
          description:
            "A single voice telephone number for the specified system that presents the telephone number as typical for the system's service area.",
          type: 'string',
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
        timezone: {
          description: 'The time zone where the system is located.',
          type: 'string',
          enum: TIMEZONE_IDENTIFIER_LIST,
        },
        license_url: {
          description:
            'A fully qualified URL of a page that defines the license terms for the GBFS data for this system.',
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
          description: 'A fully qualified URL pointing to the terms of service (added in v2.3-RC)',
          type: 'string',
          format: 'uri',
        },
        terms_last_updated: {
          description:
            'The date that the terms of service provided at terms_url were last updated (added in v2.3-RC)',
          type: 'string',
          format: 'date',
        },
        privacy_url: {
          description:
            'A fully qualified URL pointing to the privacy policy for the service (added in v2.3-RC).',
          type: 'string',
          format: 'uri',
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
      required: ['system_id', 'language', 'name', 'timezone'],
      dependentRequired: {
        terms_url: ['terms_last_updated'],
        privacy_url: ['privacy_last_updated'],
      },
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS System Information V2.3
 * Details including system operator, system location, year implemented, URL, contact info, and time zone.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_informationjson)
 */
export interface GBFSSystemInformationV23 {
  last_updated: number;
  ttl: number;
  version: '2.3';
  data: {
    system_id: string;
    language: string; // Matches BCP-47 language tags
    name: string;
    short_name?: string;
    operator?: string;
    url?: string;
    purchase_url?: string;
    start_date?: string; // ISO 8601 format
    phone_number?: string;
    email?: string;
    feed_contact_email?: string;
    timezone: TimeZone;
    license_url?: string;
    brand_assets?: {
      brand_last_modified: string; // ISO 8601 format
      brand_terms_url?: string;
      brand_image_url: string;
      brand_image_url_dark?: string;
      color?: string; // Hexadecimal color code
    };
    terms_url?: string;
    terms_last_updated?: string; // ISO 8601 format
    privacy_url?: string;
    privacy_last_updated?: string; // ISO 8601 format
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

/**
 * # GBFS System Information Schema V2.2
 * Details including system operator, system location, year implemented, URL, contact info, and time zone.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_informationjson)
 */
export const gbfsSystemInformationSchemaV22 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_informationjson',
  description:
    'Details including system operator, system location, year implemented, URL, contact info, and time zone.',
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
      properties: {
        system_id: {
          description:
            'Identifier for this vehicle share system. This should be globally unique (even between different systems).',
          type: 'string',
        },
        language: {
          description:
            'The language that will be used throughout the rest of the files. It must match the value in the gbfs.json file.',
          type: 'string',
          pattern: '^[a-z]{2,3}(-[A-Z]{2})?$',
        },
        name: {
          description: 'Name of the system to be displayed to customers.',
          type: 'string',
        },
        short_name: {
          description: 'Optional abbreviation for a system.',
          type: 'string',
        },
        operator: {
          description: 'Name of the operator.',
          type: 'string',
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
        phone_number: {
          description:
            "A single voice telephone number for the specified system that presents the telephone number as typical for the system's service area.",
          type: 'string',
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
        timezone: {
          description: 'The time zone where the system is located.',
          type: 'string',
          enum: TIMEZONE_IDENTIFIER_LIST,
        },
        license_url: {
          description:
            'A fully qualified URL of a page that defines the license terms for the GBFS data for this system.',
          type: 'string',
          format: 'uri',
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
      required: ['system_id', 'language', 'name', 'timezone'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS System Information Schema V2.2
 * Details including system operator, system location, year implemented, URL, contact info, and time zone.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_informationjson)
 */
export interface GBFSSystemInformationV22 {
  last_updated: number;
  ttl: number;
  version: '2.2';
  data: {
    system_id: string;
    language: string; // Matches BCP-47 language tags
    name: string;
    short_name?: string;
    operator?: string;
    url?: string;
    purchase_url?: string;
    start_date?: string; // ISO 8601 format
    phone_number?: string;
    email?: string;
    feed_contact_email?: string;
    timezone: TimeZone;
    license_url?: string;
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

/**
 * # GBFS System Information Schema V2.1
 * Details including system operator, system location, year implemented, URL, contact info, and time zone.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_informationjson)
 */
export const gbfsSystemInformationSchemaV21 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_informationjson',
  description:
    'Details including system operator, system location, year implemented, URL, contact info, and time zone.',
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
      properties: {
        system_id: {
          description:
            'Identifier for this vehicle share system. This should be globally unique (even between different systems).',
          type: 'string',
        },
        language: {
          description:
            'The language that will be used throughout the rest of the files. It must match the value in the gbfs.json file.',
          type: 'string',
          pattern: '^[a-z]{2,3}(-[A-Z]{2})?$',
        },
        name: {
          description: 'Name of the system to be displayed to customers.',
          type: 'string',
        },
        short_name: {
          description: 'Optional abbreviation for a system.',
          type: 'string',
        },
        operator: {
          description: 'Name of the operator.',
          type: 'string',
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
        phone_number: {
          description:
            "A single voice telephone number for the specified system that presents the telephone number as typical for the system's service area.",
          type: 'string',
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
        timezone: {
          description: 'The time zone where the system is located.',
          type: 'string',
          enum: TIMEZONE_IDENTIFIER_LIST,
        },
        license_url: {
          description:
            'A fully qualified URL of a page that defines the license terms for the GBFS data for this system.',
          type: 'string',
          format: 'uri',
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
      required: ['system_id', 'language', 'name', 'timezone'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS System Information Schema V2.1
 * Details including system operator, system location, year implemented, URL, contact info, and time zone.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_informationjson)
 */
export interface GBFSSystemInformationV21 {
  last_updated: number;
  ttl: number;
  version: '2.1';
  data: {
    system_id: string;
    language: string; // Matches BCP-47 language tags
    name: string;
    short_name?: string;
    operator?: string;
    url?: string;
    purchase_url?: string;
    start_date?: string; // ISO 8601 format
    phone_number?: string;
    email?: string;
    feed_contact_email?: string;
    timezone: TimeZone;
    license_url?: string;
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

/**
 * # GBFS System Information Schema V2.0
 * Details including system operator, system location, year implemented, URL, contact info, and time zone.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_informationjson)
 */
export const gbfsSystemInformationSchemaV20 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_informationjson',
  description:
    'Details including system operator, system location, year implemented, URL, contact info, and time zone.',
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
      properties: {
        system_id: {
          description:
            'Identifier for this vehicle share system. This should be globally unique (even between different systems).',
          type: 'string',
        },
        language: {
          description:
            'The language that will be used throughout the rest of the files. It must match the value in the gbfs.json file.',
          type: 'string',
          pattern: '^[a-z]{2,3}(-[A-Z]{2})?$',
        },
        name: {
          description: 'Name of the system to be displayed to customers.',
          type: 'string',
        },
        short_name: {
          description: 'Optional abbreviation for a system.',
          type: 'string',
        },
        operator: {
          description: 'Name of the operator.',
          type: 'string',
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
        phone_number: {
          description:
            "A single voice telephone number for the specified system that presents the telephone number as typical for the system's service area.",
          type: 'string',
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
        timezone: {
          description: 'The time zone where the system is located.',
          type: 'string',
          enum: TIMEZONE_IDENTIFIER_LIST,
        },
        license_url: {
          description:
            'A fully qualified URL of a page that defines the license terms for the GBFS data for this system.',
          type: 'string',
          format: 'uri',
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
      required: ['system_id', 'language', 'name', 'timezone'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS System Information Schema V2.0
 * Details including system operator, system location, year implemented, URL, contact info, and time zone.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_informationjson)
 */
export interface GBFSSystemInformationV20 {
  last_updated: number;
  ttl: number;
  version: '2.0';
  data: {
    system_id: string;
    language: string; // Matches BCP-47 language tags
    name: string;
    short_name?: string;
    operator?: string;
    url?: string;
    purchase_url?: string;
    start_date?: string; // ISO 8601 format
    phone_number?: string;
    email?: string;
    feed_contact_email?: string;
    timezone: TimeZone;
    license_url?: string;
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
