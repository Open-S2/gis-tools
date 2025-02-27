/**
 * # GBFS Vehicle Types Schema V3.1-RC & V3.0
 * Describes the types of vehicles that the system operator has available for rent (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#vehicle_typesjson)
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#vehicle_typesjson)
 */
export type GBFSVehicleTypesV3 = GBFSVehicleTypesV31RC | GBFSVehicleTypesV30;

/**
 * # GBFS Vehicle Types Schema V3.1-RC
 * Describes the types of vehicles that System operator has available for rent (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#vehicle_typesjson)
 */
export const gbfsVehicleTypesSchemaV31RC = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#vehicle_typesjson',
  description:
    'Describes the types of vehicles that System operator has available for rent (added in v2.1-RC).',
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
              eco_labels: {
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
                description:
                  'The public name of this vehicle type. An array with one object per supported language with the following keys:',
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
                description:
                  'The name of the vehicle manufacturer. An array with one object per supported language with the following keys:',
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
              model: {
                description:
                  'The name of the vehicle model. An array with one object per supported language with the following keys:',
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
              color: {
                description: 'The color of the vehicle. Added in v2.3',
                type: 'string',
              },
              description: {
                description:
                  'Customer-readable description of the vehicle type outlining special features or how-tos. An array with one object per supported language with the following keys:',
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
 * # GBFS Vehicle Types Schema V3.1-RC
 * Describes the types of vehicles that the system operator has available for rent (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#vehicle_typesjson)
 */
export interface GBFSVehicleTypesV31RC {
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
   * GBFS version number to which the feed conforms, according to the versioning framework.
   * **Const**: 3.1-RC
   */
  version: '3.1-RC';

  /**
   * Contains the vehicle type data.
   */
  data: {
    /**
     * Array of vehicle types in the system.
     */
    vehicle_types: Array<{
      /**
       * Unique identifier of a vehicle type.
       */
      vehicle_type_id: string;

      /**
       * The vehicle's general form factor.
       * **Enum**: ['bicycle', 'cargo_bicycle', 'car', 'moped', 'scooter_standing', 'scooter_seated', 'other', 'scooter']
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
       * **Enum**: ['human', 'electric_assist', 'electric', 'combustion', 'combustion_diesel', 'hybrid', 'plug_in_hybrid', 'hydrogen_fuel_cell']
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
       * **Minimum**: 0
       */
      rider_capacity?: number;

      /**
       * Cargo volume available in the vehicle, expressed in liters.
       * **Minimum**: 0
       */
      cargo_volume_capacity?: number;

      /**
       * The capacity of the vehicle cargo space, expressed in kilograms.
       * **Minimum**: 0
       */
      cargo_load_capacity?: number;

      /**
       * Maximum distance in meters the vehicle can travel without recharging or refueling.
       * Required if `propulsion_type` is electric, combustion, hybrid, etc.
       * **Minimum**: 0
       */
      max_range_meters?: number;

      /**
       * Vehicle air quality certificate.
       */
      eco_labels?: Array<{
        /**
         * Country code following the ISO 3166-1 alpha-2 notation.
         * **Pattern**: `^[A-Z]{2}$`
         */
        country_code: string;

        /**
         * Name of the eco label.
         */
        eco_sticker: string;
      }>;

      /**
       * The public name of this vehicle type in various languages.
       */
      name?: Array<{
        /**
         * The translated text.
         */
        text: string;

        /**
         * IETF BCP 47 language code.
         * **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
         */
        language: string;
      }>;

      /**
       * Description of accessories available in the vehicle.
       * **Enum**: ['air_conditioning', 'automatic', 'manual', 'convertible', 'cruise_control', 'doors_2', 'doors_3', 'doors_4', 'doors_5', 'navigation']
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
       * Maximum quantity of CO2, in grams, emitted per kilometer.
       * **Minimum**: 0
       */
      g_CO2_km?: number;

      /**
       * URL to an image that helps users identify the vehicle.
       * **Format**: uri
       */
      vehicle_image?: string;

      /**
       * Manufacturer name of the vehicle.
       */
      make?: Array<{
        text: string;
        language: string; // **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
      }>;

      /**
       * Model name of the vehicle.
       */
      model?: Array<{
        text: string;
        language: string; // **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
      }>;

      /**
       * Color of the vehicle.
       */
      color?: string;

      /**
       * Customer-readable description of the vehicle type outlining special features or how-tos.
       */
      description?: Array<{
        text: string;
        language: string; // **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
      }>;

      /**
       * Number of wheels this vehicle type has.
       * **Minimum**: 0
       */
      wheel_count?: number;

      /**
       * Maximum permitted speed in kilometers per hour.
       * **Minimum**: 0
       */
      max_permitted_speed?: number;

      /**
       * Rated power of the motor for this vehicle type in watts.
       * **Minimum**: 0
       */
      rated_power?: number;

      /**
       * Maximum time in minutes a vehicle can be reserved before a rental begins.
       * **Minimum**: 0
       */
      default_reserve_time?: number;

      /**
       * Conditions for returning the vehicle at the end of the trip.
       * **Enum**: ['free_floating', 'roundtrip_station', 'any_station', 'hybrid']
       */
      return_constraint?: 'free_floating' | 'roundtrip_station' | 'any_station' | 'hybrid';

      /**
       * Assets associated with the vehicle type.
       */
      vehicle_assets?: {
        /**
         * URL pointing to the location of a graphic icon file for this vehicle type.
         * **Format**: uri
         */
        icon_url: string;

        /**
         * URL for a dark mode version of the vehicle icon.
         * **Format**: uri
         */
        icon_url_dark?: string;

        /**
         * Date of last modification for vehicle icon images.
         * **Format**: date
         */
        icon_last_modified: string;
      };

      /**
       * A plan ID as defined in system_pricing_plans.json.
       */
      default_pricing_plan_id?: string;

      /**
       * Array of all pricing plan IDs for this vehicle type.
       */
      pricing_plan_ids?: Array<string>;
    }>;
  };
}

/**
 * # GBFS Vehicle Types Schema V3.0
 * Describes the types of vehicles that System operator has available for rent (added in v2.1-RC).
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#vehicle_typesjson)
 */
export interface GBFSVehicleTypesV30 {
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
   * Vehicle type data.
   */
  data: {
    vehicle_types: Array<{
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
       * The furthest distance the vehicle can travel without recharging or refueling.
       */
      max_range_meters?: number;

      /**
       * Public name of the vehicle type.
       */
      name?: Array<{
        text: string;
        language: string;
      }>;

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
       * Maximum CO2 emissions per kilometer, in grams.
       */
      g_CO2_km?: number;

      /**
       * URL to an image of the vehicle.
       */
      vehicle_image?: string;

      /**
       * Manufacturer of the vehicle.
       */
      make?: Array<{
        text: string;
        language: string;
      }>;

      /**
       * Model of the vehicle.
       */
      model?: Array<{
        text: string;
        language: string;
      }>;

      /**
       * The vehicle's color.
       */
      color?: string;

      /**
       * Customer-readable description of the vehicle type.
       */
      description?: Array<{
        text: string;
        language: string;
      }>;

      /**
       * Number of wheels on the vehicle.
       */
      wheel_count?: number;

      /**
       * The maximum speed permitted for the vehicle.
       */
      max_permitted_speed?: number;

      /**
       * The rated motor power in watts.
       */
      rated_power?: number;

      /**
       * Default reserve time for the vehicle, in minutes.
       */
      default_reserve_time?: number;

      /**
       * Return conditions for the vehicle.
       */
      return_constraint?: 'free_floating' | 'roundtrip_station' | 'any_station' | 'hybrid';

      /**
       * Information about the vehicle's assets.
       */
      vehicle_assets?: {
        icon_url: string;
        icon_url_dark?: string;
        icon_last_modified: string;
      };

      /**
       * Default pricing plan ID for this vehicle type.
       */
      default_pricing_plan_id?: string;

      /**
       * Array of all pricing plan IDs available for this vehicle type.
       */
      pricing_plan_ids?: string[];

      /**
       * Rider capacity of the vehicle.
       */
      rider_capacity?: number;

      /**
       * Cargo volume capacity in liters.
       */
      cargo_volume_capacity?: number;

      /**
       * Cargo load capacity in kilograms.
       */
      cargo_load_capacity?: number;

      /**
       * Eco labels for the vehicle.
       */
      eco_labels?: Array<{
        country_code: string;
        eco_sticker: string;
      }>;
    }>;
  };
}
