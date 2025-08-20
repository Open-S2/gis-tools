// import type { MultiPolygonGeometry } from '../../../index.js';

// /**
//  * # GBFS Manifest Schema V3.1-RC & V3.0
//  * An index of gbfs.json URLs for each GBFS data set produced by a publisher. A single instance of
//  * this file should be published at a single stable URL, for example: https://example.com/gbfs/manifest.json.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#manifestjson)
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#manifestjson)
//  */
// export type GBFSManifestV3 = GBFSManifestV30;

// /**
//  * # GBFS Manifest Schema V3.0
//  * An index of gbfs.json URLs for each GBFS data set produced by a publisher. A single instance of
//  * this file should be published at a single stable URL, for example: https://example.com/gbfs/manifest.json.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#manifestjson)
//  */
// export interface GBFSManifestV30 {
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
//    * GBFS version number to which the feed conforms, according to the versioning framework.
//    * **Const**: '3.0'
//    */
//   version: '3.0';

//   /**
//    * Data object containing the list of datasets.
//    */
//   data: {
//     /**
//      * Array of datasets containing system IDs and versions.
//      */
//     datasets: Array<{
//       /**
//        * The `system_id` from system_information.json for the corresponding data set(s).
//        */
//       system_id: string;

//       /**
//        * Array of available versions of the feed, sorted by increasing MAJOR and MINOR version number.
//        */
//       versions: Array<{
//         /**
//          * Semantic version of the feed in the form X.Y.
//          */
//         version: '1.0' | '1.1' | '2.0' | '2.1' | '2.2' | '2.3' | '3.0';

//         /**
//          * URL of the corresponding gbfs.json endpoint.
//          * **Format**: uri
//          */
//         url: string;
//       }>;
//     }>;
//   };
// }
