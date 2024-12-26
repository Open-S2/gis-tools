import { parseCSVAsRecord } from '../../';

/**
 * Describes the type of pathway between two stops or station nodes.
 *
 * 1 - Walkway
 * 2 - Stairs
 * 3 - Moving sidewalk (travelator)
 * 4 - Escalator
 * 5 - Elevator
 * 6 - Fare gate (payment gate)
 * 7 - Exit gate
 */
export enum GTFSPathwayMode {
  Walkway = 1,
  Stairs = 2,
  MovingSidewalk = 3,
  Escalator = 4,
  Elevator = 5,
  FareGate = 6,
  ExitGate = 7,
}

/**
 * Indicates whether a pathway can be used in both directions:
 *
 * 0 - Unidirectional
 * 1 - Bidirectional
 *
 * Note: Exit gates (pathway_mode=7) must not be bidirectional.
 */
export enum GTFSIsBidirectional {
  Unidirectional = 0,
  Bidirectional = 1,
}

/**
 * # Pathways
 *
 * **Optional**
 * Represents edges in a station graph describing station interiors, connecting
 * platforms, entrances/exits, generic nodes, or boarding areas.
 *
 * **Primary Key**: (pathway_id)
 *
 * Pathways must be complete if included:
 * - No dangling locations if any pathways exist, except for platforms that have boarding areas.
 * - Platforms with boarding areas must not have pathways directly; their boarding areas do.
 * - Each platform (location_type=0) or boarding area (4) must have at least
 *   one path to an entrance/exit (2) unless it’s impossible for riders to exit at that platform.
 */
export class GTFSPathway {
  /**
   * **Required**
   * Unique ID for the pathway record.
   */
  id: string;
  /**
   * **Required**
   * The stop or node from which this pathway begins.
   * Must be location_type=0, 2, 3, or 4 (platform, entrance/exit, generic node, or boarding area).
   * Stations (location_type=1) are forbidden here.
   */
  fromStopId: string;
  /**
   * **Required**
   * The stop or node at which this pathway ends.
   * Must be location_type=0, 2, 3, or 4 (platform, entrance/exit, generic node, or boarding area).
   * Stations (location_type=1) are forbidden here.
   */
  toStopId: string;
  /**
   * **Required**
   * Pathway mode, e.g. walkway, stairs, escalator.
   */
  mode: GTFSPathwayMode;
  /**
   * **Required**
   * 0 = Unidirectional, 1 = Bidirectional
   */
  isBidirectional: GTFSIsBidirectional;
  /**
   * **Optional**
   * Horizontal length in meters of the pathway.
   * Recommended for walkway, fare gate, exit gate.
   */
  length?: number;
  /**
   * **Optional**
   * Average time in seconds needed to traverse this pathway.
   * Recommended for moving sidewalk, escalator, elevator.
   */
  traversalTime?: number;
  /**
   * **Optional**
   * Number of stairs in this pathway.
   * Positive: fromStopId to toStopId goes upwards
   * Negative: fromStopId to toStopId goes downwards
   * Recommended for pathway_mode=2 (stairs).
   */
  stairCount?: number;
  /**
   * **Optional**
   * Maximum slope ratio. Positive for upwards, negative for downwards.
   * E.g., 0.083 is an 8.3% slope.
   * Used for walkway (1) or moving sidewalk (3) if relevant.
   */
  maxSlope?: number;
  /**
   * **Optional**
   * Minimum width of the pathway in meters, recommended if less than 1 meter.
   */
  minWidth?: number;
  /**
   * **Optional**
   * Public facing text on signage to help riders navigate (e.g. "Follow signs to X").
   */
  signpostedAs?: string;
  /**
   * **Optional**
   * Public facing text on signage when traversing the pathway in reverse
   * (toStopId -> fromStopId), if different from `signpostedAs`.
   */
  reversedSignpostedAs?: string;

  /** @param data - the parsed GTFS CSV data */
  constructor(data: Record<string, string>) {
    this.id = data.pathway_id;
    this.fromStopId = data.from_stop_id;
    this.toStopId = data.to_stop_id;
    // Required numeric fields
    this.mode =
      data.pathway_mode !== undefined
        ? (parseInt(data.pathway_mode, 10) as GTFSPathwayMode)
        : GTFSPathwayMode.Walkway;

    this.isBidirectional =
      data.is_bidirectional !== undefined
        ? (parseInt(data.is_bidirectional, 10) as GTFSIsBidirectional)
        : GTFSIsBidirectional.Unidirectional;
    // Optional fields
    if (data.length !== undefined && data.length !== '') {
      this.length = parseFloat(data.length);
    }
    if (data.traversal_time !== undefined && data.traversal_time !== '') {
      this.traversalTime = parseInt(data.traversal_time, 10);
    }
    if (data.stair_count !== undefined && data.stair_count !== '') {
      this.stairCount = parseInt(data.stair_count, 10);
    }
    if (data.max_slope !== undefined && data.max_slope !== '') {
      this.maxSlope = parseFloat(data.max_slope);
    }
    if (data.min_width !== undefined && data.min_width !== '') {
      this.minWidth = parseFloat(data.min_width);
    }
    this.signpostedAs = data.signposted_as;
    this.reversedSignpostedAs = data.reversed_signposted_as;
  }
}

/**
 * @param input - the input string to parse from
 * @returns - an array of Pathways
 */
export function parseGTFSPathways(input: string): Record<string, GTFSPathway> {
  const data = parseCSVAsRecord(input);
  const res: Record<string, GTFSPathway> = {};
  for (const d of data) {
    const pathway = new GTFSPathway(d);
    res[pathway.id] = pathway;
  }

  return res;
}
