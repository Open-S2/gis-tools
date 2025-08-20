// /**
//  * # GBFS System Regions Schema V3.1-RC & V3.0
//  * Describes regions for a system that is broken up by geographic or political region.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_regionsjson)
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_regionsjson)
//  */
// export type GBFSSystemRegionsV3 = GBFSSystemRegionsV30;

// /**
//  * # GBFS System Regions Schema V3.0
//  * Describes regions for a system that is broken up by geographic or political region.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_regionsjson)
//  */
// export interface GBFSSystemRegionsV30 {
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
//    * Data describing regions for a system.
//    */
//   data: {
//     /**
//      * Array of regions.
//      */
//     regions: Array<{
//       /**
//        * Identifier of the region.
//        */
//       region_id: string;

//       /**
//        * Public name for this region.
//        */
//       name: Array<{
//         text: string;
//         language: string;
//       }>;
//     }>;
//   };
// }
