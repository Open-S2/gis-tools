import { parseCSVAsRecord } from '../../';

/**
 * # Location Group Stops
 *
 * **Optional**
 * Assigns stops from `stops.txt` to location groups (`location_groups.txt`).
 */
export class GTFSLocationGroupStop {
  /**
   * **Required**
   * Identifies a location group (`location_groups.location_group_id`).
   */
  locationGroupId: string;
  /**
   * **Required**
   * Identifies a stop (`stops.stop_id`) belonging to that location group.
   */
  stopId: string;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.locationGroupId = data.location_group_id;
    this.stopId = data.stop_id;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of GTFSLocationGroupStops
 */
export function parseGTFSLocationGroupStops(input: string): GTFSLocationGroupStop[] {
  const data = parseCSVAsRecord(input);
  return data.map((d) => new GTFSLocationGroupStop(d));
}
