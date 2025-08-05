// import { TIMEZONE_IDENTIFIER_LIST } from '../../../index.js';

// import type { TimeZone } from '../../../index.js';

// /**
//  * # GBFS System Information Schema V2.3, V2.2, V2.1, OR V2.0
//  * Details including system operator, system location, year implemented, URL, contact info, time zone.
//  *
//  * ## Links
//  * - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_informationjson)
//  * - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_informationjson)
//  * - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_informationjson)
//  * - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_informationjson)
//  */
// export type GBFSSystemInformationV2 =
//   | GBFSSystemInformationV23
//   | GBFSSystemInformationV22
//   | GBFSSystemInformationV21
//   | GBFSSystemInformationV20;

// /**
//  * # GBFS System Information V2.3
//  * Details including system operator, system location, year implemented, URL, contact info, and time zone.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_informationjson)
//  */
// export interface GBFSSystemInformationV23 {
//   last_updated: number;
//   ttl: number;
//   version: '2.3';
//   data: {
//     system_id: string;
//     language: string; // Matches BCP-47 language tags
//     name: string;
//     short_name?: string;
//     operator?: string;
//     url?: string;
//     purchase_url?: string;
//     start_date?: string; // ISO 8601 format
//     phone_number?: string;
//     email?: string;
//     feed_contact_email?: string;
//     timezone: TimeZone;
//     license_url?: string;
//     brand_assets?: {
//       brand_last_modified: string; // ISO 8601 format
//       brand_terms_url?: string;
//       brand_image_url: string;
//       brand_image_url_dark?: string;
//       color?: string; // Hexadecimal color code
//     };
//     terms_url?: string;
//     terms_last_updated?: string; // ISO 8601 format
//     privacy_url?: string;
//     privacy_last_updated?: string; // ISO 8601 format
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

// /**
//  * # GBFS System Information Schema V2.2
//  * Details including system operator, system location, year implemented, URL, contact info, and time zone.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_informationjson)
//  */
// export interface GBFSSystemInformationV22 {
//   last_updated: number;
//   ttl: number;
//   version: '2.2';
//   data: {
//     system_id: string;
//     language: string; // Matches BCP-47 language tags
//     name: string;
//     short_name?: string;
//     operator?: string;
//     url?: string;
//     purchase_url?: string;
//     start_date?: string; // ISO 8601 format
//     phone_number?: string;
//     email?: string;
//     feed_contact_email?: string;
//     timezone: TimeZone;
//     license_url?: string;
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

// /**
//  * # GBFS System Information Schema V2.1
//  * Details including system operator, system location, year implemented, URL, contact info, and time zone.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_informationjson)
//  */
// export interface GBFSSystemInformationV21 {
//   last_updated: number;
//   ttl: number;
//   version: '2.1';
//   data: {
//     system_id: string;
//     language: string; // Matches BCP-47 language tags
//     name: string;
//     short_name?: string;
//     operator?: string;
//     url?: string;
//     purchase_url?: string;
//     start_date?: string; // ISO 8601 format
//     phone_number?: string;
//     email?: string;
//     feed_contact_email?: string;
//     timezone: TimeZone;
//     license_url?: string;
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

// /**
//  * # GBFS System Information Schema V2.0
//  * Details including system operator, system location, year implemented, URL, contact info, and time zone.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_informationjson)
//  */
// export interface GBFSSystemInformationV20 {
//   last_updated: number;
//   ttl: number;
//   version: '2.0';
//   data: {
//     system_id: string;
//     language: string; // Matches BCP-47 language tags
//     name: string;
//     short_name?: string;
//     operator?: string;
//     url?: string;
//     purchase_url?: string;
//     start_date?: string; // ISO 8601 format
//     phone_number?: string;
//     email?: string;
//     feed_contact_email?: string;
//     timezone: TimeZone;
//     license_url?: string;
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
