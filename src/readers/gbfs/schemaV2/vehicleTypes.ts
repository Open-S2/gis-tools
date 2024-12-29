/**
 * # GBFS Vehicle Types V2.3, V2.2, V2.1, OR V2.0
 * Describes the types of vehicles that System operator has available for rent (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_pricing_plansjson)
 * - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_pricing_plansjson)
 * - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_pricing_plansjson)
 */
export type GBFSVehicleTypesV2 = GBFSVehicleTypesV23 | GBFSVehicleTypesV22 | GBFSVehicleTypesV21;

/**
 * # GBFS Vehicle Types Schema V2.3
 * Describes the types of vehicles that System operator has available for rent (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#vehicle_typesjson)
 */
export const gbfsVehicleTypesSchemaV2 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#vehicle_typesjson',
  description:
    'Describes the types of vehicles that System operator has available for rent (added in v2.1-RC).',
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
        vehicle_types: {
          description:
            'Array that contains one object per vehicle type in the system as defined below.',
          type: 'array',
          items: {
            type: 'object',
            properties: {
              vehicle_type_id: {
                description: 'Unique identifier of a vehicle type.',
                type: 'string',
              },
              form_factor: {
                description: "The vehicle's general form factor.",
                type: 'string',
                enum: [
                  'bicycle',
                  'cargo_bicycle',
                  'car',
                  'moped',
                  'scooter_standing',
                  'scooter_seated',
                  'other',
                  'scooter',
                ],
              },
              rider_capacity: {
                description:
                  'The number of riders (driver included) the vehicle can legally accommodate',
                type: 'integer',
                minimum: 0,
              },
              cargo_volume_capacity: {
                description: 'Cargo volume available in the vehicle, expressed in liters.',
                type: 'integer',
                minimum: 0,
              },
              cargo_load_capacity: {
                description:
                  'The capacity of the vehicle cargo space (excluding passengers), expressed in kilograms.',
                type: 'integer',
                minimum: 0,
              },

              propulsion_type: {
                description:
                  'The primary propulsion type of the vehicle. Updated in v2.3 to represent car-sharing',
                type: 'string',
                enum: [
                  'human',
                  'electric_assist',
                  'electric',
                  'combustion',
                  'combustion_diesel',
                  'hybrid',
                  'plug_in_hybrid',
                  'hydrogen_fuel_cell',
                ],
              },
              eco_label: {
                description: 'Vehicle air quality certificate. added in v2.3.',
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    country_code: {
                      description:
                        ' Country code following the ISO 3166-1 alpha-2 notation. Added in v2.3.',
                      type: 'string',
                      pattern: '^[A-Z]{2}',
                    },
                    eco_sticker: {
                      description: ' Name of the eco label. Added in v2.3.',
                      type: 'string',
                    },
                  },
                },
                required: ['country_code', 'eco_sticker'],
              },
              max_range_meters: {
                description:
                  'The furthest distance in meters that the vehicle can travel without recharging or refueling when it has the maximum amount of energy potential.',
                type: 'number',
                minimum: 0,
              },
              name: {
                description: 'The public name of this vehicle type.',
                type: 'string',
              },
              vehicle_accessories: {
                description: 'Description of accessories available in the vehicle.',
                type: 'array',
                items: {
                  enum: [
                    'air_conditioning',
                    'automatic',
                    'manual',
                    'convertible',
                    'cruise_control',
                    'doors_2',
                    'doors_3',
                    'doors_4',
                    'doors_5',
                    'navigation',
                  ],
                },
              },
              g_CO2_km: {
                description:
                  'Maximum quantity of CO2, in grams, emitted per kilometer, according to the WLTP. Added in v2.3',
                type: 'integer',
                minimum: 0,
              },
              vehicle_image: {
                description:
                  'URL to an image that would assist the user in identifying the vehicle. JPEG or PNG. Added in v2.3',
                type: 'string',
                format: 'uri',
              },
              make: {
                description: 'The name of the vehicle manufacturer. Added in v2.3',
                type: 'string',
              },
              model: {
                description: 'The name of the vehicle model. Added in v2.3',
                type: 'string',
              },
              color: {
                description: 'The color of the vehicle. Added in v2.3',
                type: 'string',
              },
              wheel_count: {
                description: 'Number of wheels this vehicle type has. Added in v2.3',
                type: 'integer',
                minimum: 0,
              },
              max_permitted_speed: {
                description:
                  'The maximum speed in kilometers per hour this vehicle is permitted to reach in accordance with local permit and regulations. Added in v2.3',
                type: 'integer',
                minimum: 0,
              },
              rated_power: {
                description:
                  'The rated power of the motor for this vehicle type in watts. Added in v2.3',
                type: 'integer',
                minimum: 0,
              },
              default_reserve_time: {
                description:
                  'Maximum time in minutes that a vehicle can be reserved before a rental begins added in v2.3-RC.',
                type: 'integer',
                minimum: 0,
              },
              return_constraint: {
                description:
                  'The conditions for returning the vehicle at the end of the trip. Added in v2.3-RC as return_type, and updated to return_constraint in v2.3.',
                type: 'string',
                enum: ['free_floating', 'roundtrip_station', 'any_station', 'hybrid'],
              },
              vehicle_assets: {
                description:
                  'An object where each key defines one of the items listed below added in v2.3-RC.',
                type: 'object',
                properties: {
                  icon_url: {
                    description:
                      'A fully qualified URL pointing to the location of a graphic icon file that MAY be used to represent this vehicle type on maps and in other applications added in v2.3-RC.',
                    type: 'string',
                    format: 'uri',
                  },
                  icon_url_dark: {
                    description:
                      'A fully qualified URL pointing to the location of a graphic icon file to be used to represent this vehicle type when in dark mode added in v2.3-RC.',
                    type: 'string',
                    format: 'uri',
                  },
                  icon_last_modified: {
                    description:
                      'Date that indicates the last time any included vehicle icon images were modified or updated added in v2.3-RC.',
                    type: 'string',
                    format: 'date',
                  },
                },
                required: ['icon_url', 'icon_last_modified'],
              },
              default_pricing_plan_id: {
                description: 'A plan_id as defined in system_pricing_plans.json added in v2.3-RC.',
                type: 'string',
              },
              pricing_plan_ids: {
                description:
                  'Array of all pricing plan IDs as defined in system_pricing_plans.json added in v2.3-RC.',
                type: 'array',
                items: {
                  type: 'string',
                },
              },
            },
            required: ['vehicle_type_id', 'form_factor', 'propulsion_type'],
            if: {
              properties: {
                propulsion_type: {
                  enum: [
                    'electric',
                    'electric_assist',
                    'combustion',
                    'combustion_diesel',
                    'hybrid',
                    'plug_in_hybrid',
                    'hydrogen_fuel_cell',
                  ],
                },
              },
            },
            then: {
              required: ['max_range_meters'],
            },
          },
        },
      },
      required: ['vehicle_types'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS Vehicle Types V2.3
 * Describes the types of vehicles that System operator has available for rent (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#vehicle_typesjson)
 */
export interface GBFSVehicleTypesV23 {
  /**
   * Last time the data in the feed was updated in POSIX time.
   * Minimum: 1450155600.
   */
  last_updated: number;

  /**
   * Number of seconds before the data in the feed will be updated again
   * (0 if the data should always be refreshed).
   */
  ttl: number;

  /**
   * GBFS version number to which the feed conforms.
   */
  version: '2.3';

  /**
   * Response data in the form of name:value pairs.
   */
  data: {
    /**
     * Array of vehicle types available in the system.
     */
    vehicle_types: {
      /**
       * Unique identifier of a vehicle type.
       */
      vehicle_type_id: string;

      /**
       * The vehicle's general form factor.
       */
      form_factor:
        | 'bicycle'
        | 'cargo_bicycle'
        | 'car'
        | 'moped'
        | 'scooter_standing'
        | 'scooter_seated'
        | 'other'
        | 'scooter';

      /**
       * The primary propulsion type of the vehicle.
       */
      propulsion_type:
        | 'human'
        | 'electric_assist'
        | 'electric'
        | 'combustion'
        | 'combustion_diesel'
        | 'hybrid'
        | 'plug_in_hybrid'
        | 'hydrogen_fuel_cell';

      /**
       * The number of riders (driver included) the vehicle can legally accommodate.
       * Minimum: 0.
       */
      rider_capacity?: number;

      /**
       * Cargo volume available in the vehicle, expressed in liters.
       * Minimum: 0.
       */
      cargo_volume_capacity?: number;

      /**
       * The capacity of the vehicle cargo space (excluding passengers), expressed in kilograms.
       * Minimum: 0.
       */
      cargo_load_capacity?: number;

      /**
       * The furthest distance in meters that the vehicle can travel without recharging or refueling
       * when it has the maximum amount of energy potential.
       * Minimum: 0.
       */
      max_range_meters?: number;

      /**
       * The public name of this vehicle type.
       */
      name?: string;

      /**
       * Description of accessories available in the vehicle.
       */
      vehicle_accessories?: Array<
        | 'air_conditioning'
        | 'automatic'
        | 'manual'
        | 'convertible'
        | 'cruise_control'
        | 'doors_2'
        | 'doors_3'
        | 'doors_4'
        | 'doors_5'
        | 'navigation'
      >;

      /**
       * Maximum quantity of CO2, in grams, emitted per kilometer, according to the WLTP.
       * Minimum: 0.
       */
      g_CO2_km?: number;

      /**
       * URL to an image that would assist the user in identifying the vehicle.
       * JPEG or PNG.
       */
      vehicle_image?: string;

      /**
       * The name of the vehicle manufacturer.
       */
      make?: string;

      /**
       * The name of the vehicle model.
       */
      model?: string;

      /**
       * The color of the vehicle.
       */
      color?: string;

      /**
       * Number of wheels this vehicle type has.
       * Minimum: 0.
       */
      wheel_count?: number;

      /**
       * The maximum speed in kilometers per hour this vehicle is permitted to reach in accordance
       * with local permit and regulations.
       * Minimum: 0.
       */
      max_permitted_speed?: number;

      /**
       * The rated power of the motor for this vehicle type in watts.
       * Minimum: 0.
       */
      rated_power?: number;

      /**
       * Maximum time in minutes that a vehicle can be reserved before a rental begins.
       * Minimum: 0.
       */
      default_reserve_time?: number;

      /**
       * The conditions for returning the vehicle at the end of the trip.
       */
      return_constraint?: 'free_floating' | 'roundtrip_station' | 'any_station' | 'hybrid';

      /**
       * A plan_id as defined in system_pricing_plans.json.
       */
      default_pricing_plan_id?: string;

      /**
       * Array of all pricing plan IDs as defined in system_pricing_plans.json.
       */
      pricing_plan_ids?: string[];

      /**
       * Vehicle air quality certificate.
       */
      eco_label?: Array<{
        /**
         * Country code following the ISO 3166-1 alpha-2 notation.
         */
        country_code: string;

        /**
         * Name of the eco label.
         */
        eco_sticker: string;
      }>;

      /**
       * An object where each key defines vehicle assets.
       */
      vehicle_assets?: {
        /**
         * A fully qualified URL pointing to the location of a graphic icon file
         * that MAY be used to represent this vehicle type on maps and in other applications.
         */
        icon_url: string;

        /**
         * A fully qualified URL pointing to the location of a graphic icon file to
         * be used to represent this vehicle type when in dark mode.
         */
        icon_url_dark?: string;

        /**
         * Date that indicates the last time any included vehicle icon images were modified or updated.
         */
        icon_last_modified: string;
      };
    }[];
  };
}

/**
 * # GBFS Vehicle Types Schema V2.2
 * Describes the types of vehicles that System operator has available for rent (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#vehicle_typesjson)
 */
export const gbfsVehicleTypesSchemaV22 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#vehicle_typesjson-added-in-v21',
  description:
    'Describes the types of vehicles that System operator has available for rent (added in v2.1-RC).',
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
        vehicle_types: {
          description:
            'Array that contains one object per vehicle type in the system as defined below.',
          type: 'array',
          items: {
            type: 'object',
            properties: {
              vehicle_type_id: {
                description: 'Unique identifier of a vehicle type.',
                type: 'string',
              },
              form_factor: {
                description: "The vehicle's general form factor.",
                type: 'string',
                enum: ['bicycle', 'car', 'moped', 'other', 'scooter'],
              },
              propulsion_type: {
                description: 'The primary propulsion type of the vehicle.',
                type: 'string',
                enum: ['human', 'electric_assist', 'electric', 'combustion'],
              },
              max_range_meters: {
                description:
                  'The furthest distance in meters that the vehicle can travel without recharging or refueling when it has the maximum amount of energy potential.',
                type: 'number',
                minimum: 0,
              },
              name: {
                description: 'The public name of this vehicle type.',
                type: 'string',
              },
            },
            required: ['vehicle_type_id', 'form_factor', 'propulsion_type'],
            if: {
              properties: {
                propulsion_type: {
                  enum: ['electric', 'electric_assist', 'combustion'],
                },
              },
            },
            then: {
              required: ['max_range_meters'],
            },
          },
        },
      },
      required: ['vehicle_types'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS Vehicle Types V2.2
 * Describes the types of vehicles that System operator has available for rent (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#vehicle_typesjson)
 */
export interface GBFSVehicleTypesV22 {
  /**
   * Last time the data in the feed was updated in POSIX time.
   * Minimum: 1450155600.
   */
  last_updated: number;

  /**
   * Number of seconds before the data in the feed will be updated again
   * (0 if the data should always be refreshed).
   */
  ttl: number;

  /**
   * GBFS version number to which the feed conforms.
   */
  version: '2.2';

  /**
   * Response data in the form of name:value pairs.
   */
  data: {
    /**
     * Array of vehicle types available in the system.
     */
    vehicle_types: {
      /**
       * Unique identifier of a vehicle type.
       */
      vehicle_type_id: string;

      /**
       * The vehicle's general form factor.
       */
      form_factor: 'bicycle' | 'car' | 'moped' | 'other' | 'scooter';

      /**
       * The primary propulsion type of the vehicle.
       */
      propulsion_type: 'human' | 'electric_assist' | 'electric' | 'combustion';

      /**
       * The furthest distance in meters that the vehicle can travel without recharging or refueling
       * when it has the maximum amount of energy potential.
       * Minimum: 0.
       */
      max_range_meters?: number;

      /**
       * The public name of this vehicle type.
       */
      name?: string;
    }[];
  };
}

/**
 * # GBFS Vehicle Types Schema V2.1
 * Describes the types of vehicles that System operator has available for rent (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#vehicle_typesjson)
 */
export const gbfsVehicleTypesSchemaV21 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#vehicle_typesjson-added-in-v21',
  description:
    'Describes the types of vehicles that System operator has available for rent (added in v2.1-RC).',
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
        vehicle_types: {
          description:
            'Array that contains one object per vehicle type in the system as defined below.',
          type: 'array',
          items: {
            type: 'object',
            properties: {
              vehicle_type_id: {
                description: 'Unique identifier of a vehicle type.',
                type: 'string',
              },
              form_factor: {
                description: "The vehicle's general form factor.",
                type: 'string',
                enum: ['bicycle', 'car', 'moped', 'other', 'scooter'],
              },
              propulsion_type: {
                description: 'The primary propulsion type of the vehicle.',
                type: 'string',
                enum: ['human', 'electric_assist', 'electric', 'combustion'],
              },
              max_range_meters: {
                description:
                  'The furthest distance in meters that the vehicle can travel without recharging or refueling when it has the maximum amount of energy potential.',
                type: 'number',
                minimum: 0,
              },
              name: {
                description: 'The public name of this vehicle type.',
                type: 'string',
              },
            },
            required: ['vehicle_type_id', 'form_factor', 'propulsion_type'],
            if: {
              properties: {
                propulsion_type: {
                  enum: ['electric', 'electric_assist', 'combustion'],
                },
              },
            },
            then: {
              required: ['max_range_meters'],
            },
          },
        },
      },
      required: ['vehicle_types'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS Vehicle Types V2.1
 * Describes the types of vehicles that System operator has available for rent (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#vehicle_typesjson)
 */
export interface GBFSVehicleTypesV21 {
  /**
   * Last time the data in the feed was updated in POSIX time.
   * Minimum: 1450155600.
   */
  last_updated: number;

  /**
   * Number of seconds before the data in the feed will be updated again
   * (0 if the data should always be refreshed).
   */
  ttl: number;

  /**
   * GBFS version number to which the feed conforms.
   */
  version: '2.1';

  /**
   * Response data in the form of name:value pairs.
   */
  data: {
    /**
     * Array of vehicle types available in the system.
     */
    vehicle_types: {
      /**
       * Unique identifier of a vehicle type.
       */
      vehicle_type_id: string;

      /**
       * The vehicle's general form factor.
       */
      form_factor: 'bicycle' | 'car' | 'moped' | 'other' | 'scooter';

      /**
       * The primary propulsion type of the vehicle.
       */
      propulsion_type: 'human' | 'electric_assist' | 'electric' | 'combustion';

      /**
       * The furthest distance in meters that the vehicle can travel without recharging or refueling
       * when it has the maximum amount of energy potential.
       * Minimum: 0.
       */
      max_range_meters?: number;

      /**
       * The public name of this vehicle type.
       */
      name?: string;
    }[];
  };
}
