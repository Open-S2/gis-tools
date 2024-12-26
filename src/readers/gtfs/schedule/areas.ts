import { parseCSVAsRecord } from '../../';

/**
 * # Areas
 *
 * **Optional**
 * Defines area identifiers.
 * Each record in `areas.txt` contains a unique `area_id` that can be referenced
 * in `stop_areas.txt`.
 */
export class GTFSArea {
  /**
   * **Required**
   * Identifies an area (`area_id`). Must be unique within `areas.txt`.
   */
  areaId: string;
  /**
   * **Optional**
   * Name of the area as displayed to the rider.
   */
  areaName?: string;
  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.areaId = data.area_id;
    this.areaName = data.area_name;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of Areas
 */
export function parseGTFSAreas(input: string): GTFSArea[] {
  const data = parseCSVAsRecord(input);
  return data.map((d) => new GTFSArea(d));
}
