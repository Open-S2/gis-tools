/**
 * # GBFS System Pricing Plans Schema V1.1 OR GBFS System Pricing Plans Schema V1.0
 * Describes the pricing schemes of the system.
 *
 * ## Links
 * - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_pricing_plansjson)
 * - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_pricing_plansjson)
 */
export type GBFSSystemPricingPlansV1 = GBFSSystemPricingPlansV11 | GBFSSystemPricingPlansV10;

/**
 * # GBFS System Pricing Plans Schema V1.1
 * Describes the pricing schemes of the system.
 *
 * ## Links
 * - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_pricing_plansjson)
 */
export const gbfsSystemPricingPlansSchemaV11 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_pricing_plansjson',
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
      const: '1.1',
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
                type: 'number',
                minimum: 0,
                maximum: 1,
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
 * GBFS System Pricing Plans Schema V1.1 Interface
 */
export interface GBFSSystemPricingPlansV11 {
  /** Last time the data in the feed was updated in POSIX time. */
  last_updated: number;

  /** Number of seconds before the data in the feed will be updated again. */
  ttl: number;

  /** GBFS version number (1.1). */
  version: '1.1';

  /** Data containing pricing plans of the system. */
  data: {
    plans: Array<{
      plan_id: string;
      url?: string;
      name: string;
      currency: string;
      price: number;
      is_taxable: number;
      description: string;
    }>;
  };
}

/**
 * # GBFS System Pricing Plans Schema V1.0
 * Describes the pricing schemes of the system.
 *
 * ## Links
 * - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_pricing_plansjson)
 */
export const gbfsSystemPricingPlansSchemaV10 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_pricing_plansjson',
  description: 'Describes the pricing schemes of the system.',
  type: 'object',
  properties: {
    last_updated: {
      description: 'Last time the data in the feed was updated in POSIX time.',
      type: 'integer',
      minimum: 0,
      maximum: 1924988399,
    },
    ttl: {
      description:
        'Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).',
      type: 'integer',
      minimum: 0,
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
              },
              name: {
                description: 'Name of this pricing plan.',
                type: 'string',
              },
              currency: {
                description: 'Currency used to pay the fare in ISO 4217 code.',
                type: 'string',
                minLength: 3,
                maxLength: 3,
              },
              price: {
                description: 'Fare price.',
                type: 'number',
              },
              is_taxable: {
                description: 'Will additional tax be added to the base price?',
                type: 'number',
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
  required: ['last_updated', 'ttl', 'data'],
};

/**
 * GBFS System Pricing Plans Schema V1.0 Interface
 */
export interface GBFSSystemPricingPlansV10 {
  /** Last time the data in the feed was updated in POSIX time. */
  last_updated: number;

  /** Number of seconds before the data in the feed will be updated again. */
  ttl: number;

  /** Data containing pricing plans of the system. */
  data: {
    plans: Array<{
      plan_id: string;
      url?: string;
      name: string;
      currency: string;
      price: number;
      is_taxable: number;
      description: string;
    }>;
  };
}
