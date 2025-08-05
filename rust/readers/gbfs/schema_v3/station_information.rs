// import type { MultiPolygonGeometry } from '../../../index.js';

// /**
//  * # GBFS Station Information Schema V3.1-RC & V3.0
//  * List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#station_informationjson)
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#station_informationjson)
//  */
// export type GBFSStationInformationV3 = GBFSStationInformationV31RC | GBFSStationInformationV30;

// /**
//  * Information about a single station.
//  */
// export interface GBFSStationV3 {
//   /**
//    * Identifier of the station.
//    */
//   station_id: string;

//   /**
//    * Public name of the station.
//    */
//   name: Array<{
//     /**
//      * The translated text.
//      */
//     text: string;

//     /**
//      * IETF BCP 47 language code.
//      * **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
//      */
//     language: string;
//   }>;

//   /**
//    * The latitude of the station.
//    * **Minimum**: -90
//    * **Maximum**: 90
//    */
//   lat: number;

//   /**
//    * The longitude of the station.
//    * **Minimum**: -180
//    * **Maximum**: 180
//    */
//   lon: number;

//   /**
//    * Short name or alternative identifier for the station.
//    */
//   short_name?: Array<{
//     text: string;
//     language: string;
//   }>;

//   /**
//    * Address where the station is located.
//    */
//   address?: string;

//   /**
//    * Cross street or landmark where the station is located.
//    */
//   cross_street?: string;

//   /**
//    * Identifier of the region where the station is located.
//    */
//   region_id?: string;

//   /**
//    * Postal code where the station is located.
//    */
//   post_code?: string;

//   /**
//    * Hours of operation for the station in OSM opening_hours format.
//    */
//   station_opening_hours?: string;

//   /**
//    * Payment methods accepted at the station.
//    * **Enum**: ['key', 'creditcard', 'paypass', 'applepay', 'androidpay', 'transitcard', 'accountnumber', 'phone']
//    */
//   rental_methods?: Array<
//     | 'key'
//     | 'creditcard'
//     | 'paypass'
//     | 'applepay'
//     | 'androidpay'
//     | 'transitcard'
//     | 'accountnumber'
//     | 'phone'
//   >;

//   /**
//    * Is this station a location with or without physical infrastructure? (added in v2.1-RC)
//    */
//   is_virtual_station?: boolean;

//   /**
//    * A multipolygon describing the area of a virtual station. (added in v2.1-RC)
//    */
//   station_area?: MultiPolygonGeometry;

//   /**
//    * Type of parking station. (added in v2.3)
//    * **Enum**: ['parking_lot', 'street_parking', 'underground_parking', 'sidewalk_parking', 'other']
//    */
//   parking_type?:
//     | 'parking_lot'
//     | 'street_parking'
//     | 'underground_parking'
//     | 'sidewalk_parking'
//     | 'other';

//   /**
//    * Are parking hoops present at this station? (added in v2.3)
//    */
//   parking_hoop?: boolean;

//   /**
//    * Contact phone of the station. (added in v2.3)
//    */
//   contact_phone?: string;

//   /**
//    * Total docking points installed at the station, both available and unavailable.
//    * **Minimum**: 0
//    */
//   capacity?: number;

//   /**
//    * Parking capacity for virtual stations per vehicle type.
//    */
//   vehicle_types_capacity?: Array<{
//     vehicle_type_ids: string[];
//     count: number;
//   }>;

//   /**
//    * Docking capacity per vehicle type at the station.
//    */
//   vehicle_docks_capacity?: Array<{
//     vehicle_type_ids: string[];
//     count: number;
//   }>;

//   /**
//    * Are valet services provided at the station? (added in v2.1-RC)
//    */
//   is_valet_station?: boolean;

//   /**
//    * Does the station support charging of electric vehicles? (added in v2.3-RC)
//    */
//   is_charging_station?: boolean;

//   /**
//    * Rental URIs for Android, iOS, and web.
//    */
//   rental_uris?: {
//     /**
//      * URI for Android apps. (added in v1.1)
//      * **Format**: uri
//      */
//     android?: string;

//     /**
//      * URI for iOS apps. (added in v1.1)
//      * **Format**: uri
//      */
//     ios?: string;

//     /**
//      * URL for web browsers. (added in v1.1)
//      * **Format**: uri
//      */
//     web?: string;
//   };
// }

// /**
//  * # GBFS Station Information Schema V3.1-RC
//  * List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#station_informationjson)
//  */
// export interface GBFSStationInformationV31RC {
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
//    * Contains station data for the system.
//    */
//   data: {
//     /**
//      * Array of stations, each containing location, capacity, and other metadata.
//      */
//     stations: GBFSStationV3[];
//   };
// }

// /**
//  * # GBFS Station Information Schema V3.0
//  * List of all stations, their capacities, and locations. REQUIRED for systems utilizing docks.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#station_informationjson)
//  */
// export interface GBFSStationInformationV30 {
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
//    * Data object containing station information.
//    */
//   data: {
//     /**
//      * List of stations with their attributes.
//      */
//     stations: GBFSStationV3[];
//   };
// }
