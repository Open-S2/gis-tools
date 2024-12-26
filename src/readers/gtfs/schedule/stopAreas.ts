import { parseCSVAsRecord } from '../../';

/**
 * # Stop Areas
 *
 * **Optional**
 * Assigns stops to areas. Multiple rows can reference the same `area_id` to
 * indicate that different stops belong to the same area. Conversely, a single
 * `stop_id` can appear in multiple areas if needed.
 */
export class GTFSStopArea {
  /**
   * **Required**
   * Identifies an area (`areas.area_id`).
   */
  areaId: string;
  /**
   * **Required**
   * Identifies a stop (`stops.stop_id`). If a station is defined (location_type=1),
   * it implies all its child platforms (location_type=0) also belong to this area,
   * unless otherwise assigned.
   */
  stopId: string;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.areaId = data.area_id;
    this.stopId = data.stop_id;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of StopAreas
 */
export function parseGTFSStopAreas(input: string): GTFSStopArea[] {
  const data = parseCSVAsRecord(input);
  return data.map((d) => new GTFSStopArea(d));
}
