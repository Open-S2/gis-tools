// /**
//  * # GBFS Station Status Schema V2.3, V2.2, V2.1, OR V2.0
//  * List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
//  *
//  * ## Links
//  * - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#station_statusjson)
//  * - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#station_statusjson)
//  * - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#station_statusjson)
//  * - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#station_statusjson)
//  */
// export type GBFSStationStatusV2 =
//   | GBFSStationStatusV23
//   | GBFSStationStatusV22
//   | GBFSStationStatusV21
//   | GBFSStationStatusV20;

// /**
//  * # GBFS Station Status V2.3
//  * Describes the capacity and rental availability of the station.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#station_statusjson)
//  */
// export interface GBFSStationStatusV23 {
//   /**
//    * Last time the data in the feed was updated in POSIX time.
//    * **Minimum**: 1450155600
//    */
//   last_updated: number;

//   /**
//    * Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
//    * **Minimum**: 0
//    */
//   ttl: number;

//   /**
//    * GBFS version number to which the feed conforms, according to the versioning framework.
//    * **Const**: 2.3
//    */
//   version: '2.3';

//   /**
//    * Contains station status information.
//    */
//   data: {
//     /**
//      * Array of station status objects.
//      */
//     stations: Array<{
//       station_id: string;
//       num_bikes_available: number;
//       vehicle_types_available?: Array<{
//         vehicle_type_id: string;
//         count: number;
//       }>;
//       num_bikes_disabled?: number;
//       num_docks_available?: number;
//       num_docks_disabled?: number;
//       is_installed: boolean;
//       is_renting: boolean;
//       is_returning: boolean;
//       last_reported: number;
//       vehicle_docks_available?: Array<{
//         vehicle_type_ids: string[];
//         count: number;
//       }>;
//     }>;
//   };
// }

// /**
//  * # GBFS Station Status V2.2
//  * Describes the capacity and rental availability of the station.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#station_statusjson)
//  */
// export interface GBFSStationStatusV22 {
//   /**
//    * Last time the data in the feed was updated in POSIX time.
//    * **Minimum**: 1450155600
//    */
//   last_updated: number;

//   /**
//    * Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
//    * **Minimum**: 0
//    */
//   ttl: number;

//   /**
//    * GBFS version number to which the feed conforms, according to the versioning framework.
//    * **Const**: 2.2
//    */
//   version: '2.2';

//   /**
//    * Contains station status information.
//    */
//   data: {
//     /**
//      * Array of station status objects.
//      */
//     stations: Array<{
//       station_id: string;
//       num_bikes_available: number;
//       vehicle_types_available?: Array<{
//         vehicle_type_id: string;
//         count: number;
//       }>;
//       num_bikes_disabled?: number;
//       num_docks_available?: number;
//       num_docks_disabled?: number;
//       is_installed: boolean;
//       is_renting: boolean;
//       is_returning: boolean;
//       last_reported: number;
//       vehicle_docks_available?: Array<{
//         vehicle_type_ids: string[];
//         count: number;
//       }>;
//     }>;
//   };
// }

// /**
//  * # GBFS Station Status V2.1
//  * Describes the capacity and rental availability of the station.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#station_statusjson)
//  */
// export interface GBFSStationStatusV21 {
//   /**
//    * Last time the data in the feed was updated in POSIX time.
//    * **Minimum**: 1450155600
//    */
//   last_updated: number;

//   /**
//    * Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
//    * **Minimum**: 0
//    */
//   ttl: number;

//   /**
//    * GBFS version number to which the feed conforms, according to the versioning framework.
//    * **Const**: 2.1
//    */
//   version: '2.1';

//   /**
//    * Contains station status information.
//    */
//   data: {
//     /**
//      * Array of station status objects.
//      */
//     stations: Array<{
//       station_id: string;
//       num_bikes_available: number;
//       vehicle_types_available?: Array<{
//         vehicle_type_id: string;
//         count: number;
//       }>;
//       num_bikes_disabled?: number;
//       num_docks_available?: number;
//       num_docks_disabled?: number;
//       is_installed: boolean;
//       is_renting: boolean;
//       is_returning: boolean;
//       last_reported: number;
//       vehicle_docks_available?: Array<{
//         vehicle_type_ids: string[];
//         count: number;
//       }>;
//     }>;
//   };
// }

// /**
//  * # GBFS Station Status V2.0
//  * Describes the capacity and rental availability of the station.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#station_statusjson)
//  */
// export interface GBFSStationStatusV20 {
//   /**
//    * Last time the data in the feed was updated in POSIX time.
//    * **Minimum**: 1450155600
//    */
//   last_updated: number;

//   /**
//    * Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
//    * **Minimum**: 0
//    */
//   ttl: number;

//   /**
//    * GBFS version number to which the feed conforms, according to the versioning framework.
//    * **Const**: 2.0
//    */
//   version: '2.0';

//   /**
//    * Contains station status information.
//    */
//   data: {
//     /**
//      * Array of station status objects.
//      */
//     stations: Array<{
//       station_id: string;
//       num_bikes_available: number;
//       num_bikes_disabled?: number;
//       num_docks_available?: number;
//       num_docks_disabled?: number;
//       is_installed: boolean;
//       is_renting: boolean;
//       is_returning: boolean;
//       last_reported: number;
//     }>;
//   };
// }
