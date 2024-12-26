import { parseCSVAsRecord } from '../../';

/**
 * # Location Groups
 *
 * **Optional**
 * Defines groups of stops where a rider may request pickup or drop off.
 * `location_group_id` must be unique across:
 * - stops.stop_id
 * - locations.geojson ID
 * - location_groups.location_group_id
 */
export class GTFSLocationGroup {
  /**
   * **Required**
   * Identifies a location group. Must be unique (e.g., "zoneA", "northSideGroup").
   */
  id: string;
  /**
   * **Optional**
   * The name of the location group as displayed to the rider.
   */
  name?: string;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.id = data.location_group_id;
    this.name = data.location_group_name;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of GTFSLocationGroups
 */
export function parseGTFSLocationGroups(input: string): Record<string, GTFSLocationGroup> {
  const data = parseCSVAsRecord(input);
  const res: Record<string, GTFSLocationGroup> = {};
  for (const d of data) {
    const lg = new GTFSLocationGroup(d);
    res[lg.id] = lg;
  }

  return res;
}
