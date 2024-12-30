import type { MultiPolygonGeometry } from '../../..';

/**
 * # GBFS Station Information Schema V2.3, V2.2, V2.1, OR V2.0
 * List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
 *
 * ## Links
 * - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#station_informationjson)
 * - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#station_informationjson)
 * - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#station_informationjson)
 * - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#station_informationjson)
 */
export type GBFSStationInformationV2 =
  | GBFSStationInformationV23
  | GBFSStationInformationV22
  | GBFSStationInformationV21
  | GBFSStationInformationV20;

/**
 * # GBFS Station Information Schema V2.3
 * List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#station_informationjson)
 */
export const gbfsStationInformationSchemaV23 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#station_informationjson',
  description:
    'List of all stations, their capacities and locations. REQUIRED of systems utilizing docks.',
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
      description: 'Array that contains one object per station as defined below.',
      type: 'object',
      properties: {
        stations: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              station_id: {
                description: 'Identifier of a station.',
                type: 'string',
              },
              name: {
                description: 'Public name of the station.',
                type: 'string',
              },
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
                description: 'Address where station is located.',
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
                description: 'Postal code where station is located.',
                type: 'string',
              },
              rental_methods: {
                description: 'Payment methods accepted at this station.',
                type: 'array',
                items: {
                  type: 'string',
                  enum: [
                    'key',
                    'creditcard',
                    'paypass',
                    'applepay',
                    'androidpay',
                    'transitcard',
                    'accountnumber',
                    'phone',
                  ],
                },
                minItems: 1,
              },
              is_virtual_station: {
                description:
                  'Is this station a location with or without physical infrastructure? (added in v2.1-RC)',
                type: 'boolean',
              },
              station_area: {
                description:
                  'A multipolygon that describes the area of a virtual station (added in v2.1-RC).',
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
              parking_type: {
                description: 'Type of parking station. Added in v2.3',
                type: 'string',
                enum: [
                  'parking_lot',
                  'street_parking',
                  'underground_parking',
                  'sidewalk_parking',
                  'other',
                ],
              },
              parking_hoop: {
                description: 'Are parking hoops present at this station? Added in v2.3',
                type: 'boolean',
              },
              contact_phone: {
                description: 'Contact phone of the station. Added in v2.3',
                type: 'string',
              },
              capacity: {
                description:
                  'Number of total docking points installed at this station, both available and unavailable.',
                type: 'integer',
                minimum: 0,
              },
              vehicle_capacity: {
                description:
                  'An object where each key is a vehicle_type_id and the value is a number presenting the total number of vehicles of this type that can park within the station_area (added in v2.1-RC).',
                type: 'object',
                additionalProperties: {
                  type: 'number',
                },
              },
              is_valet_station: {
                description: 'Are valet services provided at this station? (added in v2.1-RC)',
                type: 'boolean',
              },
              is_charging_station: {
                description:
                  'Does the station support charging of electric vehicles? (added in v2.3-RC)',
                type: 'boolean',
              },
              rental_uris: {
                description:
                  'Contains rental URIs for Android, iOS, and web in the android, ios, and web fields (added in v1.1).',
                type: 'object',
                properties: {
                  android: {
                    description:
                      'URI that can be passed to an Android app with an intent (added in v1.1).',
                    type: 'string',
                    format: 'uri',
                  },
                  ios: {
                    description:
                      'URI that can be used on iOS to launch the rental app for this station (added in v1.1).',
                    type: 'string',
                    format: 'uri',
                  },
                  web: {
                    description:
                      'URL that can be used by a web browser to show more information about renting a vehicle at this station (added in v1.1).',
                    type: 'string',
                    format: 'uri',
                  },
                },
              },
              vehicle_type_capacity: {
                description:
                  'An object where each key is a vehicle_type_id and the value is a number representing the total docking points installed at this station for each vehicle type (added in v2.1-RC).',
                type: 'object',
                additionalProperties: {
                  type: 'number',
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
 * # GBFS Station Information V2.3
 * List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#station_informationjson)
 */
export interface GBFSStationInformationV23 {
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
   * Contains station information for the system.
   */
  data: {
    /**
     * Array of station objects.
     */
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
        | 'key'
        | 'creditcard'
        | 'paypass'
        | 'applepay'
        | 'androidpay'
        | 'transitcard'
        | 'accountnumber'
        | 'phone'
      >;
      is_virtual_station?: boolean;
      station_area?: MultiPolygonGeometry;
      parking_type?:
        | 'parking_lot'
        | 'street_parking'
        | 'underground_parking'
        | 'sidewalk_parking'
        | 'other';
      parking_hoop?: boolean;
      contact_phone?: string;
      capacity?: number;
      vehicle_capacity?: Record<string, number>;
      is_valet_station?: boolean;
      is_charging_station?: boolean;
      rental_uris?: {
        android?: string;
        ios?: string;
        web?: string;
      };
      vehicle_type_capacity?: Record<string, number>;
    }>;
  };
}

/**
 * # GBFS Station Information Schema V2.2
 * List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#station_informationjson)
 */
export const gbfsStationInformationSchemaV22 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#station_informationjson',
  description:
    'List of all stations, their capacities and locations. REQUIRED of systems utilizing docks.',
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
      description: 'Array that contains one object per station as defined below.',
      type: 'object',
      properties: {
        stations: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              station_id: {
                description: 'Identifier of a station.',
                type: 'string',
              },
              name: {
                description: 'Public name of the station.',
                type: 'string',
              },
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
                description: 'Address where station is located.',
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
                description: 'Postal code where station is located.',
                type: 'string',
              },
              rental_methods: {
                description: 'Payment methods accepted at this station.',
                type: 'array',
                items: {
                  type: 'string',
                  enum: [
                    'key',
                    'creditcard',
                    'paypass',
                    'applepay',
                    'androidpay',
                    'transitcard',
                    'accountnumber',
                    'phone',
                  ],
                },
                minItems: 1,
              },
              is_virtual_station: {
                description:
                  'Is this station a location with or without physical infrastructure? (added in v2.1-RC)',
                type: 'boolean',
              },
              station_area: {
                description:
                  'A multipolygon that describes the area of a virtual station (added in v2.1-RC).',
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
              capacity: {
                description:
                  'Number of total docking points installed at this station, both available and unavailable.',
                type: 'integer',
                minimum: 0,
              },
              vehicle_capacity: {
                description:
                  'An object where each key is a vehicle_type_id and the value is a number presenting the total number of vehicles of this type that can park within the station_area (added in v2.1-RC).',
                type: 'object',
                additionalProperties: {
                  type: 'number',
                },
              },
              is_valet_station: {
                description: 'Are valet services provided at this station? (added in v2.1-RC)',
                type: 'boolean',
              },
              rental_uris: {
                description:
                  'Contains rental URIs for Android, iOS, and web in the android, ios, and web fields (added in v1.1).',
                type: 'object',
                properties: {
                  android: {
                    description:
                      'URI that can be passed to an Android app with an intent (added in v1.1).',
                    type: 'string',
                    format: 'uri',
                  },
                  ios: {
                    description:
                      'URI that can be used on iOS to launch the rental app for this station (added in v1.1).',
                    type: 'string',
                    format: 'uri',
                  },
                  web: {
                    description:
                      'URL that can be used by a web browser to show more information about renting a vehicle at this station (added in v1.1).',
                    type: 'string',
                    format: 'uri',
                  },
                },
              },
              vehicle_type_capacity: {
                description:
                  'An object where each key is a vehicle_type_id and the value is a number representing the total docking points installed at this station for each vehicle type (added in v2.1-RC).',
                type: 'object',
                additionalProperties: {
                  type: 'number',
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
 * # GBFS Station Information V2.2
 * List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#station_informationjson)
 */
export interface GBFSStationInformationV22 {
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
   * Contains station information for the system.
   */
  data: {
    /**
     * Array of station objects.
     */
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
        | 'key'
        | 'creditcard'
        | 'paypass'
        | 'applepay'
        | 'androidpay'
        | 'transitcard'
        | 'accountnumber'
        | 'phone'
      >;
      is_virtual_station?: boolean;
      station_area?: MultiPolygonGeometry;
      capacity?: number;
      vehicle_capacity?: Record<string, number>;
      is_valet_station?: boolean;
      rental_uris?: {
        android?: string;
        ios?: string;
        web?: string;
      };
      vehicle_type_capacity?: Record<string, number>;
    }>;
  };
}

/**
 * # GBFS Station Information Schema V2.1
 * List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#station_informationjson)
 */
export const gbfsStationInformationSchemaV21 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#station_informationjson',
  description:
    'List of all stations, their capacities and locations. REQUIRED of systems utilizing docks.',
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
      description: 'Array that contains one object per station as defined below.',
      type: 'object',
      properties: {
        stations: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              station_id: {
                description: 'Identifier of a station.',
                type: 'string',
              },
              name: {
                description: 'Public name of the station.',
                type: 'string',
              },
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
                description: 'Address where station is located.',
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
                description: 'Postal code where station is located.',
                type: 'string',
              },
              rental_methods: {
                description: 'Payment methods accepted at this station.',
                type: 'array',
                items: {
                  type: 'string',
                  enum: [
                    'key',
                    'creditcard',
                    'paypass',
                    'applepay',
                    'androidpay',
                    'transitcard',
                    'accountnumber',
                    'phone',
                  ],
                },
                minItems: 1,
              },
              is_virtual_station: {
                description:
                  'Is this station a location with or without physical infrastructure? (added in v2.1-RC)',
                type: 'boolean',
              },
              station_area: {
                description:
                  'A multipolygon that describes the area of a virtual station (added in v2.1-RC).',
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
              capacity: {
                description:
                  'Number of total docking points installed at this station, both available and unavailable.',
                type: 'integer',
                minimum: 0,
              },
              vehicle_capacity: {
                description:
                  'An object where each key is a vehicle_type_id and the value is a number presenting the total number of vehicles of this type that can park within the station_area (added in v2.1-RC).',
                type: 'object',
                additionalProperties: {
                  type: 'number',
                },
              },
              is_valet_station: {
                description: 'Are valet services provided at this station? (added in v2.1-RC)',
                type: 'boolean',
              },
              rental_uris: {
                description:
                  'Contains rental URIs for Android, iOS, and web in the android, ios, and web fields (added in v1.1).',
                type: 'object',
                properties: {
                  android: {
                    description:
                      'URI that can be passed to an Android app with an intent (added in v1.1).',
                    type: 'string',
                    format: 'uri',
                  },
                  ios: {
                    description:
                      'URI that can be used on iOS to launch the rental app for this station (added in v1.1).',
                    type: 'string',
                    format: 'uri',
                  },
                  web: {
                    description:
                      'URL that can be used by a web browser to show more information about renting a vehicle at this station (added in v1.1).',
                    type: 'string',
                    format: 'uri',
                  },
                },
              },
              vehicle_type_capacity: {
                description:
                  'An object where each key is a vehicle_type_id and the value is a number representing the total docking points installed at this station for each vehicle type (added in v2.1-RC).',
                type: 'object',
                additionalProperties: {
                  type: 'number',
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
 * # GBFS Station Information V2.1
 * List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#station_informationjson)
 */
export interface GBFSStationInformationV21 {
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
   * Contains station information for the system.
   */
  data: {
    /**
     * Array of station objects.
     */
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
        | 'key'
        | 'creditcard'
        | 'paypass'
        | 'applepay'
        | 'androidpay'
        | 'transitcard'
        | 'accountnumber'
        | 'phone'
      >;
      is_virtual_station?: boolean;
      station_area?: MultiPolygonGeometry;
      capacity?: number;
      vehicle_capacity?: Record<string, number>;
      is_valet_station?: boolean;
      rental_uris?: {
        android?: string;
        ios?: string;
        web?: string;
      };
      vehicle_type_capacity?: Record<string, number>;
    }>;
  };
}

/**
 * # GBFS Station Information Schema V2.0
 * List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#station_informationjson)
 */
export const gbfsStationInformationSchemaV20 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#station_informationjson',
  description:
    'List of all stations, their capacities and locations. REQUIRED of systems utilizing docks.',
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
      description: 'Array that contains one object per station as defined below.',
      type: 'object',
      properties: {
        stations: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              station_id: {
                description: 'Identifier of a station.',
                type: 'string',
              },
              name: {
                description: 'Public name of the station.',
                type: 'string',
              },
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
                description: 'Address where station is located.',
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
                description: 'Postal code where station is located.',
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
                  'Contains rental URIs for Android, iOS, and web in the android, ios, and web fields (added in v1.1).',
                type: 'object',
                properties: {
                  android: {
                    description:
                      'URI that can be passed to an Android app with an intent (added in v1.1).',
                    type: 'string',
                    format: 'uri',
                  },
                  ios: {
                    description:
                      'URI that can be used on iOS to launch the rental app for this station (added in v1.1).',
                    type: 'string',
                    format: 'uri',
                  },
                  web: {
                    description:
                      'URL that can be used by a web browser to show more information about renting a vehicle at this station (added in v1.1).',
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
 * # GBFS Station Information V2.0
 * List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#station_informationjson)
 */
export interface GBFSStationInformationV20 {
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
   * Contains station information for the system.
   */
  data: {
    /**
     * Array of station objects.
     */
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
