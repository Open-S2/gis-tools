// /**
//  * # GBFS Schema V3.1-RC OR GBFS Schema V3.0
//  * Auto-discovery file that links to all of the other files published by the system.
//  *
//  * ## Links
//  * - [GBFS Specification V3.1-RC](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#gbfsjson)
//  * - [GBFS Specification V3.0](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#gbfsjson)
//  */
// pub type GBFSV3 = GBFSV30;

// /**
//  * # GBFS Schema V3.0
//  * Auto-discovery file that links to all of the other files published by the system.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#gbfsjson)
//  */
// pub struct GBFSV30 {
//   /**
//    * Last time the data in the feed was updated in RFC3339 format.
//    * **Format**: date-time
//    */
//   last_updated: String,

//   /**
//    * Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
//    * **Minimum**: 0
//    */
//   ttl: u64,

//   /**
//    * GBFS version number to which the feed conforms, according to the versioning framework (added in v1.1).
//    * **Const**: 3.0
//    */
//   version: String,

//   /**
//    * Contains the data for feeds published by the auto-discovery file.
//    */
//   data: {
//     /**
//      * An array of all feeds published by the auto-discovery file. Each element is an object with the following keys:
//      */
//     feeds: Array<{
//       /**
//        * Key identifying the type of feed this is. The key must be the base file name defined in the spec for the corresponding feed type.
//        * **Enum**: ['gbfs', 'gbfs_versions', 'system_information', 'vehicle_types', 'station_information', 'station_status', 'vehicle_status', 'system_alerts', 'system_regions', 'system_pricing_plans', 'geofencing_zones']
//        */
//       name:
//         | 'gbfs'
//         | 'gbfs_versions'
//         | 'system_information'
//         | 'vehicle_types'
//         | 'station_information'
//         | 'station_status'
//         | 'vehicle_status'
//         | 'system_alerts'
//         | 'system_regions'
//         | 'system_pricing_plans'
//         | 'geofencing_zones';

//       /**
//        * URL for the feed.
//        * **Format**: uri
//        */
//       url: String,
//     }>;
//   };
// }
