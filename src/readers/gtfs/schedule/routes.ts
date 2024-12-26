import { parseCSVAsRecord } from '../../';

/**
 * # Route Information
 *
 * ## Details
 * **Required** - Transit routes. A route is a group of trips that are displayed to riders as a
 * single service.
 */
export class GTFSRoute {
  /**
   * **Required**
   * Identifies a route.
   */
  id: string;
  /**
   * **Conditionally Required**
   * Agency for the specified route.
   * Required if multiple agencies are defined in agency.txt.
   */
  agencyId?: string;
  /**
   * **Conditionally Required**
   * Short name of a route, e.g. "32", "100X", "Green".
   * Required if `route_long_name` is empty. Recommended otherwise.
   */
  shortName?: string;
  /**
   * **Conditionally Required**
   * Full name of a route, generally more descriptive than `shortName`.
   * Required if `route_short_name` is empty. Optional otherwise.
   */
  longName?: string;
  /**
   * **Optional**
   * Description of a route providing useful info, not a duplicate of short/long name.
   */
  desc?: string;
  /**
   * **Required**
   * Indicates the type of transportation used on a route.
   * Valid options include:
   * 0 - Tram, 1 - Subway, 2 - Rail, 3 - Bus, 4 - Ferry,
   * 5 - Cable tram, 6 - Aerial lift, 7 - Funicular,
   * 11 - Trolleybus, 12 - Monorail.
   */
  type: number;
  /**
   * **Optional**
   * URL of a web page about the route. Should differ from `agency.agency_url`.
   */
  url?: string;
  /**
   * **Optional**
   * Route color (hex) matching public-facing material. Defaults to `FFFFFF` if empty.
   */
  color?: string;
  /**
   * **Optional**
   * Text color (hex) used against the `route_color`. Defaults to `000000` if empty.
   */
  textColor?: string;
  /**
   * **Optional**
   * Orders routes for ideal presentation (smaller values displayed first).
   */
  sortOrder?: number;
  /**
   * **Conditionally Forbidden**
   * Continuous pickup setting for the entire route.
   * 0 - Continuous stopping pickup,
   * 1/empty - No continuous stopping pickup,
   * 2 - Must phone agency,
   * 3 - Must coordinate with driver.
   *
   * Forbidden if `stop_times.start_pickup_drop_off_window`
   * or `stop_times.end_pickup_drop_off_window` are used.
   */
  continuousPickup?: number;
  /**
   * **Conditionally Forbidden**
   * Continuous drop-off setting for the entire route.
   * 0 - Continuous stopping drop off,
   * 1/empty - No continuous stopping drop off,
   * 2 - Must phone agency,
   * 3 - Must coordinate with driver.
   *
   * Forbidden if `stop_times.start_pickup_drop_off_window`
   * or `stop_times.end_pickup_drop_off_window` are used.
   */
  continuousDropOff?: number;
  /**
   * **Conditionally Forbidden**
   * Identifies a group of routes. Multiple rows may share the same `network_id`.
   * Forbidden if `route_networks.txt` is used.
   */
  networkId?: string;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.id = data.route_id;
    this.agencyId = data.agency_id;
    this.shortName = data.route_short_name;
    this.longName = data.route_long_name;
    this.desc = data.route_desc;
    this.type = data.route_type !== undefined ? parseInt(data.route_type, 10) : 0; // default to 0 or handle as needed
    this.url = data.route_url;
    this.color = data.route_color;
    this.textColor = data.route_text_color;
    this.sortOrder =
      data.route_sort_order !== undefined ? parseInt(data.route_sort_order, 10) : undefined;
    this.continuousPickup =
      data.continuous_pickup !== undefined ? parseInt(data.continuous_pickup, 10) : undefined;
    this.continuousDropOff =
      data.continuous_drop_off !== undefined ? parseInt(data.continuous_drop_off, 10) : undefined;
    this.networkId = data.network_id;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of Routes
 */
export function parseGTFSRoutes(input: string): Record<string, GTFSRoute> {
  const data = parseCSVAsRecord(input);
  const res: Record<string, GTFSRoute> = {};
  for (const d of data) {
    const route = new GTFSRoute(d);
    res[route.id] = route;
  }

  return res;
}
