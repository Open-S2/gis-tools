// import { LICENSES, TIMEZONE_IDENTIFIER_LIST } from '../../../index.js';

// import type { Licenses, TimeZone } from '../../../index.js';

// /**
//  * # GBFS System Information Schema V3.1-RC & V3.0
//  * Details including system operator, system location, year implemented, URL, contact info, time zone.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_informationjson)
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_informationjson)
//  */
// export type GBFSSystemInformationV3 = GBFSSystemInformationV30;

// /**
//  * # GBFS System Information Schema V3.0
//  * Details including system operator, system location, year implemented, URL, contact info, time zone.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_informationjson)
//  */
// export interface GBFSSystemInformationV30 {
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
//    * System information data object.
//    */
//   data: {
//     /**
//      * Globally unique identifier for the system.
//      */
//     system_id: string;

//     /**
//      * List of languages used in translated strings.
//      */
//     languages: string[];

//     /**
//      * Name of the system to be displayed to customers.
//      */
//     name: Array<{
//       text: string;
//       language: string;
//     }>;

//     /**
//      * Hours and dates of operation in OSM opening_hours format.
//      */
//     opening_hours: string;

//     /**
//      * Abbreviation for the system.
//      */
//     short_name?: Array<{
//       text: string;
//       language: string;
//     }>;

//     /**
//      * Name of the system operator.
//      */
//     operator?: Array<{
//       text: string;
//       language: string;
//     }>;

//     /**
//      * URL of the vehicle share system.
//      */
//     url?: string;

//     /**
//      * URL to purchase a membership.
//      */
//     purchase_url?: string;

//     /**
//      * Date the system began operations.
//      */
//     start_date?: string;

//     /**
//      * Date after which the data source will no longer be available.
//      */
//     termination_date?: string;

//     /**
//      * Customer service phone number in E.164 format.
//      */
//     phone_number?: string;

//     /**
//      * Email address actively monitored by customer service.
//      */
//     email?: string;

//     /**
//      * Contact email for feed consumers to report technical issues.
//      */
//     feed_contact_email: string;

//     /**
//      * URL to the manifest.json file for the publisher.
//      */
//     manifest_url?: string;

//     /**
//      * Time zone of the system.
//      */
//     timezone: TimeZone;

//     /**
//      * Standard license identifier for the dataset.
//      */
//     license_id?: Licenses;

//     /**
//      * URL defining the license terms.
//      */
//     license_url?: string;

//     /**
//      * Name of the organization to which attribution should be provided.
//      */
//     attribution_organization_name?: Array<{
//       text: string;
//       language: string;
//     }>;

//     /**
//      * URL of the organization for attribution.
//      */
//     attribution_url?: string;

//     /**
//      * Brand assets and related information.
//      */
//     brand_assets?: {
//       brand_last_modified: string;
//       brand_terms_url?: string;
//       brand_image_url: string;
//       brand_image_url_dark?: string;
//       color?: string;
//     };

//     /**
//      * Terms of service URL.
//      */
//     terms_url?: Array<{
//       text: string;
//       language: string;
//     }>;

//     /**
//      * Date terms of service were last updated.
//      */
//     terms_last_updated?: string;

//     /**
//      * Privacy policy URL.
//      */
//     privacy_url?: Array<{
//       text: string;
//       language: string;
//     }>;

//     /**
//      * Date the privacy policy was last updated.
//      */
//     privacy_last_updated?: string;

//     /**
//      * Rental app information for Android and iOS platforms.
//      */
//     rental_apps?: {
//       android?: {
//         store_uri: string;
//         discovery_uri: string;
//       };
//       ios?: {
//         store_uri: string;
//         discovery_uri: string;
//       };
//     };
//   };
// }
