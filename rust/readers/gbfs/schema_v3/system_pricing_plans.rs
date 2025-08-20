// /**
//  * # GBFS System Pricing Plans Schema V3.1-RC & V3.0
//  * Describes the pricing schemes of the system.
//  *
//  * ## Links
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_pricing_plansjson)
//  * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_pricing_plansjson)
//  */
// export type GBFSSystemPricingPlansV3 = GBFSSystemPricingPlansV30;

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
