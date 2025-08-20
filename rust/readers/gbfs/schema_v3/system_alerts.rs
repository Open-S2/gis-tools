// /**
//  * # GBFS System Alerts Schema V3.1-RC & V3.0
//  * Describes ad-hoc changes to the system.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_alertsjson)
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_alertsjson)
//  */
// export type GBFSSystemAlertsV3 = GBFSSystemAlertsV30;

// /**
//  * # GBFS System Alerts Schema V3.0
//  * Describes ad-hoc changes to the system.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_alertsjson)
//  */
// export interface GBFSSystemAlertsV30 {
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
//    * Data object containing system alerts.
//    */
//   data: {
//     /**
//      * Array of alerts for the system.
//      */
//     alerts: Array<{
//       /**
//        * Identifier for this alert.
//        */
//       alert_id: string;

//       /**
//        * Type of alert.
//        * Possible values: 'system_closure', 'station_closure', 'station_move', 'other'.
//        */
//       type: 'system_closure' | 'station_closure' | 'station_move' | 'other';

//       /**
//        * Array of objects indicating when the alert is in effect.
//        */
//       times?: Array<{
//         /**
//          * Start time of the alert in RFC3339 format.
//          * **Format**: date-time
//          */
//         start: string;

//         /**
//          * End time of the alert in RFC3339 format.
//          * **Format**: date-time
//          */
//         end?: string;
//       }>;

//       /**
//        * Array of identifiers of the stations for which this alert applies.
//        */
//       station_ids?: string[];

//       /**
//        * Array of identifiers of the regions for which this alert applies.
//        */
//       region_ids?: string[];

//       /**
//        * URL where customers can learn more information about this alert.
//        */
//       url?: Array<{
//         /**
//          * Translated text for the URL.
//          */
//         text: string;

//         /**
//          * IETF BCP 47 language code.
//          */
//         language: string;
//       }>;

//       /**
//        * Short summary of this alert to be displayed to the customer.
//        */
//       summary: Array<{
//         /**
//          * Translated text for the summary.
//          */
//         text: string;

//         /**
//          * IETF BCP 47 language code.
//          */
//         language: string;
//       }>;

//       /**
//        * Detailed description of the alert.
//        */
//       description?: Array<{
//         /**
//          * Translated text for the description.
//          */
//         text: string;

//         /**
//          * IETF BCP 47 language code.
//          */
//         language: string;
//       }>;

//       /**
//        * Indicates the last time the info for the alert was updated in RFC3339 format.
//        * **Format**: date-time
//        */
//       last_updated?: string;
//     }>;
//   };
// }
