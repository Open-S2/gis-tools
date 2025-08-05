// /**
//  * # GBFS System Pricing Plans Schema V3.1-RC & V3.0
//  * Describes the pricing schemes of the system.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_pricing_plansjson)
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_pricing_plansjson)
//  */
// export type GBFSSystemPricingPlansV3 = GBFSSystemPricingPlansV31RC | GBFSSystemPricingPlansV30;

// /**
//  * # GBFS System Pricing Plans Schema V3.1-RC
//  * Describes the pricing schemes of the system.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_pricing_plansjson)
//  */
// export interface GBFSSystemPricingPlansV31RC {
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
//    * Array containing pricing plans for the system.
//    */
//   data: {
//     /**
//      * Array of pricing plans.
//      */
//     plans: Array<{
//       /**
//        * Identifier of a pricing plan in the system.
//        */
//       plan_id: string;

//       /**
//        * URL where the customer can learn more about this pricing plan.
//        * **Format**: uri
//        */
//       url?: string;

//       /**
//        * Name of this pricing plan.
//        */
//       name: Array<{
//         /**
//          * Translated text of the name.
//          */
//         text: string;

//         /**
//          * IETF BCP 47 language code.
//          * **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
//          */
//         language: string;
//       }>;

//       /**
//        * Currency used to pay the fare in ISO 4217 code.
//        * **Pattern**: `^\w{3}$`
//        */
//       currency: string;

//       /**
//        * Fare price.
//        * **Minimum**: 0
//        */
//       price: number;

//       /**
//        * Cost per minute to reserve the vehicle prior to rental.
//        * **Minimum**: 0
//        */
//       reservation_price_per_min?: number;

//       /**
//        * Flat rate to reserve the vehicle prior to rental.
//        * **Minimum**: 0
//        */
//       reservation_price_flat_rate?: number;

//       /**
//        * Indicates whether additional tax will be added to the base price.
//        */
//       is_taxable: boolean;

//       /**
//        * Customer-readable description of the pricing plan.
//        */
//       description: Array<{
//         /**
//          * Translated text of the description.
//          */
//         text: string;

//         /**
//          * IETF BCP 47 language code.
//          * **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
//          */
//         language: string;
//       }>;

//       /**
//        * Pricing based on distance traveled in kilometers.
//        */
//       per_km_pricing?: Array<{
//         /**
//          * Number of kilometers after which this segment applies.
//          * **Minimum**: 0
//          */
//         start: number;

//         /**
//          * Rate charged for each kilometer in this segment.
//          */
//         rate: number;

//         /**
//          * Interval in kilometers at which the rate is reapplied.
//          * **Minimum**: 0
//          */
//         interval: number;

//         /**
//          * Kilometer at which the rate no longer applies.
//          * **Minimum**: 0
//          */
//         end?: number;
//       }>;

//       /**
//        * Pricing based on time traveled in minutes.
//        */
//       per_min_pricing?: Array<{
//         /**
//          * Number of minutes after which this segment applies.
//          * **Minimum**: 0
//          */
//         start: number;

//         /**
//          * Rate charged for each minute in this segment.
//          */
//         rate: number;

//         /**
//          * Interval in minutes at which the rate is reapplied.
//          * **Minimum**: 0
//          */
//         interval: number;

//         /**
//          * Minute at which the rate no longer applies.
//          * **Minimum**: 0
//          */
//         end?: number;
//       }>;

//       /**
//        * Indicates whether surge pricing is currently applied.
//        */
//       surge_pricing?: boolean;
//     }>;
//   };
// }

// /**
//  * # GBFS System Pricing Plans Schema V3.0
//  * Describes the pricing schemes of the system.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_pricing_plansjson)
//  */
// export interface GBFSSystemPricingPlansV30 {
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
//    * Pricing plan data.
//    */
//   data: {
//     /**
//      * Array of pricing plans.
//      */
//     plans: Array<{
//       /**
//        * Identifier of the pricing plan.
//        */
//       plan_id: string;

//       /**
//        * URL where customers can learn more about this pricing plan.
//        */
//       url?: string;

//       /**
//        * Name of the pricing plan.
//        */
//       name: Array<{
//         text: string;
//         language: string;
//       }>;

//       /**
//        * Currency in ISO 4217 format.
//        */
//       currency: string;

//       /**
//        * Base price of the pricing plan.
//        */
//       price: number;

//       /**
//        * Indicates if additional tax is applied to the base price.
//        */
//       is_taxable: boolean;

//       /**
//        * Description of the pricing plan.
//        */
//       description: Array<{
//         text: string;
//         language: string;
//       }>;

//       /**
//        * Segments for distance-based pricing.
//        */
//       per_km_pricing?: Array<{
//         start: number;
//         rate: number;
//         interval: number;
//         end?: number;
//       }>;

//       /**
//        * Segments for time-based pricing.
//        */
//       per_min_pricing?: Array<{
//         start: number;
//         rate: number;
//         interval: number;
//         end?: number;
//       }>;

//       /**
//        * Indicates if surge pricing is active.
//        */
//       surge_pricing?: boolean;
//     }>;
//   };
// }
