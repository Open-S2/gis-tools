import type { MultiPolygonGeometry, Properties } from '../../..';

/**
 * # GBFS Station Information Schema V3.1-RC & V3.0
 * List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#station_informationjson)
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#station_informationjson)
 */
export type GBFSStationInformationV3 = GBFSStationInformationV31RC | GBFSStationInformationV30;

/**
 * # GBFS Station Information Schema V3.1-RC
 * List of all stations, their capacities and locations. REQUIRED of systems utilizing docks.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#station_informationjson)
 */
export const gbfsStationInformationSchemaV31RC = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#station_informationjson',
  description:
    'List of all stations, their capacities and locations. REQUIRED of systems utilizing docks.',
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
              short_name: {
                description: 'Short name or other type of identifier.',
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
              lat: {
                description: 'The latitude of the station.',
                type: 'number',
                minimum: -90,
                maximum: 90,
              },
              lon: {
                description: 'The longitude fo the station.',
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
              station_opening_hours: {
                description: 'Hours of operation for the station in OSM opening_hours format.',
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
              vehicle_types_capacity: {
                description:
                  "This field's value is an array of objects containing the keys vehicle_type_ids and count defined below. These objects are used to model the parking capacity of virtual stations (defined using the is_virtual_station field) for each vehicle type that can be returned to this station.",
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    vehicle_type_ids: {
                      description:
                        'The vehicle_type_ids, as defined in vehicle_types.json, that may park at the virtual station.',
                      type: 'array',
                      items: {
                        type: 'string',
                      },
                    },
                    count: {
                      description:
                        'A number representing the total number of vehicles of the specified vehicle_type_ids that can park within the virtual station.',
                      type: 'integer',
                      minimum: 0,
                    },
                  },
                  required: ['vehicle_type_ids', 'count'],
                },
              },
              vehicle_docks_capacity: {
                description:
                  "This field's value is an array of objects containing the keys vehicle_type_ids and count defined below. These objects are used to model the total docking capacity of a station, both available and unavailable, for each type of vehicle that may dock at this station.",
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    vehicle_type_ids: {
                      description:
                        'An array of strings where each string represents a vehicle_type_id that is able to use a particular type of dock at the station.',
                      type: 'array',
                      items: {
                        type: 'string',
                      },
                    },
                    count: {
                      description:
                        'A number representing the total number of docks at the station, both available and unavailable, that may accept the vehicle types specified by vehicle_type_ids.',
                      type: 'integer',
                      minimum: 0,
                    },
                  },
                  required: ['vehicle_type_ids', 'count'],
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
                  'Contains rental uris for Android, iOS, and web in the android, ios, and web fields (added in v1.1).',
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
 * Information about a single station.
 */
export interface GBFSStationV3 extends Properties {
  /**
   * Identifier of the station.
   */
  station_id: string;

  /**
   * Public name of the station.
   */
  name: Array<{
    /**
     * The translated text.
     */
    text: string;

    /**
     * IETF BCP 47 language code.
     * **Pattern**: ^[a-z]{2,3}(-[A-Z]{2})?$
     */
    language: string;
  }>;

  /**
   * The latitude of the station.
   * **Minimum**: -90
   * **Maximum**: 90
   */
  lat: number;

  /**
   * The longitude of the station.
   * **Minimum**: -180
   * **Maximum**: 180
   */
  lon: number;

  /**
   * Short name or alternative identifier for the station.
   */
  // @ts-expect-error - allow for now
  short_name?: Array<{
    text: string;
    language: string;
  }>;

  /**
   * Address where the station is located.
   */
  // @ts-expect-error - allow for now
  address?: string;

  /**
   * Cross street or landmark where the station is located.
   */
  // @ts-expect-error - allow for now
  cross_street?: string;

  /**
   * Identifier of the region where the station is located.
   */
  // @ts-expect-error - allow for now
  region_id?: string;

  /**
   * Postal code where the station is located.
   */
  // @ts-expect-error - allow for now
  post_code?: string;

  /**
   * Hours of operation for the station in OSM opening_hours format.
   */
  // @ts-expect-error - allow for now
  station_opening_hours?: string;

  /**
   * Payment methods accepted at the station.
   * **Enum**: ['key', 'creditcard', 'paypass', 'applepay', 'androidpay', 'transitcard', 'accountnumber', 'phone']
   */
  // @ts-expect-error - allow for now
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

  /**
   * Is this station a location with or without physical infrastructure? (added in v2.1-RC)
   */
  // @ts-expect-error - allow for now
  is_virtual_station?: boolean;

  /**
   * A multipolygon describing the area of a virtual station. (added in v2.1-RC)
   */
  // @ts-expect-error - allow for now
  station_area?: MultiPolygonGeometry;

  /**
   * Type of parking station. (added in v2.3)
   * **Enum**: ['parking_lot', 'street_parking', 'underground_parking', 'sidewalk_parking', 'other']
   */
  // @ts-expect-error - allow for now
  parking_type?:
    | 'parking_lot'
    | 'street_parking'
    | 'underground_parking'
    | 'sidewalk_parking'
    | 'other';

  /**
   * Are parking hoops present at this station? (added in v2.3)
   */
  // @ts-expect-error - allow for now
  parking_hoop?: boolean;

  /**
   * Contact phone of the station. (added in v2.3)
   */
  // @ts-expect-error - allow for now
  contact_phone?: string;

  /**
   * Total docking points installed at the station, both available and unavailable.
   * **Minimum**: 0
   */
  // @ts-expect-error - allow for now
  capacity?: number;

  /**
   * Parking capacity for virtual stations per vehicle type.
   */
  // @ts-expect-error - allow for now
  vehicle_types_capacity?: Array<{
    vehicle_type_ids: string[];
    count: number;
  }>;

  /**
   * Docking capacity per vehicle type at the station.
   */
  // @ts-expect-error - allow for now
  vehicle_docks_capacity?: Array<{
    vehicle_type_ids: string[];
    count: number;
  }>;

  /**
   * Are valet services provided at the station? (added in v2.1-RC)
   */
  // @ts-expect-error - allow for now
  is_valet_station?: boolean;

  /**
   * Does the station support charging of electric vehicles? (added in v2.3-RC)
   */
  // @ts-expect-error - allow for now
  is_charging_station?: boolean;

  /**
   * Rental URIs for Android, iOS, and web.
   */
  // @ts-expect-error - allow for now
  rental_uris?: {
    /**
     * URI for Android apps. (added in v1.1)
     * **Format**: uri
     */
    android?: string;

    /**
     * URI for iOS apps. (added in v1.1)
     * **Format**: uri
     */
    ios?: string;

    /**
     * URL for web browsers. (added in v1.1)
     * **Format**: uri
     */
    web?: string;
  };
}

/**
 * # GBFS Station Information Schema V3.1-RC
 * List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#station_informationjson)
 */
export interface GBFSStationInformationV31RC {
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
   * Contains station data for the system.
   */
  data: {
    /**
     * Array of stations, each containing location, capacity, and other metadata.
     */
    stations: GBFSStationV3[];
  };
}

/**
 * # GBFS Station Information Schema V3.0
 * List of all stations, their capacities and locations. REQUIRED of systems utilizing docks.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#station_informationjson)
 */
export const gbfsStationInformationSchemaV30 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#station_informationjson',
  description:
    'List of all stations, their capacities and locations. REQUIRED of systems utilizing docks.',
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
              short_name: {
                description: 'Short name or other type of identifier.',
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
              lat: {
                description: 'The latitude of the station.',
                type: 'number',
                minimum: -90,
                maximum: 90,
              },
              lon: {
                description: 'The longitude fo the station.',
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
              station_opening_hours: {
                description: 'Hours of operation for the station in OSM opening_hours format.',
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
              vehicle_types_capacity: {
                description:
                  "This field's value is an array of objects containing the keys vehicle_type_ids and count defined below. These objects are used to model the parking capacity of virtual stations (defined using the is_virtual_station field) for each vehicle type that can be returned to this station.",
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    vehicle_type_ids: {
                      description:
                        'The vehicle_type_ids, as defined in vehicle_types.json, that may park at the virtual station.',
                      type: 'array',
                      items: {
                        type: 'string',
                      },
                    },
                    count: {
                      description:
                        'A number representing the total number of vehicles of the specified vehicle_type_ids that can park within the virtual station.',
                      type: 'integer',
                      minimum: 0,
                    },
                  },
                  required: ['vehicle_type_ids', 'count'],
                },
              },
              vehicle_docks_capacity: {
                description:
                  "This field's value is an array of objects containing the keys vehicle_type_ids and count defined below. These objects are used to model the total docking capacity of a station, both available and unavailable, for each type of vehicle that may dock at this station.",
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    vehicle_type_ids: {
                      description:
                        'An array of strings where each string represents a vehicle_type_id that is able to use a particular type of dock at the station.',
                      type: 'array',
                      items: {
                        type: 'string',
                      },
                    },
                    count: {
                      description:
                        'A number representing the total number of docks at the station, both available and unavailable, that may accept the vehicle types specified by vehicle_type_ids.',
                      type: 'integer',
                      minimum: 0,
                    },
                  },
                  required: ['vehicle_type_ids', 'count'],
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
                  'Contains rental uris for Android, iOS, and web in the android, ios, and web fields (added in v1.1).',
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
 * # GBFS Station Information Schema V3.0
 * List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#station_informationjson)
 */
export interface GBFSStationInformationV30 {
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
   * GBFS version number to which the feed conforms.
   * **Const**: '3.0'
   */
  version: '3.0';

  /**
   * Data object containing station information.
   */
  data: {
    /**
     * List of stations with their attributes.
     */
    stations: GBFSStationV3[];
  };
}
