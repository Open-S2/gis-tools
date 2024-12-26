import { parseCSVAsRecord } from '../../';

/**
 * # Networks
 *
 * **Conditionally Forbidden**
 * Defines network identifiers. Used to group routes under a named network
 * for fare leg rules. This file is forbidden if `network_id` exists in `routes.txt`,
 * otherwise optional.
 */
export class GTFSNetwork {
  /**
   * **Required**
   * Identifies a network (`network_id`). Must be unique in `networks.txt`.
   */
  id: string;
  /**
   * **Optional**
   * The name of the network as used by the local agency and its riders.
   */
  name?: string;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.id = data.network_id;
    this.name = data.network_name;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of GTFSs
 */
export function parseGTFSNetworks(input: string): Record<string, GTFSNetwork> {
  const data = parseCSVAsRecord(input);
  const res: Record<string, GTFSNetwork> = {};
  for (const d of data) {
    const agency = new GTFSNetwork(d);
    res[agency.id] = agency;
  }

  return res;
}
