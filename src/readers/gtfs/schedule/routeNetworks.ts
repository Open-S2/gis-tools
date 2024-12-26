import { parseCSVAsRecord } from '../../';

/**
 * # Route Networks
 *
 * **Conditionally Forbidden**
 * Assigns routes (`routes.route_id`) to networks (`networks.network_id`).
 * This file is forbidden if `network_id` exists in `routes.txt`. Otherwise, it is optional.
 */
export class GTFSRouteNetwork {
  /**
   * **Required**
   * Identifies a network (`networks.network_id`) to which one or multiple routes belong.
   */
  networkId: string;
  /**
   * **Required**
   * Identifies a route (`routes.route_id`). One route can only belong to one network.
   */
  routeId: string;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.networkId = data.network_id;
    this.routeId = data.route_id;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of RouteNetworks
 */
export function parseGTFSRouteNetworks(input: string): GTFSRouteNetwork[] {
  const data = parseCSVAsRecord(input);
  return data.map((d) => new GTFSRouteNetwork(d));
}
