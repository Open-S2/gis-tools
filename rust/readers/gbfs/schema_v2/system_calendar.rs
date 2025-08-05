// /**
//  * # GBFS System Calendar Schema V2.3, V2.2, V2.1, OR V2.0
//  * Describes the operating calendar for a system.
//  *
//  * ## Links
//  * - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_calendarjson)
//  * - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_calendarjson)
//  * - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_calendarjson)
//  * - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_calendarjson)
//  */
// export type GBFSSystemCalendarV2 =
//   | GBFSSystemCalendarV23
//   | GBFSSystemCalendarV22
//   | GBFSSystemCalendarV21
//   | GBFSSystemCalendarV20;

// /**
//  * # GBFS System Calendar V2.3
//  * Describes the operating calendar for a system.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_calendarjson)
//  */
// export interface GBFSSystemCalendarV23 {
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
//    * Contains the operations calendar data.
//    */
//   data: {
//     /**
//      * Array of calendars for the system operations.
//      */
//     calendars: Array<{
//       /**
//        * Starting month for the system operations.
//        * **Minimum**: 1
//        * **Maximum**: 12
//        */
//       start_month: number;

//       /**
//        * Starting day for the system operations.
//        * **Minimum**: 1
//        * **Maximum**: 31
//        */
//       start_day: number;

//       /**
//        * Starting year for the system operations.
//        * **Pattern**: `^\\d{4}$`
//        */
//       start_year?: number;

//       /**
//        * End month for the system operations.
//        * **Minimum**: 1
//        * **Maximum**: 12
//        */
//       end_month: number;

//       /**
//        * End day for the system operations.
//        * **Minimum**: 1
//        * **Maximum**: 31
//        */
//       end_day: number;

//       /**
//        * End year for the system operations.
//        * **Pattern**: `^\\d{4}$`
//        */
//       end_year?: number;
//     }>;
//   };
// }

// /**
//  * # GBFS System Calendar V2.2
//  * Describes the operating calendar for a system.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_calendarjson)
//  */
// export interface GBFSSystemCalendarV22 {
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
//    * Contains the operations calendar data.
//    */
//   data: {
//     /**
//      * Array of calendars for the system operations.
//      */
//     calendars: Array<{
//       /**
//        * Starting month for the system operations.
//        * **Minimum**: 1
//        * **Maximum**: 12
//        */
//       start_month: number;

//       /**
//        * Starting day for the system operations.
//        * **Minimum**: 1
//        * **Maximum**: 31
//        */
//       start_day: number;

//       /**
//        * Starting year for the system operations.
//        * **Pattern**: `^\\d{4}$`
//        */
//       start_year?: number;

//       /**
//        * End month for the system operations.
//        * **Minimum**: 1
//        * **Maximum**: 12
//        */
//       end_month: number;

//       /**
//        * End day for the system operations.
//        * **Minimum**: 1
//        * **Maximum**: 31
//        */
//       end_day: number;

//       /**
//        * End year for the system operations.
//        * **Pattern**: `^\\d{4}$`
//        */
//       end_year?: number;
//     }>;
//   };
// }

// /**
//  * # GBFS System Calendar V2.1
//  * Describes the operating calendar for a system.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_calendarjson)
//  */
// export interface GBFSSystemCalendarV21 {
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
//    * Contains the operations calendar data.
//    */
//   data: {
//     /**
//      * Array of calendars for the system operations.
//      */
//     calendars: Array<{
//       /**
//        * Starting month for the system operations.
//        * **Minimum**: 1
//        * **Maximum**: 12
//        */
//       start_month: number;

//       /**
//        * Starting day for the system operations.
//        * **Minimum**: 1
//        * **Maximum**: 31
//        */
//       start_day: number;

//       /**
//        * Starting year for the system operations.
//        * **Pattern**: `^\\d{4}$`
//        */
//       start_year?: number;

//       /**
//        * End month for the system operations.
//        * **Minimum**: 1
//        * **Maximum**: 12
//        */
//       end_month: number;

//       /**
//        * End day for the system operations.
//        * **Minimum**: 1
//        * **Maximum**: 31
//        */
//       end_day: number;

//       /**
//        * End year for the system operations.
//        * **Pattern**: `^\\d{4}$`
//        */
//       end_year?: number;
//     }>;
//   };
// }

// /**
//  * # GBFS System Calendar V2.0
//  * Describes the operating calendar for a system.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_calendarjson)
//  */
// export interface GBFSSystemCalendarV20 {
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
//    * Contains the operations calendar data.
//    */
//   data: {
//     /**
//      * Array of calendars for the system operations.
//      */
//     calendars: Array<{
//       /**
//        * Starting month for the system operations.
//        * **Minimum**: 1
//        * **Maximum**: 12
//        */
//       start_month: number;

//       /**
//        * Starting day for the system operations.
//        * **Minimum**: 1
//        * **Maximum**: 31
//        */
//       start_day: number;

//       /**
//        * Starting year for the system operations.
//        * **Pattern**: `^\\d{4}$`
//        */
//       start_year?: number;

//       /**
//        * End month for the system operations.
//        * **Minimum**: 1
//        * **Maximum**: 12
//        */
//       end_month: number;

//       /**
//        * End day for the system operations.
//        * **Minimum**: 1
//        * **Maximum**: 31
//        */
//       end_day: number;

//       /**
//        * End year for the system operations.
//        * **Pattern**: `^\\d{4}$`
//        */
//       end_year?: number;
//     }>;
//   };
// }
