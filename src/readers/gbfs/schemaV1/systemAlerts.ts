/**
 * # GBFS System Alerts Schema V1.1 OR GBFS System Alerts Schema V1.0
 * Describes ad-hoc changes to the system.
 *
 * ## Links
 * - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_alertsjson)
 * - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_alertsjson)
 */
export type GBFSSystemAlertsV1 = GBFSSystemAlertsV11 | GBFSSystemAlertsV10;

/**
 * # GBFS System Alerts Schema V1.1
 * Describes ad-hoc changes to the system.
 *
 * ## Links
 * - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_alertsjson)
 */
export const gbfsSystemAlertsSchemaV11 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_alertsjson',
  description: 'Describes ad-hoc changes to the system.',
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
      description: 'Array that contains ad-hoc alerts for the system.',
      type: 'object',
      properties: {
        alerts: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              alert_id: { description: 'Identifier for this alert.', type: 'string' },
              type: {
                description: 'Type of alert.',
                type: 'string',
                enum: ['SYSTEM_CLOSURE', 'STATION_CLOSURE', 'STATION_MOVE', 'OTHER'],
              },
              times: {
                description: 'Array of objects indicating when the alert is in effect.',
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    start: {
                      description: 'Start time of the alert.',
                      type: 'number',
                      minimum: 1450155600,
                    },
                    end: {
                      description: 'End time of the alert.',
                      type: 'number',
                      minimum: 1450155600,
                    },
                  },
                },
                additionalItems: false,
                required: ['start'],
              },
              station_ids: {
                description: 'Array of identifiers of the stations for which this alert applies.',
                type: 'array',
                items: { type: 'string' },
              },
              region_ids: {
                description: 'Array of identifiers of the regions for which this alert applies.',
                type: 'array',
                items: { type: 'string' },
              },
              url: {
                description: 'URL where the customer can learn more information about this alert.',
                type: 'string',
                format: 'uri',
              },
              summary: {
                description: 'A short summary of this alert to be displayed to the customer.',
                type: 'string',
              },
              description: {
                description: 'Detailed description of the alert.',
                type: 'string',
              },
              last_updated: {
                description: 'Indicates the last time the info for the alert was updated.',
                type: 'number',
                minimum: 1450155600,
              },
            },
            required: ['alert_id', 'type', 'summary'],
          },
        },
      },
      required: ['alerts'],
    },
  },
  required: ['last_updated', 'ttl', 'version', 'data'],
};

/**
 * GBFS System Alerts Schema V1.1 Interface
 */
export interface GBFSSystemAlertsV11 {
  /** Last time the data in the feed was updated in POSIX time. */
  last_updated: number;

  /** Number of seconds before the data in the feed will be updated again. */
  ttl: number;

  /** GBFS version number (1.1). */
  version: '1.1';

  /** Data containing ad-hoc alerts for the system. */
  data: {
    alerts: Array<{
      alert_id: string;
      type: 'SYSTEM_CLOSURE' | 'STATION_CLOSURE' | 'STATION_MOVE' | 'OTHER';
      times?: Array<{
        start: number;
        end?: number;
      }>;
      station_ids?: string[];
      region_ids?: string[];
      url?: string;
      summary: string;
      description?: string;
      last_updated?: number;
    }>;
  };
}

/**
 * # GBFS System Alerts Schema V1.0
 * Describes ad-hoc changes to the system.
 *
 * ## Links
 * - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_alertsjson)
 */
export const gbfsSystemAlertsSchemaV10 = {
  $schema: 'http://json-schema.org/draft-07/schema',
  $id: 'https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_alertsjson',
  description: 'Describes ad-hoc changes to the system.',
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
      description: 'Array that contains ad-hoc alerts for the system.',
      type: 'object',
      properties: {
        alerts: {
          type: 'array',
          items: {
            type: 'object',
            properties: {
              alert_id: { description: 'Identifier for this alert.', type: 'string' },
              type: {
                description: 'Type of alert.',
                type: 'string',
                enum: ['SYSTEM_CLOSURE', 'STATION_CLOSURE', 'STATION_MOVE', 'OTHER'],
              },
              times: {
                description: 'Array of objects indicating when the alert is in effect.',
                type: 'array',
                items: {
                  type: 'object',
                  properties: {
                    start: { type: 'number', minimum: 0 },
                    end: { type: 'number', minimum: 0 },
                  },
                },
                additionalItems: false,
                required: ['start'],
              },
              station_ids: {
                description: 'Array of identifiers of the stations for which this alert applies.',
                type: 'array',
                items: { type: 'string' },
              },
              region_ids: {
                description: 'Array of identifiers of the regions for which this alert applies.',
                type: 'array',
                items: { type: 'string' },
              },
              url: {
                description: 'URL where the customer can learn more information about this alert.',
                type: 'string',
              },
              summary: {
                description: 'A short summary of this alert to be displayed to the customer.',
                type: 'string',
              },
              description: {
                description: 'Detailed description of the alert.',
                type: 'string',
              },
              last_updated: {
                description: 'Indicates the last time the info for the alert was updated.',
                type: 'integer',
                minimum: 0,
                maximum: 1924988399,
              },
            },
            required: ['alert_id', 'type', 'summary'],
          },
        },
      },
      required: ['alerts'],
    },
  },
  required: ['last_updated', 'ttl', 'data'],
};

/**
 * GBFS System Alerts Schema V1.0 Interface
 */
export interface GBFSSystemAlertsV10 {
  /** Last time the data in the feed was updated in POSIX time. */
  last_updated: number;

  /** Number of seconds before the data in the feed will be updated again. */
  ttl: number;

  /** Data containing ad-hoc alerts for the system. */
  data: {
    alerts: Array<{
      alert_id: string;
      type: 'SYSTEM_CLOSURE' | 'STATION_CLOSURE' | 'STATION_MOVE' | 'OTHER';
      times?: Array<{
        start: number;
        end?: number;
      }>;
      station_ids?: string[];
      region_ids?: string[];
      url?: string;
      summary: string;
      description?: string;
      last_updated?: number;
    }>;
  };
}
