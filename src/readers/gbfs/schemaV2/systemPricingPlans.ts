/**
 * # GBFS System Pricing Plans V2.3, V2.2, V2.1, OR V2.0
 * Describes the pricing schemes of the system.
 *
 * ## Links
 * - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_pricing_plansjson)
 * - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_pricing_plansjson)
 * - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_pricing_plansjson)
 * - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_pricing_plansjson)
 */
export type GBFSSystemPricingPlansV2 =
  | GBFSSystemPricingPlansV23
  | GBFSSystemPricingPlansV22
  | GBFSSystemPricingPlansV21
  | GBFSSystemPricingPlansV20;

/**
 * # GBFS System Pricing Plans Schema V2.3
 * Describes the pricing schemes of the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_pricing_plansjson)
 */
export const gbfsSystemPricingPlansSchemaV23 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_pricing_plansjson',
  description: 'Describes the pricing schemes of the system.',
  type: 'object',
  properties: {
    last_updated: {
      description: 'Last time the data in the feed was updated in POSIX time.',
      type: 'integer',
      minimum: 1450155600,
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
      const: '2.3',
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
                type: 'string',
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
                type: 'string',
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
 * # GBFS System Pricing Plans Schema V2.3
 * Describes the pricing schemes of the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_pricing_plansjson)
 */
export interface GBFSSystemPricingPlansV23 {
  last_updated: number;
  ttl: number;
  version: '2.3';
  data: {
    plans: Array<{
      plan_id: string;
      url?: string;
      name: string;
      currency: string; // ISO 4217 currency code
      price: number;
      is_taxable: boolean;
      description: string;
      per_km_pricing?: Array<{
        start: number;
        rate: number;
        interval: number;
        end?: number;
      }>;
      per_min_pricing?: Array<{
        start: number;
        rate: number;
        interval: number;
        end?: number;
      }>;
      surge_pricing?: boolean;
    }>;
  };
}

/**
 * # GBFS System Pricing Plans Schema V2.2
 * Describes the pricing schemes of the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_pricing_plansjson)
 */
export const gbfsSystemPricingPlansSchemaV22 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_pricing_plansjson',
  description: 'Describes the pricing schemes of the system.',
  type: 'object',
  properties: {
    last_updated: {
      description: 'Last time the data in the feed was updated in POSIX time.',
      type: 'integer',
      minimum: 1450155600,
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
      const: '2.2',
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
                type: 'string',
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
                type: 'string',
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
 * # GBFS System Pricing Plans Schema V2.2
 * Describes the pricing schemes of the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_pricing_plansjson)
 */
export interface GBFSSystemPricingPlansV22 {
  last_updated: number;
  ttl: number;
  version: '2.2';
  data: {
    plans: Array<{
      plan_id: string;
      url?: string;
      name: string;
      currency: string; // ISO 4217 currency code
      price: number;
      is_taxable: boolean;
      description: string;
      per_km_pricing?: Array<{
        start: number;
        rate: number;
        interval: number;
        end?: number;
      }>;
      per_min_pricing?: Array<{
        start: number;
        rate: number;
        interval: number;
        end?: number;
      }>;
      surge_pricing?: boolean;
    }>;
  };
}

/**
 * # GBFS System Pricing Plans Schema V2.1
 * Describes the pricing schemes of the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_pricing_plansjson)
 */
export const gbfsSystemPricingPlansSchemaV21 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_pricing_plansjson',
  description: 'Describes the pricing schemes of the system.',
  type: 'object',
  properties: {
    last_updated: {
      description: 'Last time the data in the feed was updated in POSIX time.',
      type: 'integer',
      minimum: 1450155600,
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
      const: '2.1',
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
                type: 'string',
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
                type: 'string',
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
 * # GBFS System Pricing Plans Schema V2.1
 * Describes the pricing schemes of the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_pricing_plansjson)
 */
export interface GBFSSystemPricingPlansV21 {
  last_updated: number;
  ttl: number;
  version: '2.1';
  data: {
    plans: Array<{
      plan_id: string;
      url?: string;
      name: string;
      currency: string; // ISO 4217 currency code
      price: number;
      is_taxable: boolean;
      description: string;
    }>;
  };
}

/**
 * # GBFS System Pricing Plans Schema V2.0
 * Describes the pricing schemes of the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_pricing_plansjson)
 */
export const gbfsSystemPricingPlansSchemaV20 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_pricing_plansjson',
  description: 'Describes the pricing schemes of the system.',
  type: 'object',
  properties: {
    last_updated: {
      description: 'Last time the data in the feed was updated in POSIX time.',
      type: 'integer',
      minimum: 1450155600,
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
      const: '2.0',
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
                type: 'string',
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
                type: 'string',
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
 * # GBFS System Pricing Plans Schema V2.0
 * Describes the pricing schemes of the system.
 *
 * ## Links
 * - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_pricing_plansjson)
 */
export interface GBFSSystemPricingPlansV20 {
  last_updated: number;
  ttl: number;
  version: '2.0';
  data: {
    plans: Array<{
      plan_id: string;
      url?: string;
      name: string;
      currency: string; // ISO 4217 currency code
      price: number;
      is_taxable: boolean;
      description: string;
    }>;
  };
}
