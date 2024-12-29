/**
 * # GBFS System Pricing Plans Schema V3.1-RC & V3.0
 * Describes the pricing schemes of the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_pricing_plansjson)
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_pricing_plansjson)
 */
export type GBFSSystemPricingPlansV3 = GBFSSystemPricingPlansV31RC | GBFSSystemPricingPlansV30;

/**
 * # GBFS System Pricing Plans Schema V3.1-RC
 * Describes the pricing schemes of the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_pricing_plansjson)
 */
export const gbfsSystemPricingPlansSchemaV31RC = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_pricing_plansjson',
  description: 'Describes the pricing schemes of the system.',
  type: 'object',
  properties: {
    last_updated: {
      description: 'Last time the data in the feed was updated in RFC3339 format.',
      type: 'string',
      format: 'date-time',
    },
    ttl: {
      description:
        'Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).',
      type: 'integer',
      minimum: 0,
    },
    version: {
      description:
        'GBFS version number to which the feed conforms, according to the versioning framework (added in v1.1).',
      type: 'string',
      const: '3.1-RC',
    },
    data: {
      description: 'Array that contains one object per plan as defined below.',
      type: 'object',
      properties: {
        plans: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              plan_id: {
                description: 'Identifier of a pricing plan in the system.',
                type: 'string',
              },
              url: {
                description: 'URL where the customer can learn more about this pricing plan.',
                type: 'string',
                format: 'uri',
              },
              name: {
                description: 'Name of this pricing plan.',
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    text: {
                      description: 'The translated text.',
                      type: 'string',
                    },
                    language: {
                      description: 'IETF BCP 47 language code.',
                      type: 'string',
                      pattern: '^[a-z]{2,3}(-[A-Z]{2})?$',
                    },
                  },
                  required: ['text', 'language'],
                },
              },
              currency: {
                description: 'Currency used to pay the fare in ISO 4217 code.',
                type: 'string',
                pattern: '^\\w{3}$',
              },
              price: {
                description: 'Fare price.',
                type: 'number',
                minimum: 0,
              },
              reservation_price_per_min: {
                description:
                  'The cost, described as per minute rate, to reserve the vehicle prior to beginning a rental.',
                type: 'number',
                minimum: 0,
              },
              reservation_price_flat_rate: {
                description:
                  'The cost, described as a flat rate, to reserve the vehicle prior to beginning a rental.',
                type: 'number',
                minimum: 0,
              },
              is_taxable: {
                description: 'Will additional tax be added to the base price?',
                type: 'boolean',
              },
              description: {
                description: 'Customer-readable description of the pricing plan.',
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    text: {
                      description: 'The translated text.',
                      type: 'string',
                    },
                    language: {
                      description: 'IETF BCP 47 language code.',
                      type: 'string',
                      pattern: '^[a-z]{2,3}(-[A-Z]{2})?$',
                    },
                  },
                  required: ['text', 'language'],
                },
              },
              per_km_pricing: {
                description:
                  'Array of segments when the price is a function of distance travelled, displayed in kilometers (added in v2.1-RC2).',
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    start: {
                      description:
                        'Number of kilometers that have to elapse before this segment starts applying (added in v2.1-RC2).',
                      type: 'integer',
                      minimum: 0,
                    },
                    rate: {
                      description:
                        'Rate that is charged for each kilometer interval after the start (added in v2.1-RC2).',
                      type: 'number',
                    },
                    interval: {
                      description:
                        'Interval in kilometers at which the rate of this segment is either reapplied indefinitely, or if defined, up until (but not including) end kilometer (added in v2.1-RC2).',
                      type: 'integer',
                      minimum: 0,
                    },
                    end: {
                      description:
                        'The kilometer at which the rate will no longer apply (added in v2.1-RC2).',
                      type: 'integer',
                      minimum: 0,
                    },
                  },
                  required: ['start', 'rate', 'interval'],
                },
              },
              per_min_pricing: {
                description:
                  'Array of segments when the price is a function of time travelled, displayed in minutes (added in v2.1-RC2).',
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    start: {
                      description:
                        'Number of minutes that have to elapse before this segment starts applying (added in v2.1-RC2).',
                      type: 'integer',
                      minimum: 0,
                    },
                    rate: {
                      description:
                        'Rate that is charged for each minute interval after the start (added in v2.1-RC2).',
                      type: 'number',
                    },
                    interval: {
                      description:
                        'Interval in minutes at which the rate of this segment is either reapplied (added in v2.1-RC2).',
                      type: 'integer',
                      minimum: 0,
                    },
                    end: {
                      description:
                        'The minute at which the rate will no longer apply (added in v2.1-RC2).',
                      type: 'integer',
                      minimum: 0,
                    },
                  },
                  required: ['start', 'rate', 'interval'],
                },
              },
              surge_pricing: {
                description:
                  'Is there currently an increase in price in response to increased demand in this pricing plan? (added in v2.1-RC2)',
                type: 'boolean',
              },
            },
            required: ['plan_id', 'name', 'currency', 'price', 'is_taxable', 'description'],
            dependencies: {
              reservation_price_flat_rate: { not: { required: ['reservation_price_per_min'] } },
              reservation_price_per_min: { not: { required: ['reservation_price_flat_rate'] } },
            },
          },
        },
      },
      required: ['plans'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS System Pricing Plans Schema V3.1-RC
 * Describes the pricing schemes of the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_pricing_plansjson)
 */
export interface GBFSSystemPricingPlansV31RC {
  /**
   * Last time the data in the feed was updated in RFC3339 format.
   * **Format**: date-time
   */
  last_updated: string;

  /**
   * Number of seconds before the data in the feed will be updated again
   * (0 if the data should always be refreshed).
   * **Minimum**: 0
   */
  ttl: number;

  /**
   * GBFS version number to which the feed conforms, according to the versioning framework (added in v1.1).
   * **Const**: 3.1-RC
   */
  version: '3.1-RC';

  /**
   * Array containing pricing plans for the system.
   */
  data: {
    /**
     * Array of pricing plans.
     */
    plans: Array<{
      /**
       * Identifier of a pricing plan in the system.
       */
      plan_id: string;

      /**
       * URL where the customer can learn more about this pricing plan.
       * **Format**: uri
       */
      url?: string;

      /**
       * Name of this pricing plan.
       */
      name: Array<{
        /**
         * Translated text of the name.
         */
        text: string;

        /**
         * IETF BCP 47 language code.
         * **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
         */
        language: string;
      }>;

      /**
       * Currency used to pay the fare in ISO 4217 code.
       * **Pattern**: `^\w{3}$`
       */
      currency: string;

      /**
       * Fare price.
       * **Minimum**: 0
       */
      price: number;

      /**
       * Cost per minute to reserve the vehicle prior to rental.
       * **Minimum**: 0
       */
      reservation_price_per_min?: number;

      /**
       * Flat rate to reserve the vehicle prior to rental.
       * **Minimum**: 0
       */
      reservation_price_flat_rate?: number;

      /**
       * Indicates whether additional tax will be added to the base price.
       */
      is_taxable: boolean;

      /**
       * Customer-readable description of the pricing plan.
       */
      description: Array<{
        /**
         * Translated text of the description.
         */
        text: string;

        /**
         * IETF BCP 47 language code.
         * **Pattern**: `^[a-z]{2,3}(-[A-Z]{2})?$`
         */
        language: string;
      }>;

      /**
       * Pricing based on distance traveled in kilometers.
       */
      per_km_pricing?: Array<{
        /**
         * Number of kilometers after which this segment applies.
         * **Minimum**: 0
         */
        start: number;

        /**
         * Rate charged for each kilometer in this segment.
         */
        rate: number;

        /**
         * Interval in kilometers at which the rate is reapplied.
         * **Minimum**: 0
         */
        interval: number;

        /**
         * Kilometer at which the rate no longer applies.
         * **Minimum**: 0
         */
        end?: number;
      }>;

      /**
       * Pricing based on time traveled in minutes.
       */
      per_min_pricing?: Array<{
        /**
         * Number of minutes after which this segment applies.
         * **Minimum**: 0
         */
        start: number;

        /**
         * Rate charged for each minute in this segment.
         */
        rate: number;

        /**
         * Interval in minutes at which the rate is reapplied.
         * **Minimum**: 0
         */
        interval: number;

        /**
         * Minute at which the rate no longer applies.
         * **Minimum**: 0
         */
        end?: number;
      }>;

      /**
       * Indicates whether surge pricing is currently applied.
       */
      surge_pricing?: boolean;
    }>;
  };
}

/**
 * # GBFS System Pricing Plans Schema V3.0
 * Describes the pricing schemes of the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_pricing_plansjson)
 */
export const gbfsSystemPricingPlansSchemaV30 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_pricing_plansjson',
  description: 'Describes the pricing schemes of the system.',
  type: 'object',
  properties: {
    last_updated: {
      description: 'Last time the data in the feed was updated in RFC3339 format.',
      type: 'string',
      format: 'date-time',
    },
    ttl: {
      description:
        'Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).',
      type: 'integer',
      minimum: 0,
    },
    version: {
      description:
        'GBFS version number to which the feed conforms, according to the versioning framework (added in v1.1).',
      type: 'string',
      const: '3.0',
    },
    data: {
      description: 'Array that contains one object per plan as defined below.',
      type: 'object',
      properties: {
        plans: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              plan_id: {
                description: 'Identifier of a pricing plan in the system.',
                type: 'string',
              },
              url: {
                description: 'URL where the customer can learn more about this pricing plan.',
                type: 'string',
                format: 'uri',
              },
              name: {
                description: 'Name of this pricing plan.',
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    text: {
                      description: 'The translated text.',
                      type: 'string',
                    },
                    language: {
                      description: 'IETF BCP 47 language code.',
                      type: 'string',
                      pattern: '^[a-z]{2,3}(-[A-Z]{2})?$',
                    },
                  },
                  required: ['text', 'language'],
                },
              },
              currency: {
                description: 'Currency used to pay the fare in ISO 4217 code.',
                type: 'string',
                pattern: '^\\w{3}$',
              },
              price: {
                description: 'Fare price.',
                type: 'number',
                minimum: 0,
              },
              is_taxable: {
                description: 'Will additional tax be added to the base price?',
                type: 'boolean',
              },
              description: {
                description: 'Customer-readable description of the pricing plan.',
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    text: {
                      description: 'The translated text.',
                      type: 'string',
                    },
                    language: {
                      description: 'IETF BCP 47 language code.',
                      type: 'string',
                      pattern: '^[a-z]{2,3}(-[A-Z]{2})?$',
                    },
                  },
                  required: ['text', 'language'],
                },
              },
              per_km_pricing: {
                description:
                  'Array of segments when the price is a function of distance travelled, displayed in kilometers (added in v2.1-RC2).',
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    start: {
                      description:
                        'Number of kilometers that have to elapse before this segment starts applying (added in v2.1-RC2).',
                      type: 'integer',
                      minimum: 0,
                    },
                    rate: {
                      description:
                        'Rate that is charged for each kilometer interval after the start (added in v2.1-RC2).',
                      type: 'number',
                    },
                    interval: {
                      description:
                        'Interval in kilometers at which the rate of this segment is either reapplied indefinitely, or if defined, up until (but not including) end kilometer (added in v2.1-RC2).',
                      type: 'integer',
                      minimum: 0,
                    },
                    end: {
                      description:
                        'The kilometer at which the rate will no longer apply (added in v2.1-RC2).',
                      type: 'integer',
                      minimum: 0,
                    },
                  },
                  required: ['start', 'rate', 'interval'],
                },
              },
              per_min_pricing: {
                description:
                  'Array of segments when the price is a function of time travelled, displayed in minutes (added in v2.1-RC2).',
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    start: {
                      description:
                        'Number of minutes that have to elapse before this segment starts applying (added in v2.1-RC2).',
                      type: 'integer',
                      minimum: 0,
                    },
                    rate: {
                      description:
                        'Rate that is charged for each minute interval after the start (added in v2.1-RC2).',
                      type: 'number',
                    },
                    interval: {
                      description:
                        'Interval in minutes at which the rate of this segment is either reapplied (added in v2.1-RC2).',
                      type: 'integer',
                      minimum: 0,
                    },
                    end: {
                      description:
                        'The minute at which the rate will no longer apply (added in v2.1-RC2).',
                      type: 'integer',
                      minimum: 0,
                    },
                  },
                  required: ['start', 'rate', 'interval'],
                },
              },
              surge_pricing: {
                description:
                  'Is there currently an increase in price in response to increased demand in this pricing plan? (added in v2.1-RC2)',
                type: 'boolean',
              },
            },
            required: ['plan_id', 'name', 'currency', 'price', 'is_taxable', 'description'],
          },
        },
      },
      required: ['plans'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * # GBFS System Pricing Plans Schema V3.0
 * Describes the pricing schemes of the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_pricing_plansjson)
 */
export interface GBFSSystemPricingPlansV30 {
  /**
   * Last time the data in the feed was updated in RFC3339 format.
   */
  last_updated: string;

  /**
   * Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
   */
  ttl: number;

  /**
   * GBFS version number to which the feed conforms.
   */
  version: '3.0';

  /**
   * Pricing plan data.
   */
  data: {
    /**
     * Array of pricing plans.
     */
    plans: Array<{
      /**
       * Identifier of the pricing plan.
       */
      plan_id: string;

      /**
       * URL where customers can learn more about this pricing plan.
       */
      url?: string;

      /**
       * Name of the pricing plan.
       */
      name: Array<{
        text: string;
        language: string;
      }>;

      /**
       * Currency in ISO 4217 format.
       */
      currency: string;

      /**
       * Base price of the pricing plan.
       */
      price: number;

      /**
       * Indicates if additional tax is applied to the base price.
       */
      is_taxable: boolean;

      /**
       * Description of the pricing plan.
       */
      description: Array<{
        text: string;
        language: string;
      }>;

      /**
       * Segments for distance-based pricing.
       */
      per_km_pricing?: Array<{
        start: number;
        rate: number;
        interval: number;
        end?: number;
      }>;

      /**
       * Segments for time-based pricing.
       */
      per_min_pricing?: Array<{
        start: number;
        rate: number;
        interval: number;
        end?: number;
      }>;

      /**
       * Indicates if surge pricing is active.
       */
      surge_pricing?: boolean;
    }>;
  };
}
