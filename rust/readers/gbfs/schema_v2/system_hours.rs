// /**
//  * # GBFS System Hours Schema V2.3, V2.2, V2.1, OR V2.0
//  * Describes the operating calendar for a system.
//  *
//  * ## Links
//  * - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_hoursjson)
//  * - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_hoursjson)
//  * - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_hoursjson)
//  * - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_hoursjson)
//  */
// export type GBFSSystemHoursV2 =
//   | GBFSSystemHoursV23
//   | GBFSSystemHoursV22
//   | GBFSSystemHoursV21
//   | GBFSSystemHoursV20;

// /**
//  * # GBFS System Hours V2.3
//  * Describes the system hours of operation.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_hoursjson)
//  */
// export interface GBFSSystemHoursV23 {
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
//    * Contains system hours data.
//    */
//   data: {
//     /**
//      * Rental hours for the system.
//      */
//     rental_hours: Array<{
//       /**
//        * Array of member and nonmember values indicating that this set of rental hours applies to either members or non-members only.
//        * **Enum**: ["member", "nonmember"]
//        * **Min Items**: 1
//        * **Max Items**: 2
//        */
//       user_types: ('member' | 'nonmember')[];

//       /**
//        * Abbreviations of English names of the days of the week.
//        * **Enum**: ["sun", "mon", "tue", "wed", "thu", "fri", "sat"]
//        * **Min Items**: 1
//        * **Max Items**: 7
//        */
//       days: ('sun' | 'mon' | 'tue' | 'wed' | 'thu' | 'fri' | 'sat')[];

//       /**
//        * Start time for the hours of operation of the system.
//        * **Pattern**: `^([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$`
//        */
//       start_time: string;

//       /**
//        * End time for the hours of operation of the system.
//        * **Pattern**: `^([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$`
//        */
//       end_time: string;
//     }>;
//   };
// }

// /**
//  * # GBFS System Hours V2.2
//  * Describes the system hours of operation.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_hoursjson)
//  */
// export interface GBFSSystemHoursV22 {
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
//    * Contains system hours data.
//    */
//   data: {
//     /**
//      * Rental hours for the system.
//      */
//     rental_hours: Array<{
//       /**
//        * Array of member and nonmember values indicating that this set of rental hours applies to either members or non-members only.
//        * **Enum**: ["member", "nonmember"]
//        * **Min Items**: 1
//        * **Max Items**: 2
//        */
//       user_types: ('member' | 'nonmember')[];

//       /**
//        * Abbreviations of English names of the days of the week.
//        * **Enum**: ["sun", "mon", "tue", "wed", "thu", "fri", "sat"]
//        * **Min Items**: 1
//        * **Max Items**: 7
//        */
//       days: ('sun' | 'mon' | 'tue' | 'wed' | 'thu' | 'fri' | 'sat')[];

//       /**
//        * Start time for the hours of operation of the system.
//        * **Pattern**: `^([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$`
//        */
//       start_time: string;

//       /**
//        * End time for the hours of operation of the system.
//        * **Pattern**: `^([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$`
//        */
//       end_time: string;
//     }>;
//   };
// }

// /**
//  * # GBFS System Hours V2.1
//  * Describes the system hours of operation.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_hoursjson)
//  */
// export interface GBFSSystemHoursV21 {
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
//    * Contains system hours data.
//    */
//   data: {
//     /**
//      * Rental hours for the system.
//      */
//     rental_hours: Array<{
//       /**
//        * Array of member and nonmember values indicating that this set of rental hours applies to either members or non-members only.
//        * **Enum**: ["member", "nonmember"]
//        * **Min Items**: 1
//        * **Max Items**: 2
//        */
//       user_types: ('member' | 'nonmember')[];

//       /**
//        * Abbreviations of English names of the days of the week.
//        * **Enum**: ["sun", "mon", "tue", "wed", "thu", "fri", "sat"]
//        * **Min Items**: 1
//        * **Max Items**: 7
//        */
//       days: ('sun' | 'mon' | 'tue' | 'wed' | 'thu' | 'fri' | 'sat')[];

//       /**
//        * Start time for the hours of operation of the system.
//        * **Pattern**: `^([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$`
//        */
//       start_time: string;

//       /**
//        * End time for the hours of operation of the system.
//        * **Pattern**: `^([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$`
//        */
//       end_time: string;
//     }>;
//   };
// }

// /**
//  * # GBFS System Hours V2.0
//  * Describes the system hours of operation.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_hoursjson)
//  */
// export interface GBFSSystemHoursV20 {
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
//    * Contains system hours data.
//    */
//   data: {
//     /**
//      * Rental hours for the system.
//      */
//     rental_hours: Array<{
//       /**
//        * Array of member and nonmember values indicating that this set of rental hours applies to either members or non-members only.
//        * **Enum**: ["member", "nonmember"]
//        * **Min Items**: 1
//        * **Max Items**: 2
//        */
//       user_types: ('member' | 'nonmember')[];

//       /**
//        * Abbreviations of English names of the days of the week.
//        * **Enum**: ["sun", "mon", "tue", "wed", "thu", "fri", "sat"]
//        * **Min Items**: 1
//        * **Max Items**: 7
//        */
//       days: ('sun' | 'mon' | 'tue' | 'wed' | 'thu' | 'fri' | 'sat')[];

//       /**
//        * Start time for the hours of operation of the system.
//        * **Pattern**: `^([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$`
//        */
//       start_time: string;

//       /**
//        * End time for the hours of operation of the system.
//        * **Pattern**: `^([0-1][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$`
//        */
//       end_time: string;
//     }>;
//   };
// }
