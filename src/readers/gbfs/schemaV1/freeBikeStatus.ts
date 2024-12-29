/**
 * # Free Bike Status Schema V1.1 OR Free Bike Status Schema V1.0
 * Describes the vehicles that are available for rent.
 *
 * ## Links
 * - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#free_bike_statusjson)
 * - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#free_bike_statusjson)
 */
export type GBFSFreeBikeStatusV1 = GBFSFreeBikeStatusV11 | GBFSFreeBikeStatusV10;

/**
 * # Free Bike Status Schema V1.1
 * Describes the vehicles that are not at a station and are available for rent.
 *
 * ## Links
 * - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#free_bike_statusjson)
 */
export const gbfsFreeBikeStatusSchemaV11 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#free_bike_statusjson',
  description: 'Describes the vehicles that are not at a station and are available for rent.',
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
      description: 'Array that contains one object per bike as defined below.',
      type: 'object',
      properties: {
        bikes: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              bike_id: { description: 'Identifier of a vehicle.', type: 'string' },
              lat: {
                description: 'The latitude of the vehicle.',
                type: 'number',
                minimum: -90,
                maximum: 90,
              },
              lon: {
                description: 'The longitude of the vehicle.',
                type: 'number',
                minimum: -180,
                maximum: 180,
              },
              is_reserved: {
                description: 'Is the vehicle currently reserved?',
                type: 'number',
                minimum: 0,
                maximum: 1,
              },
              is_disabled: {
                description: 'Is the vehicle currently disabled (broken)?',
                type: 'number',
                minimum: 0,
                maximum: 1,
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
                  ios: { description: 'URI for iOS rental app.', type: 'string', format: 'uri' },
                  web: {
                    description: 'Web URL for renting this vehicle.',
                    type: 'string',
                    format: 'uri',
                  },
                },
              },
            },
            required: ['bike_id', 'lat', 'lon', 'is_reserved', 'is_disabled'],
          },
        },
      },
      required: ['bikes'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * Free Bike Status Schema V1.1 Interface
 */
export interface GBFSFreeBikeStatusV11 {
  /** Last time the data in the feed was updated in POSIX time. */
  last_updated: number;

  /** Number of seconds before the data in the feed will be updated again. */
  ttl: number;

  /** GBFS version number (1.1). */
  version: '1.1';

  /** Data containing an array of bikes. */
  data: {
    bikes: Array<{
      bike_id: string;
      lat: number;
      lon: number;
      is_reserved: 0 | 1;
      is_disabled: 0 | 1;
      rental_uris?: {
        android?: string;
        ios?: string;
        web?: string;
      };
    }>;
  };
}

/**
 * # Free Bike Status Schema V1.0
 * Describes the vehicles that are available for rent.
 *
 * ## Links
 * - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#free_bike_statusjson)
 */
export const gbfsFreeBikeStatusSchemaV10 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#free_bike_statusjson',
  description: 'Describes the vehicles that are available for rent.',
  type: 'object',
  properties: {
    last_updated: {
      description: 'Last time the data in the feed was updated in POSIX time.',
      type: 'integer',
      minimum: 0,
      maximum: 1924988399,
    },
    ttl: {
      description: 'Number of seconds before the data in the feed will be updated again.',
      type: 'integer',
      minimum: 0,
    },
    data: {
      description: 'Array that contains one object per bike as defined below.',
      type: 'object',
      properties: {
        bikes: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              bike_id: { description: 'Identifier of a vehicle.', type: 'string' },
              lat: {
                description: 'The latitude of the vehicle.',
                type: 'number',
                minimum: -90,
                maximum: 90,
              },
              lon: {
                description: 'The longitude of the vehicle.',
                type: 'number',
                minimum: -180,
                maximum: 180,
              },
              is_reserved: {
                description: 'Is the vehicle currently reserved?',
                oneOf: [{ type: 'boolean' }, { type: 'number' }],
              },
              is_disabled: {
                description: 'Is the vehicle currently disabled (broken)?',
                oneOf: [{ type: 'boolean' }, { type: 'number' }],
              },
            },
            required: ['bike_id', 'lat', 'lon', 'is_reserved', 'is_disabled'],
          },
        },
      },
      required: ['bikes'],
    },
  },
  required: ['last_updated', 'ttl', 'data'],
};

/**
 * Free Bike Status Schema V1.0 Interface
 */
export interface GBFSFreeBikeStatusV10 {
  /** Last time the data in the feed was updated in POSIX time. */
  last_updated: number;

  /** Number of seconds before the data in the feed will be updated again. */
  ttl: number;

  /** Data containing an array of bikes. */
  data: {
    bikes: Array<{
      bike_id: string;
      lat: number;
      lon: number;
      is_reserved: boolean | number;
      is_disabled: boolean | number;
    }>;
  };
}
