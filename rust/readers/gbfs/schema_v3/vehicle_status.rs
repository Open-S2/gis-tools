// import type { Properties } from '../../../index.js';

// /**
//  * # GBFS Vehicle Status Schema V3.1-RC & V3.0
//  * Describes the vehicles that are available for rent (as of v3.0, formerly free_bike_status).
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#vehicle_statusjson)
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#vehicle_statusjson)
//  */
// export type GBFSVehicleStatusV3 = GBFSVehicleStatusV31RC | GBFSVehicleStatusV30;

// /**
//  * GBFS Vehicle V3
//  */
// export interface GBFSVehicleV3 extends Properties {
//   /**
//    * Rotating (as of v2.0) identifier of a vehicle.
//    */
//   vehicle_id: string;

//   /**
//    * The latitude of the vehicle.
//    * **Range**: [-90, 90]
//    */
//   lat?: number;

//   /**
//    * The longitude of the vehicle.
//    * **Range**: [-180, 180]
//    */
//   lon?: number;

//   /**
//    * Is the vehicle currently reserved?
//    */
//   is_reserved: boolean;

//   /**
//    * Is the vehicle currently disabled (broken)?
//    */
//   is_disabled: boolean;

//   /**
//    * Contains rental URIs for Android, iOS, and web.
//    */
//   rental_uris?: {
//     android?: string; // **Format**: uri
//     ios?: string; // **Format**: uri
//     web?: string; // **Format**: uri
//   };

//   /**
//    * The vehicle_type_id of this vehicle (added in v2.1-RC).
//    */
//   vehicle_type_id?: string;

//   /**
//    * The last time this vehicle reported its status to the operator's backend.
//    * **Format**: date-time
//    */
//   last_reported?: string;

//   /**
//    * The furthest distance in meters the vehicle can travel without recharging or refueling.
//    * **Minimum**: 0
//    */
//   current_range_meters?: number;

//   /**
//    * Current percentage of fuel or battery power remaining in the vehicle.
//    * **Range**: [0, 1]
//    */
//   current_fuel_percent?: number;

//   /**
//    * Identifier referencing the station_id if the vehicle is currently at a station.
//    */
//   station_id?: string;

//   /**
//    * The station_id of the station this vehicle must be returned to.
//    */
//   home_station_id?: string;

//   /**
//    * The plan_id of the pricing plan this vehicle is eligible for.
//    */
//   pricing_plan_id?: string;

//   /**
//    * List of vehicle equipment provided by the operator.
//    * **Enum**: ['child_seat_a', 'child_seat_b', 'child_seat_c', 'winter_tires', 'snow_chains']
//    */
//   vehicle_equipment?: Array<
//     'child_seat_a' | 'child_seat_b' | 'child_seat_c' | 'winter_tires' | 'snow_chains'
//   >;

//   /**
//    * The date and time when any rental of the vehicle must be completed.
//    * **Pattern**: `^([0-9]{4})-([0-9]{2})-([0-9]{2})T([0-9]{2}):([0-9]{2}):([0-9]{2})(([+-]([0-9]{2}):([0-9]{2}))|Z)$`
//    */
//   available_until?: string;
// }

// /**
//  * # GBFS Vehicle Status Schema V3.1-RC
//  * Describes the vehicles that are available for rent (as of v3.0, formerly free_bike_status).
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#vehicle_statusjson)
//  */
// export interface GBFSVehicleStatusV31RC {
//   /**
//    * Last time the data in the feed was updated in RFC3339 format.
//    * **Format**: date-time
//    */
//   last_updated: string;

//   /**
//    * Number of seconds before the data in the feed will be updated again
//    * (0 if the data should always be refreshed).
//    * **Minimum**: 0
//    */
//   ttl: number;

//   /**
//    * GBFS version number to which the feed conforms, according to the versioning framework (added in v1.1).
//    * **Const**: 3.1-RC
//    */
//   version: '3.1-RC';

//   /**
//    * Contains the vehicle data.
//    */
//   data: {
//     /**
//      * Array of vehicles available for rent.
//      */
//     vehicles: GBFSVehicleV3[];
//   };
// }

// /**
//  * # GBFS Vehicle Status Schema V3.0
//  * Describes the vehicles that are available for rent (as of v3.0, formerly free_bike_status).
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#vehicle_statusjson)
//  */
// export interface GBFSVehicleStatusV30 {
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
//    * Vehicle data containing information on available vehicles for rent.
//    */
//   data: {
//     vehicles: GBFSVehicleV3[];
//   };
// }
