import type { FeatureCollection, MValue, Properties, ValueArrayObject } from '../../..';

/**
 * # GBFS Geofencing Zones Schema V3.1-RC & V3.0
 * Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#geofencing_zonesjson)
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#geofencing_zonesjson)
 */
export type GBFSGeofencingZonesV3 = GBFSGeofencingZonesV31RC | GBFSGeofencingZonesV30;

/**
 * # GBFS Geofencing Zones Schema V3.1-RC
 * Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#geofencing_zonesjson)
 */
export const gbfsGeofencingZonesSchemaV31RC = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#geofencing_zonesjson',
  description:
    'Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).',
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
                      start: {
                        description: 'Start time of the geofencing zone in RFC3339 format.',
                        type: 'string',
                        format: 'date-time',
                      },
                      end: {
                        description: 'End time of the geofencing zone in RFC3339 format.',
                        type: 'string',
                        format: 'date-time',
                      },
                      rules: {
                        description:
                          'Array of Rule objects defining restrictions that apply within the area of the polygon.',
                        type: 'array',
                        items: {
                          type: 'object',
                          properties: {
                            vehicle_type_ids: {
                              type: 'array',
                              description:
                                'Array of vehicle type IDs for which these restrictions apply.',
                              items: { type: 'string' },
                            },
                            ride_start_allowed: {
                              description: 'Is the ride allowed to start in this zone?',
                              type: 'boolean',
                            },
                            ride_end_allowed: {
                              description: 'Is the ride allowed to end in this zone?',
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
                                'Vehicle MUST be parked at stations defined in station_information.json within this geofence zone',
                              type: 'boolean',
                            },
                          },
                          required: [
                            'ride_start_allowed',
                            'ride_end_allowed',
                            'ride_through_allowed',
                          ],
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
        global_rules: {
          description:
            'Array of Rule objects defining restrictions that apply globally in all areas as the default restrictions, except where overridden with an explicit geofencing zone.',
          type: 'array',
          items: {
            type: 'object',
            properties: {
              vehicle_type_ids: {
                type: 'array',
                description: 'Array of vehicle type IDs for which these restrictions apply.',
                items: { type: 'string' },
              },
              ride_start_allowed: {
                description: 'Is the ride allowed to start in this zone?',
                type: 'boolean',
              },
              ride_end_allowed: {
                description: 'Is the ride allowed to end in this zone?',
                type: 'boolean',
              },
              ride_through_allowed: {
                description: 'Is the ride allowed to travel through this zone?',
                type: 'boolean',
              },
              maximum_speed_kph: {
                description: 'What is the maximum speed allowed, in kilometers per hour?',
                type: 'integer',
                minimum: 0,
              },
              station_parking: {
                description:
                  'Vehicle MUST be parked at stations defined in station_information.json within this geofence zone',
                type: 'boolean',
              },
            },
            required: ['ride_start_allowed', 'ride_end_allowed', 'ride_through_allowed'],
          },
        },
      },
      required: ['geofencing_zones', 'global_rules'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * GBFS V3: Public name of the geofencing zone.
 */
export interface GBFSGeofencingZonesV3PropertiesName extends ValueArrayObject {
  /**
   * The translated text.
   */
  text: string;
  /**
   * IETF BCP 47 language code.
   * **pattern** ^[a-z]{2,3}(-[A-Z]{2})?$
   */
  language: string;
}

/**
 * GBFS V3: Restrictions that apply within the area of the polygon.
 */
export interface GBFSGeofencingZonesV3PropertiesRule extends ValueArrayObject {
  /**
   * Array of vehicle type IDs for which these restrictions apply.
   */
  // @ts-expect-error - we need to clean this / remove it
  vehicle_type_ids?: string[];
  /**
   * Is the ride allowed to start in this zone?
   */
  ride_start_allowed: boolean;
  /**
   * Is the ride allowed to end in this zone?
   */
  ride_end_allowed: boolean;
  /**
   * Is the ride allowed to travel through this zone?
   */
  ride_through_allowed: boolean;
  /**
   * Maximum speed allowed, in kilometers per hour.
   * **minimum** 0
   */
  maximum_speed_kph?: number;
  /**
   * Vehicle MUST be parked at stations defined in station_information.json within this zone.
   */
  station_parking?: boolean;
}

/** Properties of a geofencing zone */
export interface GBFSGeofencingZonesV3Properties extends Properties {
  /** Public name of the geofencing zone. */
  name: GBFSGeofencingZonesV3PropertiesName[];
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
  rules?: GBFSGeofencingZonesV3PropertiesRule[];
}

/**
 * # GBFS Geofencing Zones Schema V3.1-RC
 * Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#geofencing_zonesjson)
 */
export interface GBFSGeofencingZonesV31RC {
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
   * Array that contains geofencing information for the system.
   */
  data: {
    /**
     * Geofencing zones and their associated rules and attributes.
     */
    geofencing_zones: FeatureCollection<undefined, MValue, GBFSGeofencingZonesV3Properties>;

    /**
     * Global rules defining restrictions that apply globally in all areas.
     */
    global_rules: Array<{
      /**
       * Array of vehicle type IDs for which these restrictions apply.
       */
      vehicle_type_ids?: string[];

      /**
       * Is the ride allowed to start in this zone?
       */
      ride_start_allowed: boolean;

      /**
       * Is the ride allowed to end in this zone?
       */
      ride_end_allowed: boolean;

      /**
       * Is the ride allowed to travel through this zone?
       */
      ride_through_allowed: boolean;

      /**
       * Maximum speed allowed, in kilometers per hour.
       * **minimum** 0
       */
      maximum_speed_kph?: number;

      /**
       * Vehicle MUST be parked at stations defined in station_information.json within this geofence zone.
       */
      station_parking?: boolean;
    }>;
  };
}

/**
 * # GBFS Geofencing Zones Schema V3.0
 * Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#geofencing_zonesjson)
 */
export const gbfsGeofencingZonesSchemaV30 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#geofencing_zonesjson',
  description:
    'Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).',
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
                      start: {
                        description: 'Start time of the geofencing zone in RFC3339 format.',
                        type: 'string',
                        format: 'date-time',
                      },
                      end: {
                        description: 'End time of the geofencing zone in RFC3339 format.',
                        type: 'string',
                        format: 'date-time',
                      },
                      rules: {
                        description:
                          'Array of Rule objects defining restrictions that apply within the area of the polygon.',
                        type: 'array',
                        items: {
                          type: 'object',
                          properties: {
                            vehicle_type_ids: {
                              type: 'array',
                              description:
                                'Array of vehicle type IDs for which these restrictions apply.',
                              items: { type: 'string' },
                            },
                            ride_start_allowed: {
                              description: 'Is the ride allowed to start in this zone?',
                              type: 'boolean',
                            },
                            ride_end_allowed: {
                              description: 'Is the ride allowed to end in this zone?',
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
                                'Vehicle MUST be parked at stations defined in station_information.json within this geofence zone',
                              type: 'boolean',
                            },
                          },
                          required: [
                            'ride_start_allowed',
                            'ride_end_allowed',
                            'ride_through_allowed',
                          ],
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
        global_rules: {
          description:
            'Array of Rule objects defining restrictions that apply globally in all areas as the default restrictions, except where overridden with an explicit geofencing zone.',
          type: 'array',
          items: {
            type: 'object',
            properties: {
              vehicle_type_ids: {
                type: 'array',
                description: 'Array of vehicle type IDs for which these restrictions apply.',
                items: { type: 'string' },
              },
              ride_start_allowed: {
                description: 'Is the ride allowed to start in this zone?',
                type: 'boolean',
              },
              ride_end_allowed: {
                description: 'Is the ride allowed to end in this zone?',
                type: 'boolean',
              },
              ride_through_allowed: {
                description: 'Is the ride allowed to travel through this zone?',
                type: 'boolean',
              },
              maximum_speed_kph: {
                description: 'What is the maximum speed allowed, in kilometers per hour?',
                type: 'integer',
                minimum: 0,
              },
              station_parking: {
                description:
                  'Vehicle MUST be parked at stations defined in station_information.json within this geofence zone',
                type: 'boolean',
              },
            },
            required: ['ride_start_allowed', 'ride_end_allowed', 'ride_through_allowed'],
          },
        },
      },
      required: ['geofencing_zones', 'global_rules'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS Geofencing Zones Schema V3.0
 * Describes geofencing zones and their associated rules and attributes (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#geofencing_zonesjson)
 */
export interface GBFSGeofencingZonesV30 {
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
   * Array that contains geofencing information for the system.
   */
  data: {
    geofencing_zones: FeatureCollection<undefined, MValue, GBFSGeofencingZonesV3Properties>;
    /**
     * Array of global rules defining restrictions that apply by default.
     */
    global_rules: Array<{
      /**
       * Array of vehicle type IDs for which these restrictions apply.
       */
      vehicle_type_ids?: string[];

      /**
       * Is the ride allowed to start in this zone?
       */
      ride_start_allowed: boolean;

      /**
       * Is the ride allowed to end in this zone?
       */
      ride_end_allowed: boolean;

      /**
       * Is the ride allowed to travel through this zone?
       */
      ride_through_allowed: boolean;

      /**
       * Maximum speed allowed in kilometers per hour.
       */
      maximum_speed_kph?: number;

      /**
       * Must vehicles be parked at stations within this geofence zone?
       */
      station_parking?: boolean;
    }>;
  };
}
