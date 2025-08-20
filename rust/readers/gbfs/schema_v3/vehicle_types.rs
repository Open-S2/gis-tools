// /**
//  * # GBFS Vehicle Types Schema V3.1-RC & V3.0
//  * Describes the types of vehicles that the system operator has available for rent (added in v2.1-RC).
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#vehicle_typesjson)
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#vehicle_typesjson)
//  */
// export type GBFSVehicleTypesV3 = GBFSVehicleTypesV30;

// /**
//  * # GBFS Vehicle Types Schema V3.0
//  * Describes the types of vehicles that System operator has available for rent (added in v2.1-RC).
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#vehicle_typesjson)
//  */
// export interface GBFSVehicleTypesV30 {
//   /**
//    * Last time the data in the feed was updated in RFC3339 format.
//    */
//   last_updated: string;

//   /**
//    * Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
//    */
//   ttl: number;

//   /**
//    * GBFS version number to which the feed conforms.
//    */
//   version: '3.0';

//   /**
//    * Vehicle type data.
//    */
//   data: {
//     vehicle_types: Array<{
//       /**
//        * Unique identifier of a vehicle type.
//        */
//       vehicle_type_id: string;

//       /**
//        * The vehicle's general form factor.
//        */
//       form_factor:
//         | 'bicycle'
//         | 'cargo_bicycle'
//         | 'car'
//         | 'moped'
//         | 'scooter_standing'
//         | 'scooter_seated'
//         | 'other'
//         | 'scooter';

//       /**
//        * The primary propulsion type of the vehicle.
//        */
//       propulsion_type:
//         | 'human'
//         | 'electric_assist'
//         | 'electric'
//         | 'combustion'
//         | 'combustion_diesel'
//         | 'hybrid'
//         | 'plug_in_hybrid'
//         | 'hydrogen_fuel_cell';

//       /**
//        * The furthest distance the vehicle can travel without recharging or refueling.
//        */
//       max_range_meters?: number;

//       /**
//        * Public name of the vehicle type.
//        */
//       name?: Array<{
//         text: string;
//         language: string;
//       }>;

//       /**
//        * Description of accessories available in the vehicle.
//        */
//       vehicle_accessories?: Array<
//         | 'air_conditioning'
//         | 'automatic'
//         | 'manual'
//         | 'convertible'
//         | 'cruise_control'
//         | 'doors_2'
//         | 'doors_3'
//         | 'doors_4'
//         | 'doors_5'
//         | 'navigation'
//       >;

//       /**
//        * Maximum CO2 emissions per kilometer, in grams.
//        */
//       g_CO2_km?: number;

//       /**
//        * URL to an image of the vehicle.
//        */
//       vehicle_image?: string;

//       /**
//        * Manufacturer of the vehicle.
//        */
//       make?: Array<{
//         text: string;
//         language: string;
//       }>;

//       /**
//        * Model of the vehicle.
//        */
//       model?: Array<{
//         text: string;
//         language: string;
//       }>;

//       /**
//        * The vehicle's color.
//        */
//       color?: string;

//       /**
//        * Customer-readable description of the vehicle type.
//        */
//       description?: Array<{
//         text: string;
//         language: string;
//       }>;

//       /**
//        * Number of wheels on the vehicle.
//        */
//       wheel_count?: number;

//       /**
//        * The maximum speed permitted for the vehicle.
//        */
//       max_permitted_speed?: number;

//       /**
//        * The rated motor power in watts.
//        */
//       rated_power?: number;

//       /**
//        * Default reserve time for the vehicle, in minutes.
//        */
//       default_reserve_time?: number;

//       /**
//        * Return conditions for the vehicle.
//        */
//       return_constraint?: 'free_floating' | 'roundtrip_station' | 'any_station' | 'hybrid';

//       /**
//        * Information about the vehicle's assets.
//        */
//       vehicle_assets?: {
//         icon_url: string;
//         icon_url_dark?: string;
//         icon_last_modified: string;
//       };

//       /**
//        * Default pricing plan ID for this vehicle type.
//        */
//       default_pricing_plan_id?: string;

//       /**
//        * Array of all pricing plan IDs available for this vehicle type.
//        */
//       pricing_plan_ids?: string[];

//       /**
//        * Rider capacity of the vehicle.
//        */
//       rider_capacity?: number;

//       /**
//        * Cargo volume capacity in liters.
//        */
//       cargo_volume_capacity?: number;

//       /**
//        * Cargo load capacity in kilograms.
//        */
//       cargo_load_capacity?: number;

//       /**
//        * Eco labels for the vehicle.
//        */
//       eco_labels?: Array<{
//         country_code: string;
//         eco_sticker: string;
//       }>;
//     }>;
//   };
// }
