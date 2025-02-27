import type { FeatureCollection, MValue, Properties, ValueArrayObject } from '../../..';

/**
 * # GBFS Geofencing Zones Schema V2.3, V2.2, V2.1, OR V2.0
 * Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#geofencing_zonesjson)
 * - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#geofencing_zonesjson)
 * - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#geofencing_zonesjson)
 */
export type GBFSGeofencingZonesV2 =
  | GBFSGeofencingZonesV23
  | GBFSGeofencingZonesV22
  | GBFSGeofencingZonesV21;

/**
 * # GBFS Geofencing Zones Schema V2.3
 * Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#geofencing_zonesjson)
 */
export const gbfsGeofencingZonesSchemaV23 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#geofencing_zonesjson',
  description:
    'Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).',
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
      description: 'Array that contains geofencing information for the system.',
      type: 'object',
      properties: {
        geofencing_zones: {
          type: 'object',
          description:
            'Each geofenced zone and its associated rules and attributes is described as an object within the array of features.',
          properties: {
            type: {
              description: 'FeatureCollection as per IETF RFC 7946.',
              type: 'string',
              enum: ['FeatureCollection'],
            },
            features: {
              description: 'Array of objects.',
              type: 'array',
              items: {
                title: 'GeoJSON Feature',
                type: 'object',
                properties: {
                  type: {
                    type: 'string',
                    enum: ['Feature'],
                  },
                  properties: {
                    description: 'Describing travel allowances and limitations.',
                    type: 'object',
                    properties: {
                      name: {
                        description: 'Public name of the geofencing zone.',
                        type: 'string',
                      },
                      start: {
                        description: 'Start time of the geofencing zone in POSIX time.',
                        type: 'integer',
                        minimum: 1450155600,
                      },
                      end: {
                        description: 'End time of the geofencing zone in POSIX time.',
                        type: 'integer',
                        minimum: 1450155600,
                      },
                      rules: {
                        description: 'Array that contains one object per rule.',
                        type: 'array',
                        items: {
                          type: 'object',
                          properties: {
                            vehicle_type_id: {
                              type: 'array',
                              description:
                                'Array of vehicle type IDs for which these restrictions apply.',
                              items: { type: 'string' },
                            },
                            ride_allowed: {
                              description:
                                'Is the undocked ride allowed to start and end in this zone?',
                              type: 'boolean',
                            },
                            ride_through_allowed: {
                              description: 'Is the ride allowed to travel through this zone?',
                              type: 'boolean',
                            },
                            maximum_speed_kph: {
                              description:
                                'What is the maximum speed allowed, in kilometers per hour?',
                              type: 'integer',
                              minimum: 0,
                            },
                            station_parking: {
                              description:
                                'Vehicle MUST be parked at stations defined in station_information.json within this geofence zone.',
                              type: 'boolean',
                            },
                          },
                          required: ['ride_allowed', 'ride_through_allowed'],
                        },
                      },
                    },
                  },
                  geometry: {
                    description:
                      'A polygon that describes where rides might not be able to start, end, go through, or have other limitations. Must follow the right-hand rule.',
                    title: 'GeoJSON MultiPolygon',
                    type: 'object',
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
                    required: ['type', 'coordinates'],
                  },
                },
                required: ['type', 'geometry', 'properties'],
              },
            },
          },
          required: ['type', 'features'],
        },
      },
      required: ['geofencing_zones'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * GBFS V3: Restrictions that apply within the area of the polygon.
 */
export interface GBFSGeofencingZonesV2PropertiesRule extends ValueArrayObject {
  /**
   * Array of vehicle type IDs for which these restrictions apply.
   */
  // @ts-expect-error - we need to clean this / remove it
  vehicle_type_id?: string[];
  /**
   * Is the undocked ride allowed to start and end in this zone?
   */
  ride_allowed: boolean;
  /**
   * Is the ride allowed to travel through this zone?
   */
  ride_through_allowed: boolean;
  /**
   * Maximum speed allowed, in kilometers per hour.
   * **Minimum**: 0
   */
  maximum_speed_kph?: number;
  /**
   * Vehicle MUST be parked at stations defined in station_information.json within this geofence zone.
   */
  station_parking?: boolean;
}

/** Properties of a geofencing zone */
export interface GBFSGeofencingZonesV2Properties extends Properties {
  /** Public name of the geofencing zone. */
  name: string;
  /**
   * Start time of the geofencing zone in RFC3339 format.
   * **format** date-time
   */
  start?: string;
  /**
   * End time of the geofencing zone in RFC3339 format.
   * **format** date-time
   */
  end?: string;
  /** Array of rules defining restrictions within the geofence. */
  rules?: GBFSGeofencingZonesV2PropertiesRule[];
}

/**
 * # GBFS Geofencing Zones V2.3
 * Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#geofencing_zonesjson)
 */
export interface GBFSGeofencingZonesV23 {
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
   * Contains geofencing information for the system.
   */
  data: {
    geofencing_zones: FeatureCollection<undefined, MValue, GBFSGeofencingZonesV2Properties>;
  };
}

/**
 * # GBFS Geofencing Zones Schema V2.2
 * Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#geofencing_zonesjson)
 */
export const gbfsGeofencingZonesSchemaV22 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#geofencing_zonesjson',
  description:
    'Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).',
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
      description: 'Array that contains geofencing information for the system.',
      type: 'object',
      properties: {
        geofencing_zones: {
          type: 'object',
          description:
            'Each geofenced zone and its associated rules and attributes is described as an object within the array of features.',
          properties: {
            type: {
              description: 'FeatureCollection as per IETF RFC 7946.',
              type: 'string',
              enum: ['FeatureCollection'],
            },
            features: {
              description: 'Array of GeoJSON Feature objects.',
              type: 'array',
              items: {
                title: 'GeoJSON Feature',
                type: 'object',
                properties: {
                  type: {
                    type: 'string',
                    enum: ['Feature'],
                  },
                  properties: {
                    description: 'Describing travel allowances and limitations.',
                    type: 'object',
                    properties: {
                      name: {
                        description: 'Public name of the geofencing zone.',
                        type: 'string',
                      },
                      start: {
                        description: 'Start time of the geofencing zone in POSIX time.',
                        type: 'number',
                        minimum: 1450155600,
                      },
                      end: {
                        description: 'End time of the geofencing zone in POSIX time.',
                        type: 'number',
                        minimum: 1450155600,
                      },
                      rules: {
                        description: 'Array that contains one object per rule.',
                        type: 'array',
                        items: {
                          type: 'object',
                          properties: {
                            vehicle_type_id: {
                              type: 'array',
                              description:
                                'Array of vehicle type IDs for which these restrictions apply.',
                              items: { type: 'string' },
                            },
                            ride_allowed: {
                              description:
                                'Is the undocked ride allowed to start and end in this zone?',
                              type: 'boolean',
                            },
                            ride_through_allowed: {
                              description: 'Is the ride allowed to travel through this zone?',
                              type: 'boolean',
                            },
                            maximum_speed_kph: {
                              description:
                                'What is the maximum speed allowed, in kilometers per hour?',
                              type: 'integer',
                              minimum: 0,
                            },
                          },
                          required: ['ride_allowed', 'ride_through_allowed'],
                        },
                      },
                    },
                  },
                  geometry: {
                    description: 'A polygon that describes geofencing boundaries.',
                    title: 'GeoJSON MultiPolygon',
                    type: 'object',
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
                              items: { type: 'number' },
                            },
                          },
                        },
                      },
                    },
                    required: ['type', 'coordinates'],
                  },
                },
                required: ['type', 'geometry', 'properties'],
              },
            },
          },
          required: ['type', 'features'],
        },
      },
      required: ['geofencing_zones'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS Geofencing Zones V2.2
 * Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#geofencing_zonesjson)
 */
export interface GBFSGeofencingZonesV22 {
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
   * Contains geofencing information for the system.
   */
  data: {
    geofencing_zones: FeatureCollection<undefined, MValue, GBFSGeofencingZonesV2Properties>;
  };
}

/**
 * # GBFS Geofencing Zones Schema V2.1
 * Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#geofencing_zonesjson)
 */
export const gbfsGeofencingZonesSchemaV21 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#geofencing_zonesjson',
  description:
    'Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).',
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
      description: 'Array that contains geofencing information for the system.',
      type: 'object',
      properties: {
        geofencing_zones: {
          type: 'object',
          description:
            'Each geofenced zone and its associated rules and attributes is described as an object within the array of features.',
          properties: {
            type: {
              description: 'FeatureCollection as per IETF RFC 7946.',
              type: 'string',
              enum: ['FeatureCollection'],
            },
            features: {
              description: 'Array of GeoJSON Feature objects.',
              type: 'array',
              items: {
                title: 'GeoJSON Feature',
                type: 'object',
                properties: {
                  type: {
                    type: 'string',
                    enum: ['Feature'],
                  },
                  properties: {
                    description: 'Describing travel allowances and limitations.',
                    type: 'object',
                    properties: {
                      name: {
                        description: 'Public name of the geofencing zone.',
                        type: 'string',
                      },
                      start: {
                        description: 'Start time of the geofencing zone in POSIX time.',
                        type: 'number',
                        minimum: 1450155600,
                      },
                      end: {
                        description: 'End time of the geofencing zone in POSIX time.',
                        type: 'number',
                        minimum: 1450155600,
                      },
                      rules: {
                        description: 'Array that contains one object per rule.',
                        type: 'array',
                        items: {
                          type: 'object',
                          properties: {
                            vehicle_type_id: {
                              type: 'array',
                              description:
                                'Array of vehicle type IDs for which these restrictions apply.',
                              items: { type: 'string' },
                            },
                            ride_allowed: {
                              description:
                                'Is the undocked ride allowed to start and end in this zone?',
                              type: 'boolean',
                            },
                            ride_through_allowed: {
                              description: 'Is the ride allowed to travel through this zone?',
                              type: 'boolean',
                            },
                            maximum_speed_kph: {
                              description:
                                'What is the maximum speed allowed, in kilometers per hour?',
                              type: 'integer',
                              minimum: 0,
                            },
                          },
                          required: ['ride_allowed', 'ride_through_allowed'],
                        },
                      },
                    },
                  },
                  geometry: {
                    description: 'A polygon that describes geofencing boundaries.',
                    title: 'GeoJSON MultiPolygon',
                    type: 'object',
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
                              items: { type: 'number' },
                            },
                          },
                        },
                      },
                    },
                    required: ['type', 'coordinates'],
                  },
                },
                required: ['type', 'geometry', 'properties'],
              },
            },
          },
          required: ['type', 'features'],
        },
      },
      required: ['geofencing_zones'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS Geofencing Zones V2.1
 * Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#geofencing_zonesjson)
 */
export interface GBFSGeofencingZonesV21 {
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
   * Contains geofencing information for the system.
   */
  data: {
    geofencing_zones: FeatureCollection<undefined, MValue, GBFSGeofencingZonesV2Properties>;
  };
}
