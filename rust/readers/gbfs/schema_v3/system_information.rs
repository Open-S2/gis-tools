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
// export type GBFSSystemInformationV3 = GBFSSystemInformationV31RC | GBFSSystemInformationV30;

// /**
//  * # GBFS System Information Schema V3.1-RC
//  * Details including system operator, system location, year implemented, URL, contact info, time zone.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_informationjson)
//  */
// export interface GBFSSystemInformationV31RC {
//   /**
//    * Last time the data in the feed was updated in RFC3339 format.
//    * **Format**: date-time
//    */
//   last_updated: string;

//   /**
//    * Number of seconds before the data in the feed will be updated again
//    * (0 if the data should always be refreshed).
//    * **Minimum**: 0
//    */
//   ttl: number;

//   /**
//    * GBFS version number to which the feed conforms, according to the versioning framework (added in v1.1).
//    * **Const**: 3.1-RC
//    */
//   version: '3.1-RC';

//   /**
//    * Response data in the form of name:value pairs.
//    */
//   data: {
//     /**
//      * Identifier for this vehicle share system. This should be globally unique.
//      */
//     system_id: string;

//     /**
//      * List of languages used in translated strings.
//      * Each element must be an IETF BCP 47 language code.
//      * **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
//      */
//     languages: string[];

//     /**
//      * Name of the system to be displayed to customers.
//      * An array with one object per supported language.
//      */
//     name: Array<{
//       text: string;
//       language: string;
//     }>;

//     /**
//      * Hours and dates of operation for the system in OSM opening_hours format. (added in v3.0)
//      */
//     opening_hours: string;

//     /**
//      * Abbreviation for the system. An array with one object per supported language.
//      */
//     short_name?: Array<{
//       text: string;
//       language: string;
//     }>;

//     /**
//      * Name of the system operator. An array with one object per supported language.
//      */
//     operator?: Array<{
//       text: string;
//       language: string;
//     }>;

//     /**
//      * The URL of the vehicle share system.
//      * **Format**: uri
//      */
//     url?: string;

//     /**
//      * URL where a customer can purchase a membership.
//      * **Format**: uri
//      */
//     purchase_url?: string;

//     /**
//      * Date that the system began operations.
//      * **Format**: date
//      */
//     start_date?: string;

//     /**
//      * Date after which this data source will no longer be available.
//      * **Format**: date
//      */
//     termination_date?: string;

//     /**
//      * Contact phone number for the customer service department.
//      * **Pattern**: `^\+[1-9]\d{1,14}$`
//      */
//     phone_number?: string;

//     /**
//      * Email address for customer service.
//      * **Format**: email
//      */
//     email?: string;

//     /**
//      * Contact email for consumers of this feed to report technical issues.
//      * **Format**: email
//      */
//     feed_contact_email: string;

//     /**
//      * A fully qualified URL pointing to the manifest.json file for the publisher.
//      * **Format**: uri
//      */
//     manifest_url?: string;

//     /**
//      * The time zone where the system is located.
//      * **Enum**: [... full list of timezones ...]
//      */
//     timezone: TimeZone;

//     /**
//      * License information, either `license_id` or `license_url` must be provided.
//      */
//     license_id?: Licenses;
//     license_url?: string; // **Format**: uri

//     /**
//      * Brand assets for the service.
//      */
//     brand_assets?: {
//       brand_last_modified: string; // **Format**: date
//       brand_terms_url?: string; // **Format**: uri
//       brand_image_url: string; // **Format**: uri
//       brand_image_url_dark?: string; // **Format**: uri
//       color?: string; // **Pattern**: `^#([a-fA-F0-9]{6})$`
//     };

//     /**
//      * Terms of service.
//      */
//     terms_url?: Array<{
//       text: string; // **Format**: uri
//       language: string; // **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
//     }>;
//     terms_last_updated?: string; // **Format**: date

//     /**
//      * Privacy policy.
//      */
//     privacy_url?: Array<{
//       text: string; // **Format**: uri
//       language: string; // **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
//     }>;
//     privacy_last_updated?: string; // **Format**: date

//     /**
//      * Rental app information.
//      */
//     rental_apps?: {
//       android?: {
//         store_uri: string; // **Format**: uri
//         discovery_uri: string; // **Format**: uri
//       };
//       ios?: {
//         store_uri: string; // **Format**: uri
//         discovery_uri: string; // **Format**: uri
//       };
//     };
//   };
// }

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
