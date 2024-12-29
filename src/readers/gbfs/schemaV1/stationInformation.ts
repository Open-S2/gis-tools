/**
 * # GBFS Station Information Schema V1.1 OR GBFS Station Information Schema V1.0
 * List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
 *
 * ## Links
 * - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#station_informationjson)
 * - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#station_informationjson)
 */
export type GBFSStationInformationV1 = GBFSStationInformationV11 | GBFSStationInformationV10;

/**
 * # GBFS Station Information Schema V1.1
 * List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
 *
 * ## Links
 * - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#station_informationjson)
 */
export const gbfsStationInformationSchemaV11 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#station_informationjson',
  description:
    'List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.',
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
      description: 'Array that contains one object per station as defined below.',
      type: 'object',
      properties: {
        stations: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              station_id: { description: 'Identifier of a station.', type: 'string' },
              name: { description: 'Public name of the station.', type: 'string' },
              short_name: {
                description: 'Short name or other type of identifier.',
                type: 'string',
              },
              lat: {
                description: 'The latitude of the station.',
                type: 'number',
                minimum: -90,
                maximum: 90,
              },
              lon: {
                description: 'The longitude of the station.',
                type: 'number',
                minimum: -180,
                maximum: 180,
              },
              address: {
                description: 'Address where the station is located.',
                type: 'string',
              },
              cross_street: {
                description: 'Cross street or landmark where the station is located.',
                type: 'string',
              },
              region_id: {
                description: 'Identifier of the region where the station is located.',
                type: 'string',
              },
              post_code: {
                description: 'Postal code where the station is located.',
                type: 'string',
              },
              rental_methods: {
                description: 'Payment methods accepted at this station.',
                type: 'array',
                items: {
                  type: 'string',
                  enum: [
                    'KEY',
                    'CREDITCARD',
                    'PAYPASS',
                    'APPLEPAY',
                    'ANDROIDPAY',
                    'TRANSITCARD',
                    'ACCOUNTNUMBER',
                    'PHONE',
                  ],
                },
                minItems: 1,
              },
              capacity: {
                description:
                  'Number of total docking points installed at this station, both available and unavailable.',
                type: 'integer',
                minimum: 0,
              },
              rental_uris: {
                description:
                  'Contains rental uris for Android, iOS, and web in the android, ios, and web fields (added in v1.1).',
                type: 'object',
                properties: {
                  android: {
                    description: 'URI for Android app intent.',
                    type: 'string',
                    format: 'uri',
                  },
                  ios: {
                    description: 'URI for iOS rental app.',
                    type: 'string',
                    format: 'uri',
                  },
                  web: {
                    description: 'Web URL for renting a vehicle at this station.',
                    type: 'string',
                    format: 'uri',
                  },
                },
              },
            },
            required: ['station_id', 'name', 'lat', 'lon'],
          },
        },
      },
      required: ['stations'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * GBFS Station Information Schema V1.1 Interface
 */
export interface GBFSStationInformationV11 {
  /** Last time the data in the feed was updated in POSIX time. */
  last_updated: number;
  /** Number of seconds before the data in the feed will be updated again. */
  ttl: number;
  /** GBFS version number (1.1). */
  version: '1.1';
  /** Data containing an array of stations. */
  data: {
    stations: Array<{
      station_id: string;
      name: string;
      short_name?: string;
      lat: number;
      lon: number;
      address?: string;
      cross_street?: string;
      region_id?: string;
      post_code?: string;
      rental_methods?: Array<
        | 'KEY'
        | 'CREDITCARD'
        | 'PAYPASS'
        | 'APPLEPAY'
        | 'ANDROIDPAY'
        | 'TRANSITCARD'
        | 'ACCOUNTNUMBER'
        | 'PHONE'
      >;
      capacity?: number;
      rental_uris?: {
        android?: string;
        ios?: string;
        web?: string;
      };
    }>;
  };
}

/**
 * # GBFS Station Information Schema V1.0
 * List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
 *
 * ## Links
 * - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#station_informationjson)
 */
export const gbfsStationInformationSchemaV10 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#station_informationjson',
  description:
    'List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.',
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
      description: 'Array that contains one object per station as defined below.',
      type: 'object',
      properties: {
        stations: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              station_id: { description: 'Identifier of a station.', type: 'string' },
              name: { description: 'Public name of the station.', type: 'string' },
              short_name: {
                description: 'Short name or other type of identifier.',
                type: 'string',
              },
              lat: {
                description: 'The latitude of the station.',
                type: 'number',
                minimum: -90,
                maximum: 90,
              },
              lon: {
                description: 'The longitude of the station.',
                type: 'number',
                minimum: -180,
                maximum: 180,
              },
              address: {
                description: 'Address where the station is located.',
                type: 'string',
              },
              cross_street: {
                description: 'Cross street or landmark where the station is located.',
                type: 'string',
              },
              region_id: {
                description: 'Identifier of the region where the station is located.',
                type: 'string',
              },
              post_code: {
                description: 'Postal code where the station is located.',
                type: 'string',
              },
              rental_methods: {
                description: 'Payment methods accepted at this station.',
                type: 'array',
                items: {
                  type: 'string',
                  enum: [
                    'KEY',
                    'CREDITCARD',
                    'PAYPASS',
                    'APPLEPAY',
                    'ANDROIDPAY',
                    'TRANSITCARD',
                    'ACCOUNTNUMBER',
                    'PHONE',
                  ],
                },
              },
              capacity: {
                description:
                  'Number of total docking points installed at this station, both available and unavailable.',
                type: 'integer',
                minimum: 0,
              },
            },
            required: ['station_id', 'name', 'lat', 'lon'],
          },
        },
      },
      required: ['stations'],
    },
  },
  required: ['last_updated', 'ttl', 'data'],
};

/**
 * GBFS Station Information Schema V1.0 Interface
 */
export interface GBFSStationInformationV10 {
  /** Last time the data in the feed was updated in POSIX time. */
  last_updated: number;
  /** Number of seconds before the data in the feed will be updated again. */
  ttl: number;
  /** Data containing an array of stations. */
  data: {
    stations: Array<{
      station_id: string;
      name: string;
      short_name?: string;
      lat: number;
      lon: number;
      address?: string;
      cross_street?: string;
      region_id?: string;
      post_code?: string;
      rental_methods?: Array<
        | 'KEY'
        | 'CREDITCARD'
        | 'PAYPASS'
        | 'APPLEPAY'
        | 'ANDROIDPAY'
        | 'TRANSITCARD'
        | 'ACCOUNTNUMBER'
        | 'PHONE'
      >;
      capacity?: number;
    }>;
  };
}
