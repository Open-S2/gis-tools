// /**
//  * # GBFS Alerts Schema V2.3, V2.2, V2.1, OR V2.0
//  * Describes ad-hoc changes to the system.
//  *
//  * ## Links
//  * - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_alertsjson)
//  * - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_alertsjson)
//  * - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_alertsjson)
//  * - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_alertsjson)
//  */
// export type GBFSSystemAlertsV2 =
//   | GBFSSystemAlertsV23
//   | GBFSSystemAlertsV22
//   | GBFSSystemAlertsV21
//   | GBFSSystemAlertsV20;

// /**
//  * # GBFS System Alerts V2.3
//  * Describes ad-hoc changes to the system.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_alertsjson)
//  */
// export interface GBFSSystemAlertsV23 {
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
//    * Contains system alerts data.
//    */
//   data: {
//     /**
//      * Array of alerts for the system.
//      */
//     alerts: Array<{
//       alert_id: string;
//       type: 'system_closure' | 'station_closure' | 'station_move' | 'other';
//       times?: Array<{
//         start: number;
//         end?: number;
//       }>;
//       station_ids?: string[];
//       region_ids?: string[];
//       url?: string;
//       summary: string;
//       description?: string;
//       last_updated?: number;
//     }>;
//   };
// }

// /**
//  * # GBFS System Alerts V2.2
//  * Describes ad-hoc changes to the system.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_alertsjson)
//  */
// export interface GBFSSystemAlertsV22 {
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
//    * Contains system alerts data.
//    */
//   data: {
//     /**
//      * Array of alerts for the system.
//      */
//     alerts: Array<{
//       alert_id: string;
//       type: 'system_closure' | 'station_closure' | 'station_move' | 'other';
//       times?: Array<{
//         start: number;
//         end?: number;
//       }>;
//       station_ids?: string[];
//       region_ids?: string[];
//       url?: string;
//       summary: string;
//       description?: string;
//       last_updated?: number;
//     }>;
//   };
// }

// /**
//  * # GBFS System Alerts V2.1
//  * Describes ad-hoc changes to the system.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_alertsjson)
//  */
// export interface GBFSSystemAlertsV21 {
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
//    * Contains system alerts data.
//    */
//   data: {
//     /**
//      * Array of alerts for the system.
//      */
//     alerts: Array<{
//       alert_id: string;
//       type: 'system_closure' | 'station_closure' | 'station_move' | 'other';
//       times?: Array<{
//         start: number;
//         end?: number;
//       }>;
//       station_ids?: string[];
//       region_ids?: string[];
//       url?: string;
//       summary: string;
//       description?: string;
//       last_updated?: number;
//     }>;
//   };
// }

// /**
//  * # GBFS System Alerts V2.0
//  * Describes ad-hoc changes to the system.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_alertsjson)
//  */
// export interface GBFSSystemAlertsV20 {
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
//    * Contains system alerts data.
//    */
//   data: {
//     /**
//      * Array of alerts for the system.
//      */
//     alerts: Array<{
//       alert_id: string;
//       type: 'SYSTEM_CLOSURE' | 'STATION_CLOSURE' | 'STATION_MOVE' | 'OTHER';
//       times?: Array<{
//         start: number;
//         end?: number;
//       }>;
//       station_ids?: string[];
//       region_ids?: string[];
//       url?: string;
//       summary: string;
//       description?: string;
//       last_updated?: number;
//     }>;
//   };
// }
