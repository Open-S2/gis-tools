import { parseCSVAsRecord } from '../../';

/**
 * # Stop Information
 *
 * ## Details
 * **Conditionally Required** - Stops where vehicles pick up or drop off riders.
 * Also defines stations, entrances, etc.
 */
export class GTFSStop {
  /**
   * **Required**
   * Identifies a location: stop/platform, station, entrance/exit, generic node, or boarding area.
   * Must be unique across:
   * - stops.stop_id
   * - locations.geojson id
   * - location_groups.location_group_id
   * Multiple routes may use the same `stop_id`.
   */
  id: string;
  /**
   * **Optional**
   * Short text or a number that identifies the location for riders.
   */
  code?: string;
  /**
   * **Conditionally Required**
   * Name of the location. Required if `location_type` is 0, 1, or 2. Optional otherwise.
   */
  name?: string;
  /**
   * **Optional**
   * Readable version of the stop_name for text-to-speech systems.
   */
  ttsName?: string;
  /**
   * **Optional**
   * Description providing useful information about the location.
   * Should not be a duplicate of `name`.
   */
  desc?: string;
  /**
   * **Conditionally Required**
   * Latitude of the location. Required if `location_type` is 0, 1, or 2. Optional otherwise.
   */
  lat?: number;
  /**
   * **Conditionally Required**
   * Longitude of the location. Required if `location_type` is 0, 1, or 2. Optional otherwise.
   */
  lon?: number;
  /**
   * **Optional**
   * Identifies the fare zone for a stop.
   */
  zoneId?: string;
  /**
   * **Optional**
   * URL of a web page about this location.
   */
  url?: string;
  /**
   * **Optional**
   * Location type. Valid options:
   * 0 or empty = Stop/Platform,
   * 1 = Station,
   * 2 = Entrance/Exit,
   * 3 = Generic Node,
   * 4 = Boarding Area.
   */
  locationType?: number;
  /**
   * **Conditionally Required**
   * Defines hierarchy between different locations. Required if `location_type` is 2, 3, or 4.
   */
  parentStation?: string;
  /**
   * **Optional**
   * Timezone of the location. Inherits from parent station if not specified.
   */
  timezone?: string;
  /**
   * **Optional**
   * Indicates whether wheelchair boardings are possible at this location.
   * For parentless stops: 0 = no info, 1 = possible, 2 = not possible.
   * For child stops, entrance/exits: inherits or overrides parent station accessibility.
   */
  wheelchairBoarding?: number;
  /**
   * **Optional**
   * Level of the location. References levels.level_id.
   */
  levelId?: string;
  /**
   * **Optional**
   * Platform identifier for a platform stop.
   */
  platformCode?: string;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.id = data.stop_id;
    this.code = data.stop_code;
    this.name = data.stop_name;
    this.ttsName = data.tts_stop_name;
    this.desc = data.stop_desc;
    this.lat = data.stop_lat !== undefined ? parseFloat(data.stop_lat) : undefined;
    this.lon = data.stop_lon !== undefined ? parseFloat(data.stop_lon) : undefined;
    this.zoneId = data.zone_id;
    this.url = data.stop_url;
    this.locationType =
      data.location_type !== undefined ? parseInt(data.location_type, 10) : undefined;
    this.parentStation = data.parent_station;
    this.timezone = data.stop_timezone;
    this.wheelchairBoarding =
      data.wheelchair_boarding !== undefined ? parseInt(data.wheelchair_boarding, 10) : undefined;
    this.levelId = data.level_id;
    this.platformCode = data.platform_code;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of Stops
 */
export function parseGTFSStops(input: string): Record<string, GTFSStop> {
  const data = parseCSVAsRecord(input);
  const res: Record<string, GTFSStop> = {};
  for (const d of data) {
    const stop = new GTFSStop(d);
    res[stop.id] = stop;
  }

  return res;
}
