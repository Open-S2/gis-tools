// /**
//  * # GBFS Station Status Schema V3.1-RC & V3.0
//  * Describes the capacity and rental availability of the station.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#station_statusjson)
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#station_statusjson)
//  */
// export type GBFSStationStatusV3 = GBFSStationStatusV30;

// /**
//  * # GBFS Station Status Schema V3.0
//  * Describes the capacity and rental availability of the station.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#station_statusjson)
//  */
// export interface GBFSStationStatusV30 {
//   /**
//    * Last time the data in the feed was updated in RFC3339 format.
//    * **Format**: date-time
//    */
//   last_updated: string;

//   /**
//    * Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
//    * **Minimum**: 0
//    */
//   ttl: number;

//   /**
//    * GBFS version number to which the feed conforms.
//    * **Const**: '3.0'
//    */
//   version: '3.0';

//   /**
//    * Data object containing station statuses.
//    */
//   data: {
//     /**
//      * List of stations with their rental availability and capacities.
//      */
//     stations: Array<{
//       /**
//        * Identifier of the station.
//        */
//       station_id: string;

//       /**
//        * Number of vehicles physically available for rental at the station.
//        * **Minimum**: 0
//        */
//       num_vehicles_available: number;

//       /**
//        * Details of vehicles available by type at the station.
//        */
//       vehicle_types_available?: Array<{
//         vehicle_type_id: string;
//         count: number;
//       }>;

//       /**
//        * Number of disabled vehicles at the station.
//        * **Minimum**: 0
//        */
//       num_vehicles_disabled?: number;

//       /**
//        * Number of functional docks physically at the station.
//        * **Minimum**: 0
//        */
//       num_docks_available?: number;

//       /**
//        * Number of disabled but empty docks at the station.
//        * **Minimum**: 0
//        */
//       num_docks_disabled?: number;

//       /**
//        * Indicates whether the station is installed on the street.
//        */
//       is_installed: boolean;

//       /**
//        * Indicates whether the station is currently renting vehicles.
//        */
//       is_renting: boolean;

//       /**
//        * Indicates whether the station is accepting vehicle returns.
//        */
//       is_returning: boolean;

//       /**
//        * Last reported status time in RFC3339 format.
//        * **Format**: date-time
//        */
//       last_reported: string;

//       /**
//        * Details of docks available by vehicle type at the station.
//        */
//       vehicle_docks_available?: Array<{
//         vehicle_type_ids: string[];
//         count: number;
//       }>;
//     }>;
//   };
// }
