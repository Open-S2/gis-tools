import { parseCSVAsRecord } from '../../';

/**
 * # Levels
 *
 * **Conditionally Required**
 * Describes levels in a station, useful with `pathways.txt`.
 * Required if `pathways` include elevators (`pathway_mode=5`), otherwise optional.
 */
export class GTFSLevel {
  /**
   * **Required**
   * Identifies a level in a station (`level_id`).
   */
  id: string;
  /**
   * **Required**
   * Numeric index indicating this level's relative position:
   * - 0 for ground level
   * - Positive above ground
   * - Negative below ground
   */
  levelIndex: number;
  /**
   * **Optional**
   * Name of the level as displayed to the rider (e.g., "Mezzanine", "Platform").
   */
  levelName?: string;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.id = data.level_id;
    // Convert to float or default to 0
    this.levelIndex = data.level_index !== undefined ? parseFloat(data.level_index) : 0;
    this.levelName = data.level_name;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of Level
 */
export function parseGTFSLevels(input: string): Record<string, GTFSLevel> {
  const data = parseCSVAsRecord(input);
  const res: Record<string, GTFSLevel> = {};
  for (const d of data) {
    const level = new GTFSLevel(d);
    res[level.id] = level;
  }

  return res;
}
